//! # Notification Repository
//!
//! Repository implementation with CRUD operations for:
//! - Templates (notification templates)
//! - Queue (notification queue)
//! - UserSettings (user preferences)

use chrono::Utc;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::{
    NewNotification, NewTemplate, NewUserSettings, Notification, NotificationStats,
    NotificationStatus, NotificationType, NotificationWithTemplate, Template, UpdateTemplate,
    UpdateUserSettings, UserSettings,
};

// =============================================================================
// REPOSITORY ERRORS
// =============================================================================

/// Repository-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

// =============================================================================
// NOTIFICATION REPOSITORY
// =============================================================================

/// Repository for notification management.
#[derive(Clone)]
pub struct NotificationRepository {
    pool: PgPool,
}

impl NotificationRepository {
    /// Creates a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // TEMPLATE OPERATIONS
    // =========================================================================

    /// Creates a new template.
    pub async fn create_template(&self, template: NewTemplate) -> Result<Template> {
        let row = sqlx::query(
            r#"
            INSERT INTO notifications.templates (
                name, type, subject_template, body_template,
                variables, is_active
            )
            VALUES ($1, $2, $3, $4, $5, true)
            RETURNING template_id, name, type, subject_template, body_template,
                      variables, is_active, created_at, updated_at
            "#,
        )
        .bind(&template.name)
        .bind(template.notification_type.to_string())
        .bind(&template.subject_template)
        .bind(&template.body_template)
        .bind(&template.variables)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_template_row(&row))
    }

    /// Gets a template by ID.
    pub async fn get_template_by_id(&self, id: Uuid) -> Result<Template> {
        let row = sqlx::query(
            r#"
            SELECT template_id, name, type, subject_template, body_template,
                   variables, is_active, created_at, updated_at
            FROM notifications.templates
            WHERE template_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Template {}", id)))?;

        Ok(self.map_template_row(&row))
    }

    /// Gets a template by name.
    pub async fn get_template_by_name(&self, name: &str) -> Result<Option<Template>> {
        let row = sqlx::query(
            r#"
            SELECT template_id, name, type, subject_template, body_template,
                   variables, is_active, created_at, updated_at
            FROM notifications.templates
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| self.map_template_row(&r)))
    }

    /// Lists all active templates.
    pub async fn list_templates(&self, include_inactive: bool) -> Result<Vec<Template>> {
        let query = if include_inactive {
            r#"
            SELECT template_id, name, type, subject_template, body_template,
                   variables, is_active, created_at, updated_at
            FROM notifications.templates
            ORDER BY name
            "#
        } else {
            r#"
            SELECT template_id, name, type, subject_template, body_template,
                   variables, is_active, created_at, updated_at
            FROM notifications.templates
            WHERE is_active = true
            ORDER BY name
            "#
        };

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| self.map_template_row(r)).collect())
    }

    /// Lists templates by type.
    pub async fn list_templates_by_type(
        &self,
        notification_type: NotificationType,
    ) -> Result<Vec<Template>> {
        let rows = sqlx::query(
            r#"
            SELECT template_id, name, type, subject_template, body_template,
                   variables, is_active, created_at, updated_at
            FROM notifications.templates
            WHERE type = $1 AND is_active = true
            ORDER BY name
            "#,
        )
        .bind(notification_type.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| self.map_template_row(r)).collect())
    }

    /// Updates a template.
    pub async fn update_template(&self, id: Uuid, update: UpdateTemplate) -> Result<Template> {
        let current = self.get_template_by_id(id).await?;

        let row = sqlx::query(
            r#"
            UPDATE notifications.templates
            SET name = $2,
                type = $3,
                subject_template = $4,
                body_template = $5,
                variables = $6,
                is_active = $7,
                updated_at = NOW()
            WHERE template_id = $1
            RETURNING template_id, name, type, subject_template, body_template,
                      variables, is_active, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(update.name.unwrap_or(current.name))
        .bind(
            update
                .notification_type
                .unwrap_or(current.notification_type)
                .to_string(),
        )
        .bind(update.subject_template.unwrap_or(current.subject_template))
        .bind(update.body_template.unwrap_or(current.body_template))
        .bind(update.variables.unwrap_or(current.variables))
        .bind(update.is_active.unwrap_or(current.is_active))
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_template_row(&row))
    }

    /// Deactivates a template (soft delete).
    pub async fn deactivate_template(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE notifications.templates
            SET is_active = false, updated_at = NOW()
            WHERE template_id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(format!("Template {}", id)));
        }

        Ok(())
    }

    /// Permanently deletes a template.
    pub async fn delete_template(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM notifications.templates WHERE template_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(format!("Template {}", id)));
        }

        Ok(())
    }

    fn map_template_row(&self, row: &sqlx::postgres::PgRow) -> Template {
        let type_str: String = row.get("type");
        Template {
            template_id: row.get("template_id"),
            name: row.get("name"),
            notification_type: NotificationType::from_str(&type_str).unwrap_or(NotificationType::Email),
            subject_template: row.get("subject_template"),
            body_template: row.get("body_template"),
            variables: row.get("variables"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    // =========================================================================
    // NOTIFICATION QUEUE OPERATIONS
    // =========================================================================

    /// Queues a new notification.
    pub async fn create_notification(&self, notification: NewNotification) -> Result<Notification> {
        let row = sqlx::query(
            r#"
            INSERT INTO notifications.queue (
                user_id, template_id, type, subject, content,
                priority, scheduled_for, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING notification_id, user_id, template_id, type, subject, content,
                      status, priority, scheduled_for, sent_at, read_at, error_message,
                      retry_count, metadata, created_at
            "#,
        )
        .bind(notification.user_id)
        .bind(notification.template_id)
        .bind(notification.notification_type.to_string())
        .bind(&notification.subject)
        .bind(&notification.content)
        .bind(notification.priority.unwrap_or(3))
        .bind(notification.scheduled_for.unwrap_or_else(Utc::now))
        .bind(&notification.metadata.unwrap_or_else(|| serde_json::json!({})))
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_notification_row(&row))
    }

    /// Gets a notification by ID.
    pub async fn get_notification_by_id(&self, id: Uuid) -> Result<Notification> {
        let row = sqlx::query(
            r#"
            SELECT notification_id, user_id, template_id, type, subject, content,
                   status, priority, scheduled_for, sent_at, read_at, error_message,
                   retry_count, metadata, created_at
            FROM notifications.queue
            WHERE notification_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Notification {}", id)))?;

        Ok(self.map_notification_row(&row))
    }

    /// Gets notification with template information.
    pub async fn get_notification_with_template(
        &self,
        id: Uuid,
    ) -> Result<NotificationWithTemplate> {
        let row = sqlx::query(
            r#"
            SELECT
                n.notification_id, n.user_id, n.template_id, n.type, n.subject, n.content,
                n.status, n.priority, n.scheduled_for, n.sent_at, n.read_at, n.error_message,
                n.retry_count, n.metadata, n.created_at,
                t.name as template_name
            FROM notifications.queue n
            JOIN notifications.templates t ON n.template_id = t.template_id
            WHERE n.notification_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Notification {}", id)))?;

        let notification = self.map_notification_row(&row);
        let template_name: String = row.get("template_name");

        Ok(NotificationWithTemplate {
            notification,
            template_name,
        })
    }

    /// Lists notifications for a user.
    pub async fn list_user_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>> {
        let rows = sqlx::query(
            r#"
            SELECT notification_id, user_id, template_id, type, subject, content,
                   status, priority, scheduled_for, sent_at, read_at, error_message,
                   retry_count, metadata, created_at
            FROM notifications.queue
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| self.map_notification_row(r)).collect())
    }

    /// Lists pending notifications ready to send.
    pub async fn list_pending_notifications(&self, limit: i64) -> Result<Vec<Notification>> {
        let rows = sqlx::query(
            r#"
            SELECT notification_id, user_id, template_id, type, subject, content,
                   status, priority, scheduled_for, sent_at, read_at, error_message,
                   retry_count, metadata, created_at
            FROM notifications.queue
            WHERE status = 'pending' AND scheduled_for <= NOW()
            ORDER BY priority DESC, scheduled_for ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| self.map_notification_row(r)).collect())
    }

    /// Lists failed notifications that can be retried.
    pub async fn list_retriable_notifications(
        &self,
        max_retries: i32,
        limit: i64,
    ) -> Result<Vec<Notification>> {
        let rows = sqlx::query(
            r#"
            SELECT notification_id, user_id, template_id, type, subject, content,
                   status, priority, scheduled_for, sent_at, read_at, error_message,
                   retry_count, metadata, created_at
            FROM notifications.queue
            WHERE status = 'failed' AND retry_count < $1
            ORDER BY priority DESC, created_at ASC
            LIMIT $2
            "#,
        )
        .bind(max_retries)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| self.map_notification_row(r)).collect())
    }

    /// Updates notification status.
    pub async fn update_notification_status(
        &self,
        id: Uuid,
        status: NotificationStatus,
        error_message: Option<String>,
    ) -> Result<Notification> {
        let now = Utc::now();
        let sent_at = if status == NotificationStatus::Sent {
            Some(now)
        } else {
            None
        };

        let row = sqlx::query(
            r#"
            UPDATE notifications.queue
            SET status = $2,
                sent_at = COALESCE($3, sent_at),
                error_message = $4,
                retry_count = CASE WHEN $2 = 'failed' THEN retry_count + 1 ELSE retry_count END
            WHERE notification_id = $1
            RETURNING notification_id, user_id, template_id, type, subject, content,
                      status, priority, scheduled_for, sent_at, read_at, error_message,
                      retry_count, metadata, created_at
            "#,
        )
        .bind(id)
        .bind(status.to_string())
        .bind(sent_at)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_notification_row(&row))
    }

    /// Marks a notification as read.
    pub async fn mark_notification_read(&self, id: Uuid) -> Result<Notification> {
        let row = sqlx::query(
            r#"
            UPDATE notifications.queue
            SET status = 'read', read_at = NOW()
            WHERE notification_id = $1
            RETURNING notification_id, user_id, template_id, type, subject, content,
                      status, priority, scheduled_for, sent_at, read_at, error_message,
                      retry_count, metadata, created_at
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Notification {}", id)))?;

        Ok(self.map_notification_row(&row))
    }

    /// Marks all notifications as read for a user.
    pub async fn mark_all_read(&self, user_id: Uuid) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE notifications.queue
            SET status = 'read', read_at = NOW()
            WHERE user_id = $1 AND status != 'read'
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Deletes a notification.
    pub async fn delete_notification(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM notifications.queue WHERE notification_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(format!("Notification {}", id)));
        }

        Ok(())
    }

    /// Gets notification statistics for a user.
    pub async fn get_user_notification_stats(&self, user_id: Uuid) -> Result<NotificationStats> {
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'pending') as pending,
                COUNT(*) FILTER (WHERE status = 'sent') as sent,
                COUNT(*) FILTER (WHERE status = 'failed') as failed,
                COUNT(*) FILTER (WHERE status = 'read') as read
            FROM notifications.queue
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(NotificationStats {
            total: row.get::<i64, _>("total"),
            pending: row.get::<i64, _>("pending"),
            sent: row.get::<i64, _>("sent"),
            failed: row.get::<i64, _>("failed"),
            read: row.get::<i64, _>("read"),
        })
    }

    fn map_notification_row(&self, row: &sqlx::postgres::PgRow) -> Notification {
        let type_str: String = row.get("type");
        let status_str: String = row.get("status");

        Notification {
            notification_id: row.get("notification_id"),
            user_id: row.get("user_id"),
            template_id: row.get("template_id"),
            notification_type: NotificationType::from_str(&type_str).unwrap_or(NotificationType::Email),
            subject: row.get("subject"),
            content: row.get("content"),
            status: NotificationStatus::from_str(&status_str).unwrap_or(NotificationStatus::Pending),
            priority: row.get("priority"),
            scheduled_for: row.get("scheduled_for"),
            sent_at: row.get("sent_at"),
            read_at: row.get("read_at"),
            error_message: row.get("error_message"),
            retry_count: row.get("retry_count"),
            metadata: row.get("metadata"),
            created_at: row.get("created_at"),
        }
    }

    // =========================================================================
    // USER SETTINGS OPERATIONS
    // =========================================================================

    /// Creates or updates user settings.
    pub async fn upsert_user_settings(&self, settings: NewUserSettings) -> Result<UserSettings> {
        let row = sqlx::query(
            r#"
            INSERT INTO notifications.user_settings (
                user_id, email_enabled, push_enabled, in_app_enabled, sms_enabled,
                quiet_hours_start, quiet_hours_end, timezone
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (user_id) DO UPDATE SET
                email_enabled = COALESCE(EXCLUDED.email_enabled, notifications.user_settings.email_enabled),
                push_enabled = COALESCE(EXCLUDED.push_enabled, notifications.user_settings.push_enabled),
                in_app_enabled = COALESCE(EXCLUDED.in_app_enabled, notifications.user_settings.in_app_enabled),
                sms_enabled = COALESCE(EXCLUDED.sms_enabled, notifications.user_settings.sms_enabled),
                quiet_hours_start = COALESCE(EXCLUDED.quiet_hours_start, notifications.user_settings.quiet_hours_start),
                quiet_hours_end = COALESCE(EXCLUDED.quiet_hours_end, notifications.user_settings.quiet_hours_end),
                timezone = COALESCE(EXCLUDED.timezone, notifications.user_settings.timezone),
                updated_at = NOW()
            RETURNING user_id, email_enabled, push_enabled, in_app_enabled, sms_enabled,
                      quiet_hours_start, quiet_hours_end, timezone, updated_at
            "#,
        )
        .bind(settings.user_id)
        .bind(settings.email_enabled.unwrap_or(true))
        .bind(settings.push_enabled.unwrap_or(true))
        .bind(settings.in_app_enabled.unwrap_or(true))
        .bind(settings.sms_enabled.unwrap_or(false))
        .bind(settings.quiet_hours_start)
        .bind(settings.quiet_hours_end)
        .bind(&settings.timezone)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_user_settings_row(&row))
    }

    /// Gets user settings by user ID.
    pub async fn get_user_settings(&self, user_id: Uuid) -> Result<UserSettings> {
        let row = sqlx::query(
            r#"
            SELECT user_id, email_enabled, push_enabled, in_app_enabled, sms_enabled,
                   quiet_hours_start, quiet_hours_end, timezone, updated_at
            FROM notifications.user_settings
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(self.map_user_settings_row(&r)),
            None => {
                // Return default settings if user has none
                Ok(UserSettings {
                    user_id,
                    email_enabled: true,
                    push_enabled: true,
                    in_app_enabled: true,
                    sms_enabled: false,
                    quiet_hours_start: None,
                    quiet_hours_end: None,
                    timezone: None,
                    updated_at: Utc::now(),
                })
            }
        }
    }

    /// Updates user settings.
    pub async fn update_user_settings(
        &self,
        user_id: Uuid,
        update: UpdateUserSettings,
    ) -> Result<UserSettings> {
        let current = self.get_user_settings(user_id).await?;

        let quiet_start = match update.quiet_hours_start {
            Some(Some(t)) => Some(t),
            Some(None) => None,
            None => current.quiet_hours_start,
        };

        let quiet_end = match update.quiet_hours_end {
            Some(Some(t)) => Some(t),
            Some(None) => None,
            None => current.quiet_hours_end,
        };

        let timezone = match update.timezone {
            Some(Some(t)) => Some(t),
            Some(None) => None,
            None => current.timezone,
        };

        let row = sqlx::query(
            r#"
            INSERT INTO notifications.user_settings (
                user_id, email_enabled, push_enabled, in_app_enabled, sms_enabled,
                quiet_hours_start, quiet_hours_end, timezone
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (user_id) DO UPDATE SET
                email_enabled = $2,
                push_enabled = $3,
                in_app_enabled = $4,
                sms_enabled = $5,
                quiet_hours_start = $6,
                quiet_hours_end = $7,
                timezone = $8,
                updated_at = NOW()
            RETURNING user_id, email_enabled, push_enabled, in_app_enabled, sms_enabled,
                      quiet_hours_start, quiet_hours_end, timezone, updated_at
            "#,
        )
        .bind(user_id)
        .bind(update.email_enabled.unwrap_or(current.email_enabled))
        .bind(update.push_enabled.unwrap_or(current.push_enabled))
        .bind(update.in_app_enabled.unwrap_or(current.in_app_enabled))
        .bind(update.sms_enabled.unwrap_or(current.sms_enabled))
        .bind(quiet_start)
        .bind(quiet_end)
        .bind(&timezone)
        .fetch_one(&self.pool)
        .await?;

        Ok(self.map_user_settings_row(&row))
    }

    /// Deletes user settings.
    pub async fn delete_user_settings(&self, user_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM notifications.user_settings WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(format!(
                "User settings for {}",
                user_id
            )));
        }

        Ok(())
    }

    fn map_user_settings_row(&self, row: &sqlx::postgres::PgRow) -> UserSettings {
        UserSettings {
            user_id: row.get("user_id"),
            email_enabled: row.get("email_enabled"),
            push_enabled: row.get("push_enabled"),
            in_app_enabled: row.get("in_app_enabled"),
            sms_enabled: row.get("sms_enabled"),
            quiet_hours_start: row.get("quiet_hours_start"),
            quiet_hours_end: row.get("quiet_hours_end"),
            timezone: row.get("timezone"),
            updated_at: row.get("updated_at"),
        }
    }
}
