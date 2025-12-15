//! # Repository Module
//!
//! Contains the data access layer for the users-service.
//! Implements the repository pattern to abstract database operations.
//!
//! ## Design Pattern: Repository
//!
//! The repository pattern provides:
//! - Abstraction over data storage
//! - Single point of database access
//! - Testability through trait-based design
//! - Clean separation of concerns
//!
//! ## Implementations
//!
//! - [`UserProfileRepository`]: PostgreSQL-based implementation

mod user_repository;

pub use user_repository::{UserProfileRepository, ProfileUpdate, PreferencesUpdate};
