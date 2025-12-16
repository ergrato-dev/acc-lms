//! # Course Domain Entities
//!
//! Core domain entities for the courses service. These entities map to the
//! PostgreSQL schema defined in `db/migrations/postgresql/001_initial_schema.sql`.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Category
//!     └── Course (aggregate root)
//!             ├── Section
//!             │       └── Lesson
//!             └── [Metadata: tags, requirements, objectives]
//! ```
//!
//! ## Design Decisions
//!
//! 1. **Course as Aggregate Root**: All modifications go through Course
//! 2. **Soft Deletes**: Courses use `deleted_at` for recovery capability
//! 3. **Computed Fields**: `average_rating`, `total_enrollments` updated via events
//! 4. **JSONB for Lists**: `requirements`, `learning_objectives` stored as JSONB

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Course publication status.
///
/// Controls visibility and editability of courses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "course_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    /// Initial state, only visible to owner/admin
    Draft,
    /// Under review (future feature)
    Review,
    /// Publicly visible and purchasable
    Published,
    /// Temporarily hidden from catalog
    Archived,
}

impl Default for CourseStatus {
    fn default() -> Self {
        CourseStatus::Draft
    }
}

impl std::fmt::Display for CourseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseStatus::Draft => write!(f, "draft"),
            CourseStatus::Review => write!(f, "review"),
            CourseStatus::Published => write!(f, "published"),
            CourseStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Course difficulty level.
///
/// Helps students find appropriate content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    /// No prior knowledge required
    Beginner,
    /// Some foundational knowledge expected
    Intermediate,
    /// Expert-level content
    Advanced,
}

impl Default for DifficultyLevel {
    fn default() -> Self {
        DifficultyLevel::Beginner
    }
}

impl std::fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DifficultyLevel::Beginner => write!(f, "beginner"),
            DifficultyLevel::Intermediate => write!(f, "intermediate"),
            DifficultyLevel::Advanced => write!(f, "advanced"),
        }
    }
}

/// Lesson content type.
///
/// Determines rendering and progress tracking behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LessonType {
    /// Video content (stored in MinIO)
    Video,
    /// Text/markdown article
    Article,
    /// Interactive quiz (links to assignments-service)
    Quiz,
    /// Practical assignment
    Assignment,
    /// Live session (future feature)
    LiveSession,
}

impl Default for LessonType {
    fn default() -> Self {
        LessonType::Video
    }
}

impl std::fmt::Display for LessonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonType::Video => write!(f, "video"),
            LessonType::Article => write!(f, "article"),
            LessonType::Quiz => write!(f, "quiz"),
            LessonType::Assignment => write!(f, "assignment"),
            LessonType::LiveSession => write!(f, "live_session"),
        }
    }
}

// =============================================================================
// CATEGORY
// =============================================================================

/// Course category for organizing the catalog.
///
/// Categories support hierarchical structure via `parent_category_id`.
///
/// # Database Mapping
///
/// Maps to `course_categories` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    /// Unique identifier
    pub category_id: Uuid,
    /// Display name
    pub name: String,
    /// URL-friendly identifier
    pub slug: String,
    /// Optional description
    pub description: Option<String>,
    /// Icon URL for UI
    pub icon_url: Option<String>,
    /// Parent category for hierarchy (null = root)
    pub parent_category_id: Option<Uuid>,
    /// Display order in listings
    pub sort_order: i32,
    /// Visibility flag
    pub is_active: bool,
    /// When created
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// COURSE
// =============================================================================

