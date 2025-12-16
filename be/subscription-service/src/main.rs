//! Subscription Service - Entry point
//!
//! Starts the Actix-web HTTP server for subscription management

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use subscription_service::{
    api::{configure_routes, AppState},
    repository::{SubscriptionRepository, BillingRepository, CouponRepository},
    service::{SubscriptionService, BillingService, CouponService},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,subscription_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Subscription Service...");

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8092".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection established");

    // Create repositories
    let subscription_repo = SubscriptionRepository::new(pool.clone());
    let billing_repo = BillingRepository::new(pool.clone());
    let coupon_repo = CouponRepository::new(pool.clone());

    // Create services
    let subscription_service = Arc::new(SubscriptionService::new(subscription_repo));
    let billing_service = Arc::new(BillingService::new(billing_repo));
    let coupon_service = Arc::new(CouponService::new(coupon_repo));

    // Create app state
    let app_state = web::Data::new(AppState {
        subscription_service,
        billing_service,
        coupon_service,
    });

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Service", "subscription-service"))
                    .add(("X-Version", env!("CARGO_PKG_VERSION")))
            )
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
