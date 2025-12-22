//! # Messaging Error Types
//!
//! Custom error types for the messaging service.

use std::fmt;

/// Messaging service errors.
#[derive(Debug)]
pub enum MessagingError {
    /// Conversation not found
    ConversationNotFound(uuid::Uuid),
    /// Message not found
    MessageNotFound(uuid::Uuid),
    /// User not found
    UserNotFound(uuid::Uuid),
    /// User not a participant in conversation
    NotParticipant(uuid::Uuid),
    /// Cannot message yourself
    CannotMessageSelf,
    /// Conversation already exists between users
    ConversationExists(uuid::Uuid),
    /// Message too long
    MessageTooLong(usize),
    /// User not authorized for this action
    Unauthorized,
    /// Database error
    Database(String),
    /// Cache error
    CacheError(String),
    /// Internal error
    Internal(String),
    /// Validation error
    Validation(String),
    /// Resource not found
    NotFound(String),
    /// Forbidden action
    Forbidden(String),
}

impl fmt::Display for MessagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagingError::ConversationNotFound(id) => {
                write!(f, "Conversation not found: {}", id)
            }
            MessagingError::MessageNotFound(id) => write!(f, "Message not found: {}", id),
            MessagingError::UserNotFound(id) => write!(f, "User not found: {}", id),
            MessagingError::NotParticipant(id) => {
                write!(f, "User is not a participant in conversation: {}", id)
            }
            MessagingError::CannotMessageSelf => write!(f, "Cannot send message to yourself"),
            MessagingError::ConversationExists(id) => {
                write!(f, "Conversation already exists: {}", id)
            }
            MessagingError::MessageTooLong(len) => {
                write!(f, "Message too long: {} characters (max 5000)", len)
            }
            MessagingError::Unauthorized => write!(f, "Not authorized for this action"),
            MessagingError::Database(msg) => write!(f, "Database error: {}", msg),
            MessagingError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            MessagingError::Internal(msg) => write!(f, "Internal error: {}", msg),
            MessagingError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MessagingError::NotFound(msg) => write!(f, "Not found: {}", msg),
            MessagingError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
        }
    }
}

impl std::error::Error for MessagingError {}

impl actix_web::ResponseError for MessagingError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let status = match self {
            MessagingError::ConversationNotFound(_) => StatusCode::NOT_FOUND,
            MessagingError::MessageNotFound(_) => StatusCode::NOT_FOUND,
            MessagingError::UserNotFound(_) => StatusCode::NOT_FOUND,
            MessagingError::NotParticipant(_) => StatusCode::FORBIDDEN,
            MessagingError::CannotMessageSelf => StatusCode::BAD_REQUEST,
            MessagingError::ConversationExists(_) => StatusCode::CONFLICT,
            MessagingError::MessageTooLong(_) => StatusCode::BAD_REQUEST,
            MessagingError::Unauthorized => StatusCode::UNAUTHORIZED,
            MessagingError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MessagingError::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MessagingError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MessagingError::Validation(_) => StatusCode::BAD_REQUEST,
            MessagingError::NotFound(_) => StatusCode::NOT_FOUND,
            MessagingError::Forbidden(_) => StatusCode::FORBIDDEN,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "code": format!("{:?}", self).split('(').next().unwrap_or("Unknown")
        }))
    }
}

impl From<sqlx::Error> for MessagingError {
    fn from(err: sqlx::Error) -> Self {
        MessagingError::Database(err.to_string())
    }
}

impl From<redis::RedisError> for MessagingError {
    fn from(err: redis::RedisError) -> Self {
        MessagingError::CacheError(err.to_string())
    }
}

/// Result type alias for messaging operations.
pub type MessagingResult<T> = Result<T, MessagingError>;
