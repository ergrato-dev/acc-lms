//! # Assessment Domain Entities
//!
//! Core domain entities for the assessments service. These entities map to the
//! PostgreSQL schema defined in `db/migrations/postgresql/002_assignments_and_grades.sql`.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Quiz (aggregate root)
//!     └── QuizQuestion
//!
//! QuizSubmission (aggregate root)
//!     └── QuizResponse
//! ```
//!
//! ## Design Decisions
//!
//! 1. **Quiz as Aggregate**: Questions belong to quizzes
//! 2. **Submission as Aggregate**: Responses belong to submissions
//! 3. **JSONB for flexibility**: Options and answers stored as JSON
//! 4. **Auto-grading support**: Boolean flag for manual review needs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Question types supported by the quiz system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    /// Single correct answer from options
    SingleChoice,
    /// Multiple correct answers from options
    MultipleChoice,
    /// True or false question
    TrueFalse,
    /// Short text answer
    ShortAnswer,
    /// Long form essay answer (manual grading)
    Essay,
    /// Code submission (may need external evaluation)
    Code,
}

impl Default for QuestionType {
    fn default() -> Self {
        QuestionType::SingleChoice
    }
}

impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionType::SingleChoice => write!(f, "single_choice"),
            QuestionType::MultipleChoice => write!(f, "multiple_choice"),
            QuestionType::TrueFalse => write!(f, "true_false"),
            QuestionType::ShortAnswer => write!(f, "short_answer"),
            QuestionType::Essay => write!(f, "essay"),
            QuestionType::Code => write!(f, "code"),
        }
    }
}

impl std::str::FromStr for QuestionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "single_choice" => Ok(QuestionType::SingleChoice),
            "multiple_choice" => Ok(QuestionType::MultipleChoice),
            "true_false" => Ok(QuestionType::TrueFalse),
            "short_answer" => Ok(QuestionType::ShortAnswer),
            "essay" => Ok(QuestionType::Essay),
            "code" => Ok(QuestionType::Code),
            _ => Err(format!("Invalid question type: {}", s)),
        }
    }
}

/// Quiz submission status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubmissionStatus {
    /// Student is currently taking the quiz
    InProgress,
    /// Student has submitted the quiz
    Submitted,
    /// Quiz has been graded
    Graded,
}

impl Default for SubmissionStatus {
    fn default() -> Self {
        SubmissionStatus::InProgress
    }
}

impl std::fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmissionStatus::InProgress => write!(f, "in_progress"),
            SubmissionStatus::Submitted => write!(f, "submitted"),
            SubmissionStatus::Graded => write!(f, "graded"),
        }
    }
}

impl std::str::FromStr for SubmissionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in_progress" => Ok(SubmissionStatus::InProgress),
            "submitted" => Ok(SubmissionStatus::Submitted),
            "graded" => Ok(SubmissionStatus::Graded),
            _ => Err(format!("Invalid submission status: {}", s)),
        }
    }
}

// =============================================================================
// QUIZ
// =============================================================================

/// Quiz/assessment definition.
///
/// Represents a quiz that can be taken by students enrolled in a course.
///
/// # Database Mapping
///
/// Maps to `assessments.quizzes` table.
///
/// # Cross-Schema References
///
/// - `course_id`: References `courses.courses(course_id)`
/// - `lesson_id`: References `courses.lessons(lesson_id)` (optional)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quiz {
    /// Unique identifier
    pub quiz_id: Uuid,
    /// Course this quiz belongs to
    pub course_id: Uuid,
    /// Optional lesson this quiz is attached to
    pub lesson_id: Option<Uuid>,
    /// Quiz title
    pub title: String,
    /// Description of the quiz
    pub description: Option<String>,
    /// Instructions for taking the quiz
    pub instructions: Option<String>,
    /// Total possible points
    pub total_points: i32,
    /// Minimum percentage to pass (0-100)
    pub passing_score_percentage: f64,
    /// Time limit in minutes (None = unlimited)
    pub time_limit_minutes: Option<i32>,
    /// Maximum number of attempts allowed
    pub max_attempts: Option<i32>,
    /// Whether to randomize question order
    pub shuffle_questions: bool,
    /// Whether to show correct answers after submission
    pub show_correct_answers: bool,
    /// Whether the quiz is published and available
    pub is_published: bool,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Quiz {
    /// Returns true if the quiz is available for students.
    pub fn is_available(&self) -> bool {
        self.is_published
    }

    /// Returns true if the quiz has a time limit.
    pub fn has_time_limit(&self) -> bool {
        self.time_limit_minutes.is_some()
    }

    /// Calculates passing score from total points.
    pub fn passing_score(&self) -> f64 {
        (self.total_points as f64) * (self.passing_score_percentage / 100.0)
    }
}

