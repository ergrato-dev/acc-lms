//! # Domain Events
//!
//! Domain events represent significant state changes in the authentication domain.
//! They enable **event-driven architecture** patterns such as:
//!
//! - **Audit logging**: Record all authentication events
//! - **Notifications**: Trigger emails on registration, password changes
//! - **Analytics**: Track user behavior patterns
//! - **Integration**: Communicate with other microservices
//!
//! ## Event Flow
//!
//! ```text
//! ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────────┐
//! │  Auth Service   │────▶│  Event Emitter  │────▶│  Event Handlers     │
//! │  (produces)     │     │  (dispatches)   │     │  (consumes)         │
//! └─────────────────┘     └─────────────────┘     └─────────────────────┘
//!                                                         │
//!                                 ┌───────────────────────┼───────────────────────┐
//!                                 ▼                       ▼                       ▼
//!                         ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
//!                         │ Audit Log   │         │ Email Svc   │         │ Analytics   │
//!                         │ (persist)   │         │ (notify)    │         │ (track)     │
//!                         └─────────────┘         └─────────────┘         └─────────────┘
//! ```
//!
//! ## Available Events
//!
//! | Event              | Trigger                           | Typical Actions              |
//! |--------------------|-----------------------------------|------------------------------|
//! | `UserRegistered`   | New user completes registration   | Welcome email, analytics     |
//! | `UserLoggedIn`     | Successful authentication         | Audit log, session tracking  |
//! | `UserLoggedOut`    | User ends session                 | Audit log, cleanup           |
//! | `PasswordChanged`  | User updates password             | Security email, audit log    |
//! | `EmailVerified`    | User confirms email address       | Update profile, analytics    |
//! | `PasswordResetRequested` | User requests reset         | Send reset email             |
//! | `LoginFailed`      | Failed authentication attempt     | Security monitoring          |
//!
//! ## Future Integration
//!
//! Events can be published to message queues (RabbitMQ, Kafka) for
//! asynchronous processing by other services. Current implementation
//! uses in-process handlers.
//!
//! ## Related Documentation
//!
//! - Event-driven patterns: `_docs/development/development-standards.md`
//! - Notification service: `be/notifications-service/`
//! - Analytics service: `be/analytics-service/`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// BASE EVENT TRAIT
// =============================================================================

/// Common interface for all domain events.
///
/// Events implementing this trait can be:
/// - Serialized to JSON for message queues
/// - Stored in an event log for audit purposes
/// - Dispatched to multiple handlers
pub trait DomainEvent: Serialize + Send + Sync {
    /// Returns the event type name for routing/filtering.
    fn event_type(&self) -> &'static str;

    /// Returns when the event occurred.
    fn occurred_at(&self) -> DateTime<Utc>;

    /// Returns the aggregate ID (usually user_id) this event relates to.
    fn aggregate_id(&self) -> Uuid;
}

// =============================================================================
// USER REGISTRATION EVENTS
// =============================================================================

