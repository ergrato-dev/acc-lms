//! # Analytics API Routes
//!
//! Route configuration for the analytics service.

use actix_web::web;

use crate::api::handlers;

/// Configures all API routes.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check endpoints (no /api/v1 prefix)
        .route("/health", web::get().to(handlers::health_check))
        .route("/ready", web::get().to(handlers::readiness_check))
        // API v1 routes
        .service(
            web::scope("/api/v1")
                // Event tracking
                .service(
                    web::scope("/events")
                        .route("", web::post().to(handlers::track_event))
                        .route("", web::get().to(handlers::query_events))
                        .route("/batch", web::post().to(handlers::track_events_batch))
                        .route("/{event_id}", web::get().to(handlers::get_event))
                )
                // Session management
                .service(
                    web::scope("/sessions")
                        .route("", web::post().to(handlers::start_session))
                        .route("/active/count", web::get().to(handlers::get_active_sessions_count))
                        .route("/{session_id}", web::get().to(handlers::get_session))
                        .route("/{session_id}/end", web::put().to(handlers::end_session))
                )
                // Analytics
                .service(
                    web::scope("/analytics")
                        // Event analytics
                        .route("/events/counts", web::get().to(handlers::get_event_counts))
                        .route("/events/timeseries", web::get().to(handlers::get_event_time_series))
                        // Platform analytics
                        .route("/platform", web::get().to(handlers::get_platform_stats))
                        // Course analytics
                        .route("/courses/top", web::get().to(handlers::get_top_courses))
                        .route("/courses/{course_id}", web::get().to(handlers::get_course_analytics))
                        // User analytics
                        .route("/users/{user_id}", web::get().to(handlers::get_user_engagement))
                        // Page analytics
                        .route("/pages/top", web::get().to(handlers::get_top_pages))
                )
        );
}
