//! # User Repository
//!
//! Data access layer for user-related database operations including:
//! - User CRUD operations
//! - Refresh token management
//! - User preferences
//!
//! ## Query Patterns
//!
//! All queries follow these patterns for consistency:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         Query Patterns                                  │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │ Pattern          │ Example                                              │
//! ├───────────────────┼─────────────────────────────────────────────────────┤
//! │ Soft Delete      │ WHERE deleted_at IS NULL                             │
//! │ Active Tokens    │ WHERE revoked_at IS NULL AND expires_at > NOW()      │
//! │ Returning        │ INSERT ... RETURNING * (to get generated fields)     │
//! │ Parameterized    │ Always use $1, $2 (never string interpolation)       │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Error Handling
//!
//! Database errors are converted to [`ApiError`] variants:
//!
//! | sqlx Error            | ApiError Variant      | HTTP Status |
//! |-----------------------|-----------------------|-------------|
//! | `RowNotFound`         | `NotFound`            | 404         |
//! | Unique constraint     | `Conflict`            | 409         |
//! | Other                 | `InternalError`       | 500         |
//!
//! ## Related Documentation
//!
//! - Entity definitions: [`crate::domain::entities`]
//! - Database schema: `db/migrations/postgresql/001_initial_schema.sql`
//! - Error types: [`shared::errors::ApiError`]

use chrono::{DateTime, Utc};
use shared::errors::ApiError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewRefreshToken, NewUser, RefreshToken, User, UserPreferences};

/// Repository for user-related database operations.
///
/// # Thread Safety
///
/// `UserRepository` is `Send + Sync` because `PgPool` internally uses `Arc`.
/// It can be safely shared across async tasks and Actix-web workers.
///
/// # Example
///
/// ```rust,ignore
/// let pool = database::create_pool(&config.database).await?;
/// let repo = UserRepository::new(pool);
///
/// // Find user by email
/// let user = repo.find_by_email("user@example.com").await?;
///
/// // Create new user
/// let new_user = NewUser { /* ... */ };
/// let created = repo.create(new_user).await?;
/// ```
#[derive(Debug, Clone)]
pub struct UserRepository {
    /// PostgreSQL connection pool
    pool: PgPool,
}

impl UserRepository {
    /// Creates a new repository instance with the given connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL connection pool (cloning is cheap - it's Arc-based)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pool = database::create_pool(&config.database).await?;
    /// let repo = UserRepository::new(pool);
    /// ```
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // USER OPERATIONS
    // =========================================================================

