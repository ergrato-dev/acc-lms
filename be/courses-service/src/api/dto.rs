//! # Data Transfer Objects (DTOs)
//!
//! Request and response types for the courses API.
//! All DTOs use camelCase for JSON serialization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Category, Course, CourseWithContent, DifficultyLevel, Lesson, LessonType,
    Section, SectionWithLessons,
};

#[allow(unused_imports)]
use crate::domain::CourseStatus;
use crate::service::PaginationMeta;

// =============================================================================
// COURSE DTOs
// =============================================================================

/// Request to create a new course.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCourseRequest {
    /// Course title (required)
    #[validate(length(min = 3, max = 200, message = "Title must be 3-200 characters"))]
    pub title: String,

    /// Brief description for catalog cards
    #[validate(length(min = 10, max = 300, message = "Short description must be 10-300 characters"))]
    pub short_description: String,

    /// Full description (optional, supports markdown)
    #[validate(length(max = 10000, message = "Description too long"))]
    pub full_description: Option<String>,

    /// Category ID (optional)
    pub category_id: Option<Uuid>,

    /// Custom slug (optional, auto-generated if not provided)
    #[validate(length(min = 3, max = 100))]
    pub slug: Option<String>,

    /// Price in cents (0 for free)
    #[validate(range(min = 0, max = 9999999, message = "Invalid price"))]
    pub price_cents: i32,

    /// Currency code (default: USD)
    pub currency: Option<String>,

    /// Difficulty level
    pub difficulty_level: Option<DifficultyLevel>,

    /// Estimated hours to complete
    #[validate(range(min = 1, max = 1000))]
    pub estimated_duration_hours: Option<i32>,

    /// Content language
    pub language: Option<String>,

    /// Thumbnail URL
    pub thumbnail_url: Option<String>,

    /// Trailer video URL
    pub trailer_video_url: Option<String>,

    /// Prerequisites
    pub requirements: Option<Vec<String>>,

    /// What students will learn
    pub learning_objectives: Option<Vec<String>>,

    /// Target audience
    pub target_audience: Option<Vec<String>>,
}

/// Request to update a course.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCourseRequest {
    #[validate(length(min = 3, max = 200))]
    pub title: Option<String>,

    #[validate(length(min = 10, max = 300))]
    pub short_description: Option<String>,

    #[validate(length(max = 10000))]
    pub full_description: Option<String>,

    pub category_id: Option<Uuid>,

    #[validate(range(min = 0, max = 9999999))]
    pub price_cents: Option<i32>,

    pub currency: Option<String>,

    pub difficulty_level: Option<DifficultyLevel>,

    #[validate(range(min = 1, max = 1000))]
    pub estimated_duration_hours: Option<i32>,

    pub language: Option<String>,

    pub thumbnail_url: Option<String>,

    pub trailer_video_url: Option<String>,

    pub requirements: Option<Vec<String>>,

    pub learning_objectives: Option<Vec<String>>,

    pub target_audience: Option<Vec<String>>,
}

/// Course response DTO for listings.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDto {
    pub course_id: Uuid,
    pub instructor_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub thumbnail_url: Option<String>,
    pub price_cents: i32,
    pub currency: String,
    pub formatted_price: String,
    pub difficulty_level: String,
    pub estimated_duration_hours: i32,
    pub language: String,
    pub is_published: bool,
    pub average_rating: f64,
    pub total_ratings: i32,
    pub total_enrollments: i32,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<Course> for CourseDto {
    fn from(c: Course) -> Self {
        Self {
            course_id: c.course_id,
            instructor_id: c.instructor_id,
            category_id: c.category_id,
            title: c.title.clone(),
            slug: c.slug.clone(),
            short_description: c.short_description.clone(),
            thumbnail_url: c.thumbnail_url.clone(),
            price_cents: c.price_cents,
            currency: c.currency.clone(),
            formatted_price: c.formatted_price(),
            difficulty_level: c.difficulty_level.to_string(),
            estimated_duration_hours: c.estimated_duration_hours,
            language: c.language.clone(),
            is_published: c.is_published,
            average_rating: c.average_rating,
            total_ratings: c.total_ratings,
            total_enrollments: c.total_enrollments,
            created_at: c.created_at,
            published_at: c.published_at,
        }
    }
}

