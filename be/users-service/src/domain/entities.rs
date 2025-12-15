//! # User Domain Entities
//!
//! This module defines the core domain entities for user management.
//! These entities represent the business objects in our domain and
//! encapsulate both data and behavior.
//!
//! ## Entities
//!
//! - [`UserProfile`]: Complete user profile with personal information
//! - [`UserPreferences`]: User settings and preferences
//! - [`UserStats`]: Aggregated user statistics
//! - [`UserRole`]: Role-based access control roles
//!
//! ## Design Decisions
//!
//! 1. **Separation of Concerns**: Profile data is separate from authentication
//!    data (which lives in auth-service).
//!
//! 2. **Preferences as Value Object**: Preferences are immutable and replaced
//!    entirely when updated.
//!
//! 3. **Stats are Read-Only**: Statistics are computed from other services
//!    and cached here for quick access.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// USER ROLE
// =============================================================================

/// User role for authorization purposes.
///
/// Roles define what actions a user can perform in the system.
/// Uses a hierarchical model where higher roles include lower role permissions.
///
/// # Role Hierarchy
///
/// ```text
/// Admin → Instructor → Student
///   │         │           │
///   │         │           └── Can view own profile, enroll in courses
///   │         └────────────── Can create courses, view enrolled students
///   └──────────────────────── Can manage all users, view all data
/// ```
///
/// # Database Representation
///
/// Stored as VARCHAR in PostgreSQL with values: 'student', 'instructor', 'admin'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Default role for learners. Can browse courses, enroll, and learn.
    Student,
    
    /// Content creators who can create and manage courses.
    Instructor,
    
    /// System administrators with full access.
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Student
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Student => write!(f, "student"),
            UserRole::Instructor => write!(f, "instructor"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

// =============================================================================
// USER PROFILE
// =============================================================================

/// Complete user profile information.
///
/// This entity contains all public and private profile information for a user.
/// It's the main aggregate root for user data in this service.
///
/// # Privacy
///
/// Some fields are controlled by privacy settings in [`UserPreferences`]:
/// - `email`: May be hidden based on `show_email` preference
/// - `bio`, `website`, `social_links`: May be hidden based on `show_profile` preference
///
/// # Example
///
/// ```rust
/// use users_service::domain::UserProfile;
///
/// let profile = UserProfile {
///     id: Uuid::new_v4(),
///     email: "john@example.com".to_string(),
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     role: UserRole::Student,
///     avatar_url: None,
///     bio: Some("Learning enthusiast".to_string()),
///     website: None,
///     social_links: None,
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    /// Unique identifier (UUID v4)
    pub id: Uuid,
    
    /// User's email address (unique across system)
    pub email: String,
    
    /// User's first name
    pub first_name: String,
    
    /// User's last name
    pub last_name: String,
    
    /// User's role in the system
    pub role: UserRole,
    
    /// URL to user's avatar image (hosted on MinIO/S3)
    /// None if user hasn't uploaded an avatar
    pub avatar_url: Option<String>,
    
    /// User's biography/description
    /// Max 500 characters, supports basic markdown
    pub bio: Option<String>,
    
    /// User's personal website URL
    pub website: Option<String>,
    
    /// Social media links as JSON
    /// Format: {"twitter": "url", "linkedin": "url", "github": "url"}
    pub social_links: Option<serde_json::Value>,
    
    /// When the profile was created
    pub created_at: DateTime<Utc>,
    
    /// When the profile was last updated
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    /// Returns the user's full name.
    ///
    /// # Example
    ///
    /// ```rust
    /// let profile = UserProfile { first_name: "John", last_name: "Doe", ... };
    /// assert_eq!(profile.full_name(), "John Doe");
    /// ```
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    /// Returns the display name for the user.
    ///
    /// Uses first name only if last name is empty, otherwise full name.
    pub fn display_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            self.full_name()
        }
    }
    
    /// Checks if the user has a specific role.
    ///
    /// # Example
    ///
    /// ```rust
    /// if profile.has_role(UserRole::Instructor) {
    ///     // Allow course creation
    /// }
    /// ```
    pub fn has_role(&self, role: UserRole) -> bool {
        self.role == role
    }
    
    /// Checks if user has admin privileges.
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
    
    /// Checks if user has instructor privileges (includes admins).
    pub fn is_instructor_or_above(&self) -> bool {
        matches!(self.role, UserRole::Instructor | UserRole::Admin)
    }
}

