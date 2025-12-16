//! # Routes Configuration
//!
//! Configures all API routes for the payments service.

use actix_web::web;

use crate::api::handlers;

/// Configures all routes for the payments service.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health_check))
            // Order routes
            .service(
                web::scope("/orders")
                    .route("", web::get().to(handlers::list_orders))
                    .route("", web::post().to(handlers::create_order))
                    .route("/stats", web::get().to(handlers::get_order_stats))
                    .route("/number/{order_number}", web::get().to(handlers::get_order_by_number))
                    .route("/{order_id}", web::get().to(handlers::get_order))
                    .route("/{order_id}", web::put().to(handlers::update_order))
                    .route("/{order_id}/initiate", web::post().to(handlers::initiate_payment))
                    .route("/{order_id}/complete", web::post().to(handlers::complete_payment))
                    .route("/{order_id}/refund", web::post().to(handlers::process_refund))
                    .route("/{order_id}/cancel", web::post().to(handlers::cancel_order)),
            )
            // User order routes
            .service(
                web::scope("/users/{user_id}/orders")
                    .route("", web::get().to(handlers::list_user_orders)),
            )
            // Discount code routes
            .service(
                web::scope("/discount-codes")
                    .route("", web::get().to(handlers::list_discount_codes))
                    .route("", web::post().to(handlers::create_discount_code))
                    .route("/validate", web::post().to(handlers::validate_discount_code))
                    .route("/{code}", web::get().to(handlers::get_discount_code))
                    .route("/{code_id}", web::put().to(handlers::update_discount_code)),
            )
            // Review routes
            .service(
                web::scope("/reviews")
                    .route("", web::post().to(handlers::create_review))
                    .route("/{review_id}", web::get().to(handlers::get_review))
                    .route("/{review_id}/user/{user_id}", web::put().to(handlers::update_review))
                    .route("/{review_id}/user/{user_id}", web::delete().to(handlers::delete_review))
                    .route("/{review_id}/helpful", web::post().to(handlers::vote_helpful)),
            )
            // Course review routes
            .service(
                web::scope("/courses/{course_id}/reviews")
                    .route("", web::get().to(handlers::list_course_reviews))
                    .route("/stats", web::get().to(handlers::get_review_stats)),
            ),
    );
}
