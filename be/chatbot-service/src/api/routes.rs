//! # Chatbot API Routes
//!
//! Route configuration for the chatbot service.

use actix_web::web;

use crate::api::handlers;

/// Configures all chatbot service routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Chatbot conversation routes
            .service(
                web::scope("/chatbot")
                    .route("/conversations", web::post().to(handlers::start_conversation))
                    .route("/conversations/{id}/messages", web::post().to(handlers::send_message))
                    .route("/conversations/{id}/history", web::get().to(handlers::get_history))
                    .route("/conversations/{id}/escalate", web::post().to(handlers::escalate_conversation))
                    .route("/conversations/{id}/end", web::put().to(handlers::end_conversation))
                    .route("/messages/{id}/feedback", web::post().to(handlers::add_feedback))
                    .route("/suggestions", web::get().to(handlers::get_suggestions))
                    .route("/analytics", web::get().to(handlers::get_analytics))
            )
            // Knowledge base routes
            .service(
                web::scope("/kb")
                    .route("/search", web::get().to(handlers::search_kb))
                    .route("/articles", web::post().to(handlers::create_article))
                    .route("/articles/popular", web::get().to(handlers::get_popular_articles))
                    .route("/articles/{slug}", web::get().to(handlers::get_article))
                    .route("/articles/{id}/feedback", web::post().to(handlers::record_article_feedback))
                    .route("/categories/{category}", web::get().to(handlers::get_articles_by_category))
            )
    )
    // Health routes
    .route("/health", web::get().to(handlers::health_check))
    .route("/ready", web::get().to(handlers::readiness_check));
}
