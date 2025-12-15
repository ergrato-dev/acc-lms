//! # Application Error Types
//!
//! Unified error handling for all microservices with automatic HTTP conversion.
//!
//! ## Design Philosophy
//!
//! This module follows the "make illegal states unrepresentable" principle.
//! Each error variant maps to a specific HTTP status code and error code,
//! ensuring consistent API responses across all services.
//!
//! ## Error Categories
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                          ApiError Categories                             │
//! ├─────────────────────────┬───────────────────┬────────────────────────────┤
//! │ Authentication (401)    │ Authorization(403)│ Validation (400)           │
//! ├─────────────────────────┼───────────────────┼────────────────────────────┤
//! │ InvalidCredentials      │ InsufficientPerms │ ValidationError            │
//! │ TokenExpired            │ AccessDenied      │ BadRequest                 │
//! │ InvalidToken            │                   │ InvalidUuid                │
//! │ MissingAuth             │                   │                            │
//! │ AccountLocked           │                   │                            │
//! ├─────────────────────────┴───────────────────┴────────────────────────────┤
//! │ Resources (404, 409)    │ Rate Limit (429)  │ Server (500, 503)          │
//! ├─────────────────────────┼───────────────────┼────────────────────────────┤
//! │ NotFound                │ TooManyRequests   │ DatabaseError              │
//! │ Conflict                │                   │ RedisError                 │
//! │                         │                   │ InternalError              │
//! │                         │                   │ ServiceUnavailable         │
//! └─────────────────────────┴───────────────────┴────────────────────────────┘
//! ```
//!
//! ## HTTP Response Format
//!
//! All errors are serialized to a consistent JSON format:
//!
//! ```json
//! {
//!   "code": "VALIDATION_ERROR",
//!   "message": "Validation error",
//!   "request_id": "550e8400-e29b-41d4-a716-446655440000",
//!   "details": { ... },
//!   "timestamp": "2024-01-15T10:30:00Z"
//! }
//! ```
//!
//! ## Framework Integration
//!
//! This module provides automatic conversion for both Actix-web and Axum:
//!
//! - **Actix-web**: `impl ResponseError for ApiError`
//! - **Axum**: `impl IntoResponse for ApiError`
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::errors::{ApiError, ApiResult};
//!
//! async fn get_user(id: Uuid) -> ApiResult<User> {
//!     let user = repo.find_by_id(id)
//!         .await
//!         .map_err(|e| ApiError::from(e))?  // DatabaseError → 500
//!         .ok_or(ApiError::NotFound {
//!             resource: format!("user:{}", id),
//!         })?;  // NotFound → 404
//!     
//!     Ok(user)
//! }
//! ```
//!
//! ## Related Documentation
//!
//! - See [`_docs/development/development-standards.md`] for error handling guidelines
//! - See [`_docs/business/functional-requirements.md`] for error code specifications
//! - See [`auth/jwt`](crate::auth::jwt) for token-related error handling

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::ValidationErrors;

// =============================================================================
// Type Aliases
// =============================================================================

/// Result type alias for handlers and services.
///
/// Use this instead of `Result<T, ApiError>` for cleaner signatures:
///
/// ```rust,ignore
/// async fn handler() -> ApiResult<User> { ... }
/// ```
pub type ApiResult<T> = Result<T, ApiError>;

// =============================================================================
// Main Error Enum
// =============================================================================

/// Application error type with automatic HTTP status mapping.
///
/// Each variant represents a specific error condition and maps to an
/// appropriate HTTP status code. The `#[error]` attribute defines the
/// error message format.
///
/// ## Adding New Variants
///
/// When adding new error types:
/// 1. Choose the appropriate HTTP status code
/// 2. Add the variant with `#[error("...")]` for message format
/// 3. Update `status_code()` method
/// 4. Update `error_code()` method
/// 5. Update `is_server_error()` if applicable
/// 6. Add tests
#[derive(Debug, Error)]
pub enum ApiError {
    // =========================================================================
    // Authentication Errors (401 Unauthorized)
    // =========================================================================
    // These indicate the user is not authenticated or their credentials are invalid.
    // The client should prompt the user to log in again.

    /// Wrong email/password combination.
    /// Returns 401 Unauthorized.
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// JWT access token has expired.
    /// Client should use refresh token to get new access token.
    /// Returns 401 Unauthorized.
    #[error("Token expired")]
    TokenExpired,

