//! # Data Transfer Objects (DTOs)
//!
//! DTOs define the structure of data transferred between client and server.
//! They serve as the **API contract** and handle:
//!
//! - **Request validation**: Using the `validator` crate
//! - **Deserialization**: JSON → Rust structs via `serde`
//! - **Serialization**: Rust structs → JSON for responses
//!
//! ## DTO Categories
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                              DTOs                                       │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │  Request DTOs (input)              Response DTOs (output)               │
//! │  ───────────────────               ─────────────────────                │
//! │  - RegisterRequest                 - AuthResponse                       │
//! │  - LoginRequest                    - TokenResponse                      │
//! │  - RefreshTokenRequest             - ProfileResponse                    │
//! │  - VerifyEmailRequest              - MessageResponse                    │
//! │  - ForgotPasswordRequest           - HealthResponse                     │
//! │  - ResetPasswordRequest                                                 │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Validation Rules
//!
//! | Field        | Rules                                              |
//! |--------------|-----------------------------------------------------|
//! | `email`      | Valid email format, max 255 chars                   |
//! | `password`   | Min 8 chars, max 128 chars                          |
//! | `first_name` | Non-blank, max 100 chars                            |
//! | `last_name`  | Non-blank, max 100 chars                            |
//! | `token`      | Non-blank                                           |
//!
//! ## JSON Naming Convention
//!
//! All DTOs use `camelCase` for JSON serialization to match JavaScript
//! conventions on the frontend.
//!
//! ## Related Documentation
//!
//! - Validation module: [`shared::validation`]
//! - API handlers: [`super::handlers`]

use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================
// REGISTRATION
// =============================================================================

/// Request body for user registration.
///
/// # Validation
///
/// - `email`: Valid email format, max 255 characters
/// - `password`: 8-128 characters
/// - `first_name`: 1-100 characters, not blank
/// - `last_name`: 1-100 characters, not blank
///
/// # Example JSON
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "SecurePass123!",
///   "firstName": "John",
///   "lastName": "Doe"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    /// Plain text password
    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,

    /// User's first name
    #[validate(length(min = 1, max = 100, message = "First name must be 1-100 characters"))]
    pub first_name: String,

    /// User's last name
    #[validate(length(min = 1, max = 100, message = "Last name must be 1-100 characters"))]
    pub last_name: String,
}

// =============================================================================
// LOGIN
// =============================================================================

/// Request body for user login.
///
/// # Example JSON
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "password123"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// User's email address
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    /// Plain text password
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

// =============================================================================
// TOKEN REFRESH
// =============================================================================

/// Request body for token refresh.
///
/// # Example JSON
///
/// ```json
/// {
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    /// The refresh token to exchange for new tokens
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

// =============================================================================
// LOGOUT
// =============================================================================

/// Request body for logout.
///
/// # Example JSON
///
/// ```json
/// {
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    /// The refresh token to revoke
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

// =============================================================================
// EMAIL VERIFICATION
// =============================================================================

/// Request body for email verification.
///
/// # Example JSON
///
/// ```json
/// {
///   "token": "abc123def456..."
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailRequest {
    /// Verification token from the email link
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

// =============================================================================
// PASSWORD RESET
// =============================================================================

/// Request body for initiating password reset.
///
/// # Example JSON
///
/// ```json
/// {
///   "email": "user@example.com"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPasswordRequest {
    /// Email address of the account
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Request body for completing password reset.
///
/// # Example JSON
///
/// ```json
/// {
///   "token": "reset_token_here",
///   "newPassword": "NewSecurePass123!"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    /// Password reset token from email
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    /// New password
    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub new_password: String,
}

// =============================================================================
// RESPONSE TYPES
// =============================================================================

/// Response containing authentication tokens and user profile.
///
/// Returned on successful registration or login.
///
/// # Example JSON
///
/// ```json
/// {
///   "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "tokenType": "Bearer",
///   "expiresIn": 900,
///   "user": {
///     "userId": "550e8400-e29b-41d4-a716-446655440000",
///     "email": "user@example.com",
///     "firstName": "John",
///     "lastName": "Doe",
///     "role": "student",
///     "emailVerified": false
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseDto {
    /// JWT access token for API requests
    pub access_token: String,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Access token lifetime in seconds
    pub expires_in: i64,
    /// User profile information
    pub user: UserProfileDto,
}

/// Response containing only tokens (for refresh endpoint).
///
/// # Example JSON
///
/// ```json
/// {
///   "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "tokenType": "Bearer",
///   "expiresIn": 900
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponseDto {
    /// New JWT access token
    pub access_token: String,
    /// New refresh token (rotation)
    pub refresh_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Access token lifetime in seconds
    pub expires_in: i64,
}

/// User profile in API responses (safe, no sensitive data).
///
/// # Example JSON
///
/// ```json
/// {
///   "userId": "550e8400-e29b-41d4-a716-446655440000",
///   "email": "user@example.com",
///   "firstName": "John",
///   "lastName": "Doe",
///   "role": "student",
///   "avatarUrl": null,
///   "bio": "Learning enthusiast",
///   "timezone": "UTC",
///   "languagePreference": "en",
///   "emailVerified": true,
///   "lastLoginAt": "2024-01-15T10:30:00Z",
///   "createdAt": "2024-01-01T00:00:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileDto {
    /// Unique user identifier
    pub user_id: String,
    /// User's email address
    pub email: String,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User role (student, instructor, admin)
    pub role: String,
    /// Optional avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Optional bio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// User's timezone
    pub timezone: String,
    /// Preferred language
    pub language_preference: String,
    /// Whether email is verified
    pub email_verified: bool,
    /// Last login timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<String>,
    /// Account creation timestamp
    pub created_at: String,
}

/// Generic message response for simple confirmations.
///
/// # Example JSON
///
/// ```json
/// {
///   "message": "Password reset email sent"
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    /// Human-readable message
    pub message: String,
}

impl MessageResponse {
    /// Creates a new message response.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Health check response.
///
/// # Example JSON
///
/// ```json
/// {
///   "status": "healthy",
///   "service": "auth-service",
///   "timestamp": "2024-01-15T10:30:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    /// Service health status
    pub status: String,
    /// Service name
    pub service: String,
    /// Current timestamp
    pub timestamp: String,
}

// =============================================================================
// CONVERSIONS
// =============================================================================

use crate::domain::UserProfile;

impl From<UserProfile> for UserProfileDto {
    /// Converts domain `UserProfile` to API DTO.
    ///
    /// Handles timestamp formatting and UUID serialization.
    fn from(profile: UserProfile) -> Self {
        Self {
            user_id: profile.user_id.to_string(),
            email: profile.email,
            first_name: profile.first_name,
            last_name: profile.last_name,
            role: profile.role,
            avatar_url: profile.avatar_url,
            bio: profile.bio,
            timezone: profile.timezone,
            language_preference: profile.language_preference,
            email_verified: profile.email_verified,
            last_login_at: profile.last_login_at.map(|dt| dt.to_rfc3339()),
            created_at: profile.created_at.to_rfc3339(),
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_register_request_valid() {
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_register_request_invalid_email() {
        let request = RegisterRequest {
            email: "not-an-email".to_string(),
            password: "password123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_register_request_short_password() {
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn test_login_request_valid() {
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_message_response_new() {
        let response = MessageResponse::new("Test message");
        assert_eq!(response.message, "Test message");
    }

    #[test]
    fn test_json_serialization_camel_case() {
        let response = MessageResponse::new("Hello");
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("message")); // camelCase preserved
    }
}
