//! # Wishlist Repository
//!
//! Database operations for wishlists.

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::dto::{SortOrder, WishlistSortBy};
use crate::domain::entities::{PaginatedWishlist, WishlistItem, WishlistSummary, WishlistWithCourse};
use crate::domain::errors::{WishlistError, WishlistResult};

/// Wishlist repository for database operations.
#[derive(Clone)]
pub struct WishlistRepository {
    pool: PgPool,
}

impl WishlistRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Wishlist CRUD Operations
    // ========================================================================

    /// Add a course to user's wishlist.
    pub async fn add_to_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<WishlistItem> {
        let item: WishlistItem = sqlx::query_as(
            r#"
            INSERT INTO wishlists (user_id, course_id)
            VALUES ($1, $2)
            RETURNING wishlist_id, user_id, course_id, added_at
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    /// Remove an item from wishlist by wishlist_id.
    pub async fn remove_from_wishlist(
        &self,
        user_id: Uuid,
        wishlist_id: Uuid,
    ) -> WishlistResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM wishlists
            WHERE wishlist_id = $1 AND user_id = $2
            "#,
        )
        .bind(wishlist_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(WishlistError::NotFound(wishlist_id));
        }

        Ok(())
    }

    /// Remove a course from wishlist by course_id.
    pub async fn remove_course_from_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM wishlists
            WHERE user_id = $1 AND course_id = $2
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(WishlistError::NotFound(course_id));
        }

        Ok(())
    }

    /// Clear entire wishlist for a user.
    pub async fn clear_wishlist(&self, user_id: Uuid) -> WishlistResult<i64> {
        let result = sqlx::query(
            r#"
            DELETE FROM wishlists WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Check if course is in user's wishlist.
    pub async fn is_in_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<Option<Uuid>> {
        let result: Option<(Uuid,)> = sqlx::query_as(
            r#"
            SELECT wishlist_id
            FROM wishlists
            WHERE user_id = $1 AND course_id = $2
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.0))
    }

    /// Check multiple courses in wishlist.
    pub async fn check_multiple_in_wishlist(
        &self,
        user_id: Uuid,
        course_ids: &[Uuid],
    ) -> WishlistResult<Vec<(Uuid, Option<Uuid>)>> {
        let results: Vec<(Uuid, Uuid)> = sqlx::query_as(
            r#"
            SELECT course_id, wishlist_id
            FROM wishlists
            WHERE user_id = $1 AND course_id = ANY($2)
            "#,
        )
        .bind(user_id)
        .bind(course_ids)
        .fetch_all(&self.pool)
        .await?;

        // Create a map of course_id -> wishlist_id
        let wishlist_map: std::collections::HashMap<Uuid, Uuid> =
            results.into_iter().collect();

        // Return results for all requested course_ids
        Ok(course_ids
            .iter()
            .map(|&course_id| (course_id, wishlist_map.get(&course_id).copied()))
            .collect())
    }

    /// Get user's wishlist with course details.
    pub async fn get_wishlist(
        &self,
        user_id: Uuid,
        sort_by: &WishlistSortBy,
        sort_order: &SortOrder,
        page: i32,
        per_page: i32,
    ) -> WishlistResult<PaginatedWishlist> {
        let offset = (page - 1) * per_page;

        // Get total count
        let total_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM wishlists
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        let total = total_row.0;

        // Build ORDER BY clause
        let order_by = match (sort_by, sort_order) {
            (WishlistSortBy::AddedAt, SortOrder::Desc) => "w.added_at DESC",
            (WishlistSortBy::AddedAt, SortOrder::Asc) => "w.added_at ASC",
            (WishlistSortBy::CourseTitle, SortOrder::Desc) => "c.title DESC",
            (WishlistSortBy::CourseTitle, SortOrder::Asc) => "c.title ASC",
            (WishlistSortBy::Price, SortOrder::Desc) => "COALESCE(c.discount_price, c.price) DESC",
            (WishlistSortBy::Price, SortOrder::Asc) => "COALESCE(c.discount_price, c.price) ASC",
            (WishlistSortBy::Rating, SortOrder::Desc) => "COALESCE(c.average_rating, 0) DESC",
            (WishlistSortBy::Rating, SortOrder::Asc) => "COALESCE(c.average_rating, 0) ASC",
        };

        let query = format!(
            r#"
            SELECT
                w.wishlist_id, w.user_id, w.course_id, w.added_at,
                c.title as course_title,
                c.slug as course_slug,
                c.description as course_description,
                c.thumbnail_url as course_thumbnail_url,
                c.price as course_price,
                c.discount_price as course_discount_price,
                COALESCE(c.currency, 'USD') as course_currency,
                c.level as course_level,
                c.duration_minutes as course_duration_minutes,
                c.average_rating as course_rating,
                COALESCE(c.review_count, 0) as course_review_count,
                c.instructor_id,
                COALESCE(u.first_name || ' ' || u.last_name, 'Unknown') as instructor_name,
                c.status as course_status
            FROM wishlists w
            JOIN courses c ON w.course_id = c.id
            LEFT JOIN users u ON c.instructor_id = u.id
            WHERE w.user_id = $1
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            order_by
        );

        let items: Vec<WishlistWithCourse> = sqlx::query_as(&query)
            .bind(user_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(PaginatedWishlist::new(items, total, page, per_page))
    }

    /// Get wishlist summary for a user.
    pub async fn get_wishlist_summary(&self, user_id: Uuid) -> WishlistResult<WishlistSummary> {
        let result: Option<(i64, Option<Decimal>, Option<Decimal>, Option<String>)> = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total_items,
                SUM(c.price) as total_value,
                SUM(COALESCE(c.discount_price, c.price)) as total_discounted_value,
                MAX(c.currency) as currency
            FROM wishlists w
            JOIN courses c ON w.course_id = c.id
            WHERE w.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        let (total_items, total_value, total_discounted, currency) =
            result.unwrap_or((0, None, None, None));

        Ok(WishlistSummary {
            user_id,
            total_items,
            total_value: total_value.unwrap_or_default(),
            total_discounted_value: total_discounted.unwrap_or_default(),
            currency: currency.unwrap_or_else(|| "USD".to_string()),
        })
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Check if course exists.
    pub async fn course_exists(&self, course_id: Uuid) -> WishlistResult<bool> {
        let row: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM courses WHERE id = $1
            )
            "#,
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    /// Check if user is the instructor of a course.
    pub async fn is_course_instructor(&self, user_id: Uuid, course_id: Uuid) -> WishlistResult<bool> {
        let row: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM courses
                WHERE id = $1 AND instructor_id = $2
            )
            "#,
        )
        .bind(course_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }
}
