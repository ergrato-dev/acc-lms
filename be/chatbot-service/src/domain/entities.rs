//! # Chatbot Domain Entities
//!
//! Core entities for the AI-powered chatbot system supporting all user roles.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// User role for contextual responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[default]
    Anonymous,
    Student,
    Instructor,
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Anonymous => write!(f, "anonymous"),
            UserRole::Student => write!(f, "student"),
            UserRole::Instructor => write!(f, "instructor"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

/// Message sender type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSender {
    User,
    Bot,
    System,
    Agent,
}

/// Conversation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    #[default]
    Active,
    Escalated,
    Resolved,
    Abandoned,
}

/// Escalation reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationReason {
    LowConfidence,
    NegativeSentiment,
    UserRequested,
    ComplexIssue,
    SensitiveData,
    PaymentIssue,
    TechnicalIssue,
}

impl std::fmt::Display for EscalationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EscalationReason::LowConfidence => write!(f, "low_confidence"),
            EscalationReason::NegativeSentiment => write!(f, "negative_sentiment"),
            EscalationReason::UserRequested => write!(f, "user_requested"),
            EscalationReason::ComplexIssue => write!(f, "complex_issue"),
            EscalationReason::SensitiveData => write!(f, "sensitive_data"),
            EscalationReason::PaymentIssue => write!(f, "payment_issue"),
            EscalationReason::TechnicalIssue => write!(f, "technical_issue"),
        }
    }
}

/// Feedback type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackType {
    ThumbsUp,
    ThumbsDown,
}

/// Message content type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Card,
    Carousel,
    Code,
    Image,
    Link,
    QuickReplies,
}

/// Intent category for routing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentCategory {
    // Common
    Greeting,
    Farewell,
    Thanks,
    Help,

    // Student intents
    CourseProgress,
    CourseContent,
    Certificate,
    Payment,
    TechnicalIssue,

    // Instructor intents
    CourseCreation,
    Analytics,
    Earnings,
    StudentManagement,

    // Admin intents
    UserManagement,
    SystemHealth,
    Reports,
    Configuration,

    // Fallback
    Unknown,
}

// =============================================================================
// MAIN ENTITIES
// =============================================================================

/// Chat conversation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub conversation_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_role: UserRole,
    pub status: ConversationStatus,
    pub started_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub message_count: i32,
    pub context: ConversationContext,
    pub escalation: Option<EscalationInfo>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Conversation context for personalization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationContext {
    /// Current page/section the user is viewing
    pub current_page: Option<String>,
    /// Current course context (if in a course)
    pub course_id: Option<Uuid>,
    /// Current lesson context
    pub lesson_id: Option<Uuid>,
    /// User's language preference
    pub language: String,
    /// User's timezone
    pub timezone: Option<String>,
    /// Previous intents detected
    pub intent_history: Vec<String>,
    /// Custom context data
    pub custom: HashMap<String, serde_json::Value>,
}

/// Escalation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationInfo {
    pub escalated_at: DateTime<Utc>,
    pub reason: EscalationReason,
    pub agent_id: Option<Uuid>,
    pub ticket_id: Option<String>,
    pub notes: Option<String>,
}

/// Chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender: MessageSender,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
    pub intent: Option<DetectedIntent>,
    pub confidence: Option<f64>,
    pub feedback: Option<MessageFeedback>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Message content with rich formatting support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    pub content_type: ContentType,
    pub text: String,
    pub rich_content: Option<RichContent>,
}

/// Rich content for cards, carousels, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichContent {
    Card(CardContent),
    Carousel { cards: Vec<CardContent> },
    Code { language: String, code: String },
    Image { url: String, alt: Option<String> },
    QuickReplies { options: Vec<QuickReply> },
    Links { links: Vec<LinkItem> },
}

/// Card content for rich messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardContent {
    pub title: String,
    pub subtitle: Option<String>,
    pub image_url: Option<String>,
    pub buttons: Vec<CardButton>,
}

/// Card button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardButton {
    pub label: String,
    pub action: ButtonAction,
}

/// Button action type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ButtonAction {
    Url { url: String },
    Postback { payload: String },
    Call { phone: String },
}

/// Quick reply option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReply {
    pub label: String,
    pub payload: String,
}

/// Link item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkItem {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

/// Detected intent from NLU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIntent {
    pub name: String,
    pub category: IntentCategory,
    pub confidence: f64,
    pub entities: Vec<ExtractedEntity>,
}

/// Extracted entity from user message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub entity_type: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub confidence: f64,
}

