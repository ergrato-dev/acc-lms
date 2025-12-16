//! # Domain Module
//!
//! Core domain entities, events, and value objects for the notifications service.

pub mod entities;
pub mod events;
pub mod value_objects;

pub use entities::*;
pub use events::*;
pub use value_objects::*;
