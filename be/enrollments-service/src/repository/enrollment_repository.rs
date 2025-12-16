//! # Enrollment Repository
//!
//! PostgreSQL data access for enrollments and lesson progress.
//!
//! ## Schema
//!
//! This repository operates on the `enrollments` schema:
//! - `enrollments.enrollments`: Student course enrollments
//! - `enrollments.lesson_progress`: Per-lesson progress tracking
//!
//! ## Cross-Schema Access
//!
//! Has SELECT permission on `courses` schema for validation.

use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::domain::{
    Enrollment, EnrollmentStatus, LessonProgress, LessonProgressStatus,
    NewEnrollment, NewLessonProgress, UpdateEnrollment, UpdateLessonProgress,
};

/// Repository for enrollment data access.
#[derive(Debug, Clone)]
pub struct EnrollmentRepository {
    pool: PgPool,
}

impl EnrollmentRepository {
    /// Creates a new enrollment repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // ENROLLMENT QUERIES
    // =========================================================================

    /// Lists enrollments for a user.
    pub async fn list_by_user(
        &self,
        user_id: Uuid,
        status: Option<EnrollmentStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Enrollment>, sqlx::Error> {
        let status_filter = status.map(|s| s.to_string());

        sqlx::query_as::<_, Enrollment>(
            r#"
            SELECT
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            FROM enrollments.enrollments
            WHERE user_id = $1
            AND ($2::TEXT IS NULL OR status = $2)
            ORDER BY last_accessed_at DESC NULLS LAST, created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(status_filter)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Counts enrollments for a user.
    pub async fn count_by_user(
        &self,
        user_id: Uuid,
        status: Option<EnrollmentStatus>,
    ) -> Result<i64, sqlx::Error> {
        let status_filter = status.map(|s| s.to_string());

        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM enrollments.enrollments
            WHERE user_id = $1
            AND ($2::TEXT IS NULL OR status = $2)
            "#,
        )
        .bind(user_id)
        .bind(status_filter)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Lists enrollments for a course (admin/instructor view).
    pub async fn list_by_course(
        &self,
        course_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Enrollment>, sqlx::Error> {
        sqlx::query_as::<_, Enrollment>(
            r#"
            SELECT
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            FROM enrollments.enrollments
            WHERE course_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(course_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds an enrollment by ID.
    pub async fn find_by_id(&self, enrollment_id: Uuid) -> Result<Option<Enrollment>, sqlx::Error> {
        sqlx::query_as::<_, Enrollment>(
            r#"
            SELECT
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            FROM enrollments.enrollments
            WHERE enrollment_id = $1
            "#,
        )
        .bind(enrollment_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Finds an enrollment by user and course.
    pub async fn find_by_user_and_course(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<Option<Enrollment>, sqlx::Error> {
        sqlx::query_as::<_, Enrollment>(
            r#"
            SELECT
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            FROM enrollments.enrollments
            WHERE user_id = $1 AND course_id = $2
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Checks if a user is enrolled in a course.
    pub async fn is_enrolled(&self, user_id: Uuid, course_id: Uuid) -> Result<bool, sqlx::Error> {
        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM enrollments.enrollments
                WHERE user_id = $1 AND course_id = $2
            )
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Creates a new enrollment.
    pub async fn create(&self, data: NewEnrollment) -> Result<Enrollment, sqlx::Error> {
        sqlx::query_as::<_, Enrollment>(
            r#"
            INSERT INTO enrollments.enrollments (
                user_id, course_id, status, progress_percentage,
                enrollment_source, expires_at
            )
            VALUES ($1, $2, 'active', 0.00, $3, $4)
            RETURNING
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            "#,
        )
        .bind(data.user_id)
        .bind(data.course_id)
        .bind(&data.enrollment_source)
        .bind(data.expires_at)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates an enrollment.
    pub async fn update(
        &self,
        enrollment_id: Uuid,
        data: UpdateEnrollment,
    ) -> Result<Enrollment, sqlx::Error> {
        // Build dynamic update query
        sqlx::query_as::<_, Enrollment>(
            r#"
            UPDATE enrollments.enrollments
            SET
                status = COALESCE($2, status),
                progress_percentage = COALESCE($3, progress_percentage),
                started_at = CASE WHEN $4 THEN $5 ELSE started_at END,
                completed_at = CASE WHEN $6 THEN $7 ELSE completed_at END,
                certificate_issued_at = CASE WHEN $8 THEN $9 ELSE certificate_issued_at END,
                expires_at = CASE WHEN $10 THEN $11 ELSE expires_at END,
                updated_at = NOW()
            WHERE enrollment_id = $1
            RETURNING
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            "#,
        )
        .bind(enrollment_id)
        .bind(data.status.map(|s| s.to_string()))
        .bind(data.progress_percentage)
        .bind(data.started_at.is_some())
        .bind(data.started_at.flatten())
        .bind(data.completed_at.is_some())
        .bind(data.completed_at.flatten())
        .bind(data.certificate_issued_at.is_some())
        .bind(data.certificate_issued_at.flatten())
        .bind(data.expires_at.is_some())
        .bind(data.expires_at.flatten())
        .fetch_one(&self.pool)
        .await
    }

    /// Updates the last_accessed_at timestamp.
    pub async fn update_last_accessed(&self, enrollment_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE enrollments.enrollments
            SET last_accessed_at = NOW(), updated_at = NOW()
            WHERE enrollment_id = $1
            "#,
        )
        .bind(enrollment_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Marks enrollment as started (sets started_at if null).
    pub async fn mark_started(&self, enrollment_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE enrollments.enrollments
            SET
                started_at = COALESCE(started_at, NOW()),
                last_accessed_at = NOW(),
                updated_at = NOW()
            WHERE enrollment_id = $1
            "#,
        )
        .bind(enrollment_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Marks enrollment as completed.
    pub async fn mark_completed(&self, enrollment_id: Uuid) -> Result<Enrollment, sqlx::Error> {
        sqlx::query_as::<_, Enrollment>(
            r#"
            UPDATE enrollments.enrollments
            SET
                status = 'completed',
                progress_percentage = 100.00,
                completed_at = NOW(),
                updated_at = NOW()
            WHERE enrollment_id = $1
            RETURNING
                enrollment_id, user_id, course_id, status,
                progress_percentage, started_at, completed_at,
                last_accessed_at, certificate_issued_at,
                enrollment_source, expires_at, created_at, updated_at
            "#,
        )
        .bind(enrollment_id)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // LESSON PROGRESS QUERIES
    // =========================================================================

    /// Lists lesson progress for an enrollment.
    pub async fn list_progress(
        &self,
        enrollment_id: Uuid,
    ) -> Result<Vec<LessonProgress>, sqlx::Error> {
        sqlx::query_as::<_, LessonProgress>(
            r#"
            SELECT
                progress_id, enrollment_id, lesson_id, user_id,
                status, completion_percentage, time_spent_seconds,
                last_position_seconds, completed_at,
                first_accessed_at, last_accessed_at
            FROM enrollments.lesson_progress
            WHERE enrollment_id = $1
            ORDER BY first_accessed_at ASC
            "#,
        )
        .bind(enrollment_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds progress for a specific lesson.
    pub async fn find_progress(
        &self,
        enrollment_id: Uuid,
        lesson_id: Uuid,
    ) -> Result<Option<LessonProgress>, sqlx::Error> {
        sqlx::query_as::<_, LessonProgress>(
            r#"
            SELECT
                progress_id, enrollment_id, lesson_id, user_id,
                status, completion_percentage, time_spent_seconds,
                last_position_seconds, completed_at,
                first_accessed_at, last_accessed_at
            FROM enrollments.lesson_progress
            WHERE enrollment_id = $1 AND lesson_id = $2
            "#,
        )
        .bind(enrollment_id)
        .bind(lesson_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates or updates lesson progress (upsert).
    pub async fn upsert_progress(
        &self,
        data: NewLessonProgress,
    ) -> Result<LessonProgress, sqlx::Error> {
        sqlx::query_as::<_, LessonProgress>(
            r#"
            INSERT INTO enrollments.lesson_progress (
                enrollment_id, lesson_id, user_id, status,
                completion_percentage, time_spent_seconds, last_position_seconds
            )
            VALUES ($1, $2, $3, 'not_started', 0.00, 0, 0)
            ON CONFLICT (enrollment_id, lesson_id) DO UPDATE
            SET last_accessed_at = NOW()
            RETURNING
                progress_id, enrollment_id, lesson_id, user_id,
                status, completion_percentage, time_spent_seconds,
                last_position_seconds, completed_at,
                first_accessed_at, last_accessed_at
            "#,
        )
        .bind(data.enrollment_id)
        .bind(data.lesson_id)
        .bind(data.user_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates lesson progress.
    pub async fn update_progress(
        &self,
        progress_id: Uuid,
        data: UpdateLessonProgress,
    ) -> Result<LessonProgress, sqlx::Error> {
        sqlx::query_as::<_, LessonProgress>(
            r#"
            UPDATE enrollments.lesson_progress
            SET
                status = COALESCE($2, status),
                completion_percentage = COALESCE($3, completion_percentage),
                time_spent_seconds = COALESCE($4, time_spent_seconds),
                last_position_seconds = COALESCE($5, last_position_seconds),
                completed_at = CASE WHEN $6 THEN $7 ELSE completed_at END,
                last_accessed_at = NOW()
            WHERE progress_id = $1
            RETURNING
                progress_id, enrollment_id, lesson_id, user_id,
                status, completion_percentage, time_spent_seconds,
                last_position_seconds, completed_at,
                first_accessed_at, last_accessed_at
            "#,
        )
        .bind(progress_id)
        .bind(data.status.map(|s| s.to_string()))
        .bind(data.completion_percentage)
        .bind(data.time_spent_seconds)
        .bind(data.last_position_seconds)
        .bind(data.completed_at.is_some())
        .bind(data.completed_at.flatten())
        .fetch_one(&self.pool)
        .await
    }

    /// Marks a lesson as completed.
    pub async fn mark_lesson_completed(
        &self,
        progress_id: Uuid,
    ) -> Result<LessonProgress, sqlx::Error> {
        sqlx::query_as::<_, LessonProgress>(
            r#"
            UPDATE enrollments.lesson_progress
            SET
                status = 'completed',
                completion_percentage = 100.00,
                completed_at = NOW(),
                last_accessed_at = NOW()
            WHERE progress_id = $1
            RETURNING
                progress_id, enrollment_id, lesson_id, user_id,
                status, completion_percentage, time_spent_seconds,
                last_position_seconds, completed_at,
                first_accessed_at, last_accessed_at
            "#,
        )
        .bind(progress_id)
        .fetch_one(&self.pool)
        .await
    }

    /// Counts completed lessons for an enrollment.
    pub async fn count_completed_lessons(
        &self,
        enrollment_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM enrollments.lesson_progress
            WHERE enrollment_id = $1 AND status = 'completed'
            "#,
        )
        .bind(enrollment_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    /// Calculates overall progress percentage from lesson completion.
    pub async fn calculate_progress(
        &self,
        enrollment_id: Uuid,
        total_lessons: i64,
    ) -> Result<f64, sqlx::Error> {
        if total_lessons == 0 {
            return Ok(0.0);
        }

        let completed = self.count_completed_lessons(enrollment_id).await?;
        Ok((completed as f64 / total_lessons as f64) * 100.0)
    }

    // =========================================================================
    // STATISTICS QUERIES
    // =========================================================================

    /// Gets enrollment statistics for a course.
    pub async fn get_course_stats(
        &self,
        course_id: Uuid,
    ) -> Result<CourseEnrollmentStats, sqlx::Error> {
        let result = sqlx::query_as::<_, CourseEnrollmentStats>(
            r#"
            SELECT
                COUNT(*) as total_enrollments,
                COUNT(*) FILTER (WHERE status = 'active') as active_count,
                COUNT(*) FILTER (WHERE status = 'completed') as completed_count,
                COALESCE(AVG(progress_percentage), 0) as avg_progress,
                COALESCE(AVG(progress_percentage) FILTER (WHERE status = 'completed'), 0) as avg_completion_progress
            FROM enrollments.enrollments
            WHERE course_id = $1
            "#,
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Gets user learning statistics.
    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<UserLearningStats, sqlx::Error> {
        let result = sqlx::query_as::<_, UserLearningStats>(
            r#"
            SELECT
                COUNT(*) as total_enrolled,
                COUNT(*) FILTER (WHERE status = 'completed') as total_completed,
                COUNT(*) FILTER (WHERE status = 'active') as in_progress,
                COALESCE(SUM(lp.time_spent_seconds), 0) as total_time_seconds
            FROM enrollments.enrollments e
            LEFT JOIN enrollments.lesson_progress lp ON e.enrollment_id = lp.enrollment_id
            WHERE e.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}

/// Statistics for course enrollments.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CourseEnrollmentStats {
    pub total_enrollments: i64,
    pub active_count: i64,
    pub completed_count: i64,
    pub avg_progress: f64,
    pub avg_completion_progress: f64,
}

/// User learning statistics.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserLearningStats {
    pub total_enrolled: i64,
    pub total_completed: i64,
    pub in_progress: i64,
    pub total_time_seconds: i64,
}
