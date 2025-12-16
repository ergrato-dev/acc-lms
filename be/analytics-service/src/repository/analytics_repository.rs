//! # Analytics Repository
//!
//! Repository implementation for analytics operations using PostgreSQL.

use chrono::{DateTime, Datelike, Utc};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::{
    AnalyticsQuery, CourseAnalytics, CourseStats, Event, EventCount, EventType, Metric,
    NewEvent, NewSession, PageStats, Platform, PlatformStats, Session, TimeSeriesPoint,
    UserEngagement, DeviceInfo, GeoInfo,
};
use crate::domain::value_objects::{DateRange, Pagination, TimeGranularity};

// =============================================================================
// REPOSITORY ERRORS
// =============================================================================

/// Repository-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

// =============================================================================
// ANALYTICS REPOSITORY
// =============================================================================

/// Repository for analytics operations using PostgreSQL.
#[derive(Clone)]
pub struct AnalyticsRepository {
    pool: PgPool,
}

impl AnalyticsRepository {
    /// Creates a new repository instance with a PostgreSQL pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // EVENT OPERATIONS
    // =========================================================================

    /// Inserts a new event.
    pub async fn insert_event(&self, event: NewEvent) -> Result<Event> {
        let event_id = Uuid::new_v4();
        let session_id = event.session_id.unwrap_or_else(Uuid::new_v4);
        let platform = event.platform.unwrap_or(Platform::Unknown);
        let timestamp = Utc::now();
        let created_month = timestamp.date_naive().with_day(1).unwrap();

        // Convert event_type to database enum string
        let (event_type_str, custom_event_name) = event_type_to_db(&event.event_type);
        let platform_str = platform_to_db(&platform);

        // Extract device info
        let (user_agent, browser, browser_version, os, os_version, device_type, screen_width, screen_height) =
            extract_device_info(&event.device_info);

        // Extract geo info
        let (ip_address, country, country_code, region, city, timezone) =
            extract_geo_info(&event.geo_info);

        let properties_json = serde_json::to_value(&event.properties.clone().unwrap_or_default())
            .unwrap_or(serde_json::json!({}));

        sqlx::query(r#"
            INSERT INTO analytics.events (
                event_id, event_type, custom_event_name, tenant_id, user_id, session_id,
                timestamp, page_url, page_title, referrer, platform,
                user_agent, browser, browser_version, os, os_version, device_type,
                screen_width, screen_height, ip_address, country, country_code,
                region, city, timezone, properties, duration_ms, entity_type, entity_id,
                created_month
            ) VALUES (
                $1, $2::analytics.event_type, $3, $4, $5, $6,
                $7, $8, $9, $10, $11::analytics.platform,
                $12, $13, $14, $15, $16, $17,
                $18, $19, $20::inet, $21, $22,
                $23, $24, $25, $26, $27, $28, $29,
                $30
            )
        "#)
        .bind(event_id)
        .bind(event_type_str)
        .bind(&custom_event_name)
        .bind(event.tenant_id)
        .bind(event.user_id)
        .bind(session_id)
        .bind(timestamp)
        .bind(&event.page_url)
        .bind(&event.page_title)
        .bind(&event.referrer)
        .bind(platform_str)
        .bind(&user_agent)
        .bind(&browser)
        .bind(&browser_version)
        .bind(&os)
        .bind(&os_version)
        .bind(&device_type)
        .bind(screen_width)
        .bind(screen_height)
        .bind(&ip_address)
        .bind(&country)
        .bind(&country_code)
        .bind(&region)
        .bind(&city)
        .bind(&timezone)
        .bind(&properties_json)
        .bind(event.duration_ms)
        .bind(&event.entity_type)
        .bind(event.entity_id)
        .bind(created_month)
        .execute(&self.pool)
        .await?;

        Ok(Event {
            event_id,
            event_type: event.event_type,
            user_id: event.user_id,
            session_id,
            tenant_id: event.tenant_id,
            timestamp,
            page_url: event.page_url,
            page_title: event.page_title,
            referrer: event.referrer,
            platform,
            device_info: event.device_info,
            geo_info: event.geo_info,
            properties: event.properties.unwrap_or_default(),
            duration_ms: event.duration_ms,
            entity_type: event.entity_type,
            entity_id: event.entity_id,
        })
    }

    /// Inserts multiple events in batch.
    pub async fn insert_events_batch(&self, events: Vec<NewEvent>) -> Result<Vec<Event>> {
        let mut results = Vec::with_capacity(events.len());

        // Use a transaction for batch inserts
        let mut tx = self.pool.begin().await?;

        for event in events {
            let event_id = Uuid::new_v4();
            let session_id = event.session_id.unwrap_or_else(Uuid::new_v4);
            let platform = event.platform.clone().unwrap_or(Platform::Unknown);
            let timestamp = Utc::now();
            let created_month = timestamp.date_naive().with_day(1).unwrap();

            let (event_type_str, custom_event_name) = event_type_to_db(&event.event_type);
            let platform_str = platform_to_db(&platform);
            let (user_agent, browser, browser_version, os, os_version, device_type, screen_width, screen_height) =
                extract_device_info(&event.device_info);
            let (ip_address, country, country_code, region, city, timezone) =
                extract_geo_info(&event.geo_info);

            let properties_json = serde_json::to_value(&event.properties.clone().unwrap_or_default())
                .unwrap_or(serde_json::json!({}));

            sqlx::query(r#"
                INSERT INTO analytics.events (
                    event_id, event_type, custom_event_name, tenant_id, user_id, session_id,
                    timestamp, page_url, page_title, referrer, platform,
                    user_agent, browser, browser_version, os, os_version, device_type,
                    screen_width, screen_height, ip_address, country, country_code,
                    region, city, timezone, properties, duration_ms, entity_type, entity_id,
                    created_month
                ) VALUES (
                    $1, $2::analytics.event_type, $3, $4, $5, $6,
                    $7, $8, $9, $10, $11::analytics.platform,
                    $12, $13, $14, $15, $16, $17,
                    $18, $19, $20::inet, $21, $22,
                    $23, $24, $25, $26, $27, $28, $29,
                    $30
                )
            "#)
            .bind(event_id)
            .bind(event_type_str)
            .bind(&custom_event_name)
            .bind(event.tenant_id)
            .bind(event.user_id)
            .bind(session_id)
            .bind(timestamp)
            .bind(&event.page_url)
            .bind(&event.page_title)
            .bind(&event.referrer)
            .bind(platform_str)
            .bind(&user_agent)
            .bind(&browser)
            .bind(&browser_version)
            .bind(&os)
            .bind(&os_version)
            .bind(&device_type)
            .bind(screen_width)
            .bind(screen_height)
            .bind(&ip_address)
            .bind(&country)
            .bind(&country_code)
            .bind(&region)
            .bind(&city)
            .bind(&timezone)
            .bind(&properties_json)
            .bind(event.duration_ms)
            .bind(&event.entity_type)
            .bind(event.entity_id)
            .bind(created_month)
            .execute(&mut *tx)
            .await?;

            results.push(Event {
                event_id,
                event_type: event.event_type,
                user_id: event.user_id,
                session_id,
                tenant_id: event.tenant_id,
                timestamp,
                page_url: event.page_url,
                page_title: event.page_title,
                referrer: event.referrer,
                platform,
                device_info: event.device_info,
                geo_info: event.geo_info,
                properties: event.properties.unwrap_or_default(),
                duration_ms: event.duration_ms,
                entity_type: event.entity_type,
                entity_id: event.entity_id,
            });
        }

        tx.commit().await?;
        Ok(results)
    }

    /// Gets an event by ID.
    pub async fn get_event(&self, event_id: Uuid) -> Result<Event> {
        let maybe_row: Option<PgRow> = sqlx::query(r#"
            SELECT
                event_id, event_type::text, custom_event_name, tenant_id, user_id, session_id,
                timestamp, page_url, page_title, referrer, platform::text,
                user_agent, browser, browser_version, os, os_version, device_type,
                screen_width, screen_height, ip_address::text, country, country_code,
                region, city, timezone, properties, duration_ms, entity_type, entity_id
            FROM analytics.events
            WHERE event_id = $1
        "#)
        .bind(event_id)
        .fetch_optional(&self.pool)
        .await?;

        let row = maybe_row.ok_or_else(|| RepositoryError::NotFound(format!("Event {}", event_id)))?;
        Ok(row_to_event(&row))
    }

    /// Queries events with filters.
    pub async fn query_events(&self, query: &AnalyticsQuery) -> Result<Vec<Event>> {
        let mut sql = String::from(r#"
            SELECT
                event_id, event_type::text, custom_event_name, tenant_id, user_id, session_id,
                timestamp, page_url, page_title, referrer, platform::text,
                user_agent, browser, browser_version, os, os_version, device_type,
                screen_width, screen_height, ip_address::text, country, country_code,
                region, city, timezone, properties, duration_ms, entity_type, entity_id
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
        "#);

        let mut param_count = 2;

        if query.user_id.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND user_id = ${}", param_count));
        }

        if query.tenant_id.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND tenant_id = ${}", param_count));
        }

        if let Some(ref types) = query.event_types {
            if !types.is_empty() {
                let type_strs: Vec<String> = types.iter()
                    .map(|t| format!("'{}'", event_type_to_db(t).0))
                    .collect();
                sql.push_str(&format!(" AND event_type::text IN ({})", type_strs.join(",")));
            }
        }

        if query.platform.is_some() {
            param_count += 1;
            sql.push_str(&format!(" AND platform::text = ${}", param_count));
        }

        sql.push_str(" ORDER BY timestamp DESC");

        let limit = query.limit.unwrap_or(100);
        let offset = query.offset.unwrap_or(0);
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        let mut db_query = sqlx::query(&sql)
            .bind(query.date_from)
            .bind(query.date_to);

        if let Some(user_id) = query.user_id {
            db_query = db_query.bind(user_id);
        }
        if let Some(tenant_id) = query.tenant_id {
            db_query = db_query.bind(tenant_id);
        }
        if let Some(ref platform) = query.platform {
            db_query = db_query.bind(platform_to_db(platform));
        }

        let rows: Vec<PgRow> = db_query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| row_to_event(r)).collect())
    }

