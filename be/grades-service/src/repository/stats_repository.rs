//! # Stats Repository
//!
//! Data access for instructor statistics and analytics.

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{GradeError, QuestionStats};

/// Repository for statistics data access.
pub struct StatsRepository {
    pool: PgPool,
}

impl StatsRepository {
    /// Create a new stats repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Verify instructor has access to a course.
    pub async fn verify_instructor_access(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<bool, GradeError> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM courses.courses
                WHERE course_id = $1 AND instructor_id = $2
            )
            "#
        )
        .bind(course_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.0)
    }

    /// Verify instructor has access to a quiz.
    pub async fn verify_quiz_instructor_access(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<bool, GradeError> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM assessments.quizzes q
                JOIN courses.courses c ON q.course_id = c.course_id
                WHERE q.quiz_id = $1 AND c.instructor_id = $2
            )
            "#
        )
        .bind(quiz_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.0)
    }

    /// Get course statistics overview.
    pub async fn get_course_stats_overview(
        &self,
        course_id: Uuid,
    ) -> Result<CourseStatsRow, GradeError> {
        let stats = sqlx::query_as::<_, CourseStatsRow>(
            r#"
            SELECT
                c.course_id,
                c.title as course_title,
                COUNT(DISTINCT e.user_id)::INT as total_students,
                COUNT(DISTINCT qs.user_id)::INT as students_with_submissions,
                COUNT(DISTINCT q.quiz_id)::INT as total_quizzes,
                COALESCE(AVG(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as average_score_percentage,
                COALESCE(
                    COUNT(*) FILTER (WHERE qs.passed = true)::DECIMAL /
                    NULLIF(COUNT(*) FILTER (WHERE qs.passed IS NOT NULL), 0) * 100,
                    0
                )::DECIMAL(5,2) as pass_rate
            FROM courses.courses c
            LEFT JOIN enrollments.enrollments e ON c.course_id = e.course_id AND e.status = 'active'
            LEFT JOIN assessments.quizzes q ON c.course_id = q.course_id AND q.is_published = true
            LEFT JOIN assessments.quiz_submissions qs ON q.quiz_id = qs.quiz_id AND qs.status = 'graded'
            WHERE c.course_id = $1
            GROUP BY c.course_id, c.title
            "#
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get quiz-level statistics for a course.
    pub async fn get_course_quiz_stats(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<QuizStatsRow>, GradeError> {
        let stats = sqlx::query_as::<_, QuizStatsRow>(
            r#"
            SELECT
                q.quiz_id,
                q.title as quiz_title,
                COUNT(qs.submission_id)::INT as total_submissions,
                COUNT(DISTINCT qs.user_id)::INT as unique_students,
                COALESCE(AVG(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as average_score,
                COALESCE(MAX(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as highest_score,
                COALESCE(MIN(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as lowest_score,
                COALESCE(
                    COUNT(*) FILTER (WHERE qs.passed = true)::DECIMAL /
                    NULLIF(COUNT(*) FILTER (WHERE qs.passed IS NOT NULL), 0) * 100,
                    0
                )::DECIMAL(5,2) as pass_rate,
                AVG(qs.time_spent_seconds)::INT as average_time_seconds,
                COALESCE(AVG(qs.attempt_number), 1)::DECIMAL(3,1) as average_attempts
            FROM assessments.quizzes q
            LEFT JOIN assessments.quiz_submissions qs ON q.quiz_id = qs.quiz_id AND qs.status = 'graded'
            WHERE q.course_id = $1 AND q.is_published = true
            GROUP BY q.quiz_id, q.title
            ORDER BY q.title
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get single quiz statistics.
    pub async fn get_quiz_stats(&self, quiz_id: Uuid) -> Result<QuizStatsRow, GradeError> {
        let stats = sqlx::query_as::<_, QuizStatsRow>(
            r#"
            SELECT
                q.quiz_id,
                q.title as quiz_title,
                COUNT(qs.submission_id)::INT as total_submissions,
                COUNT(DISTINCT qs.user_id)::INT as unique_students,
                COALESCE(AVG(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as average_score,
                COALESCE(MAX(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as highest_score,
                COALESCE(MIN(
                    CASE WHEN qs.max_score > 0
                    THEN (qs.score / qs.max_score * 100)
                    ELSE NULL END
                ), 0)::DECIMAL(5,2) as lowest_score,
                COALESCE(
                    COUNT(*) FILTER (WHERE qs.passed = true)::DECIMAL /
                    NULLIF(COUNT(*) FILTER (WHERE qs.passed IS NOT NULL), 0) * 100,
                    0
                )::DECIMAL(5,2) as pass_rate,
                AVG(qs.time_spent_seconds)::INT as average_time_seconds,
                COALESCE(AVG(qs.attempt_number), 1)::DECIMAL(3,1) as average_attempts
            FROM assessments.quizzes q
            LEFT JOIN assessments.quiz_submissions qs ON q.quiz_id = qs.quiz_id AND qs.status = 'graded'
            WHERE q.quiz_id = $1
            GROUP BY q.quiz_id, q.title
            "#
        )
        .bind(quiz_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get question-level statistics for a quiz.
    pub async fn get_quiz_question_stats(
        &self,
        quiz_id: Uuid,
    ) -> Result<Vec<QuestionStatsRow>, GradeError> {
        let stats = sqlx::query_as::<_, QuestionStatsRow>(
            r#"
            SELECT
                qq.question_id,
                qq.question_text,
                qq.question_type,
                COUNT(qr.response_id)::INT as total_responses,
                COUNT(*) FILTER (WHERE qr.is_correct = true)::INT as correct_responses,
                COALESCE(
                    COUNT(*) FILTER (WHERE qr.is_correct = true)::DECIMAL /
                    NULLIF(COUNT(qr.response_id), 0) * 100,
                    0
                )::DECIMAL(5,2) as correct_percentage
            FROM assessments.quiz_questions qq
            LEFT JOIN assessments.quiz_responses qr ON qq.question_id = qr.question_id
            WHERE qq.quiz_id = $1
            GROUP BY qq.question_id, qq.question_text, qq.question_type, qq.sort_order
            ORDER BY qq.sort_order
            "#
        )
        .bind(quiz_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get grade distribution for a course.
    pub async fn get_grade_distribution(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<GradePercentageRow>, GradeError> {
        let grades = sqlx::query_as::<_, GradePercentageRow>(
            r#"
            SELECT
                qs.user_id,
                CASE
                    WHEN SUM(qs.max_score) > 0
                    THEN (SUM(qs.score) / SUM(qs.max_score) * 100)::DECIMAL(5,2)
                    ELSE 0
                END as grade_percentage
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            WHERE q.course_id = $1 AND qs.status = 'graded'
            GROUP BY qs.user_id
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(grades)
    }
}

/// Course stats row.
#[derive(Debug, sqlx::FromRow)]
pub struct CourseStatsRow {
    pub course_id: Uuid,
    pub course_title: String,
    pub total_students: i32,
    pub students_with_submissions: i32,
    pub total_quizzes: i32,
    pub average_score_percentage: Decimal,
    pub pass_rate: Decimal,
}

/// Quiz stats row.
#[derive(Debug, sqlx::FromRow)]
pub struct QuizStatsRow {
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
}

/// Question stats row.
#[derive(Debug, sqlx::FromRow)]
pub struct QuestionStatsRow {
    pub question_id: Uuid,
    pub question_text: String,
    pub question_type: String,
    pub total_responses: i32,
    pub correct_responses: i32,
    pub correct_percentage: Decimal,
}

impl From<QuestionStatsRow> for QuestionStats {
    fn from(row: QuestionStatsRow) -> Self {
        QuestionStats {
            question_id: row.question_id,
            question_text: row.question_text,
            question_type: row.question_type,
            total_responses: row.total_responses,
            correct_responses: row.correct_responses,
            correct_percentage: row.correct_percentage,
            common_mistake: None, // Could be calculated with additional query
        }
    }
}

/// Grade percentage row for distribution.
#[derive(Debug, sqlx::FromRow)]
pub struct GradePercentageRow {
    pub user_id: Uuid,
    pub grade_percentage: Decimal,
}
