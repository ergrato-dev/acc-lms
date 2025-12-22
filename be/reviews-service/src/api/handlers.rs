//! # API Handlers
//!
//! HTTP request handlers for the reviews service.

use actix_web::{delete, get, patch, post, web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::errors::{ReviewError, ReviewResult};
use crate::services::ReviewsService;

/// Configure review routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(get_course_reviews)
            .service(get_course_review_stats)
            .service(get_review)
            .service(create_review)
            .service(update_review)
            .service(delete_review)
            .service(mark_helpful)
            .service(report_review)
            .service(get_user_review_for_course)
            .service(get_user_reviews)
            .service(get_instructor_stats)
            .service(get_instructor_reviews)
            .service(moderate_review),
    );
}

// ============================================================================
// Public Endpoints
// ============================================================================

/// Get reviews for a course.
///
/// GET /api/v1/courses/{course_id}/reviews
#[get("/courses/{course_id}/reviews")]
async fn get_course_reviews(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    query: web::Query<ListReviewsQuery>,
) -> ReviewResult<HttpResponse> {
    let course_id = path.into_inner();
    let current_user_id = extract_user_id_optional();

    let result = service
        .get_course_reviews(
            course_id,
            current_user_id,
            query.rating,
            &query.sort_by,
            &query.sort_order,
            query.page,
            query.per_page,
        )
        .await?;

    let response = PaginatedReviewsResponse {
        reviews: result.reviews.into_iter().map(Into::into).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get rating statistics for a course.
///
/// GET /api/v1/courses/{course_id}/reviews/stats
#[get("/courses/{course_id}/reviews/stats")]
async fn get_course_review_stats(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
) -> ReviewResult<HttpResponse> {
    let course_id = path.into_inner();
    let stats = service.get_course_rating_stats(course_id).await?;

    Ok(HttpResponse::Ok().json(CourseRatingStatsResponse::from(stats)))
}

/// Get a single review by ID.
///
/// GET /api/v1/reviews/{review_id}
#[get("/reviews/{review_id}")]
async fn get_review(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
) -> ReviewResult<HttpResponse> {
    let review_id = path.into_inner();
    let review = service.get_review(review_id).await?;

    Ok(HttpResponse::Ok().json(ReviewResponse::from(review)))
}

// ============================================================================
// Authenticated Endpoints
// ============================================================================

/// Create a new review.
///
/// POST /api/v1/reviews
#[post("/reviews")]
async fn create_review(
    service: web::Data<ReviewsService>,
    body: web::Json<CreateReviewRequest>,
) -> ReviewResult<HttpResponse> {
    body.validate()
        .map_err(|e| ReviewError::Internal(e.to_string()))?;

    let user_id = extract_user_id()?;
    let request = body.into_inner();

    let review = service
        .create_review(
            user_id,
            request.course_id,
            request.rating,
            request.title,
            request.content,
        )
        .await?;

    Ok(HttpResponse::Created().json(ReviewResponse::from(review)))
}

/// Update an existing review.
///
/// PATCH /api/v1/reviews/{review_id}
#[patch("/reviews/{review_id}")]
async fn update_review(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateReviewRequest>,
) -> ReviewResult<HttpResponse> {
    body.validate()
        .map_err(|e| ReviewError::Internal(e.to_string()))?;

    let review_id = path.into_inner();
    let user_id = extract_user_id()?;
    let request = body.into_inner();

    let review = service
        .update_review(
            review_id,
            user_id,
            request.rating,
            request.title,
            request.content,
        )
        .await?;

    Ok(HttpResponse::Ok().json(ReviewResponse::from(review)))
}

/// Delete a review.
///
/// DELETE /api/v1/reviews/{review_id}
#[delete("/reviews/{review_id}")]
async fn delete_review(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
) -> ReviewResult<HttpResponse> {
    let review_id = path.into_inner();
    let user_id = extract_user_id()?;

    service.delete_review(review_id, user_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Review deleted successfully".to_string(),
    }))
}

/// Mark a review as helpful or remove the mark.
///
/// POST /api/v1/reviews/{review_id}/helpful
#[post("/reviews/{review_id}/helpful")]
async fn mark_helpful(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    body: web::Json<MarkHelpfulRequest>,
) -> ReviewResult<HttpResponse> {
    let review_id = path.into_inner();
    let user_id = extract_user_id()?;

    let helpful_count = service
        .mark_helpful(review_id, user_id, body.helpful)
        .await?;

    Ok(HttpResponse::Ok().json(HelpfulCountResponse {
        review_id,
        helpful_count,
    }))
}

