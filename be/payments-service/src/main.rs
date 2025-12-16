//! # Payments Service
//!
//! Microservice for order processing, transactions, discount codes, and reviews.
//!
//! ## Architecture
//!
//! This service follows Clean Architecture principles:
//!
//! - **Domain**: Core business entities and value objects
//! - **Repository**: Data access layer with PostgreSQL
//! - **Service**: Business logic orchestration
//! - **API**: HTTP handlers and routing
//!
//! ## Features
//!
//! - Order lifecycle management (create, pay, refund, cancel)
//! - Transaction processing and tracking
//! - Discount code management and validation
//! - Course reviews with rating statistics

pub mod api;
pub mod domain;
pub mod repository;
pub mod service;

use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::configure_routes;
use crate::api::handlers::AppState;
use crate::repository::PaymentRepository;
use crate::service::PaymentService;

/// Main entry point for the payments service.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "payments_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting payments-service...");

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/acc_lms".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8005);

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Connected to database");

    // Create application state
    let repository = Arc::new(PaymentRepository::new(pool));
    let service = PaymentService::new(repository);
    let app_state = web::Data::new(AppState { service });

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