/// Data required to create a new quiz.
#[derive(Debug, Clone, Deserialize)]
pub struct NewQuiz {
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub total_points: Option<i32>,
    pub passing_score_percentage: Option<f64>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub show_correct_answers: Option<bool>,
}

/// Data for updating a quiz.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateQuiz {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub instructions: Option<Option<String>>,
    pub total_points: Option<i32>,
    pub passing_score_percentage: Option<f64>,
    pub time_limit_minutes: Option<Option<i32>>,
    pub max_attempts: Option<Option<i32>>,
    pub shuffle_questions: Option<bool>,
    pub show_correct_answers: Option<bool>,
    pub is_published: Option<bool>,
}

// =============================================================================
// QUIZ QUESTION
// =============================================================================

/// A question within a quiz.
///
/// Supports multiple question types with flexible answer storage.
///
/// # Database Mapping
///
/// Maps to `assessments.quiz_questions` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizQuestion {
    /// Unique identifier
    pub question_id: Uuid,
    /// Parent quiz
    pub quiz_id: Uuid,
    /// Question text (supports markdown)
    pub question_text: String,
    /// Type of question
    pub question_type: QuestionType,
    /// Points for this question
    pub points: i32,
    /// Display order within quiz
    pub sort_order: i32,
    /// Explanation shown after answering
    pub explanation: Option<String>,
    /// Answer options as JSON array
    /// Format: [{"id": "a", "text": "Option A"}, ...]
    pub options: serde_json::Value,
    /// Correct answers as JSON array
    /// Format: ["a"] for single, ["a", "c"] for multiple
    pub correct_answers: serde_json::Value,
    /// Programming language for code questions
    pub code_language: Option<String>,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
}

impl QuizQuestion {
    /// Returns true if this question requires manual grading.
    pub fn needs_manual_grading(&self) -> bool {
        matches!(self.question_type, QuestionType::Essay | QuestionType::Code)
    }

    /// Returns the number of correct answers expected.
    pub fn expected_answer_count(&self) -> usize {
        self.correct_answers.as_array().map(|a| a.len()).unwrap_or(1)
    }
}

/// Data required to create a new question.
#[derive(Debug, Clone, Deserialize)]
pub struct NewQuizQuestion {
    pub quiz_id: Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub points: Option<i32>,
    pub sort_order: i32,
    pub explanation: Option<String>,
    pub options: serde_json::Value,
    pub correct_answers: serde_json::Value,
    pub code_language: Option<String>,
}

/// Data for updating a question.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateQuizQuestion {
    pub question_text: Option<String>,
    pub question_type: Option<QuestionType>,
    pub points: Option<i32>,
    pub sort_order: Option<i32>,
    pub explanation: Option<Option<String>>,
    pub options: Option<serde_json::Value>,
    pub correct_answers: Option<serde_json::Value>,
    pub code_language: Option<Option<String>>,
}

// =============================================================================
// QUIZ SUBMISSION
// =============================================================================

/// A student's quiz attempt.
///
/// Tracks the submission status, score, and timing.
///
/// # Database Mapping
///
/// Maps to `assessments.quiz_submissions` table.
///
/// # Cross-Schema References
///
/// - `user_id`: References `auth.users(user_id)`
/// - `enrollment_id`: References `enrollments.enrollments(enrollment_id)`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizSubmission {
    /// Unique identifier
    pub submission_id: Uuid,
    /// Quiz being taken
    pub quiz_id: Uuid,
    /// Student taking the quiz
    pub user_id: Uuid,
    /// Enrollment for access validation
    pub enrollment_id: Uuid,
    /// Which attempt this is (1, 2, etc.)
    pub attempt_number: i32,
    /// Current submission status
    pub status: SubmissionStatus,
    /// Score achieved
    pub score: f64,
    /// Maximum possible score
    pub max_score: f64,
    /// Whether the student passed
    pub passed: Option<bool>,
    /// Time spent in seconds
    pub time_spent_seconds: Option<i32>,
    /// When the attempt started
    pub started_at: DateTime<Utc>,
    /// When the attempt was submitted
    pub submitted_at: Option<DateTime<Utc>>,
    /// When the attempt was graded
    pub graded_at: Option<DateTime<Utc>>,
    /// Instructor feedback on submission
    pub instructor_feedback: Option<String>,
}

