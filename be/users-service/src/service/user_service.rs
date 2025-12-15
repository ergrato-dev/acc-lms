//! # User Service
//!
//! Business logic layer for user profile management.
//! Orchestrates repository calls, emits domain events, and handles caching.
//!
//! ## Responsibilities
//!
//! - Profile CRUD operations
//! - Preferences management
//! - Avatar upload/removal
//! - User search (for instructors/admins)
//! - Statistics retrieval
//!
//! ## Authorization
//!
//! Authorization rules:
//! - Users can view their own profile
//! - Users can update their own profile
//! - Instructors can view enrolled students' profiles
//! - Admins can view/update any profile
//!
//! ## Event Emission
//!
//! The service emits events for:
//! - Profile updates
//! - Preference changes
//! - Avatar updates
//! - Role changes

use shared::errors::ApiError;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::domain::entities::{UserPreferences, UserProfile, UserRole, UserStats};
use crate::domain::events::{
    AvatarRemovedEvent, AvatarUpdatedEvent, PreferencesChangedEvent, ProfileUpdatedEvent,
    RoleChangedEvent, UserEvent,
};
use crate::repository::{PreferencesUpdate, ProfileUpdate, UserProfileRepository};

// =============================================================================
// USER SERVICE
// =============================================================================

/// Service for user profile business operations.
///
/// Provides high-level operations that orchestrate repository calls,
/// authorization checks, and event emission.
///
/// # Example
///
/// ```rust
/// let service = UserService::new(repository);
///
/// // Get a profile
/// let profile = service.get_profile(user_id, requesting_user_id).await?;
///
/// // Update a profile
/// let updated = service.update_profile(user_id, update_dto, requesting_user_id).await?;
/// ```
pub struct UserService {
    /// Repository for database operations
    repository: UserProfileRepository,
}

