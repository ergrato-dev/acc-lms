//! # Route Configuration
//!
//! Actix-web route configuration for enrollment endpoints.

use actix_web::web;

use super::handlers;

/// Configures all enrollment routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // User enrollments
            .service(
                web::scope("/enrollments")
                    .route("", web::get().to(handlers::list_my_enrollments))
                    .route("", web::post().to(handlers::enroll))
                    .route("/{enrollment_id}", web::get().to(handlers::get_enrollment))
                    .route("/{enrollment_id}/progress", web::get().to(handlers::get_enrollment_with_progress))
                    .route("/{enrollment_id}/status", web::patch().to(handlers::update_enrollment_status))
                    .route("/{enrollment_id}/certificate", web::post().to(handlers::issue_certificate))
                    // Lesson progress
                    .route("/{enrollment_id}/lessons", web::get().to(handlers::get_lesson_progress))
                    .route("/{enrollment_id}/lessons/start", web::post().to(handlers::start_lesson))
                    .route("/{enrollment_id}/lessons/progress", web::patch().to(handlers::update_progress))
                    .route("/{enrollment_id}/lessons/complete", web::post().to(handlers::complete_lesson))
                    .route("/{enrollment_id}/lessons/position", web::post().to(handlers::save_position))
            )
            // Course-scoped endpoints
            .service(
                web::scope("/courses/{course_id}")
                    .route("/enrollments", web::get().to(handlers::list_course_enrollments))
                    .route("/enrollment/check", web::get().to(handlers::check_enrollment))
                    .route("/stats", web::get().to(handlers::get_course_stats))
            )
            // User stats
            .service(
                web::scope("/users/{user_id}")
                    .route("/stats", web::get().to(handlers::get_user_stats))
            )
            // Me endpoints
            .service(
                web::scope("/me")
                    .route("/stats", web::get().to(handlers::get_my_stats))
            )
    );
}
