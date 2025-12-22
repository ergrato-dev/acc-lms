//! # API Layer
//!
//! HTTP handlers and DTOs for the wishlist service.

pub mod dto;
pub mod handlers;

pub use handlers::configure_routes;
