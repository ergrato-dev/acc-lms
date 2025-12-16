//! # ACC LMS - Analytics Service
//!
//! Microservice for learning analytics, event tracking, session management,
//! and reporting using PostgreSQL for storage.
//!
//! ## Features
//!
//! - Event tracking (page views, clicks, course events, etc.)
//! - Session management
//! - Platform and course analytics
//! - User engagement metrics
//! - Time series data
//! - Aggregated metrics and reporting
//!
//! ## API Endpoints
//!
//! ### Events
//! - POST   /api/v1/events           - Track single event
//! - POST   /api/v1/events/batch     - Track multiple events
//! - GET    /api/v1/events           - Query events
//! - GET    /api/v1/events/{id}      - Get event by ID
//!
//! ### Sessions
//! - POST   /api/v1/sessions           - Start session
//! - GET    /api/v1/sessions/{id}      - Get session
//! - PUT    /api/v1/sessions/{id}/end  - End session
//! - GET    /api/v1/sessions/active/count - Active sessions count
//!
//! ### Analytics
//! - GET    /api/v1/analytics/events/counts     - Event counts by type
//! - GET    /api/v1/analytics/events/timeseries - Event time series
//! - GET    /api/v1/analytics/platform          - Platform stats
//! - GET    /api/v1/analytics/courses/top       - Top courses
//! - GET    /api/v1/analytics/courses/{id}      - Course analytics
//! - GET    /api/v1/analytics/users/{id}        - User engagement
//! - GET    /api/v1/analytics/pages/top         - Top pages
//!
//! ### Health
//! - GET    /health - Health check
//! - GET    /ready  - Readiness check

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use api::handlers::AppState;
use repository::AnalyticsRepository;
use service::AnalyticsService;

/// Server configuration.
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    database_url: String,
    max_connections: u32,
}

impl Config {
    /// Loads configuration from environment variables.
    fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8085".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "analytics_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    info!("Starting Analytics Service...");
    info!("Host: {}:{}", config.host, config.port);

    // Create database pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection pool created");

    // Create repository and service
    let repository = AnalyticsRepository::new(pool);
    let service = Arc::new(AnalyticsService::new(repository));

    // Create app state
    let app_state = web::Data::new(AppState {
        analytics_service: service,
    });

    info!("Starting HTTP server on {}:{}", config.host, config.port);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // Add middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            // Add app state
            .app_data(app_state.clone())
            // Configure routes
            .configure(api::routes::configure)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
