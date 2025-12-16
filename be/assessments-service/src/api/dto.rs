//! # Data Transfer Objects
//!
//! Request and response DTOs for the assessment API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Quiz, QuizQuestion, QuizSubmission, QuestionType,
    QuizWithQuestions, SubmissionWithResponses,
};
use crate::repository::QuizStats;

// =============================================================================
// QUIZ DTOs
// =============================================================================

/// Request to create a quiz.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuizRequest {
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,

    #[validate(length(min = 1, max = 255))]
    pub title: String,

    pub description: Option<String>,
    pub instructions: Option<String>,

    pub total_points: Option<i32>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,

    #[validate(range(min = 0.0, max = 100.0))]
    pub passing_score_percentage: Option<f64>,

    pub shuffle_questions: Option<bool>,
    pub show_correct_answers: Option<bool>,
}

/// Request to update a quiz.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuizRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,

    pub description: Option<String>,
    pub instructions: Option<String>,
    pub total_points: Option<i32>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,

    #[validate(range(min = 0.0, max = 100.0))]
    pub passing_score_percentage: Option<f64>,

    pub shuffle_questions: Option<bool>,
    pub show_correct_answers: Option<bool>,
    pub is_published: Option<bool>,
}

/// Quiz response.
#[derive(Debug, Serialize)]
pub struct QuizResponseDto {
    pub quiz_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub total_points: i32,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub passing_score_percentage: f64,
    pub shuffle_questions: bool,
    pub show_correct_answers: bool,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Quiz> for QuizResponseDto {
    fn from(q: Quiz) -> Self {
        Self {
            quiz_id: q.quiz_id,
            course_id: q.course_id,
            lesson_id: q.lesson_id,
            title: q.title,
            description: q.description,
            instructions: q.instructions,
            total_points: q.total_points,
            time_limit_minutes: q.time_limit_minutes,
            max_attempts: q.max_attempts,
            passing_score_percentage: q.passing_score_percentage,
            shuffle_questions: q.shuffle_questions,
            show_correct_answers: q.show_correct_answers,
            is_published: q.is_published,
            created_at: q.created_at,
            updated_at: q.updated_at,
        }
    }
}

/// Quiz with questions response.
#[derive(Debug, Serialize)]
pub struct QuizWithQuestionsResponse {
    pub quiz: QuizResponseDto,
    pub questions: Vec<QuestionResponseDto>,
}

impl From<QuizWithQuestions> for QuizWithQuestionsResponse {
    fn from(qwq: QuizWithQuestions) -> Self {
        Self {
            quiz: qwq.quiz.into(),
            questions: qwq.questions.into_iter().map(Into::into).collect(),
        }
    }
}

// =============================================================================
// QUESTION DTOs
// =============================================================================

/// Request to create a question.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionRequest {
    pub quiz_id: Uuid,
    pub question_type: QuestionType,

    #[validate(length(min = 1))]
    pub question_text: String,

    pub options: serde_json::Value,
    pub correct_answers: serde_json::Value,

    pub points: Option<i32>,
    pub sort_order: i32,
    pub explanation: Option<String>,
    pub code_language: Option<String>,
}

/// Request to update a question.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuestionRequest {
    pub question_type: Option<QuestionType>,

    #[validate(length(min = 1))]
    pub question_text: Option<String>,

    pub options: Option<serde_json::Value>,
    pub correct_answers: Option<serde_json::Value>,
    pub points: Option<i32>,
    pub sort_order: Option<i32>,
    pub explanation: Option<String>,
    pub code_language: Option<String>,
}