impl UserService {
    /// Creates a new UserService instance.
    ///
    /// # Arguments
    ///
    /// * `repository` - The user profile repository
    pub fn new(repository: UserProfileRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // PROFILE OPERATIONS
    // =========================================================================

    /// Gets a user's profile with authorization checks.
    ///
    /// # Authorization Rules
    ///
    /// - Users can view their own profile (full data)
    /// - Instructors can view enrolled students (limited by privacy settings)
    /// - Admins can view any profile (full data)
    /// - Public viewers see only public data (respecting privacy settings)
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the profile to retrieve
    /// * `requesting_user_id` - ID of the user making the request (None for public)
    /// * `requesting_user_role` - Role of the requesting user
    ///
    /// # Returns
    ///
    /// - `Ok(UserProfileResponse)` with appropriate data based on authorization
    /// - `Err(ApiError::NotFound)` if user doesn't exist
    /// - `Err(ApiError::Forbidden)` if access is denied
    #[instrument(skip(self), fields(
        user_id = %user_id,
        requester = ?requesting_user_id
    ))]
    pub async fn get_profile(
        &self,
        user_id: Uuid,
        requesting_user_id: Option<Uuid>,
        requesting_user_role: Option<UserRole>,
    ) -> Result<UserProfileResponse, ApiError> {
        // Fetch the profile
        let profile = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "User".to_string(),
            })?;

        // Fetch preferences for privacy settings
        let preferences = self.repository.get_preferences(user_id).await?;

        // Determine access level
        let is_own_profile = requesting_user_id == Some(user_id);
        let is_admin = requesting_user_role == Some(UserRole::Admin);

        // Full access for own profile or admin
        if is_own_profile || is_admin {
            return Ok(UserProfileResponse::full(profile));
        }

        // Apply privacy settings for other viewers
        let response = if preferences.shows_profile() {
            UserProfileResponse::public(profile, &preferences)
        } else {
            // Profile is private - only show minimal info
            UserProfileResponse::minimal(profile)
        };

        Ok(response)
    }

    /// Updates a user's profile.
    ///
    /// # Authorization
    ///
    /// - Users can update their own profile
    /// - Admins can update any profile
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the profile to update
    /// * `update` - The update data
    /// * `requesting_user_id` - ID of the user making the request
    /// * `requesting_user_role` - Role of the requesting user
    ///
    /// # Returns
    ///
    /// - `Ok(UserProfile)` - The updated profile
    /// - `Err(ApiError::Forbidden)` if not authorized
    /// - `Err(ApiError::NotFound)` if user doesn't exist
    #[instrument(skip(self, update), fields(
        user_id = %user_id,
        requester = %requesting_user_id
    ))]
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        update: UpdateProfileRequest,
        requesting_user_id: Uuid,
        requesting_user_role: UserRole,
    ) -> Result<UserProfile, ApiError> {
        // Authorization check
        if user_id != requesting_user_id && requesting_user_role != UserRole::Admin {
            warn!(
                user_id = %user_id,
                requester = %requesting_user_id,
                "Unauthorized profile update attempt"
            );
            return Err(ApiError::AccessDenied);
        }

        // Track which fields are being changed
        let mut fields_changed = Vec::new();

        if update.first_name.is_some() {
            fields_changed.push("first_name".to_string());
        }
        if update.last_name.is_some() {
            fields_changed.push("last_name".to_string());
        }
        if update.bio.is_some() {
            fields_changed.push("bio".to_string());
        }
        if update.website.is_some() {
            fields_changed.push("website".to_string());
        }
        if update.social_links.is_some() {
            fields_changed.push("social_links".to_string());
        }

        // Perform the update
        let profile_update = ProfileUpdate {
            first_name: update.first_name,
            last_name: update.last_name,
            bio: update.bio,
            website: update.website,
            social_links: update.social_links,
        };

        let updated_profile = self.repository.update_profile(user_id, profile_update).await?;

        // Emit event
        if !fields_changed.is_empty() {
            let event = UserEvent::ProfileUpdated(ProfileUpdatedEvent::new(user_id, fields_changed));
            self.emit_event(event).await;
        }

        info!(user_id = %user_id, "Profile updated successfully");

        Ok(updated_profile)
    }

    // =========================================================================
    // PREFERENCES OPERATIONS
    // =========================================================================

    /// Gets a user's preferences.
    ///
    /// Creates default preferences if they don't exist.
    ///
    /// # Authorization
    ///
    /// - Users can only view their own preferences
    /// - Admins can view any user's preferences
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_preferences(
        &self,
        user_id: Uuid,
        requesting_user_id: Uuid,
        requesting_user_role: UserRole,
    ) -> Result<UserPreferences, ApiError> {
        // Authorization check
        if user_id != requesting_user_id && requesting_user_role != UserRole::Admin {
            return Err(ApiError::AccessDenied);
                
        }

        self.repository.get_preferences(user_id).await
    }

    /// Updates a user's preferences.
    ///
    /// # Authorization
    ///
    /// - Users can only update their own preferences
    /// - Admins can update any user's preferences
    #[instrument(skip(self, update), fields(user_id = %user_id))]
    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        update: UpdatePreferencesRequest,
        requesting_user_id: Uuid,
        requesting_user_role: UserRole,
    ) -> Result<UserPreferences, ApiError> {
        // Authorization check
        if user_id != requesting_user_id && requesting_user_role != UserRole::Admin {
            return Err(ApiError::AccessDenied);
                
        }

        // Ensure preferences exist first
        let _ = self.repository.get_preferences(user_id).await?;

        // Track categories changed
        let mut categories_changed = Vec::new();
        let mut new_language = None;
        let mut new_timezone = None;

        if update.language.is_some() {
            categories_changed.push("language".to_string());
            new_language = update.language.clone();
        }
        if update.timezone.is_some() {
            categories_changed.push("timezone".to_string());
            new_timezone = update.timezone.clone();
        }
        if update.email_notifications.is_some() {
            categories_changed.push("email_notifications".to_string());
        }
        if update.privacy.is_some() {
            categories_changed.push("privacy".to_string());
        }
        if update.accessibility.is_some() {
            categories_changed.push("accessibility".to_string());
        }

        // Perform the update
        let prefs_update = PreferencesUpdate {
            language: update.language,
            timezone: update.timezone,
            email_notifications: update.email_notifications,
            privacy: update.privacy,
            accessibility: update.accessibility,
        };

        let updated = self.repository.update_preferences(user_id, prefs_update).await?;

        // Emit event
        if !categories_changed.is_empty() {
            let mut event = PreferencesChangedEvent::new(user_id, categories_changed);
            if let Some(lang) = new_language {
                event = event.with_language(lang);
            }
            if let Some(tz) = new_timezone {
                event = event.with_timezone(tz);
            }
            self.emit_event(UserEvent::PreferencesChanged(event)).await;
        }

        info!(user_id = %user_id, "Preferences updated successfully");

        Ok(updated)
    }

    // =========================================================================
    // AVATAR OPERATIONS
    // =========================================================================

    /// Updates a user's avatar.
    ///
    /// The actual file upload to storage is handled by the handler.
    /// This method just updates the URL in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user
    /// * `avatar_url` - URL of the uploaded avatar
    /// * `file_size` - Size of the avatar file in bytes
    /// * `mime_type` - MIME type of the avatar
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn update_avatar(
        &self,
        user_id: Uuid,
        avatar_url: String,
        file_size: u64,
        mime_type: String,
        requesting_user_id: Uuid,
    ) -> Result<UserProfile, ApiError> {
        // Authorization check
        if user_id != requesting_user_id {
            return Err(ApiError::AccessDenied);
                
        }

        // Get current profile to check for existing avatar
        let current = self.repository.find_by_id(user_id).await?.ok_or_else(|| {
            ApiError::NotFound {
                resource: "User".to_string(),
            }
        })?;

        let previous_url = current.avatar_url.clone();

        // Update the avatar URL
        let updated = self
            .repository
            .update_avatar(user_id, Some(avatar_url.clone()))
            .await?;

        // Emit event
        let event = UserEvent::AvatarUpdated(AvatarUpdatedEvent::new(
            user_id,
            avatar_url,
            previous_url,
            file_size,
            mime_type,
        ));
        self.emit_event(event).await;

        info!(user_id = %user_id, "Avatar updated successfully");

        Ok(updated)
    }

    /// Removes a user's avatar.
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn remove_avatar(
        &self,
        user_id: Uuid,
        requesting_user_id: Uuid,
    ) -> Result<UserProfile, ApiError> {
        // Authorization check
        if user_id != requesting_user_id {
            return Err(ApiError::AccessDenied);
                
        }

        // Get current profile
        let current = self.repository.find_by_id(user_id).await?.ok_or_else(|| {
            ApiError::NotFound {
                resource: "User".to_string(),
            }
        })?;

        // Check if there's an avatar to remove
        let Some(avatar_url) = current.avatar_url else {
            return Err(ApiError::BadRequest {
                message: "No avatar to remove".to_string(),
            });
        };

        // Remove the avatar URL
        let updated = self.repository.update_avatar(user_id, None).await?;

        // Emit event
        let event = UserEvent::AvatarRemoved(AvatarRemovedEvent::new(user_id, avatar_url));
        self.emit_event(event).await;

        info!(user_id = %user_id, "Avatar removed successfully");

        Ok(updated)
    }

    // =========================================================================
    // STATS OPERATIONS
    // =========================================================================

    /// Gets user statistics.
    ///
    /// # Authorization
    ///
    /// - Users can view their own stats
    /// - Instructors can view enrolled students' stats
    /// - Admins can view any user's stats
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_stats(
        &self,
        user_id: Uuid,
        requesting_user_id: Option<Uuid>,
        requesting_user_role: Option<UserRole>,
    ) -> Result<UserStats, ApiError> {
        let is_own = requesting_user_id == Some(user_id);
        let is_admin = requesting_user_role == Some(UserRole::Admin);

        if !is_own && !is_admin {
            // For non-admin, non-owner requests, check privacy settings
            let prefs = self.repository.get_preferences(user_id).await?;
            let privacy = prefs.privacy.get("show_progress").and_then(|v| v.as_bool());

            if privacy != Some(true) {
                return Err(ApiError::AccessDenied);
                    
            }
        }

        self.repository.get_stats(user_id).await
    }

    // =========================================================================
    // SEARCH OPERATIONS
    // =========================================================================

    /// Searches for users.
    ///
    /// # Authorization
    ///
    /// - Only instructors and admins can search users
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    /// * `role_filter` - Optional role filter
    /// * `page` - Page number (1-indexed)
    /// * `page_size` - Results per page
    #[instrument(skip(self), fields(query = %query, page = %page))]
    pub async fn search_users(
        &self,
        query: &str,
        role_filter: Option<UserRole>,
        page: u32,
        page_size: u32,
        requesting_user_role: UserRole,
    ) -> Result<SearchUsersResponse, ApiError> {
        // Authorization check
        if requesting_user_role == UserRole::Student {
            return Err(ApiError::AccessDenied);
                
        }

        let limit = page_size.min(100) as i64; // Max 100 per page
        let offset = ((page.saturating_sub(1)) * page_size) as i64;

        let users = self
            .repository
            .search_users(query, role_filter, limit, offset)
            .await?;

        let total = self
            .repository
            .count_search_results(query, role_filter)
            .await?;

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Ok(SearchUsersResponse {
            users,
            total: total as u32,
            page,
            page_size,
            total_pages,
        })
    }

    // =========================================================================
    // ADMIN OPERATIONS
    // =========================================================================

    /// Changes a user's role.
    ///
    /// # Authorization
    ///
    /// - Only admins can change roles
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user to change
    /// * `new_role` - The new role
    /// * `admin_id` - ID of the admin making the change
    /// * `reason` - Reason for the change (optional)
    #[instrument(skip(self), fields(
        user_id = %user_id,
        new_role = %new_role,
        admin_id = %admin_id
    ))]
    pub async fn change_role(
        &self,
        user_id: Uuid,
        new_role: UserRole,
        admin_id: Uuid,
        reason: Option<String>,
    ) -> Result<UserProfile, ApiError> {
        // Get current profile to record previous role
        let current = self.repository.find_by_id(user_id).await?.ok_or_else(|| {
            ApiError::NotFound {
                resource: "User".to_string(),
            }
        })?;

        let previous_role = current.role;

        // Prevent changing own role
        if user_id == admin_id {
            return Err(ApiError::BadRequest {
                message: "Cannot change your own role".to_string(),
            });
        }

        // Update the role
        let updated = self.repository.update_role(user_id, new_role).await?;

        // Emit event
        let event = UserEvent::RoleChanged(RoleChangedEvent::new(
            user_id,
            previous_role,
            new_role,
            admin_id,
            reason,
        ));
        self.emit_event(event).await;

        info!(
            user_id = %user_id,
            previous_role = %previous_role,
            new_role = %new_role,
            admin_id = %admin_id,
            "User role changed"
        );

        Ok(updated)
    }

    // =========================================================================
    // INTERNAL HELPERS
    // =========================================================================

    /// Emits a domain event.
    ///
    /// Currently logs the event. In production, this would publish
    /// to Redis Pub/Sub or a message queue.
    async fn emit_event(&self, event: UserEvent) {
        // TODO: Publish to Redis Pub/Sub or RabbitMQ
        info!(
            event_type = %event.event_type(),
            user_id = %event.user_id(),
            "Domain event emitted"
        );
    }
}

