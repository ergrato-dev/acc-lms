//! # Domain Entities
//!
//! Core business entities for wishlist functionality.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// WISHLIST ITEM
// =============================================================================

/// A single item in a user's wishlist.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WishlistItem {
    pub wishlist_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub added_at: DateTime<Utc>,
}

/// Wishlist item with course details for display.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WishlistWithCourse {
    pub wishlist_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub added_at: DateTime<Utc>,
    // Course details
    pub course_title: String,
    pub course_slug: String,
    pub course_description: Option<String>,
    pub course_thumbnail_url: Option<String>,
    pub course_price: Decimal,
    pub course_discount_price: Option<Decimal>,
    pub course_currency: String,
    pub course_level: String,
    pub course_duration_minutes: Option<i32>,
    pub course_rating: Option<Decimal>,
    pub course_review_count: i64,
    pub instructor_id: Uuid,
    pub instructor_name: String,
    pub course_status: String,
}

// =============================================================================
// SUMMARY
// =============================================================================

/// Summary of a user's wishlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishlistSummary {
    pub user_id: Uuid,
    pub total_items: i64,
    pub total_value: Decimal,
    pub total_discounted_value: Decimal,
    pub currency: String,
}

// =============================================================================
// PAGINATION
// =============================================================================

/// Paginated wishlist result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedWishlist {
    pub items: Vec<WishlistWithCourse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl PaginatedWishlist {
    pub fn new(items: Vec<WishlistWithCourse>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
