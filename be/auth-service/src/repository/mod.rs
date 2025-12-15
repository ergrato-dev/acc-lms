//! # Repository Layer
//!
//! The repository layer provides data access abstractions for the authentication
//! domain. It encapsulates all database operations, hiding SQL complexity from
//! the service layer.
//!
//! ## Clean Architecture Principles
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                          Service Layer                                  │
//! │                    (Business logic, validation)                         │
//! └───────────────────────────────┬─────────────────────────────────────────┘
//!                                 │
//!                                 │ Uses repository trait
//!                                 ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        Repository Layer                                 │
//! │  ┌────────────────────────────────────────────────────────────────────┐ │
//! │  │                    UserRepository                                  │ │
//! │  │  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐  │ │
//! │  │  │ User Operations  │  │ Token Operations │  │ Preferences Ops  │  │ │
//! │  │  │ - create         │  │ - create_token   │  │ - get_prefs      │  │ │
//! │  │  │ - find_by_email  │  │ - find_token     │  │ - update_prefs   │  │ │
//! │  │  │ - find_by_id     │  │ - revoke_token   │  │                  │  │ │
//! │  │  │ - update         │  │ - revoke_all     │  │                  │  │ │
//! │  │  │ - delete         │  │                  │  │                  │  │ │
//! │  │  └──────────────────┘  └──────────────────┘  └──────────────────┘  │ │
//! │  └────────────────────────────────────────────────────────────────────┘ │
//! └───────────────────────────────────┬─────────────────────────────────────┘
//!                                     │
//!                                     │ sqlx queries
//!                                     ▼
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         PostgreSQL Database                             │
//! │         users │ refresh_tokens │ user_preferences │ ...                 │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Decisions
//!
//! 1. **Concrete Implementation**: Uses `UserRepository` struct directly rather
//!    than trait for simplicity. Traits can be added later for testing.
//!
//! 2. **Connection Pool**: Repository holds a `PgPool` clone (Arc internally)
//!    for efficient connection sharing.
//!
//! 3. **Error Handling**: Database errors are mapped to [`shared::errors::ApiError`]
//!    for consistent HTTP responses.
//!
//! 4. **Soft Deletes**: All queries filter by `deleted_at IS NULL`.
//!
//! ## Related Documentation
//!
//! - Database schema: `db/migrations/postgresql/001_initial_schema.sql`
//! - Error handling: [`shared::errors`]
//! - Connection pool: [`shared::database`]

pub mod user_repository;

pub use user_repository::UserRepository;
