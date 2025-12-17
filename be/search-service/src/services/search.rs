//! # Search Service
//!
//! Main course search business logic.

use std::sync::Arc;
use std::time::Instant;

use redis::AsyncCommands;
use uuid::Uuid;

use crate::domain::{
    CourseSearchResult, SavedSearch, SearchError, SearchFacets, SearchQuery, SearchResult,
    SearchResults, SearchType,
};
use crate::repository::CourseSearchRepository;
use crate::services::SemanticSearchService;

const CACHE_TTL_SECS: u64 = 300; // 5 minutes

/// Course search service.
#[derive(Clone)]
pub struct SearchService {
    repo: Arc<CourseSearchRepository>,
    semantic: Arc<SemanticSearchService>,
    redis: Arc<redis::Client>,
}

impl SearchService {
    /// Create a new search service.
    pub fn new(
        repo: CourseSearchRepository,
        semantic: SemanticSearchService,
        redis: redis::Client,
    ) -> Self {
        Self {
            repo: Arc::new(repo),
            semantic: Arc::new(semantic),
            redis: Arc::new(redis),
        }
    }

    /// Search courses.
    pub async fn search_courses(
        &self,
        query: &SearchQuery,
    ) -> SearchResult<(SearchResults<CourseSearchResult>, u64)> {
        let start = Instant::now();

        // Try cache first for full-text searches
        if matches!(query.search_type, SearchType::FullText) {
            if let Some(cached) = self.get_cached_results(query).await {
                let elapsed = start.elapsed().as_millis() as u64;
                return Ok((cached, elapsed));
            }
        }

        let results = match query.search_type {
            SearchType::FullText => self.repo.search_courses(query).await?,
            SearchType::Semantic => self.semantic_search(query).await?,
            SearchType::Hybrid => self.hybrid_search(query).await?,
        };

        // Cache full-text results
        if matches!(query.search_type, SearchType::FullText) {
            self.cache_results(query, &results).await;
        }

        let elapsed = start.elapsed().as_millis() as u64;
        Ok((results, elapsed))
    }

    /// Semantic search - use embeddings.
    async fn semantic_search(
        &self,
        query: &SearchQuery,
    ) -> SearchResult<SearchResults<CourseSearchResult>> {
        // Get courses matching semantic search
        let course_ids = self
            .semantic
            .search_courses(&query.query, query.limit() as usize, 0.7)
            .await?;

        if course_ids.is_empty() {
            return Ok(SearchResults::new(vec![], 0, query, 0));
        }

        // Fetch full course details for matching IDs
        // For now, fall back to full-text search with semantic boost
        // A full implementation would fetch by IDs and merge scores
        self.repo.search_courses(query).await
    }

    /// Hybrid search - combine full-text and semantic.
    async fn hybrid_search(
        &self,
        query: &SearchQuery,
    ) -> SearchResult<SearchResults<CourseSearchResult>> {
        // Run both searches in parallel
        let (ft_results, semantic_ids) = tokio::join!(
            self.repo.search_courses(query),
            self.semantic.search_courses(&query.query, 50, 0.6)
        );

        let ft_results = ft_results?;
        let semantic_ids = semantic_ids?;

        // Create a set of semantic match IDs with their scores
        let semantic_scores: std::collections::HashMap<Uuid, f32> = semantic_ids
            .into_iter()
            .collect();

        // Boost scores for items that appear in both results
        let mut items = ft_results.items;
        for item in &mut items {
            if let Some(semantic_score) = semantic_scores.get(&item.course_id) {
                // Boost relevance by semantic score
                let current = item.relevance_score.unwrap_or(0.0);
                item.relevance_score = Some(current * 0.6 + semantic_score * 0.4);
            }
        }

        // Re-sort by combined score
        items.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(SearchResults::new(
            items,
            ft_results.total,
            query,
            0,
        ))
    }

    /// Get search facets.
    pub async fn get_facets(&self, query: &SearchQuery) -> SearchResult<SearchFacets> {
        self.repo.get_facets(query).await
    }

    /// Save a search.
    pub async fn save_search(
        &self,
        user_id: Uuid,
        name: &str,
        query: &str,
        filters: Option<serde_json::Value>,
    ) -> SearchResult<SavedSearch> {
        self.repo.save_search(user_id, name, query, filters).await
    }

    /// List saved searches.
    pub async fn list_saved_searches(&self, user_id: Uuid) -> SearchResult<Vec<SavedSearch>> {
        self.repo.list_saved_searches(user_id).await
    }

    /// Delete a saved search.
    pub async fn delete_saved_search(
        &self,
        user_id: Uuid,
        saved_search_id: Uuid,
    ) -> SearchResult<()> {
        self.repo.delete_saved_search(user_id, saved_search_id).await
    }

    /// Health check.
    pub async fn health_check(&self) -> SearchResult<bool> {
        self.repo.health_check().await
    }

    // =========================================================================
    // CACHING
    // =========================================================================

    fn cache_key(&self, query: &SearchQuery) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        query.query.hash(&mut hasher);
        query.page.hash(&mut hasher);
        query.per_page.hash(&mut hasher);
        format!("search:courses:{:x}", hasher.finish())
    }

    async fn get_cached_results(
        &self,
        query: &SearchQuery,
    ) -> Option<SearchResults<CourseSearchResult>> {
        let key = self.cache_key(query);

        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return None,
        };

        let data: Option<String> = conn.get(&key).await.ok()?;
        data.and_then(|s| serde_json::from_str(&s).ok())
    }

    async fn cache_results(&self, query: &SearchQuery, results: &SearchResults<CourseSearchResult>) {
        let key = self.cache_key(query);

        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return,
        };

        if let Ok(data) = serde_json::to_string(results) {
            let _: Result<(), _> = conn
                .set_ex::<_, _, ()>(&key, data, CACHE_TTL_SECS)
                .await;
        }
    }
}
