//! # Embedding Repository
//!
//! Data access for content embeddings and semantic search.

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, ContentEmbedding, EmbeddingContentType, SemanticSearchResult,
};

/// Repository for embedding operations.
#[derive(Clone)]
pub struct EmbeddingRepository {
    pool: PgPool,
}

impl EmbeddingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Saves a content embedding.
    pub async fn save_embedding(&self, embedding: &ContentEmbedding) -> AIResult<()> {
        let embedding_vec: Vec<f32> = embedding.embedding.clone();

        sqlx::query(
            r#"
            INSERT INTO ai.content_embeddings (
                embedding_id, course_id, lesson_id, content_type,
                chunk_index, content_text, embedding, metadata, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7::vector, $8, $9)
            ON CONFLICT (course_id, lesson_id, chunk_index)
            DO UPDATE SET
                content_text = EXCLUDED.content_text,
                embedding = EXCLUDED.embedding,
                metadata = EXCLUDED.metadata
            "#
        )
        .bind(embedding.embedding_id)
        .bind(embedding.course_id)
        .bind(embedding.lesson_id)
        .bind(embedding.content_type.to_string())
        .bind(embedding.chunk_index)
        .bind(&embedding.content_text)
        .bind(&embedding_vec)
        .bind(&embedding.metadata)
        .bind(embedding.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Batch saves embeddings.
    pub async fn save_embeddings_batch(&self, embeddings: &[ContentEmbedding]) -> AIResult<i32> {
        let mut count = 0;
        for embedding in embeddings {
            self.save_embedding(embedding).await?;
            count += 1;
        }
        Ok(count)
    }

    /// Performs semantic search using cosine similarity.
    pub async fn search_similar(
        &self,
        query_embedding: &[f32],
        course_ids: Option<&[Uuid]>,
        limit: i32,
        min_score: f32,
    ) -> AIResult<Vec<SemanticSearchResult>> {
        let query_vec: Vec<f32> = query_embedding.to_vec();

        let rows = if let Some(ids) = course_ids {
            sqlx::query(
                r#"
                SELECT
                    e.embedding_id, e.course_id, e.lesson_id, e.content_type,
                    e.content_text as snippet,
                    1 - (e.embedding <=> $1::vector) as similarity_score,
                    c.title as course_title,
                    l.title as lesson_title
                FROM ai.content_embeddings e
                JOIN courses.courses c ON c.course_id = e.course_id
                LEFT JOIN courses.lessons l ON l.lesson_id = e.lesson_id
                WHERE e.course_id = ANY($2)
                    AND 1 - (e.embedding <=> $1::vector) >= $3
                ORDER BY similarity_score DESC
                LIMIT $4
                "#
            )
            .bind(&query_vec)
            .bind(ids)
            .bind(min_score)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query(
                r#"
                SELECT
                    e.embedding_id, e.course_id, e.lesson_id, e.content_type,
                    e.content_text as snippet,
                    1 - (e.embedding <=> $1::vector) as similarity_score,
                    c.title as course_title,
                    l.title as lesson_title
                FROM ai.content_embeddings e
                JOIN courses.courses c ON c.course_id = e.course_id
                LEFT JOIN courses.lessons l ON l.lesson_id = e.lesson_id
                WHERE 1 - (e.embedding <=> $1::vector) >= $2
                ORDER BY similarity_score DESC
                LIMIT $3
                "#
            )
            .bind(&query_vec)
            .bind(min_score)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
        };

        let rows = rows.map_err(|e| AIError::Database(e.to_string()))?;

        Ok(rows.iter().map(|row| self.map_search_result(row)).collect())
    }

    /// Gets embeddings for a course.
    pub async fn get_course_embeddings(&self, course_id: Uuid) -> AIResult<Vec<ContentEmbedding>> {
        let rows = sqlx::query(
            r#"
            SELECT embedding_id, course_id, lesson_id, content_type,
                   chunk_index, content_text, metadata, created_at
            FROM ai.content_embeddings
            WHERE course_id = $1
            ORDER BY lesson_id, chunk_index
            "#
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(rows.iter().map(|r| self.map_embedding_row(r)).collect())
    }

    /// Deletes embeddings for a course.
    pub async fn delete_course_embeddings(&self, course_id: Uuid) -> AIResult<i64> {
        let result = sqlx::query(
            "DELETE FROM ai.content_embeddings WHERE course_id = $1"
        )
        .bind(course_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    /// Checks if course has embeddings.
    pub async fn has_embeddings(&self, course_id: Uuid) -> AIResult<bool> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM ai.content_embeddings WHERE course_id = $1)"
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        use sqlx::Row;
        Ok(row.get::<bool, _>(0))
    }

    // Helper methods
    fn map_search_result(&self, row: &sqlx::postgres::PgRow) -> SemanticSearchResult {
        use sqlx::Row;

        let content_type_str: String = row.get("content_type");
        let content_type = match content_type_str.as_str() {
            "lesson_text" => EmbeddingContentType::LessonText,
            "transcription" => EmbeddingContentType::Transcription,
            "description" => EmbeddingContentType::Description,
            "resource" => EmbeddingContentType::Resource,
            _ => EmbeddingContentType::LessonText,
        };

        SemanticSearchResult {
            embedding_id: row.get("embedding_id"),
            course_id: row.get("course_id"),
            course_title: row.get("course_title"),
            lesson_id: row.get("lesson_id"),
            lesson_title: row.get("lesson_title"),
            content_type,
            snippet: row.get("snippet"),
            similarity_score: row.get("similarity_score"),
            has_access: false, // Will be set by service layer
        }
    }

    fn map_embedding_row(&self, row: &sqlx::postgres::PgRow) -> ContentEmbedding {
        use sqlx::Row;

        let content_type_str: String = row.get("content_type");
        let content_type = match content_type_str.as_str() {
            "lesson_text" => EmbeddingContentType::LessonText,
            "transcription" => EmbeddingContentType::Transcription,
            "description" => EmbeddingContentType::Description,
            "resource" => EmbeddingContentType::Resource,
            _ => EmbeddingContentType::LessonText,
        };

        ContentEmbedding {
            embedding_id: row.get("embedding_id"),
            course_id: row.get("course_id"),
            lesson_id: row.get("lesson_id"),
            content_type,
            chunk_index: row.get("chunk_index"),
            content_text: row.get("content_text"),
            embedding: vec![], // Not fetched for listing
            metadata: row.get("metadata"),
            created_at: row.get("created_at"),
        }
    }
}

impl EmbeddingContentType {
    fn to_string(&self) -> &'static str {
        match self {
            Self::LessonText => "lesson_text",
            Self::Transcription => "transcription",
            Self::Description => "description",
            Self::Resource => "resource",
        }
    }
}
