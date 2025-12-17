//! # API Handlers
//!
//! HTTP request handlers for all grades endpoints.

use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use chrono::Utc;
use uuid::Uuid;

use crate::api::{
    AppState, ApiResponse, GradeQueryParams, ExportQueryParams,
    HealthResponse, StudentGradeSummaryResponse, CourseGradeResponse,
    GradeEntryResponse, TranscriptResponse, CourseStatsResponse,
    QuizStatsResponse,
};
use crate::domain::{GradeError, GradeFilter, ExportFormat};

/// Extract user ID from request headers (JWT claim).
fn extract_user_id(req: &HttpRequest) -> Result<Uuid, GradeError> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| GradeError::Unauthorized("Missing or invalid user ID".to_string()))
}

/// Extract user role from request headers.
fn extract_user_role(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-User-Role")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Check if user is admin.
fn is_admin(req: &HttpRequest) -> bool {
    extract_user_role(req).map(|r| r == "admin").unwrap_or(false)
}

/// Check if user is instructor.
fn is_instructor(req: &HttpRequest) -> bool {
    extract_user_role(req).map(|r| r == "instructor" || r == "admin").unwrap_or(false)
}

/// Helper to create error response.
fn error_response(e: GradeError) -> HttpResponse {
    e.error_response()
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "grades-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::success(response))
}

// =============================================================================
// STUDENT GRADE ENDPOINTS
// =============================================================================

/// GET /api/v1/grades/my
/// Get grades summary for the authenticated user.
pub async fn get_my_grades(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<GradeQueryParams>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let filter = GradeFilter {
        user_id: Some(user_id),
        course_id: query.course_id,
        status: query.status.clone(),
        passed: query.passed,
        from_date: query.from_date,
        to_date: query.to_date,
    };

    match state.grade_service.get_student_summary(user_id, filter).await {
        Ok(summary) => {
            let response: StudentGradeSummaryResponse = summary.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/grades/my/courses/{course_id}
/// Get grades for a specific course.
pub async fn get_my_course_grades(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let course_id = path.into_inner();

    match state.grade_service.get_course_grades(user_id, course_id).await {
        Ok(course_grade) => {
            let response: CourseGradeResponse = course_grade.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/grades/submissions/{submission_id}
/// Get a specific submission's grade.
pub async fn get_submission_grade(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let submission_id = path.into_inner();

    match state.grade_service.get_grade_entry(submission_id, Some(user_id)).await {
        Ok(entry) => {
            let response: GradeEntryResponse = entry.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// TRANSCRIPT ENDPOINTS
// =============================================================================

/// GET /api/v1/transcript/my
/// Get academic transcript for the authenticated user.
pub async fn get_my_transcript(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    match state.transcript_service.get_transcript(user_id).await {
        Ok(transcript) => {
            let response: TranscriptResponse = transcript.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/transcript/my/export
/// Export transcript as CSV, JSON, or PDF.
pub async fn export_transcript(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<ExportQueryParams>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    let format = query.format.unwrap_or(ExportFormat::Csv);

    match state.export_service.export_transcript(user_id, format).await {
        Ok(export) => {
            // Return file directly
            HttpResponse::Ok()
                .content_type(export.content_type.as_str())
                .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", export.filename)))
                .body(export.content.unwrap_or_default())
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// INSTRUCTOR STATISTICS ENDPOINTS
// =============================================================================

/// GET /api/v1/stats/courses/{course_id}
/// Get course statistics (instructor only).
pub async fn get_course_stats(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let course_id = path.into_inner();

    // Check if user is instructor of this course
    if !is_instructor(&req) {
        match state.stats_service.verify_instructor_access(user_id, course_id).await {
            Ok(false) => return error_response(GradeError::Forbidden(
                "You do not have access to this course's statistics".to_string()
            )),
            Err(e) => return error_response(e),
            _ => {}
        }
    }

    match state.stats_service.get_course_stats(course_id).await {
        Ok(stats) => {
            let response: CourseStatsResponse = stats.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/stats/quizzes/{quiz_id}
/// Get quiz statistics (instructor only).
pub async fn get_quiz_stats(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let quiz_id = path.into_inner();

    // Verify instructor access
    if !is_instructor(&req) {
        match state.stats_service.verify_quiz_instructor_access(user_id, quiz_id).await {
            Ok(false) => return error_response(GradeError::Forbidden(
                "You do not have access to this quiz's statistics".to_string()
            )),
            Err(e) => return error_response(e),
            _ => {}
        }
    }

    match state.stats_service.get_quiz_stats(quiz_id).await {
        Ok(stats) => {
            let response: QuizStatsResponse = stats.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/stats/courses/{course_id}/export
/// Export all grades for a course as CSV (instructor only).
pub async fn export_course_grades(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<ExportQueryParams>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let course_id = path.into_inner();

    // Verify instructor access
    if !is_instructor(&req) {
        match state.stats_service.verify_instructor_access(user_id, course_id).await {
            Ok(false) => return error_response(GradeError::Forbidden(
                "You do not have access to export this course's grades".to_string()
            )),
            Err(e) => return error_response(e),
            _ => {}
        }
    }

    let format = query.format.unwrap_or(ExportFormat::Csv);

    match state.export_service.export_course_grades(course_id, format).await {
        Ok(export) => {
            HttpResponse::Ok()
                .content_type(export.content_type.as_str())
                .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", export.filename)))
                .body(export.content.unwrap_or_default())
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// ADMIN ENDPOINTS
// =============================================================================

/// GET /api/v1/admin/grades/users/{user_id}
/// Get any user's grades (admin only).
pub async fn admin_get_user_grades(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<GradeQueryParams>,
) -> HttpResponse {
    if !is_admin(&req) {
        return error_response(GradeError::Forbidden(
            "Admin access required".to_string()
        ));
    }

    let target_user_id = path.into_inner();

    let filter = GradeFilter {
        user_id: Some(target_user_id),
        course_id: query.course_id,
        status: query.status.clone(),
        passed: query.passed,
        from_date: query.from_date,
        to_date: query.to_date,
    };

    match state.grade_service.get_student_summary(target_user_id, filter).await {
        Ok(summary) => {
            let response: StudentGradeSummaryResponse = summary.into();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/admin/grades/courses/{course_id}/export
/// Export all grades for a course (admin only).
pub async fn admin_export_course_grades(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<ExportQueryParams>,
) -> HttpResponse {
    if !is_admin(&req) {
        return error_response(GradeError::Forbidden(
            "Admin access required".to_string()
        ));
    }

    let course_id = path.into_inner();
    let format = query.format.unwrap_or(ExportFormat::Csv);

    match state.export_service.export_course_grades(course_id, format).await {
        Ok(export) => {
            HttpResponse::Ok()
                .content_type(export.content_type.as_str())
                .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", export.filename)))
                .body(export.content.unwrap_or_default())
        }
        Err(e) => error_response(e),
    }
}
