//! # Data Transfer Objects
//!
//! Request and response DTOs for the API.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    ExportFormat, GradeStatus, TranscriptEntryStatus,
    GradeDistribution,
};

// =============================================================================
// GENERIC RESPONSES
// =============================================================================

/// Generic API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorResponse {
                code,
                message: message.into(),
            }),
        }
    }
}

/// Error response details.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

// =============================================================================
// GRADE DTOs
// =============================================================================

/// Query parameters for grades list.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct GradeQueryParams {
    pub course_id: Option<Uuid>,
    pub status: Option<String>,
    pub passed: Option<bool>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    #[validate(range(min = 1))]
    pub page: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i32>,
}

/// Single grade entry response.
#[derive(Debug, Serialize)]
pub struct GradeEntryResponse {
    pub submission_id: Uuid,
    pub quiz_id: Uuid,
    pub quiz_title: String,
    pub score: Decimal,
    pub max_score: Decimal,
    pub percentage: Decimal,
    pub letter_grade: String,
    pub passed: Option<bool>,
    pub attempt_number: i32,
    pub status: GradeStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub graded_at: Option<DateTime<Utc>>,
    pub instructor_feedback: Option<String>,
}

impl From<crate::domain::GradeEntry> for GradeEntryResponse {
    fn from(e: crate::domain::GradeEntry) -> Self {
        let letter_grade = e.letter_grade().to_string();
        let status = e.display_status();
        Self {
            submission_id: e.submission_id,
            quiz_id: e.quiz_id,
            quiz_title: e.quiz_title,
            score: e.score,
            max_score: e.max_score,
            percentage: e.percentage,
            letter_grade,
            passed: e.passed,
            attempt_number: e.attempt_number,
            status,
            submitted_at: e.submitted_at,
            graded_at: e.graded_at,
            instructor_feedback: e.instructor_feedback,
        }
    }
}

/// Course grades response.
#[derive(Debug, Serialize)]
pub struct CourseGradeResponse {
    pub course_id: Uuid,
    pub course_title: String,
    pub total_quizzes: i32,
    pub completed_quizzes: i32,
    pub pending_review: i32,
    pub total_points_earned: Decimal,
    pub total_points_possible: Decimal,
    pub weighted_average: Decimal,
    pub letter_grade: String,
    pub passed_quizzes: i32,
    pub failed_quizzes: i32,
    pub grades: Vec<GradeEntryResponse>,
    pub last_activity: Option<DateTime<Utc>>,
}

impl From<crate::domain::CourseGrade> for CourseGradeResponse {
    fn from(c: crate::domain::CourseGrade) -> Self {
        Self {
            course_id: c.course_id,
            course_title: c.course_title,
            total_quizzes: c.total_quizzes,
            completed_quizzes: c.completed_quizzes,
            pending_review: c.pending_review,
            total_points_earned: c.total_points_earned,
            total_points_possible: c.total_points_possible,
            weighted_average: c.weighted_average,
            letter_grade: c.letter_grade.to_string(),
            passed_quizzes: c.passed_quizzes,
            failed_quizzes: c.failed_quizzes,
            grades: c.grades.into_iter().map(Into::into).collect(),
            last_activity: c.last_activity,
        }
    }
}

/// Student grades summary response.
#[derive(Debug, Serialize)]
pub struct StudentGradeSummaryResponse {
    pub user_id: Uuid,
    pub total_courses: i32,
    pub courses_completed: i32,
    pub overall_average: Decimal,
    pub overall_letter_grade: String,
    pub gpa: Decimal,
    pub total_quizzes_taken: i32,
    pub total_quizzes_passed: i32,
    pub pending_reviews: i32,
    pub course_grades: Vec<CourseGradeResponse>,
    pub calculated_at: DateTime<Utc>,
}

impl From<crate::domain::StudentGradeSummary> for StudentGradeSummaryResponse {
    fn from(s: crate::domain::StudentGradeSummary) -> Self {
        Self {
            user_id: s.user_id,
            total_courses: s.total_courses,
            courses_completed: s.courses_completed,
            overall_average: s.overall_average,
            overall_letter_grade: s.overall_letter_grade.to_string(),
            gpa: s.gpa,
            total_quizzes_taken: s.total_quizzes_taken,
            total_quizzes_passed: s.total_quizzes_passed,
            pending_reviews: s.pending_reviews,
            course_grades: s.course_grades.into_iter().map(Into::into).collect(),
            calculated_at: s.calculated_at,
        }
    }
}

// =============================================================================
// TRANSCRIPT DTOs
// =============================================================================

/// Export query parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct ExportQueryParams {
    #[serde(default)]
    pub format: Option<ExportFormat>,
}

/// Transcript entry response.
#[derive(Debug, Serialize)]
pub struct TranscriptEntryResponse {
    pub course_id: Uuid,
    pub course_title: String,
    pub instructor_name: String,
    pub enrollment_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub progress_percentage: Decimal,
    pub grade_percentage: Decimal,
    pub letter_grade: String,
    pub status: TranscriptEntryStatus,
    pub certificate_issued: bool,
    pub certificate_url: Option<String>,
}

