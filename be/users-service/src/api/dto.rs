//! # Data Transfer Objects (DTOs)
//!
//! DTOs define the shape of data exchanged between client and server.
//! They are separate from domain entities to:
//!
//! 1. **Decouple API from Domain**: Changes to domain don't break API
//! 2. **Validation**: Apply input validation rules
//! 3. **Projection**: Return only what clients need
//! 4. **Versioning**: Support multiple API versions
//!
//! ## Naming Convention
//!
//! - `*Request` - Incoming data from client
//! - `*Response` - Outgoing data to client
//! - `*Dto` - Generic data transfer object

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::{UserPreferences, UserProfile, UserRole, UserStats};

// =============================================================================
// PROFILE DTOs
// =============================================================================

/// Response containing user profile information.
///
/// This is the primary response type for profile endpoints.
/// Contains profile data, stats, and metadata.
///
/// # Example Response
///
/// ```json
/// {
///   "user": {
///     "id": "550e8400-e29b-41d4-a716-446655440000",
///     "email": "john@example.com",
///     "firstName": "John",
///     "lastName": "Doe",
///     "role": "student",
///     "avatarUrl": "https://storage.example.com/avatars/123.jpg",
///     "bio": "Learning enthusiast",
///     "website": "https://johndoe.com",
///     "socialLinks": { "twitter": "https://twitter.com/johndoe" },
///     "createdAt": "2024-01-15T10:30:00Z"
///   },
///   "stats": {
///     "coursesEnrolled": 5,
///     "coursesCompleted": 2,
///     "totalLearningTime": "12h 30m"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    /// User profile information
    pub user: UserDto,

    /// User statistics (optional, based on access level)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<UserStatsDto>,
}

impl ProfileResponse {
    /// Creates a profile response from domain entities.
    pub fn from_profile(profile: UserProfile, stats: Option<UserStats>) -> Self {
        Self {
            user: UserDto::from(profile),
            stats: stats.map(UserStatsDto::from),
        }
    }
}

/// User data transfer object.
///
/// Represents user data in API responses.
/// Uses camelCase for JSON serialization (JavaScript convention).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    /// User's unique identifier
    pub id: Uuid,

    /// User's email address
    pub email: String,

    /// User's first name
    pub first_name: String,

    /// User's last name
    pub last_name: String,

    /// User's display name (computed)
    pub display_name: String,

    /// User's role in the system
    pub role: String,

    /// URL to user's avatar image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// User's biography
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// User's website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Social media links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<serde_json::Value>,

    /// When the account was created
    pub created_at: DateTime<Utc>,
}

impl From<UserProfile> for UserDto {
    fn from(profile: UserProfile) -> Self {
        Self {
            id: profile.id,
            display_name: profile.display_name(),
            email: profile.email,
            first_name: profile.first_name,
            last_name: profile.last_name,
            role: profile.role.to_string(),
            avatar_url: profile.avatar_url,
            bio: profile.bio,
            website: profile.website,
            social_links: profile.social_links,
            created_at: profile.created_at,
        }
    }
}

/// Request to update user profile.
///
/// All fields are optional - only provided fields will be updated.
///
/// # Validation Rules
///
/// - `firstName`: 1-50 characters
/// - `lastName`: 1-50 characters
/// - `bio`: Max 500 characters
/// - `website`: Valid URL format
///
/// # Example Request
///
/// ```json
/// {
///   "firstName": "John",
///   "bio": "Updated bio",
///   "socialLinks": {
///     "twitter": "https://twitter.com/johndoe"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    /// New first name
    #[validate(length(min = 1, max = 50, message = "First name must be 1-50 characters"))]
    pub first_name: Option<String>,

    /// New last name
    #[validate(length(min = 1, max = 50, message = "Last name must be 1-50 characters"))]
    pub last_name: Option<String>,

    /// New biography
    #[validate(length(max = 500, message = "Bio must be at most 500 characters"))]
    pub bio: Option<String>,

    /// New website URL
    #[validate(url(message = "Invalid website URL"))]
    pub website: Option<String>,

    /// New social links
    pub social_links: Option<serde_json::Value>,
}

// =============================================================================
// PREFERENCES DTOs
// =============================================================================