    /// JWT token is malformed, has invalid signature, or has been tampered with.
    /// Returns 401 Unauthorized.
    #[error("Invalid token")]
    InvalidToken,

    /// Request doesn't include required authentication header.
    /// Returns 401 Unauthorized.
    #[error("Missing authentication")]
    MissingAuth,

    /// Account temporarily locked due to too many failed login attempts.
    /// Implements brute-force protection (RF-AUTH-004).
    /// Returns 401 Unauthorized with `locked_until` in details.
    #[error("Account locked")]
    AccountLocked {
        /// When the account will be automatically unlocked
        until: chrono::DateTime<chrono::Utc>,
    },

    // =========================================================================
    // Authorization Errors (403 Forbidden)
    // =========================================================================
    // User is authenticated but lacks permission for the requested action.
    // Unlike 401, re-authenticating won't help.

    /// User's role doesn't allow this action.
    /// Example: Student trying to access admin endpoints.
    /// Returns 403 Forbidden.
    #[error("Insufficient permissions")]
    InsufficientPermissions,

    /// User can't access this specific resource (ownership check failed).
    /// Example: User A trying to view User B's private data.
    /// Returns 403 Forbidden.
    #[error("Resource access denied")]
    AccessDenied,

    // =========================================================================
    // Validation Errors (400 Bad Request)
    // =========================================================================
    // Request is syntactically correct but semantically invalid.

    /// Request body failed validation rules.
    /// Contains field-level errors from the `validator` crate.
    /// Returns 400 Bad Request with details.
    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),

    /// Generic bad request with custom message.
    /// Use for validation that doesn't fit the validator pattern.
    /// Returns 400 Bad Request.
    #[error("Invalid input: {message}")]
    BadRequest {
        /// Human-readable description of what's wrong
        message: String,
    },

    /// UUID parsing failed.
    /// Returns 400 Bad Request.
    #[error("Invalid UUID format")]
    InvalidUuid,

    // =========================================================================
    // Resource Errors (404 Not Found, 409 Conflict)
    // =========================================================================

    /// Requested resource doesn't exist.
    /// Returns 404 Not Found.
    #[error("Resource not found: {resource}")]
    NotFound {
        /// Resource identifier (e.g., "user:123", "course:abc")
        resource: String,
    },

    /// Action would create duplicate or violate uniqueness constraint.
    /// Example: Registering with an email that's already in use.
    /// Returns 409 Conflict.
    #[error("Resource already exists: {resource}")]
    Conflict {
        /// Description of the conflict (e.g., "email already registered")
        resource: String,
    },

    // =========================================================================
    // Rate Limiting (429 Too Many Requests)
    // =========================================================================

    /// Client has exceeded request rate limits.
    /// Implements protection against abuse (RNF-006).
    /// Returns 429 Too Many Requests with `retry_after_seconds` in details.
    #[error("Too many requests")]
    TooManyRequests {
        /// Seconds until the client can retry
        retry_after_seconds: u64,
    },

    // =========================================================================
    // Server Errors (500 Internal Server Error, 503 Service Unavailable)
    // =========================================================================
    // These are logged as errors and monitored.
    // Details are NOT exposed to clients for security.

    /// PostgreSQL query failed.
    /// Wraps `sqlx::Error`. Details logged, generic message returned.
    /// Returns 500 Internal Server Error.
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    /// Redis operation failed.
    /// Wraps `redis::RedisError`. Details logged, generic message returned.
    /// Returns 500 Internal Server Error.
    #[error("Redis error")]
    RedisError(#[from] redis::RedisError),

    /// Unspecified internal error.
    /// Use as last resort when no specific variant applies.
    /// Returns 500 Internal Server Error.
    #[error("Internal server error")]
    InternalError {
        /// Internal message for logging (not exposed to client)
        message: String,
    },

    /// External service (payment gateway, email provider, etc.) is down.
    /// Returns 503 Service Unavailable.
    #[error("Service unavailable")]
    ServiceUnavailable {
        /// Name of the unavailable service
        service: String,
    },
}

// =============================================================================
// Error Methods
// =============================================================================

