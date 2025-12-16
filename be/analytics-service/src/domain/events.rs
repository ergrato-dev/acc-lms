//! # Analytics Domain Events
//!
//! Domain events emitted by the analytics service.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::{AggregationPeriod, EventType};

// =============================================================================
// ANALYTICS EVENTS
// =============================================================================

/// Events related to analytics tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnalyticsEvent {
    /// Event was tracked
    EventTracked {
        event_id: Uuid,
        event_type: EventType,
        user_id: Option<Uuid>,
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Batch of events tracked
    BatchTracked {
        event_count: i32,
        session_id: Option<Uuid>,
        timestamp: DateTime<Utc>,
    },
    /// Session started
    SessionStarted {
        session_id: Uuid,
        user_id: Option<Uuid>,
        timestamp: DateTime<Utc>,
    },
    /// Session ended
    SessionEnded {
        session_id: Uuid,
        duration_seconds: i64,
        page_views: i32,
        timestamp: DateTime<Utc>,
    },
    /// Metric aggregated
    MetricAggregated {
        metric_name: String,
        period: AggregationPeriod,
        period_start: DateTime<Utc>,
        value: f64,
        timestamp: DateTime<Utc>,
    },
    /// Report generated
    ReportGenerated {
        report_id: Uuid,
        report_name: String,
        generated_by: Option<Uuid>,
        timestamp: DateTime<Utc>,
    },
}

impl AnalyticsEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            AnalyticsEvent::EventTracked { .. } => "event_tracked",
            AnalyticsEvent::BatchTracked { .. } => "batch_tracked",
            AnalyticsEvent::SessionStarted { .. } => "session_started",
            AnalyticsEvent::SessionEnded { .. } => "session_ended",
            AnalyticsEvent::MetricAggregated { .. } => "metric_aggregated",
            AnalyticsEvent::ReportGenerated { .. } => "report_generated",
        }
    }

    /// Returns the associated event ID if applicable.
    pub fn event_id(&self) -> Option<Uuid> {
        match self {
            AnalyticsEvent::EventTracked { event_id, .. } => Some(*event_id),
            _ => None,
        }
    }

    /// Returns the associated session ID if applicable.
    pub fn session_id(&self) -> Option<Uuid> {
        match self {
            AnalyticsEvent::EventTracked { session_id, .. } => Some(*session_id),
            AnalyticsEvent::BatchTracked { session_id, .. } => *session_id,
            AnalyticsEvent::SessionStarted { session_id, .. } => Some(*session_id),
            AnalyticsEvent::SessionEnded { session_id, .. } => Some(*session_id),
            _ => None,
        }
    }
}

// =============================================================================
// USER ACTIVITY EVENTS
// =============================================================================

/// Events related to user learning activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LearningActivityEvent {
    /// User enrolled in course
    CourseEnrolled {
        user_id: Uuid,
        course_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// User completed course
    CourseCompleted {
        user_id: Uuid,
        course_id: Uuid,
        completion_time_minutes: i64,
        final_score: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    /// User started lesson
    LessonStarted {
        user_id: Uuid,
        lesson_id: Uuid,
        course_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// User completed lesson
    LessonCompleted {
        user_id: Uuid,
        lesson_id: Uuid,
        course_id: Uuid,
        time_spent_minutes: i64,
        timestamp: DateTime<Utc>,
    },
    /// User completed quiz
    QuizCompleted {
        user_id: Uuid,
        quiz_id: Uuid,
        course_id: Uuid,
        score: f64,
        attempts: i32,
        timestamp: DateTime<Utc>,
    },
    /// User submitted assignment
    AssignmentSubmitted {
        user_id: Uuid,
        assignment_id: Uuid,
        course_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// User progress milestone
    ProgressMilestone {
        user_id: Uuid,
        course_id: Uuid,
        progress_percentage: i32,
        timestamp: DateTime<Utc>,
    },
}

impl LearningActivityEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            LearningActivityEvent::CourseEnrolled { .. } => "course_enrolled",
            LearningActivityEvent::CourseCompleted { .. } => "course_completed",
            LearningActivityEvent::LessonStarted { .. } => "lesson_started",
            LearningActivityEvent::LessonCompleted { .. } => "lesson_completed",
            LearningActivityEvent::QuizCompleted { .. } => "quiz_completed",
            LearningActivityEvent::AssignmentSubmitted { .. } => "assignment_submitted",
            LearningActivityEvent::ProgressMilestone { .. } => "progress_milestone",
        }
    }

    /// Returns the user ID associated with this event.
    pub fn user_id(&self) -> Uuid {
        match self {
            LearningActivityEvent::CourseEnrolled { user_id, .. } => *user_id,
            LearningActivityEvent::CourseCompleted { user_id, .. } => *user_id,
            LearningActivityEvent::LessonStarted { user_id, .. } => *user_id,
            LearningActivityEvent::LessonCompleted { user_id, .. } => *user_id,
            LearningActivityEvent::QuizCompleted { user_id, .. } => *user_id,
            LearningActivityEvent::AssignmentSubmitted { user_id, .. } => *user_id,
            LearningActivityEvent::ProgressMilestone { user_id, .. } => *user_id,
        }
    }

    /// Returns the course ID associated with this event.
    pub fn course_id(&self) -> Uuid {
        match self {
            LearningActivityEvent::CourseEnrolled { course_id, .. } => *course_id,
            LearningActivityEvent::CourseCompleted { course_id, .. } => *course_id,
            LearningActivityEvent::LessonStarted { course_id, .. } => *course_id,
            LearningActivityEvent::LessonCompleted { course_id, .. } => *course_id,
            LearningActivityEvent::QuizCompleted { course_id, .. } => *course_id,
            LearningActivityEvent::AssignmentSubmitted { course_id, .. } => *course_id,
            LearningActivityEvent::ProgressMilestone { course_id, .. } => *course_id,
        }
    }
}
