//! # API Module
//!
//! HTTP layer with DTOs, handlers, and routes for the notifications service.

pub mod dto;
pub mod handlers;
pub mod routes;

pub use routes::configure_routes;
