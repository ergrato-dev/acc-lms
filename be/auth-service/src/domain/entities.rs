//! # Authentication Domain Entities
//!
//! This module defines the core domain entities for user authentication and
//! session management. Entities represent persistent business objects that
//! map directly to database tables.
//!
//! ## Entity Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                           Domain Entities                               │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │  ┌─────────────────────┐        ┌────────────────────────┐              │
//! │  │        User         │        │    UserPreferences     │              │
//! │  ├─────────────────────┤        ├────────────────────────┤              │
//! │  │ - user_id (PK)      │───────▶│ - user_id (FK)         │              │
//! │  │ - email             │  1:1   │ - email_notifications  │              │
//! │  │ - hashed_password   │        │ - push_notifications   │              │
//! │  │ - role              │        │ - course_reminders     │              │
//! │  │ - email_verified    │        └────────────────────────┘              │
//! │  │ - timestamps        │                                                │
//! │  └─────────┬───────────┘                                                │
//! │            │                                                            │
//! │            │ 1:N                                                        │
//! │            ▼                                                            │
//! │  ┌─────────────────────┐                                                │
//! │  │    RefreshToken     │                                                │
//! │  ├─────────────────────┤                                                │
//! │  │ - token_id (PK)     │                                                │
//! │  │ - user_id (FK)      │                                                │
//! │  │ - token_hash        │                                                │
//! │  │ - device_fingerprint│                                                │
//! │  │ - expires_at        │                                                │
//! │  │ - revoked_at        │                                                │
//! │  └─────────────────────┘                                                │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Entity Types
//!
//! | Entity            | Table              | Purpose                        |
//! |-------------------|--------------------|---------------------------------|
//! | [`User`]          | `users`            | Complete user record from DB    |
//! | [`UserProfile`]   | -                  | Public user data (no secrets)   |
//! | [`UserPreferences`]| `user_preferences`| User notification settings      |
//! | [`RefreshToken`]  | `refresh_tokens`   | Session tracking for tokens     |
//!
//! ## Data Transfer Objects
//!
//! | Type              | Purpose                                          |
//! |-------------------|--------------------------------------------------|
//! | [`NewUser`]       | Data required to create a new user               |
//! | [`NewRefreshToken`]| Data required to create a refresh token         |
//!
//! ## Security Considerations
//!
//! - **Never expose `hashed_password`**: Use [`UserProfile`] for API responses
//! - **Token hashing**: Refresh tokens are stored hashed, never in plaintext
//! - **Soft deletes**: `deleted_at` field enables account recovery
//! - **Password reset tokens**: Time-limited with `password_reset_expires`
//!
//! ## Related Documentation
//!
//! - Database schema: `db/migrations/postgresql/001_initial_schema.sql`
//! - JWT tokens: [`shared::auth::jwt`]
//! - Password hashing: [`shared::auth::password`]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// USER ENTITY
// =============================================================================

/// Complete user entity as stored in the database.
///
/// This struct maps directly to the `users` table and contains all user data,
/// including sensitive fields like `hashed_password`. **Never expose this
/// struct directly in API responses** - use [`UserProfile`] instead.
///
/// # Fields
///
/// | Field                    | Type                    | Description                     |
/// |--------------------------|-------------------------|---------------------------------|
/// | `user_id`                | UUID                    | Primary key                     |
/// | `email`                  | String                  | Unique email address            |
/// | `hashed_password`        | String                  | Argon2id hash (PHC format)      |
/// | `first_name`             | String                  | User's first name               |
/// | `last_name`              | String                  | User's last name                |
/// | `role`                   | String                  | Role: student/instructor/admin  |
/// | `avatar_url`             | Option<String>          | Profile image URL               |
/// | `bio`                    | Option<String>          | User biography                  |
/// | `timezone`               | String                  | User's timezone (e.g., UTC)     |
/// | `language_preference`    | String                  | Preferred language (e.g., es)   |
/// | `email_verified`         | bool                    | Email verification status       |
/// | `email_verification_token`| Option<String>         | Token for email verification    |
/// | `password_reset_token`   | Option<String>          | Token for password reset        |
/// | `password_reset_expires` | Option<DateTime<Utc>>   | Reset token expiration          |
/// | `last_login_at`          | Option<DateTime<Utc>>   | Last successful login           |
/// | `created_at`             | DateTime<Utc>           | Account creation timestamp      |
/// | `updated_at`             | DateTime<Utc>           | Last modification timestamp     |
/// | `deleted_at`             | Option<DateTime<Utc>>   | Soft delete timestamp           |
///
/// # Database Mapping
///
/// ```sql
/// SELECT user_id, email, hashed_password, first_name, last_name, role,
///        avatar_url, bio, timezone, language_preference, email_verified,
///        email_verification_token, password_reset_token, password_reset_expires,
///        last_login_at, created_at, updated_at, deleted_at
/// FROM users
/// WHERE email = $1 AND deleted_at IS NULL;
/// ```
///
/// # Example
///
/// ```rust,ignore
/// let user: User = sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
///     .bind(user_id)
///     .fetch_one(&pool)
///     .await?;
///
/// // Verify password (never compare hashed_password directly!)
/// let is_valid = PasswordHasher::verify(&user.hashed_password, &plain_password)?;
///
/// // Convert to safe profile for API response
/// let profile: UserProfile = user.into();
/// ```
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier (UUID v4)
    pub user_id: Uuid,
    /// User's email address (unique, used for login)
    pub email: String,
    /// Argon2id password hash in PHC format
    /// See: [`shared::auth::password`] for hashing details
    pub hashed_password: String,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User role: "student", "instructor", or "admin"
    pub role: String,
    /// Optional URL to user's avatar image
    pub avatar_url: Option<String>,
    /// Optional user biography/description
    pub bio: Option<String>,
    /// User's timezone (IANA format, e.g., "America/New_York")
    pub timezone: String,
    /// Preferred UI language (ISO 639-1, e.g., "es", "en")
    pub language_preference: String,
    /// Whether the user's email has been verified
    pub email_verified: bool,
    /// Token for email verification (cleared after verification)
    pub email_verification_token: Option<String>,
    /// Token for password reset (cleared after use)
    pub password_reset_token: Option<String>,
    /// Expiration time for password reset token
    pub password_reset_expires: Option<DateTime<Utc>>,
    /// Timestamp of last successful login
    pub last_login_at: Option<DateTime<Utc>>,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp (auto-updated by trigger)
    pub updated_at: DateTime<Utc>,
    /// Soft delete timestamp (null = active account)
    pub deleted_at: Option<DateTime<Utc>>,
}

