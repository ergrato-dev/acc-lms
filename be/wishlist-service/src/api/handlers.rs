//! # API Handlers
//!
//! HTTP request handlers for the wishlist service.

use actix_web::{delete, get, post, web, HttpResponse};
use uuid::Uuid;

use crate::api::dto::*;
use crate::domain::errors::WishlistResult;
use crate::services::WishlistService;

/// Configure wishlist routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(get_wishlist)
            .service(get_wishlist_summary)
            .service(add_to_wishlist)
            .service(remove_from_wishlist)
            .service(remove_course_from_wishlist)
            .service(check_in_wishlist)
            .service(check_multiple_in_wishlist)
            .service(clear_wishlist),
    );
}

// ============================================================================
// Wishlist Endpoints
// ============================================================================

/// Get user's wishlist.
///
/// GET /api/v1/wishlist
#[get("/wishlist")]
async fn get_wishlist(
    service: web::Data<WishlistService>,
    query: web::Query<ListWishlistQuery>,
) -> WishlistResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let result = service
        .get_wishlist(
            user_id,
            query.sort_by.clone(),
            query.sort_order.clone(),
            query.page,
            query.per_page,
        )
        .await?;

    let response = PaginatedWishlistResponse {
        items: result.items.into_iter().map(Into::into).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get wishlist summary (total items, value, etc.).
///
/// GET /api/v1/wishlist/summary
#[get("/wishlist/summary")]
async fn get_wishlist_summary(
    service: web::Data<WishlistService>,
) -> WishlistResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let summary = service.get_wishlist_summary(user_id).await?;

    Ok(HttpResponse::Ok().json(WishlistSummaryResponse::from(summary)))
}

/// Add a course to wishlist.
///
/// POST /api/v1/wishlist
#[post("/wishlist")]
async fn add_to_wishlist(
    service: web::Data<WishlistService>,
    body: web::Json<AddToWishlistRequest>,
) -> WishlistResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let item = service.add_to_wishlist(user_id, body.course_id).await?;

    Ok(HttpResponse::Created().json(AddToWishlistResponse {
        wishlist_id: item.wishlist_id,
        course_id: item.course_id,
        message: "Course added to wishlist".to_string(),
    }))
}

/// Remove an item from wishlist by wishlist_id.
///
/// DELETE /api/v1/wishlist/{wishlist_id}
#[delete("/wishlist/{wishlist_id}")]
async fn remove_from_wishlist(
    service: web::Data<WishlistService>,
    path: web::Path<Uuid>,
) -> WishlistResult<HttpResponse> {
    let wishlist_id = path.into_inner();
    let user_id = extract_user_id()?;

    service.remove_from_wishlist(user_id, wishlist_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Course removed from wishlist".to_string(),
    }))
}

/// Remove a course from wishlist by course_id.
///
/// DELETE /api/v1/wishlist/course/{course_id}
#[delete("/wishlist/course/{course_id}")]
async fn remove_course_from_wishlist(
    service: web::Data<WishlistService>,
    path: web::Path<Uuid>,
) -> WishlistResult<HttpResponse> {
    let course_id = path.into_inner();
    let user_id = extract_user_id()?;

    service
        .remove_course_from_wishlist(user_id, course_id)
        .await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Course removed from wishlist".to_string(),
    }))
}

/// Check if a course is in wishlist.
///
/// GET /api/v1/wishlist/check/{course_id}
#[get("/wishlist/check/{course_id}")]
async fn check_in_wishlist(
    service: web::Data<WishlistService>,
    path: web::Path<Uuid>,
) -> WishlistResult<HttpResponse> {
    let course_id = path.into_inner();
    let user_id = extract_user_id()?;

    let wishlist_id = service.is_in_wishlist(user_id, course_id).await?;

    Ok(HttpResponse::Ok().json(InWishlistResponse {
        course_id,
        in_wishlist: wishlist_id.is_some(),
        wishlist_id,
    }))
}

/// Check if multiple courses are in wishlist request body.
#[derive(Debug, serde::Deserialize)]
pub struct CheckMultipleRequest {
    pub course_ids: Vec<Uuid>,
}

/// Check multiple courses in wishlist response.
#[derive(Debug, serde::Serialize)]
pub struct CheckMultipleResponse {
    pub results: Vec<InWishlistResponse>,
}

/// Check if multiple courses are in wishlist.
///
/// POST /api/v1/wishlist/check
#[post("/wishlist/check")]
async fn check_multiple_in_wishlist(
    service: web::Data<WishlistService>,
    body: web::Json<CheckMultipleRequest>,
) -> WishlistResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let results = service
        .check_multiple_in_wishlist(user_id, body.course_ids.clone())
        .await?;

    let response = CheckMultipleResponse {
        results: results
            .into_iter()
            .map(|(course_id, wishlist_id)| InWishlistResponse {
                course_id,
                in_wishlist: wishlist_id.is_some(),
                wishlist_id,
            })
            .collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Clear entire wishlist.
///
/// DELETE /api/v1/wishlist
#[delete("/wishlist")]
async fn clear_wishlist(service: web::Data<WishlistService>) -> WishlistResult<HttpResponse> {
    let user_id = extract_user_id()?;

    let count = service.clear_wishlist(user_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: format!("Removed {} items from wishlist", count),
    }))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract user ID from request (placeholder - should use auth middleware).
fn extract_user_id() -> WishlistResult<Uuid> {
    // TODO: In production, extract from JWT token via auth middleware
    Ok(Uuid::nil())
}