impl ApiError {
    /// Returns a machine-readable error code.
    ///
    /// These codes are stable and can be used by API clients for
    /// programmatic error handling. They follow the format:
    /// `CATEGORY_SPECIFIC_ERROR` in SCREAMING_SNAKE_CASE.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let error = ApiError::InvalidCredentials;
    /// assert_eq!(error.error_code(), "INVALID_CREDENTIALS");
    /// ```
    pub fn error_code(&self) -> &'static str {
        match self {
            // Authentication
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::MissingAuth => "MISSING_AUTH",
            Self::AccountLocked { .. } => "ACCOUNT_LOCKED",
            // Authorization
            Self::InsufficientPermissions => "INSUFFICIENT_PERMISSIONS",
            Self::AccessDenied => "ACCESS_DENIED",
            // Validation
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::BadRequest { .. } => "BAD_REQUEST",
            Self::InvalidUuid => "INVALID_UUID",
            // Resources
            Self::NotFound { .. } => "NOT_FOUND",
            Self::Conflict { .. } => "CONFLICT",
            // Rate limiting
            Self::TooManyRequests { .. } => "TOO_MANY_REQUESTS",
            // Server
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::RedisError(_) => "REDIS_ERROR",
            Self::InternalError { .. } => "INTERNAL_ERROR",
            Self::ServiceUnavailable { .. } => "SERVICE_UNAVAILABLE",
        }
    }

    /// Returns the HTTP status code for this error.
    ///
    /// This is the source of truth for error-to-status mapping.
    /// Both Actix-web and Axum implementations use this method.
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 401 Unauthorized - Authentication required or failed
            Self::InvalidCredentials
            | Self::TokenExpired
            | Self::InvalidToken
            | Self::MissingAuth
            | Self::AccountLocked { .. } => StatusCode::UNAUTHORIZED,

            // 403 Forbidden - Authenticated but not authorized
            Self::InsufficientPermissions | Self::AccessDenied => StatusCode::FORBIDDEN,

            // 400 Bad Request - Client sent invalid data
            Self::ValidationError(_) | Self::BadRequest { .. } | Self::InvalidUuid => {
                StatusCode::BAD_REQUEST
            }

            // 404 Not Found - Resource doesn't exist
            Self::NotFound { .. } => StatusCode::NOT_FOUND,

            // 409 Conflict - Would violate uniqueness/business rules
            Self::Conflict { .. } => StatusCode::CONFLICT,

            // 429 Too Many Requests - Rate limit exceeded
            Self::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,

            // 500 Internal Server Error - Something went wrong on our side
            Self::DatabaseError(_)
            | Self::RedisError(_)
            | Self::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,

            // 503 Service Unavailable - Dependency is down
            Self::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Returns `true` if this is a server-side error (5xx).
    ///
    /// Server errors should be:
    /// - Logged at ERROR level
    /// - Monitored and alerted
    /// - Not expose internal details to clients
    ///
    /// Client errors (4xx) are logged at WARN level.
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::DatabaseError(_)
                | Self::RedisError(_)
                | Self::InternalError { .. }
                | Self::ServiceUnavailable { .. }
        )
    }
}

// =============================================================================
// Response Structure
// =============================================================================

/// Standard error response body for the API.
///
/// All API errors are serialized to this format for consistency.
/// This structure is returned as the JSON body of error responses.
///
/// ## Fields
///
/// - `code`: Machine-readable error code (e.g., "VALIDATION_ERROR")
/// - `message`: Human-readable message (for debugging, not for UI)
/// - `request_id`: UUID for tracing requests across services
/// - `details`: Additional context (validation errors, retry time, etc.)
/// - `timestamp`: When the error occurred (ISO 8601 format)
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Machine-readable error code for programmatic handling
    pub code: String,
    
    /// Human-readable message (for debugging, not for user display)
    pub message: String,
    
    /// Request ID for tracing across services.
    /// Useful for correlating logs and support requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    
    /// Additional error details (varies by error type).
    /// - ValidationError: Field-level errors
    /// - TooManyRequests: `retry_after_seconds`
    /// - AccountLocked: `locked_until`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    
    /// When the error occurred (ISO 8601 format)
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    /// Creates a new error response from an `ApiError`.
    ///
    /// Automatically extracts relevant details based on error type:
    /// - ValidationError: Includes field-level validation errors
    /// - TooManyRequests: Includes retry delay
    /// - AccountLocked: Includes unlock timestamp
    pub fn new(error: &ApiError) -> Self {
        // Extract type-specific details
        let details = match error {
            ApiError::ValidationError(errors) => {
                Some(serde_json::to_value(errors).unwrap_or_default())
            }
            ApiError::TooManyRequests { retry_after_seconds } => {
                Some(serde_json::json!({ "retry_after_seconds": retry_after_seconds }))
            }
            ApiError::AccountLocked { until } => {
                Some(serde_json::json!({ "locked_until": until }))
            }
            // Server errors: Don't expose internal details
            ApiError::DatabaseError(_) | ApiError::RedisError(_) | ApiError::InternalError { .. } => {
                // Log the actual error but don't expose to client
                None
            }
            _ => None,
        };

        Self {
            code: error.error_code().to_string(),
            message: error.to_string(),
            request_id: None,
            details,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Adds a request ID for tracing.
    ///
    /// The request ID should be generated at the API gateway or middleware
    /// and passed through all services for distributed tracing.
    pub fn with_request_id(mut self, request_id: Uuid) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }
}

