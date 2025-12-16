//! # HTTP Handlers
//!
//! Actix-web handlers for enrollment endpoints.

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::service::{EnrollmentError, EnrollmentService};

/// Application state shared across handlers.
pub struct AppState {
    pub enrollment_service: EnrollmentService,
}

// =============================================================================
// ENROLLMENT HANDLERS
// =============================================================================

/// Lists enrollments for the authenticated user.
///
/// GET /api/v1/enrollments
pub async fn list_my_enrollments(
    state: web::Data<AppState>,
    query: web::Query<ListEnrollmentsQuery>,
    // TODO: Extract from JWT middleware
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let result = state.enrollment_service
        .list_user_enrollments(
            user_id.into_inner(),
            query.status(),
            query.page,
            query.page_size,
        )
        .await;

    match result {
        Ok((enrollments, total)) => {
            let data: Vec<EnrollmentDto> = enrollments.into_iter().map(Into::into).collect();
            let pagination = PaginationMeta::new(query.page, query.page_size, total);

            HttpResponse::Ok().json(PaginatedResponse { data, pagination })
        }
        Err(e) => error_response(e),
    }
}

/// Lists enrollments for a course (admin/instructor only).
///
/// GET /api/v1/courses/{course_id}/enrollments
pub async fn list_course_enrollments(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<ListEnrollmentsQuery>,
) -> HttpResponse {
    let course_id = path.into_inner();

    let result = state.enrollment_service
        .list_course_enrollments(course_id, query.page, query.page_size)
        .await;

    match result {
        Ok(enrollments) => {
            let data: Vec<EnrollmentDto> = enrollments.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(data)
        }
        Err(e) => error_response(e),
    }
}

/// Gets enrollment details.
///
/// GET /api/v1/enrollments/{enrollment_id}
pub async fn get_enrollment(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    is_admin: web::ReqData<bool>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .get_enrollment(enrollment_id, user_id.into_inner(), is_admin.into_inner())
        .await;

    match result {
        Ok(enrollment) => HttpResponse::Ok().json(EnrollmentDto::from(enrollment)),
        Err(e) => error_response(e),
    }
}

/// Gets enrollment with progress details.
///
/// GET /api/v1/enrollments/{enrollment_id}/progress
pub async fn get_enrollment_with_progress(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
    is_admin: web::ReqData<bool>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .get_enrollment_with_progress(
            enrollment_id,
            user_id.into_inner(),
            is_admin.into_inner(),
        )
        .await;

    match result {
        Ok(ewp) => HttpResponse::Ok().json(EnrollmentWithProgressDto::from(ewp)),
        Err(e) => error_response(e),
    }
}

/// Checks if user is enrolled in a course.
///
/// GET /api/v1/courses/{course_id}/enrollment/check
pub async fn check_enrollment(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let course_id = path.into_inner();
    let user_id = user_id.into_inner();

    let result = state.enrollment_service
        .check_enrollment(user_id, course_id)
        .await;

    match result {
        Ok(enrollment) => {
            let response = CheckEnrollmentResponse {
                is_enrolled: enrollment.is_some(),
                enrollment: enrollment.map(Into::into),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => error_response(e),
    }
}

/// Enrolls in a course.
///
/// POST /api/v1/enrollments
pub async fn enroll(
    state: web::Data<AppState>,
    body: web::Json<EnrollRequest>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "validation_error",
            "Invalid request data",
        ).with_details(serde_json::to_value(errors).unwrap_or_default()));
    }

    let result = state.enrollment_service
        .enroll_user(
            user_id.into_inner(),
            body.course_id,
            body.enrollment_source.clone(),
            body.expires_at,
        )
        .await;

    match result {
        Ok(enrollment) => HttpResponse::Created().json(EnrollmentDto::from(enrollment)),
        Err(e) => error_response(e),
    }
}

/// Updates enrollment status (admin only).
///
/// PATCH /api/v1/enrollments/{enrollment_id}/status
pub async fn update_enrollment_status(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEnrollmentStatusRequest>,
    user_id: web::ReqData<Uuid>,
    is_admin: web::ReqData<bool>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "validation_error",
            "Invalid request data",
        ).with_details(serde_json::to_value(errors).unwrap_or_default()));
    }

    let enrollment_id = path.into_inner();
    let status = match body.status() {
        Some(s) => s,
        None => return HttpResponse::BadRequest().json(ErrorResponse::new(
            "invalid_status",
            "Invalid enrollment status",
        )),
    };

    let result = state.enrollment_service
        .update_enrollment_status(
            enrollment_id,
            status,
            user_id.into_inner(),
            is_admin.into_inner(),
        )
        .await;

    match result {
        Ok(enrollment) => HttpResponse::Ok().json(EnrollmentDto::from(enrollment)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// PROGRESS HANDLERS
// =============================================================================

/// Starts a lesson (records first access).
///
/// POST /api/v1/enrollments/{enrollment_id}/lessons/start
pub async fn start_lesson(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<StartLessonRequest>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .start_lesson(enrollment_id, body.lesson_id, user_id.into_inner())
        .await;

    match result {
        Ok(progress) => HttpResponse::Ok().json(LessonProgressDto::from(progress)),
        Err(e) => error_response(e),
    }
}

/// Updates lesson progress.
///
/// PATCH /api/v1/enrollments/{enrollment_id}/lessons/progress
pub async fn update_progress(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProgressRequest>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "validation_error",
            "Invalid request data",
        ).with_details(serde_json::to_value(errors).unwrap_or_default()));
    }

    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .update_lesson_progress(
            enrollment_id,
            body.lesson_id,
            user_id.into_inner(),
            body.time_spent_delta,
            body.position_seconds,
            body.completion_percentage,
        )
        .await;

    match result {
        Ok(progress) => HttpResponse::Ok().json(LessonProgressDto::from(progress)),
        Err(e) => error_response(e),
    }
}

