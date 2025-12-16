//! # Enrollment Domain Entities
//!
//! Core domain entities for the enrollments service. These entities map to the
//! PostgreSQL schema defined in `db/migrations/postgresql/001_initial_schema.sql`.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Enrollment (aggregate root)
//!     └── LessonProgress
//! ```
//!
//! ## Design Decisions
//!
//! 1. **Enrollment as Aggregate Root**: All progress modifications go through Enrollment
//! 2. **Progress Percentage**: Computed from completed lessons vs total lessons
//! 3. **No FK to courses schema**: References by UUID, validated at service level

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Enrollment status.
///
/// Controls access and billing state of the enrollment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EnrollmentStatus {
    /// Student has active access to course
    Active,
    /// Student has completed the course
    Completed,
    /// Student has paused their enrollment
    Paused,
    /// Enrollment was refunded
    Refunded,
    /// Access has expired (for time-limited access)
    Expired,
}

impl Default for EnrollmentStatus {
    fn default() -> Self {
        EnrollmentStatus::Active
    }
}

impl std::fmt::Display for EnrollmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnrollmentStatus::Active => write!(f, "active"),
            EnrollmentStatus::Completed => write!(f, "completed"),
            EnrollmentStatus::Paused => write!(f, "paused"),
            EnrollmentStatus::Refunded => write!(f, "refunded"),
            EnrollmentStatus::Expired => write!(f, "expired"),
        }
    }
}

impl std::str::FromStr for EnrollmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(EnrollmentStatus::Active),
            "completed" => Ok(EnrollmentStatus::Completed),
            "paused" => Ok(EnrollmentStatus::Paused),
            "refunded" => Ok(EnrollmentStatus::Refunded),
            "expired" => Ok(EnrollmentStatus::Expired),
            _ => Err(format!("Invalid enrollment status: {}", s)),
        }
    }
}

/// Lesson progress status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LessonProgressStatus {
    /// Lesson not started
    NotStarted,
    /// Lesson in progress
    InProgress,
    /// Lesson completed
    Completed,
}

impl Default for LessonProgressStatus {
    fn default() -> Self {
        LessonProgressStatus::NotStarted
    }
}

impl std::fmt::Display for LessonProgressStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonProgressStatus::NotStarted => write!(f, "not_started"),
            LessonProgressStatus::InProgress => write!(f, "in_progress"),
            LessonProgressStatus::Completed => write!(f, "completed"),
        }
    }
}

// =============================================================================
// ENROLLMENT
// =============================================================================