/// Emitted when a new user successfully registers.
///
/// # Handlers
///
/// - **Email Service**: Send welcome email with verification link
/// - **Analytics**: Track registration source and demographics
/// - **Audit Log**: Record registration timestamp and IP
///
/// # Example
///
/// ```rust,ignore
/// let event = UserRegistered {
///     user_id: new_user.user_id,
///     email: new_user.email.clone(),
///     role: new_user.role.clone(),
///     registration_ip: request.peer_addr().map(|a| a.ip().to_string()),
///     occurred_at: Utc::now(),
/// };
/// event_emitter.emit(event).await;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    /// The newly created user's ID
    pub user_id: Uuid,
    /// User's email address
    pub email: String,
    /// Assigned role (student, instructor, admin)
    pub role: String,
    /// IP address during registration (for security/analytics)
    pub registration_ip: Option<String>,
    /// When registration completed
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for UserRegistered {
    fn event_type(&self) -> &'static str {
        "user.registered"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

// =============================================================================
// AUTHENTICATION EVENTS
// =============================================================================

/// Emitted when a user successfully logs in.
///
/// # Handlers
///
/// - **Audit Log**: Record login time, IP, and device
/// - **Analytics**: Track login patterns, peak usage times
/// - **Security**: Detect unusual login locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    /// User who logged in
    pub user_id: Uuid,
    /// IP address during login
    pub ip_address: Option<String>,
    /// Browser/client identifier
    pub user_agent: Option<String>,
    /// Device fingerprint if available
    pub device_fingerprint: Option<String>,
    /// When login occurred
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for UserLoggedIn {
    fn event_type(&self) -> &'static str {
        "user.logged_in"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

/// Emitted when a user logs out.
///
/// # Handlers
///
/// - **Audit Log**: Record logout time
/// - **Session Manager**: Clean up cached session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedOut {
    /// User who logged out
    pub user_id: Uuid,
    /// Whether all sessions were terminated (logout-all)
    pub all_sessions: bool,
    /// When logout occurred
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for UserLoggedOut {
    fn event_type(&self) -> &'static str {
        "user.logged_out"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

/// Emitted when a login attempt fails.
///
/// # Security Use Cases
///
/// - **Rate Limiting**: Block after N failed attempts
/// - **Account Lockout**: Temporarily lock account
/// - **Alerting**: Notify user of suspicious activity
/// - **Analytics**: Track attack patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginFailed {
    /// Email that was attempted (may not exist)
    pub attempted_email: String,
    /// Reason for failure
    pub reason: LoginFailureReason,
    /// IP address of the attempt
    pub ip_address: Option<String>,
    /// Browser/client identifier
    pub user_agent: Option<String>,
    /// When the attempt occurred
    pub occurred_at: DateTime<Utc>,
}

/// Reasons why a login attempt failed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoginFailureReason {
    /// User with this email doesn't exist
    UserNotFound,
    /// Password doesn't match
    InvalidPassword,
    /// Account is soft-deleted
    AccountDeleted,
    /// Account is locked due to too many attempts
    AccountLocked,
    /// Email not yet verified (if required)
    EmailNotVerified,
}

impl DomainEvent for LoginFailed {
    fn event_type(&self) -> &'static str {
        "user.login_failed"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    /// Returns a zero UUID since there's no valid user
    fn aggregate_id(&self) -> Uuid {
        Uuid::nil()
    }
}

// =============================================================================
// PASSWORD EVENTS
// =============================================================================

/// Emitted when a user changes their password.
///
/// # Handlers
///
/// - **Email Service**: Send security notification email
/// - **Session Manager**: Optionally invalidate all other sessions
/// - **Audit Log**: Record password change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChanged {
    /// User who changed password
    pub user_id: Uuid,
    /// IP address during change
    pub ip_address: Option<String>,
    /// Whether change was via reset flow or settings
    pub via_reset: bool,
    /// When password was changed
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PasswordChanged {
    fn event_type(&self) -> &'static str {
        "user.password_changed"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

/// Emitted when a user requests a password reset.
///
/// # Handlers
///
/// - **Email Service**: Send password reset email with token
/// - **Audit Log**: Record reset request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequested {
    /// User requesting reset
    pub user_id: Uuid,
    /// User's email address
    pub email: String,
    /// IP address of request
    pub ip_address: Option<String>,
    /// When reset was requested
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PasswordResetRequested {
    fn event_type(&self) -> &'static str {
        "user.password_reset_requested"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

// =============================================================================
// EMAIL VERIFICATION EVENTS
// =============================================================================

/// Emitted when a user verifies their email address.
///
/// # Handlers
///
/// - **User Service**: Update email_verified flag
/// - **Analytics**: Track verification rates
/// - **Email Service**: Send confirmation email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerified {
    /// User who verified email
    pub user_id: Uuid,
    /// Verified email address
    pub email: String,
    /// When verification occurred
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for EmailVerified {
    fn event_type(&self) -> &'static str {
        "user.email_verified"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

// =============================================================================
// EVENT ENVELOPE (FOR MESSAGE QUEUES)
// =============================================================================

/// Wrapper for events when publishing to message queues.
///
/// Adds metadata needed for reliable message delivery and processing:
/// - Unique event ID for deduplication
/// - Version for schema evolution
/// - Correlation ID for distributed tracing
///
/// # Example
///
/// ```rust,ignore
/// let event = UserRegistered { /* ... */ };
/// let envelope = EventEnvelope::new(event);
/// message_queue.publish("auth.events", envelope).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T: DomainEvent> {
    /// Unique identifier for this event instance
    pub event_id: Uuid,
    /// Event type name for routing
    pub event_type: String,
    /// Schema version for evolution
    pub version: u32,
    /// Correlation ID for distributed tracing
    pub correlation_id: Option<String>,
    /// The actual event data
    pub payload: T,
    /// When the envelope was created
    pub timestamp: DateTime<Utc>,
}

impl<T: DomainEvent> EventEnvelope<T> {
    /// Creates a new event envelope with auto-generated ID and timestamp.
    pub fn new(event: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event.event_type().to_string(),
            version: 1,
            correlation_id: None,
            payload: event,
            timestamp: Utc::now(),
        }
    }

    /// Creates an envelope with a correlation ID for tracing.
    pub fn with_correlation_id(event: T, correlation_id: String) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event.event_type().to_string(),
            version: 1,
            correlation_id: Some(correlation_id),
            payload: event,
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
    fn test_user_registered_event_type() {
        let event = UserRegistered {
            user_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "student".to_string(),
            registration_ip: Some("192.168.1.1".to_string()),
            occurred_at: Utc::now(),
        };

        assert_eq!(event.event_type(), "user.registered");
        assert_eq!(event.aggregate_id(), event.user_id);
    }

    #[test]
    fn test_login_failed_event_nil_aggregate() {
        let event = LoginFailed {
            attempted_email: "unknown@example.com".to_string(),
            reason: LoginFailureReason::UserNotFound,
            ip_address: None,
            user_agent: None,
            occurred_at: Utc::now(),
        };

        // Failed logins have no valid user, so aggregate_id is nil
        assert_eq!(event.aggregate_id(), Uuid::nil());
    }

    #[test]
    fn test_event_envelope_creation() {
        let event = UserLoggedIn {
            user_id: Uuid::new_v4(),
            ip_address: Some("10.0.0.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            device_fingerprint: None,
            occurred_at: Utc::now(),
        };

        let envelope = EventEnvelope::new(event.clone());

        assert_eq!(envelope.event_type, "user.logged_in");
        assert_eq!(envelope.version, 1);
        assert!(envelope.correlation_id.is_none());
        assert_eq!(envelope.payload.user_id, event.user_id);
    }

    #[test]
    fn test_event_serialization() {
        let event = PasswordChanged {
            user_id: Uuid::new_v4(),
            ip_address: Some("172.16.0.1".to_string()),
            via_reset: true,
            occurred_at: Utc::now(),
        };

        let json = serde_json::to_string(&event).expect("serialization should succeed");
        assert!(json.contains("via_reset"));
        assert!(json.contains("true"));
    }
}