/// Main course entity - aggregate root for course management.
///
/// Represents a complete course with all metadata needed for:
/// - Catalog display
/// - Pricing and purchasing
/// - Content organization
/// - Analytics
///
/// # Database Mapping
///
/// Maps to `courses` table with JSONB fields for:
/// - `requirements`: Array of prerequisite strings
/// - `learning_objectives`: Array of objective strings
/// - `target_audience`: Array of audience descriptors
///
/// # Example
///
/// ```rust,ignore
/// let course = Course {
///     course_id: Uuid::new_v4(),
///     instructor_id: instructor.user_id,
///     title: "Rust for Beginners".to_string(),
///     slug: "rust-for-beginners".to_string(),
///     price_cents: 4999, // $49.99
///     currency: "USD".to_string(),
///     difficulty_level: DifficultyLevel::Beginner,
///     is_published: false,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    /// Unique identifier (UUID v4)
    pub course_id: Uuid,
    /// ID of the instructor who created this course
    pub instructor_id: Uuid,
    /// Category for catalog organization
    pub category_id: Option<Uuid>,
    /// Course title (max 200 chars)
    pub title: String,
    /// URL-friendly identifier (unique, auto-generated)
    pub slug: String,
    /// Brief description for catalog cards (max 300 chars)
    pub short_description: String,
    /// Full description with markdown support
    pub full_description: Option<String>,
    /// Thumbnail image URL
    pub thumbnail_url: Option<String>,
    /// Preview/trailer video URL
    pub trailer_video_url: Option<String>,
    /// Price in smallest currency unit (cents)
    pub price_cents: i32,
    /// ISO 4217 currency code
    pub currency: String,
    /// Target skill level
    pub difficulty_level: DifficultyLevel,
    /// Estimated hours to complete
    pub estimated_duration_hours: i32,
    /// Content language (ISO 639-1)
    pub language: String,
    /// Publication status
    pub is_published: bool,
    /// When the course was published
    pub published_at: Option<DateTime<Utc>>,
    /// Average student rating (0.00-5.00)
    pub average_rating: f64,
    /// Total number of ratings
    pub total_ratings: i32,
    /// Total enrollment count
    pub total_enrollments: i32,
    /// Prerequisites as JSON array of strings
    pub requirements: serde_json::Value,
    /// What students will learn (JSON array)
    pub learning_objectives: serde_json::Value,
    /// Who this course is for (JSON array)
    pub target_audience: serde_json::Value,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Course {
    /// Returns true if the course is visible to the public.
    pub fn is_visible(&self) -> bool {
        self.is_published && self.deleted_at.is_none()
    }

    /// Returns true if the given user can edit this course.
    pub fn can_edit(&self, user_id: Uuid, is_admin: bool) -> bool {
        is_admin || self.instructor_id == user_id
    }

    /// Returns true if the course is free.
    pub fn is_free(&self) -> bool {
        self.price_cents == 0
    }

    /// Returns the price formatted for display (e.g., "$49.99").
    pub fn formatted_price(&self) -> String {
        if self.is_free() {
            "Free".to_string()
        } else {
            let dollars = self.price_cents as f64 / 100.0;
            match self.currency.as_str() {
                "USD" => format!("${:.2}", dollars),
                "EUR" => format!("€{:.2}", dollars),
                "MXN" => format!("${:.2} MXN", dollars),
                _ => format!("{:.2} {}", dollars, self.currency),
            }
        }
    }

    /// Calculates the total duration of all lessons (placeholder).
    pub fn total_duration_minutes(&self) -> i32 {
        // This would be calculated from actual lessons
        self.estimated_duration_hours * 60
    }
}

/// Data required to create a new course.
#[derive(Debug, Clone, Deserialize)]
pub struct NewCourse {
    pub instructor_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub slug: Option<String>, // Auto-generated if not provided
    pub short_description: String,
    pub full_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub price_cents: i32,
    pub currency: String,
    pub difficulty_level: DifficultyLevel,
    pub estimated_duration_hours: i32,
    pub language: String,
    pub requirements: Vec<String>,
    pub learning_objectives: Vec<String>,
    pub target_audience: Vec<String>,
}

/// Data for updating an existing course.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateCourse {
    pub category_id: Option<Option<Uuid>>,
    pub title: Option<String>,
    pub short_description: Option<String>,
    pub full_description: Option<Option<String>>,
    pub thumbnail_url: Option<Option<String>>,
    pub trailer_video_url: Option<Option<String>>,
    pub price_cents: Option<i32>,
    pub currency: Option<String>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub estimated_duration_hours: Option<i32>,
    pub language: Option<String>,
    pub requirements: Option<Vec<String>>,
    pub learning_objectives: Option<Vec<String>>,
    pub target_audience: Option<Vec<String>>,
}