/// Report a review.
///
/// POST /api/v1/reviews/{review_id}/report
#[post("/reviews/{review_id}/report")]
async fn report_review(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    body: web::Json<ReportReviewRequest>,
) -> ReviewResult<HttpResponse> {
    body.validate()
        .map_err(|e| ReviewError::Internal(e.to_string()))?;

    let review_id = path.into_inner();
    let user_id = extract_user_id()?;
    let request = body.into_inner();

    service
        .report_review(review_id, user_id, request.reason.into(), request.description)
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Review reported successfully".to_string(),
    }))
}

/// Get user's review for a specific course.
///
/// GET /api/v1/courses/{course_id}/my-review
#[get("/courses/{course_id}/my-review")]
async fn get_user_review_for_course(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
) -> ReviewResult<HttpResponse> {
    let course_id = path.into_inner();
    let user_id = extract_user_id()?;

    let review = service.get_user_review_for_course(user_id, course_id).await?;

    Ok(HttpResponse::Ok().json(UserCourseReviewResponse {
        has_reviewed: review.is_some(),
        review: review.map(Into::into),
    }))
}

/// Get all reviews by the current user.
///
/// GET /api/v1/users/me/reviews
#[get("/users/me/reviews")]
async fn get_user_reviews(
    service: web::Data<ReviewsService>,
    query: web::Query<ListReviewsQuery>,
) -> ReviewResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let result = service
        .get_user_reviews(user_id, query.page, query.per_page)
        .await?;

    let response = PaginatedReviewsResponse {
        reviews: result.reviews.into_iter().map(Into::into).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

// ============================================================================
// Instructor Endpoints
// ============================================================================

/// Get instructor rating statistics.
///
/// GET /api/v1/instructors/{instructor_id}/stats
#[get("/instructors/{instructor_id}/stats")]
async fn get_instructor_stats(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
) -> ReviewResult<HttpResponse> {
    let instructor_id = path.into_inner();
    let stats = service.get_instructor_rating_stats(instructor_id).await?;

    Ok(HttpResponse::Ok().json(InstructorRatingStatsResponse::from(stats)))
}

/// Get all reviews for an instructor's courses.
///
/// GET /api/v1/instructors/{instructor_id}/reviews
#[get("/instructors/{instructor_id}/reviews")]
async fn get_instructor_reviews(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    query: web::Query<InstructorReviewsQuery>,
) -> ReviewResult<HttpResponse> {
    let instructor_id = path.into_inner();

    let result = service
        .get_instructor_reviews(instructor_id, query.page, query.per_page)
        .await?;

    let response = PaginatedReviewsResponse {
        reviews: result.reviews.into_iter().map(Into::into).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

// ============================================================================
// Admin Endpoints
// ============================================================================

/// Moderate a review (admin only).
///
/// POST /api/v1/admin/reviews/{review_id}/moderate
#[post("/admin/reviews/{review_id}/moderate")]
async fn moderate_review(
    service: web::Data<ReviewsService>,
    path: web::Path<Uuid>,
    body: web::Json<ModerateReviewRequest>,
) -> ReviewResult<HttpResponse> {
    let review_id = path.into_inner();
    let admin_id = extract_admin_id()?;
    let request = body.into_inner();

    let review = service
        .moderate_review(review_id, admin_id, request.status.into(), request.note)
        .await?;

    Ok(HttpResponse::Ok().json(ReviewResponse::from(review)))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract user ID from request (placeholder - should use auth middleware).
fn extract_user_id() -> ReviewResult<Uuid> {
    // TODO: In production, extract from JWT token via auth middleware
    // For now, return a placeholder that will be replaced by actual auth
    Ok(Uuid::nil())
}

/// Extract optional user ID from request.
fn extract_user_id_optional() -> Option<Uuid> {
    // TODO: In production, extract from JWT token if present
    None
}

/// Extract admin ID from request (placeholder - should use auth middleware).
fn extract_admin_id() -> ReviewResult<Uuid> {
    // TODO: In production, extract from JWT token and verify admin role
    Ok(Uuid::nil())
}
