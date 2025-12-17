//! # Semantic Search Service
//!
//! Business logic for semantic search across course content.

use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, ContentEmbedding, EmbeddingContentType, SemanticSearchResult,
};
use crate::llm::LLMClient;
use crate::repository::EmbeddingRepository;

/// Service for semantic search operations.
pub struct SemanticSearchService {
    embedding_repo: EmbeddingRepository,
    llm_client: Arc<dyn LLMClient>,
}

impl SemanticSearchService {
    pub fn new(
        embedding_repo: EmbeddingRepository,
        llm_client: Arc<dyn LLMClient>,
    ) -> Self {
        Self {
            embedding_repo,
            llm_client,
        }
    }

    /// Performs semantic search.
    pub async fn search(
        &self,
        query: &str,
        user_id: Option<Uuid>,
        course_ids: Option<Vec<Uuid>>,
        limit: i32,
        min_score: f32,
    ) -> AIResult<Vec<SemanticSearchResult>> {
        if query.trim().is_empty() {
            return Err(AIError::EmptyQuery);
        }

        // Generate embedding for query
        let query_embedding = self.llm_client
            .generate_embedding(query)
            .await
            .map_err(|e| AIError::EmbeddingService(e))?;

        // Search for similar content
        let course_ids_ref = course_ids.as_deref();
        let mut results = self.embedding_repo
            .search_similar(&query_embedding, course_ids_ref, limit, min_score)
            .await?;

        // Check access for each result
        if let Some(uid) = user_id {
            for result in &mut results {
                result.has_access = self.check_user_access(uid, result.course_id).await;
            }
        }

        Ok(results)
    }

    /// Indexes course content for semantic search.
    pub async fn index_course(&self, course_id: Uuid, force_reindex: bool) -> AIResult<i32> {
        // Check if already indexed
        if !force_reindex && self.embedding_repo.has_embeddings(course_id).await? {
            return Ok(0);
        }

        // Delete existing embeddings if reindexing
        if force_reindex {
            self.embedding_repo.delete_course_embeddings(course_id).await?;
        }

        // Fetch course content (this would normally call courses-service)
        let content_chunks = self.fetch_course_content(course_id).await?;

        // Generate embeddings for each chunk
        let mut embeddings = Vec::new();
        for (i, chunk) in content_chunks.iter().enumerate() {
            let embedding_vec = self.llm_client
                .generate_embedding(&chunk.text)
                .await
                .map_err(|e| AIError::EmbeddingService(e))?;

            let embedding = ContentEmbedding {
                embedding_id: Uuid::now_v7(),
                course_id,
                lesson_id: chunk.lesson_id,
                content_type: chunk.content_type,
                chunk_index: i as i32,
                content_text: chunk.text.clone(),
                embedding: embedding_vec,
                metadata: None,
                created_at: Utc::now(),
            };

            embeddings.push(embedding);
        }

        // Save embeddings
        let count = self.embedding_repo.save_embeddings_batch(&embeddings).await?;

        Ok(count)
    }

    /// Checks if user has access to a course.
    async fn check_user_access(&self, _user_id: Uuid, _course_id: Uuid) -> bool {
        // TODO: Call enrollments-service to check access
        true
    }

    /// Fetches course content for indexing.
    async fn fetch_course_content(&self, _course_id: Uuid) -> AIResult<Vec<ContentChunk>> {
        // TODO: Call courses-service to fetch content
        // For now, return empty - this would be populated by actual course content
        Ok(vec![])
    }
}

/// A chunk of content for embedding.
struct ContentChunk {
    text: String,
    lesson_id: Option<Uuid>,
    content_type: EmbeddingContentType,
}