    /// Creates a new user in the database.
    ///
    /// # Arguments
    ///
    /// * `new_user` - User data to insert (password must be pre-hashed)
    ///
    /// # Returns
    ///
    /// The complete user record including generated fields (`user_id`, timestamps).
    ///
    /// # Errors
    ///
    /// - `ApiError::Conflict` if email already exists
    /// - `ApiError::InternalError` for other database errors
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let hashed = PasswordHasher::hash(&password)?;
    /// let new_user = NewUser {
    ///     email: "user@example.com".to_string(),
    ///     hashed_password: hashed,
    ///     first_name: "John".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     role: "student".to_string(),
    /// };
    /// let user = repo.create(new_user).await?;
    /// ```
    pub async fn create(&self, new_user: NewUser) -> Result<User, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email, hashed_password, first_name, last_name, role
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(&new_user.email)
        .bind(&new_user.hashed_password)
        .bind(&new_user.first_name)
        .bind(&new_user.last_name)
        .bind(&new_user.role)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violation (duplicate email)
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.constraint() == Some("users_email_key") {
                    return ApiError::Conflict { resource: "email".to_string() };
                }
            }
            ApiError::InternalError { message: format!("Database error: {}", e) }
        })?;

        Ok(user)
    }

    /// Finds a user by email address.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address to search for (case-sensitive)
    ///
    /// # Returns
    ///
    /// The user if found, `None` otherwise.
    ///
    /// # Note
    ///
    /// Only returns active users (`deleted_at IS NULL`).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(user) = repo.find_by_email("user@example.com").await? {
    ///     // User found, verify password
    ///     let valid = PasswordHasher::verify(&user.hashed_password, &password)?;
    /// }
    /// ```
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(user)
    }

    /// Finds a user by their unique ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user to find
    ///
    /// # Returns
    ///
    /// The user if found, `None` otherwise.
    ///
    /// # Note
    ///
    /// Only returns active users (`deleted_at IS NULL`).
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(user)
    }

    /// Updates the last login timestamp for a user.
    ///
    /// Called after successful authentication to track user activity.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user to update
    pub async fn update_last_login(&self, user_id: Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(())
    }

    /// Updates a user's password.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `new_password_hash` - Pre-hashed new password (Argon2id)
    ///
    /// # Security
    ///
    /// - Clears any pending password reset token
    /// - Should trigger a security notification email
    pub async fn update_password(
        &self,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> Result<(), ApiError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET hashed_password = $1,
                password_reset_token = NULL,
                password_reset_expires = NULL,
                updated_at = NOW()
            WHERE user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(new_password_hash)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound { resource: "user".to_string() });
        }

        Ok(())
    }

    /// Sets the email verification status for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `verified` - New verification status
    pub async fn set_email_verified(&self, user_id: Uuid, verified: bool) -> Result<(), ApiError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET email_verified = $1,
                email_verification_token = NULL,
                updated_at = NOW()
            WHERE user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(verified)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound { resource: "user".to_string() });
        }

        Ok(())
    }

    /// Sets the email verification token for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `token` - Verification token (random string)
    pub async fn set_email_verification_token(
        &self,
        user_id: Uuid,
        token: &str,
    ) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            UPDATE users
            SET email_verification_token = $1, updated_at = NOW()
            WHERE user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(())
    }

    /// Finds a user by their email verification token.
    ///
    /// Used to complete email verification flow.
    pub async fn find_by_verification_token(&self, token: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE email_verification_token = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(user)
    }

    /// Sets the password reset token and expiration for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `token` - Reset token (random string, should be hashed before storage)
    /// * `expires_at` - When the token expires
    pub async fn set_password_reset_token(
        &self,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            UPDATE users
            SET password_reset_token = $1,
                password_reset_expires = $2,
                updated_at = NOW()
            WHERE user_id = $3 AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .bind(expires_at)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(())
    }

    /// Finds a user by their password reset token.
    ///
    /// Only returns the user if:
    /// - Token matches
    /// - Token hasn't expired
    /// - Account isn't deleted
    pub async fn find_by_reset_token(&self, token: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE password_reset_token = $1
              AND password_reset_expires > NOW()
              AND deleted_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(user)
    }

    /// Soft deletes a user by setting `deleted_at`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user to delete
    ///
    /// # Note
    ///
    /// This is a soft delete - the record remains in the database
    /// but is excluded from queries. Use for GDPR compliance and
    /// potential account recovery.
    pub async fn soft_delete(&self, user_id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound { resource: "user".to_string() });
        }

        Ok(())
    }

    // =========================================================================
    // REFRESH TOKEN OPERATIONS
    // =========================================================================

    /// Creates a new refresh token in the database.
    ///
    /// # Arguments
    ///
    /// * `new_token` - Token data including hashed token value
    ///
    /// # Returns
    ///
    /// The complete token record including generated fields.
    ///
    /// # Security
    ///
    /// The `token_hash` field should contain a SHA-256 hash of the actual
    /// token. The plain token is returned to the client but never stored.
    pub async fn create_refresh_token(
        &self,
        new_token: NewRefreshToken,
    ) -> Result<RefreshToken, ApiError> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (
                user_id, token_hash, device_fingerprint, ip_address,
                user_agent, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&new_token.user_id)
        .bind(&new_token.token_hash)
        .bind(&new_token.device_fingerprint)
        .bind(&new_token.ip_address)
        .bind(&new_token.user_agent)
        .bind(&new_token.expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(token)
    }

    /// Finds a refresh token by its hash.
    ///
    /// # Arguments
    ///
    /// * `token_hash` - SHA-256 hash of the token to find
    ///
    /// # Returns
    ///
    /// The token if found and still valid (not expired, not revoked).
    ///
    /// # Note
    ///
    /// Only returns active tokens (`revoked_at IS NULL` and not expired).
    pub async fn find_refresh_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, ApiError> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT * FROM refresh_tokens
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > NOW()
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(token)
    }

    /// Revokes a specific refresh token.
    ///
    /// Used when user logs out or token is rotated.
    ///
    /// # Arguments
    ///
    /// * `token_id` - UUID of the token to revoke
    pub async fn revoke_refresh_token(&self, token_id: Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE token_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(token_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(())
    }

    /// Revokes all refresh tokens for a user.
    ///
    /// Used when user chooses "logout from all devices" or for
    /// security actions like password change.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user whose tokens to revoke
    ///
    /// # Returns
    ///
    /// Number of tokens that were revoked.
    pub async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> Result<u64, ApiError> {
        let result = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(result.rows_affected())
    }

    /// Deletes expired refresh tokens for cleanup.
    ///
    /// Should be called periodically (e.g., daily) to remove tokens
    /// that have expired. This helps keep the table size manageable.
    ///
    /// # Returns
    ///
    /// Number of tokens that were deleted.
    pub async fn delete_expired_tokens(&self) -> Result<u64, ApiError> {
        let result = sqlx::query(
            r#"
            DELETE FROM refresh_tokens
            WHERE expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(result.rows_affected())
    }

    // =========================================================================
    // USER PREFERENCES OPERATIONS
    // =========================================================================

    /// Gets user preferences, creating defaults if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    ///
    /// # Returns
    ///
    /// User preferences (creates with defaults if first access).
    pub async fn get_or_create_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<UserPreferences, ApiError> {
        // Try to find existing preferences
        let existing = sqlx::query_as::<_, UserPreferences>(
            r#"
            SELECT * FROM user_preferences
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        if let Some(prefs) = existing {
            return Ok(prefs);
        }

        // Create default preferences
        let prefs = sqlx::query_as::<_, UserPreferences>(
            r#"
            INSERT INTO user_preferences (user_id)
            VALUES ($1)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(prefs)
    }

    /// Updates user preferences.
    ///
    /// # Arguments
    ///
    /// * `user_id` - UUID of the user
    /// * `prefs` - New preference values
    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        email_notifications: bool,
        marketing_emails: bool,
        push_notifications: bool,
        course_reminders: bool,
        weekly_progress_email: bool,
    ) -> Result<UserPreferences, ApiError> {
        let prefs = sqlx::query_as::<_, UserPreferences>(
            r#"
            UPDATE user_preferences
            SET email_notifications = $1,
                marketing_emails = $2,
                push_notifications = $3,
                course_reminders = $4,
                weekly_progress_email = $5,
                updated_at = NOW()
            WHERE user_id = $6
            RETURNING *
            "#,
        )
        .bind(email_notifications)
        .bind(marketing_emails)
        .bind(push_notifications)
        .bind(course_reminders)
        .bind(weekly_progress_email)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(prefs)
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /// Checks if an email is already registered.
    ///
    /// More efficient than `find_by_email` when you only need to check existence.
    ///
    /// # Arguments
    ///
    /// * `email` - Email address to check
    ///
    /// # Returns
    ///
    /// `true` if email exists (for active user), `false` otherwise.
    pub async fn email_exists(&self, email: &str) -> Result<bool, ApiError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalError { message: format!("Database error: {}", e) })?;

        Ok(count.0 > 0)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a test database.
    // Use sqlx's test fixtures or testcontainers for integration tests.

    #[test]
    fn test_new_user_struct() {
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            hashed_password: "$argon2id$...".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            role: "student".to_string(),
        };

        assert_eq!(new_user.email, "test@example.com");
        assert_eq!(new_user.role, "student");
    }

    #[test]
    fn test_new_refresh_token_struct() {
        let new_token = NewRefreshToken {
            user_id: Uuid::new_v4(),
            token_hash: "sha256_hash_here".to_string(),
            device_fingerprint: Some("device123".to_string()),
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            expires_at: Utc::now(),
        };

        assert!(new_token.device_fingerprint.is_some());
        assert!(new_token.ip_address.is_some());
    }
}
