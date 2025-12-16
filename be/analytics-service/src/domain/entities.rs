//! # Analytics Domain Entities
//!
//! Core entities for the analytics system.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Event (user actions and system events)
//! Session (user sessions)
//! Metric (aggregated metrics)
//! Report (generated reports)
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Type of tracked event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Page or screen view
    PageView,
    /// User click action
    Click,
    /// Form submission
    FormSubmit,
    /// Video playback events
    VideoPlay,
    /// Video pause
    VideoPause,
    /// Video completion
    VideoComplete,
    /// Course enrollment
    CourseEnroll,
    /// Course completion
    CourseComplete,
    /// Lesson started
    LessonStart,
    /// Lesson completed
    LessonComplete,
    /// Quiz started
    QuizStart,
    /// Quiz completed
    QuizComplete,
    /// Assignment submitted
    AssignmentSubmit,
    /// User login
    Login,
    /// User logout
    Logout,
    /// Search performed
    Search,
    /// Download initiated
    Download,
    /// Error occurred
    Error,
    /// Custom event
    Custom(String),
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::PageView => write!(f, "page_view"),
            EventType::Click => write!(f, "click"),
            EventType::FormSubmit => write!(f, "form_submit"),
            EventType::VideoPlay => write!(f, "video_play"),
            EventType::VideoPause => write!(f, "video_pause"),
            EventType::VideoComplete => write!(f, "video_complete"),
            EventType::CourseEnroll => write!(f, "course_enroll"),
            EventType::CourseComplete => write!(f, "course_complete"),
            EventType::LessonStart => write!(f, "lesson_start"),
            EventType::LessonComplete => write!(f, "lesson_complete"),
            EventType::QuizStart => write!(f, "quiz_start"),
            EventType::QuizComplete => write!(f, "quiz_complete"),
            EventType::AssignmentSubmit => write!(f, "assignment_submit"),
            EventType::Login => write!(f, "login"),
            EventType::Logout => write!(f, "logout"),
            EventType::Search => write!(f, "search"),
            EventType::Download => write!(f, "download"),
            EventType::Error => write!(f, "error"),
            EventType::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

impl std::str::FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "page_view" => Ok(EventType::PageView),
            "click" => Ok(EventType::Click),
            "form_submit" => Ok(EventType::FormSubmit),
            "video_play" => Ok(EventType::VideoPlay),
            "video_pause" => Ok(EventType::VideoPause),
            "video_complete" => Ok(EventType::VideoComplete),
            "course_enroll" => Ok(EventType::CourseEnroll),
            "course_complete" => Ok(EventType::CourseComplete),
            "lesson_start" => Ok(EventType::LessonStart),
            "lesson_complete" => Ok(EventType::LessonComplete),
            "quiz_start" => Ok(EventType::QuizStart),
            "quiz_complete" => Ok(EventType::QuizComplete),
            "assignment_submit" => Ok(EventType::AssignmentSubmit),
            "login" => Ok(EventType::Login),
            "logout" => Ok(EventType::Logout),
            "search" => Ok(EventType::Search),
            "download" => Ok(EventType::Download),
            "error" => Ok(EventType::Error),
            s if s.starts_with("custom:") => {
                Ok(EventType::Custom(s.trim_start_matches("custom:").to_string()))
            }
            _ => Err(format!("Invalid event type: {}", s)),
        }
    }
}

/// Metric aggregation period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationPeriod {
    /// Hourly aggregation
    Hourly,
    /// Daily aggregation
    Daily,
    /// Weekly aggregation
    Weekly,
    /// Monthly aggregation
    Monthly,
}

impl std::fmt::Display for AggregationPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationPeriod::Hourly => write!(f, "hourly"),
            AggregationPeriod::Daily => write!(f, "daily"),
            AggregationPeriod::Weekly => write!(f, "weekly"),
            AggregationPeriod::Monthly => write!(f, "monthly"),
        }
    }
}

/// Platform/device type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Web,
    Android,
    #[serde(rename = "ios")]
    iOS,
    Desktop,
    #[serde(rename = "api")]
    API,
    Unknown,
}

impl Default for Platform {
    fn default() -> Self {
        Platform::Unknown
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Web => write!(f, "web"),
            Platform::Android => write!(f, "android"),
            Platform::iOS => write!(f, "ios"),
            Platform::Desktop => write!(f, "desktop"),
            Platform::API => write!(f, "api"),
            Platform::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(Platform::Web),
            "android" => Ok(Platform::Android),
            "ios" => Ok(Platform::iOS),
            "desktop" => Ok(Platform::Desktop),
            "api" => Ok(Platform::API),
            _ => Ok(Platform::Unknown),
        }
    }
}

// =============================================================================
// EVENT
// =============================================================================

/// Tracked analytics event.
///
/// # Database Mapping
///
/// Maps to ClickHouse `analytics.events` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event identifier
    pub event_id: Uuid,
    /// Event type
    pub event_type: EventType,
    /// User who triggered the event (optional for anonymous)
    pub user_id: Option<Uuid>,
    /// Session identifier
    pub session_id: Uuid,
    /// Tenant/organization identifier
    pub tenant_id: Option<Uuid>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Page or screen URL/path
    pub page_url: Option<String>,
    /// Page title
    pub page_title: Option<String>,
    /// Referrer URL
    pub referrer: Option<String>,
    /// Platform
    pub platform: Platform,
    /// Device information
    pub device_info: Option<DeviceInfo>,
    /// Geographic location
    pub geo_info: Option<GeoInfo>,
    /// Event-specific properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Duration in milliseconds (for timed events)
    pub duration_ms: Option<i64>,
    /// Associated entity type (course, lesson, etc.)
    pub entity_type: Option<String>,
    /// Associated entity ID
    pub entity_id: Option<Uuid>,
}

