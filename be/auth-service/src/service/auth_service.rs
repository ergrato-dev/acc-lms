//! # Authentication Service
//!
//! Core business logic for all authentication operations including:
//! - User registration
//! - Login/logout
//! - Token management
//! - Password reset
//! - Email verification
//!
//! ## Security Model
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         Token Architecture                              │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │  ┌─────────────────┐                      ┌─────────────────────────┐   │
//! │  │  Access Token   │                      │    Refresh Token        │   │
//! │  │  (JWT, 15 min)  │                      │    (Opaque, 7 days)     │   │
//! │  └────────┬────────┘                      └───────────┬─────────────┘   │
//! │           │                                           │                 │
//! │           │ Stored in:                                │ Stored in:      │
//! │           │ - Memory (frontend)                       │ - HttpOnly      │
//! │           │ - Authorization header                    │   cookie        │
//! │           │                                           │ - Database      │
//! │           │                                           │   (hashed)      │
//! │           │                                           │                 │
//! │           ▼                                           ▼                 │
//! │  ┌─────────────────┐                      ┌─────────────────────────┐   │
//! │  │ Stateless       │                      │ Stateful (revocable)    │   │
//! │  │ verification    │                      │ via database lookup     │   │
//! │  └─────────────────┘                      └─────────────────────────┘   │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Error Handling
//!
//! All methods return `Result<T, ApiError>` for consistent error responses.
//! Sensitive operations use generic error messages to prevent enumeration.
//!
//! | Scenario              | Error Type             | HTTP Status |
//! |-----------------------|------------------------|-------------|
//! | Invalid credentials   | `InvalidCredentials`   | 401         |
//! | Email exists          | `Conflict`             | 409         |
//! | Token expired         | `TokenExpired`         | 401         |
//! | User not found        | `NotFound`             | 404         |
//!
//! ## Related Documentation
//!
//! - JWT configuration: [`shared::auth::jwt::JwtService`]
//! - Password security: [`shared::auth::password::PasswordHasher`]
//! - Redis caching: [`shared::redis_client::RedisClient`]
//! - API endpoints: [`crate::api::handlers`]

use chrono::{Duration as ChronoDuration, Utc};
use sha2::{Digest, Sha256};
use shared::{
    auth::{jwt::JwtService, password::PasswordHasher, TokenPair},
    config::JwtConfig,
    errors::ApiError,
    redis_client::RedisClient,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    domain::{NewRefreshToken, NewUser, User, UserProfile},
    repository::UserRepository,
};

// =============================================================================
// SERVICE STRUCT
// =============================================================================

/// Authentication service with business logic for user operations.
///
/// # Thread Safety
///
/// `AuthService` is `Send + Sync` and can be safely shared across
/// Actix-web workers. Internal components use appropriate concurrency
/// primitives:
/// - `UserRepository`: Uses `PgPool` (Arc-based)
/// - `JwtService`: Wrapped in `Arc`
/// - `RedisClient`: Uses `ConnectionManager` (auto-reconnect)
/// - `PasswordHasher`: Wrapped in `Arc` for shared access
#[derive(Clone)]
pub struct AuthService {
    /// Repository for database operations
    repository: UserRepository,
    /// JWT service for token generation/validation
    jwt_service: Arc<JwtService>,
    /// Password hasher for secure password operations
    password_hasher: Arc<PasswordHasher>,
    /// Redis client for session caching and blacklisting
    redis_client: RedisClient,
    /// JWT configuration (token lifetimes)
    jwt_config: JwtConfig,
}

/// Response returned after successful authentication.
///
/// Contains the token pair and user profile for immediate use.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    /// Access and refresh tokens
    #[serde(flatten)]
    pub tokens: TokenPair,
    /// User profile (safe for client)
    pub user: UserProfile,
}

impl AuthService {
    /// Creates a new authentication service instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - Database repository for user operations
    /// * `jwt_service` - Service for JWT token operations
    /// * `password_hasher` - Service for password hashing
    /// * `redis_client` - Client for Redis caching
    /// * `jwt_config` - Configuration for token lifetimes
    pub fn new(
        repository: UserRepository,
        jwt_service: Arc<JwtService>,
        password_hasher: Arc<PasswordHasher>,
        redis_client: RedisClient,
        jwt_config: JwtConfig,
    ) -> Self {
        Self {
            repository,
            jwt_service,
            password_hasher,
            redis_client,
            jwt_config,
        }
    }

    // =========================================================================
    // REGISTRATION
    // =========================================================================