// =============================================================================
// USER PROFILE (SAFE FOR API RESPONSES)
// =============================================================================

/// Public user profile without sensitive data.
///
/// Use this struct for API responses instead of [`User`]. It excludes:
/// - `hashed_password` - Security critical
/// - `email_verification_token` - Security critical
/// - `password_reset_token` - Security critical
/// - `password_reset_expires` - Internal use only
/// - `updated_at` - Usually not needed in responses
/// - `deleted_at` - Soft-deleted users shouldn't be returned
///
/// # JSON Serialization
///
/// Fields are serialized as camelCase for JavaScript/TypeScript frontend:
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    /// Unique user identifier
    pub user_id: Uuid,
    /// User's email address
    pub email: String,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User role
    pub role: String,
    /// Optional avatar image URL
    pub avatar_url: Option<String>,
    /// Optional user biography
    pub bio: Option<String>,
    /// User's timezone
    pub timezone: String,
    /// Preferred language
    pub language_preference: String,
    /// Email verification status
    pub email_verified: bool,
    /// Last login timestamp
    pub last_login_at: Option<DateTime<Utc>>,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    /// Converts a full [`User`] entity to a safe [`UserProfile`].
    ///
    /// This conversion strips sensitive fields (password hash, tokens)
    /// making the result safe for API responses.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let user: User = repository.find_by_email(&email).await?;
    /// let profile: UserProfile = user.into();
    /// Ok(HttpResponse::Ok().json(profile))
    /// ```
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            avatar_url: user.avatar_url,
            bio: user.bio,
            timezone: user.timezone,
            language_preference: user.language_preference,
            email_verified: user.email_verified,
            last_login_at: user.last_login_at,
            created_at: user.created_at,
        }
    }
}

// =============================================================================
// USER PREFERENCES
// =============================================================================

/// User notification and communication preferences.
///
/// Stored in the `user_preferences` table with a 1:1 relationship to users.
/// Default preferences are created automatically when a user registers.
///
/// # Fields
///
/// | Field                  | Default | Description                        |
/// |------------------------|---------|------------------------------------|
/// | `email_notifications`  | true    | Receive email for important events |
/// | `marketing_emails`     | false   | Receive promotional content        |
/// | `push_notifications`   | true    | Browser/mobile push notifications  |
/// | `course_reminders`     | true    | Reminders for enrolled courses     |
/// | `weekly_progress_email`| true    | Weekly learning progress summary   |
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    /// Foreign key to users table
    pub user_id: Uuid,
    /// Enable email notifications for course updates, assignments, etc.
    pub email_notifications: bool,
    /// Enable marketing and promotional emails
    pub marketing_emails: bool,
    /// Enable browser/mobile push notifications
    pub push_notifications: bool,
    /// Enable reminders for course deadlines and schedules
    pub course_reminders: bool,
    /// Enable weekly learning progress summary email
    pub weekly_progress_email: bool,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// REFRESH TOKEN
// =============================================================================

