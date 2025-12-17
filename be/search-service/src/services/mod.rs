//! # Services Module
//!
//! Business logic layer for search operations.
//!
//! ## Modules
//!
//! - `search`: Main course search service
//! - `semantic_search`: Semantic/vector search service
//! - `content_search`: Content search within courses
//! - `suggestion`: Autocomplete and suggestions

pub mod search;
pub mod semantic_search;
pub mod content_search;
pub mod suggestion;

pub use search::SearchService;
pub use semantic_search::SemanticSearchService;
pub use content_search::ContentSearchService;
pub use suggestion::SuggestionService;
