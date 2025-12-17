//! # Suggestion Service
//!
//! Autocomplete and search suggestions.

use std::sync::Arc;

use redis::AsyncCommands;
use uuid::Uuid;

use crate::domain::{SearchError, SearchResult, SearchSuggestion, SuggestionType};

const POPULAR_SEARCHES_KEY: &str = "search:popular";
const RECENT_SEARCHES_PREFIX: &str = "search:recent:";
const MAX_SUGGESTIONS: usize = 10;

/// Search suggestion service.
#[derive(Clone)]
pub struct SuggestionService {
    pool: sqlx::PgPool,
    redis: Arc<redis::Client>,
}

impl SuggestionService {
    /// Create a new suggestion service.
    pub fn new(pool: sqlx::PgPool, redis: redis::Client) -> Self {
        Self {
            pool,
            redis: Arc::new(redis),
        }
    }

    /// Get suggestions for a partial query.
    pub async fn get_suggestions(
        &self,
        query: &str,
        user_id: Option<Uuid>,
        limit: usize,
    ) -> SearchResult<Vec<SearchSuggestion>> {
        let mut suggestions = Vec::new();
        let query_lower = query.to_lowercase();

        // 1. User's recent searches (if authenticated)
        if let Some(uid) = user_id {
            let recent = self.get_recent_searches(uid, &query_lower).await;
            suggestions.extend(recent);
        }

        // 2. Popular searches matching prefix
        let popular = self.get_popular_searches(&query_lower).await;
        suggestions.extend(popular);

        // 3. Course title matches
        let courses = self.get_course_suggestions(&query_lower).await?;
        suggestions.extend(courses);

        // 4. Category matches
        let categories = self.get_category_suggestions(&query_lower).await?;
        suggestions.extend(categories);

        // 5. Instructor matches
        let instructors = self.get_instructor_suggestions(&query_lower).await?;
        suggestions.extend(instructors);

        // Deduplicate and limit
        let mut seen = std::collections::HashSet::new();
        suggestions.retain(|s| seen.insert(s.text.to_lowercase()));
        suggestions.truncate(limit.min(MAX_SUGGESTIONS));

        Ok(suggestions)
    }

    /// Record a search for analytics and suggestions.
    pub async fn record_search(
        &self,
        query: &str,
        user_id: Option<Uuid>,
        result_count: i64,
    ) -> SearchResult<()> {
        let query_normalized = query.to_lowercase().trim().to_string();

        if query_normalized.len() < 2 {
            return Ok(());
        }

        // Increment in popular searches (sorted set)
        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            let _: Result<f64, _> = conn
                .zincr::<_, _, _, f64>(POPULAR_SEARCHES_KEY, &query_normalized, 1.0)
                .await;

            // Add to user's recent searches
            if let Some(uid) = user_id {
                let key = format!("{}{}", RECENT_SEARCHES_PREFIX, uid);
                let _: Result<i64, _> = conn
                    .lpush::<_, _, i64>(&key, &query_normalized)
                    .await;
                let _: Result<(), _> = conn
                    .ltrim::<_, ()>(&key, 0, 19) // Keep last 20
                    .await;
            }
        }

        // Also store in DB for analytics
        let _ = sqlx::query(
            r#"
            INSERT INTO search_analytics (query, user_id, result_count, searched_at)
            VALUES ($1, $2, $3, NOW())
            "#,
        )
        .bind(&query_normalized)
        .bind(user_id)
        .bind(result_count)
        .execute(&self.pool)
        .await;

        Ok(())
    }

    async fn get_recent_searches(&self, user_id: Uuid, prefix: &str) -> Vec<SearchSuggestion> {
        let key = format!("{}{}", RECENT_SEARCHES_PREFIX, user_id);

        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return vec![],
        };

        let recent: Vec<String> = conn
            .lrange(&key, 0, 19)
            .await
            .unwrap_or_default();

        recent
            .into_iter()
            .filter(|s| s.starts_with(prefix))
            .take(3)
            .map(|text| SearchSuggestion {
                text,
                suggestion_type: SuggestionType::Recent,
                result_count: None,
            })
            .collect()
    }

    async fn get_popular_searches(&self, prefix: &str) -> Vec<SearchSuggestion> {
        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return vec![],
        };

        // Get top popular searches
        let popular: Vec<(String, f64)> = conn
            .zrevrange_withscores(POPULAR_SEARCHES_KEY, 0, 99)
            .await
            .unwrap_or_default();

        popular
            .into_iter()
            .filter(|(s, _)| s.starts_with(prefix))
            .take(3)
            .map(|(text, _score)| SearchSuggestion {
                text,
                suggestion_type: SuggestionType::Popular,
                result_count: None,
            })
            .collect()
    }

    async fn get_course_suggestions(
        &self,
        prefix: &str,
    ) -> SearchResult<Vec<SearchSuggestion>> {
        let pattern = format!("{}%", prefix);

        let suggestions: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT title, enrollment_count::bigint
            FROM courses
            WHERE status = 'published'
            AND LOWER(title) LIKE $1
            ORDER BY enrollment_count DESC
            LIMIT 5
            "#,
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|(text, count)| SearchSuggestion {
                text,
                suggestion_type: SuggestionType::Course,
                result_count: Some(count),
            })
            .collect())
    }

    async fn get_category_suggestions(
        &self,
        prefix: &str,
    ) -> SearchResult<Vec<SearchSuggestion>> {
        let pattern = format!("{}%", prefix);

        let suggestions: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT cat.name, COUNT(c.course_id)::bigint as count
            FROM categories cat
            LEFT JOIN courses c ON cat.category_id = c.category_id AND c.status = 'published'
            WHERE LOWER(cat.name) LIKE $1
            GROUP BY cat.name
            ORDER BY count DESC
            LIMIT 3
            "#,
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|(text, count)| SearchSuggestion {
                text,
                suggestion_type: SuggestionType::Category,
                result_count: Some(count),
            })
            .collect())
    }

    async fn get_instructor_suggestions(
        &self,
        prefix: &str,
    ) -> SearchResult<Vec<SearchSuggestion>> {
        let pattern = format!("{}%", prefix);

        let suggestions: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT u.full_name, COUNT(c.course_id)::bigint as course_count
            FROM users u
            JOIN courses c ON u.user_id = c.instructor_id AND c.status = 'published'
            WHERE LOWER(u.full_name) LIKE $1
            AND u.role = 'instructor'
            GROUP BY u.full_name
            ORDER BY course_count DESC
            LIMIT 3
            "#,
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions
            .into_iter()
            .map(|(text, count)| SearchSuggestion {
                text,
                suggestion_type: SuggestionType::Instructor,
                result_count: Some(count),
            })
            .collect())
    }
}
