//! # Course Search Repository
//!
//! Database access for course search operations.

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    CourseSearchResult, FacetCount, PriceRangeFacet, RatingFacet, SavedSearch, SearchError,
    SearchFacets, SearchQuery, SearchResult, SearchResults, SortOrder,
};

/// Repository for course search operations.
#[derive(Debug, Clone)]
pub struct CourseSearchRepository {
    pool: PgPool,
}

impl CourseSearchRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Search courses with full-text search.
    pub async fn search_courses(
        &self,
        query: &SearchQuery,
    ) -> SearchResult<SearchResults<CourseSearchResult>> {
        // Build dynamic query based on filters
        let search_term = format!("%{}%", query.query.to_lowercase());

        // For a real implementation, we'd use PostgreSQL full-text search with ts_query
        let sql = r#"
            SELECT
                c.course_id,
                c.title,
                c.slug,
                c.short_description,
                c.thumbnail_url,
                c.instructor_id,
                u.full_name as instructor_name,
                cat.name as category,
                c.difficulty_level,
                c.language,
                COALESCE(c.price, 0) as price,
                COALESCE(c.currency, 'USD') as currency,
                c.average_rating,
                COALESCE(c.review_count, 0) as review_count,
                COALESCE(c.enrollment_count, 0) as enrollment_count,
                c.duration_minutes,
                (SELECT COUNT(*)::int FROM lessons l WHERE l.course_id = c.course_id) as lesson_count,
                c.created_at,
                ts_rank(
                    to_tsvector('spanish', COALESCE(c.title, '') || ' ' || COALESCE(c.short_description, '')),
                    plainto_tsquery('spanish', $1)
                ) as relevance_score
            FROM courses c
            LEFT JOIN users u ON c.instructor_id = u.user_id
            LEFT JOIN categories cat ON c.category_id = cat.category_id
            WHERE c.status = 'published'
            AND (
                c.title ILIKE $2
                OR c.short_description ILIKE $2
                OR to_tsvector('spanish', COALESCE(c.title, '') || ' ' || COALESCE(c.short_description, ''))
                   @@ plainto_tsquery('spanish', $1)
            )
        "#;

        let mut conditions = String::new();

        // Category filter
        if let Some(ref categories) = query.categories {
            if !categories.is_empty() {
                let cat_list = categories.iter()
                    .map(|c| format!("'{}'", c.replace("'", "''")))
                    .collect::<Vec<_>>()
                    .join(",");
                conditions.push_str(&format!(" AND cat.name IN ({})", cat_list));
            }
        }

        // Level filter
        if let Some(ref levels) = query.levels {
            if !levels.is_empty() {
                let level_list = levels.iter()
                    .map(|l| format!("'{}'", l))
                    .collect::<Vec<_>>()
                    .join(",");
                conditions.push_str(&format!(" AND c.difficulty_level IN ({})", level_list));
            }
        }

        // Price filters
        if let Some(min) = query.price_min {
            conditions.push_str(&format!(" AND c.price >= {}", min));
        }
        if let Some(max) = query.price_max {
            conditions.push_str(&format!(" AND c.price <= {}", max));
        }

        // Free only
        if query.free_only == Some(true) {
            conditions.push_str(" AND (c.price IS NULL OR c.price = 0)");
        }

        // Rating filter
        if let Some(min_rating) = query.rating_min {
            conditions.push_str(&format!(" AND c.average_rating >= {}", min_rating));
        }

        // Language filter
        if let Some(ref lang) = query.language {
            conditions.push_str(&format!(" AND c.language = '{}'", lang.replace("'", "''")));
        }

        // Instructor filter
        if let Some(instructor_id) = query.instructor_id {
            conditions.push_str(&format!(" AND c.instructor_id = '{}'", instructor_id));
        }

        // Exclude enrolled courses
        if query.exclude_enrolled == Some(true) {
            if let Some(user_id) = query.user_id {
                conditions.push_str(&format!(
                    " AND c.course_id NOT IN (SELECT course_id FROM enrollments WHERE user_id = '{}')",
                    user_id
                ));
            }
        }

        // Sort order
        let order_by = match query.sort {
            SortOrder::Relevance => "relevance_score DESC NULLS LAST, enrollment_count DESC",
            SortOrder::Newest => "c.created_at DESC",
            SortOrder::Oldest => "c.created_at ASC",
            SortOrder::Rating => "c.average_rating DESC NULLS LAST",
            SortOrder::PriceLow => "c.price ASC NULLS FIRST",
            SortOrder::PriceHigh => "c.price DESC NULLS LAST",
            SortOrder::Popular => "c.enrollment_count DESC",
        };

        let full_sql = format!(
            "{}{} ORDER BY {} LIMIT $3 OFFSET $4",
            sql, conditions, order_by
        );

        let results: Vec<CourseSearchResult> = sqlx::query_as(&full_sql)
            .bind(&query.query)
            .bind(&search_term)
            .bind(query.limit())
            .bind(query.offset())
            .fetch_all(&self.pool)
            .await?;

        // Get total count
        let count_sql = format!(
            r#"
            SELECT COUNT(*) as count
            FROM courses c
            LEFT JOIN users u ON c.instructor_id = u.user_id
            LEFT JOIN categories cat ON c.category_id = cat.category_id
            WHERE c.status = 'published'
            AND (
                c.title ILIKE $2
                OR c.short_description ILIKE $2
                OR to_tsvector('spanish', COALESCE(c.title, '') || ' ' || COALESCE(c.short_description, ''))
                   @@ plainto_tsquery('spanish', $1)
            ){}
            "#,
            conditions
        );

        let total: i64 = sqlx::query_scalar(&count_sql)
            .bind(&query.query)
            .bind(&search_term)
            .fetch_one(&self.pool)
            .await?;

        Ok(SearchResults::new(results, total, query, 0))
    }

    /// Get search facets (counts for filters).
    pub async fn get_facets(&self, query: &SearchQuery) -> SearchResult<SearchFacets> {
        let search_term = format!("%{}%", query.query.to_lowercase());

        // Category facets
        let categories: Vec<FacetCount> = sqlx::query_as(
            r#"
            SELECT cat.name as value, COUNT(*)::bigint as count
            FROM courses c
            JOIN categories cat ON c.category_id = cat.category_id
            WHERE c.status = 'published'
            AND (c.title ILIKE $1 OR c.short_description ILIKE $1)
            GROUP BY cat.name
            ORDER BY count DESC
            LIMIT 20
            "#,
        )
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        // Level facets
        let levels: Vec<FacetCount> = sqlx::query_as(
            r#"
            SELECT difficulty_level as value, COUNT(*)::bigint as count
            FROM courses
            WHERE status = 'published'
            AND difficulty_level IS NOT NULL
            AND (title ILIKE $1 OR short_description ILIKE $1)
            GROUP BY difficulty_level
            ORDER BY
                CASE difficulty_level
                    WHEN 'beginner' THEN 1
                    WHEN 'intermediate' THEN 2
                    WHEN 'advanced' THEN 3
                    WHEN 'expert' THEN 4
                END
            "#,
        )
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        // Language facets
        let languages: Vec<FacetCount> = sqlx::query_as(
            r#"
            SELECT language as value, COUNT(*)::bigint as count
            FROM courses
            WHERE status = 'published'
            AND (title ILIKE $1 OR short_description ILIKE $1)
            GROUP BY language
            ORDER BY count DESC
            LIMIT 10
            "#,
        )
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        // Price range facets
        let price_ranges = vec![
            PriceRangeFacet {
                label: "Free".to_string(),
                min: None,
                max: Some(Decimal::ZERO),
                count: self.count_by_price_range(&search_term, None, Some(Decimal::ZERO)).await?,
            },
            PriceRangeFacet {
                label: "$1 - $50".to_string(),
                min: Some(Decimal::from(1)),
                max: Some(Decimal::from(50)),
                count: self.count_by_price_range(&search_term, Some(Decimal::from(1)), Some(Decimal::from(50))).await?,
            },
            PriceRangeFacet {
                label: "$51 - $100".to_string(),
                min: Some(Decimal::from(51)),
                max: Some(Decimal::from(100)),
                count: self.count_by_price_range(&search_term, Some(Decimal::from(51)), Some(Decimal::from(100))).await?,
            },
            PriceRangeFacet {
                label: "$100+".to_string(),
                min: Some(Decimal::from(100)),
                max: None,
                count: self.count_by_price_range(&search_term, Some(Decimal::from(100)), None).await?,
            },
        ];

        // Rating facets
        let ratings = vec![
            RatingFacet {
                min_rating: 4.5,
                label: "4.5 & up".to_string(),
                count: self.count_by_rating(&search_term, 4.5).await?,
            },
            RatingFacet {
                min_rating: 4.0,
                label: "4.0 & up".to_string(),
                count: self.count_by_rating(&search_term, 4.0).await?,
            },
            RatingFacet {
                min_rating: 3.5,
                label: "3.5 & up".to_string(),
                count: self.count_by_rating(&search_term, 3.5).await?,
            },
            RatingFacet {
                min_rating: 3.0,
                label: "3.0 & up".to_string(),
                count: self.count_by_rating(&search_term, 3.0).await?,
            },
        ];

        Ok(SearchFacets {
            categories,
            levels,
            languages,
            price_ranges,
            ratings,
        })
    }

    async fn count_by_price_range(
        &self,
        search_term: &str,
        min: Option<Decimal>,
        max: Option<Decimal>,
    ) -> SearchResult<i64> {
        let sql = match (min, max) {
            (None, Some(max)) => format!(
                "SELECT COUNT(*) FROM courses WHERE status = 'published' AND (price IS NULL OR price <= {}) AND (title ILIKE $1 OR short_description ILIKE $1)",
                max
            ),
            (Some(min), Some(max)) => format!(
                "SELECT COUNT(*) FROM courses WHERE status = 'published' AND price >= {} AND price <= {} AND (title ILIKE $1 OR short_description ILIKE $1)",
                min, max
            ),
            (Some(min), None) => format!(
                "SELECT COUNT(*) FROM courses WHERE status = 'published' AND price >= {} AND (title ILIKE $1 OR short_description ILIKE $1)",
                min
            ),
            (None, None) => {
                "SELECT COUNT(*) FROM courses WHERE status = 'published' AND (title ILIKE $1 OR short_description ILIKE $1)".to_string()
            }
        };

        let count: i64 = sqlx::query_scalar(&sql)
            .bind(search_term)
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn count_by_rating(&self, search_term: &str, min_rating: f32) -> SearchResult<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM courses
            WHERE status = 'published'
            AND average_rating >= $1
            AND (title ILIKE $2 OR short_description ILIKE $2)
            "#,
        )
        .bind(min_rating)
        .bind(search_term)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Save a search for later.
    pub async fn save_search(
        &self,
        user_id: Uuid,
        name: &str,
        query: &str,
        filters: Option<serde_json::Value>,
    ) -> SearchResult<SavedSearch> {
        let saved: SavedSearch = sqlx::query_as(
            r#"
            INSERT INTO saved_searches (user_id, name, query, filters_json)
            VALUES ($1, $2, $3, $4)
            RETURNING saved_search_id, user_id, name, query, filters_json, created_at
            "#,
        )
        .bind(user_id)
        .bind(name)
        .bind(query)
        .bind(filters)
        .fetch_one(&self.pool)
        .await?;

        Ok(saved)
    }

    /// List saved searches for a user.
    pub async fn list_saved_searches(&self, user_id: Uuid) -> SearchResult<Vec<SavedSearch>> {
        let searches: Vec<SavedSearch> = sqlx::query_as(
            r#"
            SELECT saved_search_id, user_id, name, query, filters_json, created_at
            FROM saved_searches
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(searches)
    }

    /// Delete a saved search.
    pub async fn delete_saved_search(
        &self,
        user_id: Uuid,
        saved_search_id: Uuid,
    ) -> SearchResult<()> {
        sqlx::query(
            "DELETE FROM saved_searches WHERE user_id = $1 AND saved_search_id = $2",
        )
        .bind(user_id)
        .bind(saved_search_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Check database health.
    pub async fn health_check(&self) -> SearchResult<bool> {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(true)
    }
}
