//! # API Data Transfer Objects
//!
//! Request and response types for the search API.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    ContentSearchResult, CourseSearchResult, DifficultyLevel, FacetCount, FiltersApplied,
    PriceRangeFacet, RatingFacet, SearchFacets, SearchSuggestion, SearchType, SemanticSearchResult,
    SortOrder,
};

// =============================================================================
// COURSE SEARCH
// =============================================================================

/// Course search request.
#[derive(Debug, Clone, Deserialize)]
pub struct CourseSearchRequest {
    /// Search query text
    #[serde(rename = "q")]
    pub query: String,
    /// Search type
    #[serde(default)]
    pub search_type: Option<SearchType>,
    /// Categories filter
    pub categories: Option<Vec<String>>,
    /// Difficulty levels filter
    pub levels: Option<Vec<DifficultyLevel>>,
    /// Minimum price
    pub price_min: Option<Decimal>,
    /// Maximum price
    pub price_max: Option<Decimal>,
    /// Minimum rating (1-5)
    pub rating_min: Option<f32>,
    /// Language filter
    pub language: Option<String>,
    /// Instructor filter
    pub instructor_id: Option<Uuid>,
    /// Only free courses
    pub free_only: Option<bool>,
    /// Exclude enrolled courses (requires auth)
    pub exclude_enrolled: Option<bool>,
    /// Sort order
    #[serde(default)]
    pub sort: SortOrder,
    /// Page number
    pub page: Option<i32>,
    /// Results per page
    pub per_page: Option<i32>,
}

/// Course search response.
#[derive(Debug, Clone, Serialize)]
pub struct CourseSearchResponse {
    pub results: Vec<CourseSearchResultDto>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub query: String,
    pub search_time_ms: u64,
    pub filters_applied: FiltersApplied,
    pub facets: Option<SearchFacetsDto>,
}

/// Course result DTO.
#[derive(Debug, Clone, Serialize)]
pub struct CourseSearchResultDto {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub instructor: InstructorDto,
    pub category: Option<String>,
    pub level: Option<String>,
    pub language: String,
    pub price: PriceDto,
    pub rating: Option<RatingDto>,
    pub enrollment_count: i32,
    pub duration_minutes: Option<i32>,
    pub lesson_count: i32,
    pub relevance_score: Option<f32>,
    pub highlights: Option<Vec<String>>,
}

impl From<CourseSearchResult> for CourseSearchResultDto {
    fn from(r: CourseSearchResult) -> Self {
        Self {
            id: r.course_id,
            title: r.title,
            slug: r.slug,
            description: r.short_description,
            thumbnail_url: r.thumbnail_url,
            instructor: InstructorDto {
                id: r.instructor_id,
                name: r.instructor_name,
            },
            category: r.category,
            level: r.difficulty_level,
            language: r.language,
            price: PriceDto {
                amount: r.price,
                currency: r.currency,
            },
            rating: r.average_rating.map(|avg| RatingDto {
                average: avg,
                count: r.review_count,
            }),
            enrollment_count: r.enrollment_count,
            duration_minutes: r.duration_minutes,
            lesson_count: r.lesson_count,
            relevance_score: r.relevance_score,
            highlights: r.highlights,
        }
    }
}

/// Instructor DTO.
#[derive(Debug, Clone, Serialize)]
pub struct InstructorDto {
    pub id: Uuid,
    pub name: String,
}

/// Price DTO.
#[derive(Debug, Clone, Serialize)]
pub struct PriceDto {
    pub amount: Decimal,
    pub currency: String,
}

/// Rating DTO.
#[derive(Debug, Clone, Serialize)]
pub struct RatingDto {
    pub average: Decimal,
    pub count: i32,
}

// =============================================================================
// CONTENT SEARCH
// =============================================================================

/// Content search request (within enrolled courses).
#[derive(Debug, Clone, Deserialize)]
pub struct ContentSearchRequest {
    /// Search query
    #[serde(rename = "q")]
    pub query: String,
    /// Limit to specific course
    pub course_id: Option<Uuid>,
    /// Content types filter
    pub content_types: Option<Vec<String>>,
    /// Page number
    pub page: Option<i32>,
    /// Results per page
    pub per_page: Option<i32>,
}

/// Content search response.
#[derive(Debug, Clone, Serialize)]
pub struct ContentSearchResponse {
    pub results: Vec<ContentSearchResultDto>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub query: String,
    pub search_time_ms: u64,
}