/// Message feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFeedback {
    pub feedback_type: FeedbackType,
    pub submitted_at: DateTime<Utc>,
    pub comment: Option<String>,
}

// =============================================================================
// KNOWLEDGE BASE ENTITIES
// =============================================================================

/// Knowledge base article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBArticle {
    pub article_id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub subcategory: Option<String>,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub intent_triggers: Vec<String>,
    pub target_roles: Vec<UserRole>,
    pub language: String,
    pub status: ArticleStatus,
    pub view_count: i64,
    pub helpful_count: i64,
    pub not_helpful_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author_id: Uuid,
}

/// Article status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArticleStatus {
    #[default]
    Draft,
    Published,
    Archived,
}

/// KB search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBSearchResult {
    pub article: KBArticle,
    pub relevance_score: f64,
    pub matched_keywords: Vec<String>,
    pub snippet: String,
}

// =============================================================================
// SUGGESTION ENTITIES
// =============================================================================

/// Contextual suggestion for users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualSuggestion {
    pub suggestion_id: Uuid,
    pub text: String,
    pub intent: String,
    pub target_roles: Vec<UserRole>,
    pub context_conditions: SuggestionConditions,
    pub priority: i32,
    pub is_active: bool,
}

/// Conditions for showing suggestions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestionConditions {
    pub pages: Option<Vec<String>>,
    pub has_enrolled_courses: Option<bool>,
    pub has_created_courses: Option<bool>,
    pub is_new_user: Option<bool>,
    pub time_on_page_seconds: Option<i32>,
}

// =============================================================================
// ANALYTICS ENTITIES
// =============================================================================

/// Chatbot analytics summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatbotAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_conversations: i64,
    pub total_messages: i64,
    pub unique_users: i64,
    pub avg_messages_per_conversation: f64,
    pub avg_response_time_ms: f64,
    pub resolution_rate: f64,
    pub escalation_rate: f64,
    pub satisfaction_rate: f64,
    pub top_intents: Vec<IntentStat>,
    pub unanswered_queries: Vec<UnansweredQuery>,
}

/// Intent statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentStat {
    pub intent: String,
    pub count: i64,
    pub avg_confidence: f64,
}

/// Unanswered query for training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnansweredQuery {
    pub query: String,
    pub count: i64,
    pub sample_conversation_ids: Vec<Uuid>,
}

// =============================================================================
// NEW ENTITY BUILDERS
// =============================================================================

/// New conversation request.
#[derive(Debug, Clone, Deserialize)]
pub struct NewConversation {
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_role: UserRole,
    pub context: Option<ConversationContext>,
}

/// New message request.
#[derive(Debug, Clone, Deserialize)]
pub struct NewMessage {
    pub conversation_id: Uuid,
    pub content: String,
    pub context_update: Option<ConversationContext>,
}

/// New KB article.
#[derive(Debug, Clone, Deserialize)]
pub struct NewKBArticle {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub subcategory: Option<String>,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub intent_triggers: Vec<String>,
    pub target_roles: Vec<UserRole>,
    pub language: String,
    pub author_id: Uuid,
}

impl Conversation {
    pub fn new(request: NewConversation) -> Self {
        let now = Utc::now();
        Self {
            conversation_id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            user_role: request.user_role,
            status: ConversationStatus::Active,
            started_at: now,
            last_activity_at: now,
            ended_at: None,
            message_count: 0,
            context: request.context.unwrap_or_default(),
            escalation: None,
            metadata: HashMap::new(),
        }
    }
}

impl Message {
    pub fn user_message(conversation_id: Uuid, content: String) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            conversation_id,
            sender: MessageSender::User,
            content: MessageContent {
                content_type: ContentType::Text,
                text: content,
                rich_content: None,
            },
            timestamp: Utc::now(),
            intent: None,
            confidence: None,
            feedback: None,
            metadata: HashMap::new(),
        }
    }

    pub fn bot_message(conversation_id: Uuid, content: String, confidence: Option<f64>) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            conversation_id,
            sender: MessageSender::Bot,
            content: MessageContent {
                content_type: ContentType::Text,
                text: content,
                rich_content: None,
            },
            timestamp: Utc::now(),
            intent: None,
            confidence,
            feedback: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_rich_content(mut self, rich_content: RichContent) -> Self {
        self.content.rich_content = Some(rich_content);
        self
    }

    pub fn with_intent(mut self, intent: DetectedIntent) -> Self {
        self.confidence = Some(intent.confidence);
        self.intent = Some(intent);
        self
    }
}

impl ConversationContext {
    pub fn new(language: String) -> Self {
        Self {
            language,
            ..Default::default()
        }
    }
}
