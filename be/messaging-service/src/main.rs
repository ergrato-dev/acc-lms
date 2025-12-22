//! # Messaging Service
//!
//! User-to-user messaging with real-time support (RF-STU-021).
//!
//! ## Features
//! - Conversations between students and instructors
//! - Real-time message delivery
//! - Message history with pagination
//! - Read receipts and typing indicators

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;
use std::sync::Arc;

mod api;
mod domain;
mod repository;
mod services;

use repository::MessagingRepository;
use services::MessagingService;

/// Application state shared across handlers.
pub struct AppState {
    pub messaging_service: Arc<MessagingService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("messaging_service=debug".parse().unwrap()),
        )
        .init();

    dotenvy::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/acc_lms".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Connected to PostgreSQL database");

    // Redis connection
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let _redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");

    tracing::info!("Connected to Redis");

    // Initialize repository and service
    let repository = MessagingRepository::new(pool);
    let messaging_service = std::sync::Arc::new(MessagingService::new(repository));

    let app_state = web::Data::new(AppState {
        messaging_service,
    });

    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8099".to_string());

    tracing::info!("Starting Messaging Service on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            // Health check
            .route("/health", web::get().to(|| async { "OK" }))
            // REST API routes
            .configure(api::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
