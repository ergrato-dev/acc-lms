//! # Grades Service
//!
//! Aggregates grades from assessments, calculates course averages,
//! generates transcripts, and provides export functionality.
//!
//! ## Features
//!
//! - Student grade summaries by course
//! - Course-wide grade statistics (for instructors)
//! - Academic transcript generation
//! - Grade export (CSV, JSON)
//! - Grade trend analytics
//!
//! ## Port: 8094

use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;
use sqlx::postgres::PgPoolOptions;

mod api;
mod domain;
mod repository;
mod service;

pub use domain::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("grades_service=debug".parse().unwrap())
        )
        .init();

    tracing::info!("Starting Grades Service on port 8094");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acclms".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Redis connection
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");

    let redis_conn = redis_client.get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    // Initialize repositories
    let grade_repo = Arc::new(repository::GradeRepository::new(pool.clone()));
    let transcript_repo = Arc::new(repository::TranscriptRepository::new(pool.clone()));
    let stats_repo = Arc::new(repository::StatsRepository::new(pool.clone()));

    // Initialize services
    let grade_service = Arc::new(service::GradeService::new(
        grade_repo.clone(),
        redis_conn.clone(),
    ));
    let transcript_service = Arc::new(service::TranscriptService::new(
        transcript_repo.clone(),
        grade_repo.clone(),
    ));
    let stats_service = Arc::new(service::StatsService::new(
        stats_repo.clone(),
        redis_conn.clone(),
    ));
    let export_service = Arc::new(service::ExportService::new(
        grade_repo.clone(),
        transcript_repo.clone(),
    ));

    // Create app state
    let app_state = api::AppState {
        grade_service,
        transcript_service,
        stats_service,
        export_service,
    };
    let state = web::Data::new(app_state);

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
            .app_data(state.clone())
            .configure(api::configure_routes)
    })
    .bind("0.0.0.0:8094")?
    .run()
    .await
}
