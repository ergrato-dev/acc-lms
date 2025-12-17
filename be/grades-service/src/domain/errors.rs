//! # Domain Errors
//!
//! Error types for the grades service.

use actix_web::{HttpResponse, ResponseError};
use std::fmt;

/// Grades service errors.
#[derive(Debug)]
pub enum GradeError {
    /// Grade entry not found.
    NotFound(String),
    /// User not authorized to access grades.
    Unauthorized(String),
    /// Access forbidden.
    Forbidden(String),
    /// Invalid request data.
    BadRequest(String),
    /// Database error.
    Database(String),
    /// Cache error.
    Cache(String),
    /// Export error.
    Export(String),
    /// Internal server error.
    Internal(String),
}

impl fmt::Display for GradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GradeError::NotFound(msg) => write!(f, "Not found: {}", msg),
            GradeError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            GradeError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            GradeError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            GradeError::Database(msg) => write!(f, "Database error: {}", msg),
            GradeError::Cache(msg) => write!(f, "Cache error: {}", msg),
            GradeError::Export(msg) => write!(f, "Export error: {}", msg),
            GradeError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for GradeError {}

impl ResponseError for GradeError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            GradeError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            GradeError::Unauthorized(msg) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                msg.clone(),
            ),
            GradeError::Forbidden(msg) => (
                actix_web::http::StatusCode::FORBIDDEN,
                msg.clone(),
            ),
            GradeError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            GradeError::Database(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", msg),
            ),
            GradeError::Cache(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cache error: {}", msg),
            ),
            GradeError::Export(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Export error: {}", msg),
            ),
            GradeError::Internal(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        HttpResponse::build(status).json(serde_json::json!({
            "success": false,
            "error": {
                "code": status.as_u16(),
                "message": message
            }
        }))
    }
}

impl From<sqlx::Error> for GradeError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                GradeError::NotFound("Record not found".to_string())
            }
            _ => GradeError::Database(err.to_string()),
        }
    }
}

impl From<redis::RedisError> for GradeError {
    fn from(err: redis::RedisError) -> Self {
        GradeError::Cache(err.to_string())
    }
}

impl From<csv::Error> for GradeError {
    fn from(err: csv::Error) -> Self {
        GradeError::Export(err.to_string())
    }
}
