//! # API Module
//!
//! HTTP API layer for search service.
//!
//! ## Modules
//!
//! - `dto`: Data transfer objects (request/response)
//! - `handlers`: HTTP request handlers

pub mod dto;
pub mod handlers;

pub use dto::*;
pub use handlers::*;
