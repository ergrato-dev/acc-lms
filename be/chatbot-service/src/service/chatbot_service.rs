//! # Chatbot Service
//!
//! Business logic for the AI-powered chatbot system.

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    Conversation, ConversationContext, ConversationStatus, EscalationReason, FeedbackType,
    KBArticle, KBSearchResult, Message, NewConversation, NewKBArticle, NewMessage,
    UserRole, DetectedIntent, IntentCategory, MessageContent, ContentType, RichContent,
    CardContent, CardButton, ButtonAction, QuickReply, ChatbotAnalytics, ContextualSuggestion,
};
use crate::repository::{ChatbotRepository, RepositoryError};

// =============================================================================
// SERVICE ERRORS
// =============================================================================

#[derive(Debug, thiserror::Error)]
pub enum ChatbotError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Conversation not found: {0}")]
    ConversationNotFound(Uuid),

    #[error("AI service error: {0}")]
    AIService(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

pub type Result<T> = std::result::Result<T, ChatbotError>;

// =============================================================================
// AI CLIENT TRAIT
// =============================================================================

/// Trait for AI response generation.
#[async_trait::async_trait]
pub trait AIClient: Send + Sync {
    /// Generates a response for the given message and context.
    async fn generate_response(
        &self,
        message: &str,
        context: &ConversationContext,
        role: UserRole,
        history: &[Message],
    ) -> std::result::Result<AIResponse, String>;

    /// Detects intent from user message.
    async fn detect_intent(
        &self,
        message: &str,
        role: UserRole,
    ) -> std::result::Result<DetectedIntent, String>;
}

/// AI response with confidence and suggested actions.
#[derive(Debug, Clone)]
pub struct AIResponse {
    pub text: String,
    pub confidence: f64,
    pub intent: Option<DetectedIntent>,
    pub suggested_articles: Vec<Uuid>,
    pub should_escalate: bool,
    pub rich_content: Option<RichContent>,
}

// =============================================================================
// CHATBOT SERVICE
// =============================================================================

/// Service for chatbot operations.
pub struct ChatbotService {
    repository: ChatbotRepository,
    ai_client: Arc<dyn AIClient>,
    escalation_threshold: f64,
}

impl ChatbotService {
    pub fn new(repository: ChatbotRepository, ai_client: Arc<dyn AIClient>) -> Self {
        Self {
            repository,
            ai_client,
            escalation_threshold: 0.6,
        }
    }

    /// Sets the confidence threshold for escalation.
    pub fn with_escalation_threshold(mut self, threshold: f64) -> Self {
        self.escalation_threshold = threshold;
        self
    }

    // =========================================================================
    // CONVERSATION OPERATIONS
    // =========================================================================

    /// Starts a new conversation.
    pub async fn start_conversation(&self, request: NewConversation) -> Result<Conversation> {
        let conversation = self.repository.create_conversation(request).await?;

        // Send welcome message based on role
        let welcome = self.get_welcome_message(&conversation.user_role);
        let bot_message = Message::bot_message(
            conversation.conversation_id,
            welcome.text,
            Some(1.0),
        ).with_rich_content(welcome.rich_content.unwrap_or(RichContent::QuickReplies {
            options: self.get_initial_suggestions(&conversation.user_role),
        }));

        self.repository.save_message(&bot_message).await?;

        Ok(conversation)
    }

    /// Processes a user message and generates a response.
    pub async fn process_message(&self, request: NewMessage) -> Result<(Message, Message)> {
        // Get conversation
        let conversation = self.repository.get_conversation(request.conversation_id).await?;

        // Update context if provided
        if let Some(ref context) = request.context_update {
            self.repository.update_conversation_context(
                request.conversation_id,
                context,
            ).await?;
        }

        // Create user message
        let user_message = Message::user_message(request.conversation_id, request.content.clone());
        self.repository.save_message(&user_message).await?;

        // Get conversation history for context
        let history = self.repository.get_messages(request.conversation_id, 10, 0).await?;

        // Detect intent
        let intent = self.ai_client.detect_intent(&request.content, conversation.user_role)
            .await
            .ok();

        // Search KB for relevant articles
        let kb_results = self.search_knowledge_base(
            &request.content,
            conversation.user_role,
            &conversation.context.language,
        ).await.unwrap_or_default();

        // Generate AI response
        let ai_response = self.ai_client.generate_response(
            &request.content,
            &conversation.context,
            conversation.user_role,
            &history,
        ).await.map_err(|e| ChatbotError::AIService(e))?;

        // Check if should escalate
        if ai_response.should_escalate || ai_response.confidence < self.escalation_threshold {
            // Auto-escalate with reason
            let reason = if ai_response.confidence < self.escalation_threshold {
                EscalationReason::LowConfidence
            } else {
                EscalationReason::ComplexIssue
            };

            self.repository.escalate_conversation(
                request.conversation_id,
                reason,
                Some(format!("Confidence: {:.2}", ai_response.confidence)),
            ).await?;
        }

        // Build response with KB articles if available
        let response_content = if !kb_results.is_empty() {
            self.build_response_with_articles(&ai_response.text, &kb_results)
        } else {
            MessageContent {
                content_type: ContentType::Text,
                text: ai_response.text.clone(),
                rich_content: ai_response.rich_content,
            }
        };

        // Create bot message
        let mut bot_message = Message::bot_message(
            request.conversation_id,
            response_content.text.clone(),
            Some(ai_response.confidence),
        );
        bot_message.content = response_content;

        if let Some(intent) = intent {
            bot_message = bot_message.with_intent(intent);
        }

        self.repository.save_message(&bot_message).await?;

        Ok((user_message, bot_message))
    }

    /// Gets conversation history.
    pub async fn get_history(
        &self,
        conversation_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>> {
        Ok(self.repository.get_messages(conversation_id, limit, offset).await?)
    }

    /// Adds feedback to a message.
    pub async fn add_feedback(
        &self,
        message_id: Uuid,
        feedback_type: FeedbackType,
        comment: Option<String>,
    ) -> Result<()> {
        self.repository.add_message_feedback(message_id, feedback_type, comment).await?;
        Ok(())
    }

    /// Escalates a conversation to human agent.
    pub async fn escalate(
        &self,
        conversation_id: Uuid,
        reason: EscalationReason,
        notes: Option<String>,
    ) -> Result<Conversation> {
        // Add system message about escalation
        let system_message = Message {
            message_id: Uuid::new_v4(),
            conversation_id,
            sender: crate::domain::MessageSender::System,
            content: MessageContent {
                content_type: ContentType::Text,
                text: format!("Conversación escalada: {}", reason),
                rich_content: None,
            },
            timestamp: chrono::Utc::now(),
            intent: None,
            confidence: None,
            feedback: None,
            metadata: std::collections::HashMap::new(),
        };
        self.repository.save_message(&system_message).await?;

        Ok(self.repository.escalate_conversation(conversation_id, reason, notes).await?)
    }

    /// Ends a conversation.
    pub async fn end_conversation(
        &self,
        conversation_id: Uuid,
        status: ConversationStatus,
    ) -> Result<Conversation> {
        Ok(self.repository.end_conversation(conversation_id, status).await?)
    }

    // =========================================================================
    // KNOWLEDGE BASE OPERATIONS
    // =========================================================================

    /// Searches the knowledge base.
    pub async fn search_knowledge_base(
        &self,
        query: &str,
        role: UserRole,
        language: &str,
    ) -> Result<Vec<KBSearchResult>> {
        Ok(self.repository.search_articles(query, role, language, 5).await?)
    }

    /// Gets an article by slug.
    pub async fn get_article(&self, slug: &str) -> Result<KBArticle> {
        Ok(self.repository.get_article_by_slug(slug).await?)
    }

    /// Gets popular articles.
    pub async fn get_popular_articles(&self, role: UserRole, limit: i64) -> Result<Vec<KBArticle>> {
        Ok(self.repository.get_popular_articles(role, limit).await?)
    }

    /// Gets articles by category.
    pub async fn get_articles_by_category(
        &self,
        category: &str,
        role: UserRole,
        limit: i64,
    ) -> Result<Vec<KBArticle>> {
        Ok(self.repository.get_articles_by_category(category, role, limit).await?)
    }

    /// Creates a new KB article.
    pub async fn create_article(&self, article: NewKBArticle) -> Result<KBArticle> {
        Ok(self.repository.create_article(article).await?)
    }

    /// Records article feedback.
    pub async fn record_article_feedback(&self, article_id: Uuid, helpful: bool) -> Result<()> {
        self.repository.record_article_feedback(article_id, helpful).await?;
        Ok(())
    }

    // =========================================================================
    // SUGGESTIONS
    // =========================================================================

    /// Gets contextual suggestions for the user.
    pub async fn get_suggestions(
        &self,
        role: UserRole,
        page: Option<&str>,
    ) -> Result<Vec<ContextualSuggestion>> {
        Ok(self.repository.get_suggestions(role, page).await?)
    }

    // =========================================================================
    // ANALYTICS
    // =========================================================================

    /// Gets chatbot analytics.
    pub async fn get_analytics(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<ChatbotAnalytics> {
        Ok(self.repository.get_analytics(start, end).await?)
    }

    // =========================================================================
    // PRIVATE HELPERS
    // =========================================================================

    fn get_welcome_message(&self, role: &UserRole) -> AIResponse {
        let (text, suggestions) = match role {
            UserRole::Anonymous => (
                "¡Hola! Soy el asistente virtual de ACC LMS. ¿En qué puedo ayudarte hoy?".to_string(),
                vec![
                    ("Ver cursos disponibles", "browse_courses"),
                    ("¿Cómo funciona?", "how_it_works"),
                    ("Registrarme", "register"),
                ],
            ),
            UserRole::Student => (
                "¡Hola! ¿En qué puedo ayudarte con tus cursos?".to_string(),
                vec![
                    ("Mi progreso", "my_progress"),
                    ("Mis certificados", "my_certificates"),
                    ("Problema técnico", "technical_issue"),
                    ("Pagos", "payment_help"),
                ],
            ),
            UserRole::Instructor => (
                "¡Hola! ¿En qué puedo ayudarte con tu contenido?".to_string(),
                vec![
                    ("Crear curso", "create_course"),
                    ("Mis analytics", "my_analytics"),
                    ("Mis ingresos", "my_earnings"),
                    ("Gestionar estudiantes", "manage_students"),
                ],
            ),
            UserRole::Admin => (
                "¡Hola Admin! ¿Qué necesitas revisar?".to_string(),
                vec![
                    ("Estado del sistema", "system_health"),
                    ("Reportes", "reports"),
                    ("Usuarios", "user_management"),
                    ("Configuración", "configuration"),
                ],
            ),
        };

        AIResponse {
            text,
            confidence: 1.0,
            intent: None,
            suggested_articles: vec![],
            should_escalate: false,
            rich_content: Some(RichContent::QuickReplies {
                options: suggestions.iter().map(|(label, payload)| QuickReply {
                    label: label.to_string(),
                    payload: payload.to_string(),
                }).collect(),
            }),
        }
    }

    fn get_initial_suggestions(&self, role: &UserRole) -> Vec<QuickReply> {
        match role {
            UserRole::Anonymous => vec![
                QuickReply { label: "Ver cursos".to_string(), payload: "browse_courses".to_string() },
                QuickReply { label: "Registrarme".to_string(), payload: "register".to_string() },
                QuickReply { label: "Preguntas frecuentes".to_string(), payload: "faq".to_string() },
            ],
            UserRole::Student => vec![
                QuickReply { label: "Mi progreso".to_string(), payload: "my_progress".to_string() },
                QuickReply { label: "Ayuda".to_string(), payload: "help".to_string() },
            ],
            UserRole::Instructor => vec![
                QuickReply { label: "Mis cursos".to_string(), payload: "my_courses".to_string() },
                QuickReply { label: "Analytics".to_string(), payload: "analytics".to_string() },
            ],
            UserRole::Admin => vec![
                QuickReply { label: "Dashboard".to_string(), payload: "dashboard".to_string() },
                QuickReply { label: "Alertas".to_string(), payload: "alerts".to_string() },
            ],
        }
    }

    fn build_response_with_articles(
        &self,
        base_response: &str,
        articles: &[KBSearchResult],
    ) -> MessageContent {
        if articles.is_empty() {
            return MessageContent {
                content_type: ContentType::Text,
                text: base_response.to_string(),
                rich_content: None,
            };
        }

        let text = format!(
            "{}\n\nTambién encontré estos artículos que pueden ayudarte:",
            base_response
        );

        let cards: Vec<CardContent> = articles.iter().take(3).map(|result| {
            CardContent {
                title: result.article.title.clone(),
                subtitle: Some(result.snippet.clone()),
                image_url: None,
                buttons: vec![CardButton {
                    label: "Ver artículo".to_string(),
                    action: ButtonAction::Url {
                        url: format!("/ayuda/{}", result.article.slug),
                    },
                }],
            }
        }).collect();

        MessageContent {
            content_type: ContentType::Carousel,
            text,
            rich_content: Some(RichContent::Carousel { cards }),
        }
    }
}
