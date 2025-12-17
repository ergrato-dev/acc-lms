//! # Search Error Types
//!
//! Custom error types for the search service.

use std::fmt;

/// Search service errors.
#[derive(Debug)]
pub enum SearchError {
    /// Query is empty or too short
    EmptyQuery,
    /// Query too long
    QueryTooLong { max: usize, actual: usize },
    /// Invalid filter value
    InvalidFilter { field: String, reason: String },
    /// Database error
    Database(String),
    /// Embedding generation failed
    EmbeddingError(String),
    /// Redis/cache error
    CacheError(String),
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Internal error
    Internal(String),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::EmptyQuery => write!(f, "Search query cannot be empty"),
            SearchError::QueryTooLong { max, actual } => {
                write!(f, "Query too long: max {} characters, got {}", max, actual)
            }
            SearchError::InvalidFilter { field, reason } => {
                write!(f, "Invalid filter '{}': {}", field, reason)
            }
            SearchError::Database(msg) => write!(f, "Database error: {}", msg),
            SearchError::EmbeddingError(msg) => write!(f, "Embedding error: {}", msg),
            SearchError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            SearchError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            SearchError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SearchError {}

impl actix_web::ResponseError for SearchError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let status = match self {
            SearchError::EmptyQuery => StatusCode::BAD_REQUEST,
            SearchError::QueryTooLong { .. } => StatusCode::BAD_REQUEST,
            SearchError::InvalidFilter { .. } => StatusCode::BAD_REQUEST,
            SearchError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            SearchError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::EmbeddingError(_) => StatusCode::SERVICE_UNAVAILABLE,
            SearchError::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
            "code": match self {
                SearchError::EmptyQuery => "EMPTY_QUERY",
                SearchError::QueryTooLong { .. } => "QUERY_TOO_LONG",
                SearchError::InvalidFilter { .. } => "INVALID_FILTER",
                SearchError::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
                SearchError::Database(_) => "DATABASE_ERROR",
                SearchError::EmbeddingError(_) => "EMBEDDING_ERROR",
                SearchError::CacheError(_) => "CACHE_ERROR",
                SearchError::Internal(_) => "INTERNAL_ERROR",
            }
        });

        HttpResponse::build(status).json(body)
    }
}

impl From<sqlx::Error> for SearchError {
    fn from(err: sqlx::Error) -> Self {
        SearchError::Database(err.to_string())
    }
}

impl From<redis::RedisError> for SearchError {
    fn from(err: redis::RedisError) -> Self {
        SearchError::CacheError(err.to_string())
    }
}

impl From<async_openai::error::OpenAIError> for SearchError {
    fn from(err: async_openai::error::OpenAIError) -> Self {
        SearchError::EmbeddingError(err.to_string())
    }
}

/// Result type alias for search operations.
pub type SearchResult<T> = Result<T, SearchError>;
