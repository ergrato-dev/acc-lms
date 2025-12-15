//! # Service Module
//!
//! Contains the business logic layer for the users-service.
//! Services orchestrate domain operations and repository calls.
//!
//! ## Design Pattern: Service Layer
//!
//! The service layer:
//! - Encapsulates business logic
//! - Orchestrates multiple repository calls
//! - Handles cross-cutting concerns (events, caching)
//! - Provides a clean API for handlers

mod user_service;

pub use user_service::{UserService, UpdateProfileRequest, UpdatePreferencesRequest};
