//! # Assessment Domain Events
//!
//! Domain events for the assessments service. These events are emitted when
//! significant state changes occur and can be consumed by other services.
//!
//! ## Event Categories
//!
//! - **Quiz Events**: Quiz creation, publication, updates
//! - **Submission Events**: Attempt started, submitted, graded

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::SubmissionStatus;

// =============================================================================
// QUIZ EVENTS
// =============================================================================

/// Events related to quiz lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum QuizEvent {
    /// A new quiz was created
    Created {
        quiz_id: Uuid,
        course_id: Uuid,
        lesson_id: Option<Uuid>,
        title: String,
        created_at: DateTime<Utc>,
    },

    /// Quiz was published and is now available
    Published {
        quiz_id: Uuid,
        course_id: Uuid,
        published_at: DateTime<Utc>,
    },

    /// Quiz was unpublished
    Unpublished {
        quiz_id: Uuid,
        course_id: Uuid,
        unpublished_at: DateTime<Utc>,
    },

    /// Quiz was updated
    Updated {
        quiz_id: Uuid,
        course_id: Uuid,
        updated_at: DateTime<Utc>,
    },

    /// Quiz was deleted
    Deleted {
        quiz_id: Uuid,
        course_id: Uuid,
        deleted_at: DateTime<Utc>,
    },

    /// Question was added to quiz
    QuestionAdded {
        quiz_id: Uuid,
        question_id: Uuid,
        sort_order: i32,
        created_at: DateTime<Utc>,
    },

    /// Question was removed from quiz
    QuestionRemoved {
        quiz_id: Uuid,
        question_id: Uuid,
        removed_at: DateTime<Utc>,
    },
}

impl QuizEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            QuizEvent::Created { .. } => "quiz.created",
            QuizEvent::Published { .. } => "quiz.published",
            QuizEvent::Unpublished { .. } => "quiz.unpublished",
            QuizEvent::Updated { .. } => "quiz.updated",
            QuizEvent::Deleted { .. } => "quiz.deleted",
            QuizEvent::QuestionAdded { .. } => "quiz.question_added",
            QuizEvent::QuestionRemoved { .. } => "quiz.question_removed",
        }
    }

    /// Returns the quiz ID associated with this event.
    pub fn quiz_id(&self) -> Uuid {
        match self {
            QuizEvent::Created { quiz_id, .. } => *quiz_id,
            QuizEvent::Published { quiz_id, .. } => *quiz_id,
            QuizEvent::Unpublished { quiz_id, .. } => *quiz_id,
            QuizEvent::Updated { quiz_id, .. } => *quiz_id,
            QuizEvent::Deleted { quiz_id, .. } => *quiz_id,
            QuizEvent::QuestionAdded { quiz_id, .. } => *quiz_id,
            QuizEvent::QuestionRemoved { quiz_id, .. } => *quiz_id,
        }
    }
}

// =============================================================================
// SUBMISSION EVENTS
// =============================================================================

/// Events related to quiz submissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum SubmissionEvent {
    /// Student started a quiz attempt
    Started {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        enrollment_id: Uuid,
        attempt_number: i32,
        started_at: DateTime<Utc>,
    },

    /// Student submitted their answers
    Submitted {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        time_spent_seconds: i32,
        submitted_at: DateTime<Utc>,
    },

    /// Quiz was auto-graded
    AutoGraded {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        score: f64,
        max_score: f64,
        passed: bool,
        graded_at: DateTime<Utc>,
    },

    /// Quiz was manually graded by instructor
    ManuallyGraded {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        grader_id: Uuid,
        score: f64,
        max_score: f64,
        passed: bool,
        graded_at: DateTime<Utc>,
    },

    /// Student passed the quiz
    Passed {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        enrollment_id: Uuid,
        score_percentage: f64,
        passed_at: DateTime<Utc>,
    },

    /// Student failed the quiz
    Failed {
        submission_id: Uuid,
        quiz_id: Uuid,
        user_id: Uuid,
        enrollment_id: Uuid,
        score_percentage: f64,
        attempts_remaining: Option<i32>,
        failed_at: DateTime<Utc>,
    },
}

impl SubmissionEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            SubmissionEvent::Started { .. } => "submission.started",
            SubmissionEvent::Submitted { .. } => "submission.submitted",
            SubmissionEvent::AutoGraded { .. } => "submission.auto_graded",
            SubmissionEvent::ManuallyGraded { .. } => "submission.manually_graded",
            SubmissionEvent::Passed { .. } => "submission.passed",
            SubmissionEvent::Failed { .. } => "submission.failed",
        }
    }

    /// Returns the submission ID associated with this event.
    pub fn submission_id(&self) -> Uuid {
        match self {
            SubmissionEvent::Started { submission_id, .. } => *submission_id,
            SubmissionEvent::Submitted { submission_id, .. } => *submission_id,
            SubmissionEvent::AutoGraded { submission_id, .. } => *submission_id,
            SubmissionEvent::ManuallyGraded { submission_id, .. } => *submission_id,
            SubmissionEvent::Passed { submission_id, .. } => *submission_id,
            SubmissionEvent::Failed { submission_id, .. } => *submission_id,
        }
    }

    /// Returns the user ID associated with this event.
    pub fn user_id(&self) -> Uuid {
        match self {
            SubmissionEvent::Started { user_id, .. } => *user_id,
            SubmissionEvent::Submitted { user_id, .. } => *user_id,
            SubmissionEvent::AutoGraded { user_id, .. } => *user_id,
            SubmissionEvent::ManuallyGraded { user_id, .. } => *user_id,
            SubmissionEvent::Passed { user_id, .. } => *user_id,
            SubmissionEvent::Failed { user_id, .. } => *user_id,
        }
    }
}
