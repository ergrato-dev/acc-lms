//! # Domain Module
//!
//! Contains the core domain entities, value objects, and domain events
//! for the users-service. This module follows Domain-Driven Design (DDD)
//! principles.
//!
//! ## Structure
//!
//! - `entities`: Core domain entities (UserProfile, UserPreferences)
//! - `events`: Domain events emitted by the service
//! - `value_objects`: Value objects for type safety (Email, UserId, etc.)

pub mod entities;
pub mod events;
pub mod value_objects;

// Re-export commonly used types for convenience
pub use entities::{UserProfile, UserPreferences, UserStats, UserRole};
pub use events::UserEvent;
pub use value_objects::{UserId, Language, Timezone};
