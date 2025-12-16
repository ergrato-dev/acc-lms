//! # Course Service
//!
//! Business logic for course management including:
//! - Course CRUD with authorization
//! - Section and lesson management
//! - Publishing workflow
//! - Slug generation
//!
//! ## Authorization Model
//!
//! | Action | Student | Instructor | Admin |
//! |--------|---------|------------|-------|
//! | List published | ✓ | ✓ | ✓ |
//! | View detail | ✓ (if published) | ✓ (own) | ✓ |
//! | Create | ✗ | ✓ | ✓ |
//! | Update | ✗ | ✓ (own) | ✓ |
//! | Publish | ✗ | ✓ (own) | ✓ |
//! | Delete | ✗ | ✓ (own) | ✓ |

use shared::errors::ApiError;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::domain::{
    Category, Course, CourseWithContent, Lesson, NewCourse, NewLesson, NewSection,
    Section, SectionWithLessons, Slug, UpdateCourse, UpdateLesson,
};
use crate::repository::CourseRepository;

// =============================================================================
// USER ROLE (simplified for this service)
// =============================================================================

/// User role for authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Student,
    Instructor,
    Admin,
}

impl UserRole {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "instructor" => UserRole::Instructor,
            "admin" => UserRole::Admin,
            _ => UserRole::Student,
        }
    }

    pub fn can_create_course(&self) -> bool {
        matches!(self, UserRole::Instructor | UserRole::Admin)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

// =============================================================================
// COURSE SERVICE
// =============================================================================

/// Service for course business operations.
pub struct CourseService {
    repository: CourseRepository,
}

impl CourseService {
    /// Creates a new course service.
    pub fn new(repository: CourseRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // COURSE LISTING
    // =========================================================================

    /// Lists published courses with pagination.
    ///
    /// Available to all users (public endpoint).
    #[instrument(skip(self))]
    pub async fn list_courses(
        &self,
        page: i64,
        page_size: i64,
        category_id: Option<Uuid>,
        search: Option<&str>,
        min_price: Option<i32>,
        max_price: Option<i32>,
    ) -> Result<CoursesListResponse, ApiError> {
        let page_size = page_size.min(50).max(1); // Limit page size
        let page = page.max(1);

        let (courses, total) = self
            .repository
            .list_published(page, page_size, category_id, search, min_price, max_price)
            .await?;

        Ok(CoursesListResponse {
            courses,
            pagination: PaginationMeta {
                page,
                page_size,
                total,
                total_pages: (total as f64 / page_size as f64).ceil() as i64,
            },
        })
    }

    /// Lists courses for an instructor.
    ///
    /// Instructors see all their courses (including drafts).
    /// Admins can view any instructor's courses.
    #[instrument(skip(self))]
    pub async fn list_instructor_courses(
        &self,
        instructor_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Vec<Course>, ApiError> {
        // Authorization: only the instructor or admin
        if instructor_id != requesting_user_id && !requesting_role.is_admin() {
            return Err(ApiError::AccessDenied);
        }

        self.repository.list_by_instructor(instructor_id).await
    }

    // =========================================================================
    // COURSE DETAIL
    // =========================================================================

    /// Gets course by ID with full content structure.
    ///
    /// - Published courses: visible to everyone
    /// - Draft courses: only owner or admin
    #[instrument(skip(self))]
    pub async fn get_course(
        &self,
        course_id: Uuid,
        requesting_user_id: Option<Uuid>,
        requesting_role: Option<UserRole>,
    ) -> Result<CourseWithContent, ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization check for unpublished courses
        if !course.is_published {
            let is_owner = requesting_user_id == Some(course.instructor_id);
            let is_admin = requesting_role == Some(UserRole::Admin);

            if !is_owner && !is_admin {
                return Err(ApiError::NotFound {
                    resource: "Course".to_string(),
                });
            }
        }

        // Load sections and lessons
        let sections = self.repository.list_sections(course_id).await?;
        let mut sections_with_lessons = Vec::with_capacity(sections.len());
        let mut total_lessons = 0;
        let mut total_duration = 0;

        for section in sections {
            let lessons = self.repository.list_lessons(section.section_id).await?;
            total_lessons += lessons.len() as i32;
            total_duration += lessons.iter().map(|l| l.duration_seconds).sum::<i32>();

            sections_with_lessons.push(SectionWithLessons { section, lessons });
        }

        // Load category if present
        let category = if course.category_id.is_some() {
            // In production, we'd load the specific category
            None // Simplified for now
        } else {
            None
        };

        Ok(CourseWithContent {
            course,
            category,
            instructor_name: "Instructor".to_string(), // Would be loaded from users-service
            sections: sections_with_lessons,
            total_lessons,
            total_duration_seconds: total_duration,
        })
    }

    /// Gets course by slug.
    #[instrument(skip(self))]
    pub async fn get_course_by_slug(
        &self,
        slug: &str,
        requesting_user_id: Option<Uuid>,
        requesting_role: Option<UserRole>,
    ) -> Result<CourseWithContent, ApiError> {
        let course = self
            .repository
            .find_by_slug(slug)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        self.get_course(course.course_id, requesting_user_id, requesting_role)
            .await
    }

    // =========================================================================
    // COURSE CRUD
    // =========================================================================

    /// Creates a new course.
    ///
    /// Only instructors and admins can create courses.
    #[instrument(skip(self, new_course))]
    pub async fn create_course(
        &self,
        new_course: NewCourse,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Course, ApiError> {
        // Authorization
        if !requesting_role.can_create_course() {
            return Err(ApiError::AccessDenied);
        }

        // Validate instructor_id matches requesting user (unless admin)
        if new_course.instructor_id != requesting_user_id && !requesting_role.is_admin() {
            return Err(ApiError::AccessDenied);
        }

        // Generate slug
        let base_slug = new_course
            .slug
            .clone()
            .map(|s| Slug::from_title(&s))
            .unwrap_or_else(|| Slug::from_title(&new_course.title));

        let slug = self.ensure_unique_slug(base_slug.to_string()).await?;

        // Create course
        let course = self.repository.create(new_course, slug).await?;

        info!(
            course_id = %course.course_id,
            slug = %course.slug,
            "Course created"
        );

        Ok(course)
    }

    /// Updates an existing course.
    ///
    /// Only owner or admin can update.
    #[instrument(skip(self, update))]
    pub async fn update_course(
        &self,
        course_id: Uuid,
        update: UpdateCourse,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Course, ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.update(course_id, update).await
    }

    /// Publishes a course.
    ///
    /// Only owner or admin can publish.
    #[instrument(skip(self))]
    pub async fn publish_course(
        &self,
        course_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Course, ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        // Validation: course must have at least one lesson
        let lessons = self.repository.list_course_lessons(course_id).await?;
        if lessons.is_empty() {
            return Err(ApiError::BadRequest {
                message: "Course must have at least one lesson to publish".to_string(),
            });
        }

        let published = self.repository.publish(course_id).await?;

        info!(course_id = %course_id, "Course published");

        // TODO: Emit course.published event

        Ok(published)
    }

    /// Unpublishes a course.
    #[instrument(skip(self))]
    pub async fn unpublish_course(
        &self,
        course_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Course, ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        let unpublished = self.repository.unpublish(course_id).await?;

        info!(course_id = %course_id, "Course unpublished");

        Ok(unpublished)
    }

    /// Deletes a course (soft delete).
    #[instrument(skip(self))]
    pub async fn delete_course(
        &self,
        course_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<(), ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.delete(course_id).await?;

        info!(course_id = %course_id, "Course deleted");

        Ok(())
    }

    // =========================================================================
    // CATEGORIES
    // =========================================================================

    /// Lists all categories.
    #[instrument(skip(self))]
    pub async fn list_categories(&self) -> Result<Vec<Category>, ApiError> {
        self.repository.list_categories().await
    }

    // =========================================================================
    // SECTIONS
    // =========================================================================

    /// Creates a new section.
    #[instrument(skip(self, new_section))]
    pub async fn create_section(
        &self,
        new_section: NewSection,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Section, ApiError> {
        let course = self
            .repository
            .find_by_id(new_section.course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.create_section(new_section).await
    }

    /// Deletes a section.
    #[instrument(skip(self))]
    pub async fn delete_section(
        &self,
        section_id: Uuid,
        course_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<(), ApiError> {
        let course = self
            .repository
            .find_by_id(course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.delete_section(section_id).await
    }

    // =========================================================================
    // LESSONS
    // =========================================================================

    /// Creates a new lesson.
    #[instrument(skip(self, new_lesson))]
    pub async fn create_lesson(
        &self,
        new_lesson: NewLesson,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Lesson, ApiError> {
        let course = self
            .repository
            .find_by_id(new_lesson.course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.create_lesson(new_lesson).await
    }

    /// Updates a lesson.
    #[instrument(skip(self, update))]
    pub async fn update_lesson(
        &self,
        lesson_id: Uuid,
        update: UpdateLesson,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<Lesson, ApiError> {
        let lesson = self
            .repository
            .find_lesson(lesson_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Lesson".to_string(),
            })?;

        let course = self
            .repository
            .find_by_id(lesson.course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.update_lesson(lesson_id, update).await
    }

    /// Deletes a lesson.
    #[instrument(skip(self))]
    pub async fn delete_lesson(
        &self,
        lesson_id: Uuid,
        requesting_user_id: Uuid,
        requesting_role: UserRole,
    ) -> Result<(), ApiError> {
        let lesson = self
            .repository
            .find_lesson(lesson_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Lesson".to_string(),
            })?;

        let course = self
            .repository
            .find_by_id(lesson.course_id)
            .await?
            .ok_or(ApiError::NotFound {
                resource: "Course".to_string(),
            })?;

        // Authorization
        if !course.can_edit(requesting_user_id, requesting_role.is_admin()) {
            return Err(ApiError::AccessDenied);
        }

        self.repository.delete_lesson(lesson_id).await
    }

    // =========================================================================
    // HELPERS
    // =========================================================================

    /// Ensures a slug is unique, appending numbers if needed.
    async fn ensure_unique_slug(&self, base_slug: String) -> Result<String, ApiError> {
        let mut slug = base_slug.clone();
        let mut counter = 1;

        while self.repository.slug_exists(&slug).await? {
            slug = format!("{}-{}", base_slug, counter);
            counter += 1;

            if counter > 100 {
                return Err(ApiError::InternalError {
                    message: "Could not generate unique slug".to_string(),
                });
            }
        }

        Ok(slug)
    }
}

// =============================================================================
// RESPONSE TYPES
// =============================================================================

/// Response for paginated course listings.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoursesListResponse {
    pub courses: Vec<Course>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}
