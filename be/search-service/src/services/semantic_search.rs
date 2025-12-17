//! # Semantic Search Service
//!
//! Vector-based semantic search using OpenAI embeddings.

use std::sync::Arc;
use std::time::Instant;

use async_openai::{
    config::OpenAIConfig,
    types::{CreateEmbeddingRequest, EmbeddingInput},
    Client,
};
use uuid::Uuid;

use crate::domain::{SearchError, SearchResult, SemanticSearchResult};
use crate::repository::EmbeddingRepository;

const EMBEDDING_MODEL: &str = "text-embedding-3-small";
const EMBEDDING_DIMENSIONS: usize = 1536;

/// Semantic search service using vector embeddings.
#[derive(Clone)]
pub struct SemanticSearchService {
    repo: Arc<EmbeddingRepository>,
    openai: Arc<Client<OpenAIConfig>>,
}

impl SemanticSearchService {
    /// Create a new semantic search service.
    pub fn new(repo: EmbeddingRepository, openai: Client<OpenAIConfig>) -> Self {
        Self {
            repo: Arc::new(repo),
            openai: Arc::new(openai),
        }
    }

    /// Search content semantically within user's enrolled courses.
    pub async fn search(
        &self,
        user_id: Uuid,
        query: &str,
        course_id: Option<Uuid>,
        limit: usize,
        threshold: f32,
    ) -> SearchResult<(Vec<SemanticSearchResult>, u64)> {
        let start = Instant::now();

        // Generate embedding for query
        let embedding = self.generate_embedding(query).await?;

        // Search by similarity
        let results = self
            .repo
            .search_by_embedding(user_id, &embedding, course_id, limit, threshold)
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;
        Ok((results, elapsed))
    }

    /// Search courses by semantic similarity.
    pub async fn search_courses(
        &self,
        query: &str,
        limit: usize,
        threshold: f32,
    ) -> SearchResult<Vec<(Uuid, f32)>> {
        let embedding = self.generate_embedding(query).await?;
        self.repo
            .search_courses_by_embedding(&embedding, limit, threshold)
            .await
    }

    /// Generate embedding for text using OpenAI.
    pub async fn generate_embedding(&self, text: &str) -> SearchResult<Vec<f32>> {
        let request = CreateEmbeddingRequest {
            model: EMBEDDING_MODEL.to_string(),
            input: EmbeddingInput::String(text.to_string()),
            encoding_format: None,
            dimensions: Some(EMBEDDING_DIMENSIONS as u32),
            user: None,
        };

        let response = self
            .openai
            .embeddings()
            .create(request)
            .await
            .map_err(|e| SearchError::EmbeddingError(e.to_string()))?;

        let embedding = response
            .data
            .first()
            .ok_or_else(|| SearchError::EmbeddingError("No embedding returned".to_string()))?
            .embedding
            .clone();

        Ok(embedding)
    }

    /// Index content by generating and storing its embedding.
    pub async fn index_content(
        &self,
        lesson_id: Uuid,
        content_type: &str,
        text: &str,
        timestamp_seconds: Option<i32>,
    ) -> SearchResult<Uuid> {
        // Split text into chunks if too long
        let chunks = self.chunk_text(text, 500, 100);

        let mut embedding_id = Uuid::nil();
        for chunk in chunks {
            let embedding = self.generate_embedding(&chunk).await?;
            embedding_id = self
                .repo
                .store_embedding(lesson_id, content_type, &chunk, &embedding, timestamp_seconds)
                .await?;
        }

        Ok(embedding_id)
    }

    /// Remove embeddings for a lesson.
    pub async fn remove_lesson_index(&self, lesson_id: Uuid) -> SearchResult<u64> {
        self.repo.delete_lesson_embeddings(lesson_id).await
    }

    /// Chunk text with overlap for better context.
    fn chunk_text(&self, text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.len() <= chunk_size {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < words.len() {
            let end = (start + chunk_size).min(words.len());
            let chunk = words[start..end].join(" ");
            chunks.push(chunk);

            if end >= words.len() {
                break;
            }

            start = end - overlap;
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text_short() {
        let service = SemanticSearchService {
            repo: Arc::new(unsafe { std::mem::zeroed() }),
            openai: Arc::new(unsafe { std::mem::zeroed() }),
        };

        let text = "This is a short text";
        let chunks = service.chunk_text(text, 500, 100);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }
}