// =============================================================================
// USER PREFERENCES
// =============================================================================

/// User preferences and settings.
///
/// Controls how the application behaves for this specific user.
/// All preferences have sensible defaults for new users.
///
/// # Categories
///
/// - **Localization**: Language, timezone
/// - **Notifications**: Email notification settings
/// - **Privacy**: Profile visibility settings
/// - **Accessibility**: High contrast, font size, etc.
///
/// # Default Values
///
/// New users get Spanish (es) as language with America/Mexico_City timezone,
/// all email notifications enabled, and public profile visibility.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    /// User ID this preferences belong to
    pub user_id: Uuid,
    
    /// Preferred language code (ISO 639-1)
    /// Supported: 'es' (Spanish), 'en' (English), 'pt' (Portuguese)
    pub language: String,
    
    /// User's timezone (IANA timezone database)
    /// Example: 'America/Mexico_City', 'Europe/Madrid', 'America/Sao_Paulo'
    pub timezone: String,
    
    /// Email notification settings as JSON
    /// Structure: { "marketing": bool, "course_updates": bool, "reminders": bool }
    pub email_notifications: serde_json::Value,
    
    /// Privacy settings as JSON
    /// Structure: { "show_email": bool, "show_profile": bool, "show_progress": bool }
    pub privacy: serde_json::Value,
    
    /// Accessibility settings as JSON
    /// Structure: { "high_contrast": bool, "font_size": "normal"|"large"|"x-large" }
    pub accessibility: serde_json::Value,
    
    /// When preferences were last updated
    pub updated_at: DateTime<Utc>,
}

impl Default for UserPreferences {
    /// Creates default preferences for new users.
    ///
    /// Defaults to:
    /// - Language: Spanish (es) - primary market
    /// - Timezone: America/Mexico_City
    /// - All email notifications enabled
    /// - Public profile visibility
    /// - Standard accessibility settings
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            language: "es".to_string(),
            timezone: "America/Mexico_City".to_string(),
            email_notifications: serde_json::json!({
                "marketing": true,
                "course_updates": true,
                "reminders": true,
                "weekly_digest": true
            }),
            privacy: serde_json::json!({
                "show_email": false,
                "show_profile": true,
                "show_progress": true
            }),
            accessibility: serde_json::json!({
                "high_contrast": false,
                "font_size": "normal",
                "reduce_motion": false
            }),
            updated_at: Utc::now(),
        }
    }
}

impl UserPreferences {
    /// Creates new preferences for a specific user with defaults.
    pub fn new_for_user(user_id: Uuid) -> Self {
        Self {
            user_id,
            ..Default::default()
        }
    }
    
    /// Gets the language code.
    pub fn get_language(&self) -> &str {
        &self.language
    }
    
    /// Gets the timezone.
    pub fn get_timezone(&self) -> &str {
        &self.timezone
    }
    
    /// Checks if marketing emails are enabled.
    pub fn wants_marketing_emails(&self) -> bool {
        self.email_notifications
            .get("marketing")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }
    
    /// Checks if the user wants their email to be public.
    pub fn shows_email(&self) -> bool {
        self.privacy
            .get("show_email")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
    
    /// Checks if the user wants their profile to be public.
    pub fn shows_profile(&self) -> bool {
        self.privacy
            .get("show_profile")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }
}

// =============================================================================
// USER STATS
// =============================================================================

/// Aggregated user statistics.
///
/// These statistics are computed from data in other services
/// (enrollments, courses, etc.) and cached here for quick access.
/// They're updated asynchronously when relevant events occur.
///
/// # Update Triggers
///
/// - `enrollment.created`: Increment `courses_enrolled`
/// - `course.completed`: Increment `courses_completed`, update `total_learning_time`
/// - `certificate.issued`: Increment `certificates_earned`
///
/// # Caching
///
/// Stats are cached in Redis with a TTL of 5 minutes for performance.
/// Cache is invalidated when any contributing event is received.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserStats {
    /// User ID these stats belong to
    pub user_id: Uuid,
    
