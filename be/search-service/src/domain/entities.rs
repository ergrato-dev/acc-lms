//! # Domain Entities
//!
//! Core business entities for search functionality.
//!
//! ## Entity Overview
//!
//! - `SearchQuery`: User's search request with filters
//! - `CourseSearchResult`: Course found in search
//! - `ContentSearchResult`: Content item found in search
//! - `SearchSuggestion`: Autocomplete suggestion
//! - `SearchFilters`: Available filter facets

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Type of search to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    /// Full-text search (default)
    #[default]
    FullText,
    /// Semantic/vector similarity search
    Semantic,
    /// Combined full-text + semantic
    Hybrid,
}

/// Content type for content search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Lesson,
    Video,
    Article,
    Quiz,
    Resource,
    Transcript,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Lesson => write!(f, "lesson"),
            ContentType::Video => write!(f, "video"),
            ContentType::Article => write!(f, "article"),
            ContentType::Quiz => write!(f, "quiz"),
            ContentType::Resource => write!(f, "resource"),
            ContentType::Transcript => write!(f, "transcript"),
        }
    }
}

/// Course difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl std::fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DifficultyLevel::Beginner => write!(f, "beginner"),
            DifficultyLevel::Intermediate => write!(f, "intermediate"),
            DifficultyLevel::Advanced => write!(f, "advanced"),
            DifficultyLevel::Expert => write!(f, "expert"),
        }
    }
}

/// Sort order for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Most relevant first (default)
    #[default]
    Relevance,
    /// Newest first
    Newest,
    /// Oldest first
    Oldest,
    /// Highest rated first
    Rating,
    /// Lowest price first
    PriceLow,
    /// Highest price first
    PriceHigh,
    /// Most popular (enrollments)
    Popular,
}

// =============================================================================
// SEARCH QUERY
// =============================================================================

/// Search query with filters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search text
    pub query: String,
    /// Type of search
    #[serde(default)]
    pub search_type: SearchType,
    /// Filter by categories
    pub categories: Option<Vec<String>>,
    /// Filter by difficulty levels
    pub levels: Option<Vec<DifficultyLevel>>,
    /// Minimum price filter
    pub price_min: Option<Decimal>,
    /// Maximum price filter
    pub price_max: Option<Decimal>,
    /// Minimum rating filter (1-5)
    pub rating_min: Option<f32>,
    /// Filter by language
    pub language: Option<String>,
    /// Filter by instructor ID
    pub instructor_id: Option<Uuid>,
    /// Include only free courses
    pub free_only: Option<bool>,
    /// Exclude already enrolled courses (requires user_id)
    pub exclude_enrolled: Option<bool>,
    /// User ID for personalization
    pub user_id: Option<Uuid>,
    /// Sort order
    #[serde(default)]
    pub sort: SortOrder,
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Results per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 20 }

impl SearchQuery {
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

// =============================================================================
// SEARCH RESULTS
// =============================================================================

/// Course search result.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseSearchResult {
    pub course_id: Uuid,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub instructor_id: Uuid,
    pub instructor_name: String,
    pub category: Option<String>,
    pub difficulty_level: Option<String>,
    pub language: String,
    pub price: Decimal,
    pub currency: String,
    pub average_rating: Option<Decimal>,
    pub review_count: i32,
    pub enrollment_count: i32,
    pub duration_minutes: Option<i32>,
    pub lesson_count: i32,
    pub created_at: DateTime<Utc>,
    /// Search relevance score (0-1)
    #[sqlx(default)]
    pub relevance_score: Option<f32>,
    /// Text highlight snippets
    #[sqlx(skip)]
    pub highlights: Option<Vec<String>>,
}

/// Content search result (lesson, transcript, etc).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContentSearchResult {
    pub content_id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub lesson_id: Option<Uuid>,
    pub lesson_title: Option<String>,
    pub content_type: String,
    pub title: String,
    pub snippet: Option<String>,
    /// Search relevance score
    #[sqlx(default)]
    pub relevance_score: Option<f32>,
    /// Position in video (seconds) if applicable
    pub timestamp_seconds: Option<i32>,
}

/// Paginated search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub query: String,
    pub search_time_ms: u64,
    pub filters_applied: FiltersApplied,
}

impl<T> SearchResults<T> {
    pub fn new(
        items: Vec<T>,
        total: i64,
        query: &SearchQuery,
        search_time_ms: u64,
    ) -> Self {
        let total_pages = ((total as f64) / (query.per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page: query.page,
            per_page: query.per_page,
            total_pages,
            query: query.query.clone(),
            search_time_ms,
            filters_applied: FiltersApplied::from_query(query),
        }
    }
}

/// Summary of which filters were applied.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FiltersApplied {
    pub categories: Option<Vec<String>>,
    pub levels: Option<Vec<String>>,
    pub price_range: Option<(Decimal, Decimal)>,
    pub rating_min: Option<f32>,
    pub language: Option<String>,
    pub free_only: bool,
}

impl FiltersApplied {
    pub fn from_query(q: &SearchQuery) -> Self {
        Self {
            categories: q.categories.clone(),
            levels: q.levels.as_ref().map(|l| l.iter().map(|d| d.to_string()).collect()),
            price_range: match (q.price_min, q.price_max) {
                (Some(min), Some(max)) => Some((min, max)),
                _ => None,
            },
            rating_min: q.rating_min,
            language: q.language.clone(),
            free_only: q.free_only.unwrap_or(false),
        }
    }
}

// =============================================================================
// SUGGESTIONS & FACETS
// =============================================================================

/// Autocomplete suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub suggestion_type: SuggestionType,
    /// Number of results this would return
    pub result_count: Option<i64>,
}

/// Type of suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    /// Course title match
    Course,
    /// Category match
    Category,
    /// Instructor name match
    Instructor,
    /// Recent search by user
    Recent,
    /// Popular search
    Popular,
}

/// Available filter facets with counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacets {
    pub categories: Vec<FacetCount>,
    pub levels: Vec<FacetCount>,
    pub languages: Vec<FacetCount>,
    pub price_ranges: Vec<PriceRangeFacet>,
    pub ratings: Vec<RatingFacet>,
}

/// Facet with count.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FacetCount {
    pub value: String,
    pub count: i64,
}

/// Price range facet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRangeFacet {
    pub label: String,
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
    pub count: i64,
}

/// Rating facet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingFacet {
    pub min_rating: f32,
    pub label: String,
    pub count: i64,
}

// =============================================================================
// SEMANTIC SEARCH
// =============================================================================

/// Semantic search result with similarity score.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SemanticSearchResult {
    pub content_id: Uuid,
    pub content_type: String,
    pub course_id: Uuid,
    pub course_title: String,
    pub lesson_id: Option<Uuid>,
    pub lesson_title: Option<String>,
    pub text_chunk: String,
    /// Cosine similarity score (0-1)
    pub similarity: f32,
    /// Timestamp if from video transcript
    pub timestamp_seconds: Option<i32>,
}

// =============================================================================
// SAVED SEARCHES & HISTORY
// =============================================================================

/// User's saved search.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SavedSearch {
    pub saved_search_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub query: String,
    pub filters_json: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Recent search entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSearch {
    pub query: String,
    pub searched_at: DateTime<Utc>,
    pub result_count: i64,
}
