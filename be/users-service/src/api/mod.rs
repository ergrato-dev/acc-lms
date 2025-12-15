//! # API Module
//!
//! Contains the HTTP layer for the users-service.
//! Handles request parsing, validation, and response formatting.
//!
//! ## Structure
//!
//! - `routes`: Route configuration
//! - `handlers`: Request handlers
//! - `dto`: Data Transfer Objects for request/response bodies
//! - `extractors`: Custom extractors for common patterns

pub mod dto;
pub mod handlers;
pub mod routes;
