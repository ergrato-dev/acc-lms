// =============================================================================
// ACC LMS - Content Service Entry Point
// =============================================================================
// Servicio de gestión de contenido multimedia con almacenamiento híbrido
// Soporta: LocalStorage (desarrollo/low-budget) + trait para S3/MinIO futuro
// =============================================================================

use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;
mod storage;

use repository::ContentRepository;
use service::ContentService;
use storage::{LocalStorage, LocalStorageConfig, StorageBackend};

#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server
    pub host: String,
    pub port: u16,

    // Database
    pub database_url: String,

    // Storage
    pub storage_type: String,
    pub local_storage_path: String,
    pub local_storage_base_url: String,
    pub max_file_size: u64,

    // JWT
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/acc_lms".to_string()),
            storage_type: std::env::var("STORAGE_TYPE")
                .unwrap_or_else(|_| "local".to_string()),
            local_storage_path: std::env::var("LOCAL_STORAGE_PATH")
                .unwrap_or_else(|_| "./uploads".to_string()),
            local_storage_base_url: std::env::var("LOCAL_STORAGE_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8083/uploads".to_string()),
            max_file_size: std::env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "2147483648".to_string()) // 2GB default
                .parse()
                .expect("MAX_FILE_SIZE must be a number"),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "development-secret-change-in-production".to_string()),
        }
    }
}

fn create_storage(config: &AppConfig) -> Result<Arc<dyn StorageBackend>, Box<dyn std::error::Error + Send + Sync>> {
    match config.storage_type.as_str() {
        "local" => {
            let storage_config = LocalStorageConfig {
                base_path: std::path::PathBuf::from(&config.local_storage_path),
                base_url: config.local_storage_base_url.clone(),
                upload_endpoint: format!("http://{}:{}/api/v1/content/upload", config.host, config.port),
                max_file_size: config.max_file_size,
            };

            let storage = LocalStorage::new(storage_config)?;
            Ok(Arc::new(storage))
        }
        // Futuro: "s3" | "minio" => { ... }
        other => {
            error!("Unknown storage type: {}", other);
            Err(format!("Unknown storage type: {}", other).into())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "content_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = AppConfig::from_env();

    info!(
        "Starting content-service on {}:{} with {} storage",
        config.host, config.port, config.storage_type
    );

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    info!("Database connected");

    // Create storage backend
    let storage = create_storage(&config)
        .expect("Failed to initialize storage");

    info!("Storage initialized: {}", config.storage_type);

    // Create repository and service
    let repository = Arc::new(ContentRepository::new(pool.clone()));
    let content_service = Arc::new(ContentService::new(repository.clone(), storage.clone()));

    let bind_addr = format!("{}:{}", config.host, config.port);

    // Configure static file serving for uploads (LocalStorage)
    let uploads_path = config.local_storage_path.clone();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())

            // CORS
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )

            // Inject service
            .app_data(web::Data::new(content_service.clone()))

            // Payload config (for large uploads)
            .app_data(web::PayloadConfig::new(2 * 1024 * 1024 * 1024)) // 2GB

            // Static files for uploads
            .service(
                actix_files::Files::new("/uploads", &uploads_path)
                    .show_files_listing()
                    .use_etag(true)
            )

            // API routes
            .configure(api::routes::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
