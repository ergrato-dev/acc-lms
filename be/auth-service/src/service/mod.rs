//! # Service Layer
//!
//! The service layer contains the core business logic for authentication
//! operations. It orchestrates between the repository layer (data access)
//! and the API layer (HTTP handling).
//!
//! ## Clean Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                            API Layer                                    │
//! │                    (HTTP handlers, DTOs)                                │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 │ Calls service methods
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                          Service Layer                                  │
//! │  ┌────────────────────────────────────────────────────────────────────┐ │
//! │  │                       AuthService                                  │ │
//! │  │  ┌──────────────────────────────────────────────────────────────┐  │ │
//! │  │  │ Business Logic:                                              │  │ │
//! │  │  │ - User registration with validation                          │  │ │
//! │  │  │ - Login with password verification                           │  │ │
//! │  │  │ - Token generation and refresh                               │  │ │
//! │  │  │ - Session management (logout, logout-all)                    │  │ │
//! │  │  │ - Password reset flow                                        │  │ │
//! │  │  │ - Email verification                                         │  │ │
//! │  │  └──────────────────────────────────────────────────────────────┘  │ │
//! │  └────────────────────────────────────────────────────────────────────┘ │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 │ Uses repository + external services
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │              Repository Layer          │    External Services          │
//! │            (Data persistence)          │  (JWT, Redis, Hashing)        │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Service Responsibilities
//!
//! | Operation           | Validation                    | Side Effects                |
//! |---------------------|-------------------------------|-----------------------------|
//! | `register`          | Email uniqueness, password    | Create user, send email     |
//! | `login`             | Credentials verification      | Update last_login, session  |
//! | `refresh_token`     | Token validity                | Rotate tokens               |
//! | `logout`            | Token ownership               | Revoke token, blacklist     |
//! | `logout_all`        | User authentication           | Revoke all, blacklist all   |
//! | `verify_email`      | Token validity                | Update email_verified       |
//! | `forgot_password`   | Email existence               | Generate reset token        |
//! | `reset_password`    | Token validity, password      | Update password, clear token|
//!
//! ## Related Documentation
//!
//! - JWT handling: [`shared::auth::jwt`]
//! - Password hashing: [`shared::auth::password`]
//! - Repository: [`crate::repository::UserRepository`]

pub mod auth_service;

pub use auth_service::AuthService;