/// Response containing user preferences.
///
/// # Example Response
///
/// ```json
/// {
///   "language": "es",
///   "timezone": "America/Mexico_City",
///   "emailNotifications": {
///     "marketing": true,
///     "courseUpdates": true,
///     "reminders": true
///   },
///   "privacy": {
///     "showEmail": false,
///     "showProfile": true,
///     "showProgress": true
///   },
///   "accessibility": {
///     "highContrast": false,
///     "fontSize": "normal"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesResponse {
    /// Preferred language code (es, en, pt)
    pub language: String,

    /// User's timezone (IANA format)
    pub timezone: String,

    /// Email notification settings
    pub email_notifications: serde_json::Value,

    /// Privacy settings
    pub privacy: serde_json::Value,

    /// Accessibility settings
    pub accessibility: serde_json::Value,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<UserPreferences> for PreferencesResponse {
    fn from(prefs: UserPreferences) -> Self {
        Self {
            language: prefs.language,
            timezone: prefs.timezone,
            email_notifications: prefs.email_notifications,
            privacy: prefs.privacy,
            accessibility: prefs.accessibility,
            updated_at: prefs.updated_at,
        }
    }
}

/// Request to update user preferences.
///
/// All fields are optional - only provided fields will be updated.
///
/// # Validation Rules
///
/// - `language`: Must be one of: es, en, pt
/// - `timezone`: Must be valid IANA timezone
///
/// # Example Request
///
/// ```json
/// {
///   "language": "en",
///   "emailNotifications": {
///     "marketing": false
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePreferencesRequest {
    /// New language code
    #[validate(custom(function = "validate_language"))]
    pub language: Option<String>,

    /// New timezone
    #[validate(length(min = 1, max = 50, message = "Invalid timezone"))]
    pub timezone: Option<String>,

    /// New email notification settings
    pub email_notifications: Option<serde_json::Value>,

    /// New privacy settings
    pub privacy: Option<serde_json::Value>,

    /// New accessibility settings
    pub accessibility: Option<serde_json::Value>,
}

/// Validates language code.
fn validate_language(language: &str) -> Result<(), validator::ValidationError> {
    const VALID_LANGUAGES: &[&str] = &["es", "en", "pt"];

    if VALID_LANGUAGES.contains(&language) {
        Ok(())
    } else {
        let mut error = validator::ValidationError::new("invalid_language");
        error.message = Some("Language must be one of: es, en, pt".into());
        Err(error)
    }
}

// =============================================================================
// STATS DTOs
// =============================================================================

/// User statistics data transfer object.
///
/// Provides a formatted view of user learning statistics.
///
/// # Example Response
///
/// ```json
/// {
///   "coursesEnrolled": 5,
///   "coursesCompleted": 2,
///   "certificatesEarned": 1,
///   "totalLearningTime": "12h 30m",
///   "averageCompletionRate": 75.5,
///   "currentStreakDays": 5,
///   "longestStreakDays": 14,
///   "lastActivityAt": "2024-01-15T10:30:00Z"
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsDto {
    /// Total courses enrolled
    pub courses_enrolled: i32,

    /// Courses completed (100% progress)
    pub courses_completed: i32,

    /// Certificates earned
    pub certificates_earned: i32,

    /// Total learning time (formatted string)
    pub total_learning_time: String,

    /// Total learning time in minutes (for charts/calculations)
    pub total_learning_time_minutes: i64,

    /// Average completion rate (percentage)
    pub average_completion_rate: f64,

    /// Current streak in days
    pub current_streak_days: i32,

    /// Longest streak ever achieved
    pub longest_streak_days: i32,

    /// Last activity timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<DateTime<Utc>>,
}

impl From<UserStats> for UserStatsDto {
    fn from(stats: UserStats) -> Self {
        Self {
            courses_enrolled: stats.courses_enrolled,
            courses_completed: stats.courses_completed,
            certificates_earned: stats.certificates_earned,
            total_learning_time: stats.formatted_learning_time(),
            total_learning_time_minutes: stats.total_learning_time_minutes,
            average_completion_rate: stats.completion_percentage(),
            current_streak_days: stats.current_streak_days,
            longest_streak_days: stats.longest_streak_days,
            last_activity_at: stats.last_activity_at,
        }
    }
}

// =============================================================================
// SEARCH DTOs
// =============================================================================

/// Request for user search.
///
/// # Query Parameters
///
/// - `q`: Search query (required)
/// - `role`: Filter by role (optional)
/// - `page`: Page number, 1-indexed (default: 1)
/// - `pageSize`: Results per page (default: 20, max: 100)
///
/// # Example
///
/// ```
/// GET /api/v1/users/search?q=john&role=student&page=1&pageSize=20
/// ```
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchUsersQuery {
    /// Search query
    #[validate(length(min = 1, max = 100, message = "Query must be 1-100 characters"))]
    pub q: String,

    /// Filter by role
    pub role: Option<String>,

    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: u32,

    /// Results per page
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 100, message = "Page size must be 1-100"))]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