    /// Counts events by type within a date range.
    pub async fn count_events_by_type(&self, range: &DateRange) -> Result<Vec<EventCount>> {
        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                COALESCE(custom_event_name, event_type::text) as event_type,
                COUNT(*) as count
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
            GROUP BY COALESCE(custom_event_name, event_type::text)
            ORDER BY count DESC
        "#)
        .bind(range.start)
        .bind(range.end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row: &PgRow| {
            EventCount {
                event_type: row.get("event_type"),
                count: row.get("count"),
            }
        }).collect())
    }

    /// Gets time series data for events.
    pub async fn get_event_time_series(
        &self,
        range: &DateRange,
        granularity: TimeGranularity,
        event_type: Option<EventType>,
    ) -> Result<Vec<TimeSeriesPoint>> {
        let truncate_fn = match granularity {
            TimeGranularity::Minute => "minute",
            TimeGranularity::Hour => "hour",
            TimeGranularity::Day => "day",
            TimeGranularity::Week => "week",
            TimeGranularity::Month => "month",
            TimeGranularity::Year => "year",
        };

        let type_filter = if let Some(ref et) = event_type {
            let (ts, _) = event_type_to_db(et);
            format!(" AND event_type::text = '{}'", ts)
        } else {
            String::new()
        };

        let type_label = event_type.as_ref().map(|et| event_type_to_db(et).0.to_string());

        let sql = format!(r#"
            SELECT
                DATE_TRUNC('{}', timestamp) as bucket,
                COUNT(*)::float8 as value
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2 {}
            GROUP BY bucket
            ORDER BY bucket ASC
        "#, truncate_fn, type_filter);

        let rows: Vec<PgRow> = sqlx::query(&sql)
            .bind(range.start)
            .bind(range.end)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.iter().map(|row: &PgRow| {
            TimeSeriesPoint {
                timestamp: row.get("bucket"),
                value: row.get("value"),
                label: type_label.clone(),
            }
        }).collect())
    }

    // =========================================================================
    // SESSION OPERATIONS
    // =========================================================================

    /// Creates a new session.
    pub async fn create_session(&self, session: NewSession) -> Result<Session> {
        let session_id = Uuid::new_v4();
        let platform = session.platform.unwrap_or(Platform::Unknown);
        let platform_str = platform_to_db(&platform);
        let now = Utc::now();

        let (user_agent, browser, browser_version, os, os_version, device_type, screen_width, screen_height) =
            extract_device_info(&session.device_info);
        let (ip_address, country, country_code, region, city, timezone) =
            extract_geo_info(&session.geo_info);

        sqlx::query(r#"
            INSERT INTO analytics.sessions (
                session_id, tenant_id, user_id, started_at, last_activity_at, is_active,
                entry_page, platform, user_agent, browser, browser_version,
                os, os_version, device_type, screen_width, screen_height,
                ip_address, country, country_code, region, city, timezone
            ) VALUES (
                $1, $2, $3, $4, $4, TRUE,
                $5, $6::analytics.platform, $7, $8, $9,
                $10, $11, $12, $13, $14,
                $15::inet, $16, $17, $18, $19, $20
            )
        "#)
        .bind(session_id)
        .bind(session.tenant_id)
        .bind(session.user_id)
        .bind(now)
        .bind(&session.entry_page)
        .bind(platform_str)
        .bind(&user_agent)
        .bind(&browser)
        .bind(&browser_version)
        .bind(&os)
        .bind(&os_version)
        .bind(&device_type)
        .bind(screen_width)
        .bind(screen_height)
        .bind(&ip_address)
        .bind(&country)
        .bind(&country_code)
        .bind(&region)
        .bind(&city)
        .bind(&timezone)
        .execute(&self.pool)
        .await?;

        Ok(Session {
            session_id,
            user_id: session.user_id,
            tenant_id: session.tenant_id,
            started_at: now,
            ended_at: None,
            duration_seconds: None,
            platform,
            device_info: session.device_info,
            geo_info: session.geo_info,
            entry_page: session.entry_page,
            exit_page: None,
            page_views: 0,
            events_count: 0,
            is_active: true,
        })
    }

    /// Gets a session by ID.
    pub async fn get_session(&self, session_id: Uuid) -> Result<Session> {
        let maybe_row: Option<PgRow> = sqlx::query(r#"
            SELECT
                session_id, tenant_id, user_id, started_at, ended_at, duration_seconds,
                last_activity_at, is_active, entry_page, exit_page, page_views, events_count,
                platform::text, user_agent, browser, browser_version, os, os_version,
                device_type, screen_width, screen_height, ip_address::text, country,
                country_code, region, city, timezone
            FROM analytics.sessions
            WHERE session_id = $1
        "#)
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;

        let row = maybe_row.ok_or_else(|| RepositoryError::NotFound(format!("Session {}", session_id)))?;
        Ok(row_to_session(&row))
    }

    /// Updates session with new event (handled by trigger, but can be called manually).
    pub async fn update_session_event(&self, session_id: Uuid, page_url: Option<String>) -> Result<Session> {
        sqlx::query(r#"
            UPDATE analytics.sessions
            SET
                events_count = events_count + 1,
                page_views = CASE WHEN $2 IS NOT NULL THEN page_views + 1 ELSE page_views END,
                exit_page = COALESCE($2, exit_page),
                last_activity_at = NOW()
            WHERE session_id = $1
        "#)
        .bind(session_id)
        .bind(&page_url)
        .execute(&self.pool)
        .await?;

        self.get_session(session_id).await
    }

    /// Ends a session.
    pub async fn end_session(&self, session_id: Uuid) -> Result<Session> {
        sqlx::query(r#"
            UPDATE analytics.sessions
            SET
                ended_at = NOW(),
                duration_seconds = EXTRACT(EPOCH FROM (NOW() - started_at))::bigint,
                is_active = FALSE
            WHERE session_id = $1
        "#)
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        self.get_session(session_id).await
    }

    /// Gets active sessions count.
    pub async fn count_active_sessions(&self) -> Result<i64> {
        let row: PgRow = sqlx::query("SELECT COUNT(*) as count FROM analytics.sessions WHERE is_active = TRUE")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get("count"))
    }

    // =========================================================================
    // ANALYTICS QUERIES
    // =========================================================================

    /// Gets platform statistics.
    pub async fn get_platform_stats(&self, range: &DateRange) -> Result<PlatformStats> {
        // Get user metrics
        let user_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(DISTINCT user_id) as total_users,
                COUNT(DISTINCT user_id) FILTER (WHERE user_id IS NOT NULL) as active_users
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
        "#)
        .bind(range.start)
        .bind(range.end)
        .fetch_one(&self.pool)
        .await?;

        // Get session metrics
        let session_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) as total_sessions,
                COALESCE(AVG(duration_seconds), 0)::float8 as avg_duration,
                COALESCE(
                    COUNT(*) FILTER (WHERE page_views <= 1)::float8 / NULLIF(COUNT(*)::float8, 0) * 100,
                    0
                ) as bounce_rate
            FROM analytics.sessions
            WHERE started_at >= $1 AND started_at <= $2
        "#)
        .bind(range.start)
        .bind(range.end)
        .fetch_one(&self.pool)
        .await?;

        // Get page views
        let pv_row: PgRow = sqlx::query(r#"
            SELECT COUNT(*) as page_views
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2 AND event_type = 'page_view'
        "#)
        .bind(range.start)
        .bind(range.end)
        .fetch_one(&self.pool)
        .await?;

        // Get platform breakdown
        let platform_rows: Vec<PgRow> = sqlx::query(r#"
            SELECT platform::text, COUNT(*) as count
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
            GROUP BY platform
        "#)
        .bind(range.start)
        .bind(range.end)
        .fetch_all(&self.pool)
        .await?;

        let mut platform_breakdown: HashMap<String, i64> = HashMap::new();
        for row in platform_rows.iter() {
            let platform: String = row.get("platform");
            let count: i64 = row.get("count");
            platform_breakdown.insert(platform, count);
        }

        Ok(PlatformStats {
            period_start: range.start,
            period_end: range.end,
            total_users: user_row.get("total_users"),
            active_users: user_row.get("active_users"),
            new_users: 0, // Would need user creation tracking
            total_sessions: session_row.get("total_sessions"),
            total_page_views: pv_row.get("page_views"),
            average_session_duration_seconds: session_row.get("avg_duration"),
            bounce_rate: session_row.get("bounce_rate"),
            top_pages: vec![],
            top_courses: vec![],
            platform_breakdown,
        })
    }

    /// Gets course analytics.
    pub async fn get_course_analytics(&self, course_id: Uuid, range: &DateRange) -> Result<CourseAnalytics> {
        let row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) FILTER (WHERE event_type = 'course_enroll') as enrollments,
                COUNT(*) FILTER (WHERE event_type = 'course_complete') as completions,
                COUNT(DISTINCT user_id) as active_students,
                COALESCE(SUM(duration_ms) / 60000, 0)::bigint as time_spent_minutes
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
              AND entity_id = $3
              AND entity_type = 'course'
        "#)
        .bind(range.start)
        .bind(range.end)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        let enrollments: i64 = row.get("enrollments");
        let completions: i64 = row.get("completions");
        let completion_rate = if enrollments > 0 {
            (completions as f64 / enrollments as f64) * 100.0
        } else {
            0.0
        };

        Ok(CourseAnalytics {
            course_id,
            total_enrollments: enrollments,
            active_students: row.get("active_students"),
            completion_rate,
            average_progress: 0.0, // Would need progress tracking
            average_score: None,
            total_time_spent_minutes: row.get("time_spent_minutes"),
            lesson_completion_rates: HashMap::new(),
        })
    }

    /// Gets user engagement metrics.
    pub async fn get_user_engagement(&self, user_id: Uuid, range: &DateRange) -> Result<UserEngagement> {
        // Event counts
        let event_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) FILTER (WHERE event_type = 'course_enroll') as courses_enrolled,
                COUNT(*) FILTER (WHERE event_type = 'course_complete') as courses_completed,
                COUNT(*) FILTER (WHERE event_type = 'lesson_complete') as lessons_completed,
                COUNT(*) FILTER (WHERE event_type = 'quiz_complete') as quizzes_completed,
                MAX(timestamp) as last_active
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2 AND user_id = $3
        "#)
        .bind(range.start)
        .bind(range.end)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Session stats
        let session_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) as total_sessions,
                COALESCE(SUM(duration_seconds) / 60, 0)::bigint as total_time_minutes
            FROM analytics.sessions
            WHERE started_at >= $1 AND started_at <= $2 AND user_id = $3
        "#)
        .bind(range.start)
        .bind(range.end)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserEngagement {
            user_id,
            total_sessions: session_row.get("total_sessions"),
            total_time_spent_minutes: session_row.get("total_time_minutes"),
            courses_enrolled: event_row.get("courses_enrolled"),
            courses_completed: event_row.get("courses_completed"),
            lessons_completed: event_row.get("lessons_completed"),
            quizzes_completed: event_row.get("quizzes_completed"),
            average_quiz_score: None,
            last_active_at: event_row.get("last_active"),
            streak_days: 0, // Would need daily tracking
        })
    }

    /// Gets top pages by views.
    pub async fn get_top_pages(&self, range: &DateRange, limit: i64) -> Result<Vec<PageStats>> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(r#"
            SELECT
                page_url,
                MAX(page_title) as page_title,
                COUNT(*) as views,
                COUNT(DISTINCT user_id) as unique_visitors,
                COALESCE(AVG(duration_ms) / 1000.0, 0)::float8 as avg_time
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
              AND event_type = 'page_view'
              AND page_url IS NOT NULL
            GROUP BY page_url
            ORDER BY views DESC
            LIMIT $3
        "#)
        .bind(range.start)
        .bind(range.end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row: &sqlx::postgres::PgRow| PageStats {
            page_url: row.get("page_url"),
            page_title: row.get("page_title"),
            views: row.get("views"),
            unique_visitors: row.get("unique_visitors"),
            average_time_seconds: row.get("avg_time"),
        }).collect())
    }

    /// Gets top courses by enrollments.
    pub async fn get_top_courses(&self, range: &DateRange, limit: i64) -> Result<Vec<CourseStats>> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(r#"
            SELECT
                entity_id as course_id,
                COUNT(*) FILTER (WHERE event_type = 'course_enroll') as enrollments,
                COUNT(*) FILTER (WHERE event_type = 'course_complete') as completions,
                COUNT(DISTINCT user_id) as active_students
            FROM analytics.events
            WHERE timestamp >= $1 AND timestamp <= $2
              AND entity_type = 'course'
              AND entity_id IS NOT NULL
            GROUP BY entity_id
            ORDER BY enrollments DESC
            LIMIT $3
        "#)
        .bind(range.start)
        .bind(range.end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row: &sqlx::postgres::PgRow| {
            let course_id: Uuid = row.get("course_id");
            CourseStats {
                course_id,
                course_name: format!("Course {}", course_id),
                enrollments: row.get("enrollments"),
                completions: row.get("completions"),
                active_students: row.get("active_students"),
            }
        }).collect())
    }

    // =========================================================================
    // METRICS OPERATIONS
    // =========================================================================

    /// Stores an aggregated metric.
    pub async fn store_metric(&self, metric: Metric) -> Result<()> {
        let dimensions_json = serde_json::to_value(&metric.dimensions)
            .unwrap_or(serde_json::json!({}));

        sqlx::query(r#"
            INSERT INTO analytics.metrics (
                metric_id, name, tenant_id, period_start, period_end,
                granularity, value, count, dimensions
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (name, tenant_id, period_start, granularity, dimensions)
            DO UPDATE SET value = EXCLUDED.value, count = EXCLUDED.count
        "#)
        .bind(metric.metric_id)
        .bind(&metric.name)
        .bind(metric.tenant_id)
        .bind(metric.period_start)
        .bind(metric.period_end)
        .bind(&metric.granularity)
        .bind(metric.value)
        .bind(metric.count)
        .bind(&dimensions_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Gets metrics by name and period.
    pub async fn get_metrics(
        &self,
        name: &str,
        range: &DateRange,
        pagination: &Pagination,
    ) -> Result<Vec<Metric>> {
        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                metric_id, name, tenant_id, period_start, period_end,
                granularity, value, count, dimensions
            FROM analytics.metrics
            WHERE name = $1 AND period_start >= $2 AND period_start <= $3
            ORDER BY period_start DESC
            LIMIT $4 OFFSET $5
        "#)
        .bind(name)
        .bind(range.start)
        .bind(range.end)
        .bind(pagination.limit)
        .bind(pagination.offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row: &PgRow| {
            let dimensions: serde_json::Value = row.get("dimensions");
            Metric {
                metric_id: row.get("metric_id"),
                name: row.get("name"),
                tenant_id: row.get("tenant_id"),
                period_start: row.get("period_start"),
                period_end: row.get("period_end"),
                granularity: row.get("granularity"),
                value: row.get("value"),
                count: row.get("count"),
                dimensions: serde_json::from_value(dimensions).unwrap_or_default(),
            }
        }).collect())
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Converts EventType to database enum string.
fn event_type_to_db(event_type: &EventType) -> (&'static str, Option<String>) {
    match event_type {
        EventType::PageView => ("page_view", None),
        EventType::Click => ("click", None),
        EventType::FormSubmit => ("form_submit", None),
        EventType::CourseEnroll => ("course_enroll", None),
        EventType::CourseComplete => ("course_complete", None),
        EventType::LessonStart => ("lesson_start", None),
        EventType::LessonComplete => ("lesson_complete", None),
        EventType::QuizStart => ("quiz_start", None),
        EventType::QuizComplete => ("quiz_complete", None),
        EventType::AssignmentSubmit => ("assignment_submit", None),
        EventType::VideoPlay => ("video_play", None),
        EventType::VideoPause => ("video_pause", None),
        EventType::VideoComplete => ("video_complete", None),
        EventType::Search => ("search", None),
        EventType::Download => ("download", None),
        EventType::Login => ("login", None),
        EventType::Logout => ("logout", None),
        EventType::Error => ("error", None),
        EventType::Custom(name) => ("custom", Some(name.clone())),
    }
}

/// Converts Platform to database enum string.
fn platform_to_db(platform: &Platform) -> &'static str {
    match platform {
        Platform::Web => "web",
        Platform::Android => "android",
        Platform::iOS => "ios",
        Platform::Desktop => "desktop",
        Platform::API => "api",
        Platform::Unknown => "unknown",
    }
}

/// Converts database platform string to Platform.
fn db_to_platform(s: &str) -> Platform {
    match s {
        "web" => Platform::Web,
        "android" => Platform::Android,
        "ios" => Platform::iOS,
        "desktop" => Platform::Desktop,
        "api" => Platform::API,
        _ => Platform::Unknown,
    }
}

/// Converts database event type string to EventType.
fn db_to_event_type(s: &str, custom_name: Option<String>) -> EventType {
    match s {
        "page_view" => EventType::PageView,
        "click" => EventType::Click,
        "form_submit" => EventType::FormSubmit,
        "course_enroll" => EventType::CourseEnroll,
        "course_complete" => EventType::CourseComplete,
        "lesson_start" => EventType::LessonStart,
        "lesson_complete" => EventType::LessonComplete,
        "quiz_start" => EventType::QuizStart,
        "quiz_complete" => EventType::QuizComplete,
        "assignment_submit" => EventType::AssignmentSubmit,
        "video_play" => EventType::VideoPlay,
        "video_pause" => EventType::VideoPause,
        "video_complete" => EventType::VideoComplete,
        "search" => EventType::Search,
        "download" => EventType::Download,
        "login" => EventType::Login,
        "logout" => EventType::Logout,
        "error" => EventType::Error,
        "custom" => EventType::Custom(custom_name.unwrap_or_else(|| "custom".to_string())),
        other => EventType::Custom(other.to_string()),
    }
}

/// Extracts device info fields.
fn extract_device_info(device_info: &Option<DeviceInfo>) -> (
    Option<String>, Option<String>, Option<String>, Option<String>,
    Option<String>, Option<String>, Option<i32>, Option<i32>
) {
    match device_info {
        Some(di) => (
            di.user_agent.clone(),
            di.browser.clone(),
            di.browser_version.clone(),
            di.os.clone(),
            di.os_version.clone(),
            di.device_type.clone(),
            di.screen_width,
            di.screen_height,
        ),
        None => (None, None, None, None, None, None, None, None),
    }
}

/// Extracts geo info fields.
fn extract_geo_info(geo_info: &Option<GeoInfo>) -> (
    Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>
) {
    match geo_info {
        Some(gi) => (
            gi.ip_address.clone(),
            gi.country.clone(),
            gi.country_code.clone(),
            gi.region.clone(),
            gi.city.clone(),
            gi.timezone.clone(),
        ),
        None => (None, None, None, None, None, None),
    }
}

/// Converts a database row to an Event.
fn row_to_event(row: &sqlx::postgres::PgRow) -> Event {
    let event_type_str: String = row.get("event_type");
    let custom_event_name: Option<String> = row.get("custom_event_name");
    let platform_str: String = row.get("platform");
    let properties_json: serde_json::Value = row.get("properties");

    Event {
        event_id: row.get("event_id"),
        event_type: db_to_event_type(&event_type_str, custom_event_name),
        user_id: row.get("user_id"),
        session_id: row.get("session_id"),
        tenant_id: row.get("tenant_id"),
        timestamp: row.get("timestamp"),
        page_url: row.get("page_url"),
        page_title: row.get("page_title"),
        referrer: row.get("referrer"),
        platform: db_to_platform(&platform_str),
        device_info: Some(DeviceInfo {
            user_agent: row.get("user_agent"),
            browser: row.get("browser"),
            browser_version: row.get("browser_version"),
            os: row.get("os"),
            os_version: row.get("os_version"),
            device_type: row.get("device_type"),
            screen_width: row.get("screen_width"),
            screen_height: row.get("screen_height"),
        }),
        geo_info: Some(GeoInfo {
            ip_address: row.get("ip_address"),
            country: row.get("country"),
            country_code: row.get("country_code"),
            region: row.get("region"),
            city: row.get("city"),
            timezone: row.get("timezone"),
        }),
        properties: serde_json::from_value(properties_json).unwrap_or_default(),
        duration_ms: row.get("duration_ms"),
        entity_type: row.get("entity_type"),
        entity_id: row.get("entity_id"),
    }
}

/// Converts a database row to a Session.
fn row_to_session(row: &sqlx::postgres::PgRow) -> Session {
    let platform_str: String = row.get("platform");

    Session {
        session_id: row.get("session_id"),
        user_id: row.get("user_id"),
        tenant_id: row.get("tenant_id"),
        started_at: row.get("started_at"),
        ended_at: row.get("ended_at"),
        duration_seconds: row.get("duration_seconds"),
        platform: db_to_platform(&platform_str),
        device_info: Some(DeviceInfo {
            user_agent: row.get("user_agent"),
            browser: row.get("browser"),
            browser_version: row.get("browser_version"),
            os: row.get("os"),
            os_version: row.get("os_version"),
            device_type: row.get("device_type"),
            screen_width: row.get("screen_width"),
            screen_height: row.get("screen_height"),
        }),
        geo_info: Some(GeoInfo {
            ip_address: row.get("ip_address"),
            country: row.get("country"),
            country_code: row.get("country_code"),
            region: row.get("region"),
            city: row.get("city"),
            timezone: row.get("timezone"),
        }),
        entry_page: row.get("entry_page"),
        exit_page: row.get("exit_page"),
        page_views: row.get("page_views"),
        events_count: row.get("events_count"),
        is_active: row.get("is_active"),
    }
}
