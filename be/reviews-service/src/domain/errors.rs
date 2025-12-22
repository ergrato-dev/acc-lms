//! # Review Error Types
//!
//! Custom error types for the reviews service.

use std::fmt;

/// Reviews service errors.
#[derive(Debug)]
pub enum ReviewError {
    /// Review not found
    NotFound(uuid::Uuid),
    /// User already reviewed this course
    AlreadyReviewed,
    /// User not enrolled in course
    NotEnrolled,
    /// User cannot review own course
    CannotReviewOwnCourse,
    /// Invalid rating value
    InvalidRating { value: i16 },
    /// Review content too short
    ContentTooShort { min: usize },
    /// Review content too long
    ContentTooLong { max: usize },
    /// User not authorized for this action
    Unauthorized,
    /// Cannot modify review (status doesn't allow)
    CannotModify,
    /// Already voted on this review
    AlreadyVoted,
    /// Database error
    Database(String),
    /// Cache error
    CacheError(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for ReviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReviewError::NotFound(id) => write!(f, "Review not found: {}", id),
            ReviewError::AlreadyReviewed => write!(f, "You have already reviewed this course"),
            ReviewError::NotEnrolled => write!(f, "You must be enrolled in the course to review it"),
            ReviewError::CannotReviewOwnCourse => write!(f, "You cannot review your own course"),
            ReviewError::InvalidRating { value } => {
                write!(f, "Invalid rating: {}. Must be between 1 and 5", value)
            }
            ReviewError::ContentTooShort { min } => {
                write!(f, "Review content too short. Minimum {} characters", min)
            }
            ReviewError::ContentTooLong { max } => {
                write!(f, "Review content too long. Maximum {} characters", max)
            }
            ReviewError::Unauthorized => write!(f, "Not authorized for this action"),
            ReviewError::CannotModify => write!(f, "Cannot modify this review"),
            ReviewError::AlreadyVoted => write!(f, "You have already voted on this review"),
            ReviewError::Database(msg) => write!(f, "Database error: {}", msg),
            ReviewError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            ReviewError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ReviewError {}

impl actix_web::ResponseError for ReviewError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let status = match self {
            ReviewError::NotFound(_) => StatusCode::NOT_FOUND,
            ReviewError::AlreadyReviewed => StatusCode::CONFLICT,
            ReviewError::NotEnrolled => StatusCode::FORBIDDEN,
            ReviewError::CannotReviewOwnCourse => StatusCode::FORBIDDEN,
            ReviewError::InvalidRating { .. } => StatusCode::BAD_REQUEST,
            ReviewError::ContentTooShort { .. } => StatusCode::BAD_REQUEST,
            ReviewError::ContentTooLong { .. } => StatusCode::BAD_REQUEST,
            ReviewError::Unauthorized => StatusCode::FORBIDDEN,
            ReviewError::CannotModify => StatusCode::FORBIDDEN,
            ReviewError::AlreadyVoted => StatusCode::CONFLICT,
            ReviewError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ReviewError::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ReviewError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
            "code": match self {
                ReviewError::NotFound(_) => "REVIEW_NOT_FOUND",
                ReviewError::AlreadyReviewed => "ALREADY_REVIEWED",
                ReviewError::NotEnrolled => "NOT_ENROLLED",
                ReviewError::CannotReviewOwnCourse => "CANNOT_REVIEW_OWN_COURSE",
                ReviewError::InvalidRating { .. } => "INVALID_RATING",
                ReviewError::ContentTooShort { .. } => "CONTENT_TOO_SHORT",
                ReviewError::ContentTooLong { .. } => "CONTENT_TOO_LONG",
                ReviewError::Unauthorized => "UNAUTHORIZED",
                ReviewError::CannotModify => "CANNOT_MODIFY",
                ReviewError::AlreadyVoted => "ALREADY_VOTED",
                ReviewError::Database(_) => "DATABASE_ERROR",
                ReviewError::CacheError(_) => "CACHE_ERROR",
                ReviewError::Internal(_) => "INTERNAL_ERROR",
            }
        });

        HttpResponse::build(status).json(body)
    }
}

impl From<sqlx::Error> for ReviewError {
    fn from(err: sqlx::Error) -> Self {
        ReviewError::Database(err.to_string())
    }
}

impl From<redis::RedisError> for ReviewError {
    fn from(err: redis::RedisError) -> Self {
        ReviewError::CacheError(err.to_string())
    }
}

/// Result type alias for review operations.
pub type ReviewResult<T> = Result<T, ReviewError>;