impl From<crate::domain::TranscriptEntry> for TranscriptEntryResponse {
    fn from(e: crate::domain::TranscriptEntry) -> Self {
        Self {
            course_id: e.course_id,
            course_title: e.course_title,
            instructor_name: e.instructor_name,
            enrollment_date: e.enrollment_date,
            completion_date: e.completion_date,
            progress_percentage: e.progress_percentage,
            grade_percentage: e.grade_percentage,
            letter_grade: e.letter_grade.to_string(),
            status: e.status,
            certificate_issued: e.certificate_issued,
            certificate_url: e.certificate_url,
        }
    }
}

/// Transcript summary response.
#[derive(Debug, Serialize)]
pub struct TranscriptSummaryResponse {
    pub total_courses_enrolled: i32,
    pub courses_completed: i32,
    pub courses_in_progress: i32,
    pub overall_gpa: Decimal,
    pub overall_letter_grade: String,
    pub total_certificates: i32,
    pub member_since: DateTime<Utc>,
}

impl From<crate::domain::TranscriptSummary> for TranscriptSummaryResponse {
    fn from(s: crate::domain::TranscriptSummary) -> Self {
        Self {
            total_courses_enrolled: s.total_courses_enrolled,
            courses_completed: s.courses_completed,
            courses_in_progress: s.courses_in_progress,
            overall_gpa: s.overall_gpa,
            overall_letter_grade: s.overall_letter_grade.to_string(),
            total_certificates: s.total_certificates,
            member_since: s.member_since,
        }
    }
}

/// Full transcript response.
#[derive(Debug, Serialize)]
pub struct TranscriptResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub generated_at: DateTime<Utc>,
    pub summary: TranscriptSummaryResponse,
    pub entries: Vec<TranscriptEntryResponse>,
}

impl From<crate::domain::Transcript> for TranscriptResponse {
    fn from(t: crate::domain::Transcript) -> Self {
        Self {
            user_id: t.user_id,
            user_name: t.user_name,
            user_email: t.user_email,
            generated_at: t.generated_at,
            summary: t.summary.into(),
            entries: t.entries.into_iter().map(Into::into).collect(),
        }
    }
}

// =============================================================================
// STATISTICS DTOs (Instructor)
// =============================================================================

/// Question stats response.
#[derive(Debug, Serialize)]
pub struct QuestionStatsResponse {
    pub question_id: Uuid,
    pub question_text: String,
    pub question_type: String,
    pub total_responses: i32,
    pub correct_responses: i32,
    pub correct_percentage: Decimal,
    pub common_mistake: Option<String>,
}

impl From<crate::domain::QuestionStats> for QuestionStatsResponse {
    fn from(q: crate::domain::QuestionStats) -> Self {
        Self {
            question_id: q.question_id,
            question_text: q.question_text,
            question_type: q.question_type,
            total_responses: q.total_responses,
            correct_responses: q.correct_responses,
            correct_percentage: q.correct_percentage,
            common_mistake: q.common_mistake,
        }
    }
}

/// Quiz statistics response.
#[derive(Debug, Serialize)]
pub struct QuizStatsResponse {
    pub quiz_id: Uuid,
    pub quiz_title: String,
    pub total_submissions: i32,
    pub unique_students: i32,
    pub average_score: Decimal,
    pub highest_score: Decimal,
    pub lowest_score: Decimal,
    pub pass_rate: Decimal,
    pub average_time_seconds: Option<i32>,
    pub average_attempts: Decimal,
    pub question_stats: Vec<QuestionStatsResponse>,
}

impl From<crate::domain::QuizStats> for QuizStatsResponse {
    fn from(q: crate::domain::QuizStats) -> Self {
        Self {
            quiz_id: q.quiz_id,
            quiz_title: q.quiz_title,
            total_submissions: q.total_submissions,
            unique_students: q.unique_students,
            average_score: q.average_score,
            highest_score: q.highest_score,
            lowest_score: q.lowest_score,
            pass_rate: q.pass_rate,
            average_time_seconds: q.average_time_seconds,
            average_attempts: q.average_attempts,
            question_stats: q.question_stats.into_iter().map(Into::into).collect(),
        }
    }
}

/// Course statistics response.
#[derive(Debug, Serialize)]
pub struct CourseStatsResponse {
    pub course_id: Uuid,
    pub course_title: String,
    pub total_students: i32,
    pub students_with_submissions: i32,
    pub total_quizzes: i32,
    pub average_score_percentage: Decimal,
    pub pass_rate: Decimal,
    pub grade_distribution: GradeDistribution,
    pub quiz_stats: Vec<QuizStatsResponse>,
    pub calculated_at: DateTime<Utc>,
}

impl From<crate::domain::CourseStats> for CourseStatsResponse {
    fn from(c: crate::domain::CourseStats) -> Self {
        Self {
            course_id: c.course_id,
            course_title: c.course_title,
            total_students: c.total_students,
            students_with_submissions: c.students_with_submissions,
            total_quizzes: c.total_quizzes,
            average_score_percentage: c.average_score_percentage,
            pass_rate: c.pass_rate,
            grade_distribution: c.grade_distribution,
            quiz_stats: c.quiz_stats.into_iter().map(Into::into).collect(),
            calculated_at: c.calculated_at,
        }
    }
}

// =============================================================================
// EXPORT DTOs
// =============================================================================

/// Export response with file data.
#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: usize,
    /// Base64 encoded content (for JSON response) or raw bytes (for direct download)
    pub content: Option<String>,
}
