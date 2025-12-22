//! # Reviews Service
//!
//! Course reviews and ratings management.
//!
//! ## Features
//!
//! - Create, update, delete reviews for enrolled courses
//! - Star ratings (1-5) with text comments
//! - Helpful votes on reviews
//! - Course rating aggregation
//! - Instructor rating calculation
//! - Review moderation and reporting
//!
//! ## Port: 8097

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;
use sqlx::postgres::PgPoolOptions;

mod api;
mod domain;
mod repository;
mod services;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("reviews_service=debug".parse().unwrap())
        )
        .init();

    tracing::info!("Starting Reviews Service on port 8097");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acclms".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Redis connection for caching
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");

    // Initialize repository and service
    let reviews_repo = repository::ReviewsRepository::new(pool.clone());
    let reviews_service = services::ReviewsService::new(reviews_repo, redis_client);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(reviews_service.clone()))
            .configure(api::configure_routes)
    })
    .bind("0.0.0.0:8097")?
    .run()
    .await
}