impl QuizSubmission {
    /// Returns true if the submission is complete.
    pub fn is_complete(&self) -> bool {
        self.status == SubmissionStatus::Submitted || self.status == SubmissionStatus::Graded
    }

    /// Returns true if the submission has been graded.
    pub fn is_graded(&self) -> bool {
        self.status == SubmissionStatus::Graded
    }

    /// Calculates score percentage.
    pub fn score_percentage(&self) -> f64 {
        if self.max_score > 0.0 {
            (self.score / self.max_score) * 100.0
        } else {
            0.0
        }
    }

    /// Returns formatted time spent (e.g., "15:30").
    pub fn formatted_time_spent(&self) -> String {
        let seconds = self.time_spent_seconds.unwrap_or(0);
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{}:{:02}", minutes, secs)
    }
}

/// Data required to start a new submission.
#[derive(Debug, Clone, Deserialize)]
pub struct NewQuizSubmission {
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub max_score: f64,
}

/// Data for updating a submission.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateQuizSubmission {
    pub status: Option<SubmissionStatus>,
    pub score: Option<f64>,
    pub passed: Option<Option<bool>>,
    pub time_spent_seconds: Option<i32>,
    pub submitted_at: Option<Option<DateTime<Utc>>>,
    pub graded_at: Option<Option<DateTime<Utc>>>,
    pub instructor_feedback: Option<Option<String>>,
}

// =============================================================================
// QUIZ RESPONSE
// =============================================================================

/// A student's response to a single question.
///
/// # Database Mapping
///
/// Maps to `assessments.quiz_responses` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizResponse {
    /// Unique identifier
    pub response_id: Uuid,
    /// Parent submission
    pub submission_id: Uuid,
    /// Question being answered
    pub question_id: Uuid,
    /// Student's answer as JSON
    /// Format varies by question type
    pub answer_data: serde_json::Value,
    /// Whether the answer was correct
    pub is_correct: Option<bool>,
    /// Points earned for this response
    pub points_earned: f64,
    /// Instructor feedback on this response
    pub instructor_feedback: Option<String>,
    /// Whether this was auto-graded
    pub auto_graded: bool,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
}

impl QuizResponse {
    /// Returns true if this response needs manual review.
    pub fn needs_review(&self) -> bool {
        !self.auto_graded && self.is_correct.is_none()
    }
}

/// Data required to create a new response.
#[derive(Debug, Clone, Deserialize)]
pub struct NewQuizResponse {
    pub submission_id: Uuid,
    pub question_id: Uuid,
    pub answer_data: serde_json::Value,
}

// =============================================================================
// AGGREGATES
// =============================================================================

/// Quiz with all its questions loaded.
#[derive(Debug, Clone, Serialize)]
pub struct QuizWithQuestions {
    /// The quiz entity
    #[serde(flatten)]
    pub quiz: Quiz,
    /// All questions in this quiz
    pub questions: Vec<QuizQuestion>,
}

impl QuizWithQuestions {
    /// Returns questions in sorted order.
    pub fn sorted_questions(&self) -> Vec<&QuizQuestion> {
        let mut questions: Vec<_> = self.questions.iter().collect();
        questions.sort_by_key(|q| q.sort_order);
        questions
    }

    /// Returns total question count.
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }
}

/// Submission with all responses loaded.
#[derive(Debug, Clone, Serialize)]
pub struct SubmissionWithResponses {
    /// The submission entity
    #[serde(flatten)]
    pub submission: QuizSubmission,
    /// All responses in this submission
    pub responses: Vec<QuizResponse>,
}

impl SubmissionWithResponses {
    /// Returns count of answered questions.
    pub fn answered_count(&self) -> usize {
        self.responses.len()
    }

    /// Returns count of correct answers.
    pub fn correct_count(&self) -> usize {
        self.responses.iter()
            .filter(|r| r.is_correct == Some(true))
            .count()
    }

    /// Returns count of responses needing manual review.
    pub fn pending_review_count(&self) -> usize {
        self.responses.iter()
            .filter(|r| r.needs_review())
            .count()
    }
}
