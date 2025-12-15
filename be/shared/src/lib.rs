//! # ACC LMS - Shared Library
//!
//! Core shared functionality for all ACC LMS microservices.
//!
//! This crate provides common utilities, types, and services that are used across
//! the entire backend ecosystem. It follows the DRY principle to avoid code
//! duplication and ensure consistency.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Microservices                           │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │
//! │  │  Auth   │ │  Users  │ │ Courses │ │  ...    │           │
//! │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘           │
//! │       │           │           │           │                 │
//! │       └───────────┴───────────┴───────────┘                 │
//! │                       │                                     │
//! │              ┌────────▼────────┐                            │
//! │              │  shared crate   │ ◄── You are here           │
//! │              └─────────────────┘                            │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Modules
//!
//! | Module | Purpose | Key Types |
//! |--------|---------|-----------|
//! | [`config`] | Environment-based configuration | [`AppConfig`] |
//! | [`errors`] | Standardized error handling | [`ApiError`], [`ApiResult`] |
//! | [`auth`] | JWT tokens, password hashing, middleware | [`JwtService`], [`PasswordHasher`] |
//! | [`database`] | PostgreSQL connection pool | [`create_pool`](database::create_pool) |
//! | [`redis_client`] | Redis for cache & sessions | [`RedisClient`] |
//! | [`tracing_config`] | Structured logging setup | [`init_tracing`](tracing_config::init_tracing) |
//! | [`validation`] | Request validation helpers | Custom validators |
//!
//! ## Design Decisions
//!
//! 1. **Single source of truth**: All shared types live here to prevent drift
//! 2. **Framework agnostic where possible**: Core logic works with both Actix-web and Axum
//! 3. **Security first**: Auth and crypto follow OWASP guidelines
//! 4. **Observable by default**: Structured logging and tracing built-in
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::{AppConfig, ApiError, ApiResult};
//! use shared::auth::{JwtService, PasswordHasher};
//! use shared::database;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AppConfig::from_env()?;
//!     let db_pool = database::create_pool(&config.database).await?;
//!     let jwt = JwtService::new(config.jwt.clone());
//!     
//!     // Ready to build your service!
//!     Ok(())
//! }
//! ```
//!
//! ## Related Documentation
//!
//! - [Development Standards](../../_docs/development/development-standards.md)
//! - [Functional Requirements](../../_docs/business/functional-requirements.md)
//! - [Non-Functional Requirements](../../_docs/non-functional-requirements.md)

pub mod auth;
pub mod config;
pub mod database;
pub mod errors;
pub mod redis_client;
pub mod tracing_config;
pub mod validation;

// Re-exports for convenience - import commonly used types directly from `shared`
pub use config::AppConfig;
pub use errors::{ApiError, ApiResult};

