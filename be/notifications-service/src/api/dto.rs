//! # API Data Transfer Objects
//!
//! Request and response DTOs for the HTTP API.

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Notification, NotificationStats, NotificationStatus, NotificationType, Template, UserSettings,
};

// =============================================================================
// TEMPLATE DTOs
// =============================================================================

/// Request to create a new template.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTemplateRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub notification_type: NotificationType,

    #[validate(length(max = 500))]
    pub subject_template: Option<String>,

    #[validate(length(min = 1, max = 10000))]
    pub body_template: String,

    pub variables: Option<serde_json::Value>,
}

/// Request to update a template.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTemplateRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub notification_type: Option<NotificationType>,

    #[validate(length(max = 500))]
    pub subject_template: Option<Option<String>>,

    #[validate(length(min = 1, max = 10000))]
    pub body_template: Option<String>,

    pub variables: Option<serde_json::Value>,

    pub is_active: Option<bool>,
}

/// Template response.
#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub template_id: Uuid,
    pub name: String,
    pub notification_type: NotificationType,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Template> for TemplateResponse {
    fn from(t: Template) -> Self {
        Self {
            template_id: t.template_id,
            name: t.name,
            notification_type: t.notification_type,
            subject_template: t.subject_template,
            body_template: t.body_template,
            variables: t.variables,
            is_active: t.is_active,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// =============================================================================
// NOTIFICATION DTOs
// =============================================================================

/// Request to send a notification using a template.
#[derive(Debug, Deserialize, Validate)]
pub struct SendNotificationRequest {
    pub user_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub template_name: String,

    pub variables: serde_json::Value,

    #[validate(range(min = 1, max = 10))]
    pub priority: Option<i32>,

    pub scheduled_for: Option<DateTime<Utc>>,
}

/// Request to create a notification directly.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,

    pub template_id: Uuid,

    pub notification_type: NotificationType,

    #[validate(length(max = 500))]
    pub subject: Option<String>,

    #[validate(length(min = 1, max = 10000))]
    pub content: String,

    #[validate(range(min = 1, max = 10))]
    pub priority: Option<i32>,

    pub scheduled_for: Option<DateTime<Utc>>,

    pub metadata: Option<serde_json::Value>,
}

/// Notification response.
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub template_id: Uuid,
    pub notification_type: NotificationType,
    pub subject: Option<String>,
    pub content: String,
    pub status: NotificationStatus,
    pub priority: i32,
    pub scheduled_for: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            notification_id: n.notification_id,
            user_id: n.user_id,
            template_id: n.template_id,
            notification_type: n.notification_type,
            subject: n.subject,
            content: n.content,
            status: n.status,
            priority: n.priority,
            scheduled_for: n.scheduled_for,
            sent_at: n.sent_at,
            read_at: n.read_at,
            error_message: n.error_message,
            retry_count: n.retry_count,
            created_at: n.created_at,
        }
    }
}

/// Notification list response.
#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: usize,
}

/// Notification statistics response.
#[derive(Debug, Serialize)]
pub struct NotificationStatsResponse {
    pub total: i64,
    pub pending: i64,
    pub sent: i64,
    pub failed: i64,
    pub read: i64,
}

impl From<NotificationStats> for NotificationStatsResponse {
    fn from(s: NotificationStats) -> Self {
        Self {
            total: s.total,
            pending: s.pending,
            sent: s.sent,
            failed: s.failed,
            read: s.read,
        }
    }
}

/// Unread count response.
#[derive(Debug, Serialize)]
pub struct UnreadCountResponse {
    pub unread_count: i64,
}

// =============================================================================
// USER SETTINGS DTOs
// =============================================================================

/// Request to update user settings.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserSettingsRequest {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,

    #[validate(length(max = 5))]
    pub quiet_hours_start: Option<Option<String>>,

    #[validate(length(max = 5))]
    pub quiet_hours_end: Option<Option<String>>,

    #[validate(length(max = 50))]
    pub timezone: Option<Option<String>>,
}

/// User settings response.
#[derive(Debug, Serialize)]
pub struct UserSettingsResponse {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub in_app_enabled: bool,
    pub sms_enabled: bool,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub timezone: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserSettings> for UserSettingsResponse {
    fn from(s: UserSettings) -> Self {
        Self {
            user_id: s.user_id,
            email_enabled: s.email_enabled,
            push_enabled: s.push_enabled,
            in_app_enabled: s.in_app_enabled,
            sms_enabled: s.sms_enabled,
            quiet_hours_start: s.quiet_hours_start,
            quiet_hours_end: s.quiet_hours_end,
            timezone: s.timezone,
            updated_at: s.updated_at,
        }
    }
}

// =============================================================================
// COMMON DTOs
// =============================================================================

/// Query parameters for listing.
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl ListQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
}

/// Query parameters for templates.
#[derive(Debug, Deserialize)]
pub struct TemplateListQuery {
    pub include_inactive: Option<bool>,
    pub notification_type: Option<NotificationType>,
}

/// Generic API error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn with_details(error: &str, message: &str, details: serde_json::Value) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            details: Some(details),
        }
    }
}

/// Success response wrapper.
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

/// Message-only response.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
        }
    }
}