/// Refresh token record for session management.
///
/// Refresh tokens are stored in the database to enable:
/// - **Token rotation**: Issue new refresh token on each refresh
/// - **Session revocation**: Logout invalidates tokens
/// - **Multi-device tracking**: One user can have multiple active sessions
/// - **Security auditing**: Track login locations and devices
///
/// # Security Model
///
/// ```text
/// Token Flow:
/// ┌──────────────┐     ┌────────────────┐     ┌─────────────────┐
/// │ Login        │────▶│ Generate Token │────▶│ Hash & Store    │
/// │ (plain token)│     │ (random bytes) │     │ (SHA-256 hash)  │
/// └──────────────┘     └────────────────┘     └─────────────────┘
///                             │
///                             ▼
///                      Return to client
///                      (plain token)
///
/// Token Refresh:
/// ┌──────────────┐     ┌────────────────┐     ┌─────────────────┐
/// │ Client sends │────▶│ Hash received  │────▶│ Compare with DB │
/// │ plain token  │     │ token          │     │ (constant time) │
/// └──────────────┘     └────────────────┘     └─────────────────┘
/// ```
///
/// # Fields
///
/// | Field               | Type                  | Purpose                    |
/// |---------------------|-----------------------|----------------------------|
/// | `token_id`          | UUID                  | Primary key                |
/// | `user_id`           | UUID                  | Foreign key to users       |
/// | `token_hash`        | String                | SHA-256 hash of token      |
/// | `device_fingerprint`| Option<String>        | Client device identifier   |
/// | `ip_address`        | Option<String>        | Client IP at creation      |
/// | `user_agent`        | Option<String>        | Browser/client identifier  |
/// | `expires_at`        | DateTime<Utc>         | Token expiration           |
/// | `created_at`        | DateTime<Utc>         | Token creation time        |
/// | `revoked_at`        | Option<DateTime<Utc>> | Revocation timestamp       |
#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    /// Unique token identifier (UUID v4)
    pub token_id: Uuid,
    /// User who owns this token
    pub user_id: Uuid,
    /// SHA-256 hash of the actual token (never store plaintext)
    pub token_hash: String,
    /// Optional device fingerprint for session binding
    pub device_fingerprint: Option<String>,
    /// IP address when token was created
    pub ip_address: Option<String>,
    /// User-Agent header from the creating request
    pub user_agent: Option<String>,
    /// When this token expires
    pub expires_at: DateTime<Utc>,
    /// When this token was created
    pub created_at: DateTime<Utc>,
    /// If set, token has been revoked (logout)
    pub revoked_at: Option<DateTime<Utc>>,
}

// =============================================================================
// DATA TRANSFER OBJECTS FOR CREATION
// =============================================================================

/// Data required to create a new user.
///
/// This struct contains the minimum required fields for user registration.
/// Note that `hashed_password` should already be hashed before creating
/// this struct - never store plain passwords.
///
/// # Example
///
/// ```rust,ignore
/// let hashed = PasswordHasher::hash(&plain_password)?;
/// let new_user = NewUser {
///     email: "user@example.com".to_string(),
///     hashed_password: hashed,
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     role: "student".to_string(),
/// };
/// let user = repository.create(new_user).await?;
/// ```
#[derive(Debug, Clone)]
pub struct NewUser {
    /// Email address (must be unique)
    pub email: String,
    /// Pre-hashed password (Argon2id)
    pub hashed_password: String,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User role (defaults to "student" if not specified)
    pub role: String,
}

/// Data required to create a new refresh token.
///
/// Used when generating tokens after login or token refresh.
///
/// # Security Note
///
/// The `token_hash` should be a SHA-256 hash of the actual token.
/// The plain token is returned to the client but never stored.
///
/// # Example
///
/// ```rust,ignore
/// let plain_token = generate_secure_token(); // Returns random bytes
/// let token_hash = sha256_hash(&plain_token);
///
/// let new_token = NewRefreshToken {
///     user_id,
///     token_hash,
///     device_fingerprint: request.headers().get("X-Device-Fingerprint").map(|h| h.to_string()),
///     ip_address: request.peer_addr().map(|addr| addr.ip().to_string()),
///     user_agent: request.headers().get("User-Agent").map(|h| h.to_string()),
///     expires_at: Utc::now() + Duration::days(7),
/// };
///
/// repository.create_refresh_token(new_token).await?;
/// // Return plain_token to client (not token_hash!)
/// ```
#[derive(Debug, Clone)]
pub struct NewRefreshToken {
    /// User who owns this token
    pub user_id: Uuid,
    /// SHA-256 hash of the actual token
    pub token_hash: String,
    /// Optional device fingerprint
    pub device_fingerprint: Option<String>,
    /// Client IP address
    pub ip_address: Option<String>,
    /// Client User-Agent
    pub user_agent: Option<String>,
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
}
