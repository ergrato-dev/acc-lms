//! # Notifications Service
//!
//! Microservice for managing notifications including:
//! - Notification templates
//! - Notification queue and delivery
//! - User notification preferences
//!
//! ## Architecture
//!
//! This service follows Clean Architecture principles:
//! - **Domain**: Core entities, events, and value objects
//! - **Repository**: Data access layer
//! - **Service**: Business logic
//! - **API**: HTTP handlers and routes

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use api::handlers::AppState;
use repository::NotificationRepository;
use service::NotificationService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info,notifications_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Notifications Service...");

    // Load configuration
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acc_lms".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8084".to_string())
        .parse()
        .expect("PORT must be a valid number");
    let max_retries: i32 = env::var("MAX_RETRIES")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .expect("MAX_RETRIES must be a valid number");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Create repository and service
    let repository = NotificationRepository::new(pool);
    let service = NotificationService::with_config(repository, max_retries, 5);

    // Create app state
    let app_state = web::Data::new(AppState { service });

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    // Start server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(api::configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
