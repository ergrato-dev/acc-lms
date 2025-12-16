//! # API Handlers
//!
//! HTTP request handlers for the notifications service.

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::{
    CreateNotificationRequest, CreateTemplateRequest, ErrorResponse, ListQuery, MessageResponse,
    NotificationListResponse, NotificationResponse, NotificationStatsResponse, SendNotificationRequest,
    SuccessResponse, TemplateListQuery, TemplateResponse, UnreadCountResponse, UpdateTemplateRequest,
    UpdateUserSettingsRequest, UserSettingsResponse,
};
use crate::domain::{
    NewNotification, NewTemplate, NewUserSettings, UpdateTemplate, UpdateUserSettings,
};
use crate::service::{NotificationError, NotificationService};

/// Application state containing the service.
pub struct AppState {
    pub service: NotificationService,
}

// =============================================================================
// ERROR HANDLING
// =============================================================================

/// Converts NotificationError to HttpResponse.
fn error_response(err: NotificationError) -> HttpResponse {
    match err {
        NotificationError::TemplateNotFound(msg) => {
            HttpResponse::NotFound().json(ErrorResponse::new("not_found", &msg))
        }
        NotificationError::UserSettingsNotFound(id) => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "not_found",
                &format!("User settings not found for user: {}", id),
            ))
        }
        NotificationError::NotificationTypeDisabled(t) => {
            HttpResponse::BadRequest().json(ErrorResponse::new(
                "notification_disabled",
                &format!("Notification type {} is disabled for this user", t),
            ))
        }
        NotificationError::InvalidTemplate(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse::new("invalid_template", &msg))
        }
        NotificationError::QuietHoursActive(id) => {
            HttpResponse::BadRequest().json(ErrorResponse::new(
                "quiet_hours",
                &format!("Quiet hours active for user: {}", id),
            ))
        }
        NotificationError::MaxRetriesExceeded(id) => {
            HttpResponse::BadRequest().json(ErrorResponse::new(
                "max_retries",
                &format!("Max retries exceeded for notification: {}", id),
            ))
        }
        NotificationError::Validation(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse::new("validation_error", &msg))
        }
        NotificationError::Repository(e) => {
            tracing::error!("Repository error: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::new("internal_error", "An internal error occurred"))
        }
    }
}

// =============================================================================
// TEMPLATE HANDLERS
// =============================================================================

/// Creates a new template.
pub async fn create_template(
    state: web::Data<AppState>,
    body: web::Json<CreateTemplateRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::with_details(
            "validation_error",
            "Invalid request body",
            serde_json::to_value(e).unwrap_or_default(),
        ));
    }

    let new_template = NewTemplate {
        name: body.name.clone(),
        notification_type: body.notification_type,
        subject_template: body.subject_template.clone(),
        body_template: body.body_template.clone(),
        variables: body.variables.clone(),
    };

    match state.service.create_template(new_template).await {
        Ok((template, _event)) => {
            HttpResponse::Created().json(SuccessResponse::new(TemplateResponse::from(template)))
        }
        Err(e) => error_response(e),
    }
}

/// Gets a template by ID.
pub async fn get_template(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let template_id = path.into_inner();

    match state.service.get_template(template_id).await {
        Ok(template) => HttpResponse::Ok().json(SuccessResponse::new(TemplateResponse::from(template))),
        Err(e) => error_response(e),
    }
}

/// Lists all templates.
pub async fn list_templates(
    state: web::Data<AppState>,
    query: web::Query<TemplateListQuery>,
) -> HttpResponse {
    let include_inactive = query.include_inactive.unwrap_or(false);

    let result = if let Some(notification_type) = query.notification_type {
        state.service.list_templates_by_type(notification_type).await
    } else {
        state.service.list_templates(include_inactive).await
    };

    match result {
        Ok(templates) => {
            let response: Vec<TemplateResponse> = templates.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => error_response(e),
    }
}

/// Updates a template.
pub async fn update_template(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateTemplateRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::with_details(
            "validation_error",
            "Invalid request body",
            serde_json::to_value(e).unwrap_or_default(),
        ));
    }

    let template_id = path.into_inner();
    let update = UpdateTemplate {
        name: body.name.clone(),
        notification_type: body.notification_type,
        subject_template: body.subject_template.clone(),
        body_template: body.body_template.clone(),
        variables: body.variables.clone(),
        is_active: body.is_active,
    };

    match state.service.update_template(template_id, update).await {
        Ok((template, _event)) => {
            HttpResponse::Ok().json(SuccessResponse::new(TemplateResponse::from(template)))
        }
        Err(e) => error_response(e),
    }
}

/// Deactivates a template.
pub async fn deactivate_template(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let template_id = path.into_inner();

    match state.service.deactivate_template(template_id).await {
        Ok(_event) => HttpResponse::Ok().json(MessageResponse::new("Template deactivated successfully")),
        Err(e) => error_response(e),
    }
}