/// Main enrollment entity - represents a student's access to a course.
///
/// Tracks enrollment status, progress, and access dates.
///
/// # Database Mapping
///
/// Maps to `enrollments.enrollments` table.
///
/// # Cross-Schema References
///
/// - `user_id`: References `auth.users(user_id)` - validated at service level
/// - `course_id`: References `courses.courses(course_id)` - validated at service level
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Enrollment {
    /// Unique identifier
    pub enrollment_id: Uuid,
    /// Student who enrolled (references auth.users)
    pub user_id: Uuid,
    /// Course enrolled in (references courses.courses)
    pub course_id: Uuid,
    /// Current enrollment status
    pub status: EnrollmentStatus,
    /// Overall progress percentage (0.00 - 100.00)
    pub progress_percentage: f64,
    /// When the student started the course
    pub started_at: Option<DateTime<Utc>>,
    /// When the student completed the course
    pub completed_at: Option<DateTime<Utc>>,
    /// Last time student accessed the course
    pub last_accessed_at: Option<DateTime<Utc>>,
    /// When certificate was issued (after completion)
    pub certificate_issued_at: Option<DateTime<Utc>>,
    /// How the student enrolled (purchase, gift, promo, admin)
    pub enrollment_source: Option<String>,
    /// When access expires (null = never)
    pub expires_at: Option<DateTime<Utc>>,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Enrollment {
    /// Returns true if the student has active access to the course.
    pub fn has_access(&self) -> bool {
        match self.status {
            EnrollmentStatus::Active | EnrollmentStatus::Completed => {
                // Check expiration
                if let Some(expires_at) = self.expires_at {
                    expires_at > Utc::now()
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    /// Returns true if the enrollment is completed.
    pub fn is_completed(&self) -> bool {
        self.status == EnrollmentStatus::Completed
    }

    /// Returns true if the enrollment is eligible for a certificate.
    pub fn can_get_certificate(&self) -> bool {
        self.is_completed() && self.certificate_issued_at.is_none()
    }

    /// Returns formatted progress string (e.g., "75%").
    pub fn formatted_progress(&self) -> String {
        format!("{:.0}%", self.progress_percentage)
    }
}

/// Data required to create a new enrollment.
#[derive(Debug, Clone, Deserialize)]
pub struct NewEnrollment {
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub enrollment_source: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Data for updating an enrollment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateEnrollment {
    pub status: Option<EnrollmentStatus>,
    pub progress_percentage: Option<f64>,
    pub started_at: Option<Option<DateTime<Utc>>>,
    pub completed_at: Option<Option<DateTime<Utc>>>,
    pub certificate_issued_at: Option<Option<DateTime<Utc>>>,
    pub expires_at: Option<Option<DateTime<Utc>>>,
}

// =============================================================================
// LESSON PROGRESS
// =============================================================================

/// Progress tracking for individual lessons.
///
/// Tracks how much of a lesson a student has completed, time spent,
/// and video playback position.
///
/// # Database Mapping
///
/// Maps to `enrollments.lesson_progress` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LessonProgress {
    /// Unique identifier
    pub progress_id: Uuid,
    /// Parent enrollment
    pub enrollment_id: Uuid,
    /// Lesson being tracked (references courses.lessons)
    pub lesson_id: Uuid,
    /// Student (denormalized for queries)
    pub user_id: Uuid,
    /// Current progress status
    pub status: LessonProgressStatus,
    /// Completion percentage (0.00 - 100.00)
    pub completion_percentage: f64,
    /// Total time spent in seconds
    pub time_spent_seconds: i32,
    /// Last video position in seconds (for resume)
    pub last_position_seconds: i32,
    /// When the lesson was completed
    pub completed_at: Option<DateTime<Utc>>,
    /// First access to this lesson
    pub first_accessed_at: DateTime<Utc>,
    /// Last access to this lesson
    pub last_accessed_at: DateTime<Utc>,
}

impl LessonProgress {
    /// Returns true if the lesson is completed.
    pub fn is_completed(&self) -> bool {
        self.status == LessonProgressStatus::Completed
    }

    /// Returns formatted time spent (e.g., "15:30").
    pub fn formatted_time_spent(&self) -> String {
        let minutes = self.time_spent_seconds / 60;
        let seconds = self.time_spent_seconds % 60;
        format!("{}:{:02}", minutes, seconds)
    }
}

/// Data for creating new lesson progress.
#[derive(Debug, Clone, Deserialize)]
pub struct NewLessonProgress {
    pub enrollment_id: Uuid,
    pub lesson_id: Uuid,
    pub user_id: Uuid,
}

/// Data for updating lesson progress.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateLessonProgress {
    pub status: Option<LessonProgressStatus>,
    pub completion_percentage: Option<f64>,
    pub time_spent_seconds: Option<i32>,
    pub last_position_seconds: Option<i32>,
    pub completed_at: Option<Option<DateTime<Utc>>>,
}

// =============================================================================
// AGGREGATES
// =============================================================================

/// Enrollment with all lesson progress loaded.
///
/// Used for detailed progress views.
#[derive(Debug, Clone, Serialize)]
pub struct EnrollmentWithProgress {
    /// The enrollment entity
    pub enrollment: Enrollment,
    /// Progress for each lesson
    pub lesson_progress: Vec<LessonProgress>,
}

/// Summary of a student's enrollment for dashboard.
#[derive(Debug, Clone, Serialize)]
pub struct EnrollmentSummary {
    pub enrollment_id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub course_thumbnail_url: Option<String>,
    pub instructor_name: String,
    pub status: EnrollmentStatus,
    pub progress_percentage: f64,
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub next_lesson_id: Option<Uuid>,
    pub next_lesson_title: Option<String>,
}
