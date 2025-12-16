//! # Analytics API Handlers
//!
//! HTTP request handlers for analytics endpoints.

use actix_web::{web, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::{
    AnalyticsQueryParams, BatchTrackResponse, CourseAnalyticsResponse, ErrorResponse,
    EventCountResponse, EventQueryParams, EventResponse, PageStatsResponse, PaginatedResponse,
    PaginationMeta, PlatformStatsResponse, SessionResponse, StartSessionRequest, SuccessResponse,
    TimeSeriesPointResponse, TrackBatchRequest, TrackEventRequest, UserEngagementResponse,
    parse_event_type, parse_platform, CourseStatsResponse,
};
use crate::domain::{EventType, NewEvent, NewSession, Platform};
use crate::domain::value_objects::{DateRange, Pagination};
use crate::service::AnalyticsService;

/// Application state.
pub struct AppState {
    pub analytics_service: Arc<AnalyticsService>,
}

// =============================================================================
// EVENT HANDLERS
// =============================================================================

/// POST /api/v1/events - Track single event.
pub async fn track_event(
    state: web::Data<AppState>,
    body: web::Json<TrackEventRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            &e.to_string(),
        ));
    }

    let req = body.into_inner();

    // Build new event
    let new_event = NewEvent {
        event_type: parse_event_type(&req.event_type),
        user_id: req.user_id,
        session_id: req.session_id,
        tenant_id: req.tenant_id,
        page_url: req.page_url,
        page_title: req.page_title,
        referrer: req.referrer,
        platform: req.platform.as_deref().map(parse_platform),
        device_info: req.device_info.map(Into::into),
        geo_info: req.geo_info.map(Into::into),
        properties: req.properties,
        duration_ms: req.duration_ms,
        entity_type: req.entity_type,
        entity_id: req.entity_id,
    };

    match state.analytics_service.track_event(new_event).await {
        Ok(tracked_event) => {
            HttpResponse::Created().json(SuccessResponse::new(EventResponse::from(tracked_event)))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "TRACK_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// POST /api/v1/events/batch - Track multiple events.
pub async fn track_events_batch(
    state: web::Data<AppState>,
    body: web::Json<TrackBatchRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            &e.to_string(),
        ));
    }

    let req = body.into_inner();

    // Build new events
    let new_events: Vec<NewEvent> = req.events
        .into_iter()
        .map(|e| NewEvent {
            event_type: parse_event_type(&e.event_type),
            user_id: e.user_id,
            session_id: e.session_id,
            tenant_id: e.tenant_id,
            page_url: e.page_url,
            page_title: e.page_title,
            referrer: e.referrer,
            platform: e.platform.as_deref().map(parse_platform),
            device_info: e.device_info.map(Into::into),
            geo_info: e.geo_info.map(Into::into),
            properties: e.properties,
            duration_ms: e.duration_ms,
            entity_type: e.entity_type,
            entity_id: e.entity_id,
        })
        .collect();

    let count = new_events.len() as i32;

    match state.analytics_service.track_events_batch(new_events).await {
        Ok(events) => {
            let event_ids: Vec<Uuid> = events.iter().map(|e| e.event_id).collect();
            HttpResponse::Created().json(SuccessResponse::new(BatchTrackResponse {
                tracked_count: count,
                event_ids,
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "BATCH_TRACK_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/events/{event_id} - Get event by ID.
pub async fn get_event(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let event_id = path.into_inner();

    match state.analytics_service.get_event(event_id).await {
        Ok(Some(event)) => {
            HttpResponse::Ok().json(SuccessResponse::new(EventResponse::from(event)))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "NOT_FOUND",
                "Event not found",
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "GET_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/events - Query events.
pub async fn query_events(
    state: web::Data<AppState>,
    query: web::Query<EventQueryParams>,
) -> HttpResponse {
    let date_range = DateRange::new(query.date_from, query.date_to);
    let pagination = Pagination::new(
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    );

    // Parse event types
    let event_types: Option<Vec<EventType>> = query.event_types.as_ref().map(|s| {
        s.split(',')
            .map(|t| parse_event_type(t.trim()))
            .collect()
    });

    match state.analytics_service.query_events(
        &date_range,
        event_types.as_deref(),
        query.user_id,
        query.tenant_id,
        &pagination,
    ).await {
        Ok(events) => {
            let response_events: Vec<EventResponse> = events
                .into_iter()
                .map(EventResponse::from)
                .collect();

            HttpResponse::Ok().json(PaginatedResponse {
                success: true,
                data: response_events,
                pagination: PaginationMeta {
                    limit: pagination.limit,
                    offset: pagination.offset,
                    total: None,
                },
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "QUERY_ERROR",
                &e.to_string(),
            ))
        }
    }
}

// =============================================================================
// SESSION HANDLERS
// =============================================================================

/// POST /api/v1/sessions - Start a session.
pub async fn start_session(
    state: web::Data<AppState>,
    body: web::Json<StartSessionRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            &e.to_string(),
        ));
    }

    let req = body.into_inner();

    let new_session = NewSession {
        user_id: req.user_id,
        tenant_id: req.tenant_id,
        platform: req.platform.as_deref().map(parse_platform),
        device_info: req.device_info.map(Into::into),
        geo_info: req.geo_info.map(Into::into),
        entry_page: req.entry_page,
    };

    match state.analytics_service.start_session(new_session).await {
        Ok(session) => {
            HttpResponse::Created().json(SuccessResponse::new(SessionResponse::from(session)))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "SESSION_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/sessions/{session_id} - Get session by ID.
pub async fn get_session(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let session_id = path.into_inner();

    match state.analytics_service.get_session(session_id).await {
        Ok(Some(session)) => {
            HttpResponse::Ok().json(SuccessResponse::new(SessionResponse::from(session)))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "NOT_FOUND",
                "Session not found",
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "GET_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// PUT /api/v1/sessions/{session_id}/end - End a session.
pub async fn end_session(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let session_id = path.into_inner();

    match state.analytics_service.end_session(session_id).await {
        Ok(Some(session)) => {
            HttpResponse::Ok().json(SuccessResponse::new(SessionResponse::from(session)))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "NOT_FOUND",
                "Session not found",
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "END_SESSION_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/sessions/active/count - Get active sessions count.
pub async fn get_active_sessions_count(
    state: web::Data<AppState>,
) -> HttpResponse {
    match state.analytics_service.get_active_sessions_count().await {
        Ok(count) => {
            HttpResponse::Ok().json(SuccessResponse::new(serde_json::json!({
                "active_sessions": count
            })))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "COUNT_ERROR",
                &e.to_string(),
            ))
        }
    }
}

// =============================================================================
// ANALYTICS HANDLERS
// =============================================================================

/// GET /api/v1/analytics/events/counts - Get event counts by type.
pub async fn get_event_counts(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let date_range = query.to_date_range();

    match state.analytics_service.get_event_counts(&date_range).await {
        Ok(counts) => {
            let response: Vec<EventCountResponse> = counts
                .into_iter()
                .map(EventCountResponse::from)
                .collect();

            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "COUNT_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/events/timeseries - Get event time series.
pub async fn get_event_time_series(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let date_range = query.to_date_range();
    let granularity = query.to_granularity();
    let event_type = query.event_type.as_deref().map(parse_event_type);

    match state.analytics_service.get_event_time_series(
        &date_range,
        granularity,
        event_type.as_ref(),
    ).await {
        Ok(series) => {
            let response: Vec<TimeSeriesPointResponse> = series
                .into_iter()
                .map(TimeSeriesPointResponse::from)
                .collect();

            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "TIMESERIES_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/platform - Get platform stats.
pub async fn get_platform_stats(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let date_range = query.to_date_range();

    match state.analytics_service.get_platform_stats(&date_range).await {
        Ok(stats) => {
            HttpResponse::Ok().json(SuccessResponse::new(PlatformStatsResponse::from(stats)))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "PLATFORM_STATS_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/courses/{course_id} - Get course analytics.
pub async fn get_course_analytics(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let course_id = path.into_inner();
    let date_range = query.to_date_range();

    match state.analytics_service.get_course_analytics(course_id, &date_range).await {
        Ok(analytics) => {
            HttpResponse::Ok().json(SuccessResponse::new(CourseAnalyticsResponse::from(analytics)))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "COURSE_ANALYTICS_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/users/{user_id} - Get user engagement.
pub async fn get_user_engagement(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let date_range = query.to_date_range();

    match state.analytics_service.get_user_engagement(user_id, &date_range).await {
        Ok(engagement) => {
            HttpResponse::Ok().json(SuccessResponse::new(UserEngagementResponse::from(engagement)))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "USER_ENGAGEMENT_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/pages/top - Get top pages.
pub async fn get_top_pages(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let date_range = query.to_date_range();
    let limit = query.limit.unwrap_or(10);

    match state.analytics_service.get_top_pages(&date_range, limit).await {
        Ok(pages) => {
            let response: Vec<PageStatsResponse> = pages
                .into_iter()
                .map(PageStatsResponse::from)
                .collect();

            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "TOP_PAGES_ERROR",
                &e.to_string(),
            ))
        }
    }
}

/// GET /api/v1/analytics/courses/top - Get top courses.
pub async fn get_top_courses(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQueryParams>,
) -> HttpResponse {
    let date_range = query.to_date_range();
    let limit = query.limit.unwrap_or(10);

    match state.analytics_service.get_top_courses(&date_range, limit).await {
        Ok(courses) => {
            let response: Vec<CourseStatsResponse> = courses
                .into_iter()
                .map(CourseStatsResponse::from)
                .collect();

            HttpResponse::Ok().json(SuccessResponse::new(response))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "TOP_COURSES_ERROR",
                &e.to_string(),
            ))
        }
    }
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// GET /health - Health check.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "analytics-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// GET /ready - Readiness check.
pub async fn readiness_check(
    state: web::Data<AppState>,
) -> HttpResponse {
    // Check if service is ready by performing a simple operation
    match state.analytics_service.get_active_sessions_count().await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "ready",
                "service": "analytics-service",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Err(_) => {
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "not_ready",
                "service": "analytics-service",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
    }
}
