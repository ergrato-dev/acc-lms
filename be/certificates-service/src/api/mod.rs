//! # API Module
//!
//! HTTP API layer for certificates service.

pub mod dto;
pub mod handlers;

use actix_web::web;

/// Configure API routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Certificate generation and management
            .route("/certificates", web::post().to(handlers::generate_certificate))
            .route("/users/{user_id}/certificates", web::get().to(handlers::list_user_certificates))
            .route("/certificates/{certificate_id}", web::get().to(handlers::get_certificate))
            .route("/certificates/{certificate_id}/pdf", web::get().to(handlers::download_certificate_pdf))
            .route("/certificates/{certificate_id}/revoke", web::post().to(handlers::revoke_certificate))
            // Public verification (no auth required)
            .route("/verify/{verification_code}", web::get().to(handlers::verify_certificate))
            // Templates (admin)
            .route("/templates", web::get().to(handlers::list_templates))
            .route("/templates", web::post().to(handlers::create_template))
            .route("/templates/{template_id}", web::get().to(handlers::get_template))
            .route("/templates/{template_id}", web::put().to(handlers::update_template))
            .route("/templates/{template_id}", web::delete().to(handlers::delete_template))
            // Check if certificate exists for user/course
            .route("/users/{user_id}/courses/{course_id}/certificate", web::get().to(handlers::get_user_course_certificate)),
    );
}
