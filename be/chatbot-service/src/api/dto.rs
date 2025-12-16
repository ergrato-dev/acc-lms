//! # Chatbot API DTOs
//!
//! Request and response types for the chatbot API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Conversation, ConversationContext, ConversationStatus, EscalationReason,
    FeedbackType, KBArticle, KBSearchResult, Message, UserRole, ChatbotAnalytics,
    ContextualSuggestion, RichContent,
};

// =============================================================================
// GENERIC RESPONSES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

// =============================================================================
// CONVERSATION DTOs
// =============================================================================

/// Request to start a new conversation.
#[derive(Debug, Deserialize, Validate)]
pub struct StartConversationRequest {
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    #[serde(default)]
    pub user_role: UserRole,
    pub context: Option<ConversationContextDto>,
}

/// Conversation context DTO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversationContextDto {
    pub current_page: Option<String>,
    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,
    #[serde(default = "default_language")]
    pub language: String,
    pub timezone: Option<String>,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

fn default_language() -> String {
    "es".to_string()
}

impl From<ConversationContextDto> for ConversationContext {
    fn from(dto: ConversationContextDto) -> Self {
        ConversationContext {
            current_page: dto.current_page,
            course_id: dto.course_id,
            lesson_id: dto.lesson_id,
            language: dto.language,
            timezone: dto.timezone,
            intent_history: vec![],
            custom: dto.custom,
        }
    }
}

/// Response for conversation operations.
#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub conversation_id: Uuid,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub messages: Vec<MessageResponse>,
}

impl From<(Conversation, Vec<Message>)> for ConversationResponse {
    fn from((conv, messages): (Conversation, Vec<Message>)) -> Self {
        Self {
            conversation_id: conv.conversation_id,
            status: format!("{:?}", conv.status).to_lowercase(),
            started_at: conv.started_at,
            messages: messages.into_iter().map(MessageResponse::from).collect(),
        }
    }
}

// =============================================================================
// MESSAGE DTOs
// =============================================================================

/// Request to send a message.
#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    pub context_update: Option<ConversationContextDto>,
}

/// Message response.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message_id: Uuid,
    pub sender: String,
    pub content: MessageContentResponse,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct MessageContentResponse {
    pub content_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_content: Option<RichContent>,
}

impl From<Message> for MessageResponse {
    fn from(msg: Message) -> Self {
        Self {
            message_id: msg.message_id,
            sender: format!("{:?}", msg.sender).to_lowercase(),
            content: MessageContentResponse {
                content_type: format!("{:?}", msg.content.content_type).to_lowercase(),
                text: msg.content.text,
                rich_content: msg.content.rich_content,
            },
            timestamp: msg.timestamp,
            confidence: msg.confidence,
        }
    }
}

/// Response for send message (includes both user and bot messages).
#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub user_message: MessageResponse,
    pub bot_message: MessageResponse,
}

// =============================================================================
// FEEDBACK DTOs
// =============================================================================

/// Request to add feedback.
#[derive(Debug, Deserialize, Validate)]
pub struct FeedbackRequest {
    pub feedback_type: FeedbackTypeDto,
    #[validate(length(max = 500))]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackTypeDto {
    ThumbsUp,
    ThumbsDown,
}

impl From<FeedbackTypeDto> for FeedbackType {
    fn from(dto: FeedbackTypeDto) -> Self {
        match dto {
            FeedbackTypeDto::ThumbsUp => FeedbackType::ThumbsUp,
            FeedbackTypeDto::ThumbsDown => FeedbackType::ThumbsDown,
        }
    }
}

// =============================================================================
// ESCALATION DTOs
// =============================================================================

