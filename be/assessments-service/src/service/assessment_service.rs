//! # Assessment Service
//!
//! Business logic for quiz management, submissions, and grading.
//!
//! ## Responsibilities
//!
//! - Quiz CRUD with authorization
//! - Question management
//! - Submission lifecycle (start, answer, submit)
//! - Auto-grading for objective questions
//! - Manual grading workflow

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    NewQuiz, NewQuizQuestion, NewQuizResponse, NewQuizSubmission,
    Quiz, QuizQuestion, QuestionType, QuizResponse, QuizSubmission,
    QuizWithQuestions, SubmissionStatus, SubmissionWithResponses,
    UpdateQuiz, UpdateQuizQuestion, UpdateQuizSubmission,
};
use crate::repository::{AssessmentRepository, QuizStats};

/// Assessment service errors.
#[derive(Debug, thiserror::Error)]
pub enum AssessmentError {
    #[error("Quiz not found")]
    QuizNotFound,

    #[error("Question not found")]
    QuestionNotFound,

    #[error("Submission not found")]
    SubmissionNotFound,

    #[error("Quiz is not published")]
    QuizNotPublished,

    #[error("Maximum attempts exceeded")]
    MaxAttemptsExceeded,

    #[error("Submission already completed")]
    SubmissionAlreadyCompleted,

    #[error("Cannot modify published quiz with submissions")]
    QuizHasSubmissions,

    #[error("Not authorized to access this resource")]
    Unauthorized,

    #[error("Time limit exceeded")]
    TimeLimitExceeded,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Result type for assessment operations.
pub type AssessmentResult<T> = Result<T, AssessmentError>;

/// Service for assessment business logic.
#[derive(Debug, Clone)]
pub struct AssessmentService {
    repository: Arc<AssessmentRepository>,
}

impl AssessmentService {
    /// Creates a new assessment service.
    pub fn new(repository: Arc<AssessmentRepository>) -> Self {
        Self { repository }
    }

    // =========================================================================
    // QUIZ OPERATIONS
    // =========================================================================

    /// Lists quizzes for a course.
    pub async fn list_course_quizzes(
        &self,
        course_id: Uuid,
        is_instructor: bool,
        page: i64,
        page_size: i64,
    ) -> AssessmentResult<Vec<Quiz>> {
        let offset = (page - 1) * page_size;

        self.repository
            .list_quizzes_by_course(course_id, is_instructor, page_size, offset)
            .await
            .map_err(Into::into)
    }

    /// Gets a quiz by ID.
    pub async fn get_quiz(&self, quiz_id: Uuid) -> AssessmentResult<Quiz> {
        self.repository
            .find_quiz_by_id(quiz_id)
            .await?
            .ok_or(AssessmentError::QuizNotFound)
    }

    /// Gets a quiz with all questions.
    pub async fn get_quiz_with_questions(
        &self,
        quiz_id: Uuid,
        include_answers: bool,
    ) -> AssessmentResult<QuizWithQuestions> {
        let quiz = self.get_quiz(quiz_id).await?;
        let mut questions = self.repository.list_questions_by_quiz(quiz_id).await?;

        // Hide correct answers for students
        if !include_answers {
            for q in &mut questions {
                q.correct_answers = serde_json::json!([]);
            }
        }

        Ok(QuizWithQuestions { quiz, questions })
    }

    /// Gets quizzes for a lesson.
    pub async fn get_lesson_quizzes(&self, lesson_id: Uuid) -> AssessmentResult<Vec<Quiz>> {
        self.repository
            .find_quizzes_by_lesson(lesson_id)
            .await
            .map_err(Into::into)
    }

    /// Creates a new quiz.
    pub async fn create_quiz(
        &self,
        data: NewQuiz,
        _creator_id: Uuid,
    ) -> AssessmentResult<Quiz> {
        // TODO: Validate course exists and user has permission

        self.repository.create_quiz(data).await.map_err(Into::into)
    }

    /// Updates a quiz.
    pub async fn update_quiz(
        &self,
        quiz_id: Uuid,
        data: UpdateQuiz,
    ) -> AssessmentResult<Quiz> {
        // Verify quiz exists
        let _quiz = self.get_quiz(quiz_id).await?;

        // TODO: Check for existing submissions if unpublishing

        self.repository.update_quiz(quiz_id, data).await.map_err(Into::into)
    }

