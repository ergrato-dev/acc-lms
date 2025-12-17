//! # Transcript Repository
//!
//! Data access for transcript information.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{GradeError, TranscriptEntry, TranscriptEntryStatus, LetterGrade};

/// Repository for transcript data access.
pub struct TranscriptRepository {
    pool: PgPool,
}

impl TranscriptRepository {
    /// Create a new transcript repository.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get user profile info for transcript.
    pub async fn get_user_info(&self, user_id: Uuid) -> Result<UserInfo, GradeError> {
        let info = sqlx::query_as::<_, UserInfo>(
            r#"
            SELECT
                user_id,
                first_name || ' ' || last_name as full_name,
                email,
                created_at as member_since
            FROM auth.users
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(info)
    }

    /// Get transcript entries for a user.
    pub async fn get_transcript_entries(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TranscriptEntryRow>, GradeError> {
        let entries = sqlx::query_as::<_, TranscriptEntryRow>(
            r#"
            SELECT
                c.course_id,
                c.title as course_title,
                COALESCE(u.first_name || ' ' || u.last_name, 'Unknown') as instructor_name,
                e.enrolled_at as enrollment_date,
                e.completed_at as completion_date,
                e.progress_percentage,
                e.status as enrollment_status,
                cert.certificate_id IS NOT NULL as certificate_issued,
                cert.certificate_url
            FROM enrollments.enrollments e
            JOIN courses.courses c ON e.course_id = c.course_id
            LEFT JOIN auth.users u ON c.instructor_id = u.user_id
            LEFT JOIN enrollments.certificates cert ON e.enrollment_id = cert.enrollment_id
            WHERE e.user_id = $1
            ORDER BY e.enrolled_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    /// Get grade summary for a course enrollment.
    pub async fn get_enrollment_grade_summary(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<EnrollmentGradeSummary, GradeError> {
        let summary = sqlx::query_as::<_, EnrollmentGradeSummary>(
            r#"
            SELECT
                COALESCE(SUM(qs.score), 0) as total_score,
                COALESCE(SUM(qs.max_score), 0) as total_max_score,
                COUNT(*) FILTER (WHERE qs.status = 'graded') as graded_count
            FROM assessments.quiz_submissions qs
            JOIN assessments.quizzes q ON qs.quiz_id = q.quiz_id
            WHERE qs.user_id = $1 AND q.course_id = $2
            "#
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(summary)
    }

    /// Count total certificates for a user.
    pub async fn count_user_certificates(&self, user_id: Uuid) -> Result<i32, GradeError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM enrollments.certificates cert
            JOIN enrollments.enrollments e ON cert.enrollment_id = e.enrollment_id
            WHERE e.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 as i32)
    }
}

/// User info for transcript.
#[derive(Debug, sqlx::FromRow)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub member_since: DateTime<Utc>,
}

/// Raw transcript entry from database.
#[derive(Debug, sqlx::FromRow)]
pub struct TranscriptEntryRow {
    pub course_id: Uuid,
    pub course_title: String,
    pub instructor_name: String,
    pub enrollment_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub progress_percentage: Decimal,
    pub enrollment_status: String,
    pub certificate_issued: bool,
    pub certificate_url: Option<String>,
}

impl TranscriptEntryRow {
    /// Convert to domain TranscriptEntry with grade info.
    pub fn to_transcript_entry(self, grade_percentage: Decimal) -> TranscriptEntry {
        let status = match self.enrollment_status.as_str() {
            "completed" => TranscriptEntryStatus::Completed,
            "dropped" => TranscriptEntryStatus::Dropped,
            "withdrawn" => TranscriptEntryStatus::Withdrawn,
            _ => TranscriptEntryStatus::InProgress,
        };

        TranscriptEntry {
            course_id: self.course_id,
            course_title: self.course_title,
            instructor_name: self.instructor_name,
            enrollment_date: self.enrollment_date,
            completion_date: self.completion_date,
            progress_percentage: self.progress_percentage,
            grade_percentage,
            letter_grade: LetterGrade::from_percentage(grade_percentage),
            status,
            certificate_issued: self.certificate_issued,
            certificate_url: self.certificate_url,
        }
    }
}

/// Grade summary for an enrollment.
#[derive(Debug, sqlx::FromRow)]
pub struct EnrollmentGradeSummary {
    pub total_score: Decimal,
    pub total_max_score: Decimal,
    pub graded_count: i64,
}

impl EnrollmentGradeSummary {
    /// Calculate percentage grade.
    pub fn percentage(&self) -> Decimal {
        if self.total_max_score > Decimal::ZERO {
            (self.total_score / self.total_max_score) * Decimal::new(100, 0)
        } else {
            Decimal::ZERO
        }
    }
}
