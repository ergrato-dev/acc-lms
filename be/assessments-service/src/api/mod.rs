//! # API Layer
//!
//! HTTP handlers, DTOs, and route configuration.

pub mod dto;
pub mod handlers;
pub mod routes;

pub use routes::configure_routes;
