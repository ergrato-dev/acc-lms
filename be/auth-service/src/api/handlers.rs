//! # Request Handlers
//!
//! HTTP request handlers that bridge the API layer with the service layer.
//! Each handler follows this pattern:
//!
//! 1. **Extract** request data using Actix extractors
//! 2. **Validate** input using the `validator` crate
//! 3. **Call** service layer methods
//! 4. **Transform** and return HTTP response
//!
//! ## Error Handling
//!
//! All handlers return `Result<HttpResponse, ApiError>`. The `ApiError` type
//! automatically converts to appropriate HTTP status codes via the
//! `ResponseError` trait implementation.
//!
//! ## Authentication
//!
//! Protected endpoints extract user info from the JWT token using the
//! `Authorization` header. The token is validated by middleware before
//! reaching the handler.
//!
//! ## Request Flow
//!
//! ```text
//! ┌─────────────┐     ┌────────────┐     ┌─────────────┐     ┌──────────┐
//! │   Request   │────▶│ Middleware │────▶│   Handler   │────▶│ Response │
//! │  (JSON)     │     │ (Auth,Log) │     │ (Validate)  │     │  (JSON)  │
//! └─────────────┘     └────────────┘     └──────┬──────┘     └──────────┘
//!                                               │
//!                                               ▼
//!                                        ┌─────────────┐
//!                                        │   Service   │
//!                                        │   Layer     │
//!                                        └─────────────┘
//! ```
//!
//! ## Related Documentation
//!
//! - DTOs: [`super::dto`]
//! - Routes: [`super::routes`]
//! - Service: [`crate::service::AuthService`]
//! - Errors: [`shared::errors::ApiError`]

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use shared::{errors::ApiError, validation};
use tracing::{info, warn};

use crate::AppState;

use super::dto::{
    AuthResponseDto, ForgotPasswordRequest, HealthResponse, LoginRequest, LogoutRequest,
    MessageResponse, RefreshTokenRequest, RegisterRequest, ResetPasswordRequest,
    TokenResponseDto, UserProfileDto, VerifyEmailRequest,
};

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint for monitoring and load balancers.
///
/// # Route
///
/// `GET /health`
///
/// # Response
///
/// - **200 OK**: Service is healthy
///
/// ```json
/// {
///   "status": "healthy",
///   "service": "auth-service",
///   "timestamp": "2024-01-15T10:30:00Z"
/// }
/// ```
///
/// # Example
///
/// ```bash
/// curl http://localhost:8001/health
/// ```
pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "auth-service".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok().json(response)
}

// =============================================================================
// REGISTRATION
// =============================================================================

/// Registers a new user account.
///
/// # Route
///
/// `POST /api/v1/auth/register`
///
/// # Request Body
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "SecurePass123!",
///   "firstName": "John",
///   "lastName": "Doe"
/// }
/// ```
///
/// # Responses
///
/// - **201 Created**: Registration successful
/// - **400 Bad Request**: Validation failed
/// - **409 Conflict**: Email already registered
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/register \
///   -H "Content-Type: application/json" \
///   -d '{"email":"user@example.com","password":"password123","firstName":"John","lastName":"Doe"}'
/// ```
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request body - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    let response = state
        .auth_service
        .register(&body.email, &body.password, &body.first_name, &body.last_name)
        .await?;

    // Convert to DTO
    let dto = AuthResponseDto {
        access_token: response.tokens.access_token,
        refresh_token: response.tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: response.tokens.expires_in,
        user: response.user.into(),
    };

    Ok(HttpResponse::Created().json(dto))
}

// =============================================================================
// LOGIN
// =============================================================================

/// Authenticates a user and returns tokens.
///
/// # Route
///
/// `POST /api/v1/auth/login`
///
/// # Request Body
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "password123"
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Login successful
/// - **400 Bad Request**: Validation failed
/// - **401 Unauthorized**: Invalid credentials
///
/// # Security
///
/// - Uses constant-time password comparison
/// - Generic error message prevents user enumeration
/// - Device info is extracted from request headers
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/login \
///   -H "Content-Type: application/json" \
///   -d '{"email":"user@example.com","password":"password123"}'
/// ```
pub async fn login(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request body - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Extract device info from request
    let device_fingerprint = req
        .headers()
        .get("X-Device-Fingerprint")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let ip_address = req
        .connection_info()
        .peer_addr()
        .map(String::from);

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Call service layer
    let response = state
        .auth_service
        .login(
            &body.email,
            &body.password,
            device_fingerprint,
            ip_address,
            user_agent,
        )
        .await?;

    // Convert to DTO
    let dto = AuthResponseDto {
        access_token: response.tokens.access_token,
        refresh_token: response.tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: response.tokens.expires_in,
        user: response.user.into(),
    };

    Ok(HttpResponse::Ok().json(dto))
}