/// Content result DTO.
#[derive(Debug, Clone, Serialize)]
pub struct ContentSearchResultDto {
    pub id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub lesson_id: Option<Uuid>,
    pub lesson_title: Option<String>,
    pub content_type: String,
    pub title: String,
    pub snippet: Option<String>,
    pub relevance_score: Option<f32>,
    pub timestamp_seconds: Option<i32>,
}

impl From<ContentSearchResult> for ContentSearchResultDto {
    fn from(r: ContentSearchResult) -> Self {
        Self {
            id: r.content_id,
            course_id: r.course_id,
            course_title: r.course_title,
            lesson_id: r.lesson_id,
            lesson_title: r.lesson_title,
            content_type: r.content_type,
            title: r.title,
            snippet: r.snippet,
            relevance_score: r.relevance_score,
            timestamp_seconds: r.timestamp_seconds,
        }
    }
}

// =============================================================================
// SEMANTIC SEARCH
// =============================================================================

/// Semantic search request.
#[derive(Debug, Clone, Deserialize)]
pub struct SemanticSearchRequest {
    /// Natural language query
    #[serde(rename = "q")]
    pub query: String,
    /// Limit to specific course
    pub course_id: Option<Uuid>,
    /// Number of results
    pub limit: Option<i32>,
    /// Minimum similarity threshold (0-1)
    pub threshold: Option<f32>,
}

/// Semantic search response.
#[derive(Debug, Clone, Serialize)]
pub struct SemanticSearchResponse {
    pub results: Vec<SemanticSearchResultDto>,
    pub query: String,
    pub search_time_ms: u64,
}

/// Semantic result DTO.
#[derive(Debug, Clone, Serialize)]
pub struct SemanticSearchResultDto {
    pub content_id: Uuid,
    pub content_type: String,
    pub course_id: Uuid,
    pub course_title: String,
    pub lesson_id: Option<Uuid>,
    pub lesson_title: Option<String>,
    pub text: String,
    pub similarity: f32,
    pub timestamp_seconds: Option<i32>,
}

impl From<SemanticSearchResult> for SemanticSearchResultDto {
    fn from(r: SemanticSearchResult) -> Self {
        Self {
            content_id: r.content_id,
            content_type: r.content_type,
            course_id: r.course_id,
            course_title: r.course_title,
            lesson_id: r.lesson_id,
            lesson_title: r.lesson_title,
            text: r.text_chunk,
            similarity: r.similarity,
            timestamp_seconds: r.timestamp_seconds,
        }
    }
}

// =============================================================================
// SUGGESTIONS & FACETS
// =============================================================================

/// Autocomplete/suggestions request.
#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionsRequest {
    /// Partial query text
    #[serde(rename = "q")]
    pub query: String,
    /// Max suggestions to return
    pub limit: Option<i32>,
}

/// Suggestions response.
#[derive(Debug, Clone, Serialize)]
pub struct SuggestionsResponse {
    pub suggestions: Vec<SuggestionDto>,
}

/// Suggestion DTO.
#[derive(Debug, Clone, Serialize)]
pub struct SuggestionDto {
    pub text: String,
    #[serde(rename = "type")]
    pub suggestion_type: String,
    pub result_count: Option<i64>,
}

impl From<SearchSuggestion> for SuggestionDto {
    fn from(s: SearchSuggestion) -> Self {
        Self {
            text: s.text,
            suggestion_type: format!("{:?}", s.suggestion_type).to_lowercase(),
            result_count: s.result_count,
        }
    }
}

/// Facets DTO.
#[derive(Debug, Clone, Serialize)]
pub struct SearchFacetsDto {
    pub categories: Vec<FacetCount>,
    pub levels: Vec<FacetCount>,
    pub languages: Vec<FacetCount>,
    pub price_ranges: Vec<PriceRangeFacet>,
    pub ratings: Vec<RatingFacet>,
}

impl From<SearchFacets> for SearchFacetsDto {
    fn from(f: SearchFacets) -> Self {
        Self {
            categories: f.categories,
            levels: f.levels,
            languages: f.languages,
            price_ranges: f.price_ranges,
            ratings: f.ratings,
        }
    }
}

// =============================================================================
// SAVED SEARCHES
// =============================================================================

/// Create saved search request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSavedSearchRequest {
    pub name: String,
    pub query: String,
    pub filters: Option<serde_json::Value>,
}

/// Saved search DTO.
#[derive(Debug, Clone, Serialize)]
pub struct SavedSearchDto {
    pub id: Uuid,
    pub name: String,
    pub query: String,
    pub filters: Option<serde_json::Value>,
    pub created_at: String,
}
