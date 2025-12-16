//! # Notification Domain Entities
//!
//! Core entities for the notifications system.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Template (notification template)
//! Notification (notification queue)
//! UserSettings (user preferences)
//! ```

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Notification delivery type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    /// Email notification
    Email,
    /// Push notification
    Push,
    /// In-app notification
    InApp,
    /// SMS text message
    Sms,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::Email => write!(f, "email"),
            NotificationType::Push => write!(f, "push"),
            NotificationType::InApp => write!(f, "in_app"),
            NotificationType::Sms => write!(f, "sms"),
        }
    }
}

impl std::str::FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "email" => Ok(NotificationType::Email),
            "push" => Ok(NotificationType::Push),
            "in_app" => Ok(NotificationType::InApp),
            "sms" => Ok(NotificationType::Sms),
            _ => Err(format!("Invalid notification type: {}", s)),
        }
    }
}

/// Notification delivery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    /// Pending delivery
    Pending,
    /// Successfully sent
    Sent,
    /// Delivery failed
    Failed,
    /// Read by user
    Read,
}

impl Default for NotificationStatus {
    fn default() -> Self {
        NotificationStatus::Pending
    }
}

impl std::fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationStatus::Pending => write!(f, "pending"),
            NotificationStatus::Sent => write!(f, "sent"),
            NotificationStatus::Failed => write!(f, "failed"),
            NotificationStatus::Read => write!(f, "read"),
        }
    }
}

impl std::str::FromStr for NotificationStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(NotificationStatus::Pending),
            "sent" => Ok(NotificationStatus::Sent),
            "failed" => Ok(NotificationStatus::Failed),
            "read" => Ok(NotificationStatus::Read),
            _ => Err(format!("Invalid notification status: {}", s)),
        }
    }
}

// =============================================================================
// TEMPLATE
// =============================================================================

/// Notification template.
///
/// # Database Mapping
///
/// Maps to `notifications.templates` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Template {
    /// Unique identifier
    pub template_id: Uuid,
    /// Unique template name
    pub name: String,
    /// Notification type
    #[sqlx(rename = "type")]
    pub notification_type: NotificationType,
    /// Subject template (for email)
    pub subject_template: Option<String>,
    /// Message body template
    pub body_template: String,
    /// Expected template variables
    pub variables: serde_json::Value,
    /// Whether template is active
    pub is_active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Template {
    /// Renders the template with provided variables.
    pub fn render(&self, variables: &serde_json::Value) -> (Option<String>, String) {
        let subject = self.subject_template.as_ref().map(|s| {
            render_template(s, variables)
        });
        let body = render_template(&self.body_template, variables);
        (subject, body)
    }
}

/// Renders a template by replacing {{variable}} with values.
fn render_template(template: &str, variables: &serde_json::Value) -> String {
    let mut result = template.to_string();

    if let Some(obj) = variables.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }

    result
}

/// Data for creating a new template.
#[derive(Debug, Clone, Deserialize)]
pub struct NewTemplate {
    pub name: String,
    pub notification_type: NotificationType,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub variables: Option<serde_json::Value>,
}

/// Data for updating a template.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub notification_type: Option<NotificationType>,
    pub subject_template: Option<Option<String>>,
    pub body_template: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

// =============================================================================
// NOTIFICATION (QUEUE)
// =============================================================================

/// Notification in the delivery queue.
///
/// # Database Mapping
///
/// Maps to `notifications.queue` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    /// Unique identifier
    pub notification_id: Uuid,
    /// Recipient user
    pub user_id: Uuid,
    /// Template used
    pub template_id: Uuid,
    /// Notification type
    #[sqlx(rename = "type")]
    pub notification_type: NotificationType,
    /// Subject (for email)
    pub subject: Option<String>,
    /// Rendered content
    pub content: String,
    /// Current status
    pub status: NotificationStatus,
    /// Priority (1-5, higher = more priority)
    pub priority: i32,
    /// When to send
    pub scheduled_for: DateTime<Utc>,
    /// When sent
    pub sent_at: Option<DateTime<Utc>>,
    /// When read
    pub read_at: Option<DateTime<Utc>>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Retry count
    pub retry_count: i32,
    /// Additional metadata
    pub metadata: serde_json::Value,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Notification {
    /// Checks if notification can be retried.
    pub fn can_retry(&self, max_retries: i32) -> bool {
        self.status == NotificationStatus::Failed && self.retry_count < max_retries
    }

    /// Checks if notification is ready to send.
    pub fn is_ready_to_send(&self) -> bool {
        self.status == NotificationStatus::Pending && self.scheduled_for <= Utc::now()
    }
}

/// Data for creating a new notification.
#[derive(Debug, Clone, Deserialize)]
pub struct NewNotification {
    pub user_id: Uuid,
    pub template_id: Uuid,
    pub notification_type: NotificationType,
    pub subject: Option<String>,
    pub content: String,
    pub priority: Option<i32>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

/// Data for sending notification using template.
#[derive(Debug, Clone, Deserialize)]
pub struct SendNotificationRequest {
    pub user_id: Uuid,
    pub template_name: String,
    pub variables: serde_json::Value,
    pub priority: Option<i32>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

// =============================================================================
// USER SETTINGS
// =============================================================================

/// User notification preferences.
///
/// # Database Mapping
///
/// Maps to `notifications.user_settings` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    /// User ID
    pub user_id: Uuid,
    /// Email notifications enabled
    pub email_enabled: bool,
    /// Push notifications enabled
    pub push_enabled: bool,
    /// In-app notifications enabled
    pub in_app_enabled: bool,
    /// SMS enabled
    pub sms_enabled: bool,
    /// Quiet hours start time
    pub quiet_hours_start: Option<NaiveTime>,
    /// Quiet hours end time
    pub quiet_hours_end: Option<NaiveTime>,
    /// User timezone
    pub timezone: Option<String>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl UserSettings {
    /// Checks if a notification type is enabled.
    pub fn is_type_enabled(&self, notification_type: NotificationType) -> bool {
        match notification_type {
            NotificationType::Email => self.email_enabled,
            NotificationType::Push => self.push_enabled,
            NotificationType::InApp => self.in_app_enabled,
            NotificationType::Sms => self.sms_enabled,
        }
    }

    /// Checks if current time falls within quiet hours.
    pub fn is_quiet_hours(&self, current_time: NaiveTime) -> bool {
        match (self.quiet_hours_start, self.quiet_hours_end) {
            (Some(start), Some(end)) => {
                if start <= end {
                    current_time >= start && current_time <= end
                } else {
                    // Crosses midnight (e.g., 22:00 - 08:00)
                    current_time >= start || current_time <= end
                }
            }
            _ => false,
        }
    }
}

/// Data for creating user preferences.
#[derive(Debug, Clone, Deserialize)]
pub struct NewUserSettings {
    pub user_id: Uuid,
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub timezone: Option<String>,
}

/// Data for updating user preferences.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserSettings {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub quiet_hours_start: Option<Option<NaiveTime>>,
    pub quiet_hours_end: Option<Option<NaiveTime>>,
    pub timezone: Option<Option<String>>,
}

// =============================================================================
// AGGREGATES
// =============================================================================

/// Notification with template information.
#[derive(Debug, Clone, Serialize)]
pub struct NotificationWithTemplate {
    #[serde(flatten)]
    pub notification: Notification,
    pub template_name: String,
}

/// Notification statistics.
#[derive(Debug, Clone, Serialize)]
pub struct NotificationStats {
    pub total: i64,
    pub pending: i64,
    pub sent: i64,
    pub failed: i64,
    pub read: i64,
}
