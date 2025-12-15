//! # Domain Events
//!
//! Events emitted by the users-service when significant domain actions occur.
//! These events are published to a message broker (Redis Pub/Sub or RabbitMQ)
//! and can be consumed by other services for:
//!
//! - **Analytics**: Track user engagement and behavior
//! - **Notifications**: Trigger email or push notifications
//! - **Audit**: Maintain audit logs of user actions
//! - **Sync**: Keep data synchronized across services
//!
//! ## Event Structure
//!
//! All events follow a common structure:
//! - `event_type`: Identifier for the event type
//! - `timestamp`: When the event occurred (UTC)
//! - `payload`: Event-specific data
//!
//! ## Naming Convention
//!
//! Events follow the pattern: `{aggregate}.{action}`
//! Examples: `user.profile_updated`, `user.avatar_uploaded`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::UserRole;

// =============================================================================
// USER EVENT ENUM
// =============================================================================

/// All possible events emitted by the users-service.
///
/// This enum provides type-safe event handling and ensures all events
/// have the required structure.
///
/// # Usage
///
/// ```rust
/// use users_service::domain::events::UserEvent;
///
/// let event = UserEvent::ProfileUpdated(ProfileUpdatedEvent {
///     user_id: user.id,
///     fields_changed: vec!["bio".to_string(), "website".to_string()],
///     timestamp: Utc::now(),
/// });
///
/// // Serialize for publishing
/// let json = serde_json::to_string(&event)?;
/// redis.publish("user-events", json).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "payload")]
pub enum UserEvent {
    /// User profile was updated
    #[serde(rename = "user.profile_updated")]
    ProfileUpdated(ProfileUpdatedEvent),
    
    /// User preferences were changed
    #[serde(rename = "user.preferences_changed")]
    PreferencesChanged(PreferencesChangedEvent),
    
    /// User avatar was uploaded or changed
    #[serde(rename = "user.avatar_updated")]
    AvatarUpdated(AvatarUpdatedEvent),
    
    /// User avatar was removed
    #[serde(rename = "user.avatar_removed")]
    AvatarRemoved(AvatarRemovedEvent),
    
    /// User role was changed (admin action)
    #[serde(rename = "user.role_changed")]
    RoleChanged(RoleChangedEvent),
    
    /// User was deactivated
    #[serde(rename = "user.deactivated")]
    Deactivated(UserDeactivatedEvent),
    
    /// User was reactivated
    #[serde(rename = "user.reactivated")]
    Reactivated(UserReactivatedEvent),
}

impl UserEvent {
    /// Returns the event type as a string.
    ///
    /// Useful for routing and filtering events.
    pub fn event_type(&self) -> &'static str {
        match self {
            UserEvent::ProfileUpdated(_) => "user.profile_updated",
            UserEvent::PreferencesChanged(_) => "user.preferences_changed",
            UserEvent::AvatarUpdated(_) => "user.avatar_updated",
            UserEvent::AvatarRemoved(_) => "user.avatar_removed",
            UserEvent::RoleChanged(_) => "user.role_changed",
            UserEvent::Deactivated(_) => "user.deactivated",
            UserEvent::Reactivated(_) => "user.reactivated",
        }
    }
    
    /// Returns the user ID associated with this event.
    pub fn user_id(&self) -> Uuid {
        match self {
            UserEvent::ProfileUpdated(e) => e.user_id,
            UserEvent::PreferencesChanged(e) => e.user_id,
            UserEvent::AvatarUpdated(e) => e.user_id,
            UserEvent::AvatarRemoved(e) => e.user_id,
            UserEvent::RoleChanged(e) => e.user_id,
            UserEvent::Deactivated(e) => e.user_id,
            UserEvent::Reactivated(e) => e.user_id,
        }
    }
    
    /// Returns the timestamp of this event.
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            UserEvent::ProfileUpdated(e) => e.timestamp,
            UserEvent::PreferencesChanged(e) => e.timestamp,
            UserEvent::AvatarUpdated(e) => e.timestamp,
            UserEvent::AvatarRemoved(e) => e.timestamp,
            UserEvent::RoleChanged(e) => e.timestamp,
            UserEvent::Deactivated(e) => e.timestamp,
            UserEvent::Reactivated(e) => e.timestamp,
        }
    }
}

