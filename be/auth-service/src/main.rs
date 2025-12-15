//! # ACC LMS - Authentication Service
//!
//! The **auth-service** is a core microservice responsible for user authentication
//! and authorization in the ACC Learning Management System.
//!
//! ## Architecture Overview
//!
//! This service follows **Clean Architecture** principles with clear separation
//! between layers:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        HTTP Layer (api/)                        │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
//! │  │   routes    │  │  handlers   │  │    DTOs (Request/Res)   │ │
//! │  └──────┬──────┘  └──────┬──────┘  └────────────┬────────────┘ │
//! └─────────┼────────────────┼─────────────────────┼───────────────┘
//!           │                │                      │
//!           ▼                ▼                      ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Service Layer (service/)                     │
//! │  ┌────────────────────────────────────────────────────────────┐ │
//! │  │  AuthService: Registration, Login, Token Refresh, Logout   │ │
//! │  └─────────────────────────┬──────────────────────────────────┘ │
//! └─────────────────────────────┼───────────────────────────────────┘
//!                               │
//!                               ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                  Repository Layer (repository/)                 │
//! │  ┌───────────────────────────────────────────────────────────┐  │
//! │  │  UserRepository: CRUD operations for users & tokens       │  │
//! │  └──────────────────────────┬────────────────────────────────┘  │
//! └─────────────────────────────┼───────────────────────────────────┘
//!                               │
//!                               ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Domain Layer (domain/)                       │
//! │  ┌──────────────┐  ┌───────────────┐  ┌─────────────────────┐   │
//! │  │   Entities   │  │ Value Objects │  │   Domain Events     │   │
//! │  └──────────────┘  └───────────────┘  └─────────────────────┘   │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## API Endpoints
//!
//! All authentication endpoints are prefixed with `/api/v1/auth`:
//!
//! | Method | Endpoint        | Description                | Auth Required |
//! |--------|-----------------|----------------------------|---------------|
//! | POST   | `/register`     | Create new user account    | No            |
//! | POST   | `/login`        | Authenticate user          | No            |
//! | POST   | `/refresh`      | Refresh access token       | No*           |
//! | POST   | `/logout`       | Invalidate current session | Yes           |
//! | POST   | `/logout-all`   | Invalidate all sessions    | Yes           |
//! | GET    | `/me`           | Get authenticated profile  | Yes           |
//! | POST   | `/verify-email` | Verify email address       | No            |
//! | POST   | `/forgot-password` | Request password reset  | No            |
//! | POST   | `/reset-password`  | Complete password reset | No            |
//!
//! *Requires valid refresh token in request body
//!
//! ## Health Check
//!
//! | Method | Endpoint  | Description              |
//! |--------|-----------|--------------------------|
//! | GET    | `/health` | Service health status    |
//!
//! ## Authentication Flow
//!
//! ```text
//! User                    Auth Service              Database           Redis
//!   │                          │                        │                 │
//!   │  POST /login             │                        │                 │
//!   │  {email, password}       │                        │                 │
//!   │─────────────────────────▶│                        │                 │
//!   │                          │  SELECT user           │                 │
//!   │                          │───────────────────────▶│                 │
//!   │                          │  User record           │                 │
//!   │                          │◀───────────────────────│                 │
//!   │                          │                        │                 │
//!   │                          │  Verify password       │                 │
//!   │                          │  (Argon2id)            │                 │
//!   │                          │                        │                 │
//!   │                          │  Store refresh_token   │                 │
//!   │                          │───────────────────────▶│                 │
//!   │                          │                        │                 │
//!   │                          │  Cache session         │                 │
//!   │                          │────────────────────────────────────────▶│
//!   │                          │                        │                 │
//!   │  {access_token,          │                        │                 │
//!   │   refresh_token}         │                        │                 │
//!   │◀─────────────────────────│                        │                 │
//! ```
//!
//! ## Security Features
//!
//! - **Password Hashing**: Argon2id with OWASP-recommended parameters
//! - **JWT Tokens**: Short-lived access tokens (15 min), longer refresh tokens (7 days)
//! - **Token Blacklisting**: Redis-based invalidation for logout
//! - **Rate Limiting**: Protection against brute-force attacks
//! - **Device Tracking**: Fingerprint and IP logging for sessions
//!
//! ## Configuration
//!
//! Environment variables are loaded via [`shared::config::AppConfig`].
//! Key settings for auth-service:
//!
//! - `SERVICE_NAME=auth-service`
//! - `JWT_SECRET` - Secret key for token signing
//! - `JWT_ACCESS_EXPIRY_SECS` - Access token lifetime (default: 900)
//! - `JWT_REFRESH_EXPIRY_SECS` - Refresh token lifetime (default: 604800)
//!
//! ## Related Documentation
//!
//! - Authentication design: [`shared::auth`]
//! - JWT implementation: [`shared::auth::jwt`]
//! - Password security: [`shared::auth::password`]
//! - User stories: `_docs/business/user-stories.md`
//! - Security requirements: `_docs/business/non-functional-requirements.md`

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use shared::{
    auth::{jwt::JwtService, password::PasswordHasher},
    config::AppConfig,
    database, redis_client::RedisClient,
    tracing_config,
};
use std::sync::Arc;
use tracing::info;

mod api;
mod domain;
mod repository;
mod service;

use api::routes;
use repository::UserRepository;
use service::AuthService;

