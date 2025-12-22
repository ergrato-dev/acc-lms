//! # Wishlist Error Types
//!
//! Custom error types for the wishlist service.

use std::fmt;

/// Wishlist service errors.
#[derive(Debug)]
pub enum WishlistError {
    /// Item not found in wishlist
    NotFound(uuid::Uuid),
    /// Course not found
    CourseNotFound(uuid::Uuid),
    /// Course already in wishlist
    AlreadyInWishlist,
    /// Cannot add own course to wishlist
    CannotAddOwnCourse,
    /// User not authorized for this action
    Unauthorized,
    /// Database error
    Database(String),
    /// Cache error
    CacheError(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for WishlistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WishlistError::NotFound(id) => write!(f, "Item not found in wishlist: {}", id),
            WishlistError::CourseNotFound(id) => write!(f, "Course not found: {}", id),
            WishlistError::AlreadyInWishlist => write!(f, "Course is already in your wishlist"),
            WishlistError::CannotAddOwnCourse => write!(f, "Cannot add your own course to wishlist"),
            WishlistError::Unauthorized => write!(f, "Not authorized for this action"),
            WishlistError::Database(msg) => write!(f, "Database error: {}", msg),
            WishlistError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            WishlistError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for WishlistError {}

impl actix_web::ResponseError for WishlistError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let status = match self {
            WishlistError::NotFound(_) => StatusCode::NOT_FOUND,
            WishlistError::CourseNotFound(_) => StatusCode::NOT_FOUND,
            WishlistError::AlreadyInWishlist => StatusCode::CONFLICT,
            WishlistError::CannotAddOwnCourse => StatusCode::FORBIDDEN,
            WishlistError::Unauthorized => StatusCode::FORBIDDEN,
            WishlistError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WishlistError::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WishlistError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
            "code": match self {
                WishlistError::NotFound(_) => "ITEM_NOT_FOUND",
                WishlistError::CourseNotFound(_) => "COURSE_NOT_FOUND",
                WishlistError::AlreadyInWishlist => "ALREADY_IN_WISHLIST",
                WishlistError::CannotAddOwnCourse => "CANNOT_ADD_OWN_COURSE",
                WishlistError::Unauthorized => "UNAUTHORIZED",
                WishlistError::Database(_) => "DATABASE_ERROR",
                WishlistError::CacheError(_) => "CACHE_ERROR",
                WishlistError::Internal(_) => "INTERNAL_ERROR",
            }
        });

        HttpResponse::build(status).json(body)
    }
}

impl From<sqlx::Error> for WishlistError {
    fn from(err: sqlx::Error) -> Self {
        WishlistError::Database(err.to_string())
    }
}

impl From<redis::RedisError> for WishlistError {
    fn from(err: redis::RedisError) -> Self {
        WishlistError::CacheError(err.to_string())
    }
}

/// Result type alias for wishlist operations.
pub type WishlistResult<T> = Result<T, WishlistError>;
