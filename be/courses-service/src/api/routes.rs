//! # API Routes Configuration
//!
//! Defines all HTTP routes for the courses service.

use actix_web::web;

use super::handlers;

/// Configures all routes for the courses service.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health_check))
        // Public API
        .service(
            web::scope("/api/v1")
                // Categories
                .route("/categories", web::get().to(handlers::list_categories))
                // Courses - public
                .route("/courses", web::get().to(handlers::list_courses))
                .route("/courses", web::post().to(handlers::create_course))
                .route("/courses/slug/{slug}", web::get().to(handlers::get_course_by_slug))
                .route("/courses/{id}", web::get().to(handlers::get_course))
                .route("/courses/{id}", web::patch().to(handlers::update_course))
                .route("/courses/{id}", web::delete().to(handlers::delete_course))
                // Course publishing
                .route("/courses/{id}/publish", web::post().to(handlers::publish_course))
                .route("/courses/{id}/unpublish", web::post().to(handlers::unpublish_course))
                // Sections
                .route(
                    "/courses/{course_id}/sections",
                    web::post().to(handlers::create_section),
                )
                .route(
                    "/courses/{course_id}/sections/{section_id}",
                    web::delete().to(handlers::delete_section),
                )
                // Lessons
                .route(
                    "/courses/{course_id}/lessons",
                    web::post().to(handlers::create_lesson),
                )
                .route(
                    "/courses/{course_id}/lessons/{lesson_id}",
                    web::patch().to(handlers::update_lesson),
                )
                .route(
                    "/courses/{course_id}/lessons/{lesson_id}",
                    web::delete().to(handlers::delete_lesson),
                )
                // Instructor routes
                .route(
                    "/instructor/courses",
                    web::get().to(handlers::list_instructor_courses),
                ),
        );
}