    /// Publishes a quiz.
    pub async fn publish_quiz(&self, quiz_id: Uuid) -> AssessmentResult<Quiz> {
        let quiz = self.get_quiz(quiz_id).await?;

        // Validate quiz has questions
        let questions = self.repository.list_questions_by_quiz(quiz_id).await?;
        if questions.is_empty() {
            return Err(AssessmentError::Validation(
                "Cannot publish quiz without questions".into()
            ));
        }

        // Validate total points match
        let question_points: i32 = questions.iter().map(|q| q.points).sum();
        if question_points != quiz.total_points {
            return Err(AssessmentError::Validation(
                format!(
                    "Question points ({}) don't match quiz total ({})",
                    question_points, quiz.total_points
                )
            ));
        }

        self.repository.publish_quiz(quiz_id).await.map_err(Into::into)
    }

    /// Deletes a quiz.
    pub async fn delete_quiz(&self, quiz_id: Uuid) -> AssessmentResult<bool> {
        // Verify quiz exists
        let _quiz = self.get_quiz(quiz_id).await?;

        // TODO: Check if quiz has submissions

        self.repository.delete_quiz(quiz_id).await.map_err(Into::into)
    }

    // =========================================================================
    // QUESTION OPERATIONS
    // =========================================================================

    /// Gets questions for a quiz.
    pub async fn get_quiz_questions(&self, quiz_id: Uuid) -> AssessmentResult<Vec<QuizQuestion>> {
        self.repository
            .list_questions_by_quiz(quiz_id)
            .await
            .map_err(Into::into)
    }

    /// Gets a question by ID.
    pub async fn get_question(&self, question_id: Uuid) -> AssessmentResult<QuizQuestion> {
        self.repository
            .find_question_by_id(question_id)
            .await?
            .ok_or(AssessmentError::QuestionNotFound)
    }

    /// Adds a question to a quiz.
    pub async fn add_question(&self, data: NewQuizQuestion) -> AssessmentResult<QuizQuestion> {
        // Verify quiz exists and is not published (or allow if no submissions)
        let quiz = self.get_quiz(data.quiz_id).await?;

        if quiz.is_published {
            let stats = self.repository.get_quiz_stats(quiz.quiz_id).await?;
            if stats.total_submissions > 0 {
                return Err(AssessmentError::QuizHasSubmissions);
            }
        }

        self.repository.create_question(data).await.map_err(Into::into)
    }

    /// Updates a question.
    pub async fn update_question(
        &self,
        question_id: Uuid,
        data: UpdateQuizQuestion,
    ) -> AssessmentResult<QuizQuestion> {
        let question = self.get_question(question_id).await?;

        // Check if parent quiz has submissions
        let quiz = self.get_quiz(question.quiz_id).await?;
        if quiz.is_published {
            let stats = self.repository.get_quiz_stats(quiz.quiz_id).await?;
            if stats.total_submissions > 0 {
                return Err(AssessmentError::QuizHasSubmissions);
            }
        }

        self.repository
            .update_question(question_id, data)
            .await
            .map_err(Into::into)
    }

    /// Removes a question from a quiz.
    pub async fn remove_question(&self, question_id: Uuid) -> AssessmentResult<bool> {
        let question = self.get_question(question_id).await?;

        // Check if parent quiz has submissions
        let quiz = self.get_quiz(question.quiz_id).await?;
        if quiz.is_published {
            let stats = self.repository.get_quiz_stats(quiz.quiz_id).await?;
            if stats.total_submissions > 0 {
                return Err(AssessmentError::QuizHasSubmissions);
            }
        }

        self.repository.delete_question(question_id).await.map_err(Into::into)
    }

    // =========================================================================
    // SUBMISSION OPERATIONS
    // =========================================================================

