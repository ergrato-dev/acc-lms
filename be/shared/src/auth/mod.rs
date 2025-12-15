//! # Authentication and Authorization Module
//!
//! This module provides all authentication and authorization functionality
//! for the ACC LMS microservices architecture.
//!
//! ## Module Structure
//!
//! ```text
//! auth/
//! ├── jwt.rs        - JWT token generation and validation
//! ├── password.rs   - Secure password hashing with Argon2id
//! └── middleware.rs - Actix-web authentication middleware
//! ```
//!
//! ## Overview
//!
//! | Component | Purpose | See Also |
//! |-----------|---------|----------|
//! | [`JwtService`] | Generate and validate JWT tokens | [RFC 7519](https://tools.ietf.org/html/rfc7519) |
//! | [`PasswordHasher`] | Argon2id password hashing | [OWASP Password Storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) |
//! | [`AuthMiddleware`] | Request authentication | Actix-web middleware docs |
//! | [`AuthenticatedUser`] | Extractor for authenticated user | Actix-web extractors |
//!
//! ## Security Implementation (RF-AUTH-001)
//!
//! This module implements the authentication requirements from
//! [`_docs/business/functional-requirements.md`]:
//!
//! - **JWT with short-lived access tokens** (15 min default)
//! - **Refresh tokens for session continuity** (7 days default)
//! - **Argon2id password hashing** (OWASP recommended)
//! - **Role-based access control (RBAC)** with hierarchy
//!
//! ## Token Flow
//!
//! ```text
//! ┌────────┐        ┌────────────┐        ┌────────────┐
//! │ Client │        │ Auth API   │        │ Protected  │
//! └───┬────┘        └─────┬──────┘        │    API     │
//!     │                   │               └─────┬──────┘
//!     │ POST /login       │                     │
//!     │ (email, password) │                     │
//!     │──────────────────>│                     │
//!     │                   │                     │
//!     │ {access_token,    │                     │
//!     │  refresh_token}   │                     │
//!     │<──────────────────│                     │
//!     │                   │                     │
//!     │ GET /resource     │                     │
//!     │ Authorization:    │                     │
//!     │ Bearer <access>   │                     │
//!     │────────────────────────────────────────>│
//!     │                   │                     │
//!     │ 200 OK {data}     │                     │
//!     │<────────────────────────────────────────│
//!     │                   │                     │
//!     │ (after 15 min)    │                     │
//!     │ POST /refresh     │                     │
//!     │ (refresh_token)   │                     │
//!     │──────────────────>│                     │
//!     │                   │                     │
//!     │ {new access_token,│                     │
//!     │  new refresh}     │                     │
//!     │<──────────────────│                     │
//! ```
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::auth::{JwtService, PasswordHasher, AuthenticatedUser};
//! use shared::config::AppConfig;
//!
//! // In your service initialization
//! let config = AppConfig::from_env()?;
//! let jwt_service = Arc::new(JwtService::new(config.jwt.clone()));
//! let password_hasher = PasswordHasher::new();
//!
//! // Login handler
//! async fn login(credentials: Credentials) -> ApiResult<TokenPair> {
//!     let user = user_repo.find_by_email(&credentials.email).await?;
//!     
//!     if password_hasher.verify(&credentials.password, &user.password_hash)? {
//!         let tokens = jwt_service.generate_tokens(
//!             user.id,
//!             &user.email,
//!             &user.role
//!         )?;
//!         Ok(tokens)
//!     } else {
//!         Err(ApiError::InvalidCredentials)
//!     }
//! }
//!
//! // Protected handler (user extracted from JWT)
//! async fn get_profile(user: AuthenticatedUser) -> ApiResult<Profile> {
//!     profile_repo.find_by_user_id(user.user_id).await
//! }
//! ```
//!
//! ## Related Documentation
//!
//! - [`_docs/business/functional-requirements.md`] - RF-AUTH-001 to RF-AUTH-004
//! - [`_docs/development/development-standards.md`] - Security practices

pub mod jwt;
pub mod middleware;
pub mod password;

// Re-export main types for convenient access
pub use jwt::{Claims, JwtService, TokenPair};
pub use middleware::{AuthMiddleware, AuthenticatedUser};
pub use password::PasswordHasher;
