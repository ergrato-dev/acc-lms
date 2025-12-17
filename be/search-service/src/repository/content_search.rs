//! # Content Search Repository
//!
//! Database access for content search within enrolled courses.

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{ContentSearchResult, SearchError, SearchResult, SearchResults, SearchQuery};

/// Repository for content search operations.
#[derive(Debug, Clone)]
pub struct ContentSearchRepository {
    pool: PgPool,
}

impl ContentSearchRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Search content within user's enrolled courses.
    pub async fn search_content(
        &self,
        user_id: Uuid,
        query: &str,
        course_id: Option<Uuid>,
        content_types: Option<&[String]>,
        page: i32,
        per_page: i32,
    ) -> SearchResult<SearchResults<ContentSearchResult>> {
        let search_term = format!("%{}%", query.to_lowercase());
        let offset = ((page - 1) * per_page) as i64;
        let limit = per_page as i64;

        // Build conditions
        let mut conditions = String::new();

        if let Some(cid) = course_id {
            conditions.push_str(&format!(" AND lc.course_id = '{}'", cid));
        }

        if let Some(types) = content_types {
            if !types.is_empty() {
                let type_list = types.iter()
                    .map(|t| format!("'{}'", t.replace("'", "''")))
                    .collect::<Vec<_>>()
                    .join(",");
                conditions.push_str(&format!(" AND lc.content_type IN ({})", type_list));
            }
        }

        // Search in lesson content (lessons, transcripts, etc.)
        let sql = format!(
            r#"
            SELECT
                lc.content_id,
                c.course_id,
                c.title as course_title,
                l.lesson_id,
                l.title as lesson_title,
                lc.content_type,
                lc.title,
                ts_headline('spanish', lc.content_text, plainto_tsquery('spanish', $1),
                    'StartSel=<mark>, StopSel=</mark>, MaxWords=35, MinWords=15') as snippet,
                ts_rank(to_tsvector('spanish', COALESCE(lc.title, '') || ' ' || COALESCE(lc.content_text, '')),
                    plainto_tsquery('spanish', $1)) as relevance_score,
                lc.timestamp_seconds
            FROM lesson_content lc
            JOIN lessons l ON lc.lesson_id = l.lesson_id
            JOIN courses c ON l.course_id = c.course_id
            JOIN enrollments e ON c.course_id = e.course_id
            WHERE e.user_id = $2
            AND e.status = 'active'
            AND (
                lc.title ILIKE $3
                OR lc.content_text ILIKE $3
                OR to_tsvector('spanish', COALESCE(lc.title, '') || ' ' || COALESCE(lc.content_text, ''))
                   @@ plainto_tsquery('spanish', $1)
            )
            {}
            ORDER BY relevance_score DESC NULLS LAST
            LIMIT $4 OFFSET $5
            "#,
            conditions
        );

        let results: Vec<ContentSearchResult> = sqlx::query_as(&sql)
            .bind(query)
            .bind(user_id)
            .bind(&search_term)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        // Get total count
        let count_sql = format!(
            r#"
            SELECT COUNT(*)::bigint
            FROM lesson_content lc
            JOIN lessons l ON lc.lesson_id = l.lesson_id
            JOIN courses c ON l.course_id = c.course_id
            JOIN enrollments e ON c.course_id = e.course_id
            WHERE e.user_id = $2
            AND e.status = 'active'
            AND (
                lc.title ILIKE $3
                OR lc.content_text ILIKE $3
                OR to_tsvector('spanish', COALESCE(lc.title, '') || ' ' || COALESCE(lc.content_text, ''))
                   @@ plainto_tsquery('spanish', $1)
            )
            {}
            "#,
            conditions
        );

        let total: i64 = sqlx::query_scalar(&count_sql)
            .bind(query)
            .bind(user_id)
            .bind(&search_term)
            .fetch_one(&self.pool)
            .await?;

        let search_query = SearchQuery {
            query: query.to_string(),
            page,
            per_page,
            ..Default::default()
        };

        Ok(SearchResults::new(results, total, &search_query, 0))
    }

    /// Search transcripts with timestamps for video content.
    pub async fn search_transcripts(
        &self,
        user_id: Uuid,
        query: &str,
        course_id: Option<Uuid>,
        limit: i32,
    ) -> SearchResult<Vec<ContentSearchResult>> {
        let search_term = format!("%{}%", query.to_lowercase());

        let mut conditions = String::new();
        if let Some(cid) = course_id {
            conditions.push_str(&format!(" AND c.course_id = '{}'", cid));
        }

        let sql = format!(
            r#"
            SELECT
                t.transcript_id as content_id,
                c.course_id,
                c.title as course_title,
                l.lesson_id,
                l.title as lesson_title,
                'transcript' as content_type,
                CONCAT('Transcript at ', t.timestamp_seconds, 's') as title,
                ts_headline('spanish', t.text, plainto_tsquery('spanish', $1),
                    'StartSel=<mark>, StopSel=</mark>, MaxWords=25, MinWords=10') as snippet,
                ts_rank(to_tsvector('spanish', t.text), plainto_tsquery('spanish', $1)) as relevance_score,
                t.timestamp_seconds
            FROM transcripts t
            JOIN lessons l ON t.lesson_id = l.lesson_id
            JOIN courses c ON l.course_id = c.course_id
            JOIN enrollments e ON c.course_id = e.course_id
            WHERE e.user_id = $2
            AND e.status = 'active'
            AND (
                t.text ILIKE $3
                OR to_tsvector('spanish', t.text) @@ plainto_tsquery('spanish', $1)
            )
            {}
            ORDER BY relevance_score DESC NULLS LAST
            LIMIT $4
            "#,
            conditions
        );

        let results: Vec<ContentSearchResult> = sqlx::query_as(&sql)
            .bind(query)
            .bind(user_id)
            .bind(&search_term)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(results)
    }
}
