//! # User Profile Repository
//!
//! PostgreSQL-based repository for user profile operations.
//! Handles all database interactions for user profiles, preferences, and stats.
//!
//! ## Query Patterns
//!
//! This repository uses prepared statements and parameterized queries
//! to prevent SQL injection and improve performance.
//!
//! ## Error Handling
//!
//! All database errors are converted to domain errors using the [`ApiError`]
//! type from the shared crate. This ensures consistent error responses
//! across all services.

use chrono::Utc;
use shared::errors::ApiError;
use sqlx::{PgPool, Row};
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::domain::entities::{UserPreferences, UserProfile, UserRole, UserStats};

// =============================================================================
// USER PROFILE REPOSITORY
// =============================================================================

/// Repository for user profile database operations.
///
/// Provides CRUD operations for:
/// - User profiles
/// - User preferences
/// - User statistics
///
/// # Connection Pool
///
/// Uses a shared connection pool for efficient database access.
/// The pool is created at application startup and shared across handlers.
///
/// # Thread Safety
///
/// The repository is Clone and Send + Sync, making it safe to share
/// across Actix-web workers.
#[derive(Clone)]
pub struct UserProfileRepository {
    /// PostgreSQL connection pool
    pool: PgPool,
}

impl UserProfileRepository {
    /// Creates a new repository with the given connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL connection pool
    ///
    /// # Example
    ///
    /// ```rust
    /// let pool = create_pool(&database_url).await?;
    /// let repository = UserProfileRepository::new(pool);
    /// ```
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // PROFILE OPERATIONS
    // =========================================================================

    /// Finds a user profile by ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user to find
    ///
    /// # Returns
    ///
    /// - `Ok(Some(UserProfile))` if found
    /// - `Ok(None)` if not found
    /// - `Err(ApiError)` on database error
    ///
    /// # SQL Query
    ///
    /// ```sql
    /// SELECT id, email, first_name, last_name, role, avatar_url, bio,
    ///        website, social_links, created_at, updated_at
    /// FROM users
    /// WHERE id = $1
    /// ```
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, ApiError> {
        let result = sqlx::query_as::<_, UserProfile>(
            r#"
            SELECT id, email, first_name, last_name, role, avatar_url, bio,
                   website, social_links, created_at, updated_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to fetch user profile");
            ApiError::InternalError {
                message: "Failed to fetch user profile".to_string(),
            }
        })?;