/// Question response.
#[derive(Debug, Serialize)]
pub struct QuestionResponseDto {
    pub question_id: Uuid,
    pub quiz_id: Uuid,
    pub question_type: QuestionType,
    pub question_text: String,
    pub options: serde_json::Value,
    #[serde(skip_serializing_if = "is_empty_array")]
    pub correct_answers: serde_json::Value,
    pub points: i32,
    pub sort_order: i32,
    pub explanation: Option<String>,
    pub code_language: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

fn is_empty_array(v: &serde_json::Value) -> bool {
    v.as_array().map(|a| a.is_empty()).unwrap_or(false)
}

impl From<QuizQuestion> for QuestionResponseDto {
    fn from(q: QuizQuestion) -> Self {
        Self {
            question_id: q.question_id,
            quiz_id: q.quiz_id,
            question_type: q.question_type,
            question_text: q.question_text,
            options: q.options,
            correct_answers: q.correct_answers,
            points: q.points,
            sort_order: q.sort_order,
            explanation: q.explanation,
            code_language: q.code_language,
            created_at: q.created_at,
        }
    }
}

// =============================================================================
// SUBMISSION DTOs
// =============================================================================

/// Request to start a quiz.
#[derive(Debug, Deserialize)]
pub struct StartQuizRequest {
    pub enrollment_id: Uuid,
}

/// Request to save an answer.
#[derive(Debug, Deserialize)]
pub struct SaveAnswerRequest {
    pub question_id: Uuid,
    pub answer_data: serde_json::Value,
}

/// Request to submit a quiz.
#[derive(Debug, Deserialize)]
pub struct SubmitQuizRequest {
    pub time_spent_seconds: i32,
}

/// Submission response.
#[derive(Debug, Serialize)]
pub struct SubmissionResponseDto {
    pub submission_id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub attempt_number: i32,
    pub status: String,
    pub score: f64,
    pub max_score: f64,
    pub passed: Option<bool>,
    pub time_spent_seconds: Option<i32>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub graded_at: Option<chrono::DateTime<chrono::Utc>>,
    pub instructor_feedback: Option<String>,
}

impl From<QuizSubmission> for SubmissionResponseDto {
    fn from(s: QuizSubmission) -> Self {
        Self {
            submission_id: s.submission_id,
            quiz_id: s.quiz_id,
            user_id: s.user_id,
            enrollment_id: s.enrollment_id,
            attempt_number: s.attempt_number,
            status: s.status.to_string(),
            score: s.score,
            max_score: s.max_score,
            passed: s.passed,
            time_spent_seconds: s.time_spent_seconds,
            started_at: s.started_at,
            submitted_at: s.submitted_at,
            graded_at: s.graded_at,
            instructor_feedback: s.instructor_feedback,
        }
    }
}

/// Submission with responses.
#[derive(Debug, Serialize)]
pub struct SubmissionWithResponsesResponse {
    pub submission: SubmissionResponseDto,
    pub responses: Vec<AnswerResponseDto>,
}

impl From<SubmissionWithResponses> for SubmissionWithResponsesResponse {
    fn from(swr: SubmissionWithResponses) -> Self {
        Self {
            submission: swr.submission.into(),
            responses: swr.responses.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for a single question answer.
#[derive(Debug, Serialize)]
pub struct AnswerResponseDto {
    pub response_id: Uuid,
    pub submission_id: Uuid,
    pub question_id: Uuid,
    pub answer_data: serde_json::Value,
    pub is_correct: Option<bool>,
    pub points_earned: f64,
    pub instructor_feedback: Option<String>,
    pub auto_graded: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::QuizResponse> for AnswerResponseDto {
    fn from(r: crate::domain::QuizResponse) -> Self {
        Self {
            response_id: r.response_id,
            submission_id: r.submission_id,
            question_id: r.question_id,
            answer_data: r.answer_data,
            is_correct: r.is_correct,
            points_earned: r.points_earned,
            instructor_feedback: r.instructor_feedback,
            auto_graded: r.auto_graded,
            created_at: r.created_at,
        }
    }
}

// =============================================================================
// GRADING DTOs
// =============================================================================

/// Request to grade a submission.
#[derive(Debug, Deserialize, Validate)]
pub struct GradeSubmissionRequest {
    #[validate(range(min = 0.0))]
    pub score: f64,

    pub feedback: Option<String>,
}

/// Request to grade a response.
#[derive(Debug, Deserialize, Validate)]
pub struct GradeResponseRequest {
    #[validate(range(min = 0.0))]
    pub points_earned: f64,

    pub feedback: Option<String>,
}

// =============================================================================
// COMMON DTOs
// =============================================================================

/// Paginated list response.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total: Option<i64>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, page_size: i64) -> Self {
        Self {
            data,
            page,
            page_size,
            total: None,
        }
    }

    pub fn with_total(mut self, total: i64) -> Self {
        self.total = Some(total);
        self
    }
}

/// Quiz statistics response.
#[derive(Debug, Serialize)]
pub struct QuizStatsResponse {
    pub total_submissions: i64,
    pub graded_count: i64,
    pub passed_count: i64,
    pub average_score: f64,
    pub average_time_seconds: f64,
    pub pass_rate: Option<f64>,
}

impl From<QuizStats> for QuizStatsResponse {
    fn from(s: QuizStats) -> Self {
        let pass_rate = if s.graded_count > 0 {
            Some((s.passed_count as f64 / s.graded_count as f64) * 100.0)
        } else {
            None
        };

        Self {
            total_submissions: s.total_submissions,
            graded_count: s.graded_count,
            passed_count: s.passed_count,
            average_score: s.avg_score,
            average_time_seconds: s.avg_time_seconds,
            pass_rate,
        }
    }
}

/// Pagination query parameters.
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 20 }

/// Error response.
#[derive(Debug, Serialize)]
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
