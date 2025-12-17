//! # Repository Module
//!
//! Data access layer for search service.
//!
//! ## Modules
//!
//! - `course_search`: Course search repository
//! - `content_search`: Content search repository
//! - `embedding`: Vector embedding repository

pub mod course_search;
pub mod content_search;
pub mod embedding;

pub use course_search::CourseSearchRepository;
pub use content_search::ContentSearchRepository;
pub use embedding::EmbeddingRepository;
