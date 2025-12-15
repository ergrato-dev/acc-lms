//! # API Layer
//!
//! The API layer handles HTTP request/response processing for authentication
//! endpoints. It bridges the HTTP world with the service layer.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                            HTTP Request                                 │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                          Routes (routes.rs)                             │
//! │  Define URL patterns and HTTP methods, map to handlers                  │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        Handlers (handlers.rs)                           │
//! │  1. Extract request data (JSON body, headers, path params)              │
//! │  2. Validate input using DTOs                                           │
//! │  3. Call service layer                                                  │
//! │  4. Transform response                                                  │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                          DTOs (dto.rs)                                  │
//! │  - Request validation (serde, validator)                                │
//! │  - Response serialization                                               │
//! │  - API contract definitions                                             │
//! └───────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Organization
//!
//! | Module       | Responsibility                                      |
//! |--------------|-----------------------------------------------------|
//! | `routes`     | Route definitions with actix-web's `configure()`    |
//! | `handlers`   | Request handlers (extractors → service → response)  |
//! | `dto`        | Data Transfer Objects for request/response          |
//!
//! ## Endpoint Summary
//!
//! | Method | Path                       | Handler              | Auth |
//! |--------|----------------------------|----------------------|------|
//! | POST   | `/api/v1/auth/register`    | `register`           | No   |
//! | POST   | `/api/v1/auth/login`       | `login`              | No   |
//! | POST   | `/api/v1/auth/refresh`     | `refresh_token`      | No   |
//! | POST   | `/api/v1/auth/logout`      | `logout`             | Yes  |
//! | POST   | `/api/v1/auth/logout-all`  | `logout_all`         | Yes  |
//! | GET    | `/api/v1/auth/me`          | `get_profile`        | Yes  |
//! | POST   | `/api/v1/auth/verify-email`| `verify_email`       | No   |
//! | POST   | `/api/v1/auth/forgot-password` | `forgot_password`| No   |
//! | POST   | `/api/v1/auth/reset-password`  | `reset_password` | No   |
//! | GET    | `/health`                  | `health_check`       | No   |
//!
//! ## Related Documentation
//!
//! - Service layer: [`crate::service::AuthService`]
//! - Error responses: [`shared::errors::ApiError`]
//! - Authentication middleware: [`shared::auth::middleware`]

pub mod dto;
pub mod handlers;
pub mod routes;
