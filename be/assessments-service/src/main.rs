//! # Assessments Service
//!
//! Quiz and assessment management microservice for ACC LMS.
//!
//! ## Features
//!
//! - Quiz CRUD with questions
//! - Quiz submissions and grading
//! - Auto-grading for objective questions
//! - Manual grading workflow for essays/code
//! - Quiz statistics

use std::sync::Arc;

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use api::configure_routes;
use api::handlers::AppState;
use repository::AssessmentRepository;
use service::AssessmentService;

/// Application configuration.
#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    host: String,
    port: u16,
    max_connections: u32,
}

impl Config {
    /// Loads configuration from environment variables.
    fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://assessments_svc:password@localhost:5432/acc_lms".into()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8005),
            max_connections: std::env::var("MAX_CONNECTIONS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(10),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "assessments_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    info!("Starting Assessments Service on {}:{}", config.host, config.port);

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection established");

    // Create repository and service
    let repository = Arc::new(AssessmentRepository::new(pool));
    let service = AssessmentService::new(repository);

    // Create app state
    let app_state = web::Data::new(AppState { service });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Service", "assessments-service"))
            )
            // Add app state
            .app_data(app_state.clone())
            // Configure routes
            .configure(configure_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
