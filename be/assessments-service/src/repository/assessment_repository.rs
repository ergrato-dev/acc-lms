//! # Assessment Repository
//!
//! PostgreSQL data access for quizzes, questions, submissions, and responses.
//!
//! ## Schema
//!
//! This repository operates on the `assessments` schema:
//! - `assessments.quizzes`: Quiz definitions
//! - `assessments.quiz_questions`: Questions for quizzes
//! - `assessments.quiz_submissions`: Student quiz attempts
//! - `assessments.quiz_responses`: Individual question responses
//!
//! ## Cross-Schema Access
//!
//! Has SELECT permission on `courses` and `enrollments` schemas for validation.

use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::domain::{
    NewQuiz, NewQuizQuestion, NewQuizResponse, NewQuizSubmission,
    Quiz, QuizQuestion, QuizResponse, QuizSubmission,
    SubmissionStatus, UpdateQuiz, UpdateQuizQuestion, UpdateQuizSubmission,
};

/// Repository for assessment data access.
#[derive(Debug, Clone)]
pub struct AssessmentRepository {
    pool: PgPool,
}

impl AssessmentRepository {
    /// Creates a new assessment repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // QUIZ QUERIES
    // =========================================================================

    /// Lists quizzes for a course.
    pub async fn list_quizzes_by_course(
        &self,
        course_id: Uuid,
        include_unpublished: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Quiz>, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            SELECT
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            FROM assessments.quizzes
            WHERE course_id = $1
            AND ($2 OR is_published = TRUE)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(course_id)
        .bind(include_unpublished)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds a quiz by ID.
    pub async fn find_quiz_by_id(&self, quiz_id: Uuid) -> Result<Option<Quiz>, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            SELECT
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            FROM assessments.quizzes
            WHERE quiz_id = $1
            "#,
        )
        .bind(quiz_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Finds quizzes by lesson.
    pub async fn find_quizzes_by_lesson(&self, lesson_id: Uuid) -> Result<Vec<Quiz>, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            SELECT
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            FROM assessments.quizzes
            WHERE lesson_id = $1 AND is_published = TRUE
            ORDER BY created_at ASC
            "#,
        )
        .bind(lesson_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Creates a new quiz.
    pub async fn create_quiz(&self, data: NewQuiz) -> Result<Quiz, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            INSERT INTO assessments.quizzes (
                course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            "#,
        )
        .bind(data.course_id)
        .bind(data.lesson_id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(&data.instructions)
        .bind(data.total_points.unwrap_or(100))
        .bind(data.passing_score_percentage.unwrap_or(70.0))
        .bind(data.time_limit_minutes)
        .bind(data.max_attempts)
        .bind(data.shuffle_questions.unwrap_or(false))
        .bind(data.show_correct_answers.unwrap_or(true))
        .fetch_one(&self.pool)
        .await
    }

    /// Updates a quiz.
    pub async fn update_quiz(&self, quiz_id: Uuid, data: UpdateQuiz) -> Result<Quiz, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            UPDATE assessments.quizzes
            SET
                title = COALESCE($2, title),
                description = CASE WHEN $3 THEN $4 ELSE description END,
                instructions = CASE WHEN $5 THEN $6 ELSE instructions END,
                total_points = COALESCE($7, total_points),
                passing_score_percentage = COALESCE($8, passing_score_percentage),
                time_limit_minutes = CASE WHEN $9 THEN $10 ELSE time_limit_minutes END,
                max_attempts = CASE WHEN $11 THEN $12 ELSE max_attempts END,
                shuffle_questions = COALESCE($13, shuffle_questions),
                show_correct_answers = COALESCE($14, show_correct_answers),
                is_published = COALESCE($15, is_published),
                updated_at = NOW()
            WHERE quiz_id = $1
            RETURNING
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            "#,
        )
        .bind(quiz_id)
        .bind(&data.title)
        .bind(data.description.is_some())
        .bind(data.description.flatten())
        .bind(data.instructions.is_some())
        .bind(data.instructions.flatten())
        .bind(data.total_points)
        .bind(data.passing_score_percentage)
        .bind(data.time_limit_minutes.is_some())
        .bind(data.time_limit_minutes.flatten())
        .bind(data.max_attempts.is_some())
        .bind(data.max_attempts.flatten())
        .bind(data.shuffle_questions)
        .bind(data.show_correct_answers)
        .bind(data.is_published)
        .fetch_one(&self.pool)
        .await
    }

    /// Deletes a quiz.
    pub async fn delete_quiz(&self, quiz_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM assessments.quizzes WHERE quiz_id = $1"
        )
        .bind(quiz_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Publishes a quiz.
    pub async fn publish_quiz(&self, quiz_id: Uuid) -> Result<Quiz, sqlx::Error> {
        sqlx::query_as::<_, Quiz>(
            r#"
            UPDATE assessments.quizzes
            SET is_published = TRUE, updated_at = NOW()
            WHERE quiz_id = $1
            RETURNING
                quiz_id, course_id, lesson_id, title, description, instructions,
                total_points, passing_score_percentage, time_limit_minutes,
                max_attempts, shuffle_questions, show_correct_answers,
                is_published, created_at, updated_at
            "#,
        )
        .bind(quiz_id)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // QUESTION QUERIES
    // =========================================================================

    /// Lists questions for a quiz.
    pub async fn list_questions_by_quiz(&self, quiz_id: Uuid) -> Result<Vec<QuizQuestion>, sqlx::Error> {
        sqlx::query_as::<_, QuizQuestion>(
            r#"
            SELECT
                question_id, quiz_id, question_text, question_type,
                points, sort_order, explanation, options, correct_answers,
                code_language, created_at
            FROM assessments.quiz_questions
            WHERE quiz_id = $1
            ORDER BY sort_order ASC
            "#,
        )
        .bind(quiz_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds a question by ID.
    pub async fn find_question_by_id(&self, question_id: Uuid) -> Result<Option<QuizQuestion>, sqlx::Error> {
        sqlx::query_as::<_, QuizQuestion>(
            r#"
            SELECT
                question_id, quiz_id, question_text, question_type,
                points, sort_order, explanation, options, correct_answers,
                code_language, created_at
            FROM assessments.quiz_questions
            WHERE question_id = $1
            "#,
        )
        .bind(question_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates a new question.
    pub async fn create_question(&self, data: NewQuizQuestion) -> Result<QuizQuestion, sqlx::Error> {
        sqlx::query_as::<_, QuizQuestion>(
            r#"
            INSERT INTO assessments.quiz_questions (
                quiz_id, question_text, question_type, points, sort_order,
                explanation, options, correct_answers, code_language
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                question_id, quiz_id, question_text, question_type,
                points, sort_order, explanation, options, correct_answers,
                code_language, created_at
            "#,
        )
        .bind(data.quiz_id)
        .bind(&data.question_text)
        .bind(data.question_type.to_string())
        .bind(data.points.unwrap_or(5))
        .bind(data.sort_order)
        .bind(&data.explanation)
        .bind(&data.options)
        .bind(&data.correct_answers)
        .bind(&data.code_language)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates a question.
    pub async fn update_question(&self, question_id: Uuid, data: UpdateQuizQuestion) -> Result<QuizQuestion, sqlx::Error> {
        sqlx::query_as::<_, QuizQuestion>(
            r#"
            UPDATE assessments.quiz_questions
            SET
                question_text = COALESCE($2, question_text),
                question_type = COALESCE($3, question_type),
                points = COALESCE($4, points),
                sort_order = COALESCE($5, sort_order),
                explanation = CASE WHEN $6 THEN $7 ELSE explanation END,
                options = COALESCE($8, options),
                correct_answers = COALESCE($9, correct_answers),
                code_language = CASE WHEN $10 THEN $11 ELSE code_language END
            WHERE question_id = $1
            RETURNING
                question_id, quiz_id, question_text, question_type,
                points, sort_order, explanation, options, correct_answers,
                code_language, created_at
            "#,
        )
        .bind(question_id)
        .bind(&data.question_text)
        .bind(data.question_type.map(|t| t.to_string()))
        .bind(data.points)
        .bind(data.sort_order)
        .bind(data.explanation.is_some())
        .bind(data.explanation.flatten())
        .bind(&data.options)
        .bind(&data.correct_answers)
        .bind(data.code_language.is_some())
        .bind(data.code_language.flatten())
        .fetch_one(&self.pool)
        .await
    }

    /// Deletes a question.
    pub async fn delete_question(&self, question_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM assessments.quiz_questions WHERE question_id = $1"
        )
        .bind(question_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // SUBMISSION QUERIES
    // =========================================================================

    /// Lists submissions for a quiz.
    pub async fn list_submissions_by_quiz(
        &self,
        quiz_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<QuizSubmission>, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            SELECT
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            FROM assessments.quiz_submissions
            WHERE quiz_id = $1
            ORDER BY started_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(quiz_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Lists submissions by user for a quiz.
    pub async fn list_user_submissions(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<Vec<QuizSubmission>, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            SELECT
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            FROM assessments.quiz_submissions
            WHERE user_id = $1 AND quiz_id = $2
            ORDER BY attempt_number ASC
            "#,
        )
        .bind(user_id)
        .bind(quiz_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds a submission by ID.
    pub async fn find_submission_by_id(&self, submission_id: Uuid) -> Result<Option<QuizSubmission>, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            SELECT
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            FROM assessments.quiz_submissions
            WHERE submission_id = $1
            "#,
        )
        .bind(submission_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Gets the current attempt number for a user on a quiz.
    pub async fn get_attempt_count(&self, user_id: Uuid, quiz_id: Uuid) -> Result<i32, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM assessments.quiz_submissions
            WHERE user_id = $1 AND quiz_id = $2
            "#,
        )
        .bind(user_id)
        .bind(quiz_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0 as i32)
    }

    /// Creates a new submission (starts a quiz attempt).
    pub async fn create_submission(&self, data: NewQuizSubmission) -> Result<QuizSubmission, sqlx::Error> {
        // Get next attempt number
        let attempt = self.get_attempt_count(data.user_id, data.quiz_id).await? + 1;

        sqlx::query_as::<_, QuizSubmission>(
            r#"
            INSERT INTO assessments.quiz_submissions (
                quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score
            )
            VALUES ($1, $2, $3, $4, 'in_progress', 0, $5)
            RETURNING
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            "#,
        )
        .bind(data.quiz_id)
        .bind(data.user_id)
        .bind(data.enrollment_id)
        .bind(attempt)
        .bind(data.max_score)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates a submission.
    pub async fn update_submission(
        &self,
        submission_id: Uuid,
        data: UpdateQuizSubmission,
    ) -> Result<QuizSubmission, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            UPDATE assessments.quiz_submissions
            SET
                status = COALESCE($2, status),
                score = COALESCE($3, score),
                passed = CASE WHEN $4 THEN $5 ELSE passed END,
                time_spent_seconds = COALESCE($6, time_spent_seconds),
                submitted_at = CASE WHEN $7 THEN $8 ELSE submitted_at END,
                graded_at = CASE WHEN $9 THEN $10 ELSE graded_at END,
                instructor_feedback = CASE WHEN $11 THEN $12 ELSE instructor_feedback END
            WHERE submission_id = $1
            RETURNING
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            "#,
        )
        .bind(submission_id)
        .bind(data.status.map(|s| s.to_string()))
        .bind(data.score)
        .bind(data.passed.is_some())
        .bind(data.passed.flatten())
        .bind(data.time_spent_seconds)
        .bind(data.submitted_at.is_some())
        .bind(data.submitted_at.flatten())
        .bind(data.graded_at.is_some())
        .bind(data.graded_at.flatten())
        .bind(data.instructor_feedback.is_some())
        .bind(data.instructor_feedback.flatten())
        .fetch_one(&self.pool)
        .await
    }

    /// Submits a quiz attempt.
    pub async fn submit_quiz(
        &self,
        submission_id: Uuid,
        time_spent_seconds: i32,
    ) -> Result<QuizSubmission, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            UPDATE assessments.quiz_submissions
            SET
                status = 'submitted',
                time_spent_seconds = $2,
                submitted_at = NOW()
            WHERE submission_id = $1
            RETURNING
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            "#,
        )
        .bind(submission_id)
        .bind(time_spent_seconds)
        .fetch_one(&self.pool)
        .await
    }

    /// Grades a submission.
    pub async fn grade_submission(
        &self,
        submission_id: Uuid,
        score: f64,
        passed: bool,
        feedback: Option<String>,
    ) -> Result<QuizSubmission, sqlx::Error> {
        sqlx::query_as::<_, QuizSubmission>(
            r#"
            UPDATE assessments.quiz_submissions
            SET
                status = 'graded',
                score = $2,
                passed = $3,
                instructor_feedback = $4,
                graded_at = NOW()
            WHERE submission_id = $1
            RETURNING
                submission_id, quiz_id, user_id, enrollment_id, attempt_number,
                status, score, max_score, passed, time_spent_seconds,
                started_at, submitted_at, graded_at, instructor_feedback
            "#,
        )
        .bind(submission_id)
        .bind(score)
        .bind(passed)
        .bind(feedback)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // RESPONSE QUERIES
    // =========================================================================

    /// Lists responses for a submission.
    pub async fn list_responses_by_submission(
        &self,
        submission_id: Uuid,
    ) -> Result<Vec<QuizResponse>, sqlx::Error> {
        sqlx::query_as::<_, QuizResponse>(
            r#"
            SELECT
                response_id, submission_id, question_id, answer_data,
                is_correct, points_earned, instructor_feedback, auto_graded, created_at
            FROM assessments.quiz_responses
            WHERE submission_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(submission_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Creates or updates a response (upsert).
    pub async fn upsert_response(&self, data: NewQuizResponse) -> Result<QuizResponse, sqlx::Error> {
        sqlx::query_as::<_, QuizResponse>(
            r#"
            INSERT INTO assessments.quiz_responses (
                submission_id, question_id, answer_data
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (submission_id, question_id) DO UPDATE
            SET answer_data = $3
            RETURNING
                response_id, submission_id, question_id, answer_data,
                is_correct, points_earned, instructor_feedback, auto_graded, created_at
            "#,
        )
        .bind(data.submission_id)
        .bind(data.question_id)
        .bind(&data.answer_data)
        .fetch_one(&self.pool)
        .await
    }

    /// Grades a response.
    pub async fn grade_response(
        &self,
        response_id: Uuid,
        is_correct: bool,
        points_earned: f64,
        feedback: Option<String>,
        auto_graded: bool,
    ) -> Result<QuizResponse, sqlx::Error> {
        sqlx::query_as::<_, QuizResponse>(
            r#"
            UPDATE assessments.quiz_responses
            SET
                is_correct = $2,
                points_earned = $3,
                instructor_feedback = $4,
                auto_graded = $5
            WHERE response_id = $1
            RETURNING
                response_id, submission_id, question_id, answer_data,
                is_correct, points_earned, instructor_feedback, auto_graded, created_at
            "#,
        )
        .bind(response_id)
        .bind(is_correct)
        .bind(points_earned)
        .bind(feedback)
        .bind(auto_graded)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Gets quiz statistics.
    pub async fn get_quiz_stats(&self, quiz_id: Uuid) -> Result<QuizStats, sqlx::Error> {
        sqlx::query_as::<_, QuizStats>(
            r#"
            SELECT
                COUNT(*) as total_submissions,
                COUNT(*) FILTER (WHERE status = 'graded') as graded_count,
                COUNT(*) FILTER (WHERE passed = TRUE) as passed_count,
                COALESCE(AVG(score) FILTER (WHERE status = 'graded'), 0) as avg_score,
                COALESCE(AVG(time_spent_seconds) FILTER (WHERE status != 'in_progress'), 0) as avg_time_seconds
            FROM assessments.quiz_submissions
            WHERE quiz_id = $1
            "#,
        )
        .bind(quiz_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Gets submission statistics for grading queue.
    pub async fn get_pending_grading_count(&self, course_id: Uuid) -> Result<i64, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM assessments.quiz_submissions s
            JOIN assessments.quizzes q ON s.quiz_id = q.quiz_id
            WHERE q.course_id = $1 AND s.status = 'submitted'
            "#,
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }
}

/// Statistics for a quiz.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuizStats {
    pub total_submissions: i64,
    pub graded_count: i64,
    pub passed_count: i64,
    pub avg_score: f64,
    pub avg_time_seconds: f64,
}

/// Statistics for submissions.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SubmissionStats {
    pub total_responses: i64,
    pub correct_count: i64,
    pub pending_review: i64,
}
