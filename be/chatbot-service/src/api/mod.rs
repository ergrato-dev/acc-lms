//! # API Module

pub mod dto;
pub mod handlers;
pub mod routes;

pub use handlers::AppState;
pub use routes::configure_routes;
