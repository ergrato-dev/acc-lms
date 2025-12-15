//! # ACC LMS - Users Service
//!
//! Microservice for user profile management. See `auth-service` for authentication.
//!
//! ## Planned Features
//!
//! - User profile CRUD
//! - Avatar management
//! - Preferences management
//! - User search and filtering

//! # ACC LMS - Users Service
//!
//! Microservice responsible for user profile management, preferences, and avatar handling.
//! This service is separate from `auth-service` to follow single responsibility principle:
//! - **auth-service**: Authentication, authorization, tokens
//! - **users-service**: Profile data, preferences, settings
//!
//! ## Features
//!
//! - **Profile Management**: CRUD operations for user profiles
//! - **Preferences**: Language, timezone, notification settings, privacy, accessibility
//! - **Avatar Upload**: Image upload with validation and optimization
//! - **User Stats**: Aggregated statistics (courses enrolled, completed, etc.)
//!
//! ## API Endpoints
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | GET | `/api/v1/users/:id` | Get user profile by ID |
//! | PATCH | `/api/v1/users/:id` | Update user profile |
//! | GET | `/api/v1/users/:id/preferences` | Get user preferences |
//! | PATCH | `/api/v1/users/:id/preferences` | Update user preferences |
//! | POST | `/api/v1/users/:id/avatar` | Upload avatar |
//! | DELETE | `/api/v1/users/:id/avatar` | Remove avatar |
//! | GET | `/api/v1/users/search` | Search users (admin/instructor) |
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        users-service                            │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  ┌─────────────┐   ┌─────────────┐   ┌─────────────────────┐   │
//! │  │    API      │   │   Service   │   │    Repository       │   │
//! │  │  handlers   │ → │    Layer    │ → │  (PostgreSQL)       │   │
//! │  └─────────────┘   └─────────────┘   └─────────────────────┘   │
//! │         │                 │                    │               │
//! │         │                 │                    ▼               │
//! │         │                 │          ┌─────────────────────┐   │
//! │         │                 └────────→ │   Storage (MinIO)   │   │
//! │         │                            │   (for avatars)     │   │
//! │         │                            └─────────────────────┘   │
//! │         │                                                      │
//! │         ▼                                                      │
//! │  ┌─────────────────────────────────────────────────────────────┐   │
//! │  │                    Domain Events                        │   │
//! │  │  • user.profile_updated                                 │   │
//! │  │  • user.preferences_changed                             │   │
//! │  │  • user.avatar_updated                                  │   │
//! │  └─────────────────────────────────────────────────────────────┘   │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Configuration
//!
//! Environment variables:
//! - `DATABASE_URL`: PostgreSQL connection string
//! - `REDIS_URL`: Redis connection string
//! - `MINIO_ENDPOINT`: MinIO/S3 endpoint for avatar storage
//! - `MINIO_ACCESS_KEY`: MinIO access key
//! - `MINIO_SECRET_KEY`: MinIO secret key
//! - `AVATAR_BUCKET`: Bucket name for avatars
//! - `JWT_SECRET`: Secret for JWT validation (shared with auth-service)

use actix_web::{middleware, web, App, HttpServer};
use shared::{
    config::AppConfig,
    database::create_pool,
    tracing_config::init_tracing,
};
use std::sync::Arc;
use tracing::info;

mod api;
mod domain;
mod repository;
mod service;

use api::routes::configure_routes;
use repository::UserProfileRepository;
use service::UserService;

// =============================================================================
// APPLICATION STATE
// =============================================================================

/// Shared application state accessible by all handlers.
///
/// Contains all dependencies needed for request processing:
/// - Database connections
/// - Service instances
/// - Configuration
///
/// # Thread Safety
///
/// All fields are wrapped in `Arc` or are inherently thread-safe,
/// making `AppState` safe to share across Actix-web workers.
#[derive(Clone)]
pub struct AppState {
    /// User service for business logic
    pub user_service: Arc<UserService>,
    
    /// Service configuration
    pub config: Arc<AppConfig>,
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

/// Main entry point for the users-service.
///
/// Initializes:
/// 1. Tracing/logging
/// 2. Configuration
/// 3. Database connection pool
/// 4. Service dependencies
/// 5. HTTP server with routes
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // -------------------------------------------------------------------------
    // Initialize Tracing
    // -------------------------------------------------------------------------
    // Second parameter: is_production (false for development)
    init_tracing("users-service", false);
    
    info!("Starting ACC LMS Users Service");
    
    // -------------------------------------------------------------------------
    // Load Configuration
    // -------------------------------------------------------------------------
    let config = AppConfig::from_env()
        .expect("Failed to load configuration");
    
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    
    info!(
        host = %config.server.host,
        port = %config.server.port,
        "Configuration loaded"
    );
    
    // -------------------------------------------------------------------------
    // Initialize Database
    // -------------------------------------------------------------------------
    let db_pool = create_pool(&config.database)
        .await
        .expect("Failed to create database pool");
    
    info!("Database connection pool created");
    
    // -------------------------------------------------------------------------
    // Initialize Services
    // -------------------------------------------------------------------------
    let repository = UserProfileRepository::new(db_pool);
    let user_service = Arc::new(UserService::new(repository));
    
    // -------------------------------------------------------------------------
    // Create Application State
    // -------------------------------------------------------------------------
    let app_state = AppState {
        user_service,
        config: Arc::new(config),
    };
    
    // -------------------------------------------------------------------------
    // Start HTTP Server
    // -------------------------------------------------------------------------
    info!(address = %bind_address, "Starting HTTP server");
    
    HttpServer::new(move || {
        App::new()
            // Share state across handlers
            .app_data(web::Data::new(app_state.clone()))
            // Logging middleware
            .wrap(middleware::Logger::default())
            // Compression middleware
            .wrap(middleware::Compress::default())
            // Configure routes
            .configure(configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
