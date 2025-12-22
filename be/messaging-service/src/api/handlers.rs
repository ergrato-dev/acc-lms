//! # API Handlers
//!
//! Request handlers for messaging endpoints.

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::errors::MessagingError;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub messaging_service: std::sync::Arc<crate::services::messaging::MessagingService>,
}

// =============================================================================
// Conversation Handlers
// =============================================================================

/// List conversations for a user.
pub async fn list_conversations(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    query: web::Query<ListConversationsQuery>,
) -> Result<HttpResponse, MessagingError> {
    let result = state.messaging_service.list_conversations(
        *user_id,
        query.page,
        query.per_page,
        query.conversation_type.clone(),
        query.include_archived,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Create a new conversation.
pub async fn create_conversation(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    body: web::Json<CreateConversationRequest>,
) -> Result<HttpResponse, MessagingError> {
    body.validate().map_err(|e| MessagingError::Validation(e.to_string()))?;

    let result = state.messaging_service.create_conversation(
        *user_id,
        &body.conversation_type,
        body.title.clone(),
        body.course_id,
        &body.participant_ids,
    ).await?;

    Ok(HttpResponse::Created().json(result))
}

/// Get a specific conversation.
pub async fn get_conversation(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, MessagingError> {
    let (user_id, conversation_id) = path.into_inner();

    let result = state.messaging_service.get_conversation(
        user_id,
        conversation_id,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Mute or unmute a conversation.
pub async fn mute_conversation(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ToggleMuteRequest>,
) -> Result<HttpResponse, MessagingError> {
    let (_, conversation_id) = path.into_inner();

    state.messaging_service.toggle_mute(
        body.user_id,
        conversation_id,
        body.muted,
    ).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Archive or unarchive a conversation.
pub async fn archive_conversation(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<ToggleArchiveRequest>,
) -> Result<HttpResponse, MessagingError> {
    let (_, conversation_id) = path.into_inner();

    state.messaging_service.toggle_archive(
        body.user_id,
        conversation_id,
        body.archived,
    ).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Leave a conversation.
pub async fn leave_conversation(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, MessagingError> {
    let (user_id, conversation_id) = path.into_inner();

    state.messaging_service.leave_conversation(user_id, conversation_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get or create a direct conversation with another user.
pub async fn get_or_create_direct_conversation(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, MessagingError> {
    let (user_id, other_user_id) = path.into_inner();

    let result = state.messaging_service.get_or_create_direct_conversation(
        user_id,
        other_user_id,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

// =============================================================================
// Message Handlers
// =============================================================================

/// List messages in a conversation.
pub async fn list_messages(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<ListMessagesQuery>,
) -> Result<HttpResponse, MessagingError> {
    let (user_id, conversation_id) = path.into_inner();

    let result = state.messaging_service.list_messages(
        user_id,
        conversation_id,
        query.page,
        query.per_page,
        query.before,
        query.after,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Send a message to a conversation.
pub async fn send_message(
    state: web::Data<AppState>,
    conversation_id: web::Path<Uuid>,
    body: web::Json<SendMessageRequest>,
) -> Result<HttpResponse, MessagingError> {
    body.validate().map_err(|e| MessagingError::Validation(e.to_string()))?;

    let result = state.messaging_service.send_message(
        body.sender_id,
        *conversation_id,
        &body.content,
        &body.message_type,
        body.reply_to_id,
    ).await?;

    Ok(HttpResponse::Created().json(result))
}

/// Edit a message.
pub async fn edit_message(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<EditMessageRequest>,
) -> Result<HttpResponse, MessagingError> {
    let (conversation_id, message_id) = path.into_inner();
    body.validate().map_err(|e| MessagingError::Validation(e.to_string()))?;

    let result = state.messaging_service.edit_message(
        body.user_id,
        conversation_id,
        message_id,
        &body.content,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Delete a message.
pub async fn delete_message(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<DeleteMessageRequest>,
) -> Result<HttpResponse, MessagingError> {
    let (conversation_id, message_id) = path.into_inner();

    state.messaging_service.delete_message(
        body.user_id,
        conversation_id,
        message_id,
    ).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Mark messages as read.
pub async fn mark_as_read(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<MarkAsReadRequest>,
) -> Result<HttpResponse, MessagingError> {
    let (_, conversation_id) = path.into_inner();

    state.messaging_service.mark_as_read(
        body.user_id,
        conversation_id,
    ).await?;

    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// Search and Unread Handlers
// =============================================================================

/// Search messages.
pub async fn search_messages(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    query: web::Query<SearchMessagesQuery>,
) -> Result<HttpResponse, MessagingError> {
    query.validate().map_err(|e| MessagingError::Validation(e.to_string()))?;

    let result = state.messaging_service.search_messages(
        *user_id,
        &query.q,
        query.conversation_id,
        query.page,
        query.per_page,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Get unread counts for a user.
pub async fn get_unread_counts(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, MessagingError> {
    let result = state.messaging_service.get_unread_counts(*user_id).await?;

    Ok(HttpResponse::Ok().json(result))
}

// =============================================================================
// Health Check
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "messaging-service"
    }))
}
