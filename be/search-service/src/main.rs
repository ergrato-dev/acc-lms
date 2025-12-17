//! # Search Service
//!
//! Full-text and semantic search for courses, lessons, and content.
//!
//! ## Features
//!
//! - Full-text search for courses (title, description)
//! - Semantic search using vector embeddings
//! - Autocomplete/suggestions
//! - Faceted filtering (category, level, price, rating)
//! - Personalized recommendations for authenticated users
//! - Search within enrolled course content
//!
//! ## Port: 8095

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
                .add_directive("search_service=debug".parse().unwrap())
        )
        .init();

    tracing::info!("Starting Search Service on port 8095");

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

    // OpenAI client for embeddings
    let openai_client = async_openai::Client::new();

    // Initialize repositories
    let course_search_repo = repository::CourseSearchRepository::new(pool.clone());
    let content_search_repo = repository::ContentSearchRepository::new(pool.clone());
    let embedding_repo = repository::EmbeddingRepository::new(pool.clone());

    // Initialize services
    let semantic_search_service = services::SemanticSearchService::new(
        embedding_repo.clone(),
        openai_client,
    );
    let search_service = services::SearchService::new(
        course_search_repo.clone(),
        semantic_search_service.clone(),
        redis_client.clone(),
    );
    let content_search_service = services::ContentSearchService::new(
        content_search_repo.clone(),
    );
    let suggestion_service = services::SuggestionService::new(
        pool.clone(),
        redis_client.clone(),
    );

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
            .app_data(web::Data::new(search_service.clone()))
            .app_data(web::Data::new(semantic_search_service.clone()))
            .app_data(web::Data::new(content_search_service.clone()))
            .app_data(web::Data::new(suggestion_service.clone()))
            .configure(api::configure_routes)
    })
    .bind("0.0.0.0:8095")?
    .run()
    .await
}