// =============================================================================
// Actix-web Integration
// =============================================================================

/// Implements Actix-web's `ResponseError` trait.
///
/// This allows returning `ApiError` directly from handlers:
///
/// ```rust,ignore
/// async fn handler() -> Result<impl Responder, ApiError> {
///     Err(ApiError::NotFound { resource: "user:123".into() })
/// }
/// ```
///
/// The error is automatically converted to an HTTP response with:
/// - Appropriate status code
/// - JSON body with `ErrorResponse` structure
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        // Delegate to our implementation
        self.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse::new(self);
        HttpResponse::build(self.status_code()).json(response)
    }
}

// =============================================================================
// Axum Integration
// =============================================================================

/// Implements Axum's `IntoResponse` trait.
///
/// This allows returning `ApiError` directly from handlers:
///
/// ```rust,ignore
/// async fn handler() -> Result<Json<User>, ApiError> {
///     Err(ApiError::NotFound { resource: "user:123".into() })
/// }
/// ```
///
/// The error is automatically converted to an HTTP response with:
/// - Appropriate status code
/// - JSON body with `ErrorResponse` structure
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Convert actix_web StatusCode to axum's StatusCode
        // (They're different types from different crates)
        let status = match self.status_code() {
            StatusCode::UNAUTHORIZED => axum::http::StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN => axum::http::StatusCode::FORBIDDEN,
            StatusCode::BAD_REQUEST => axum::http::StatusCode::BAD_REQUEST,
            StatusCode::NOT_FOUND => axum::http::StatusCode::NOT_FOUND,
            StatusCode::CONFLICT => axum::http::StatusCode::CONFLICT,
            StatusCode::TOO_MANY_REQUESTS => axum::http::StatusCode::TOO_MANY_REQUESTS,
            StatusCode::SERVICE_UNAVAILABLE => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let response = ErrorResponse::new(&self);
        (status, axum::Json(response)).into_response()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_errors_return_401() {
        assert_eq!(ApiError::InvalidCredentials.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::TokenExpired.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::InvalidToken.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::MissingAuth.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_authorization_errors_return_403() {
        assert_eq!(ApiError::InsufficientPermissions.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(ApiError::AccessDenied.status_code(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_resource_errors() {
        assert_eq!(
            ApiError::NotFound { resource: "user".to_string() }.status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::Conflict { resource: "email".to_string() }.status_code(),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn test_error_codes_are_screaming_snake_case() {
        assert_eq!(ApiError::InvalidCredentials.error_code(), "INVALID_CREDENTIALS");
        assert_eq!(ApiError::NotFound { resource: "test".to_string() }.error_code(), "NOT_FOUND");
        assert_eq!(ApiError::TooManyRequests { retry_after_seconds: 60 }.error_code(), "TOO_MANY_REQUESTS");
    }

    #[test]
    fn test_server_errors_are_flagged() {
        assert!(ApiError::InternalError { message: "test".to_string() }.is_server_error());
        assert!(ApiError::ServiceUnavailable { service: "test".to_string() }.is_server_error());
        assert!(!ApiError::InvalidCredentials.is_server_error());
        assert!(!ApiError::NotFound { resource: "test".to_string() }.is_server_error());
    }

    #[test]
    fn test_error_response_includes_timestamp() {
        let error = ApiError::InvalidCredentials;
        let response = ErrorResponse::new(&error);
        
        // Timestamp should be recent (within last minute)
        let now = chrono::Utc::now();
        let diff = now - response.timestamp;
        assert!(diff.num_seconds() < 60);
    }

    #[test]
    fn test_error_response_with_request_id() {
        let error = ApiError::InvalidCredentials;
        let request_id = Uuid::new_v4();
        let response = ErrorResponse::new(&error).with_request_id(request_id);
        
        assert_eq!(response.request_id, Some(request_id.to_string()));
    }
}