/// Request to escalate conversation.
#[derive(Debug, Deserialize, Validate)]
pub struct EscalateRequest {
    pub reason: EscalationReasonDto,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationReasonDto {
    UserRequested,
    ComplexIssue,
    PaymentIssue,
    TechnicalIssue,
}

impl From<EscalationReasonDto> for EscalationReason {
    fn from(dto: EscalationReasonDto) -> Self {
        match dto {
            EscalationReasonDto::UserRequested => EscalationReason::UserRequested,
            EscalationReasonDto::ComplexIssue => EscalationReason::ComplexIssue,
            EscalationReasonDto::PaymentIssue => EscalationReason::PaymentIssue,
            EscalationReasonDto::TechnicalIssue => EscalationReason::TechnicalIssue,
        }
    }
}

/// Escalation response.
#[derive(Debug, Serialize)]
pub struct EscalationResponse {
    pub conversation_id: Uuid,
    pub status: String,
    pub ticket_id: Option<String>,
    pub message: String,
}

// =============================================================================
// KNOWLEDGE BASE DTOs
// =============================================================================

/// KB search request.
#[derive(Debug, Deserialize, Validate)]
pub struct KBSearchRequest {
    #[validate(length(min = 1, max = 200))]
    pub query: String,
    #[serde(default)]
    pub role: UserRole,
    #[serde(default = "default_language")]
    pub language: String,
}

/// KB article response.
#[derive(Debug, Serialize)]
pub struct KBArticleResponse {
    pub article_id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category: String,
    pub subcategory: Option<String>,
    pub tags: Vec<String>,
    pub view_count: i64,
    pub helpful_count: i64,
    pub not_helpful_count: i64,
}

impl From<KBArticle> for KBArticleResponse {
    fn from(article: KBArticle) -> Self {
        Self {
            article_id: article.article_id,
            slug: article.slug,
            title: article.title,
            content: article.content,
            summary: article.summary,
            category: article.category,
            subcategory: article.subcategory,
            tags: article.tags,
            view_count: article.view_count,
            helpful_count: article.helpful_count,
            not_helpful_count: article.not_helpful_count,
        }
    }
}

/// KB search result response.
#[derive(Debug, Serialize)]
pub struct KBSearchResultResponse {
    pub article: KBArticleResponse,
    pub relevance_score: f64,
    pub snippet: String,
}

impl From<KBSearchResult> for KBSearchResultResponse {
    fn from(result: KBSearchResult) -> Self {
        Self {
            article: KBArticleResponse::from(result.article),
            relevance_score: result.relevance_score,
            snippet: result.snippet,
        }
    }
}

/// Article feedback request.
#[derive(Debug, Deserialize)]
pub struct ArticleFeedbackRequest {
    pub helpful: bool,
}

/// Create article request.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateArticleRequest {
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub summary: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub category: String,
    pub subcategory: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub intent_triggers: Vec<String>,
    #[serde(default)]
    pub target_roles: Vec<UserRole>,
    #[serde(default = "default_language")]
    pub language: String,
}

// =============================================================================
// SUGGESTIONS DTOs
// =============================================================================

/// Suggestions request.
#[derive(Debug, Deserialize)]
pub struct SuggestionsRequest {
    #[serde(default)]
    pub role: UserRole,
    pub page: Option<String>,
}

/// Suggestion response.
#[derive(Debug, Serialize)]
pub struct SuggestionResponse {
    pub suggestion_id: Uuid,
    pub text: String,
    pub intent: String,
}

impl From<ContextualSuggestion> for SuggestionResponse {
    fn from(s: ContextualSuggestion) -> Self {
        Self {
            suggestion_id: s.suggestion_id,
            text: s.text,
            intent: s.intent,
        }
    }
}

// =============================================================================
// ANALYTICS DTOs
// =============================================================================

/// Analytics query params.
#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

/// Analytics response.
#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub total_conversations: i64,
    pub total_messages: i64,
    pub unique_users: i64,
    pub avg_messages_per_conversation: f64,
    pub avg_response_time_ms: f64,
    pub resolution_rate: f64,
    pub escalation_rate: f64,
    pub satisfaction_rate: f64,
    pub top_intents: Vec<IntentStatResponse>,
}

#[derive(Debug, Serialize)]
pub struct IntentStatResponse {
    pub intent: String,
    pub count: i64,
    pub avg_confidence: f64,
}

impl From<ChatbotAnalytics> for AnalyticsResponse {
    fn from(a: ChatbotAnalytics) -> Self {
        Self {
            period_start: a.period_start,
            period_end: a.period_end,
            total_conversations: a.total_conversations,
            total_messages: a.total_messages,
            unique_users: a.unique_users,
            avg_messages_per_conversation: a.avg_messages_per_conversation,
            avg_response_time_ms: a.avg_response_time_ms,
            resolution_rate: a.resolution_rate,
            escalation_rate: a.escalation_rate,
            satisfaction_rate: a.satisfaction_rate,
            top_intents: a.top_intents.into_iter().map(|i| IntentStatResponse {
                intent: i.intent,
                count: i.count,
                avg_confidence: i.avg_confidence,
            }).collect(),
        }
    }
}
