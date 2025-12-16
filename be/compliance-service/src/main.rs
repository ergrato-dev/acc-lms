// =============================================================================
// ACC LMS - Compliance Service
// =============================================================================
// Servicio de compliance para gestión de datos personales y cumplimiento
// regulatorio multi-jurisdiccional:
// - Colombia: Ley 1581/2012 (Habeas Data), derechos ARCO
// - EU: GDPR (Arts. 15-22)
// - California: CCPA/CPRA
// - Brasil: LGPD
// =============================================================================

use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod domain;
mod repository;
mod service;

use repository::ComplianceRepository;
use service::ComplianceService;

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

    info!("🔒 Starting ACC LMS Compliance Service");
    info!("📋 Jurisdictions: Colombia (ARCO), EU (GDPR), California (CCPA), Brazil (LGPD)");

    // Cargar configuración
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acc_lms".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8090".to_string())
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
    let repository = Arc::new(ComplianceRepository::new(pool));
    let service = Arc::new(ComplianceService::new(repository.clone()));

    info!("🚀 Server starting on {}:{}", host, port);
    info!("📍 Health check: http://{}:{}/health", host, port);
    info!("📖 API base: http://{}:{}/api/v1/compliance", host, port);

    // Servidor HTTP
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(service.clone()))
            .app_data(web::JsonConfig::default()
                .limit(1024 * 1024) // 1MB max
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
