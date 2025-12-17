//! # Embedding Repository
//!
//! Database access for vector embeddings and semantic search.

use pgvector::Vector;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{SearchError, SearchResult, SemanticSearchResult};

/// Repository for embedding and semantic search operations.
#[derive(Debug, Clone)]
pub struct EmbeddingRepository {
    pool: PgPool,
}

impl EmbeddingRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Search content by vector similarity.
    pub async fn search_by_embedding(
        &self,
        user_id: Uuid,
        embedding: &[f32],
        course_id: Option<Uuid>,
        limit: usize,
        threshold: f32,
    ) -> SearchResult<Vec<SemanticSearchResult>> {
        let vector = Vector::from(embedding.to_vec());

        let mut conditions = String::new();
        if let Some(cid) = course_id {
            conditions.push_str(&format!(" AND c.course_id = '{}'", cid));
        }

        // Using pgvector's cosine distance operator (<=>)
        // 1 - distance = similarity (cosine distance is 1 - cosine similarity)
        let sql = format!(
            r#"
            SELECT
                ce.embedding_id as content_id,
                ce.content_type,
                c.course_id,
                c.title as course_title,
                l.lesson_id,
                l.title as lesson_title,
                ce.text_chunk,
                (1 - (ce.embedding <=> $1::vector)) as similarity,
                ce.timestamp_seconds
            FROM content_embeddings ce
            JOIN lessons l ON ce.lesson_id = l.lesson_id
            JOIN courses c ON l.course_id = c.course_id
            JOIN enrollments e ON c.course_id = e.course_id
            WHERE e.user_id = $2
            AND e.status = 'active'
            AND (1 - (ce.embedding <=> $1::vector)) >= $3
            {}
            ORDER BY similarity DESC
            LIMIT $4
            "#,
            conditions
        );

        let results: Vec<SemanticSearchResult> = sqlx::query_as(&sql)
            .bind(&vector)
            .bind(user_id)
            .bind(threshold)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }

    /// Store an embedding for content.
    pub async fn store_embedding(
        &self,
        lesson_id: Uuid,
        content_type: &str,
        text_chunk: &str,
        embedding: &[f32],
        timestamp_seconds: Option<i32>,
    ) -> SearchResult<Uuid> {
        let vector = Vector::from(embedding.to_vec());

        let embedding_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO content_embeddings (lesson_id, content_type, text_chunk, embedding, timestamp_seconds)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING embedding_id
            "#,
        )
        .bind(lesson_id)
        .bind(content_type)
        .bind(text_chunk)
        .bind(&vector)
        .bind(timestamp_seconds)
        .fetch_one(&self.pool)
        .await?;

        Ok(embedding_id)
    }

    /// Delete embeddings for a lesson.
    pub async fn delete_lesson_embeddings(&self, lesson_id: Uuid) -> SearchResult<u64> {
        let result = sqlx::query("DELETE FROM content_embeddings WHERE lesson_id = $1")
            .bind(lesson_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// Search courses by embedding (for semantic course search).
    pub async fn search_courses_by_embedding(
        &self,
        embedding: &[f32],
        limit: usize,
        threshold: f32,
    ) -> SearchResult<Vec<(Uuid, f32)>> {
        let vector = Vector::from(embedding.to_vec());

        let results: Vec<(Uuid, f32)> = sqlx::query_as(
            r#"
            SELECT
                c.course_id,
                (1 - (ce.embedding <=> $1::vector)) as similarity
            FROM course_embeddings ce
            JOIN courses c ON ce.course_id = c.course_id
            WHERE c.status = 'published'
            AND (1 - (ce.embedding <=> $1::vector)) >= $2
            ORDER BY similarity DESC
            LIMIT $3
            "#,
        )
        .bind(&vector)
        .bind(threshold)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}