// =============================================================================
// PROFILE UPDATED EVENT
// =============================================================================

/// Event emitted when a user updates their profile.
///
/// # Fields Changed
///
/// The `fields_changed` array lists which fields were modified:
/// - `first_name`, `last_name`: Name changes
/// - `bio`: Biography update
/// - `website`: Website URL change
/// - `social_links`: Social media links modified
///
/// # Consumers
///
/// - **Search Service**: Re-index user profile
/// - **Analytics**: Track profile completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdatedEvent {
    /// ID of the user whose profile was updated
    pub user_id: Uuid,
    
    /// List of field names that were changed
    pub fields_changed: Vec<String>,
    
    /// When the update occurred
    pub timestamp: DateTime<Utc>,
}

impl ProfileUpdatedEvent {
    /// Creates a new ProfileUpdatedEvent.
    pub fn new(user_id: Uuid, fields_changed: Vec<String>) -> Self {
        Self {
            user_id,
            fields_changed,
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// PREFERENCES CHANGED EVENT
// =============================================================================

/// Event emitted when a user changes their preferences.
///
/// # Categories
///
/// Preferences are grouped into categories:
/// - `language`: UI language changed
/// - `timezone`: Timezone setting changed
/// - `email_notifications`: Email notification settings changed
/// - `privacy`: Privacy settings changed
/// - `accessibility`: Accessibility settings changed
///
/// # Consumers
///
/// - **Notification Service**: Update email preferences
/// - **Frontend**: Clear cached preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesChangedEvent {
    /// ID of the user whose preferences changed
    pub user_id: Uuid,
    
    /// Categories of preferences that changed
    pub categories_changed: Vec<String>,
    
    /// New language setting (if changed)
    pub new_language: Option<String>,
    
    /// New timezone setting (if changed)
    pub new_timezone: Option<String>,
    
    /// When the change occurred
    pub timestamp: DateTime<Utc>,
}

impl PreferencesChangedEvent {
    /// Creates a new PreferencesChangedEvent.
    pub fn new(user_id: Uuid, categories_changed: Vec<String>) -> Self {
        Self {
            user_id,
            categories_changed,
            new_language: None,
            new_timezone: None,
            timestamp: Utc::now(),
        }
    }
    
    /// Sets the new language value.
    pub fn with_language(mut self, language: String) -> Self {
        self.new_language = Some(language);
        self
    }
    
    /// Sets the new timezone value.
    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.new_timezone = Some(timezone);
        self
    }
}

// =============================================================================
// AVATAR EVENTS
// =============================================================================

/// Event emitted when a user uploads or updates their avatar.
///
/// # Consumers
///
/// - **CDN Service**: Propagate new avatar to CDN
/// - **Search Service**: Update avatar URL in index
/// - **Cache Service**: Invalidate avatar cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUpdatedEvent {
    /// ID of the user whose avatar was updated
    pub user_id: Uuid,
    
    /// URL of the new avatar
    pub new_avatar_url: String,
    
    /// URL of the previous avatar (for cleanup)
    pub previous_avatar_url: Option<String>,
    
    /// Size of the new avatar in bytes
    pub file_size_bytes: u64,
    
    /// MIME type of the avatar (e.g., "image/jpeg")
    pub mime_type: String,
    
    /// When the upload occurred
    pub timestamp: DateTime<Utc>,
}

impl AvatarUpdatedEvent {
    /// Creates a new AvatarUpdatedEvent.
    pub fn new(
        user_id: Uuid,
        new_avatar_url: String,
        previous_avatar_url: Option<String>,
        file_size_bytes: u64,
        mime_type: String,
    ) -> Self {
        Self {
            user_id,
            new_avatar_url,
            previous_avatar_url,
            file_size_bytes,
            mime_type,
            timestamp: Utc::now(),
        }
    }
}

/// Event emitted when a user removes their avatar.
///
/// # Consumers
///
/// - **Storage Service**: Delete avatar file from storage
/// - **CDN Service**: Purge avatar from CDN cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarRemovedEvent {
    /// ID of the user whose avatar was removed
    pub user_id: Uuid,
    
    /// URL of the removed avatar (for cleanup)
    pub removed_avatar_url: String,
    
    /// When the removal occurred
    pub timestamp: DateTime<Utc>,
}

