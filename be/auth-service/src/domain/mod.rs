//! # Domain Layer
//!
//! The domain layer contains the core business entities and domain events
//! for the authentication service. This layer is **framework-agnostic** and
//! represents the heart of the business logic.
//!
//! ## Clean Architecture Principles
//!
//! ```text
//! ┌───────────────────────────────────────────────────────────────────┐
//! │                         Domain Layer                              │
//! │  ┌─────────────────────────────────────────────────────────────┐  │
//! │  │                       Entities                              │  │
//! │  │  - User: Core user entity with authentication data          │  │
//! │  │  - UserProfile: Public user data (no sensitive fields)      │  │
//! │  │  - RefreshToken: Session/token management                   │  │
//! │  │  - UserPreferences: User settings and preferences           │  │
//! │  └─────────────────────────────────────────────────────────────┘  │
//! │                                                                   │
//! │  ┌─────────────────────────────────────────────────────────────┐  │
//! │  │                     Domain Events                           │  │
//! │  │  - UserRegistered: Emitted when a new user signs up         │  │
//! │  │  - UserLoggedIn: Emitted on successful authentication       │  │
//! │  │  - UserLoggedOut: Emitted when user ends session            │  │
//! │  │  - PasswordChanged: Emitted when password is updated        │  │
//! │  │  - EmailVerified: Emitted when email is confirmed           │  │
//! │  └─────────────────────────────────────────────────────────────┘  │
//! └───────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Structure
//!
//! | Module     | Purpose                                           |
//! |------------|---------------------------------------------------|
//! | `entities` | Core domain entities mapped to database tables    |
//! | `events`   | Domain events for event-driven architecture       |
//!
//! ## Design Decisions
//!
//! 1. **Entities vs DTOs**: Domain entities contain all database fields,
//!    while separate DTOs in the API layer handle request/response transformation.
//!
//! 2. **`FromRow` Derive**: Entities use sqlx's `FromRow` for automatic
//!    mapping from database rows, reducing boilerplate.
//!
//! 3. **Soft Deletes**: The `deleted_at` field enables soft deletion,
//!    preserving data for audit trails.
//!
//! ## Related Documentation
//!
//! - Database schema: `_docs/architecture/database-architecture.md`
//! - User stories: `_docs/business/user-stories.md`

pub mod entities;
pub mod events;

pub use entities::*;