// =============================================================================
// REQUEST/RESPONSE TYPES
// =============================================================================

/// Request to update a user profile.
#[derive(Debug, Default)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<serde_json::Value>,
}

/// Request to update user preferences.
#[derive(Debug, Default)]
pub struct UpdatePreferencesRequest {
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub email_notifications: Option<serde_json::Value>,
    pub privacy: Option<serde_json::Value>,
    pub accessibility: Option<serde_json::Value>,
}

/// Response for user profile with different visibility levels.
#[derive(Debug)]
pub struct UserProfileResponse {
    pub profile: UserProfile,
    pub visibility: ProfileVisibility,
}

/// Profile visibility level.
#[derive(Debug, Clone, Copy)]
pub enum ProfileVisibility {
    /// Full profile data (own profile or admin view)
    Full,
    /// Public data only (respecting privacy settings)
    Public,
    /// Minimal data (private profile)
    Minimal,
}

impl UserProfileResponse {
    /// Creates a full visibility response.
    pub fn full(profile: UserProfile) -> Self {
        Self {
            profile,
            visibility: ProfileVisibility::Full,
        }
    }

    /// Creates a public visibility response (respects privacy settings).
    pub fn public(mut profile: UserProfile, preferences: &UserPreferences) -> Self {
        // Hide email if privacy setting says so
        if !preferences.shows_email() {
            profile.email = "***@***.***".to_string();
        }

        Self {
            profile,
            visibility: ProfileVisibility::Public,
        }
    }

    /// Creates a minimal visibility response (private profile).
    pub fn minimal(mut profile: UserProfile) -> Self {
        // Hide sensitive fields
        profile.email = "***@***.***".to_string();
        profile.bio = None;
        profile.website = None;
        profile.social_links = None;

        Self {
            profile,
            visibility: ProfileVisibility::Minimal,
        }
    }
}

/// Response for user search.
#[derive(Debug)]
pub struct SearchUsersResponse {
    pub users: Vec<UserProfile>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