/// Deletes a template.
pub async fn delete_template(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let template_id = path.into_inner();

    match state.service.delete_template(template_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// NOTIFICATION HANDLERS
// =============================================================================

/// Sends a notification using a template.
pub async fn send_notification(
    state: web::Data<AppState>,
    body: web::Json<SendNotificationRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::with_details(
            "validation_error",
            "Invalid request body",
            serde_json::to_value(e).unwrap_or_default(),
        ));
    }

    let request = crate::domain::SendNotificationRequest {
        user_id: body.user_id,
        template_name: body.template_name.clone(),
        variables: body.variables.clone(),
        priority: body.priority,
        scheduled_for: body.scheduled_for,
    };

    match state.service.send_notification(request).await {
        Ok((notification, _event)) => {
            HttpResponse::Created().json(SuccessResponse::new(NotificationResponse::from(notification)))
        }
        Err(e) => error_response(e),
    }
}

/// Creates a notification directly.
pub async fn create_notification(
    state: web::Data<AppState>,
    body: web::Json<CreateNotificationRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::with_details(
            "validation_error",
            "Invalid request body",
            serde_json::to_value(e).unwrap_or_default(),
        ));
    }

    let new_notification = NewNotification {
        user_id: body.user_id,
        template_id: body.template_id,
        notification_type: body.notification_type,
        subject: body.subject.clone(),
        content: body.content.clone(),
        priority: body.priority,
        scheduled_for: body.scheduled_for,
        metadata: body.metadata.clone(),
    };

    match state.service.create_notification(new_notification).await {
        Ok((notification, _event)) => {
            HttpResponse::Created().json(SuccessResponse::new(NotificationResponse::from(notification)))
        }
        Err(e) => error_response(e),
    }
}

/// Gets a notification by ID.
pub async fn get_notification(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let notification_id = path.into_inner();

    match state.service.get_notification(notification_id).await {
        Ok(notification) => {
            HttpResponse::Ok().json(SuccessResponse::new(NotificationResponse::from(notification)))
        }
        Err(e) => error_response(e),
    }
}

/// Lists notifications for a user.
pub async fn list_user_notifications(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<ListQuery>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match state
        .service
        .list_user_notifications(user_id, query.limit(), query.offset())
        .await
    {
        Ok(notifications) => {
            let total = notifications.len();
            let response = NotificationListResponse {
                notifications: notifications.into_iter().map(Into::into).collect(),
                total,
            };
            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => error_response(e),
    }
}

/// Marks a notification as read.
pub async fn mark_as_read(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let notification_id = path.into_inner();

    match state.service.mark_as_read(notification_id).await {
        Ok((notification, _event)) => {
            HttpResponse::Ok().json(SuccessResponse::new(NotificationResponse::from(notification)))
        }
        Err(e) => error_response(e),
    }
}

/// Gets unread count for a user.
pub async fn get_unread_count(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match state.service.count_unread(user_id).await {
        Ok(count) => HttpResponse::Ok().json(SuccessResponse::new(UnreadCountResponse {
            unread_count: count,
        })),
        Err(e) => error_response(e),
    }
}

/// Gets notification statistics.
pub async fn get_stats(
    state: web::Data<AppState>,
    query: web::Query<UserIdQuery>,
) -> HttpResponse {
    match state.service.get_stats(query.user_id).await {
        Ok(stats) => HttpResponse::Ok().json(SuccessResponse::new(NotificationStatsResponse::from(stats))),
        Err(e) => error_response(e),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UserIdQuery {
    pub user_id: Option<Uuid>,
}

// =============================================================================
// USER SETTINGS HANDLERS
// =============================================================================

/// Gets user notification settings.
pub async fn get_user_settings(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match state.service.get_user_settings(user_id).await {
        Ok(settings) => {
            HttpResponse::Ok().json(SuccessResponse::new(UserSettingsResponse::from(settings)))
        }
        Err(e) => error_response(e),
    }
}

/// Updates user notification settings.
pub async fn update_user_settings(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserSettingsRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::with_details(
            "validation_error",
            "Invalid request body",
            serde_json::to_value(e).unwrap_or_default(),
        ));
    }

    let user_id = path.into_inner();

    // Parse quiet hours from strings
    let quiet_start = body.quiet_hours_start.clone().map(|opt| {
        opt.and_then(|s| chrono::NaiveTime::parse_from_str(&s, "%H:%M").ok())
    });
    let quiet_end = body.quiet_hours_end.clone().map(|opt| {
        opt.and_then(|s| chrono::NaiveTime::parse_from_str(&s, "%H:%M").ok())
    });

    let update = UpdateUserSettings {
        email_enabled: body.email_enabled,
        push_enabled: body.push_enabled,
        in_app_enabled: body.in_app_enabled,
        sms_enabled: body.sms_enabled,
        quiet_hours_start: quiet_start,
        quiet_hours_end: quiet_end,
        timezone: body.timezone.clone(),
    };

    match state.service.update_user_settings(user_id, update).await {
        Ok((settings, _event)) => {
            HttpResponse::Ok().json(SuccessResponse::new(UserSettingsResponse::from(settings)))
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "notifications-service"
    }))
}