/// Shared application state injected into all request handlers.
///
/// This struct is wrapped in [`actix_web::web::Data`] and made available
/// to all handlers via extractor pattern. It contains the core services
/// needed for authentication operations.
///
/// # Fields
///
/// | Field          | Type                  | Purpose                           |
/// |----------------|-----------------------|-----------------------------------|
/// | `auth_service` | [`AuthService`]       | Business logic for auth operations|
/// | `jwt_service`  | `Arc<JwtService>`     | JWT token generation/validation   |
///
/// # Thread Safety
///
/// - `AuthService` contains async-safe components (connection pools, Arc-wrapped clients)
/// - `JwtService` is wrapped in `Arc` for shared ownership across worker threads
///
/// # Usage in Handlers
///
/// ```rust,ignore
/// async fn login(
///     state: web::Data<AppState>,
///     body: web::Json<LoginRequest>,
/// ) -> Result<HttpResponse, ApiError> {
///     let tokens = state.auth_service.login(&body.email, &body.password).await?;
///     Ok(HttpResponse::Ok().json(tokens))
/// }
/// ```
pub struct AppState {
    /// Authentication service with business logic for user operations
    pub auth_service: AuthService,
    /// JWT service for token operations, shared via Arc for efficiency
    pub jwt_service: Arc<JwtService>,
}

/// Application entry point and server initialization.
///
/// # Initialization Sequence
///
/// 1. Load configuration from environment variables
/// 2. Initialize structured logging/tracing
/// 3. Create PostgreSQL connection pool
/// 4. Create Redis client for caching/sessions
/// 5. Instantiate services with dependencies
/// 6. Configure and start HTTP server
///
/// # Error Handling
///
/// The main function uses `expect()` for critical initialization errors
/// that should prevent the service from starting (fail-fast pattern).
///
/// # Graceful Shutdown
///
/// Actix-web handles SIGTERM/SIGINT signals and gracefully shuts down:
/// - Stops accepting new connections
/// - Waits for in-flight requests to complete
/// - Closes database and Redis connections
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ─────────────────────────────────────────────────────────────────────
    // Step 1: Load configuration from environment
    // ─────────────────────────────────────────────────────────────────────
    // Configuration is loaded from .env file and environment variables.
    // See: shared/src/config.rs for all available settings.
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // ─────────────────────────────────────────────────────────────────────
    // Step 2: Initialize structured logging
    // ─────────────────────────────────────────────────────────────────────
    // Sets up tracing subscriber with JSON output in production,
    // pretty-printed output in development.
    tracing_config::init_tracing(&config.service_name, config.is_production());

    info!(
        service = %config.service_name,
        host = %config.server.host,
        port = %config.server.port,
        "Starting auth-service"
    );

    // ─────────────────────────────────────────────────────────────────────
    // Step 3: Create PostgreSQL connection pool
    // ─────────────────────────────────────────────────────────────────────
    // Pool is configured with connection limits and timeouts.
    // See: shared/src/database.rs for pool configuration details.
    let db_pool = database::create_pool(&config.database)
        .await
        .expect("Failed to create database pool");

    // ─────────────────────────────────────────────────────────────────────
    // Step 4: Create Redis client
    // ─────────────────────────────────────────────────────────────────────
    // Redis is used for session storage, token blacklisting, and caching.
    // See: shared/src/redis_client.rs for client implementation.
    let redis_client = RedisClient::new(&config.redis)
        .await
        .expect("Failed to connect to Redis");

    // ─────────────────────────────────────────────────────────────────────
    // Step 5: Instantiate services with dependencies
    // ─────────────────────────────────────────────────────────────────────
    // Services are created with dependency injection pattern.
    // JwtService and PasswordHasher are wrapped in Arc for shared ownership.
    let jwt_service = Arc::new(JwtService::new(config.jwt.clone()));
    let password_hasher = Arc::new(PasswordHasher::new());
    let user_repository = UserRepository::new(db_pool.clone());
    let auth_service = AuthService::new(
        user_repository,
        jwt_service.clone(),
        password_hasher,
        redis_client,
        config.jwt.clone(),
    );

    // Wrap state in web::Data for thread-safe sharing across workers
    let app_state = web::Data::new(AppState {
        auth_service,
        jwt_service,
    });

    // Database pool is also shared for health checks and direct queries
    let db_pool = web::Data::new(db_pool);

    // ─────────────────────────────────────────────────────────────────────
    // Step 6: Configure and start HTTP server
    // ─────────────────────────────────────────────────────────────────────
    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    info!("Server listening on {}:{}", server_host, server_port);

    HttpServer::new(move || {
        // ─────────────────────────────────────────────────────────────
        // CORS Configuration
        // ─────────────────────────────────────────────────────────────
        // In production, replace `allow_any_origin()` with specific
        // allowed origins from configuration:
        // ```rust
        // Cors::default()
        //     .allowed_origin("https://app.example.com")
        //     .allowed_origin("https://admin.example.com")
        // ```
        let cors = Cors::default()
            .allow_any_origin() // TODO: Configure specific origins for production
            .allow_any_method()
            .allow_any_header()
            .max_age(3600); // Cache preflight for 1 hour

        App::new()
            // ─────────────────────────────────────────────────────────
            // Middleware Stack (order matters: first added = last executed)
            // ─────────────────────────────────────────────────────────
            .wrap(cors)                                      // CORS headers
            .wrap(middleware::Logger::default())             // Request logging
            .wrap(middleware::Compress::default())           // Response compression
            .wrap(tracing_actix_web::TracingLogger::default()) // Distributed tracing
            // ─────────────────────────────────────────────────────────
            // Shared State
            // ─────────────────────────────────────────────────────────
            .app_data(app_state.clone())
            .app_data(db_pool.clone())
            // ─────────────────────────────────────────────────────────
            // Routes Configuration
            // ─────────────────────────────────────────────────────────
            // See: api/routes.rs for endpoint definitions
            .configure(routes::configure)
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
