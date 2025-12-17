//! # Grade Repository
//!
//! Data access for grade entries and aggregations.

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{GradeEntry, GradeError, GradeFilter, Pagination};

/// Repository for grade data access.
pub struct GradeRepository {
    pool: PgPool,
}

impl GradeRepository {
    /// Create a new grade repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all grade entries for a user.
    pub async fn get_user_grades(
        &self,
        user_id: Uuid,
        filter: &GradeFilter,
        pagination: &Pagination,
    ) -> Result<Vec<GradeEntry>, GradeError> {
        let mut query = String::from(
            r#"
            SELECT
                qs.submission_id,
                qs.quiz_id,
                qs.user_id,
                qs.enrollment_id,
                q.course_id,
                q.title as quiz_title,
                qs.score,
                qs.max_score,
                CASE
                    WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                    ELSE 0
                END as percentage,
                qs.passed,
                qs.attempt_number,
                qs.status,
                qs.submitted_at,
                qs.graded_at,
                qs.instructor_feedback
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            WHERE qs.user_id = $1
            "#
        );

        // Add filter conditions
        let mut param_count = 1;
        let mut params: Vec<String> = vec![];

        if let Some(course_id) = filter.course_id {
            param_count += 1;
            query.push_str(&format!(" AND q.course_id = ${}", param_count));
            params.push(course_id.to_string());
        }

        if let Some(ref status) = filter.status {
            param_count += 1;
            query.push_str(&format!(" AND qs.status = ${}", param_count));
            params.push(status.clone());
        }

        if let Some(passed) = filter.passed {
            param_count += 1;
            query.push_str(&format!(" AND qs.passed = ${}", param_count));
            params.push(passed.to_string());
        }

        query.push_str(" ORDER BY qs.submitted_at DESC NULLS LAST");
        query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.limit(), pagination.offset()));

        // Execute query with dynamic parameters
        // For simplicity, we'll use a base query for now
        let grades = sqlx::query_as::<_, GradeEntry>(
            r#"
            SELECT
                qs.submission_id,
                qs.quiz_id,
                qs.user_id,
                qs.enrollment_id,
                q.course_id,
                q.title as quiz_title,
                qs.score,
                qs.max_score,
                CASE
                    WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                    ELSE 0
                END as percentage,
                qs.passed,
                qs.attempt_number,
                qs.status,
                qs.submitted_at,
                qs.graded_at,
                qs.instructor_feedback
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            WHERE qs.user_id = $1
            ORDER BY qs.submitted_at DESC NULLS LAST
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(grades)
    }

    /// Get grade entries for a specific course.
    pub async fn get_course_grades_for_user(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<Vec<GradeEntry>, GradeError> {
        let grades = sqlx::query_as::<_, GradeEntry>(
            r#"
            SELECT
                qs.submission_id,
                qs.quiz_id,
                qs.user_id,
                qs.enrollment_id,
                q.course_id,
                q.title as quiz_title,
                qs.score,
                qs.max_score,
                CASE
                    WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                    ELSE 0
                END as percentage,
                qs.passed,
                qs.attempt_number,
                qs.status,
                qs.submitted_at,
                qs.graded_at,
                qs.instructor_feedback
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            WHERE qs.user_id = $1 AND q.course_id = $2
            ORDER BY qs.submitted_at DESC NULLS LAST
            "#
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(grades)
    }

    /// Get a specific grade entry by submission ID.
    pub async fn get_grade_entry(
        &self,
        submission_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<GradeEntry, GradeError> {
        let query = if let Some(uid) = user_id {
            sqlx::query_as::<_, GradeEntry>(
                r#"
                SELECT
                    qs.submission_id,
                    qs.quiz_id,
                    qs.user_id,
                    qs.enrollment_id,
                    q.course_id,
                    q.title as quiz_title,
                    qs.score,
                    qs.max_score,
                    CASE
                        WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                        ELSE 0
                    END as percentage,
                    qs.passed,
                    qs.attempt_number,
                    qs.status,
                    qs.submitted_at,
                    qs.graded_at,
                    qs.instructor_feedback
                FROM assessments.quiz_submissions qs
                JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
                WHERE qs.submission_id = $1 AND qs.user_id = $2
                "#
            )
            .bind(submission_id)
            .bind(uid)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, GradeEntry>(
                r#"
                SELECT
                    qs.submission_id,
                    qs.quiz_id,
                    qs.user_id,
                    qs.enrollment_id,
                    q.course_id,
                    q.title as quiz_title,
                    qs.score,
                    qs.max_score,
                    CASE
                        WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                        ELSE 0
                    END as percentage,
                    qs.passed,
                    qs.attempt_number,
                    qs.status,
                    qs.submitted_at,
                    qs.graded_at,
                    qs.instructor_feedback
                FROM assessments.quiz_submissions qs
                JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
                WHERE qs.submission_id = $1
                "#
            )
            .bind(submission_id)
            .fetch_one(&self.pool)
            .await?
        };

        Ok(query)
    }

    /// Get course info for grades aggregation.
    pub async fn get_user_course_info(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<CourseInfo>, GradeError> {
        let courses = sqlx::query_as::<_, CourseInfo>(
            r#"
            SELECT DISTINCT
                c.course_id,
                c.title as course_title,
                e.enrollment_id
            FROM courses.courses c
            JOIN enrollments.enrollments e ON c.course_id = e.course_id
            WHERE e.user_id = $1 AND e.status = 'active'
            ORDER BY c.title
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(courses)
    }

    /// Get all grades for a course (instructor view).
    pub async fn get_all_course_grades(
        &self,
        course_id: Uuid,
    ) -> Result<Vec<StudentGradeRow>, GradeError> {
        let grades = sqlx::query_as::<_, StudentGradeRow>(
            r#"
            SELECT
                qs.user_id,
                u.first_name || ' ' || u.last_name as student_name,
                u.email as student_email,
                qs.submission_id,
                q.title as quiz_title,
                qs.score,
                qs.max_score,
                CASE
                    WHEN qs.max_score > 0 THEN (qs.score / qs.max_score * 100)::DECIMAL(5,2)
                    ELSE 0
                END as percentage,
                qs.passed,
                qs.status,
                qs.submitted_at
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            JOIN auth.users u ON qs.user_id = u.user_id
            WHERE q.course_id = $1
            ORDER BY u.last_name, u.first_name, q.title
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(grades)
    }
}

/// Course info for aggregation.
#[derive(Debug, sqlx::FromRow)]
pub struct CourseInfo {
    pub course_id: Uuid,
    pub course_title: String,
    pub enrollment_id: Uuid,
}

/// Student grade row for export.
#[derive(Debug, sqlx::FromRow)]
pub struct StudentGradeRow {
    pub user_id: Uuid,
    pub student_name: String,
    pub student_email: String,
    pub submission_id: Uuid,
    pub quiz_title: String,
    pub score: Decimal,
    pub max_score: Decimal,
    pub percentage: Decimal,
    pub passed: Option<bool>,
    pub status: String,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
}