// =============================================================================
// TOKEN REFRESH
// =============================================================================

/// Refreshes tokens using a valid refresh token.
///
/// # Route
///
/// `POST /api/v1/auth/refresh`
///
/// # Request Body
///
/// ```json
/// {
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Tokens refreshed
/// - **401 Unauthorized**: Invalid or expired refresh token
///
/// # Security
///
/// Implements token rotation - the old refresh token is invalidated
/// when a new pair is issued.
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/refresh \
///   -H "Content-Type: application/json" \
///   -d '{"refreshToken":"<your_refresh_token>"}'
/// ```
pub async fn refresh_token(
    state: web::Data<AppState>,
    body: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    let tokens = state.auth_service.refresh_token(&body.refresh_token).await?;

    // Convert to DTO
    let dto = TokenResponseDto {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.expires_in,
    };

    Ok(HttpResponse::Ok().json(dto))
}

// =============================================================================
// LOGOUT
// =============================================================================

/// Logs out the current session.
///
/// # Route
///
/// `POST /api/v1/auth/logout`
///
/// # Headers
///
/// - `Authorization: Bearer <access_token>` (required)
///
/// # Request Body
///
/// ```json
/// {
///   "refreshToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Logout successful
/// - **401 Unauthorized**: Invalid or missing access token
///
/// # Security
///
/// - Revokes the refresh token in database
/// - Blacklists the access token in Redis
/// - Clears cached session
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/logout \
///   -H "Authorization: Bearer <access_token>" \
///   -H "Content-Type: application/json" \
///   -d '{"refreshToken":"<refresh_token>"}'
/// ```
pub async fn logout(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<LogoutRequest>,
) -> Result<HttpResponse, ApiError> {
    // Extract and validate access token
    let access_token = extract_bearer_token(&req)?;
    let claims = state
        .jwt_service
        .validate_access_token(&access_token)?;

    // Validate request body - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    state
        .auth_service
        .logout(claims.sub, &access_token, &body.refresh_token)
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Logged out successfully")))
}

/// Logs out from all sessions.
///
/// # Route
///
/// `POST /api/v1/auth/logout-all`
///
/// # Headers
///
/// - `Authorization: Bearer <access_token>` (required)
///
/// # Responses
///
/// - **200 OK**: All sessions terminated
/// - **401 Unauthorized**: Invalid or missing access token
///
/// # Use Cases
///
/// - Security: User suspects account compromise
/// - Privacy: User wants to end all sessions
/// - After password change
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/logout-all \
///   -H "Authorization: Bearer <access_token>"
/// ```
pub async fn logout_all(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    // Extract and validate access token
    let access_token = extract_bearer_token(&req)?;
    let claims = state
        .jwt_service
        .validate_access_token(&access_token)?;

    // Call service layer
    let revoked_count = state
        .auth_service
        .logout_all(claims.sub, &access_token)
        .await?;

    info!(
        user_id = %claims.sub,
        sessions = revoked_count,
        "User logged out from all sessions"
    );

    Ok(HttpResponse::Ok().json(MessageResponse::new(format!(
        "Logged out from {} session(s)",
        revoked_count
    ))))
}

// =============================================================================
// USER PROFILE
// =============================================================================

/// Gets the authenticated user's profile.
///
/// # Route
///
/// `GET /api/v1/auth/me`
///
/// # Headers
///
/// - `Authorization: Bearer <access_token>` (required)
///
/// # Responses
///
/// - **200 OK**: Profile returned
/// - **401 Unauthorized**: Invalid or missing access token
/// - **404 Not Found**: User not found (deleted)
///
/// # Example
///
/// ```bash
/// curl http://localhost:8001/api/v1/auth/me \
///   -H "Authorization: Bearer <access_token>"
/// ```
pub async fn get_profile(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    // Extract and validate access token
    let access_token = extract_bearer_token(&req)?;
    let claims = state
        .jwt_service
        .validate_access_token(&access_token)?;

    // Get profile from service
    let profile = state.auth_service.get_profile(claims.sub).await?;

    // Convert to DTO
    let dto: UserProfileDto = profile.into();

    Ok(HttpResponse::Ok().json(dto))
}