    /// Starts a quiz attempt.
    pub async fn start_quiz(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        enrollment_id: Uuid,
    ) -> AssessmentResult<QuizSubmission> {
        let quiz = self.get_quiz(quiz_id).await?;

        // Check quiz is published
        if !quiz.is_published {
            return Err(AssessmentError::QuizNotPublished);
        }

        // Check max attempts
        if let Some(max) = quiz.max_attempts {
            let attempts = self.repository.get_attempt_count(user_id, quiz_id).await?;
            if attempts >= max {
                return Err(AssessmentError::MaxAttemptsExceeded);
            }
        }

        // TODO: Validate enrollment is active

        let data = NewQuizSubmission {
            quiz_id,
            user_id,
            enrollment_id,
            max_score: quiz.total_points as f64,
        };

        self.repository.create_submission(data).await.map_err(Into::into)
    }

    /// Gets a submission by ID.
    pub async fn get_submission(
        &self,
        submission_id: Uuid,
        user_id: Uuid,
        is_instructor: bool,
    ) -> AssessmentResult<QuizSubmission> {
        let submission = self.repository
            .find_submission_by_id(submission_id)
            .await?
            .ok_or(AssessmentError::SubmissionNotFound)?;

        // Authorization: students can only see their own submissions
        if submission.user_id != user_id && !is_instructor {
            return Err(AssessmentError::Unauthorized);
        }

        Ok(submission)
    }

    /// Gets a submission with all responses.
    pub async fn get_submission_with_responses(
        &self,
        submission_id: Uuid,
        user_id: Uuid,
        is_instructor: bool,
    ) -> AssessmentResult<SubmissionWithResponses> {
        let submission = self.get_submission(submission_id, user_id, is_instructor).await?;
        let responses = self.repository.list_responses_by_submission(submission_id).await?;

        Ok(SubmissionWithResponses { submission, responses })
    }

