//! # Certificates Service
//!
//! Certificate generation and verification service (RF-STU-021).
//!
//! ## Features
//! - Generate PDF certificates on course completion
//! - QR code for verification
//! - Public verification endpoint
//! - Certificate templates per course

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

mod api;
mod domain;
mod repository;
mod services;

use api::handlers::AppState;
use repository::CertificatesRepository;
use services::CertificatesService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("certificates_service=debug".parse().unwrap()),
        )
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/acc_lms".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL database");

    // Redis connection
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let _redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");

    tracing::info!("Connected to Redis");

    // Base URL for verification links
    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "https://acc-lms.com".to_string());

    // Initialize repository and service
    let repository = CertificatesRepository::new(pool);
    let certificates_service = std::sync::Arc::new(
        CertificatesService::new(repository, base_url)
    );

    let app_state = web::Data::new(AppState {
        certificates_service,
    });

    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8100".to_string());

    tracing::info!("Starting Certificates Service on {}", bind_address);

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
            .configure(api::configure_routes)
            .route("/health", web::get().to(api::handlers::health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}