    /// Registers a new user account.
    ///
    /// # Process
    ///
    /// 1. Validate email doesn't exist
    /// 2. Hash password with Argon2id
    /// 3. Create user in database
    /// 4. Generate token pair
    /// 5. Store refresh token (hashed)
    /// 6. Return tokens and profile
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<AuthResponse, ApiError> {
        // Check if email already exists
        if self.repository.email_exists(email).await? {
            return Err(ApiError::Conflict {
                resource: "Email already registered".to_string(),
            });
        }

        // Hash password using Argon2id
        let hashed_password = self.password_hasher.hash(password)?;

        // Create new user
        let new_user = NewUser {
            email: email.to_string(),
            hashed_password,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            role: "student".to_string(), // Default role
        };

        let user = self.repository.create(new_user).await?;

        info!(
            user_id = %user.user_id,
            email = %email,
            "User registered successfully"
        );

        // Generate tokens and store refresh token
        let tokens = self.generate_and_store_tokens(&user, None, None, None).await?;

        Ok(AuthResponse {
            tokens,
            user: user.into(),
        })
    }

    // =========================================================================
    // LOGIN
    // =========================================================================

    /// Authenticates a user with email and password.
    ///
    /// # Security
    ///
    /// Uses constant-time comparison for password verification.
    /// Generic error message prevents user enumeration.
    pub async fn login(
        &self,
        email: &str,
        password: &str,
        device_fingerprint: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<AuthResponse, ApiError> {
        // Find user by email
        let user = self
            .repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| {
                // Use same error for both "not found" and "wrong password"
                // to prevent user enumeration attacks
                warn!(email = %email, "Login attempt for non-existent user");
                ApiError::InvalidCredentials
            })?;

        // Verify password
        let is_valid = self.password_hasher.verify(password, &user.hashed_password)?;

        if !is_valid {
            warn!(
                user_id = %user.user_id,
                email = %email,
                "Failed login attempt - invalid password"
            );
            return Err(ApiError::InvalidCredentials);
        }

        // Update last login
        self.repository.update_last_login(user.user_id).await?;

        info!(
            user_id = %user.user_id,
            email = %email,
            "User logged in successfully"
        );

        // Generate tokens
        let tokens = self
            .generate_and_store_tokens(&user, device_fingerprint, ip_address, user_agent)
            .await?;

        // Cache session in Redis for quick validation
        let session_key = format!("session:{}", user.user_id);
        if let Err(e) = self
            .redis_client
            .set(&session_key, &user.role, Some(Duration::from_secs(3600)))
            .await
        {
            warn!(error = %e, "Failed to cache session in Redis");
            // Non-fatal: continue even if Redis fails
        }

        Ok(AuthResponse {
            tokens,
            user: user.into(),
        })
    }

    // =========================================================================
    // TOKEN REFRESH
    // =========================================================================

    /// Refreshes tokens using a valid refresh token.
    ///
    /// Implements **token rotation**: the old refresh token is revoked and
    /// a new pair is issued.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, ApiError> {
        // Hash the provided token
        let token_hash = Self::hash_token(refresh_token);

        // Find the token in database
        let stored_token = self
            .repository
            .find_refresh_token(&token_hash)
            .await?
            .ok_or(ApiError::InvalidToken)?;

        // Load user
        let user = self
            .repository
            .find_by_id(stored_token.user_id)
            .await?
            .ok_or(ApiError::InvalidToken)?;

        // Revoke the old token (token rotation)
        self.repository
            .revoke_refresh_token(stored_token.token_id)
            .await?;

        info!(
            user_id = %user.user_id,
            "Token refreshed successfully"
        );

        // Generate new tokens
        let tokens = self
            .generate_and_store_tokens(
                &user,
                stored_token.device_fingerprint,
                stored_token.ip_address,
                stored_token.user_agent,
            )
            .await?;

