//! # Route Configuration
//!
//! Actix-web route definitions for assessment endpoints.

use actix_web::web;

use crate::api::handlers;

/// Configures all assessment routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health_check))

        // Quiz routes
        .service(
            web::scope("/quizzes")
                .route("", web::post().to(handlers::create_quiz))
                .route("/{quiz_id}", web::get().to(handlers::get_quiz))
                .route("/{quiz_id}", web::patch().to(handlers::update_quiz))
                .route("/{quiz_id}", web::delete().to(handlers::delete_quiz))
                .route("/{quiz_id}/full", web::get().to(handlers::get_quiz_with_questions))
                .route("/{quiz_id}/questions", web::get().to(handlers::list_quiz_questions))
                .route("/{quiz_id}/publish", web::post().to(handlers::publish_quiz))
                .route("/{quiz_id}/stats", web::get().to(handlers::get_quiz_stats))
                .route("/{quiz_id}/start", web::post().to(handlers::start_quiz))
                .route("/{quiz_id}/my-submissions", web::get().to(handlers::get_my_submissions))
        )

        // Course quizzes
        .route("/courses/{course_id}/quizzes", web::get().to(handlers::list_course_quizzes))

        // Lesson quizzes
        .route("/lessons/{lesson_id}/quizzes", web::get().to(handlers::get_lesson_quizzes))

        // Question routes
        .service(
            web::scope("/questions")
                .route("", web::post().to(handlers::create_question))
                .route("/{question_id}", web::patch().to(handlers::update_question))
                .route("/{question_id}", web::delete().to(handlers::delete_question))
        )

        // Submission routes
        .service(
            web::scope("/submissions")
                .route("/{submission_id}", web::get().to(handlers::get_submission))
                .route("/{submission_id}/full", web::get().to(handlers::get_submission_with_responses))
                .route("/{submission_id}/answers", web::post().to(handlers::save_answer))
                .route("/{submission_id}/submit", web::post().to(handlers::submit_quiz))
                .route("/{submission_id}/grade", web::post().to(handlers::grade_submission))
        )

        // Response routes
        .service(
            web::scope("/responses")
                .route("/{response_id}/grade", web::post().to(handlers::grade_response))
        );
}
