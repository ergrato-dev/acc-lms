//! # Domain Entities
//!
//! Core domain models for the messaging service.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Conversation between users.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub conversation_id: Uuid,
    pub conversation_type: String,
    pub title: Option<String>,
    pub course_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Participant in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversationParticipant {
    pub participant_id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub is_muted: bool,
    pub is_archived: bool,
}

/// Message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub reply_to_id: Option<Uuid>,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Message with sender information.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageWithSender {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub sender_avatar_url: Option<String>,
    pub content: String,
    pub message_type: String,
    pub reply_to_id: Option<Uuid>,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Preview of a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePreview {
    pub message_id: Uuid,
    pub sender_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Preview of a participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPreview {
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_online: bool,
}

/// Conversation with preview information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPreview {
    pub conversation_id: Uuid,
    pub conversation_type: String,
    pub title: Option<String>,
    pub course_id: Option<Uuid>,
    pub last_message: Option<MessagePreview>,
    pub unread_count: i64,
    pub participants: Vec<ParticipantPreview>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of conversations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedConversations {
    pub items: Vec<ConversationPreview>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl PaginatedConversations {
    pub fn new(items: Vec<ConversationPreview>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

/// Paginated list of messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMessages {
    pub items: Vec<MessageWithSender>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub has_more: bool,
}

impl PaginatedMessages {
    pub fn new(items: Vec<MessageWithSender>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        let has_more = page < total_pages;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
            has_more,
        }
    }
}

/// Unread count per conversation.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversationUnreadCount {
    pub conversation_id: Uuid,
    pub unread_count: i64,
}