        Ok(tokens)
    }

    // =========================================================================
    // LOGOUT
    // =========================================================================

    /// Logs out the current session.
    pub async fn logout(
        &self,
        user_id: Uuid,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<(), ApiError> {
        // Revoke the refresh token
        let token_hash = Self::hash_token(refresh_token);
        if let Some(stored_token) = self.repository.find_refresh_token(&token_hash).await? {
            self.repository
                .revoke_refresh_token(stored_token.token_id)
                .await?;
        }

        // Blacklist the access token in Redis
        let ttl = Duration::from_secs(self.jwt_config.access_token_ttl_seconds as u64);
        if let Err(e) = self
            .redis_client
            .blacklist_token(access_token, ttl)
            .await
        {
            warn!(error = %e, "Failed to blacklist token in Redis");
        }

        // Remove session from Redis
        let session_key = format!("session:{}", user_id);
        if let Err(e) = self.redis_client.delete(&session_key).await {
            warn!(error = %e, "Failed to delete session from Redis");
        }

        info!(user_id = %user_id, "User logged out");

        Ok(())
    }

    /// Logs out from all sessions.
    pub async fn logout_all(&self, user_id: Uuid, access_token: &str) -> Result<u64, ApiError> {
        // Revoke all refresh tokens
        let revoked_count = self.repository.revoke_all_refresh_tokens(user_id).await?;

        // Blacklist current access token
        let ttl = Duration::from_secs(self.jwt_config.access_token_ttl_seconds as u64);
        if let Err(e) = self
            .redis_client
            .blacklist_token(access_token, ttl)
            .await
        {
            warn!(error = %e, "Failed to blacklist token in Redis");
        }

        // Remove session from Redis
        let session_key = format!("session:{}", user_id);
        if let Err(e) = self.redis_client.delete(&session_key).await {
            warn!(error = %e, "Failed to delete session from Redis");
        }

        info!(
            user_id = %user_id,
            sessions_terminated = revoked_count,
            "User logged out from all sessions"
        );

        Ok(revoked_count)
    }

    // =========================================================================
    // USER PROFILE
    // =========================================================================

    /// Gets the authenticated user's profile.
    pub async fn get_profile(&self, user_id: Uuid) -> Result<UserProfile, ApiError> {
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: format!("user:{}", user_id),
            })?;

        Ok(user.into())
    }

    // =========================================================================
    // EMAIL VERIFICATION
    // =========================================================================

    /// Initiates email verification by generating and storing a token.
    pub async fn initiate_email_verification(&self, user_id: Uuid) -> Result<String, ApiError> {
        let token = Self::generate_random_token();

        self.repository
            .set_email_verification_token(user_id, &token)
            .await?;

        Ok(token)
    }

    /// Verifies email using the verification token.
    pub async fn verify_email(&self, token: &str) -> Result<Uuid, ApiError> {
        let user = self
            .repository
            .find_by_verification_token(token)
            .await?
            .ok_or_else(|| ApiError::BadRequest {
                message: "Invalid verification token".to_string(),
            })?;

        self.repository.set_email_verified(user.user_id, true).await?;

        info!(user_id = %user.user_id, "Email verified successfully");

        Ok(user.user_id)
    }

    // =========================================================================
    // PASSWORD RESET
    // =========================================================================

    /// Initiates password reset flow.
    pub async fn initiate_password_reset(&self, email: &str) -> Result<Option<String>, ApiError> {
        let user = match self.repository.find_by_email(email).await? {
            Some(u) => u,
            None => {
                // Don't reveal that email doesn't exist
                info!(email = %email, "Password reset requested for unknown email");
                return Ok(None);
            }
        };

        let token = Self::generate_random_token();
        let expires_at = Utc::now() + ChronoDuration::hours(1);

        self.repository
            .set_password_reset_token(user.user_id, &token, expires_at)
            .await?;

        info!(user_id = %user.user_id, "Password reset token generated");

        Ok(Some(token))
    }

    /// Completes password reset using the reset token.
    pub async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), ApiError> {
        let user = self
            .repository
            .find_by_reset_token(token)
            .await?
            .ok_or_else(|| ApiError::BadRequest {
                message: "Invalid or expired reset token".to_string(),
            })?;

        // Hash new password
        let hashed_password = self.password_hasher.hash(new_password)?;

        // Update password (also clears reset token)
        self.repository
            .update_password(user.user_id, &hashed_password)
            .await?;

        // Revoke all sessions for security
        self.repository
            .revoke_all_refresh_tokens(user.user_id)
            .await?;

        info!(user_id = %user.user_id, "Password reset completed");

        Ok(())
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /// Generates tokens and stores the refresh token in database.
    async fn generate_and_store_tokens(
        &self,
        user: &User,
        device_fingerprint: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<TokenPair, ApiError> {
        // Generate token pair (requires email for claims)
        let tokens = self
            .jwt_service
            .generate_tokens(user.user_id, &user.email, &user.role)?;

        // Hash refresh token for storage
        let token_hash = Self::hash_token(&tokens.refresh_token);

        // Calculate expiration
        let expires_at = Utc::now()
            + ChronoDuration::seconds(self.jwt_config.refresh_token_ttl_seconds as i64);

        // Store in database
        let new_token = NewRefreshToken {
            user_id: user.user_id,
            token_hash,
            device_fingerprint,
            ip_address,
            user_agent,
            expires_at,
        };

        self.repository.create_refresh_token(new_token).await?;

        Ok(tokens)
    }

    /// Hashes a token using SHA-256.
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Generates a cryptographically secure random token.
    fn generate_random_token() -> String {
        use rand::Rng;
        let bytes: [u8; 32] = rand::thread_rng().gen();
        hex::encode(bytes)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_token_deterministic() {
        let token = "test_token_123";
        let hash1 = AuthService::hash_token(token);
        let hash2 = AuthService::hash_token(token);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_token_different_inputs() {
        let hash1 = AuthService::hash_token("token1");
        let hash2 = AuthService::hash_token("token2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_generate_random_token_unique() {
        let token1 = AuthService::generate_random_token();
        let token2 = AuthService::generate_random_token();
        assert_ne!(token1, token2);
        assert_eq!(token1.len(), 64); // 32 bytes = 64 hex chars
    }
}
