//! # Certificate Error Types
//!
//! Custom error types for the certificates service.

use std::fmt;

/// Certificates service errors.
#[derive(Debug)]
pub enum CertificatesError {
    /// Certificate not found
    CertificateNotFound(uuid::Uuid),
    /// Certificate not found by code
    CertificateCodeNotFound(String),
    /// Template not found
    TemplateNotFound(uuid::Uuid),
    /// User not found
    UserNotFound(uuid::Uuid),
    /// Course not found
    CourseNotFound(uuid::Uuid),
    /// Certificate already exists for this user/course
    CertificateAlreadyExists { user_id: uuid::Uuid, course_id: uuid::Uuid },
    /// Course not completed
    CourseNotCompleted { user_id: uuid::Uuid, course_id: uuid::Uuid },
    /// Certificate has been revoked
    CertificateRevoked(uuid::Uuid),
    /// Certificate has expired
    CertificateExpired(uuid::Uuid),
    /// PDF generation failed
    PdfGenerationFailed(String),
    /// Invalid verification code format
    InvalidVerificationCode(String),
    /// Validation error
    Validation(String),
    /// Not found
    NotFound(String),
    /// Forbidden
    Forbidden(String),
    /// Database error
    Database(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for CertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CertificatesError::CertificateNotFound(id) => {
                write!(f, "Certificate not found: {}", id)
            }
            CertificatesError::CertificateCodeNotFound(code) => {
                write!(f, "Certificate not found with code: {}", code)
            }
            CertificatesError::TemplateNotFound(id) => {
                write!(f, "Template not found: {}", id)
            }
            CertificatesError::UserNotFound(id) => {
                write!(f, "User not found: {}", id)
            }
            CertificatesError::CourseNotFound(id) => {
                write!(f, "Course not found: {}", id)
            }
            CertificatesError::CertificateAlreadyExists { user_id, course_id } => {
                write!(f, "Certificate already exists for user {} and course {}", user_id, course_id)
            }
            CertificatesError::CourseNotCompleted { user_id, course_id } => {
                write!(f, "Course {} not completed by user {}", course_id, user_id)
            }
            CertificatesError::CertificateRevoked(id) => {
                write!(f, "Certificate has been revoked: {}", id)
            }
            CertificatesError::CertificateExpired(id) => {
                write!(f, "Certificate has expired: {}", id)
            }
            CertificatesError::PdfGenerationFailed(msg) => {
                write!(f, "PDF generation failed: {}", msg)
            }
            CertificatesError::InvalidVerificationCode(code) => {
                write!(f, "Invalid verification code format: {}", code)
            }
            CertificatesError::Validation(msg) => write!(f, "Validation error: {}", msg),
            CertificatesError::NotFound(msg) => write!(f, "Not found: {}", msg),
            CertificatesError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            CertificatesError::Database(msg) => write!(f, "Database error: {}", msg),
            CertificatesError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CertificatesError {}

impl actix_web::ResponseError for CertificatesError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::http::StatusCode;
        use actix_web::HttpResponse;

        let status = match self {
            CertificatesError::CertificateNotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::CertificateCodeNotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::TemplateNotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::UserNotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::CourseNotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::CertificateAlreadyExists { .. } => StatusCode::CONFLICT,
            CertificatesError::CourseNotCompleted { .. } => StatusCode::PRECONDITION_FAILED,
            CertificatesError::CertificateRevoked(_) => StatusCode::GONE,
            CertificatesError::CertificateExpired(_) => StatusCode::GONE,
            CertificatesError::PdfGenerationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CertificatesError::InvalidVerificationCode(_) => StatusCode::BAD_REQUEST,
            CertificatesError::Validation(_) => StatusCode::BAD_REQUEST,
            CertificatesError::NotFound(_) => StatusCode::NOT_FOUND,
            CertificatesError::Forbidden(_) => StatusCode::FORBIDDEN,
            CertificatesError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CertificatesError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "code": format!("{:?}", self).split('(').next().unwrap_or("Unknown")
        }))
    }
}

impl From<sqlx::Error> for CertificatesError {
    fn from(err: sqlx::Error) -> Self {
        CertificatesError::Database(err.to_string())
    }
}

impl From<redis::RedisError> for CertificatesError {
    fn from(err: redis::RedisError) -> Self {
        CertificatesError::Internal(format!("Redis error: {}", err))
    }
}

/// Result type alias for certificates operations.
pub type CertificatesResult<T> = Result<T, CertificatesError>;
