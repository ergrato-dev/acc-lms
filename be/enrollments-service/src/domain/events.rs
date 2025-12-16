//! # Enrollment Domain Events
//!
//! Events emitted by the enrollments service for cross-service communication.
//! These events are published to a message broker (e.g., NATS, RabbitMQ) for:
//!
//! - Analytics tracking
//! - Notifications triggering
//! - Course statistics updates
//! - Certificate generation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::{EnrollmentStatus, LessonProgressStatus};

// =============================================================================
// ENROLLMENT EVENTS
// =============================================================================

/// Events related to enrollment lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EnrollmentEvent {
    /// Student enrolled in a course.
    Created {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        enrollment_source: String,
        occurred_at: DateTime<Utc>,
    },

    /// Enrollment status changed.
    StatusChanged {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        old_status: EnrollmentStatus,
        new_status: EnrollmentStatus,
        occurred_at: DateTime<Utc>,
    },

    /// Student started the course (first lesson accessed).
    Started {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        occurred_at: DateTime<Utc>,
    },

    /// Student completed the course.
    Completed {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        completion_time_days: i32,
        final_progress: f64,
        occurred_at: DateTime<Utc>,
    },

    /// Certificate was issued.
    CertificateIssued {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        certificate_id: Uuid,
        occurred_at: DateTime<Utc>,
    },

    /// Enrollment was refunded.
    Refunded {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        reason: String,
        occurred_at: DateTime<Utc>,
    },

    /// Enrollment expired.
    Expired {
        enrollment_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
}

impl EnrollmentEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            EnrollmentEvent::Created { .. } => "enrollment.created",
            EnrollmentEvent::StatusChanged { .. } => "enrollment.status_changed",
            EnrollmentEvent::Started { .. } => "enrollment.started",
            EnrollmentEvent::Completed { .. } => "enrollment.completed",
            EnrollmentEvent::CertificateIssued { .. } => "enrollment.certificate_issued",
            EnrollmentEvent::Refunded { .. } => "enrollment.refunded",
            EnrollmentEvent::Expired { .. } => "enrollment.expired",
        }
    }

    /// Returns the enrollment_id for this event.
    pub fn enrollment_id(&self) -> Uuid {
        match self {
            EnrollmentEvent::Created { enrollment_id, .. }
            | EnrollmentEvent::StatusChanged { enrollment_id, .. }
            | EnrollmentEvent::Started { enrollment_id, .. }
            | EnrollmentEvent::Completed { enrollment_id, .. }
            | EnrollmentEvent::CertificateIssued { enrollment_id, .. }
            | EnrollmentEvent::Refunded { enrollment_id, .. }
            | EnrollmentEvent::Expired { enrollment_id, .. } => *enrollment_id,
        }
    }

    /// Returns the user_id for this event.
    pub fn user_id(&self) -> Uuid {
        match self {
            EnrollmentEvent::Created { user_id, .. }
            | EnrollmentEvent::StatusChanged { user_id, .. }
            | EnrollmentEvent::Started { user_id, .. }
            | EnrollmentEvent::Completed { user_id, .. }
            | EnrollmentEvent::CertificateIssued { user_id, .. }
            | EnrollmentEvent::Refunded { user_id, .. }
            | EnrollmentEvent::Expired { user_id, .. } => *user_id,
        }
    }
}

// =============================================================================
// PROGRESS EVENTS
// =============================================================================

/// Events related to lesson progress tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProgressEvent {
    /// Lesson progress was updated.
    Updated {
        progress_id: Uuid,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        old_status: LessonProgressStatus,
        new_status: LessonProgressStatus,
        completion_percentage: f64,
        occurred_at: DateTime<Utc>,
    },

    /// Lesson was completed.
    LessonCompleted {
        progress_id: Uuid,
        enrollment_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        time_spent_seconds: i32,
        occurred_at: DateTime<Utc>,
    },

    /// Video position was saved (for resume).
    PositionSaved {
        progress_id: Uuid,
        lesson_id: Uuid,
        user_id: Uuid,
        position_seconds: i32,
        occurred_at: DateTime<Utc>,
    },

    /// Learning streak updated.
    StreakUpdated {
        user_id: Uuid,
        current_streak: i32,
        longest_streak: i32,
        occurred_at: DateTime<Utc>,
    },
}

impl ProgressEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            ProgressEvent::Updated { .. } => "progress.updated",
            ProgressEvent::LessonCompleted { .. } => "progress.lesson_completed",
            ProgressEvent::PositionSaved { .. } => "progress.position_saved",
            ProgressEvent::StreakUpdated { .. } => "progress.streak_updated",
        }
    }
}
