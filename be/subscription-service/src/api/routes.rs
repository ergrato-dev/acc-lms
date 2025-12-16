//! Routes configuration for subscription service

use actix_web::web;
use crate::api::handlers;

/// Configure all routes for the subscription service
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Health check
            .route("/health", web::get().to(handlers::health_check))

            // Plans (public)
            .service(
                web::scope("/plans")
                    .route("", web::get().to(handlers::get_plans))
                    .route("/{plan_id}", web::get().to(handlers::get_plan))
                    .route("/slug/{slug}", web::get().to(handlers::get_plan_by_slug))
                    // Admin only
                    .route("", web::post().to(handlers::create_plan))
            )

            // Subscriptions (authenticated)
            .service(
                web::scope("/subscriptions")
                    .route("", web::post().to(handlers::create_subscription))
                    .route("/me", web::get().to(handlers::get_my_subscription))
                    .route("/{subscription_id}", web::get().to(handlers::get_subscription))
                    .route("/{subscription_id}", web::patch().to(handlers::change_plan))
                    .route("/{subscription_id}/cancel", web::post().to(handlers::cancel_subscription))
                    .route("/{subscription_id}/reactivate", web::post().to(handlers::reactivate_subscription))
            )

            // Invoices (authenticated)
            .service(
                web::scope("/invoices")
                    .route("", web::get().to(handlers::list_invoices))
                    .route("/{invoice_id}", web::get().to(handlers::get_invoice))
            )

            // Payment Methods (authenticated)
            .service(
                web::scope("/payment-methods")
                    .route("", web::get().to(handlers::list_payment_methods))
                    .route("", web::post().to(handlers::add_payment_method))
                    .route("/{payment_method_id}", web::delete().to(handlers::delete_payment_method))
            )

            // Coupons
            .service(
                web::scope("/coupons")
                    .route("", web::get().to(handlers::get_coupons))
                    .route("", web::post().to(handlers::create_coupon))
                    .route("/validate", web::post().to(handlers::validate_coupon))
            )

            // Usage (authenticated)
            .service(
                web::scope("/usage")
                    .route("", web::post().to(handlers::record_usage))
            )

            // Billing Events
            .route("/billing-events", web::get().to(handlers::list_billing_events))

            // Stripe Webhook
            .route("/webhooks/stripe", web::post().to(handlers::stripe_webhook))
    );
}
