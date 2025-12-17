//! API Module
//!
//! HTTP handlers and DTOs for AI Service

pub mod dto;
pub mod handlers;

pub use dto::*;
pub use handlers::*;

use std::sync::Arc;
use crate::service::{TutorService, SemanticSearchService, SummaryService, QuizGeneratorService};

/// Application state shared across handlers.
pub struct AppState {
    pub tutor_service: Arc<TutorService>,
    pub search_service: Arc<SemanticSearchService>,
    pub summary_service: Arc<SummaryService>,
    pub quiz_generator_service: Arc<QuizGeneratorService>,
}

/// Configures all routes for the AI service.
pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api/v1")
            // Health check
            .route("/health", actix_web::web::get().to(handlers::health_check))

            // Tutor endpoints
            .service(
                actix_web::web::scope("/tutor")
                    .route("/sessions", actix_web::web::post().to(handlers::create_tutor_session))
                    .route("/sessions/{session_id}", actix_web::web::get().to(handlers::get_tutor_session))
                    .route("/sessions/{session_id}/messages", actix_web::web::post().to(handlers::send_tutor_message))
                    .route("/sessions/{session_id}/messages", actix_web::web::get().to(handlers::get_session_messages))
                    .route("/sessions/{session_id}/stream", actix_web::web::post().to(handlers::send_tutor_message_stream))
            )

            // Semantic search endpoints
            .service(
                actix_web::web::scope("/search")
                    .route("/semantic", actix_web::web::post().to(handlers::semantic_search))
                    .route("/courses/{course_id}/index", actix_web::web::post().to(handlers::index_course_content))
            )

            // Content generation endpoints
            .service(
                actix_web::web::scope("/generate")
                    .route("/summary", actix_web::web::post().to(handlers::generate_summary))
                    .route("/key-points", actix_web::web::post().to(handlers::generate_key_points))
                    .route("/glossary", actix_web::web::post().to(handlers::generate_glossary))
                    .route("/status/{request_id}", actix_web::web::get().to(handlers::get_generation_status))
            )

            // Quiz generation endpoints
            .service(
                actix_web::web::scope("/quizzes")
                    .route("/generate", actix_web::web::post().to(handlers::generate_quiz))
                    .route("/{request_id}", actix_web::web::get().to(handlers::get_generated_quiz))
                    .route("/{request_id}/questions", actix_web::web::get().to(handlers::get_generated_questions))
            )

            // Usage/quota endpoints
            .service(
                actix_web::web::scope("/usage")
                    .route("/quota", actix_web::web::get().to(handlers::get_usage_quota))
            )
    );
}