    /// Gets user's submissions for a quiz.
    pub async fn get_user_submissions(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> AssessmentResult<Vec<QuizSubmission>> {
        self.repository
            .list_user_submissions(user_id, quiz_id)
            .await
            .map_err(Into::into)
    }

    /// Saves an answer for a question.
    pub async fn save_answer(
        &self,
        submission_id: Uuid,
        question_id: Uuid,
        answer_data: serde_json::Value,
        user_id: Uuid,
    ) -> AssessmentResult<QuizResponse> {
        // Verify submission belongs to user and is in progress
        let submission = self.repository
            .find_submission_by_id(submission_id)
            .await?
            .ok_or(AssessmentError::SubmissionNotFound)?;

        if submission.user_id != user_id {
            return Err(AssessmentError::Unauthorized);
        }

        if submission.status != SubmissionStatus::InProgress {
            return Err(AssessmentError::SubmissionAlreadyCompleted);
        }

        // TODO: Check time limit

        let data = NewQuizResponse {
            submission_id,
            question_id,
            answer_data,
        };

        self.repository.upsert_response(data).await.map_err(Into::into)
    }

    /// Submits a quiz attempt for grading.
    pub async fn submit_quiz(
        &self,
        submission_id: Uuid,
        user_id: Uuid,
        time_spent_seconds: i32,
    ) -> AssessmentResult<QuizSubmission> {
        // Verify submission
        let submission = self.repository
            .find_submission_by_id(submission_id)
            .await?
            .ok_or(AssessmentError::SubmissionNotFound)?;

        if submission.user_id != user_id {
            return Err(AssessmentError::Unauthorized);
        }

        if submission.status != SubmissionStatus::InProgress {
            return Err(AssessmentError::SubmissionAlreadyCompleted);
        }

        // Submit the attempt
        let submitted = self.repository
            .submit_quiz(submission_id, time_spent_seconds)
            .await?;

        // Auto-grade if possible
        let graded = self.auto_grade_submission(submission_id).await?;

        Ok(graded.unwrap_or(submitted))
    }

    /// Auto-grades a submission.
    async fn auto_grade_submission(
        &self,
        submission_id: Uuid,
    ) -> AssessmentResult<Option<QuizSubmission>> {
        let submission = self.repository
            .find_submission_by_id(submission_id)
            .await?
            .ok_or(AssessmentError::SubmissionNotFound)?;

        let quiz = self.get_quiz(submission.quiz_id).await?;
        let questions = self.repository.list_questions_by_quiz(quiz.quiz_id).await?;
        let responses = self.repository.list_responses_by_submission(submission_id).await?;

        let mut total_score = 0.0;
        let mut needs_manual_grading = false;

        for response in &responses {
            let question = questions.iter()
                .find(|q| q.question_id == response.question_id);

            if let Some(q) = question {
                if q.needs_manual_grading() {
                    needs_manual_grading = true;
                    continue;
                }

                // Auto-grade based on question type
                let (is_correct, points) = self.auto_grade_response(q, &response.answer_data);

                self.repository.grade_response(
                    response.response_id,
                    is_correct,
                    points,
                    None,
                    true, // auto_graded
                ).await?;

                total_score += points;
            }
        }

        // If all questions auto-graded, mark submission as graded
        if !needs_manual_grading {
            let passed = (total_score / submission.max_score) * 100.0 >= quiz.passing_score_percentage;

            let graded = self.repository
                .grade_submission(submission_id, total_score, passed, None)
                .await?;

            return Ok(Some(graded));
        }

        Ok(None)
    }

    /// Auto-grades a single response based on question type (internal helper).
    fn auto_grade_response(&self, question: &QuizQuestion, answer: &serde_json::Value) -> (bool, f64) {
        let correct_answers = &question.correct_answers;

        match question.question_type {
            QuestionType::SingleChoice | QuestionType::TrueFalse => {
                // Compare single answer
                let is_correct = answer == correct_answers.get(0).unwrap_or(&serde_json::json!(null));
                let points = if is_correct { question.points as f64 } else { 0.0 };
                (is_correct, points)
            }
            QuestionType::MultipleChoice => {
                // Compare all selected answers
                let user_answers: Vec<String> = answer.as_array()
                    .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();
                let correct: Vec<String> = correct_answers.as_array()
                    .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();

                let mut user_set: std::collections::HashSet<_> = user_answers.iter().collect();
                let correct_set: std::collections::HashSet<_> = correct.iter().collect();

                let is_correct = user_set == correct_set;
                let points = if is_correct { question.points as f64 } else { 0.0 };
                (is_correct, points)
            }
            QuestionType::ShortAnswer => {
                // Case-insensitive comparison
                let user_answer = answer.as_str().unwrap_or("").to_lowercase().trim().to_string();
                let is_correct = correct_answers.as_array()
                    .map(|arr| arr.iter().any(|a| {
                        a.as_str().map(|s| s.to_lowercase().trim() == user_answer).unwrap_or(false)
                    }))
                    .unwrap_or(false);
                let points = if is_correct { question.points as f64 } else { 0.0 };
                (is_correct, points)
            }
            QuestionType::Essay | QuestionType::Code => {
                // Requires manual grading
                (false, 0.0)
            }
        }
    }

    /// Manually grades a submission (instructor).
    pub async fn grade_submission(
        &self,
        submission_id: Uuid,
        score: f64,
        feedback: Option<String>,
        passing_percentage: f64,
    ) -> AssessmentResult<QuizSubmission> {
        let submission = self.repository
            .find_submission_by_id(submission_id)
            .await?
            .ok_or(AssessmentError::SubmissionNotFound)?;

        let passed = (score / submission.max_score) * 100.0 >= passing_percentage;

        self.repository
            .grade_submission(submission_id, score, passed, feedback)
            .await
            .map_err(Into::into)
    }

    /// Grades a single response (instructor).
    pub async fn grade_single_response(
        &self,
        response_id: Uuid,
        points_earned: f64,
        feedback: Option<String>,
    ) -> AssessmentResult<QuizResponse> {
        // TODO: Get question to validate max points

        self.repository
            .grade_response(response_id, points_earned > 0.0, points_earned, feedback, false)
            .await
            .map_err(Into::into)
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Gets quiz statistics.
    pub async fn get_quiz_stats(&self, quiz_id: Uuid) -> AssessmentResult<QuizStats> {
        self.repository.get_quiz_stats(quiz_id).await.map_err(Into::into)
    }

    /// Gets pending grading count for a course.
    pub async fn get_pending_grading_count(&self, course_id: Uuid) -> AssessmentResult<i64> {
        self.repository.get_pending_grading_count(course_id).await.map_err(Into::into)
    }
}
