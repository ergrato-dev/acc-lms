//! # HTTP Request Handlers
//!
//! Handlers for all courses API endpoints.

use actix_web::{web, HttpRequest, HttpResponse};
use shared::errors::ApiError;
use tracing::{info, instrument};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::{
    CategoryDto, CourseDetailDto, CourseDto, CoursesListDto, CreateCourseRequest,
    CreateLessonRequest, CreateSectionRequest, LessonDto, ListCoursesQuery,
    PaginationDto, UpdateCourseRequest, UpdateLessonRequest,
};
use crate::domain::{DifficultyLevel, NewCourse, NewLesson, NewSection, UpdateCourse, UpdateLesson};
use crate::service::course_service::UserRole;
use crate::AppState;

// =============================================================================
// HELPERS
// =============================================================================

/// Extracts user claims from the Authorization header.
///
/// Returns (user_id, role) if present.
fn extract_user_claims(req: &HttpRequest) -> (Option<Uuid>, Option<UserRole>) {
    // In production, this would decode the JWT and extract claims
    // For now, we use custom headers for testing
    let user_id = req
        .headers()
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    let role = req
        .headers()
        .get("X-User-Role")
        .and_then(|v| v.to_str().ok())
        .map(UserRole::from_str);

    (user_id, role)
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "courses-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// =============================================================================
// COURSES
// =============================================================================

/// Lists published courses with pagination and filters.
///
/// `GET /api/v1/courses`
#[instrument(skip(state))]
pub async fn list_courses(
    state: web::Data<AppState>,
    query: web::Query<ListCoursesQuery>,
) -> Result<HttpResponse, ApiError> {
    let response = state
        .course_service
        .list_courses(
            query.page.unwrap_or(1),
            query.page_size.unwrap_or(20),
            query.category_id,
            query.search.as_deref(),
            query.min_price,
            query.max_price,
        )
        .await?;

    let dto = CoursesListDto {
        courses: response.courses.into_iter().map(CourseDto::from).collect(),
        pagination: PaginationDto::from(response.pagination),
    };

    Ok(HttpResponse::Ok().json(dto))
}

/// Gets course detail by ID.
///
/// `GET /api/v1/courses/:id`
#[instrument(skip(state, req))]
pub async fn get_course(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);

    let course = state
        .course_service
        .get_course(course_id, user_id, role)
        .await?;

    Ok(HttpResponse::Ok().json(CourseDetailDto::from(course)))
}

/// Gets course detail by slug.
///
/// `GET /api/v1/courses/slug/:slug`
#[instrument(skip(state, req))]
pub async fn get_course_by_slug(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let slug = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);

    let course = state
        .course_service
        .get_course_by_slug(&slug, user_id, role)
        .await?;

    Ok(HttpResponse::Ok().json(CourseDetailDto::from(course)))
}

/// Creates a new course.
///
/// `POST /api/v1/courses`
#[instrument(skip(state, req, body))]
pub async fn create_course(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateCourseRequest>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;

    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let new_course = NewCourse {
        instructor_id: user_id,
        category_id: body.category_id,
        title: body.title.clone(),
        slug: body.slug.clone(),
        short_description: body.short_description.clone(),
        full_description: body.full_description.clone(),
        thumbnail_url: body.thumbnail_url.clone(),
        trailer_video_url: body.trailer_video_url.clone(),
        price_cents: body.price_cents,
        currency: body.currency.clone().unwrap_or_else(|| "USD".to_string()),
        difficulty_level: body.difficulty_level.unwrap_or(DifficultyLevel::Beginner),
        estimated_duration_hours: body.estimated_duration_hours.unwrap_or(1),
        language: body.language.clone().unwrap_or_else(|| "en".to_string()),
        requirements: body.requirements.clone().unwrap_or_default(),
        learning_objectives: body.learning_objectives.clone().unwrap_or_default(),
        target_audience: body.target_audience.clone().unwrap_or_default(),
    };

    let course = state
        .course_service
        .create_course(new_course, user_id, role)
        .await?;

    info!(course_id = %course.course_id, "Course created via API");

    Ok(HttpResponse::Created().json(CourseDto::from(course)))
}

/// Updates a course.
///
/// `PATCH /api/v1/courses/:id`
#[instrument(skip(state, req, body))]
pub async fn update_course(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
    body: web::Json<UpdateCourseRequest>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;

    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let update = UpdateCourse {
        category_id: body.category_id.map(Some),
        title: body.title.clone(),
        short_description: body.short_description.clone(),
        full_description: body.full_description.clone().map(Some),
        thumbnail_url: body.thumbnail_url.clone().map(Some),
        trailer_video_url: body.trailer_video_url.clone().map(Some),
        price_cents: body.price_cents,
        currency: body.currency.clone(),
        difficulty_level: body.difficulty_level,
        estimated_duration_hours: body.estimated_duration_hours,
        language: body.language.clone(),
        requirements: body.requirements.clone(),
        learning_objectives: body.learning_objectives.clone(),
        target_audience: body.target_audience.clone(),
    };

    let course = state
        .course_service
        .update_course(course_id, update, user_id, role)
        .await?;

    Ok(HttpResponse::Ok().json(CourseDto::from(course)))
}