    /// Total number of courses the user has enrolled in
    pub courses_enrolled: i32,
    
    /// Number of courses the user has completed (100% progress)
    pub courses_completed: i32,
    
    /// Number of certificates earned
    pub certificates_earned: i32,
    
    /// Total learning time in minutes
    pub total_learning_time_minutes: i64,
    
    /// Average course completion rate (0.0 - 1.0)
    pub average_completion_rate: f64,
    
    /// Current streak in days (consecutive days with activity)
    pub current_streak_days: i32,
    
    /// Longest streak ever achieved
    pub longest_streak_days: i32,
    
    /// Date of last activity
    pub last_activity_at: Option<DateTime<Utc>>,
    
    /// When stats were last calculated
    pub calculated_at: DateTime<Utc>,
}

impl Default for UserStats {
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            courses_enrolled: 0,
            courses_completed: 0,
            certificates_earned: 0,
            total_learning_time_minutes: 0,
            average_completion_rate: 0.0,
            current_streak_days: 0,
            longest_streak_days: 0,
            last_activity_at: None,
            calculated_at: Utc::now(),
        }
    }
}

impl UserStats {
    /// Creates empty stats for a new user.
    pub fn new_for_user(user_id: Uuid) -> Self {
        Self {
            user_id,
            ..Default::default()
        }
    }
    
    /// Formats the total learning time as a human-readable string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let stats = UserStats { total_learning_time_minutes: 125, ... };
    /// assert_eq!(stats.formatted_learning_time(), "2h 5m");
    /// ```
    pub fn formatted_learning_time(&self) -> String {
        let hours = self.total_learning_time_minutes / 60;
        let minutes = self.total_learning_time_minutes % 60;
        
        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
    
    /// Returns completion rate as a percentage (0-100).
    pub fn completion_percentage(&self) -> f64 {
        self.average_completion_rate * 100.0
    }
}

// =============================================================================
// SOCIAL LINKS
// =============================================================================

/// Social media links structure.
///
/// Used for type-safe serialization/deserialization of social links.
/// All fields are optional as users may only have some social profiles.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SocialLinks {
    /// Twitter/X profile URL
    pub twitter: Option<String>,
    
    /// LinkedIn profile URL
    pub linkedin: Option<String>,
    
    /// GitHub profile URL
    pub github: Option<String>,
    
    /// YouTube channel URL
    pub youtube: Option<String>,
    
    /// Personal website URL (duplicate of UserProfile.website for convenience)
    pub website: Option<String>,
}

impl SocialLinks {
    /// Creates a new empty SocialLinks.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Checks if any social link is set.
    pub fn has_any(&self) -> bool {
        self.twitter.is_some()
            || self.linkedin.is_some()
            || self.github.is_some()
            || self.youtube.is_some()
            || self.website.is_some()
    }
    
    /// Converts to JSON value for storage.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::json!({}))
    }
    
    /// Creates from JSON value.
    pub fn from_json(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_default() {
        assert_eq!(UserRole::default(), UserRole::Student);
    }

    #[test]
    fn test_user_profile_full_name() {
        let profile = UserProfile {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            role: UserRole::Student,
            avatar_url: None,
            bio: None,
            website: None,
            social_links: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(profile.full_name(), "John Doe");
    }

    #[test]
    fn test_user_preferences_default() {
        let prefs = UserPreferences::default();
        
        assert_eq!(prefs.language, "es");
        assert_eq!(prefs.timezone, "America/Mexico_City");
    }

    #[test]
    fn test_user_stats_formatted_time() {
        let mut stats = UserStats::default();
        
        stats.total_learning_time_minutes = 125;
        assert_eq!(stats.formatted_learning_time(), "2h 5m");
        
        stats.total_learning_time_minutes = 45;
        assert_eq!(stats.formatted_learning_time(), "45m");
    }

    #[test]
    fn test_social_links() {
        let mut links = SocialLinks::new();
        assert!(!links.has_any());
        
        links.github = Some("https://github.com/user".to_string());
        assert!(links.has_any());
    }
}
