//! # Content Search Service
//!
//! Search within enrolled course content.

use std::sync::Arc;
use std::time::Instant;

use uuid::Uuid;

use crate::domain::{ContentSearchResult, SearchError, SearchQuery, SearchResult, SearchResults};
use crate::repository::ContentSearchRepository;

/// Content search service.
#[derive(Clone)]
pub struct ContentSearchService {
    repo: Arc<ContentSearchRepository>,
}

impl ContentSearchService {
    /// Create a new content search service.
    pub fn new(repo: ContentSearchRepository) -> Self {
        Self { repo: Arc::new(repo) }
    }

    /// Search content within user's enrolled courses.
    pub async fn search(
        &self,
        user_id: Uuid,
        query: &str,
        course_id: Option<Uuid>,
        content_types: Option<&[String]>,
        page: i32,
        per_page: i32,
    ) -> SearchResult<(SearchResults<ContentSearchResult>, u64)> {
        let start = Instant::now();

        let results = self
            .repo
            .search_content(user_id, query, course_id, content_types, page, per_page)
            .await?;

        let elapsed = start.elapsed().as_millis() as u64;
        Ok((results, elapsed))
    }

    /// Search video transcripts with timestamps.
    pub async fn search_transcripts(
        &self,
        user_id: Uuid,
        query: &str,
        course_id: Option<Uuid>,
        limit: i32,
    ) -> SearchResult<Vec<ContentSearchResult>> {
        self.repo
            .search_transcripts(user_id, query, course_id, limit)
            .await
    }
}