/// Full course detail response.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDetailDto {
    pub course_id: Uuid,
    pub instructor_id: Uuid,
    pub instructor_name: String,
    pub category: Option<CategoryDto>,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub full_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub price_cents: i32,
    pub currency: String,
    pub formatted_price: String,
    pub difficulty_level: String,
    pub estimated_duration_hours: i32,
    pub language: String,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub average_rating: f64,
    pub total_ratings: i32,
    pub total_enrollments: i32,
    pub requirements: Vec<String>,
    pub learning_objectives: Vec<String>,
    pub target_audience: Vec<String>,
    pub sections: Vec<SectionDto>,
    pub total_lessons: i32,
    pub total_duration_seconds: i32,
    pub total_duration_formatted: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CourseWithContent> for CourseDetailDto {
    fn from(cwc: CourseWithContent) -> Self {
        let c = cwc.course;
        let total_hours = cwc.total_duration_seconds / 3600;
        let total_minutes = (cwc.total_duration_seconds % 3600) / 60;
        let duration_formatted = if total_hours > 0 {
            format!("{}h {}m", total_hours, total_minutes)
        } else {
            format!("{}m", total_minutes)
        };

        Self {
            course_id: c.course_id,
            instructor_id: c.instructor_id,
            instructor_name: cwc.instructor_name,
            category: cwc.category.map(CategoryDto::from),
            title: c.title,
            slug: c.slug,
            short_description: c.short_description,
            full_description: c.full_description,
            thumbnail_url: c.thumbnail_url,
            trailer_video_url: c.trailer_video_url,
            price_cents: c.price_cents,
            currency: c.currency.clone(),
            formatted_price: format!(
                "{}",
                if c.price_cents == 0 {
                    "Free".to_string()
                } else {
                    format!("${:.2}", c.price_cents as f64 / 100.0)
                }
            ),
            difficulty_level: c.difficulty_level.to_string(),
            estimated_duration_hours: c.estimated_duration_hours,
            language: c.language,
            is_published: c.is_published,
            published_at: c.published_at,
            average_rating: c.average_rating,
            total_ratings: c.total_ratings,
            total_enrollments: c.total_enrollments,
            requirements: serde_json::from_value(c.requirements).unwrap_or_default(),
            learning_objectives: serde_json::from_value(c.learning_objectives).unwrap_or_default(),
            target_audience: serde_json::from_value(c.target_audience).unwrap_or_default(),
            sections: cwc.sections.into_iter().map(SectionDto::from).collect(),
            total_lessons: cwc.total_lessons,
            total_duration_seconds: cwc.total_duration_seconds,
            total_duration_formatted: duration_formatted,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

/// Paginated courses list response.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoursesListDto {
    pub courses: Vec<CourseDto>,
    pub pagination: PaginationDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationDto {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl From<PaginationMeta> for PaginationDto {
    fn from(p: PaginationMeta) -> Self {
        Self {
            page: p.page,
            page_size: p.page_size,
            total: p.total,
            total_pages: p.total_pages,
        }
    }
}

// =============================================================================
// CATEGORY DTOs
// =============================================================================

/// Category response DTO.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub parent_category_id: Option<Uuid>,
}

impl From<Category> for CategoryDto {
    fn from(c: Category) -> Self {
        Self {
            category_id: c.category_id,
            name: c.name,
            slug: c.slug,
            description: c.description,
            icon_url: c.icon_url,
            parent_category_id: c.parent_category_id,
        }
    }
}

// =============================================================================
// SECTION DTOs
// =============================================================================

/// Request to create a section.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSectionRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub sort_order: Option<i32>,
}

/// Section response DTO.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionDto {
    pub section_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub lessons: Vec<LessonDto>,
    pub total_duration_seconds: i32,
}

impl From<SectionWithLessons> for SectionDto {
    fn from(swl: SectionWithLessons) -> Self {
        let total_duration: i32 = swl.lessons.iter().map(|l| l.duration_seconds).sum();
        Self {
            section_id: swl.section.section_id,
            course_id: swl.section.course_id,
            title: swl.section.title,
            description: swl.section.description,
            sort_order: swl.section.sort_order,
            lessons: swl.lessons.into_iter().map(LessonDto::from).collect(),
            total_duration_seconds: total_duration,
        }
    }
}

// =============================================================================
// LESSON DTOs
// =============================================================================

/// Request to create a lesson.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateLessonRequest {
    pub section_id: Uuid,

    #[validate(length(min = 1, max = 200))]
    pub title: String,

    pub content_type: LessonType,

    pub content_ref: Option<String>,

    #[validate(range(min = 0))]
    pub duration_seconds: Option<i32>,

    pub is_preview: Option<bool>,

    pub sort_order: Option<i32>,
}

/// Request to update a lesson.
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLessonRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,

    pub content_type: Option<LessonType>,

    pub content_ref: Option<String>,

    #[validate(range(min = 0))]
    pub duration_seconds: Option<i32>,

    pub is_preview: Option<bool>,

    pub sort_order: Option<i32>,
}

/// Lesson response DTO.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonDto {
    pub lesson_id: Uuid,
    pub section_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub content_type: String,
    pub content_ref: Option<String>,
    pub duration_seconds: i32,
    pub duration_formatted: String,
    pub is_preview: bool,
    pub sort_order: i32,
}

impl From<Lesson> for LessonDto {
    fn from(l: Lesson) -> Self {
        let duration_formatted = l.formatted_duration();
        Self {
            lesson_id: l.lesson_id,
            section_id: l.section_id,
            course_id: l.course_id,
            title: l.title,
            content_type: l.content_type.to_string(),
            content_ref: l.content_ref,
            duration_seconds: l.duration_seconds,
            duration_formatted,
            is_preview: l.is_preview,
            sort_order: l.sort_order,
        }
    }
}

// =============================================================================
// QUERY PARAMETERS
// =============================================================================

/// Query parameters for course listing.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCoursesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub category_id: Option<Uuid>,
    pub search: Option<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
}

// =============================================================================
// COMMON RESPONSES
// =============================================================================

/// Generic message response.
#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
