//! # API Data Transfer Objects
//!
//! Request and response DTOs for the wishlist API.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{WishlistSummary, WishlistWithCourse};

// ============================================================================
// Request DTOs
// ============================================================================

/// Add course to wishlist request.
#[derive(Debug, Deserialize)]
pub struct AddToWishlistRequest {
    pub course_id: Uuid,
}

/// Query parameters for listing wishlist.
#[derive(Debug, Deserialize, Default)]
pub struct ListWishlistQuery {
    /// Sort field
    #[serde(default)]
    pub sort_by: WishlistSortBy,

    /// Sort direction
    #[serde(default)]
    pub sort_order: SortOrder,

    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: i32,

    /// Items per page
    #[serde(default = "default_page_size")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

/// Wishlist sort options.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WishlistSortBy {
    #[default]
    AddedAt,
    CourseTitle,
    Price,
    Rating,
}

/// Sort order.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

// ============================================================================
// Response DTOs
// ============================================================================

/// Wishlist item response.
#[derive(Debug, Serialize)]
pub struct WishlistItemResponse {
    pub wishlist_id: Uuid,
    pub course_id: Uuid,
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub course: CoursePreviewResponse,
}

/// Course preview for wishlist display.
#[derive(Debug, Serialize)]
pub struct CoursePreviewResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub price: Decimal,
    pub discount_price: Option<Decimal>,
    pub currency: String,
    pub level: String,
    pub duration_minutes: Option<i32>,
    pub rating: Option<Decimal>,
    pub review_count: i64,
    pub instructor: InstructorPreviewResponse,
    pub status: String,
}

/// Instructor preview for wishlist display.
#[derive(Debug, Serialize)]
pub struct InstructorPreviewResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<WishlistWithCourse> for WishlistItemResponse {
    fn from(item: WishlistWithCourse) -> Self {
        Self {
            wishlist_id: item.wishlist_id,
            course_id: item.course_id,
            added_at: item.added_at,
            course: CoursePreviewResponse {
                id: item.course_id,
                title: item.course_title,
                slug: item.course_slug,
                description: item.course_description,
                thumbnail_url: item.course_thumbnail_url,
                price: item.course_price,
                discount_price: item.course_discount_price,
                currency: item.course_currency,
                level: item.course_level,
                duration_minutes: item.course_duration_minutes,
                rating: item.course_rating,
                review_count: item.course_review_count,
                instructor: InstructorPreviewResponse {
                    id: item.instructor_id,
                    name: item.instructor_name,
                },
                status: item.course_status,
            },
        }
    }
}

/// Paginated wishlist response.
#[derive(Debug, Serialize)]
pub struct PaginatedWishlistResponse {
    pub items: Vec<WishlistItemResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Wishlist summary response.
#[derive(Debug, Serialize)]
pub struct WishlistSummaryResponse {
    pub total_items: i64,
    pub total_value: Decimal,
    pub total_discounted_value: Decimal,
    pub currency: String,
    pub potential_savings: Decimal,
}

impl From<WishlistSummary> for WishlistSummaryResponse {
    fn from(summary: WishlistSummary) -> Self {
        let savings = summary.total_value - summary.total_discounted_value;
        Self {
            total_items: summary.total_items,
            total_value: summary.total_value,
            total_discounted_value: summary.total_discounted_value,
            currency: summary.currency,
            potential_savings: savings,
        }
    }
}

/// Check if course is in wishlist response.
#[derive(Debug, Serialize)]
pub struct InWishlistResponse {
    pub course_id: Uuid,
    pub in_wishlist: bool,
    pub wishlist_id: Option<Uuid>,
}

/// Success message response.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Add to wishlist response.
#[derive(Debug, Serialize)]
pub struct AddToWishlistResponse {
    pub wishlist_id: Uuid,
    pub course_id: Uuid,
    pub message: String,
}
