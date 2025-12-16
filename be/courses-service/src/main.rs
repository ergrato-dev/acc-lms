//! # Courses Service
//!
//! Microservice for course management in ACC LMS platform.
//!
//! ## Features
//! - Course CRUD operations
//! - Section and lesson management
//! - Course publishing workflow
//! - Category management
//!
//! ## Architecture
//! This service follows Clean Architecture with:
//! - Domain layer: entities, events, value objects
//! - Repository layer: PostgreSQL data access
//! - Service layer: business logic
//! - API layer: HTTP handlers and routes

use std::env;
use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use repository::CourseRepository;
use service::CourseService;

/// Application state shared across handlers.
pub struct AppState {
    pub course_service: Arc<CourseService>,
}

/// Service configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub max_db_connections: u32,
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acc_lms".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8082".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            max_db_connections: env::var("MAX_DB_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("MAX_DB_CONNECTIONS must be a valid u32"),
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,courses_service=debug,sqlx=warn".to_string()),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.rust_log.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Courses Service...");
    info!("Configuration: host={}, port={}", config.host, config.port);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection pool created");

    // Initialize repository
    let repository = CourseRepository::new(pool);

    // Initialize service
    let course_service = Arc::new(CourseService::new(repository));

    // Create application state
    let app_state = web::Data::new(AppState { course_service });

    // Start HTTP server
    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Binding to {}", bind_address);

    HttpServer::new(move || {
        App::new()
            // Application state
            .app_data(app_state.clone())
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Service", "courses-service"))
                    .add(("X-Version", "1.0.0")),
            )
            // Configure routes
            .configure(api::routes::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // Clear env vars for test
        env::remove_var("DATABASE_URL");
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("MAX_DB_CONNECTIONS");
        env::remove_var("RUST_LOG");

        let config = Config::from_env();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8082);
        assert_eq!(config.max_db_connections, 10);
        assert!(config.database_url.contains("postgres"));
    }
}