        Ok(result)
    }

    /// Finds a user profile by email address.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address to search for
    ///
    /// # Returns
    ///
    /// - `Ok(Some(UserProfile))` if found
    /// - `Ok(None)` if not found
    /// - `Err(ApiError)` on database error
    #[instrument(skip(self), fields(email = %email))]
    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserProfile>, ApiError> {
        let result = sqlx::query_as::<_, UserProfile>(
            r#"
            SELECT id, email, first_name, last_name, role, avatar_url, bio,
                   website, social_links, created_at, updated_at
            FROM users
            WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to fetch user by email");
            ApiError::InternalError {
                message: "Failed to fetch user profile".to_string(),
            }
        })?;

        Ok(result)
    }

    /// Updates a user's profile information.
    ///
    /// Only updates provided fields (partial update).
    /// Uses COALESCE to keep existing values when new values are NULL.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user to update
    /// * `update` - The profile update data
    ///
    /// # Returns
    ///
    /// - `Ok(UserProfile)` - The updated profile
    /// - `Err(ApiError::NotFound)` if user doesn't exist
    /// - `Err(ApiError::InternalError)` on database error
    #[instrument(skip(self, update), fields(user_id = %user_id))]
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        update: ProfileUpdate,
    ) -> Result<UserProfile, ApiError> {
        let result = sqlx::query_as::<_, UserProfile>(
            r#"
            UPDATE users
            SET first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                bio = COALESCE($4, bio),
                website = COALESCE($5, website),
                social_links = COALESCE($6, social_links),
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, email, first_name, last_name, role, avatar_url, bio,
                      website, social_links, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&update.first_name)
        .bind(&update.last_name)
        .bind(&update.bio)
        .bind(&update.website)
        .bind(&update.social_links)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update user profile");
            ApiError::InternalError {
                message: "Failed to update user profile".to_string(),
            }
        })?;

        result.ok_or_else(|| {
            info!("User not found for update: {}", user_id);
            ApiError::NotFound {
                resource: "User".to_string(),
            }
        })
    }

    /// Updates a user's avatar URL.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user
    /// * `avatar_url` - The new avatar URL (None to remove)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn update_avatar(
        &self,
        user_id: Uuid,
        avatar_url: Option<String>,
    ) -> Result<UserProfile, ApiError> {
        let result = sqlx::query_as::<_, UserProfile>(
            r#"
            UPDATE users
            SET avatar_url = $2,
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, email, first_name, last_name, role, avatar_url, bio,
                      website, social_links, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&avatar_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update avatar");
            ApiError::InternalError {
                message: "Failed to update avatar".to_string(),
            }
        })?;

        result.ok_or_else(|| ApiError::NotFound {
            resource: "User".to_string(),
        })
    }

    /// Changes a user's role.
    ///
    /// # Authorization
    ///
    /// Only admins should be able to call this. Authorization is handled
    /// at the service/handler layer.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user
    /// * `new_role` - The new role to assign
    #[instrument(skip(self), fields(user_id = %user_id, new_role = %new_role))]
    pub async fn update_role(
        &self,
        user_id: Uuid,
        new_role: UserRole,
    ) -> Result<UserProfile, ApiError> {
        let result = sqlx::query_as::<_, UserProfile>(
            r#"
            UPDATE users
            SET role = $2,
                updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, email, first_name, last_name, role, avatar_url, bio,
                      website, social_links, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(new_role)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update user role");
            ApiError::InternalError {
                message: "Failed to update user role".to_string(),
            }
        })?;

        result.ok_or_else(|| ApiError::NotFound {
            resource: "User".to_string(),
        })
    }

    // =========================================================================
    // PREFERENCES OPERATIONS
    // =========================================================================

    /// Gets user preferences, creating defaults if they don't exist.
    ///
    /// Uses INSERT ... ON CONFLICT to atomically get or create preferences.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_preferences(&self, user_id: Uuid) -> Result<UserPreferences, ApiError> {
        // Try to get existing preferences
        let existing = sqlx::query_as::<_, UserPreferences>(
            r#"
            SELECT user_id, language, timezone, email_notifications,
                   privacy, accessibility, updated_at
            FROM user_preferences
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to fetch preferences");
            ApiError::InternalError {
                message: "Failed to fetch preferences".to_string(),
            }
        })?;

        if let Some(prefs) = existing {
            return Ok(prefs);
        }

        // Create default preferences if they don't exist
        let defaults = UserPreferences::new_for_user(user_id);

        let result = sqlx::query_as::<_, UserPreferences>(
            r#"
            INSERT INTO user_preferences (user_id, language, timezone, 
                                          email_notifications, privacy, 
                                          accessibility, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id) DO UPDATE SET user_id = user_preferences.user_id
            RETURNING user_id, language, timezone, email_notifications,
                      privacy, accessibility, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&defaults.language)
        .bind(&defaults.timezone)
        .bind(&defaults.email_notifications)
        .bind(&defaults.privacy)
        .bind(&defaults.accessibility)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to create default preferences");
            ApiError::InternalError {
                message: "Failed to create preferences".to_string(),
            }
        })?;

        Ok(result)
    }

    /// Updates user preferences.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user
    /// * `update` - The preferences update data
    #[instrument(skip(self, update), fields(user_id = %user_id))]
    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        update: PreferencesUpdate,
    ) -> Result<UserPreferences, ApiError> {
        let result = sqlx::query_as::<_, UserPreferences>(
            r#"
            UPDATE user_preferences
            SET language = COALESCE($2, language),
                timezone = COALESCE($3, timezone),
                email_notifications = COALESCE($4, email_notifications),
                privacy = COALESCE($5, privacy),
                accessibility = COALESCE($6, accessibility),
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING user_id, language, timezone, email_notifications,
                      privacy, accessibility, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&update.language)
        .bind(&update.timezone)
        .bind(&update.email_notifications)
        .bind(&update.privacy)
        .bind(&update.accessibility)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update preferences");
            ApiError::InternalError {
                message: "Failed to update preferences".to_string(),
            }
        })?;

        result.ok_or_else(|| {
            // If no row was updated, preferences might not exist yet
            // Create them first and try again
            ApiError::NotFound {
                resource: "Preferences".to_string(),
            }
        })
    }

    // =========================================================================
    // STATS OPERATIONS
    // =========================================================================

    /// Gets user statistics, creating defaults if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_stats(&self, user_id: Uuid) -> Result<UserStats, ApiError> {
        let result = sqlx::query_as::<_, UserStats>(
            r#"
            SELECT user_id, courses_enrolled, courses_completed, certificates_earned,
                   total_learning_time_minutes, average_completion_rate,
                   current_streak_days, longest_streak_days, last_activity_at,
                   calculated_at
            FROM user_stats
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to fetch user stats");
            ApiError::InternalError {
                message: "Failed to fetch user stats".to_string(),
            }
        })?;

        // Return existing stats or default empty stats
        Ok(result.unwrap_or_else(|| UserStats::new_for_user(user_id)))
    }

    // =========================================================================
    // SEARCH OPERATIONS
    // =========================================================================

    /// Searches for users by name or email.
    ///
    /// Supports pagination and filtering by role.
    ///
    /// # Authorization
    ///
    /// Only instructors and admins should be able to search users.
    /// Authorization is handled at the service/handler layer.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query (matches name or email)
    /// * `role_filter` - Optional role filter
    /// * `limit` - Maximum results to return
    /// * `offset` - Pagination offset
    #[instrument(skip(self), fields(query = %query, limit = %limit, offset = %offset))]
    pub async fn search_users(
        &self,
        query: &str,
        role_filter: Option<UserRole>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfile>, ApiError> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        let results = if let Some(role) = role_filter {
            sqlx::query_as::<_, UserProfile>(
                r#"
                SELECT id, email, first_name, last_name, role, avatar_url, bio,
                       website, social_links, created_at, updated_at
                FROM users
                WHERE deleted_at IS NULL
                  AND role = $1
                  AND (LOWER(first_name) LIKE $2 
                       OR LOWER(last_name) LIKE $2 
                       OR LOWER(email) LIKE $2)
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(role)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, UserProfile>(
                r#"
                SELECT id, email, first_name, last_name, role, avatar_url, bio,
                       website, social_links, created_at, updated_at
                FROM users
                WHERE deleted_at IS NULL
                  AND (LOWER(first_name) LIKE $1 
                       OR LOWER(last_name) LIKE $1 
                       OR LOWER(email) LIKE $1)
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        };

        results.map_err(|e| {
            error!(error = %e, "Failed to search users");
            ApiError::InternalError {
                message: "Failed to search users".to_string(),
            }
        })
    }

    /// Counts total users matching a search query.
    ///
    /// Used for pagination metadata.
    #[instrument(skip(self), fields(query = %query))]
    pub async fn count_search_results(
        &self,
        query: &str,
        role_filter: Option<UserRole>,
    ) -> Result<i64, ApiError> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        let count: i64 = if let Some(role) = role_filter {
            sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM users
                WHERE deleted_at IS NULL
                  AND role = $1
                  AND (LOWER(first_name) LIKE $2 
                       OR LOWER(last_name) LIKE $2 
                       OR LOWER(email) LIKE $2)
                "#,
            )
            .bind(role)
            .bind(&search_pattern)
            .fetch_one(&self.pool)
            .await
        } else {
            sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM users
                WHERE deleted_at IS NULL
                  AND (LOWER(first_name) LIKE $1 
                       OR LOWER(last_name) LIKE $1 
                       OR LOWER(email) LIKE $1)
                "#,
            )
            .bind(&search_pattern)
            .fetch_one(&self.pool)
            .await
        }
        .map_err(|e| {
            error!(error = %e, "Failed to count search results");
            ApiError::InternalError {
                message: "Failed to count users".to_string(),
            }
        })?;

        Ok(count)
    }
}

// =============================================================================
// UPDATE STRUCTS
// =============================================================================

/// Profile update data.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Default)]
pub struct ProfileUpdate {
    /// New first name
    pub first_name: Option<String>,
    /// New last name
    pub last_name: Option<String>,
    /// New biography
    pub bio: Option<String>,
    /// New website URL
    pub website: Option<String>,
    /// New social links
    pub social_links: Option<serde_json::Value>,
}

/// Preferences update data.
///
/// All fields are optional - only provided fields will be updated.
#[derive(Debug, Default)]
pub struct PreferencesUpdate {
    /// New language code
    pub language: Option<String>,
    /// New timezone
    pub timezone: Option<String>,
    /// New email notification settings
    pub email_notifications: Option<serde_json::Value>,
    /// New privacy settings
    pub privacy: Option<serde_json::Value>,
    /// New accessibility settings
    pub accessibility: Option<serde_json::Value>,
}
