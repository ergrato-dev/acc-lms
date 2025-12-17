//! AI Service - Entry point
//!
//! Starts the Actix-web HTTP server for AI-powered educational features

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ai_service::{
    api::{configure_routes, AppState},
    repository::{
        ConversationRepository, EmbeddingRepository,
        SummaryRepository, QuizGenerationRepository,
    },
    service::{
        TutorService, SemanticSearchService,
        SummaryService, QuizGeneratorService,
    },
    llm::OpenAIClient,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,ai_service=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AI Service...");

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let openai_api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8093".to_string())
        .parse()
        .expect("PORT must be a valid number");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection established");

    // Create LLM client
    let llm_client = Arc::new(OpenAIClient::new(&openai_api_key));

    // Create repositories
    let conversation_repo = ConversationRepository::new(pool.clone());
    let embedding_repo = EmbeddingRepository::new(pool.clone());
    let summary_repo = SummaryRepository::new(pool.clone());
    let quiz_gen_repo = QuizGenerationRepository::new(pool.clone());

    // Create services
    let tutor_service = Arc::new(TutorService::new(
        conversation_repo,
        embedding_repo.clone(),
        llm_client.clone(),
    ));
    let search_service = Arc::new(SemanticSearchService::new(
        embedding_repo.clone(),
        llm_client.clone(),
    ));
    let summary_service = Arc::new(SummaryService::new(
        summary_repo,
        llm_client.clone(),
    ));
    let quiz_generator_service = Arc::new(QuizGeneratorService::new(
        quiz_gen_repo,
        llm_client.clone(),
    ));

    // Create app state
    let app_state = web::Data::new(AppState {
        tutor_service,
        search_service,
        summary_service,
        quiz_generator_service,
    });

    tracing::info!("Starting HTTP server on {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Service", "ai-service"))
                    .add(("X-Version", env!("CARGO_PKG_VERSION")))
            )
            .configure(configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
