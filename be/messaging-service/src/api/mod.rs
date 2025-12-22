//! # API Module
//!
//! HTTP API layer for messaging service.

pub mod dto;
pub mod handlers;

pub use dto::*;
pub use handlers::*;

use actix_web::web;

/// Configure API routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Conversations
            .route("/users/{user_id}/conversations", web::get().to(handlers::list_conversations))
            .route("/users/{user_id}/conversations", web::post().to(handlers::create_conversation))
            .route("/users/{user_id}/conversations/{conversation_id}", web::get().to(handlers::get_conversation))
            .route("/users/{user_id}/conversations/{conversation_id}", web::delete().to(handlers::leave_conversation))
            // Direct conversations
            .route("/users/{user_id}/conversations/direct/{other_user_id}", web::get().to(handlers::get_or_create_direct_conversation))
            // Messages
            .route("/users/{user_id}/conversations/{conversation_id}/messages", web::get().to(handlers::list_messages))
            .route("/conversations/{conversation_id}/messages", web::post().to(handlers::send_message))
            .route("/conversations/{conversation_id}/messages/{message_id}", web::put().to(handlers::edit_message))
            .route("/conversations/{conversation_id}/messages/{message_id}", web::delete().to(handlers::delete_message))
            // Read receipts
            .route("/users/{user_id}/conversations/{conversation_id}/read", web::post().to(handlers::mark_as_read))
            // Unread counts
            .route("/users/{user_id}/unread", web::get().to(handlers::get_unread_counts))
            // Search
            .route("/users/{user_id}/messages/search", web::get().to(handlers::search_messages))
            // Mute/Archive
            .route("/users/{user_id}/conversations/{conversation_id}/mute", web::post().to(handlers::mute_conversation))
            .route("/users/{user_id}/conversations/{conversation_id}/archive", web::post().to(handlers::archive_conversation)),
    );
}
