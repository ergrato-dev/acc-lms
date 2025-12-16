//! # Course Domain Events
//!
//! Events emitted by the courses service for cross-service communication.
//!
//! ## Event Flow
//!
//! ```text
//! courses-service → Event Bus → Consumers
//!                              ├── search-service (indexing)
//!                              ├── analytics-service (tracking)
//!                              └── notifications-service (alerts)
//! ```
//!
//! ## Event Categories
//!
//! - **Course Events**: Lifecycle events for courses
//! - **Lesson Events**: Content change events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::{CourseStatus, DifficultyLevel, LessonType};

// =============================================================================
// COURSE EVENTS
// =============================================================================

/// Events related to course lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CourseEvent {
    /// A new course was created
    Created(CourseCreatedEvent),
    /// Course metadata was updated
    Updated(CourseUpdatedEvent),
    /// Course was published
    Published(CoursePublishedEvent),
    /// Course was unpublished
    Unpublished(CourseUnpublishedEvent),
    /// Course was deleted (soft)
    Deleted(CourseDeletedEvent),
}

/// Event: New course created.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseCreatedEvent {
    /// Event ID
    pub event_id: Uuid,
    /// Course ID
    pub course_id: Uuid,
    /// Instructor ID
    pub instructor_id: Uuid,
    /// Course title
    pub title: String,
    /// Course slug
    pub slug: String,
    /// Price in cents
    pub price_cents: i32,
    /// Currency
    pub currency: String,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
}

impl CourseCreatedEvent {
    pub fn new(
        course_id: Uuid,
        instructor_id: Uuid,
        title: String,
        slug: String,
        price_cents: i32,
        currency: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            instructor_id,
            title,
            slug,
            price_cents,
            currency,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Course updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseUpdatedEvent {
    pub event_id: Uuid,
    pub course_id: Uuid,
    /// Fields that were changed
    pub changed_fields: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl CourseUpdatedEvent {
    pub fn new(course_id: Uuid, changed_fields: Vec<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            changed_fields,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Course published.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoursePublishedEvent {
    pub event_id: Uuid,
    pub course_id: Uuid,
    pub instructor_id: Uuid,
    pub title: String,
    pub slug: String,
    pub price_cents: i32,
    pub currency: String,
    pub category_id: Option<Uuid>,
    pub difficulty_level: DifficultyLevel,
    /// For search indexing
    pub short_description: String,
    pub timestamp: DateTime<Utc>,
}

impl CoursePublishedEvent {
    pub fn new(
        course_id: Uuid,
        instructor_id: Uuid,
        title: String,
        slug: String,
        price_cents: i32,
        currency: String,
        category_id: Option<Uuid>,
        difficulty_level: DifficultyLevel,
        short_description: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            instructor_id,
            title,
            slug,
            price_cents,
            currency,
            category_id,
            difficulty_level,
            short_description,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Course unpublished.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseUnpublishedEvent {
    pub event_id: Uuid,
    pub course_id: Uuid,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl CourseUnpublishedEvent {
    pub fn new(course_id: Uuid, reason: Option<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            reason,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Course deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDeletedEvent {
    pub event_id: Uuid,
    pub course_id: Uuid,
    pub deleted_by: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl CourseDeletedEvent {
    pub fn new(course_id: Uuid, deleted_by: Uuid) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            deleted_by,
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// LESSON EVENTS
// =============================================================================

/// Events related to lesson content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LessonEvent {
    /// Lesson created
    Created(LessonCreatedEvent),
    /// Lesson updated
    Updated(LessonUpdatedEvent),
    /// Lesson deleted
    Deleted(LessonDeletedEvent),
    /// Lessons reordered
    Reordered(LessonsReorderedEvent),
}

/// Event: Lesson created.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonCreatedEvent {
    pub event_id: Uuid,
    pub lesson_id: Uuid,
    pub course_id: Uuid,
    pub section_id: Uuid,
    pub title: String,
    pub content_type: LessonType,
    pub duration_seconds: i32,
    pub timestamp: DateTime<Utc>,
}

impl LessonCreatedEvent {
    pub fn new(
        lesson_id: Uuid,
        course_id: Uuid,
        section_id: Uuid,
        title: String,
        content_type: LessonType,
        duration_seconds: i32,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            lesson_id,
            course_id,
            section_id,
            title,
            content_type,
            duration_seconds,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Lesson updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonUpdatedEvent {
    pub event_id: Uuid,
    pub lesson_id: Uuid,
    pub course_id: Uuid,
    pub changed_fields: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl LessonUpdatedEvent {
    pub fn new(lesson_id: Uuid, course_id: Uuid, changed_fields: Vec<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            lesson_id,
            course_id,
            changed_fields,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Lesson deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonDeletedEvent {
    pub event_id: Uuid,
    pub lesson_id: Uuid,
    pub course_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl LessonDeletedEvent {
    pub fn new(lesson_id: Uuid, course_id: Uuid) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            lesson_id,
            course_id,
            timestamp: Utc::now(),
        }
    }
}

/// Event: Lessons reordered within a section.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonsReorderedEvent {
    pub event_id: Uuid,
    pub course_id: Uuid,
    pub section_id: Uuid,
    /// New order: lesson_id -> sort_order
    pub new_order: Vec<(Uuid, i32)>,
    pub timestamp: DateTime<Utc>,
}

impl LessonsReorderedEvent {
    pub fn new(course_id: Uuid, section_id: Uuid, new_order: Vec<(Uuid, i32)>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            course_id,
            section_id,
            new_order,
            timestamp: Utc::now(),
        }
    }
}