/// Marks a lesson as completed.
///
/// POST /api/v1/enrollments/{enrollment_id}/lessons/complete
pub async fn complete_lesson(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<CompleteLessonRequest>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .complete_lesson(
            enrollment_id,
            body.lesson_id,
            user_id.into_inner(),
            body.total_lessons,
        )
        .await;

    match result {
        Ok((progress, course_completed)) => {
            HttpResponse::Ok().json(LessonCompletionResponse {
                progress: progress.into(),
                course_completed,
            })
        }
        Err(e) => error_response(e),
    }
}

/// Saves playback position.
///
/// POST /api/v1/enrollments/{enrollment_id}/lessons/position
pub async fn save_position(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<SavePositionRequest>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "validation_error",
            "Invalid request data",
        ).with_details(serde_json::to_value(errors).unwrap_or_default()));
    }

    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .save_position(
            enrollment_id,
            body.lesson_id,
            user_id.into_inner(),
            body.position_seconds,
        )
        .await;

    match result {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

/// Gets all lesson progress for an enrollment.
///
/// GET /api/v1/enrollments/{enrollment_id}/lessons
pub async fn get_lesson_progress(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .get_lesson_progress(enrollment_id, user_id.into_inner())
        .await;

    match result {
        Ok(progress) => {
            let data: Vec<LessonProgressDto> = progress.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(data)
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// STATISTICS HANDLERS
// =============================================================================

/// Gets course enrollment statistics.
///
/// GET /api/v1/courses/{course_id}/stats
pub async fn get_course_stats(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let course_id = path.into_inner();

    let result = state.enrollment_service
        .get_course_stats(course_id)
        .await;

    match result {
        Ok(stats) => HttpResponse::Ok().json(CourseStatsDto::from(stats)),
        Err(e) => error_response(e),
    }
}

/// Gets user learning statistics.
///
/// GET /api/v1/users/{user_id}/stats
pub async fn get_user_stats(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    requesting_user_id: web::ReqData<Uuid>,
    is_admin: web::ReqData<bool>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let requesting_id = requesting_user_id.into_inner();
    let admin = is_admin.into_inner();

    // Users can only view their own stats unless admin
    if user_id != requesting_id && !admin {
        return HttpResponse::Forbidden().json(ErrorResponse::new(
            "forbidden",
            "Not authorized to view these statistics",
        ));
    }

    let result = state.enrollment_service
        .get_user_stats(user_id)
        .await;

    match result {
        Ok(stats) => HttpResponse::Ok().json(UserStatsDto::from(stats)),
        Err(e) => error_response(e),
    }
}

/// Gets my learning statistics.
///
/// GET /api/v1/me/stats
pub async fn get_my_stats(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let result = state.enrollment_service
        .get_user_stats(user_id.into_inner())
        .await;

    match result {
        Ok(stats) => HttpResponse::Ok().json(UserStatsDto::from(stats)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// CERTIFICATE HANDLERS
// =============================================================================

/// Issues a certificate for a completed enrollment (admin only).
///
/// POST /api/v1/enrollments/{enrollment_id}/certificate
pub async fn issue_certificate(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    is_admin: web::ReqData<bool>,
) -> HttpResponse {
    let enrollment_id = path.into_inner();

    let result = state.enrollment_service
        .issue_certificate(enrollment_id, is_admin.into_inner())
        .await;

    match result {
        Ok(enrollment) => HttpResponse::Ok().json(EnrollmentDto::from(enrollment)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// ERROR HANDLING
// =============================================================================

fn error_response(error: EnrollmentError) -> HttpResponse {
    match error {
        EnrollmentError::NotFound => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "not_found",
                "Enrollment not found",
            ))
        }
        EnrollmentError::AlreadyEnrolled => {
            HttpResponse::Conflict().json(ErrorResponse::new(
                "already_enrolled",
                "User is already enrolled in this course",
            ))
        }
        EnrollmentError::CourseNotFound => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "course_not_found",
                "Course not found or unavailable",
            ))
        }
        EnrollmentError::Unauthorized => {
            HttpResponse::Forbidden().json(ErrorResponse::new(
                "forbidden",
                "Not authorized to access this enrollment",
            ))
        }
        EnrollmentError::Expired => {
            HttpResponse::Gone().json(ErrorResponse::new(
                "expired",
                "Enrollment has expired",
            ))
        }
        EnrollmentError::AlreadyCompleted => {
            HttpResponse::BadRequest().json(ErrorResponse::new(
                "already_completed",
                "Cannot perform action on completed enrollment",
            ))
        }
        EnrollmentError::LessonNotFound => {
            HttpResponse::NotFound().json(ErrorResponse::new(
                "lesson_not_found",
                "Lesson not found in course",
            ))
        }
        EnrollmentError::Validation(msg) => {
            HttpResponse::BadRequest().json(ErrorResponse::new(
                "validation_error",
                msg,
            ))
        }
        EnrollmentError::Database(e) => {
            tracing::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new(
                "internal_error",
                "An internal error occurred",
            ))
        }
    }
}