impl AvatarRemovedEvent {
    /// Creates a new AvatarRemovedEvent.
    pub fn new(user_id: Uuid, removed_avatar_url: String) -> Self {
        Self {
            user_id,
            removed_avatar_url,
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// ROLE CHANGED EVENT
// =============================================================================

/// Event emitted when a user's role is changed by an admin.
///
/// Role changes are significant security events and should be audited.
///
/// # Authorization
///
/// Only admins can change roles. This event is never emitted by user self-service.
///
/// # Consumers
///
/// - **Auth Service**: Update role in JWT claims
/// - **Audit Service**: Log the role change
/// - **Notification Service**: Notify user of role change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChangedEvent {
    /// ID of the user whose role changed
    pub user_id: Uuid,
    
    /// Previous role
    pub previous_role: UserRole,
    
    /// New role
    pub new_role: UserRole,
    
    /// ID of the admin who made the change
    pub changed_by: Uuid,
    
    /// Reason for the change (optional)
    pub reason: Option<String>,
    
    /// When the change occurred
    pub timestamp: DateTime<Utc>,
}

impl RoleChangedEvent {
    /// Creates a new RoleChangedEvent.
    pub fn new(
        user_id: Uuid,
        previous_role: UserRole,
        new_role: UserRole,
        changed_by: Uuid,
        reason: Option<String>,
    ) -> Self {
        Self {
            user_id,
            previous_role,
            new_role,
            changed_by,
            reason,
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// USER DEACTIVATED/REACTIVATED EVENTS
// =============================================================================

/// Event emitted when a user account is deactivated.
///
/// Deactivation doesn't delete data but prevents login and hides profile.
///
/// # Consumers
///
/// - **Auth Service**: Invalidate all sessions
/// - **Search Service**: Remove from search index
/// - **Notification Service**: Stop all notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeactivatedEvent {
    /// ID of the deactivated user
    pub user_id: Uuid,
    
    /// Reason for deactivation
    pub reason: String,
    
    /// ID of the admin who deactivated (None if self-deactivation)
    pub deactivated_by: Option<Uuid>,
    
    /// When the deactivation occurred
    pub timestamp: DateTime<Utc>,
}

impl UserDeactivatedEvent {
    /// Creates a new UserDeactivatedEvent for admin deactivation.
    pub fn by_admin(user_id: Uuid, reason: String, admin_id: Uuid) -> Self {
        Self {
            user_id,
            reason,
            deactivated_by: Some(admin_id),
            timestamp: Utc::now(),
        }
    }
    
    /// Creates a new UserDeactivatedEvent for self-deactivation.
    pub fn by_self(user_id: Uuid, reason: String) -> Self {
        Self {
            user_id,
            reason,
            deactivated_by: None,
            timestamp: Utc::now(),
        }
    }
}

/// Event emitted when a user account is reactivated.
///
/// # Consumers
///
/// - **Search Service**: Add back to search index
/// - **Notification Service**: Resume notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReactivatedEvent {
    /// ID of the reactivated user
    pub user_id: Uuid,
    
    /// ID of the admin who reactivated
    pub reactivated_by: Uuid,
    
    /// When the reactivation occurred
    pub timestamp: DateTime<Utc>,
}

impl UserReactivatedEvent {
    /// Creates a new UserReactivatedEvent.
    pub fn new(user_id: Uuid, reactivated_by: Uuid) -> Self {
        Self {
            user_id,
            reactivated_by,
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type() {
        let event = UserEvent::ProfileUpdated(ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            vec!["bio".to_string()],
        ));
        
        assert_eq!(event.event_type(), "user.profile_updated");
    }

    #[test]
    fn test_event_serialization() {
        let event = UserEvent::ProfileUpdated(ProfileUpdatedEvent::new(
            Uuid::new_v4(),
            vec!["bio".to_string(), "website".to_string()],
        ));
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("user.profile_updated"));
        assert!(json.contains("bio"));
    }

    #[test]
    fn test_preferences_changed_builder() {
        let event = PreferencesChangedEvent::new(
            Uuid::new_v4(),
            vec!["language".to_string()],
        )
        .with_language("en".to_string())
        .with_timezone("America/New_York".to_string());
        
        assert_eq!(event.new_language, Some("en".to_string()));
        assert_eq!(event.new_timezone, Some("America/New_York".to_string()));
    }
}
