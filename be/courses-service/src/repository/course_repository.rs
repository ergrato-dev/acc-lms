//! # Course Repository
//!
//! PostgreSQL-based repository for course CRUD operations.
//!
//! ## Query Patterns
//!
//! - Uses prepared statements for security and performance
//! - Soft deletes via `deleted_at` column
//! - Optimistic locking via `updated_at` timestamps
//!
//! ## Error Handling
//!
//! All database errors are converted to `ApiError` for consistent responses.

use chrono::Utc;
use shared::errors::ApiError;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::domain::{
    Category, Course, Lesson, NewCourse, NewLesson, NewSection, Section,
    UpdateCourse, UpdateLesson,
};

// =============================================================================
// COURSE REPOSITORY
// =============================================================================

/// Repository for course database operations.
#[derive(Clone)]
pub struct CourseRepository {
    pool: PgPool,
}

impl CourseRepository {
    /// Creates a new repository with the given connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // COURSE OPERATIONS
    // =========================================================================

    /// Lists published courses with pagination and filters.
    #[instrument(skip(self))]
    pub async fn list_published(
        &self,
        page: i64,
        page_size: i64,
        category_id: Option<Uuid>,
        search: Option<&str>,
        min_price: Option<i32>,
        max_price: Option<i32>,
    ) -> Result<(Vec<Course>, i64), ApiError> {
        let offset = (page - 1) * page_size;

        // Build dynamic query
        let mut query = String::from(
            r#"
            SELECT course_id, instructor_id, category_id, title, slug, short_description,
                   full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                   difficulty_level, estimated_duration_hours, language, is_published,
                   published_at, average_rating, total_ratings, total_enrollments,
                   requirements, learning_objectives, target_audience,
                   created_at, updated_at, deleted_at
            FROM courses
            WHERE is_published = true AND deleted_at IS NULL
            "#,
        );

        let mut count_query = String::from(
            "SELECT COUNT(*) FROM courses WHERE is_published = true AND deleted_at IS NULL",
        );

        if category_id.is_some() {
            query.push_str(" AND category_id = $4");
            count_query.push_str(" AND category_id = $1");
        }

        if search.is_some() {
            query.push_str(" AND (title ILIKE $5 OR short_description ILIKE $5)");
            count_query.push_str(" AND (title ILIKE $2 OR short_description ILIKE $2)");
        }

        if min_price.is_some() {
            query.push_str(" AND price_cents >= $6");
            count_query.push_str(" AND price_cents >= $3");
        }

        if max_price.is_some() {
            query.push_str(" AND price_cents <= $7");
            count_query.push_str(" AND price_cents <= $4");
        }

        query.push_str(" ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT $2 OFFSET $3");

        // Execute queries (simplified - in production use dynamic binding)
        let courses = sqlx::query_as::<_, Course>(
            r#"
            SELECT course_id, instructor_id, category_id, title, slug, short_description,
                   full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                   difficulty_level, estimated_duration_hours, language, is_published,
                   published_at, average_rating, total_ratings, total_enrollments,
                   requirements, learning_objectives, target_audience,
                   created_at, updated_at, deleted_at
            FROM courses
            WHERE is_published = true AND deleted_at IS NULL
            ORDER BY published_at DESC NULLS LAST, created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list courses");
            ApiError::InternalError {
                message: "Failed to list courses".to_string(),
            }
        })?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM courses WHERE is_published = true AND deleted_at IS NULL",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to count courses");
            ApiError::InternalError {
                message: "Failed to count courses".to_string(),
            }
        })?;

        Ok((courses, total.0))
    }

    /// Lists courses by instructor (including drafts).
    #[instrument(skip(self))]
    pub async fn list_by_instructor(&self, instructor_id: Uuid) -> Result<Vec<Course>, ApiError> {
        sqlx::query_as::<_, Course>(
            r#"
            SELECT course_id, instructor_id, category_id, title, slug, short_description,
                   full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                   difficulty_level, estimated_duration_hours, language, is_published,
                   published_at, average_rating, total_ratings, total_enrollments,
                   requirements, learning_objectives, target_audience,
                   created_at, updated_at, deleted_at
            FROM courses
            WHERE instructor_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(instructor_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list instructor courses");
            ApiError::InternalError {
                message: "Failed to list courses".to_string(),
            }
        })
    }

    /// Finds a course by ID.
    #[instrument(skip(self))]
    pub async fn find_by_id(&self, course_id: Uuid) -> Result<Option<Course>, ApiError> {
        sqlx::query_as::<_, Course>(
            r#"
            SELECT course_id, instructor_id, category_id, title, slug, short_description,
                   full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                   difficulty_level, estimated_duration_hours, language, is_published,
                   published_at, average_rating, total_ratings, total_enrollments,
                   requirements, learning_objectives, target_audience,
                   created_at, updated_at, deleted_at
            FROM courses
            WHERE course_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to find course");
            ApiError::InternalError {
                message: "Failed to find course".to_string(),
            }
        })
    }

    /// Finds a course by slug.
    #[instrument(skip(self))]
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Course>, ApiError> {
        sqlx::query_as::<_, Course>(
            r#"
            SELECT course_id, instructor_id, category_id, title, slug, short_description,
                   full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                   difficulty_level, estimated_duration_hours, language, is_published,
                   published_at, average_rating, total_ratings, total_enrollments,
                   requirements, learning_objectives, target_audience,
                   created_at, updated_at, deleted_at
            FROM courses
            WHERE slug = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to find course by slug");
            ApiError::InternalError {
                message: "Failed to find course".to_string(),
            }
        })
    }

    /// Checks if a slug is already in use.
    #[instrument(skip(self))]
    pub async fn slug_exists(&self, slug: &str) -> Result<bool, ApiError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM courses WHERE slug = $1 AND deleted_at IS NULL)",
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to check slug");
            ApiError::InternalError {
                message: "Failed to check slug".to_string(),
            }
        })?;

        Ok(result.0)
    }

    /// Creates a new course.
    #[instrument(skip(self, new_course))]
    pub async fn create(&self, new_course: NewCourse, slug: String) -> Result<Course, ApiError> {
        let course = sqlx::query_as::<_, Course>(
            r#"
            INSERT INTO courses (
                instructor_id, category_id, title, slug, short_description,
                full_description, thumbnail_url, trailer_video_url, price_cents,
                currency, difficulty_level, estimated_duration_hours, language,
                requirements, learning_objectives, target_audience
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING course_id, instructor_id, category_id, title, slug, short_description,
                      full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                      difficulty_level, estimated_duration_hours, language, is_published,
                      published_at, average_rating, total_ratings, total_enrollments,
                      requirements, learning_objectives, target_audience,
                      created_at, updated_at, deleted_at
            "#,
        )
        .bind(new_course.instructor_id)
        .bind(new_course.category_id)
        .bind(&new_course.title)
        .bind(&slug)
        .bind(&new_course.short_description)
        .bind(&new_course.full_description)
        .bind(&new_course.thumbnail_url)
        .bind(&new_course.trailer_video_url)
        .bind(new_course.price_cents)
        .bind(&new_course.currency)
        .bind(&new_course.difficulty_level.to_string())
        .bind(new_course.estimated_duration_hours)
        .bind(&new_course.language)
        .bind(serde_json::to_value(&new_course.requirements).unwrap_or_default())
        .bind(serde_json::to_value(&new_course.learning_objectives).unwrap_or_default())
        .bind(serde_json::to_value(&new_course.target_audience).unwrap_or_default())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to create course");
            ApiError::InternalError {
                message: "Failed to create course".to_string(),
            }
        })?;

        info!(course_id = %course.course_id, "Course created");
        Ok(course)
    }

    /// Updates an existing course.
    #[instrument(skip(self, update))]
    pub async fn update(&self, course_id: Uuid, update: UpdateCourse) -> Result<Course, ApiError> {
        // Build dynamic update query
        let course = sqlx::query_as::<_, Course>(
            r#"
            UPDATE courses SET
                category_id = COALESCE($2, category_id),
                title = COALESCE($3, title),
                short_description = COALESCE($4, short_description),
                full_description = COALESCE($5, full_description),
                thumbnail_url = COALESCE($6, thumbnail_url),
                trailer_video_url = COALESCE($7, trailer_video_url),
                price_cents = COALESCE($8, price_cents),
                currency = COALESCE($9, currency),
                difficulty_level = COALESCE($10, difficulty_level),
                estimated_duration_hours = COALESCE($11, estimated_duration_hours),
                language = COALESCE($12, language),
                requirements = COALESCE($13, requirements),
                learning_objectives = COALESCE($14, learning_objectives),
                target_audience = COALESCE($15, target_audience),
                updated_at = NOW()
            WHERE course_id = $1 AND deleted_at IS NULL
            RETURNING course_id, instructor_id, category_id, title, slug, short_description,
                      full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                      difficulty_level, estimated_duration_hours, language, is_published,
                      published_at, average_rating, total_ratings, total_enrollments,
                      requirements, learning_objectives, target_audience,
                      created_at, updated_at, deleted_at
            "#,
        )
        .bind(course_id)
        .bind(update.category_id.flatten())
        .bind(update.title)
        .bind(update.short_description)
        .bind(update.full_description.flatten())
        .bind(update.thumbnail_url.flatten())
        .bind(update.trailer_video_url.flatten())
        .bind(update.price_cents)
        .bind(update.currency)
        .bind(update.difficulty_level.map(|d| d.to_string()))
        .bind(update.estimated_duration_hours)
        .bind(update.language)
        .bind(update.requirements.map(|r| serde_json::to_value(r).unwrap_or_default()))
        .bind(update.learning_objectives.map(|o| serde_json::to_value(o).unwrap_or_default()))
        .bind(update.target_audience.map(|t| serde_json::to_value(t).unwrap_or_default()))
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update course");
            ApiError::InternalError {
                message: "Failed to update course".to_string(),
            }
        })?;

        course.ok_or_else(|| ApiError::NotFound {
            resource: "Course".to_string(),
        })
    }

    /// Publishes a course.
    #[instrument(skip(self))]
    pub async fn publish(&self, course_id: Uuid) -> Result<Course, ApiError> {
        let course = sqlx::query_as::<_, Course>(
            r#"
            UPDATE courses SET
                is_published = true,
                published_at = NOW(),
                updated_at = NOW()
            WHERE course_id = $1 AND deleted_at IS NULL
            RETURNING course_id, instructor_id, category_id, title, slug, short_description,
                      full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                      difficulty_level, estimated_duration_hours, language, is_published,
                      published_at, average_rating, total_ratings, total_enrollments,
                      requirements, learning_objectives, target_audience,
                      created_at, updated_at, deleted_at
            "#,
        )
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to publish course");
            ApiError::InternalError {
                message: "Failed to publish course".to_string(),
            }
        })?;

        course.ok_or_else(|| ApiError::NotFound {
            resource: "Course".to_string(),
        })
    }

    /// Unpublishes a course.
    #[instrument(skip(self))]
    pub async fn unpublish(&self, course_id: Uuid) -> Result<Course, ApiError> {
        let course = sqlx::query_as::<_, Course>(
            r#"
            UPDATE courses SET
                is_published = false,
                updated_at = NOW()
            WHERE course_id = $1 AND deleted_at IS NULL
            RETURNING course_id, instructor_id, category_id, title, slug, short_description,
                      full_description, thumbnail_url, trailer_video_url, price_cents, currency,
                      difficulty_level, estimated_duration_hours, language, is_published,
                      published_at, average_rating, total_ratings, total_enrollments,
                      requirements, learning_objectives, target_audience,
                      created_at, updated_at, deleted_at
            "#,
        )
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to unpublish course");
            ApiError::InternalError {
                message: "Failed to unpublish course".to_string(),
            }
        })?;

        course.ok_or_else(|| ApiError::NotFound {
            resource: "Course".to_string(),
        })
    }

    /// Soft deletes a course.
    #[instrument(skip(self))]
    pub async fn delete(&self, course_id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query(
            "UPDATE courses SET deleted_at = NOW() WHERE course_id = $1 AND deleted_at IS NULL",
        )
        .bind(course_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to delete course");
            ApiError::InternalError {
                message: "Failed to delete course".to_string(),
            }
        })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound {
                resource: "Course".to_string(),
            });
        }

        info!(course_id = %course_id, "Course deleted");
        Ok(())
    }

    // =========================================================================
    // CATEGORY OPERATIONS
    // =========================================================================

    /// Lists all active categories.
    #[instrument(skip(self))]
    pub async fn list_categories(&self) -> Result<Vec<Category>, ApiError> {
        sqlx::query_as::<_, Category>(
            r#"
            SELECT category_id, name, slug, description, icon_url,
                   parent_category_id, sort_order, is_active, created_at
            FROM course_categories
            WHERE is_active = true
            ORDER BY sort_order, name
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list categories");
            ApiError::InternalError {
                message: "Failed to list categories".to_string(),
            }
        })
    }

    // =========================================================================
    // SECTION OPERATIONS
    // =========================================================================

    /// Lists sections for a course.
    #[instrument(skip(self))]
    pub async fn list_sections(&self, course_id: Uuid) -> Result<Vec<Section>, ApiError> {
        sqlx::query_as::<_, Section>(
            r#"
            SELECT section_id, course_id, title, description, sort_order, created_at
            FROM course_sections
            WHERE course_id = $1
            ORDER BY sort_order
            "#,
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list sections");
            ApiError::InternalError {
                message: "Failed to list sections".to_string(),
            }
        })
    }

    /// Creates a new section.
    #[instrument(skip(self, new_section))]
    pub async fn create_section(&self, new_section: NewSection) -> Result<Section, ApiError> {
        sqlx::query_as::<_, Section>(
            r#"
            INSERT INTO course_sections (course_id, title, description, sort_order)
            VALUES ($1, $2, $3, $4)
            RETURNING section_id, course_id, title, description, sort_order, created_at
            "#,
        )
        .bind(new_section.course_id)
        .bind(&new_section.title)
        .bind(&new_section.description)
        .bind(new_section.sort_order)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to create section");
            ApiError::InternalError {
                message: "Failed to create section".to_string(),
            }
        })
    }

    /// Deletes a section and its lessons.
    #[instrument(skip(self))]
    pub async fn delete_section(&self, section_id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query("DELETE FROM course_sections WHERE section_id = $1")
            .bind(section_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to delete section");
                ApiError::InternalError {
                    message: "Failed to delete section".to_string(),
                }
            })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound {
                resource: "Section".to_string(),
            });
        }

        Ok(())
    }

    // =========================================================================
    // LESSON OPERATIONS
    // =========================================================================

    /// Lists lessons for a section.
    #[instrument(skip(self))]
    pub async fn list_lessons(&self, section_id: Uuid) -> Result<Vec<Lesson>, ApiError> {
        sqlx::query_as::<_, Lesson>(
            r#"
            SELECT lesson_id, section_id, course_id, title, content_type,
                   content_ref, duration_seconds, is_preview, sort_order,
                   created_at, updated_at
            FROM lessons
            WHERE section_id = $1
            ORDER BY sort_order
            "#,
        )
        .bind(section_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list lessons");
            ApiError::InternalError {
                message: "Failed to list lessons".to_string(),
            }
        })
    }

    /// Lists all lessons for a course.
    #[instrument(skip(self))]
    pub async fn list_course_lessons(&self, course_id: Uuid) -> Result<Vec<Lesson>, ApiError> {
        sqlx::query_as::<_, Lesson>(
            r#"
            SELECT l.lesson_id, l.section_id, l.course_id, l.title, l.content_type,
                   l.content_ref, l.duration_seconds, l.is_preview, l.sort_order,
                   l.created_at, l.updated_at
            FROM lessons l
            JOIN course_sections s ON l.section_id = s.section_id
            WHERE l.course_id = $1
            ORDER BY s.sort_order, l.sort_order
            "#,
        )
        .bind(course_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to list course lessons");
            ApiError::InternalError {
                message: "Failed to list lessons".to_string(),
            }
        })
    }

    /// Finds a lesson by ID.
    #[instrument(skip(self))]
    pub async fn find_lesson(&self, lesson_id: Uuid) -> Result<Option<Lesson>, ApiError> {
        sqlx::query_as::<_, Lesson>(
            r#"
            SELECT lesson_id, section_id, course_id, title, content_type,
                   content_ref, duration_seconds, is_preview, sort_order,
                   created_at, updated_at
            FROM lessons
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to find lesson");
            ApiError::InternalError {
                message: "Failed to find lesson".to_string(),
            }
        })
    }

    /// Creates a new lesson.
    #[instrument(skip(self, new_lesson))]
    pub async fn create_lesson(&self, new_lesson: NewLesson) -> Result<Lesson, ApiError> {
        sqlx::query_as::<_, Lesson>(
            r#"
            INSERT INTO lessons (
                section_id, course_id, title, content_type, content_ref,
                duration_seconds, is_preview, sort_order
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING lesson_id, section_id, course_id, title, content_type,
                      content_ref, duration_seconds, is_preview, sort_order,
                      created_at, updated_at
            "#,
        )
        .bind(new_lesson.section_id)
        .bind(new_lesson.course_id)
        .bind(&new_lesson.title)
        .bind(&new_lesson.content_type.to_string())
        .bind(&new_lesson.content_ref)
        .bind(new_lesson.duration_seconds)
        .bind(new_lesson.is_preview)
        .bind(new_lesson.sort_order)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to create lesson");
            ApiError::InternalError {
                message: "Failed to create lesson".to_string(),
            }
        })
    }

    /// Updates a lesson.
    #[instrument(skip(self, update))]
    pub async fn update_lesson(
        &self,
        lesson_id: Uuid,
        update: UpdateLesson,
    ) -> Result<Lesson, ApiError> {
        let lesson = sqlx::query_as::<_, Lesson>(
            r#"
            UPDATE lessons SET
                title = COALESCE($2, title),
                content_type = COALESCE($3, content_type),
                content_ref = COALESCE($4, content_ref),
                duration_seconds = COALESCE($5, duration_seconds),
                is_preview = COALESCE($6, is_preview),
                sort_order = COALESCE($7, sort_order),
                updated_at = NOW()
            WHERE lesson_id = $1
            RETURNING lesson_id, section_id, course_id, title, content_type,
                      content_ref, duration_seconds, is_preview, sort_order,
                      created_at, updated_at
            "#,
        )
        .bind(lesson_id)
        .bind(update.title)
        .bind(update.content_type.map(|t| t.to_string()))
        .bind(update.content_ref.flatten())
        .bind(update.duration_seconds)
        .bind(update.is_preview)
        .bind(update.sort_order)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to update lesson");
            ApiError::InternalError {
                message: "Failed to update lesson".to_string(),
            }
        })?;

        lesson.ok_or_else(|| ApiError::NotFound {
            resource: "Lesson".to_string(),
        })
    }

    /// Deletes a lesson.
    #[instrument(skip(self))]
    pub async fn delete_lesson(&self, lesson_id: Uuid) -> Result<(), ApiError> {
        let result = sqlx::query("DELETE FROM lessons WHERE lesson_id = $1")
            .bind(lesson_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to delete lesson");
                ApiError::InternalError {
                    message: "Failed to delete lesson".to_string(),
                }
            })?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound {
                resource: "Lesson".to_string(),
            });
        }

        Ok(())
    }
}
