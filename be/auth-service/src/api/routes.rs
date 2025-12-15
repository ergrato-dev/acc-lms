//! # Route Configuration
//!
//! Defines URL patterns and maps them to handler functions. Routes are
//! organized by resource and follow RESTful conventions where appropriate.
//!
//! ## Route Structure
//!
//! ```text
//! /
//! ├── health                           GET  → health_check
//! │
//! └── api/v1/auth/
//!     ├── register                     POST → register
//!     ├── login                        POST → login
//!     ├── refresh                      POST → refresh_token
//!     ├── logout                       POST → logout
//!     ├── logout-all                   POST → logout_all
//!     ├── me                           GET  → get_profile
//!     ├── verify-email                 POST → verify_email
//!     ├── forgot-password              POST → forgot_password
//!     └── reset-password               POST → reset_password
//! ```
//!
//! ## Versioning
//!
//! All auth endpoints are versioned under `/api/v1/`. This allows for
//! backward-compatible API evolution. Future breaking changes would use
//! `/api/v2/`.
//!
//! ## Authentication
//!
//! Routes are either:
//! - **Public**: No authentication required (`register`, `login`, etc.)
//! - **Protected**: Requires valid JWT in `Authorization: Bearer <token>` header
//!
//! Protected routes use the `AuthMiddleware` from [`shared::auth::middleware`].
//!
//! ## Related Documentation
//!
//! - Handler implementations: [`super::handlers`]
//! - Auth middleware: [`shared::auth::middleware`]

use actix_web::web;

use super::handlers;

/// Configures all routes for the auth service.
///
/// Called from `main.rs` during app initialization:
///
/// ```rust,ignore
/// App::new()
///     .configure(routes::configure)
/// ```
///
/// # Route Groups
///
/// 1. **Health check** (`/health`) - Service status
/// 2. **Auth endpoints** (`/api/v1/auth/*`) - Authentication operations
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // ─────────────────────────────────────────────────────────────────
        // Health Check
        // ─────────────────────────────────────────────────────────────────
        // Simple endpoint for load balancers and monitoring systems.
        // Returns 200 OK if service is running.
        .route("/health", web::get().to(handlers::health_check))
        // ─────────────────────────────────────────────────────────────────
        // Auth API v1
        // ─────────────────────────────────────────────────────────────────
        .service(
            web::scope("/api/v1/auth")
                // ─────────────────────────────────────────────────────────
                // Public Routes (no authentication required)
                // ─────────────────────────────────────────────────────────
                //
                // POST /api/v1/auth/register
                // Creates a new user account
                // Request: RegisterRequest { email, password, firstName, lastName }
                // Response: AuthResponseDto { accessToken, refreshToken, user }
                .route("/register", web::post().to(handlers::register))
                //
                // POST /api/v1/auth/login
                // Authenticates user and returns tokens
                // Request: LoginRequest { email, password }
                // Response: AuthResponseDto { accessToken, refreshToken, user }
                .route("/login", web::post().to(handlers::login))
                //
                // POST /api/v1/auth/refresh
                // Exchanges refresh token for new token pair
                // Request: RefreshTokenRequest { refreshToken }
                // Response: TokenResponseDto { accessToken, refreshToken }
                .route("/refresh", web::post().to(handlers::refresh_token))
                //
                // POST /api/v1/auth/verify-email
                // Verifies user's email address
                // Request: VerifyEmailRequest { token }
                // Response: MessageResponse { message }
                .route("/verify-email", web::post().to(handlers::verify_email))
                //
                // POST /api/v1/auth/forgot-password
                // Initiates password reset flow
                // Request: ForgotPasswordRequest { email }
                // Response: MessageResponse { message }
                .route("/forgot-password", web::post().to(handlers::forgot_password))
                //
                // POST /api/v1/auth/reset-password
                // Completes password reset
                // Request: ResetPasswordRequest { token, newPassword }
                // Response: MessageResponse { message }
                .route("/reset-password", web::post().to(handlers::reset_password))
                // ─────────────────────────────────────────────────────────
                // Protected Routes (require valid JWT)
                // ─────────────────────────────────────────────────────────
                //
                // GET /api/v1/auth/me
                // Returns authenticated user's profile
                // Headers: Authorization: Bearer <access_token>
                // Response: UserProfileDto
                .route("/me", web::get().to(handlers::get_profile))
                //
                // POST /api/v1/auth/logout
                // Ends current session
                // Headers: Authorization: Bearer <access_token>
                // Request: LogoutRequest { refreshToken }
                // Response: MessageResponse { message }
                .route("/logout", web::post().to(handlers::logout))
                //
                // POST /api/v1/auth/logout-all
                // Ends all sessions for the user
                // Headers: Authorization: Bearer <access_token>
                // Response: MessageResponse { message }
                .route("/logout-all", web::post().to(handlers::logout_all)),
        );
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    // Route configuration tests would typically be integration tests
    // that verify the routes are correctly registered and respond
    // to the expected HTTP methods.

    #[test]
    fn test_route_configuration_compiles() {
        // This test just ensures the configure function compiles
        // Actual route testing requires actix-web test utilities
        assert!(true);
    }
}
