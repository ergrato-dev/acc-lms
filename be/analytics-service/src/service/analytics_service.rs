//! # Analytics Service
//!
//! Business logic for analytics operations including event tracking,
//! session management, metrics aggregation, and reporting.

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::{
    AnalyticsEvent, AnalyticsQuery, CourseAnalytics, CourseStats, Event, EventCount,
    EventType, Metric, NewEvent, NewSession, PageStats, Platform, PlatformStats,
    Session, TimeSeriesPoint, UserEngagement,
};
use crate::domain::value_objects::{DateRange, Pagination, TimeGranularity};
use crate::repository::{AnalyticsRepository, RepositoryError};

// =============================================================================
// SERVICE ERRORS
// =============================================================================

/// Analytics service errors.
#[derive(Debug, thiserror::Error)]
pub enum AnalyticsError {
    #[error("Event not found: {0}")]
    EventNotFound(Uuid),

    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
}

pub type Result<T> = std::result::Result<T, AnalyticsError>;

// =============================================================================
// ANALYTICS SERVICE
// =============================================================================

/// Service for analytics operations.
#[derive(Clone)]
pub struct AnalyticsService {
    repository: AnalyticsRepository,
}

impl AnalyticsService {
    /// Creates a new service instance.
    pub fn new(repository: AnalyticsRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // EVENT TRACKING
    // =========================================================================

    /// Tracks a single event.
    pub async fn track_event(&self, event: NewEvent) -> Result<Event> {
        // Validate event
        self.validate_event(&event)?;

        // Insert event
        let created = self.repository.insert_event(event).await?;

        Ok(created)
    }

    /// Tracks multiple events in batch.
    pub async fn track_events_batch(&self, events: Vec<NewEvent>) -> Result<Vec<Event>> {
        if events.is_empty() {
            return Err(AnalyticsError::InvalidQuery("Empty event batch".to_string()));
        }

        // Validate all events
        for event in &events {
            self.validate_event(event)?;
        }

        // Insert events
        let created = self.repository.insert_events_batch(events).await?;

        Ok(created)
    }

    /// Gets an event by ID.
    pub async fn get_event(&self, event_id: Uuid) -> Result<Option<Event>> {
        match self.repository.get_event(event_id).await {
            Ok(event) => Ok(Some(event)),
            Err(RepositoryError::NotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Queries events with filters.
    pub async fn query_events(
        &self,
        date_range: &DateRange,
        event_types: Option<&[EventType]>,
        user_id: Option<Uuid>,
        tenant_id: Option<Uuid>,
        pagination: &Pagination,
    ) -> Result<Vec<Event>> {
        let query = AnalyticsQuery {
            date_from: date_range.start,
            date_to: date_range.end,
            event_types: event_types.map(|t| t.to_vec()),
            user_id,
            tenant_id,
            course_id: None,
            platform: None,
            group_by: None,
            limit: Some(pagination.limit),
            offset: Some(pagination.offset),
        };

        self.validate_query(&query)?;
        Ok(self.repository.query_events(&query).await?)
    }

    /// Validates an event before tracking.
    fn validate_event(&self, event: &NewEvent) -> Result<()> {
        // Basic validation
        if let EventType::Custom(ref name) = event.event_type {
            if name.is_empty() || name.len() > 100 {
                return Err(AnalyticsError::InvalidQuery(
                    "Custom event name must be 1-100 characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validates a query.
    fn validate_query(&self, query: &AnalyticsQuery) -> Result<()> {
        if query.date_from >= query.date_to {
            return Err(AnalyticsError::InvalidDateRange(
                "Start date must be before end date".to_string(),
            ));
        }

        let duration = query.date_to - query.date_from;
        if duration.num_days() > 365 {
            return Err(AnalyticsError::InvalidDateRange(
                "Date range cannot exceed 365 days".to_string(),
            ));
        }

        Ok(())
    }

    // =========================================================================
    // SESSION MANAGEMENT
    // =========================================================================

    /// Starts a new session.
    pub async fn start_session(&self, session: NewSession) -> Result<Session> {
        let created = self.repository.create_session(session).await?;
        Ok(created)
    }

    /// Gets a session by ID.
    pub async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        match self.repository.get_session(session_id).await {
            Ok(session) => Ok(Some(session)),
            Err(RepositoryError::NotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Ends a session.
    pub async fn end_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        match self.repository.end_session(session_id).await {
            Ok(session) => Ok(Some(session)),
            Err(RepositoryError::NotFound(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Gets active sessions count.
    pub async fn get_active_sessions_count(&self) -> Result<i64> {
        Ok(self.repository.count_active_sessions().await?)
    }

    // =========================================================================
    // ANALYTICS QUERIES
    // =========================================================================

    /// Gets event counts by type.
    pub async fn get_event_counts(&self, range: &DateRange) -> Result<Vec<EventCount>> {
        Ok(self.repository.count_events_by_type(range).await?)
    }

    /// Gets time series data for events.
    pub async fn get_event_time_series(
        &self,
        range: &DateRange,
        granularity: TimeGranularity,
        event_type: Option<&EventType>,
    ) -> Result<Vec<TimeSeriesPoint>> {
        Ok(self
            .repository
            .get_event_time_series(range, granularity, event_type.cloned())
            .await?)
    }

    /// Gets platform statistics.
    pub async fn get_platform_stats(&self, range: &DateRange) -> Result<PlatformStats> {
        Ok(self.repository.get_platform_stats(range).await?)
    }

    /// Gets course analytics.
    pub async fn get_course_analytics(&self, course_id: Uuid, range: &DateRange) -> Result<CourseAnalytics> {
        Ok(self.repository.get_course_analytics(course_id, range).await?)
    }

    /// Gets user engagement metrics.
    pub async fn get_user_engagement(&self, user_id: Uuid, range: &DateRange) -> Result<UserEngagement> {
        Ok(self.repository.get_user_engagement(user_id, range).await?)
    }

    /// Gets top pages.
    pub async fn get_top_pages(&self, range: &DateRange, limit: i64) -> Result<Vec<PageStats>> {
        Ok(self.repository.get_top_pages(range, limit).await?)
    }

    /// Gets top courses.
    pub async fn get_top_courses(&self, range: &DateRange, limit: i64) -> Result<Vec<CourseStats>> {
        Ok(self.repository.get_top_courses(range, limit).await?)
    }

    // =========================================================================
    // METRICS
    // =========================================================================

    /// Stores an aggregated metric.
    pub async fn store_metric(&self, metric: Metric) -> Result<()> {
        self.repository.store_metric(metric).await?;
        Ok(())
    }

    /// Gets metrics by name.
    pub async fn get_metrics(
        &self,
        name: &str,
        range: &DateRange,
        pagination: &Pagination,
    ) -> Result<Vec<Metric>> {
        Ok(self.repository.get_metrics(name, range, pagination).await?)
    }

    // =========================================================================
    // CONVENIENCE METHODS
    // =========================================================================

    /// Tracks a page view event.
    pub async fn track_page_view(
        &self,
        user_id: Option<Uuid>,
        session_id: Option<Uuid>,
        page_url: String,
        page_title: Option<String>,
        referrer: Option<String>,
        platform: Option<Platform>,
    ) -> Result<Event> {
        let event = NewEvent {
            event_type: EventType::PageView,
            user_id,
            session_id,
            tenant_id: None,
            page_url: Some(page_url),
            page_title,
            referrer,
            platform,
            device_info: None,
            geo_info: None,
            properties: None,
            duration_ms: None,
            entity_type: None,
            entity_id: None,
        };

        self.track_event(event).await
    }

    /// Tracks a course enrollment event.
    pub async fn track_course_enrollment(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        session_id: Option<Uuid>,
    ) -> Result<Event> {
        let event = NewEvent {
            event_type: EventType::CourseEnroll,
            user_id: Some(user_id),
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: None,
            duration_ms: None,
            entity_type: Some("course".to_string()),
            entity_id: Some(course_id),
        };

        self.track_event(event).await
    }

    /// Tracks a course completion event.
    pub async fn track_course_completion(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        session_id: Option<Uuid>,
        final_score: Option<f64>,
    ) -> Result<Event> {
        let mut properties = HashMap::new();
        if let Some(score) = final_score {
            properties.insert("final_score".to_string(), serde_json::json!(score));
        }

        let event = NewEvent {
            event_type: EventType::CourseComplete,
            user_id: Some(user_id),
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: if properties.is_empty() { None } else { Some(properties) },
            duration_ms: None,
            entity_type: Some("course".to_string()),
            entity_id: Some(course_id),
        };

        self.track_event(event).await
    }

    /// Tracks a lesson start event.
    pub async fn track_lesson_start(
        &self,
        user_id: Uuid,
        lesson_id: Uuid,
        course_id: Uuid,
        session_id: Option<Uuid>,
    ) -> Result<Event> {
        let mut properties = HashMap::new();
        properties.insert("course_id".to_string(), serde_json::json!(course_id));

        let event = NewEvent {
            event_type: EventType::LessonStart,
            user_id: Some(user_id),
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: Some(properties),
            duration_ms: None,
            entity_type: Some("lesson".to_string()),
            entity_id: Some(lesson_id),
        };

        self.track_event(event).await
    }

    /// Tracks a lesson completion event.
    pub async fn track_lesson_completion(
        &self,
        user_id: Uuid,
        lesson_id: Uuid,
        course_id: Uuid,
        session_id: Option<Uuid>,
        time_spent_ms: Option<i64>,
    ) -> Result<Event> {
        let mut properties = HashMap::new();
        properties.insert("course_id".to_string(), serde_json::json!(course_id));

        let event = NewEvent {
            event_type: EventType::LessonComplete,
            user_id: Some(user_id),
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: Some(properties),
            duration_ms: time_spent_ms,
            entity_type: Some("lesson".to_string()),
            entity_id: Some(lesson_id),
        };

        self.track_event(event).await
    }

    /// Tracks a quiz completion event.
    pub async fn track_quiz_completion(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
        course_id: Uuid,
        session_id: Option<Uuid>,
        score: f64,
        attempt: i32,
    ) -> Result<Event> {
        let mut properties = HashMap::new();
        properties.insert("course_id".to_string(), serde_json::json!(course_id));
        properties.insert("score".to_string(), serde_json::json!(score));
        properties.insert("attempt".to_string(), serde_json::json!(attempt));

        let event = NewEvent {
            event_type: EventType::QuizComplete,
            user_id: Some(user_id),
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: Some(properties),
            duration_ms: None,
            entity_type: Some("quiz".to_string()),
            entity_id: Some(quiz_id),
        };

        self.track_event(event).await
    }

    /// Tracks a search event.
    pub async fn track_search(
        &self,
        user_id: Option<Uuid>,
        session_id: Option<Uuid>,
        query: String,
        results_count: i64,
    ) -> Result<Event> {
        let mut properties = HashMap::new();
        properties.insert("query".to_string(), serde_json::json!(query));
        properties.insert("results_count".to_string(), serde_json::json!(results_count));

        let event = NewEvent {
            event_type: EventType::Search,
            user_id,
            session_id,
            tenant_id: None,
            page_url: None,
            page_title: None,
            referrer: None,
            platform: None,
            device_info: None,
            geo_info: None,
            properties: Some(properties),
            duration_ms: None,
            entity_type: None,
            entity_id: None,
        };

        self.track_event(event).await
    }
}