/// Publishes a course.
///
/// `POST /api/v1/courses/:id/publish`
#[instrument(skip(state, req))]
pub async fn publish_course(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let course = state
        .course_service
        .publish_course(course_id, user_id, role)
        .await?;

    info!(course_id = %course_id, "Course published via API");

    Ok(HttpResponse::Ok().json(CourseDto::from(course)))
}

/// Unpublishes a course.
///
/// `POST /api/v1/courses/:id/unpublish`
#[instrument(skip(state, req))]
pub async fn unpublish_course(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let course = state
        .course_service
        .unpublish_course(course_id, user_id, role)
        .await?;

    Ok(HttpResponse::Ok().json(CourseDto::from(course)))
}

/// Deletes a course.
///
/// `DELETE /api/v1/courses/:id`
#[instrument(skip(state, req))]
pub async fn delete_course(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    state
        .course_service
        .delete_course(course_id, user_id, role)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// CATEGORIES
// =============================================================================

/// Lists all categories.
///
/// `GET /api/v1/categories`
#[instrument(skip(state))]
pub async fn list_categories(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let categories = state.course_service.list_categories().await?;
    let dtos: Vec<CategoryDto> = categories.into_iter().map(CategoryDto::from).collect();

    Ok(HttpResponse::Ok().json(dtos))
}

// =============================================================================
// SECTIONS
// =============================================================================

/// Creates a new section.
///
/// `POST /api/v1/courses/:courseId/sections`
#[instrument(skip(state, req, body))]
pub async fn create_section(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
    body: web::Json<CreateSectionRequest>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;

    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let new_section = NewSection {
        course_id,
        title: body.title.clone(),
        description: body.description.clone(),
        sort_order: body.sort_order.unwrap_or(0),
    };

    let section = state
        .course_service
        .create_section(new_section, user_id, role)
        .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "sectionId": section.section_id,
        "courseId": section.course_id,
        "title": section.title,
        "description": section.description,
        "sortOrder": section.sort_order
    })))
}

/// Deletes a section.
///
/// `DELETE /api/v1/courses/:courseId/sections/:sectionId`
#[instrument(skip(state, req))]
pub async fn delete_section(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (course_id, section_id) = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    state
        .course_service
        .delete_section(section_id, course_id, user_id, role)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// LESSONS
// =============================================================================

/// Creates a new lesson.
///
/// `POST /api/v1/courses/:courseId/lessons`
#[instrument(skip(state, req, body))]
pub async fn create_lesson(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
    body: web::Json<CreateLessonRequest>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;

    let course_id = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let new_lesson = NewLesson {
        section_id: body.section_id,
        course_id,
        title: body.title.clone(),
        content_type: body.content_type,
        content_ref: body.content_ref.clone(),
        duration_seconds: body.duration_seconds.unwrap_or(0),
        is_preview: body.is_preview.unwrap_or(false),
        sort_order: body.sort_order.unwrap_or(0),
    };

    let lesson = state
        .course_service
        .create_lesson(new_lesson, user_id, role)
        .await?;

    Ok(HttpResponse::Created().json(LessonDto::from(lesson)))
}

/// Updates a lesson.
///
/// `PATCH /api/v1/courses/:courseId/lessons/:lessonId`
#[instrument(skip(state, req, body))]
pub async fn update_lesson(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    req: HttpRequest,
    body: web::Json<UpdateLessonRequest>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;

    let (_course_id, lesson_id) = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let update = UpdateLesson {
        title: body.title.clone(),
        content_type: body.content_type,
        content_ref: body.content_ref.clone().map(Some),
        duration_seconds: body.duration_seconds,
        is_preview: body.is_preview,
        sort_order: body.sort_order,
    };

    let lesson = state
        .course_service
        .update_lesson(lesson_id, update, user_id, role)
        .await?;

    Ok(HttpResponse::Ok().json(LessonDto::from(lesson)))
}

/// Deletes a lesson.
///
/// `DELETE /api/v1/courses/:courseId/lessons/:lessonId`
#[instrument(skip(state, req))]
pub async fn delete_lesson(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (_course_id, lesson_id) = path.into_inner();
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    state
        .course_service
        .delete_lesson(lesson_id, user_id, role)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// INSTRUCTOR COURSES
// =============================================================================

/// Lists courses for the authenticated instructor.
///
/// `GET /api/v1/instructor/courses`
#[instrument(skip(state, req))]
pub async fn list_instructor_courses(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let (user_id, role) = extract_user_claims(&req);
    let user_id = user_id.ok_or(ApiError::MissingAuth)?;
    let role = role.unwrap_or(UserRole::Student);

    let courses = state
        .course_service
        .list_instructor_courses(user_id, user_id, role)
        .await?;

    let dtos: Vec<CourseDto> = courses.into_iter().map(CourseDto::from).collect();

    Ok(HttpResponse::Ok().json(dtos))
}
