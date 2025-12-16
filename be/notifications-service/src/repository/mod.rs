//! # Repository Module
//!
//! Data access layer for the notifications service.

pub mod notification_repository;

pub use notification_repository::{NotificationRepository, RepositoryError};