// =============================================================================
// SECTION
// =============================================================================

/// A logical grouping of lessons within a course.
///
/// Sections help organize course content into chapters or modules.
/// Lessons belong to exactly one section.
///
/// # Database Mapping
///
/// Maps to `course_sections` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Section {
    /// Unique identifier
    pub section_id: Uuid,
    /// Parent course
    pub course_id: Uuid,
    /// Section title
    pub title: String,
    /// Optional description
    pub description: Option<String>,
    /// Display order within course
    pub sort_order: i32,
    /// When created
    pub created_at: DateTime<Utc>,
}

/// Data for creating a new section.
#[derive(Debug, Clone, Deserialize)]
pub struct NewSection {
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

// =============================================================================
// LESSON
// =============================================================================

/// Individual content unit within a section.
///
/// Lessons are the atomic units of learning content. They can be:
/// - Videos (stored in MinIO via content-service)
/// - Articles (markdown content)
/// - Quizzes (linked to assignments-service)
/// - Assignments (linked to assignments-service)
///
/// # Database Mapping
///
/// Maps to `lessons` table.
///
/// # Content Reference
///
/// The `content_ref` field contains:
/// - For videos: MinIO object key
/// - For articles: Article content or reference
/// - For quizzes: Quiz ID in assignments-service
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lesson {
    /// Unique identifier
    pub lesson_id: Uuid,
    /// Parent section
    pub section_id: Uuid,
    /// Parent course (denormalized for queries)
    pub course_id: Uuid,
    /// Lesson title
    pub title: String,
    /// Type of content
    pub content_type: LessonType,
    /// Reference to actual content (varies by type)
    pub content_ref: Option<String>,
    /// Duration in seconds (for videos)
    pub duration_seconds: i32,
    /// Whether this lesson is free preview
    pub is_preview: bool,
    /// Display order within section
    pub sort_order: i32,
    /// When created
    pub created_at: DateTime<Utc>,
    /// When last updated
    pub updated_at: DateTime<Utc>,
}

impl Lesson {
    /// Returns human-readable duration.
    pub fn formatted_duration(&self) -> String {
        let minutes = self.duration_seconds / 60;
        let seconds = self.duration_seconds % 60;
        if minutes > 0 {
            format!("{}:{:02}", minutes, seconds)
        } else {
            format!("0:{:02}", seconds)
        }
    }
}

/// Data for creating a new lesson.
#[derive(Debug, Clone, Deserialize)]
pub struct NewLesson {
    pub section_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub content_type: LessonType,
    pub content_ref: Option<String>,
    pub duration_seconds: i32,
    pub is_preview: bool,
    pub sort_order: i32,
}

/// Data for updating an existing lesson.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateLesson {
    pub title: Option<String>,
    pub content_type: Option<LessonType>,
    pub content_ref: Option<Option<String>>,
    pub duration_seconds: Option<i32>,
    pub is_preview: Option<bool>,
    pub sort_order: Option<i32>,
}

// =============================================================================
// COURSE WITH RELATIONS
// =============================================================================

/// Course with its sections and lessons loaded.
///
/// Used for course detail views where full structure is needed.
#[derive(Debug, Clone, Serialize)]
pub struct CourseWithContent {
    /// The course entity
    #[serde(flatten)]
    pub course: Course,
    /// Category info
    pub category: Option<Category>,
    /// Instructor name (joined from users)
    pub instructor_name: String,
    /// Ordered sections with their lessons
    pub sections: Vec<SectionWithLessons>,
    /// Total lesson count
    pub total_lessons: i32,
    /// Total video duration in seconds
    pub total_duration_seconds: i32,
}

/// Section with its lessons.
#[derive(Debug, Clone, Serialize)]
pub struct SectionWithLessons {
    /// The section entity
    #[serde(flatten)]
    pub section: Section,
    /// Ordered lessons in this section
    pub lessons: Vec<Lesson>,
}
