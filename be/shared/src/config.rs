//! # Application Configuration
//!
//! Centralized configuration management using environment variables.
//!
//! ## How It Works
//!
//! Configuration is loaded from multiple sources in order of precedence:
//!
//! 1. **Default values** - Sensible defaults for development
//! 2. **`.env` file** - Local overrides (git-ignored)
//! 3. **Environment variables** - Docker/K8s configuration
//!
//! ## Environment Variable Format
//!
//! Variables use the `APP_` prefix with `__` as nested separator:
//!
//! ```bash
//! # Server configuration
//! APP_SERVER__HOST=0.0.0.0
//! APP_SERVER__PORT=8080
//!
//! # Database (also accepts DATABASE_URL directly for Docker compatibility)
//! APP_DATABASE__URL=postgres://user:pass@localhost:5432/db
//! # or simply:
//! DATABASE_URL=postgres://user:pass@localhost:5432/db
//!
//! # JWT settings
//! JWT_SECRET=your_secret_key_minimum_32_characters
//! ```
//!
//! ## Configuration Sections
//!
//! | Section | Purpose | See Also |
//! |---------|---------|----------|
//! | `server` | HTTP server settings | Actix-web docs |
//! | `database` | PostgreSQL pool config | [`database`](crate::database) module |
//! | `redis` | Redis connection | [`redis_client`](crate::redis_client) module |
//! | `jwt` | Token settings | [`auth::jwt`](crate::auth::jwt) module |
//!
//! ## Security Notes
//!
//! - Never commit `.env` files with real secrets
//! - Use strong JWT secrets (32+ characters)
//! - In production, use secret management (Vault, AWS Secrets Manager)
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use shared::config::AppConfig;
//!
//! let config = AppConfig::from_env()?;
//!
//! if config.is_production() {
//!     // Enable stricter security settings
//! }
//! ```

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

/// Main application configuration.
///
/// This struct is the root of all configuration. It's designed to be
/// immutable after creation - create once at startup and share via `Arc`.
///
/// # Fields
///
/// All fields are public for transparency, but should be treated as read-only.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// HTTP server settings (host, port, workers)
    pub server: ServerConfig,
    
    /// PostgreSQL connection pool settings
    pub database: DatabaseConfig,
    
    /// Redis connection settings
    pub redis: RedisConfig,
    
    /// JWT token configuration
    pub jwt: JwtConfig,
    
    /// Service name for tracing and logging
    pub service_name: String,
    
    /// Runtime environment (development/staging/production)
    pub environment: AppEnvironment,
}

/// HTTP server configuration.
///
/// These settings control how Actix-web binds and scales.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// IP address to bind to.
    /// Default: `0.0.0.0` (all interfaces)
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Port number to listen on.
    /// Default: `8080`
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// Number of worker threads.
    /// Default: `0` (auto-detect based on CPU cores)
    #[serde(default)]
    pub workers: usize,
}

/// PostgreSQL database configuration.
///
/// These settings are passed to sqlx's `PgPoolOptions`.
/// For tuning guidance, see: <https://docs.rs/sqlx/latest/sqlx/pool/struct.PoolOptions.html>
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL.
    /// Format: `postgres://user:password@host:port/database`
    pub url: String,
    
    /// Maximum number of connections in the pool.
    /// Default: `10`
    /// 
    /// Rule of thumb: `(2 * cpu_cores) + disk_spindles`
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    
    /// Minimum connections to keep open.
    /// Default: `1`
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    
    /// Connection timeout in seconds.
    /// Default: `30`
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_seconds: u64,
    
    /// Maximum lifetime of a connection in seconds.
    /// Default: `1800` (30 minutes)
    /// 
    /// Connections are recycled after this time to prevent stale connections.
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime_seconds: u64,
}

/// Redis configuration.
///
/// Used for caching, session storage, and rate limiting.
/// See [`redis_client`](crate::redis_client) for usage.
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL.
    /// Format: `redis://[:password@]host:port[/db]`
    pub url: String,
    
    /// Connection pool size.
    /// Default: `10`
    #[serde(default = "default_redis_pool_size")]
    pub pool_size: u32,
}

