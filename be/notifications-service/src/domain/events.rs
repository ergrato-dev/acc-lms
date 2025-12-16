//! # Notification Domain Events
//!
//! Domain events for inter-service communication and event sourcing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::{NotificationStatus, NotificationType};

// =============================================================================
// NOTIFICATION EVENTS
// =============================================================================

/// Events related to notification lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum NotificationEvent {
    /// Notification created and queued for delivery
    Queued {
        notification_id: Uuid,
        user_id: Uuid,
        notification_type: NotificationType,
        priority: i32,
        scheduled_for: DateTime<Utc>,
        timestamp: DateTime<Utc>,
    },
    /// Notification successfully sent
    Sent {
        notification_id: Uuid,
        user_id: Uuid,
        notification_type: NotificationType,
        timestamp: DateTime<Utc>,
    },
    /// Notification delivery failed
    Failed {
        notification_id: Uuid,
        user_id: Uuid,
        error_message: String,
        retry_count: i32,
        timestamp: DateTime<Utc>,
    },
    /// Notification marked as read by user
    Read {
        notification_id: Uuid,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Notification queued for retry after failure
    Retried {
        notification_id: Uuid,
        retry_count: i32,
        timestamp: DateTime<Utc>,
    },
}

impl NotificationEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            NotificationEvent::Queued { .. } => "notification.queued",
            NotificationEvent::Sent { .. } => "notification.sent",
            NotificationEvent::Failed { .. } => "notification.failed",
            NotificationEvent::Read { .. } => "notification.read",
            NotificationEvent::Retried { .. } => "notification.retried",
        }
    }

    /// Returns the notification ID.
    pub fn notification_id(&self) -> Uuid {
        match self {
            NotificationEvent::Queued { notification_id, .. } => *notification_id,
            NotificationEvent::Sent { notification_id, .. } => *notification_id,
            NotificationEvent::Failed { notification_id, .. } => *notification_id,
            NotificationEvent::Read { notification_id, .. } => *notification_id,
            NotificationEvent::Retried { notification_id, .. } => *notification_id,
        }
    }
}

// =============================================================================
// TEMPLATE EVENTS
// =============================================================================

/// Events related to notification templates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum TemplateEvent {
    /// Template created
    Created {
        template_id: Uuid,
        name: String,
        notification_type: NotificationType,
        timestamp: DateTime<Utc>,
    },
    /// Template updated
    Updated {
        template_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    /// Template deactivated
    Deactivated {
        template_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
}

impl TemplateEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            TemplateEvent::Created { .. } => "template.created",
            TemplateEvent::Updated { .. } => "template.updated",
            TemplateEvent::Deactivated { .. } => "template.deactivated",
        }
    }
}

// =============================================================================
// USER SETTINGS EVENTS
// =============================================================================

/// Events related to user notification settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum UserSettingsEvent {
    /// User settings created
    Created {
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// User settings updated
    Updated {
        user_id: Uuid,
        changes: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl UserSettingsEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            UserSettingsEvent::Created { .. } => "user_settings.created",
            UserSettingsEvent::Updated { .. } => "user_settings.updated",
        }
    }
}
