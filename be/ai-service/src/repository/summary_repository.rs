//! # Summary Repository
//!
//! Data access for generated summaries and content.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, ContentGenerationType, ContentSummary, GenerationStatus,
};

/// Repository for summary operations.
#[derive(Clone)]
pub struct SummaryRepository {
    pool: PgPool,
}

impl SummaryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new summary record.
    pub async fn create_summary(&self, summary: &ContentSummary) -> AIResult<()> {
        sqlx::query(
            r#"
            INSERT INTO ai.content_summaries (
                summary_id, course_id, lesson_id, generation_type,
                content, language, status, model_used, tokens_used,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#
        )
        .bind(summary.summary_id)
        .bind(summary.course_id)
        .bind(summary.lesson_id)
        .bind(content_gen_type_to_str(&summary.generation_type))
        .bind(&summary.content)
        .bind(&summary.language)
        .bind(gen_status_to_str(&summary.status))
        .bind(&summary.model_used)
        .bind(summary.tokens_used)
        .bind(summary.created_at)
        .bind(summary.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets a summary by ID.
    pub async fn get_summary(&self, summary_id: Uuid) -> AIResult<ContentSummary> {
        let row = sqlx::query(
            r#"
            SELECT summary_id, course_id, lesson_id, generation_type,
                   content, language, status, model_used, tokens_used,
                   created_at, updated_at
            FROM ai.content_summaries
            WHERE summary_id = $1
            "#
        )
        .bind(summary_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?
        .ok_or(AIError::GenerationNotFound(summary_id))?;

        Ok(self.map_summary_row(&row))
    }

    /// Gets existing summary for a lesson.
    pub async fn get_lesson_summary(
        &self,
        lesson_id: Uuid,
        generation_type: ContentGenerationType,
        language: &str,
    ) -> AIResult<Option<ContentSummary>> {
        let row = sqlx::query(
            r#"
            SELECT summary_id, course_id, lesson_id, generation_type,
                   content, language, status, model_used, tokens_used,
                   created_at, updated_at
            FROM ai.content_summaries
            WHERE lesson_id = $1 AND generation_type = $2 AND language = $3
                AND status = 'completed'
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(lesson_id)
        .bind(content_gen_type_to_str(&generation_type))
        .bind(language)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(row.map(|r| self.map_summary_row(&r)))
    }

    /// Updates summary status and content.
    pub async fn update_summary(
        &self,
        summary_id: Uuid,
        content: &str,
        status: GenerationStatus,
        tokens_used: Option<i32>,
    ) -> AIResult<()> {
        sqlx::query(
            r#"
            UPDATE ai.content_summaries
            SET content = $2, status = $3, tokens_used = $4, updated_at = $5
            WHERE summary_id = $1
            "#
        )
        .bind(summary_id)
        .bind(content)
        .bind(gen_status_to_str(&status))
        .bind(tokens_used)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets all summaries for a lesson.
    pub async fn get_lesson_summaries(&self, lesson_id: Uuid) -> AIResult<Vec<ContentSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT summary_id, course_id, lesson_id, generation_type,
                   content, language, status, model_used, tokens_used,
                   created_at, updated_at
            FROM ai.content_summaries
            WHERE lesson_id = $1 AND status = 'completed'
            ORDER BY generation_type, language
            "#
        )
        .bind(lesson_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(rows.iter().map(|r| self.map_summary_row(r)).collect())
    }

    // Helper methods
    fn map_summary_row(&self, row: &sqlx::postgres::PgRow) -> ContentSummary {
        use sqlx::Row;

        let gen_type_str: String = row.get("generation_type");
        let generation_type = match gen_type_str.as_str() {
            "summary" => ContentGenerationType::Summary,
            "key_points" => ContentGenerationType::KeyPoints,
            "glossary" => ContentGenerationType::Glossary,
            "objectives" => ContentGenerationType::Objectives,
            "transcription" => ContentGenerationType::Transcription,
            _ => ContentGenerationType::Summary,
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "pending" => GenerationStatus::Pending,
            "processing" => GenerationStatus::Processing,
            "completed" => GenerationStatus::Completed,
            "failed" => GenerationStatus::Failed,
            _ => GenerationStatus::Pending,
        };

        ContentSummary {
            summary_id: row.get("summary_id"),
            course_id: row.get("course_id"),
            lesson_id: row.get("lesson_id"),
            generation_type,
            content: row.get("content"),
            language: row.get("language"),
            status,
            model_used: row.get("model_used"),
            tokens_used: row.get("tokens_used"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

fn content_gen_type_to_str(t: &ContentGenerationType) -> &'static str {
    match t {
        ContentGenerationType::Summary => "summary",
        ContentGenerationType::KeyPoints => "key_points",
        ContentGenerationType::Glossary => "glossary",
        ContentGenerationType::Objectives => "objectives",
        ContentGenerationType::Transcription => "transcription",
    }
}

fn gen_status_to_str(s: &GenerationStatus) -> &'static str {
    match s {
        GenerationStatus::Pending => "pending",
        GenerationStatus::Processing => "processing",
        GenerationStatus::Completed => "completed",
        GenerationStatus::Failed => "failed",
    }
}
