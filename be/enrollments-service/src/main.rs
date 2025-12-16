//! # ACC LMS - Enrollments Service
//!
//! Microservice for course enrollment management.
//!
//! ## Features
//!
//! - Enrollment CRUD operations
//! - Lesson progress tracking
//! - Course completion detection
//! - Certificate issuance
//! - Learning statistics

use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use api::{configure_routes, AppState};
use repository::EnrollmentRepository;
use service::EnrollmentService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,enrollments_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting ACC LMS Enrollments Service");

    // Load configuration from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8083".into())
        .parse()
        .expect("PORT must be a valid number");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Connected to database");

    // Run migrations (optional in production)
    // sqlx::migrate!("../db/migrations/postgresql")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    // Build application state
    let repository = Arc::new(EnrollmentRepository::new(pool));
    let enrollment_service = EnrollmentService::new(repository);

    let app_state = web::Data::new(AppState {
        enrollment_service,
    });

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(configure_routes)
            // Health check endpoint
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
