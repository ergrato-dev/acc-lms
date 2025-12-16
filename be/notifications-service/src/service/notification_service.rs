//! # Notification Service
//!
//! Business logic layer for notification management including:
//! - Template-based notification creation
//! - Queue management and delivery scheduling
//! - User preference handling
//! - Retry logic for failed notifications

use chrono::{NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::domain::{
    NewNotification, NewTemplate, NewUserSettings, Notification, NotificationStats,
    NotificationStatus, NotificationType, NotificationWithTemplate, SendNotificationRequest,
    Template, UpdateTemplate, UpdateUserSettings, UserSettings,
    NotificationEvent, TemplateEvent, UserSettingsEvent,
};
use crate::repository::{NotificationRepository, RepositoryError};

// =============================================================================
// SERVICE ERRORS
// =============================================================================

/// Service-level errors.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("User settings not found for user: {0}")]
    UserSettingsNotFound(Uuid),

    #[error("Notification type disabled: {0}")]
    NotificationTypeDisabled(String),

    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    #[error("Quiet hours active for user: {0}")]
    QuietHoursActive(Uuid),

    #[error("Max retries exceeded for notification: {0}")]
    MaxRetriesExceeded(Uuid),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, NotificationError>;

// =============================================================================
// NOTIFICATION SERVICE
// =============================================================================

/// Service for notification business logic.
#[derive(Clone)]
pub struct NotificationService {
    repository: NotificationRepository,
    max_retries: i32,
    default_priority: i32,
}

impl NotificationService {
    /// Creates a new service instance.
    pub fn new(repository: NotificationRepository) -> Self {
        Self {
            repository,
            max_retries: 3,
            default_priority: 5,
        }
    }

    /// Creates a new service with custom configuration.
    pub fn with_config(repository: NotificationRepository, max_retries: i32, default_priority: i32) -> Self {
        Self {
            repository,
            max_retries,
            default_priority,
        }
    }

    // =========================================================================
    // TEMPLATE OPERATIONS
    // =========================================================================

    /// Creates a new notification template.
    pub async fn create_template(&self, template: NewTemplate) -> Result<(Template, TemplateEvent)> {
        // Validate template
        if template.name.trim().is_empty() {
            return Err(NotificationError::Validation("Template name cannot be empty".to_string()));
        }

        if template.body_template.trim().is_empty() {
            return Err(NotificationError::Validation("Body template cannot be empty".to_string()));
        }

        // Check for duplicate name
        if let Some(_) = self.repository.get_template_by_name(&template.name).await? {
            return Err(NotificationError::Validation(format!(
                "Template with name '{}' already exists",
                template.name
            )));
        }

        let created = self.repository.create_template(template).await?;

        let event = TemplateEvent::Created {
            template_id: created.template_id,
            name: created.name.clone(),
            notification_type: created.notification_type,
            timestamp: Utc::now(),
        };

        Ok((created, event))
    }

    /// Gets a template by ID.
    pub async fn get_template(&self, id: Uuid) -> Result<Template> {
        self.repository
            .get_template_by_id(id)
            .await
            .map_err(|e| match e {
                RepositoryError::NotFound(_) => NotificationError::TemplateNotFound(id.to_string()),
                other => NotificationError::Repository(other),
            })
    }

    /// Gets a template by name.
    pub async fn get_template_by_name(&self, name: &str) -> Result<Template> {
        self.repository
            .get_template_by_name(name)
            .await?
            .ok_or_else(|| NotificationError::TemplateNotFound(name.to_string()))
    }

    /// Lists all templates.
    pub async fn list_templates(&self, include_inactive: bool) -> Result<Vec<Template>> {
        Ok(self.repository.list_templates(include_inactive).await?)
    }

    /// Lists templates by notification type.
    pub async fn list_templates_by_type(&self, notification_type: NotificationType) -> Result<Vec<Template>> {
        Ok(self.repository.list_templates_by_type(notification_type).await?)
    }

    /// Updates a template.
    pub async fn update_template(&self, id: Uuid, update: UpdateTemplate) -> Result<(Template, TemplateEvent)> {
        // Validate if name is being updated
        if let Some(ref name) = update.name {
            if name.trim().is_empty() {
                return Err(NotificationError::Validation("Template name cannot be empty".to_string()));
            }
        }

        let updated = self.repository.update_template(id, update).await?;

        let event = TemplateEvent::Updated {
            template_id: updated.template_id,
            name: updated.name.clone(),
            timestamp: Utc::now(),
        };

        Ok((updated, event))
    }

    /// Deactivates a template (soft delete).
    pub async fn deactivate_template(&self, id: Uuid) -> Result<TemplateEvent> {
        let template = self.get_template(id).await?;
        self.repository.deactivate_template(id).await?;

        Ok(TemplateEvent::Deactivated {
            template_id: id,
            name: template.name,
            timestamp: Utc::now(),
        })
    }

    /// Permanently deletes a template.
    pub async fn delete_template(&self, id: Uuid) -> Result<()> {
        self.repository.delete_template(id).await?;
        Ok(())
    }

    // =========================================================================
    // NOTIFICATION OPERATIONS
    // =========================================================================

    /// Sends a notification using a template.
    pub async fn send_notification(
        &self,
        request: SendNotificationRequest,
    ) -> Result<(Notification, NotificationEvent)> {
        // Get template
        let template = self.get_template_by_name(&request.template_name).await?;

        if !template.is_active {
            return Err(NotificationError::InvalidTemplate(format!(
                "Template '{}' is inactive",
                request.template_name
            )));
        }

        // Check user preferences
        let settings = self.repository.get_user_settings(request.user_id).await?;

        if !settings.is_type_enabled(template.notification_type) {
            return Err(NotificationError::NotificationTypeDisabled(
                template.notification_type.to_string(),
            ));
        }

        // Render template
        let (subject, content) = template.render(&request.variables);

        // Create notification
        let new_notification = NewNotification {
            user_id: request.user_id,
            template_id: template.template_id,
            notification_type: template.notification_type,
            subject,
            content,
            priority: request.priority,
            scheduled_for: request.scheduled_for,
            metadata: Some(request.variables.clone()),
        };

        let notification = self.repository.create_notification(new_notification).await?;

        let event = NotificationEvent::Queued {
            notification_id: notification.notification_id,
            user_id: notification.user_id,
            notification_type: notification.notification_type,
            priority: notification.priority,
            scheduled_for: notification.scheduled_for,
            timestamp: Utc::now(),
        };

        Ok((notification, event))
    }

    /// Creates a notification directly (without template).
    pub async fn create_notification(&self, notification: NewNotification) -> Result<(Notification, NotificationEvent)> {
        // Validate
        if notification.content.trim().is_empty() {
            return Err(NotificationError::Validation("Content cannot be empty".to_string()));
        }

        // Check user preferences
        let settings = self.repository.get_user_settings(notification.user_id).await?;

        if !settings.is_type_enabled(notification.notification_type) {
            return Err(NotificationError::NotificationTypeDisabled(
                notification.notification_type.to_string(),
            ));
        }

        let created = self.repository.create_notification(notification).await?;

        let event = NotificationEvent::Queued {
            notification_id: created.notification_id,
            user_id: created.user_id,
            notification_type: created.notification_type,
            priority: created.priority,
            scheduled_for: created.scheduled_for,
            timestamp: Utc::now(),
        };

        Ok((created, event))
    }

    /// Gets a notification by ID.
    pub async fn get_notification(&self, id: Uuid) -> Result<Notification> {
        Ok(self.repository.get_notification_by_id(id).await?)
    }

    /// Gets a notification with its template.
    pub async fn get_notification_with_template(&self, id: Uuid) -> Result<NotificationWithTemplate> {
        Ok(self.repository.get_notification_with_template(id).await?)
    }

    /// Lists notifications for a user.
    pub async fn list_user_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>> {
        Ok(self.repository.list_user_notifications(user_id, limit, offset).await?)
    }

    /// Gets pending notifications ready for delivery.
    pub async fn get_pending_notifications(&self, limit: i64) -> Result<Vec<Notification>> {
        Ok(self.repository.list_pending_notifications(limit).await?)
    }

    /// Marks a notification as sent.
    pub async fn mark_as_sent(&self, id: Uuid) -> Result<(Notification, NotificationEvent)> {
        let notification = self
            .repository
            .update_notification_status(id, NotificationStatus::Sent, None)
            .await?;

        let event = NotificationEvent::Sent {
            notification_id: notification.notification_id,
            user_id: notification.user_id,
            notification_type: notification.notification_type,
            timestamp: Utc::now(),
        };

        Ok((notification, event))
    }

    /// Marks a notification as failed.
    pub async fn mark_as_failed(&self, id: Uuid, error_message: &str) -> Result<(Notification, NotificationEvent)> {
        let notification = self
            .repository
            .update_notification_status(id, NotificationStatus::Failed, Some(error_message.to_string()))
            .await?;

        let event = NotificationEvent::Failed {
            notification_id: notification.notification_id,
            user_id: notification.user_id,
            error_message: error_message.to_string(),
            retry_count: notification.retry_count,
            timestamp: Utc::now(),
        };

        Ok((notification, event))
    }

    /// Marks a notification as read.
    pub async fn mark_as_read(&self, id: Uuid) -> Result<(Notification, NotificationEvent)> {
        let notification = self.repository.mark_notification_read(id).await?;

        let event = NotificationEvent::Read {
            notification_id: notification.notification_id,
            user_id: notification.user_id,
            timestamp: Utc::now(),
        };

        Ok((notification, event))
    }

    /// Retries a failed notification.
    pub async fn retry_notification(&self, id: Uuid) -> Result<(Notification, NotificationEvent)> {
        let current = self.get_notification(id).await?;

        if current.retry_count >= self.max_retries {
            return Err(NotificationError::MaxRetriesExceeded(id));
        }

        // Reset status to pending for retry
        let notification = self
            .repository
            .update_notification_status(id, NotificationStatus::Pending, None)
            .await?;

        let event = NotificationEvent::Retried {
            notification_id: notification.notification_id,
            retry_count: notification.retry_count,
            timestamp: Utc::now(),
        };

        Ok((notification, event))
    }

    /// Gets failed notifications eligible for retry.
    pub async fn get_failed_for_retry(&self) -> Result<Vec<Notification>> {
        Ok(self.repository.list_retriable_notifications(self.max_retries, 100).await?)
    }

    /// Gets notification statistics.
    pub async fn get_stats(&self, user_id: Option<Uuid>) -> Result<NotificationStats> {
        match user_id {
            Some(uid) => Ok(self.repository.get_user_notification_stats(uid).await?),
            None => Ok(NotificationStats { total: 0, pending: 0, sent: 0, failed: 0, read: 0 }),
        }
    }

    /// Counts unread notifications for a user.
    pub async fn count_unread(&self, user_id: Uuid) -> Result<i64> {
        let stats = self.repository.get_user_notification_stats(user_id).await?;
        Ok(stats.pending + stats.sent)  // Pending + Sent = not yet read
    }

    /// Cleans up old notifications.
    /// Note: Actual cleanup should be implemented via scheduled database jobs.
    pub async fn cleanup_old_notifications(&self, _days: i32) -> Result<u64> {
        // This would require a custom query in the repository
        // For now, return 0 - implement via database scheduled job
        Ok(0)
    }

    // =========================================================================
    // USER SETTINGS OPERATIONS
    // =========================================================================

    /// Gets or creates user notification settings.
    pub async fn get_user_settings(&self, user_id: Uuid) -> Result<UserSettings> {
        Ok(self.repository.get_user_settings(user_id).await?)
    }

    /// Updates user notification settings.
    pub async fn update_user_settings(
        &self,
        user_id: Uuid,
        update: UpdateUserSettings,
    ) -> Result<(UserSettings, UserSettingsEvent)> {
        let settings = self.repository.update_user_settings(user_id, update.clone()).await?;

        let event = UserSettingsEvent::Updated {
            user_id,
            changes: serde_json::to_value(&update).unwrap_or_default(),
            timestamp: Utc::now(),
        };

        Ok((settings, event))
    }

    /// Creates user settings with defaults.
    pub async fn create_user_settings(&self, settings: NewUserSettings) -> Result<(UserSettings, UserSettingsEvent)> {
        let created = self.repository.upsert_user_settings(settings).await?;

        let event = UserSettingsEvent::Created {
            user_id: created.user_id,
            timestamp: Utc::now(),
        };

        Ok((created, event))
    }

    /// Checks if notifications should be delivered based on quiet hours.
    pub async fn should_deliver_now(&self, user_id: Uuid) -> Result<bool> {
        let settings = self.get_user_settings(user_id).await?;

        let now = Utc::now().time();
        let naive_now = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second())
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        Ok(!settings.is_quiet_hours(naive_now))
    }

    /// Checks if a notification type is enabled for a user.
    pub async fn is_notification_enabled(
        &self,
        user_id: Uuid,
        notification_type: NotificationType,
    ) -> Result<bool> {
        let settings = self.repository.get_user_settings(user_id).await?;
        Ok(settings.is_type_enabled(notification_type))
    }
}
