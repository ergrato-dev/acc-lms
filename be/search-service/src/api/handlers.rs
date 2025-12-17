//! # HTTP Handlers
//!
//! Request handlers for search API endpoints.

use actix_web::{web, HttpResponse};

use crate::api::dto::{
    ContentSearchRequest, ContentSearchResponse, ContentSearchResultDto, CourseSearchRequest,
    CourseSearchResponse, CourseSearchResultDto, CreateSavedSearchRequest, SavedSearchDto,
    SearchFacetsDto, SemanticSearchRequest, SemanticSearchResponse, SemanticSearchResultDto,
    SuggestionDto, SuggestionsRequest, SuggestionsResponse,
};
use crate::domain::{SearchError, SearchQuery, SearchType, SortOrder};
use crate::services::{ContentSearchService, SearchService, SemanticSearchService, SuggestionService};

// =============================================================================
// COURSE SEARCH
// =============================================================================

/// Search courses.
///
/// GET /api/v1/search/courses
pub async fn search_courses(
    query: web::Query<CourseSearchRequest>,
    search_service: web::Data<SearchService>,
) -> Result<HttpResponse, SearchError> {
    // Validate query
    if query.query.trim().is_empty() {
        return Err(SearchError::EmptyQuery);
    }
    if query.query.len() > 500 {
        return Err(SearchError::QueryTooLong {
            max: 500,
            actual: query.query.len(),
        });
    }

    // Build search query
    let search_query = SearchQuery {
        query: query.query.clone(),
        search_type: query.search_type.unwrap_or(SearchType::FullText),
        categories: query.categories.clone(),
        levels: query.levels.clone(),
        price_min: query.price_min,
        price_max: query.price_max,
        rating_min: query.rating_min,
        language: query.language.clone(),
        instructor_id: query.instructor_id,
        free_only: query.free_only,
        exclude_enrolled: query.exclude_enrolled,
        user_id: None, // Would come from auth context
        sort: query.sort,
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    // Execute search
    let (results, search_time) = search_service.search_courses(&search_query).await?;

    // Get facets if this is first page
    let facets = if search_query.page == 1 {
        Some(SearchFacetsDto::from(search_service.get_facets(&search_query).await?))
    } else {
        None
    };

    let response = CourseSearchResponse {
        results: results.items.into_iter().map(CourseSearchResultDto::from).collect(),
        total: results.total,
        page: results.page,
        per_page: results.per_page,
        total_pages: results.total_pages,
        query: results.query,
        search_time_ms: search_time,
        filters_applied: results.filters_applied,
        facets,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get search facets only.
///
/// GET /api/v1/search/facets
pub async fn get_facets(
    query: web::Query<CourseSearchRequest>,
    search_service: web::Data<SearchService>,
) -> Result<HttpResponse, SearchError> {
    let search_query = SearchQuery {
        query: query.query.clone(),
        categories: query.categories.clone(),
        levels: query.levels.clone(),
        price_min: query.price_min,
        price_max: query.price_max,
        rating_min: query.rating_min,
        language: query.language.clone(),
        ..Default::default()
    };

    let facets = search_service.get_facets(&search_query).await?;
    Ok(HttpResponse::Ok().json(SearchFacetsDto::from(facets)))
}

// =============================================================================
// CONTENT SEARCH
// =============================================================================

/// Search content within enrolled courses.
///
/// GET /api/v1/search/content
pub async fn search_content(
    query: web::Query<ContentSearchRequest>,
    content_service: web::Data<ContentSearchService>,
) -> Result<HttpResponse, SearchError> {
    // Validate query
    if query.query.trim().is_empty() {
        return Err(SearchError::EmptyQuery);
    }

    // TODO: Get user_id from auth context
    let user_id = uuid::Uuid::nil();

    let (results, search_time) = content_service
        .search(
            user_id,
            &query.query,
            query.course_id,
            query.content_types.as_deref(),
            query.page.unwrap_or(1).max(1),
            query.per_page.unwrap_or(20).clamp(1, 100),
        )
        .await?;

    let response = ContentSearchResponse {
        results: results.items.into_iter().map(ContentSearchResultDto::from).collect(),
        total: results.total,
        page: results.page,
        per_page: results.per_page,
        total_pages: results.total_pages,
        query: results.query,
        search_time_ms: search_time,
    };

    Ok(HttpResponse::Ok().json(response))
}

// =============================================================================
// SEMANTIC SEARCH
// =============================================================================

/// Semantic search within course content.
///
/// POST /api/v1/search/semantic
pub async fn semantic_search(
    body: web::Json<SemanticSearchRequest>,
    semantic_service: web::Data<SemanticSearchService>,
) -> Result<HttpResponse, SearchError> {
    // Validate query
    if body.query.trim().is_empty() {
        return Err(SearchError::EmptyQuery);
    }
    if body.query.len() > 1000 {
        return Err(SearchError::QueryTooLong {
            max: 1000,
            actual: body.query.len(),
        });
    }

    // TODO: Get user_id from auth context
    let user_id = uuid::Uuid::nil();

    let (results, search_time) = semantic_service
        .search(
            user_id,
            &body.query,
            body.course_id,
            body.limit.unwrap_or(10) as usize,
            body.threshold.unwrap_or(0.7),
        )
        .await?;

    let response = SemanticSearchResponse {
        results: results.into_iter().map(SemanticSearchResultDto::from).collect(),
        query: body.query.clone(),
        search_time_ms: search_time,
    };

    Ok(HttpResponse::Ok().json(response))
}

// =============================================================================
// SUGGESTIONS
// =============================================================================

/// Get search suggestions/autocomplete.
///
/// GET /api/v1/search/suggest
pub async fn suggest(
    query: web::Query<SuggestionsRequest>,
    suggestion_service: web::Data<SuggestionService>,
) -> Result<HttpResponse, SearchError> {
    if query.query.trim().is_empty() {
        return Ok(HttpResponse::Ok().json(SuggestionsResponse { suggestions: vec![] }));
    }

    // TODO: Get user_id from auth context for personalized suggestions
    let user_id = None;

    let suggestions = suggestion_service
        .get_suggestions(&query.query, user_id, query.limit.unwrap_or(10) as usize)
        .await?;

    let response = SuggestionsResponse {
        suggestions: suggestions.into_iter().map(SuggestionDto::from).collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Record a search for suggestions/trending.
///
/// POST /api/v1/search/record
pub async fn record_search(
    body: web::Json<serde_json::Value>,
    suggestion_service: web::Data<SuggestionService>,
) -> Result<HttpResponse, SearchError> {
    let query = body
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let result_count = body
        .get("result_count")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // TODO: Get user_id from auth context
    let user_id = None;

    suggestion_service.record_search(query, user_id, result_count).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "recorded": true })))
}

// =============================================================================
// SAVED SEARCHES
// =============================================================================

/// Save a search for later.
///
/// POST /api/v1/search/saved
pub async fn create_saved_search(
    body: web::Json<CreateSavedSearchRequest>,
    search_service: web::Data<SearchService>,
) -> Result<HttpResponse, SearchError> {
    // TODO: Get user_id from auth context
    let user_id = uuid::Uuid::nil();

    let saved = search_service
        .save_search(user_id, &body.name, &body.query, body.filters.clone())
        .await?;

    let response = SavedSearchDto {
        id: saved.saved_search_id,
        name: saved.name,
        query: saved.query,
        filters: saved.filters_json,
        created_at: saved.created_at.to_rfc3339(),
    };

    Ok(HttpResponse::Created().json(response))
}

/// List saved searches.
///
/// GET /api/v1/search/saved
pub async fn list_saved_searches(
    search_service: web::Data<SearchService>,
) -> Result<HttpResponse, SearchError> {
    // TODO: Get user_id from auth context
    let user_id = uuid::Uuid::nil();

    let saved_searches = search_service.list_saved_searches(user_id).await?;

    let response: Vec<SavedSearchDto> = saved_searches
        .into_iter()
        .map(|s| SavedSearchDto {
            id: s.saved_search_id,
            name: s.name,
            query: s.query,
            filters: s.filters_json,
            created_at: s.created_at.to_rfc3339(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

/// Delete a saved search.
///
/// DELETE /api/v1/search/saved/{id}
pub async fn delete_saved_search(
    path: web::Path<uuid::Uuid>,
    search_service: web::Data<SearchService>,
) -> Result<HttpResponse, SearchError> {
    // TODO: Get user_id from auth context
    let user_id = uuid::Uuid::nil();
    let saved_search_id = path.into_inner();

    search_service.delete_saved_search(user_id, saved_search_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// HEALTH
// =============================================================================

/// Health check endpoint.
///
/// GET /health
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "search-service"
    }))
}

/// Readiness check.
///
/// GET /ready
pub async fn readiness_check(
    search_service: web::Data<SearchService>,
) -> HttpResponse {
    // Check database connectivity
    match search_service.health_check().await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ready",
            "database": "connected"
        })),
        _ => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "not_ready",
            "database": "disconnected"
        })),
    }
}

// =============================================================================
// ROUTE CONFIGURATION
// =============================================================================

/// Configure search routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/search")
            .route("/courses", web::get().to(search_courses))
            .route("/facets", web::get().to(get_facets))
            .route("/content", web::get().to(search_content))
            .route("/semantic", web::post().to(semantic_search))
            .route("/suggest", web::get().to(suggest))
            .route("/record", web::post().to(record_search))
            .route("/saved", web::post().to(create_saved_search))
            .route("/saved", web::get().to(list_saved_searches))
            .route("/saved/{id}", web::delete().to(delete_saved_search)),
    )
    .route("/health", web::get().to(health_check))
    .route("/ready", web::get().to(readiness_check));
}
