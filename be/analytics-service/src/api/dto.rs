//! # Analytics API DTOs
//!
//! Request and response data transfer objects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    CourseAnalytics, CourseStats, DeviceInfo, Event, EventCount, EventType, GeoInfo,
    PageStats, Platform, PlatformStats, Session, TimeSeriesPoint, UserEngagement,
};
use crate::domain::value_objects::TimeGranularity;

// =============================================================================
// COMMON RESPONSE TYPES
// =============================================================================

/// Standard success response wrapper.
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

/// Standard error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            success: false,
            error: ErrorDetail {
                code: code.to_string(),
                message: message.to_string(),
            },
        }
    }
}

/// Paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub limit: i64,
    pub offset: i64,
    pub total: Option<i64>,
}

// =============================================================================
// EVENT REQUESTS
// =============================================================================

/// Request to track a single event.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TrackEventRequest {
    #[validate(length(min = 1, max = 100))]
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    #[validate(length(max = 2000))]
    pub page_url: Option<String>,
    #[validate(length(max = 500))]
    pub page_title: Option<String>,
    #[validate(length(max = 2000))]
    pub referrer: Option<String>,
    pub platform: Option<String>,
    pub device_info: Option<DeviceInfoDto>,
    pub geo_info: Option<GeoInfoDto>,
    pub properties: Option<HashMap<String, serde_json::Value>>,
    pub duration_ms: Option<i64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

/// Request to track multiple events.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TrackBatchRequest {
    #[validate(length(min = 1, max = 100))]
    pub events: Vec<TrackEventRequest>,
}

/// Device information DTO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceInfoDto {
    pub user_agent: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub device_type: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
}

impl From<DeviceInfoDto> for DeviceInfo {
    fn from(dto: DeviceInfoDto) -> Self {
        DeviceInfo {
            user_agent: dto.user_agent,
            browser: dto.browser,
            browser_version: dto.browser_version,
            os: dto.os,
            os_version: dto.os_version,
            device_type: dto.device_type,
            screen_width: dto.screen_width,
            screen_height: dto.screen_height,
        }
    }
}

/// Geographic information DTO.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeoInfoDto {
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub timezone: Option<String>,
}

impl From<GeoInfoDto> for GeoInfo {
    fn from(dto: GeoInfoDto) -> Self {
        GeoInfo {
            ip_address: dto.ip_address,
            country: dto.country,
            country_code: dto.country_code,
            region: dto.region,
            city: dto.city,
            timezone: dto.timezone,
        }
    }
}

// =============================================================================
// EVENT RESPONSES
// =============================================================================

/// Event response.
#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub event_id: Uuid,
    pub event_type: String,
    pub user_id: Option<Uuid>,
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub page_url: Option<String>,
    pub platform: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            event_id: event.event_id,
            event_type: event.event_type.to_string(),
            user_id: event.user_id,
            session_id: event.session_id,
            timestamp: event.timestamp,
            page_url: event.page_url,
            platform: event.platform.to_string(),
            properties: event.properties,
            entity_type: event.entity_type,
            entity_id: event.entity_id,
        }
    }
}

/// Batch track response.
#[derive(Debug, Serialize)]
pub struct BatchTrackResponse {
    pub tracked_count: i32,
    pub event_ids: Vec<Uuid>,
}

// =============================================================================
// SESSION REQUESTS
// =============================================================================

/// Request to start a session.
#[derive(Debug, Deserialize, Validate)]
pub struct StartSessionRequest {
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub platform: Option<String>,
    pub device_info: Option<DeviceInfoDto>,
    pub geo_info: Option<GeoInfoDto>,
    #[validate(length(max = 2000))]
    pub entry_page: Option<String>,
}

// =============================================================================
// SESSION RESPONSES
// =============================================================================

/// Session response.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub platform: String,
    pub entry_page: Option<String>,
    pub exit_page: Option<String>,
    pub page_views: i32,
    pub events_count: i32,
    pub is_active: bool,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            session_id: session.session_id,
            user_id: session.user_id,
            started_at: session.started_at,
            ended_at: session.ended_at,
            duration_seconds: session.duration_seconds,
            platform: session.platform.to_string(),
            entry_page: session.entry_page,
            exit_page: session.exit_page,
            page_views: session.page_views,
            events_count: session.events_count,
            is_active: session.is_active,
        }
    }
}

// =============================================================================
// QUERY REQUESTS
// =============================================================================

/// Query parameters for events.
#[derive(Debug, Deserialize)]
pub struct EventQueryParams {
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub event_types: Option<String>, // Comma-separated
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub platform: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for analytics.
#[derive(Debug, Deserialize)]
pub struct AnalyticsQueryParams {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub days: Option<i64>,
    pub granularity: Option<String>,
    pub event_type: Option<String>,
    pub limit: Option<i64>,
}

impl AnalyticsQueryParams {
    /// Converts to a date range.
    pub fn to_date_range(&self) -> crate::domain::value_objects::DateRange {
        if let (Some(from), Some(to)) = (self.date_from, self.date_to) {
            crate::domain::value_objects::DateRange::new(from, to)
        } else if let Some(days) = self.days {
            crate::domain::value_objects::DateRange::last_days(days)
        } else {
            crate::domain::value_objects::DateRange::last_days(30)
        }
    }

