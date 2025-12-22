//! # Data Transfer Objects
//!
//! Request and response DTOs for the messaging API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// Request DTOs
// =============================================================================

/// Request to create a new conversation.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateConversationRequest {
    /// Type of conversation (direct, course_discussion, support)
    #[serde(default = "default_conversation_type")]
    pub conversation_type: String,
    /// Optional title for the conversation
    #[validate(length(max = 200, message = "Title cannot exceed 200 characters"))]
    pub title: Option<String>,
    /// Course ID for course discussions
    pub course_id: Option<Uuid>,
    /// List of participant user IDs
    #[validate(length(min = 1, message = "At least one participant is required"))]
    pub participant_ids: Vec<Uuid>,
}

/// Request to send a message.
#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    /// Sender user ID
    pub sender_id: Uuid,
    /// Message content
    #[validate(length(min = 1, max = 5000, message = "Message must be 1-5000 characters"))]
    pub content: String,
    /// Type of message
    #[serde(default = "default_message_type")]
    pub message_type: String,
    /// ID of message being replied to
    pub reply_to_id: Option<Uuid>,
}

/// Request to edit a message.
#[derive(Debug, Deserialize, Validate)]
pub struct EditMessageRequest {
    /// User ID making the edit
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 5000, message = "Message must be 1-5000 characters"))]
    pub content: String,
}

/// Request to delete a message.
#[derive(Debug, Deserialize)]
pub struct DeleteMessageRequest {
    pub user_id: Uuid,
}

/// Query parameters for listing conversations.
#[derive(Debug, Deserialize)]
pub struct ListConversationsQuery {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    /// Filter by conversation type
    pub conversation_type: Option<String>,
    /// Include archived conversations
    #[serde(default)]
    pub include_archived: bool,
}

/// Query parameters for listing messages.
#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_messages_per_page")]
    pub per_page: i32,
    /// Load messages before this ID (for infinite scroll)
    pub before: Option<Uuid>,
    /// Load messages after this ID
    pub after: Option<Uuid>,
}

/// Query parameters for searching messages.
#[derive(Debug, Deserialize, Validate)]
pub struct SearchMessagesQuery {
    /// Search query
    #[validate(length(min = 2, message = "Search query must be at least 2 characters"))]
    pub q: String,
    /// Filter by conversation
    pub conversation_id: Option<Uuid>,
    /// Page number
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

/// Request to toggle mute status.
#[derive(Debug, Deserialize)]
pub struct ToggleMuteRequest {
    pub user_id: Uuid,
    pub muted: bool,
}

/// Request to toggle archive status.
#[derive(Debug, Deserialize)]
pub struct ToggleArchiveRequest {
    pub user_id: Uuid,
    pub archived: bool,
}

/// Request to mark as read.
#[derive(Debug, Deserialize)]
pub struct MarkAsReadRequest {
    pub user_id: Uuid,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// Response for a conversation.
#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub conversation_id: Uuid,
    pub conversation_type: String,
    pub title: Option<String>,
    pub course_id: Option<Uuid>,
    pub participants: Vec<ParticipantResponse>,
    pub last_message: Option<MessagePreviewResponse>,
    pub unread_count: i64,
    pub is_muted: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Response for a participant.
#[derive(Debug, Serialize)]
pub struct ParticipantResponse {
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_online: bool,
    pub last_seen: Option<DateTime<Utc>>,
}

/// Response for a message preview.
#[derive(Debug, Serialize)]
pub struct MessagePreviewResponse {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Response for a full message.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender: SenderResponse,
    pub content: String,
    pub message_type: String,
    pub reply_to_id: Option<Uuid>,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Response for sender info.
#[derive(Debug, Serialize)]
pub struct SenderResponse {
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
}

/// Response for paginated conversations.
#[derive(Debug, Serialize)]
pub struct PaginatedConversationsResponse {
    pub items: Vec<ConversationResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Response for paginated messages.
#[derive(Debug, Serialize)]
pub struct PaginatedMessagesResponse {
    pub items: Vec<MessageResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub has_more: bool,
}

/// Response for unread count.
#[derive(Debug, Serialize)]
pub struct UnreadCountResponse {
    pub total_unread: i64,
    pub conversations: Vec<ConversationUnreadResponse>,
}

/// Unread count per conversation.
#[derive(Debug, Serialize)]
pub struct ConversationUnreadResponse {
    pub conversation_id: Uuid,
    pub unread_count: i64,
}

/// Simple message response after send/edit.
#[derive(Debug, Serialize)]
pub struct MessageSentResponse {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// Default Functions
// =============================================================================

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

fn default_messages_per_page() -> i32 {
    50
}

fn default_conversation_type() -> String {
    "direct".to_string()
}

fn default_message_type() -> String {
    "text".to_string()
}
