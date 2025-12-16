//! # Data Transfer Objects
//!
//! Request/response DTOs for enrollment API endpoints.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Enrollment, EnrollmentStatus, EnrollmentWithProgress, LessonProgress,
    LessonProgressStatus,
};
use crate::repository::{CourseEnrollmentStats, UserLearningStats};

// =============================================================================
// ENROLLMENT DTOs
// =============================================================================

/// Response DTO for enrollment data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentDto {
    pub enrollment_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub status: String,
    pub progress_percentage: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub certificate_issued_at: Option<DateTime<Utc>>,
    pub enrollment_source: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Enrollment> for EnrollmentDto {
    fn from(e: Enrollment) -> Self {
        Self {
            enrollment_id: e.enrollment_id,
            user_id: e.user_id,
            course_id: e.course_id,
            status: e.status.to_string(),
            progress_percentage: e.progress_percentage,
            started_at: e.started_at,
            completed_at: e.completed_at,
            last_accessed_at: e.last_accessed_at,
            certificate_issued_at: e.certificate_issued_at,
            enrollment_source: e.enrollment_source,
            expires_at: e.expires_at,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

/// Response DTO for lesson progress.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonProgressDto {
    pub progress_id: Uuid,
    pub enrollment_id: Uuid,
    pub lesson_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub completion_percentage: f64,
    pub time_spent_seconds: i32,
    pub last_position_seconds: i32,
    pub completed_at: Option<DateTime<Utc>>,
    pub first_accessed_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
}

impl From<LessonProgress> for LessonProgressDto {
    fn from(p: LessonProgress) -> Self {
        Self {
            progress_id: p.progress_id,
            enrollment_id: p.enrollment_id,
            lesson_id: p.lesson_id,
            user_id: p.user_id,
            status: p.status.to_string(),
            completion_percentage: p.completion_percentage,
            time_spent_seconds: p.time_spent_seconds,
            last_position_seconds: p.last_position_seconds,
            completed_at: p.completed_at,
            first_accessed_at: p.first_accessed_at,
            last_accessed_at: p.last_accessed_at,
        }
    }
}

/// Response DTO for enrollment with progress.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentWithProgressDto {
    #[serde(flatten)]
    pub enrollment: EnrollmentDto,
    pub lesson_progress: Vec<LessonProgressDto>,
}

impl From<EnrollmentWithProgress> for EnrollmentWithProgressDto {
    fn from(ewp: EnrollmentWithProgress) -> Self {
        Self {
            enrollment: ewp.enrollment.into(),
            lesson_progress: ewp.lesson_progress.into_iter().map(Into::into).collect(),
        }
    }
}

// =============================================================================
// REQUEST DTOs
// =============================================================================

/// Request to enroll in a course.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnrollRequest {
    pub course_id: Uuid,

    #[validate(length(max = 100))]
    pub enrollment_source: Option<String>,

    pub expires_at: Option<DateTime<Utc>>,
}

/// Query parameters for listing enrollments.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEnrollmentsQuery {
    pub status: Option<String>,

    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 20 }

impl ListEnrollmentsQuery {
    pub fn status(&self) -> Option<EnrollmentStatus> {
        self.status.as_ref().and_then(|s| s.parse().ok())
    }
}

/// Request to update enrollment status.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnrollmentStatusRequest {
    #[validate(length(min = 1, max = 20))]
    pub status: String,
}

impl UpdateEnrollmentStatusRequest {
    pub fn status(&self) -> Option<EnrollmentStatus> {
        self.status.parse().ok()
    }
}

// =============================================================================
// PROGRESS REQUEST DTOs
// =============================================================================

/// Request to start a lesson.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLessonRequest {
    pub lesson_id: Uuid,
}

/// Request to update lesson progress.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgressRequest {
    pub lesson_id: Uuid,

    /// Time spent since last update (in seconds)
    #[validate(range(min = 0, max = 86400))]
    pub time_spent_delta: Option<i32>,

    /// Current playback position (in seconds)
    #[validate(range(min = 0))]
    pub position_seconds: Option<i32>,

    /// Completion percentage (0-100)
    #[validate(range(min = 0.0, max = 100.0))]
    pub completion_percentage: Option<f64>,
}

/// Request to complete a lesson.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteLessonRequest {
    pub lesson_id: Uuid,

    /// Total number of lessons in the course (for completion check)
    pub total_lessons: i64,
}

/// Request to save playback position.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SavePositionRequest {
    pub lesson_id: Uuid,

    #[validate(range(min = 0))]
    pub position_seconds: i32,
}

// =============================================================================
// RESPONSE DTOs
// =============================================================================

/// Paginated list response.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(page: i64, page_size: i64, total: i64) -> Self {
        let total_pages = (total + page_size - 1) / page_size;
        Self {
            page,
            page_size,
            total,
            total_pages,
        }
    }
}

/// Response for lesson completion.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonCompletionResponse {
    pub progress: LessonProgressDto,
    pub course_completed: bool,
}

/// Check enrollment response.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckEnrollmentResponse {
    pub is_enrolled: bool,
    pub enrollment: Option<EnrollmentDto>,
}

// =============================================================================
// STATISTICS DTOs
// =============================================================================

/// Course enrollment statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseStatsDto {
    pub total_enrollments: i64,
    pub active_count: i64,
    pub completed_count: i64,
    pub avg_progress: f64,
    pub completion_rate: f64,
}

impl From<CourseEnrollmentStats> for CourseStatsDto {
    fn from(s: CourseEnrollmentStats) -> Self {
        let completion_rate = if s.total_enrollments > 0 {
            (s.completed_count as f64 / s.total_enrollments as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_enrollments: s.total_enrollments,
            active_count: s.active_count,
            completed_count: s.completed_count,
            avg_progress: s.avg_progress,
            completion_rate,
        }
    }
}

/// User learning statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsDto {
    pub total_enrolled: i64,
    pub total_completed: i64,
    pub in_progress: i64,
    pub total_time_hours: f64,
}

impl From<UserLearningStats> for UserStatsDto {
    fn from(s: UserLearningStats) -> Self {
        Self {
            total_enrolled: s.total_enrolled,
            total_completed: s.total_completed,
            in_progress: s.in_progress,
            total_time_hours: s.total_time_seconds as f64 / 3600.0,
        }
    }
}

// =============================================================================
// ERROR RESPONSE
// =============================================================================

/// Standard error response.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