impl SearchUsersQuery {
    /// Parses the role filter from string to enum.
    pub fn role_filter(&self) -> Option<UserRole> {
        self.role.as_ref().and_then(|r| match r.to_lowercase().as_str() {
            "student" => Some(UserRole::Student),
            "instructor" => Some(UserRole::Instructor),
            "admin" => Some(UserRole::Admin),
            _ => None,
        })
    }
}

/// Response for user search.
///
/// Includes pagination metadata.
///
/// # Example Response
///
/// ```json
/// {
///   "users": [...],
///   "pagination": {
///     "total": 42,
///     "page": 1,
///     "pageSize": 20,
///     "totalPages": 3
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchUsersResponse {
    /// List of matching users
    pub users: Vec<UserDto>,

    /// Pagination metadata
    pub pagination: PaginationMeta,
}

/// Pagination metadata.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    /// Total number of results
    pub total: u32,

    /// Current page number
    pub page: u32,

    /// Results per page
    pub page_size: u32,

    /// Total number of pages
    pub total_pages: u32,
}

// =============================================================================
// AVATAR DTOs
// =============================================================================

/// Response after avatar upload.
///
/// # Example Response
///
/// ```json
/// {
///   "avatarUrl": "https://storage.example.com/avatars/user-123/avatar.jpg",
///   "message": "Avatar uploaded successfully"
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarUploadResponse {
    /// URL of the uploaded avatar
    pub avatar_url: String,

    /// Success message
    pub message: String,
}

/// Response after avatar removal.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarRemoveResponse {
    /// Success message
    pub message: String,
}

// =============================================================================
// ROLE CHANGE DTOs
// =============================================================================

/// Request to change a user's role.
///
/// Only accessible to admins.
///
/// # Example Request
///
/// ```json
/// {
///   "role": "instructor",
///   "reason": "Promoted to instructor after completing certification"
/// }
/// ```
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRoleRequest {
    /// The new role to assign
    #[validate(custom(function = "validate_role"))]
    pub role: String,

    /// Reason for the change (optional but recommended)
    #[validate(length(max = 500, message = "Reason must be at most 500 characters"))]
    pub reason: Option<String>,
}

impl ChangeRoleRequest {
    /// Parses the role string to enum.
    pub fn role_enum(&self) -> Option<UserRole> {
        match self.role.to_lowercase().as_str() {
            "student" => Some(UserRole::Student),
            "instructor" => Some(UserRole::Instructor),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }
}

/// Validates role value.
fn validate_role(role: &str) -> Result<(), validator::ValidationError> {
    const VALID_ROLES: &[&str] = &["student", "instructor", "admin"];

    if VALID_ROLES.contains(&role.to_lowercase().as_str()) {
        Ok(())
    } else {
        let mut error = validator::ValidationError::new("invalid_role");
        error.message = Some("Role must be one of: student, instructor, admin".into());
        Err(error)
    }
}

// =============================================================================
// ERROR DTOs
// =============================================================================

/// Standard error response.
///
/// Used for all error responses to maintain consistency.
///
/// # Example Response
///
/// ```json
/// {
///   "error": {
///     "code": "NOT_FOUND",
///     "message": "User not found",
///     "details": null
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Error information
    pub error: ErrorDetails,
}

/// Error details.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetails {
    /// Error code (e.g., "NOT_FOUND", "VALIDATION_ERROR")
    pub code: String,

    /// Human-readable error message
    pub message: String,

    /// Additional error details (validation errors, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    /// Creates a new error response.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: ErrorDetails {
                code: code.into(),
                message: message.into(),
                details: None,
            },
        }
    }

    /// Creates an error response with details.
    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            error: ErrorDetails {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            },
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
    fn test_validate_language() {
        assert!(validate_language("es").is_ok());
        assert!(validate_language("en").is_ok());
        assert!(validate_language("pt").is_ok());
        assert!(validate_language("fr").is_err());
    }

    #[test]
    fn test_validate_role() {
        assert!(validate_role("student").is_ok());
        assert!(validate_role("INSTRUCTOR").is_ok());
        assert!(validate_role("Admin").is_ok());
        assert!(validate_role("superuser").is_err());
    }

    #[test]
    fn test_search_query_defaults() {
        let query: SearchUsersQuery = serde_json::from_str(r#"{"q": "test"}"#).unwrap();
        assert_eq!(query.page, 1);
        assert_eq!(query.page_size, 20);
    }

    #[test]
    fn test_error_response() {
        let error = ErrorResponse::new("NOT_FOUND", "User not found");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("NOT_FOUND"));
        assert!(json.contains("User not found"));
    }
}