/// JWT (JSON Web Token) configuration.
///
/// Controls token generation and validation.
/// For implementation details, see [`auth::jwt`](crate::auth::jwt).
///
/// ## Security Requirements (RF-GLOBAL-001)
///
/// - Access tokens: Short-lived (15 min default)
/// - Refresh tokens: Longer-lived (7 days default) with rotation
/// - Secret must be at least 32 characters
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    /// Secret key for HS256 signing.
    /// 
    /// **Security**: Must be at least 32 characters.
    /// In production, use a cryptographically random string.
    pub secret: String,
    
    /// Access token time-to-live in seconds.
    /// Default: `900` (15 minutes)
    #[serde(default = "default_access_token_ttl")]
    pub access_token_ttl_seconds: u64,
    
    /// Refresh token time-to-live in seconds.
    /// Default: `604800` (7 days)
    #[serde(default = "default_refresh_token_ttl")]
    pub refresh_token_ttl_seconds: u64,
    
    /// Token issuer claim (`iss`).
    /// Default: `acc-lms`
    #[serde(default = "default_issuer")]
    pub issuer: String,
    
    /// Token audience claim (`aud`).
    /// Default: `acc-lms-api`
    #[serde(default = "default_audience")]
    pub audience: String,
}

/// Application runtime environment.
///
/// Affects logging format, security settings, and feature flags.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AppEnvironment {
    /// Local development - verbose logging, relaxed security
    Development,
    /// Pre-production testing
    Staging,
    /// Production - JSON logging, strict security
    Production,
}

impl Default for AppEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

impl AppConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Process
    ///
    /// 1. Loads `.env` file if present (silently ignores if missing)
    /// 2. Applies default values
    /// 3. Overrides with `APP_*` environment variables
    /// 4. Applies Docker-compatible overrides (`DATABASE_URL`, etc.)
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if:
    /// - Required variables are missing
    /// - Values cannot be parsed to expected types
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = AppConfig::from_env().expect("Failed to load config");
    /// println!("Running on port {}", config.server.port);
    /// ```
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if it exists (development convenience)
        let _ = dotenvy::dotenv();

        let config = Config::builder()
            // Set sensible defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.workers", 0)?
            .set_default("environment", "development")?
            .set_default("service_name", "acc-lms")?
            // Load from APP_* environment variables
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            // Docker-compatible overrides (no prefix)
            .set_override_option(
                "database.url",
                std::env::var("DATABASE_URL").ok(),
            )?
            .set_override_option(
                "redis.url",
                std::env::var("REDIS_URL").ok(),
            )?
            .set_override_option(
                "jwt.secret",
                std::env::var("JWT_SECRET").ok(),
            )?
            .set_override_option(
                "service_name",
                std::env::var("SERVICE_NAME").ok(),
            )?
            .build()?;

        config.try_deserialize()
    }

    /// Returns `true` if running in development mode.
    ///
    /// Use this to enable development-only features like:
    /// - Verbose SQL logging
    /// - Relaxed CORS
    /// - Debug endpoints
    #[inline]
    pub fn is_development(&self) -> bool {
        self.environment == AppEnvironment::Development
    }

    /// Returns `true` if running in production mode.
    ///
    /// Use this to enable production-only features like:
    /// - JSON structured logging
    /// - Strict security headers
    /// - Rate limiting
    #[inline]
    pub fn is_production(&self) -> bool {
        self.environment == AppEnvironment::Production
    }
}

// =============================================================================
// Default Value Functions
// =============================================================================
// These functions provide defaults when env vars are not set.
// Separated for clarity and potential reuse.

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    1
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_max_lifetime() -> u64 {
    1800 // 30 minutes
}

fn default_redis_pool_size() -> u32 {
    10
}

fn default_access_token_ttl() -> u64 {
    900 // 15 minutes - security best practice
}

fn default_refresh_token_ttl() -> u64 {
    604800 // 7 days
}

fn default_issuer() -> String {
    "acc-lms".to_string()
}

fn default_audience() -> String {
    "acc-lms-api".to_string()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_environment_is_development() {
        assert_eq!(AppEnvironment::default(), AppEnvironment::Development);
    }

    #[test]
    fn test_environment_variants() {
        // Ensure all variants are distinct
        assert_ne!(AppEnvironment::Development, AppEnvironment::Production);
        assert_ne!(AppEnvironment::Development, AppEnvironment::Staging);
        assert_ne!(AppEnvironment::Staging, AppEnvironment::Production);
    }
}