// =============================================================================
// EMAIL VERIFICATION
// =============================================================================

/// Verifies a user's email address.
///
/// # Route
///
/// `POST /api/v1/auth/verify-email`
///
/// # Request Body
///
/// ```json
/// {
///   "token": "abc123def456..."
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Email verified
/// - **400 Bad Request**: Invalid token
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/verify-email \
///   -H "Content-Type: application/json" \
///   -d '{"token":"abc123def456"}'
/// ```
pub async fn verify_email(
    state: web::Data<AppState>,
    body: web::Json<VerifyEmailRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    state.auth_service.verify_email(&body.token).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Email verified successfully")))
}

// =============================================================================
// PASSWORD RESET
// =============================================================================

/// Initiates password reset flow.
///
/// # Route
///
/// `POST /api/v1/auth/forgot-password`
///
/// # Request Body
///
/// ```json
/// {
///   "email": "user@example.com"
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Always returns success (to prevent email enumeration)
///
/// # Security
///
/// - Always returns success, even if email doesn't exist
/// - Reset token is sent via email (handled by notification service)
/// - Token expires in 1 hour
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/forgot-password \
///   -H "Content-Type: application/json" \
///   -d '{"email":"user@example.com"}'
/// ```
pub async fn forgot_password(
    state: web::Data<AppState>,
    body: web::Json<ForgotPasswordRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    match state.auth_service.initiate_password_reset(&body.email).await? {
        Some(token) => {
            // In production, this token would be sent via email
            // For now, just log it (remove in production!)
            info!(
                email = %body.email,
                "Password reset token generated (would be sent via email)"
            );
            // TODO: Send email via notification service
            // notification_service.send_password_reset_email(&body.email, &token).await?;
            let _ = token; // Suppress unused warning
        }
        None => {
            // User doesn't exist, but don't reveal that
            warn!(email = %body.email, "Password reset requested for unknown email");
        }
    }

    // Always return success to prevent email enumeration
    Ok(HttpResponse::Ok().json(MessageResponse::new(
        "If an account exists with this email, a password reset link has been sent",
    )))
}

/// Completes password reset.
///
/// # Route
///
/// `POST /api/v1/auth/reset-password`
///
/// # Request Body
///
/// ```json
/// {
///   "token": "reset_token_here",
///   "newPassword": "NewSecurePass123!"
/// }
/// ```
///
/// # Responses
///
/// - **200 OK**: Password reset successful
/// - **400 Bad Request**: Invalid/expired token or validation failed
///
/// # Security
///
/// - Validates password strength
/// - Invalidates all existing sessions after reset
/// - Token can only be used once
///
/// # Example
///
/// ```bash
/// curl -X POST http://localhost:8001/api/v1/auth/reset-password \
///   -H "Content-Type: application/json" \
///   -d '{"token":"reset_token","newPassword":"NewSecurePass123!"}'
/// ```
pub async fn reset_password(
    state: web::Data<AppState>,
    body: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate request - use into_inner() to get the inner value
    let body = body.into_inner();
    validation::validate_request(&body)?;

    // Call service layer
    state
        .auth_service
        .reset_password(&body.token, &body.new_password)
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new(
        "Password reset successfully. Please log in with your new password.",
    )))
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Extracts Bearer token from Authorization header.
///
/// # Format
///
/// ```text
/// Authorization: Bearer <token>
/// ```
///
/// # Errors
///
/// Returns `ApiError::MissingAuth` if header is missing.
/// Returns `ApiError::InvalidToken` if format is invalid.
fn extract_bearer_token(req: &HttpRequest) -> Result<String, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(ApiError::MissingAuth)?
        .to_str()
        .map_err(|_| ApiError::InvalidToken)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::InvalidToken);
    }

    let token = auth_header.trim_start_matches("Bearer ").to_string();

    if token.is_empty() {
        return Err(ApiError::InvalidToken);
    }

    Ok(token)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_extract_bearer_token_valid() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer my_token_123"))
            .to_http_request();

        let result = extract_bearer_token(&req);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "my_token_123");
    }

    #[test]
    fn test_extract_bearer_token_missing_header() {
        let req = TestRequest::default().to_http_request();

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token_wrong_scheme() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Basic dXNlcjpwYXNz"))
            .to_http_request();

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token_empty() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer "))
            .to_http_request();

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }
}
