//! # Courses API Module
//!
//! HTTP layer for the courses service.

pub mod dto;
pub mod handlers;
pub mod routes;

pub use routes::configure_routes;