/// Device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub user_agent: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub device_type: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
}

/// Geographic information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoInfo {
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub timezone: Option<String>,
}

/// Data for creating a new event.
#[derive(Debug, Clone, Deserialize)]
pub struct NewEvent {
    pub event_type: EventType,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub page_url: Option<String>,
    pub page_title: Option<String>,
    pub referrer: Option<String>,
    pub platform: Option<Platform>,
    pub device_info: Option<DeviceInfo>,
    pub geo_info: Option<GeoInfo>,
    pub properties: Option<HashMap<String, serde_json::Value>>,
    pub duration_ms: Option<i64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

// =============================================================================
// SESSION
// =============================================================================

/// User session.
///
/// # Database Mapping
///
/// Maps to ClickHouse `analytics.sessions` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session identifier
    pub session_id: Uuid,
    /// User ID (optional for anonymous)
    pub user_id: Option<Uuid>,
    /// Tenant/organization
    pub tenant_id: Option<Uuid>,
    /// Session start time
    pub started_at: DateTime<Utc>,
    /// Session end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Session duration in seconds
    pub duration_seconds: Option<i64>,
    /// Platform
    pub platform: Platform,
    /// Device info
    pub device_info: Option<DeviceInfo>,
    /// Geo info
    pub geo_info: Option<GeoInfo>,
    /// Entry page
    pub entry_page: Option<String>,
    /// Exit page
    pub exit_page: Option<String>,
    /// Page views count
    pub page_views: i32,
    /// Total events count
    pub events_count: i32,
    /// Is session still active
    pub is_active: bool,
}

/// Data for creating a new session.
#[derive(Debug, Clone, Deserialize)]
pub struct NewSession {
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub platform: Option<Platform>,
    pub device_info: Option<DeviceInfo>,
    pub geo_info: Option<GeoInfo>,
    pub entry_page: Option<String>,
}

// =============================================================================
// METRICS
// =============================================================================

/// Aggregated metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric identifier
    pub metric_id: Uuid,
    /// Metric name
    pub name: String,
    /// Tenant ID
    pub tenant_id: Option<Uuid>,
    /// Period start time
    pub period_start: DateTime<Utc>,
    /// Period end time
    pub period_end: DateTime<Utc>,
    /// Granularity (hourly, daily, weekly, monthly)
    pub granularity: String,
    /// Metric value
    pub value: f64,
    /// Count of data points
    pub count: Option<i64>,
    /// Additional dimensions
    pub dimensions: HashMap<String, serde_json::Value>,
}

/// Course analytics summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseAnalytics {
    pub course_id: Uuid,
    pub total_enrollments: i64,
    pub active_students: i64,
    pub completion_rate: f64,
    pub average_progress: f64,
    pub average_score: Option<f64>,
    pub total_time_spent_minutes: i64,
    pub lesson_completion_rates: HashMap<Uuid, f64>,
}

/// User engagement analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEngagement {
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

/// Platform usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStats {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_users: i64,
    pub active_users: i64,
    pub new_users: i64,
    pub total_sessions: i64,
    pub total_page_views: i64,
    pub average_session_duration_seconds: f64,
    pub bounce_rate: f64,
    pub top_pages: Vec<PageStats>,
    pub top_courses: Vec<CourseStats>,
    pub platform_breakdown: HashMap<String, i64>,
}

/// Page view statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageStats {
    pub page_url: String,
    pub page_title: Option<String>,
    pub views: i64,
    pub unique_visitors: i64,
    pub average_time_seconds: f64,
}

/// Course statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseStats {
    pub course_id: Uuid,
    pub course_name: String,
    pub enrollments: i64,
    pub completions: i64,
    pub active_students: i64,
}

// =============================================================================
// REPORTS
// =============================================================================

/// Report configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// Report identifier
    pub report_id: Uuid,
    /// Report name
    pub name: String,
    /// Report type
    pub report_type: ReportType,
    /// Date range start
    pub date_from: DateTime<Utc>,
    /// Date range end
    pub date_to: DateTime<Utc>,
    /// Filters
    pub filters: HashMap<String, serde_json::Value>,
    /// Grouping dimensions
    pub group_by: Vec<String>,
    /// Metrics to include
    pub metrics: Vec<String>,
    /// Tenant ID
    pub tenant_id: Option<Uuid>,
    /// Created by user
    pub created_by: Option<Uuid>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Report type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    /// User engagement report
    UserEngagement,
    /// Course performance report
    CoursePerformance,
    /// Platform overview report
    PlatformOverview,
    /// Revenue report
    Revenue,
    /// Custom report
    Custom,
}

/// Query parameters for analytics.
#[derive(Debug, Clone, Deserialize)]
pub struct AnalyticsQuery {
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub event_types: Option<Vec<EventType>>,
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub platform: Option<Platform>,
    pub group_by: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Time series data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub label: Option<String>,
}

/// Event count by type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCount {
    pub event_type: String,
    pub count: i64,
}
