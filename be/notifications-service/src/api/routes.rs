//! # API Routes
//!
//! Route configuration for the notifications service.

use actix_web::web;

use crate::api::handlers;

/// Configures all routes for the notifications service.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health_check))
            // Template routes
            .service(
                web::scope("/templates")
                    .route("", web::post().to(handlers::create_template))
                    .route("", web::get().to(handlers::list_templates))
                    .route("/{id}", web::get().to(handlers::get_template))
                    .route("/{id}", web::put().to(handlers::update_template))
                    .route("/{id}", web::delete().to(handlers::delete_template))
                    .route("/{id}/deactivate", web::post().to(handlers::deactivate_template)),
            )
            // Notification routes
            .service(
                web::scope("/notifications")
                    .route("", web::post().to(handlers::create_notification))
                    .route("/send", web::post().to(handlers::send_notification))
                    .route("/stats", web::get().to(handlers::get_stats))
                    .route("/{id}", web::get().to(handlers::get_notification))
                    .route("/{id}/read", web::post().to(handlers::mark_as_read)),
            )
            // User-specific notification routes
            .service(
                web::scope("/users/{user_id}/notifications")
                    .route("", web::get().to(handlers::list_user_notifications))
                    .route("/unread-count", web::get().to(handlers::get_unread_count)),
            )
            // User settings routes
            .service(
                web::scope("/users/{user_id}/settings")
                    .route("", web::get().to(handlers::get_user_settings))
                    .route("", web::put().to(handlers::update_user_settings)),
            ),
    );
}
