//! # API Layer
//!
//! HTTP endpoints for enrollments-service.

pub mod dto;
pub mod handlers;
pub mod routes;

pub use dto::*;
pub use handlers::*;
pub use routes::configure_routes;
