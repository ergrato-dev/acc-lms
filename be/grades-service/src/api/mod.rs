//! # API Layer
//!
//! HTTP handlers, DTOs, and route configuration.

pub mod dto;
pub mod handlers;

use std::sync::Arc;
use actix_web::web;
use crate::service::{GradeService, TranscriptService, StatsService, ExportService};

pub use dto::*;

/// Application state shared across handlers.
pub struct AppState {
    pub grade_service: Arc<GradeService>,
    pub transcript_service: Arc<TranscriptService>,
    pub stats_service: Arc<StatsService>,
    pub export_service: Arc<ExportService>,
}

/// Configure API routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health_check))

            // Student grade endpoints
            .service(
                web::scope("/grades")
                    // Get my grades summary
                    .route("/my", web::get().to(handlers::get_my_grades))
                    // Get my grades for specific course
                    .route("/my/courses/{course_id}", web::get().to(handlers::get_my_course_grades))
                    // Get specific submission grade
                    .route("/submissions/{submission_id}", web::get().to(handlers::get_submission_grade))
            )

            // Transcript endpoints
            .service(
                web::scope("/transcript")
                    // Get my transcript
                    .route("/my", web::get().to(handlers::get_my_transcript))
                    // Export transcript
                    .route("/my/export", web::get().to(handlers::export_transcript))
            )

            // Instructor statistics endpoints
            .service(
                web::scope("/stats")
                    // Course statistics (instructor)
                    .route("/courses/{course_id}", web::get().to(handlers::get_course_stats))
                    // Quiz statistics (instructor)
                    .route("/quizzes/{quiz_id}", web::get().to(handlers::get_quiz_stats))
                    // Export course grades (instructor)
                    .route("/courses/{course_id}/export", web::get().to(handlers::export_course_grades))
            )

            // Admin endpoints
            .service(
                web::scope("/admin/grades")
                    // Get any student's grades (admin)
                    .route("/users/{user_id}", web::get().to(handlers::admin_get_user_grades))
                    // Export all grades for a course
                    .route("/courses/{course_id}/export", web::get().to(handlers::admin_export_course_grades))
            )
    );
}