    /// Converts to time granularity.
    pub fn to_granularity(&self) -> TimeGranularity {
        match self.granularity.as_deref() {
            Some("minute") => TimeGranularity::Minute,
            Some("hour") => TimeGranularity::Hour,
            Some("day") => TimeGranularity::Day,
            Some("week") => TimeGranularity::Week,
            Some("month") => TimeGranularity::Month,
            Some("year") => TimeGranularity::Year,
            _ => TimeGranularity::Day,
        }
    }
}

// =============================================================================
// ANALYTICS RESPONSES
// =============================================================================

/// Event count response.
#[derive(Debug, Serialize)]
pub struct EventCountResponse {
    pub event_type: String,
    pub count: i64,
}

impl From<EventCount> for EventCountResponse {
    fn from(ec: EventCount) -> Self {
        Self {
            event_type: ec.event_type,
            count: ec.count,
        }
    }
}

/// Time series point response.
#[derive(Debug, Serialize)]
pub struct TimeSeriesPointResponse {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub label: Option<String>,
}

impl From<TimeSeriesPoint> for TimeSeriesPointResponse {
    fn from(point: TimeSeriesPoint) -> Self {
        Self {
            timestamp: point.timestamp,
            value: point.value,
            label: point.label,
        }
    }
}

/// Platform stats response.
#[derive(Debug, Serialize)]
pub struct PlatformStatsResponse {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_users: i64,
    pub active_users: i64,
    pub new_users: i64,
    pub total_sessions: i64,
    pub total_page_views: i64,
    pub average_session_duration_seconds: f64,
    pub bounce_rate: f64,
    pub top_pages: Vec<PageStatsResponse>,
    pub top_courses: Vec<CourseStatsResponse>,
    pub platform_breakdown: HashMap<String, i64>,
}

impl From<PlatformStats> for PlatformStatsResponse {
    fn from(stats: PlatformStats) -> Self {
        Self {
            period_start: stats.period_start,
            period_end: stats.period_end,
            total_users: stats.total_users,
            active_users: stats.active_users,
            new_users: stats.new_users,
            total_sessions: stats.total_sessions,
            total_page_views: stats.total_page_views,
            average_session_duration_seconds: stats.average_session_duration_seconds,
            bounce_rate: stats.bounce_rate,
            top_pages: stats.top_pages.into_iter().map(Into::into).collect(),
            top_courses: stats.top_courses.into_iter().map(Into::into).collect(),
            platform_breakdown: stats.platform_breakdown,
        }
    }
}

/// Page stats response.
#[derive(Debug, Serialize)]
pub struct PageStatsResponse {
    pub page_url: String,
    pub page_title: Option<String>,
    pub views: i64,
    pub unique_visitors: i64,
    pub average_time_seconds: f64,
}

impl From<PageStats> for PageStatsResponse {
    fn from(stats: PageStats) -> Self {
        Self {
            page_url: stats.page_url,
            page_title: stats.page_title,
            views: stats.views,
            unique_visitors: stats.unique_visitors,
            average_time_seconds: stats.average_time_seconds,
        }
    }
}

/// Course stats response.
#[derive(Debug, Serialize)]
pub struct CourseStatsResponse {
    pub course_id: Uuid,
    pub course_name: String,
    pub enrollments: i64,
    pub completions: i64,
    pub active_students: i64,
}

impl From<CourseStats> for CourseStatsResponse {
    fn from(stats: CourseStats) -> Self {
        Self {
            course_id: stats.course_id,
            course_name: stats.course_name,
            enrollments: stats.enrollments,
            completions: stats.completions,
            active_students: stats.active_students,
        }
    }
}

/// Course analytics response.
#[derive(Debug, Serialize)]
pub struct CourseAnalyticsResponse {
    pub course_id: Uuid,
    pub total_enrollments: i64,
    pub active_students: i64,
    pub completion_rate: f64,
    pub average_progress: f64,
    pub average_score: Option<f64>,
    pub total_time_spent_minutes: i64,
}

impl From<CourseAnalytics> for CourseAnalyticsResponse {
    fn from(analytics: CourseAnalytics) -> Self {
        Self {
            course_id: analytics.course_id,
            total_enrollments: analytics.total_enrollments,
            active_students: analytics.active_students,
            completion_rate: analytics.completion_rate,
            average_progress: analytics.average_progress,
            average_score: analytics.average_score,
            total_time_spent_minutes: analytics.total_time_spent_minutes,
        }
    }
}

/// User engagement response.
#[derive(Debug, Serialize)]
pub struct UserEngagementResponse {
    pub user_id: Uuid,
    pub total_sessions: i64,
    pub total_time_spent_minutes: i64,
    pub courses_enrolled: i64,
    pub courses_completed: i64,
    pub lessons_completed: i64,
    pub quizzes_completed: i64,
    pub average_quiz_score: Option<f64>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub streak_days: i32,
}

impl From<UserEngagement> for UserEngagementResponse {
    fn from(engagement: UserEngagement) -> Self {
        Self {
            user_id: engagement.user_id,
            total_sessions: engagement.total_sessions,
            total_time_spent_minutes: engagement.total_time_spent_minutes,
            courses_enrolled: engagement.courses_enrolled,
            courses_completed: engagement.courses_completed,
            lessons_completed: engagement.lessons_completed,
            quizzes_completed: engagement.quizzes_completed,
            average_quiz_score: engagement.average_quiz_score,
            last_active_at: engagement.last_active_at,
            streak_days: engagement.streak_days,
        }
    }
}

// =============================================================================
// HELPERS
// =============================================================================

/// Parses event type from string.
pub fn parse_event_type(s: &str) -> EventType {
    s.parse().unwrap_or(EventType::Custom(s.to_string()))
}

/// Parses platform from string.
pub fn parse_platform(s: &str) -> Platform {
    s.parse().unwrap_or(Platform::Unknown)
}
