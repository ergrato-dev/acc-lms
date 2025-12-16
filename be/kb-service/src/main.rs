// =============================================================================
// ACC LMS - Knowledge Base Service
// =============================================================================
// Servicio de base de conocimiento para artículos de ayuda, FAQ y
// documentación de soporte.
// =============================================================================

use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod domain;
mod repository;
mod service;

use repository::KbRepository;
use service::KbService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurar logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("📚 Starting ACC LMS Knowledge Base Service");

    // Cargar configuración
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acc_lms".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8091".to_string())
        .parse()
        .expect("PORT must be a number");

    // Pool de conexiones PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    info!("✅ Connected to PostgreSQL");

    // Inicializar capas
    let repository = Arc::new(KbRepository::new(pool));
    let service = Arc::new(KbService::new(repository.clone()));

    info!("🚀 Server starting on {}:{}", host, port);
    info!("📍 Health check: http://{}:{}/health", host, port);
    info!("📖 API base: http://{}:{}/api/v1/kb", host, port);

    // Servidor HTTP
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(service.clone()))
            .app_data(web::JsonConfig::default()
                .limit(5 * 1024 * 1024) // 5MB max para contenido de artículos
                .error_handler(|err, _req| {
                    let message = format!("JSON error: {}", err);
                    actix_web::error::InternalError::from_response(
                        err,
                        actix_web::HttpResponse::BadRequest()
                            .json(serde_json::json!({
                                "code": "INVALID_JSON",
                                "message": message
                            }))
                    ).into()
                }))
            .configure(api::configure_routes)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
