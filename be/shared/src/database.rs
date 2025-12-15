//! # PostgreSQL Connection Pool
//!
//! Database connection management using sqlx's async connection pool.
//!
//! ## What is a Connection Pool?
//!
//! A connection pool maintains a set of reusable database connections.
//! Instead of creating a new connection for each query (expensive),
//! we borrow from the pool and return when done.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Connection Pool                               │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                  │
//! │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐                         │
//! │  │ Conn │  │ Conn │  │ Conn │  │ Conn │  ...  (max_connections) │
//! │  │  1   │  │  2   │  │  3   │  │  4   │                         │
//! │  └──┬───┘  └──┬───┘  └──┬───┘  └──────┘                         │
//! │     │         │         │         │                             │
//! │     ▼         ▼         ▼         ▼                             │
//! │  [busy]    [busy]    [idle]    [idle]                           │
//! │                                                                  │
//! └─────────────────────────────────────────────────────────────────┘
//!                    │
//!                    ▼
//!            ┌───────────────┐
//!            │  PostgreSQL   │
//!            │   Database    │
//!            └───────────────┘
//! ```
//!
//! ## Pool Configuration
//!
//! | Parameter | Default | Description |
//! |-----------|---------|-------------|
//! | `max_connections` | 10 | Maximum connections in pool |
//! | `min_connections` | 1 | Minimum connections to maintain |
//! | `connect_timeout` | 30s | Timeout for acquiring connection |
//! | `max_lifetime` | 30min | Recycle connections after this time |
//!
//! ## Sizing Guidelines
//!
//! **Rule of thumb**: `max_connections = (2 × cpu_cores) + disk_spindles`
//!
//! For example:
//! - 4-core server with SSD: ~10 connections
//! - 8-core server with SSD: ~18 connections
//!
//! Don't set too high! PostgreSQL has limits, and idle connections
//! consume memory on both sides.
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::database::create_pool;
//! use shared::config::AppConfig;
//!
//! let config = AppConfig::from_env()?;
//! let pool = create_pool(&config.database).await?;
//!
//! // Use in queries
//! let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
//!     .fetch_all(&pool)
//!     .await?;
//!
//! // Health check
//! shared::database::health_check(&pool).await?;
//! ```
//!
//! ## Related Documentation
//!
//! - [`crate::config::DatabaseConfig`] - Pool configuration
//! - [sqlx PgPoolOptions](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgPoolOptions.html)
//! - [`_docs/architecture/database-architecture.md`] - Schema design

use crate::config::DatabaseConfig;
use crate::errors::ApiError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

// =============================================================================
// Pool Creation
// =============================================================================

/// Creates a PostgreSQL connection pool.
///
/// This function establishes the initial connections and validates
/// connectivity. It should be called once at application startup.
///
/// ## Process
///
/// 1. Parse the connection URL
/// 2. Configure pool options (timeouts, limits)
/// 3. Establish minimum connections
/// 4. Test each connection before adding to pool
///
/// ## Parameters
///
/// - `config`: Database configuration (URL, pool sizes, timeouts)
///
/// ## Returns
///
/// A `PgPool` that can be cloned and shared across handlers.
/// `PgPool` is an `Arc` internally, so cloning is cheap.
///
/// ## Errors
///
/// Returns `ApiError::DatabaseError` if:
/// - Connection URL is invalid
/// - Database is unreachable
/// - Authentication fails
/// - Initial connections cannot be established
///
/// ## Example
///
/// ```rust,ignore
/// let pool = create_pool(&config.database).await?;
///
/// // Share across application
/// let app_state = AppState {
///     db: pool.clone(),
///     // ...
/// };
/// ```
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, ApiError> {
    info!(
        max_connections = config.max_connections,
        min_connections = config.min_connections,
        connect_timeout_seconds = config.connect_timeout_seconds,
        max_lifetime_seconds = config.max_lifetime_seconds,
        "Creating database connection pool"
    );

    let pool = PgPoolOptions::new()
        // Maximum connections in the pool
        .max_connections(config.max_connections)
        // Minimum connections to maintain (even when idle)
        .min_connections(config.min_connections)
        // How long to wait for a connection before timing out
        .acquire_timeout(Duration::from_secs(config.connect_timeout_seconds))
        // Recycle connections after this time (prevents stale connections)
        .max_lifetime(Duration::from_secs(config.max_lifetime_seconds))
        // Verify connection is valid before handing it out
        // Small performance cost but catches dead connections
        .test_before_acquire(true)
        // Connect to the database
        .connect(&config.url)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create database pool");
            ApiError::DatabaseError(e)
        })?;

    info!("Database connection pool created successfully");

    Ok(pool)
}

// =============================================================================
// Health Check
// =============================================================================

/// Verifies database connectivity.
///
/// This is used by health check endpoints to verify the database
/// is reachable and responding to queries.
///
/// ## How It Works
///
/// Executes a simple `SELECT 1` query. If this succeeds, the
/// database is considered healthy.
///
/// ## Usage
///
/// ```rust,ignore
/// // In a health check handler
/// async fn health(pool: web::Data<PgPool>) -> impl Responder {
///     match health_check(&pool).await {
///         Ok(()) => HttpResponse::Ok().body("healthy"),
///         Err(_) => HttpResponse::ServiceUnavailable().body("database unavailable"),
///     }
/// }
/// ```
///
/// ## Errors
///
/// Returns `ApiError::DatabaseError` if the query fails.
pub async fn health_check(pool: &PgPool) -> Result<(), ApiError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(ApiError::DatabaseError)?;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    // Database tests require a real database connection.
    // They are marked with #[ignore] and run with:
    //
    //   cargo test --features integration -- --ignored
    //
    // See _docs/development/development-standards.md for testing guidelines.
}

