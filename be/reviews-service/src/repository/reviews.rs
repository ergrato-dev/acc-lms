//! # Reviews Repository
//!
//! Database operations for reviews.

use sqlx::PgPool;
use uuid::Uuid;

use crate::api::dto::{ReviewSortBy, SortOrder};
use crate::domain::entities::{
    CourseRatingStats, HelpfulVote, InstructorRatingStats, PaginatedReviews,
    ReportReason, Review, ReviewReport, ReviewStatus, ReviewWithUser,
};
use crate::domain::errors::{ReviewError, ReviewResult};

/// Reviews repository for database operations.
#[derive(Clone)]
pub struct ReviewsRepository {
    pool: PgPool,
}

impl ReviewsRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Review CRUD Operations
    // ========================================================================

    /// Create a new review.
    pub async fn create_review(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        rating: i16,
        title: Option<String>,
        content: String,
    ) -> ReviewResult<Review> {
        let review: Review = sqlx::query_as(
            r#"
            INSERT INTO reviews (user_id, course_id, rating, title, content, status)
            VALUES ($1, $2, $3, $4, $5, 'published')
            RETURNING
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .bind(rating)
        .bind(title)
        .bind(Some(content))
        .fetch_one(&self.pool)
        .await?;

        Ok(review)
    }

    /// Get a review by ID.
    pub async fn get_review(&self, review_id: Uuid) -> ReviewResult<Review> {
        let review: Option<Review> = sqlx::query_as(
            r#"
            SELECT
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            FROM reviews
            WHERE review_id = $1 AND status != 'hidden' AND status != 'deleted'
            "#,
        )
        .bind(review_id)
        .fetch_optional(&self.pool)
        .await?;

        review.ok_or(ReviewError::NotFound(review_id))
    }

    /// Get a review by ID (include hidden, for admin).
    pub async fn get_review_admin(&self, review_id: Uuid) -> ReviewResult<Review> {
        let review: Option<Review> = sqlx::query_as(
            r#"
            SELECT
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            FROM reviews
            WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .fetch_optional(&self.pool)
        .await?;

        review.ok_or(ReviewError::NotFound(review_id))
    }

    /// Update a review.
    pub async fn update_review(
        &self,
        review_id: Uuid,
        rating: Option<i16>,
        title: Option<String>,
        content: Option<String>,
    ) -> ReviewResult<Review> {
        let review: Option<Review> = sqlx::query_as(
            r#"
            UPDATE reviews
            SET
                rating = COALESCE($2, rating),
                title = COALESCE($3, title),
                content = COALESCE($4, content),
                updated_at = NOW()
            WHERE review_id = $1
            RETURNING
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            "#,
        )
        .bind(review_id)
        .bind(rating)
        .bind(title)
        .bind(content)
        .fetch_optional(&self.pool)
        .await?;

        review.ok_or(ReviewError::NotFound(review_id))
    }

    /// Delete a review (soft delete - mark as deleted).
    pub async fn delete_review(&self, review_id: Uuid) -> ReviewResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE reviews SET status = 'deleted', updated_at = NOW() WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ReviewError::NotFound(review_id));
        }

        Ok(())
    }

    /// Update review status (moderation).
    pub async fn update_review_status(
        &self,
        review_id: Uuid,
        status: ReviewStatus,
    ) -> ReviewResult<Review> {
        let status_str = match status {
            ReviewStatus::Published => "published",
            ReviewStatus::PendingModeration => "pending_moderation",
            ReviewStatus::Hidden => "hidden",
            ReviewStatus::Deleted => "deleted",
        };

        let review: Option<Review> = sqlx::query_as(
            r#"
            UPDATE reviews
            SET status = $2::text::review_status, updated_at = NOW()
            WHERE review_id = $1
            RETURNING
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            "#,
        )
        .bind(review_id)
        .bind(status_str)
        .fetch_optional(&self.pool)
        .await?;

        review.ok_or(ReviewError::NotFound(review_id))
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Check if user has already reviewed a course.
    pub async fn user_has_reviewed(&self, user_id: Uuid, course_id: Uuid) -> ReviewResult<bool> {
        let row: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM reviews
                WHERE user_id = $1 AND course_id = $2 AND status != 'deleted'
            )
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    /// Get user's review for a course.
    pub async fn get_user_review_for_course(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> ReviewResult<Option<Review>> {
        let review: Option<Review> = sqlx::query_as(
            r#"
            SELECT
                review_id, course_id, user_id, rating, title, content,
                helpful_count, status,
                instructor_response, instructor_response_at,
                created_at, updated_at
            FROM reviews
            WHERE user_id = $1 AND course_id = $2 AND status != 'deleted'
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(review)
    }

    /// Get reviews for a course with pagination and sorting.
    pub async fn get_course_reviews(
        &self,
        course_id: Uuid,
        current_user_id: Option<Uuid>,
        rating_filter: Option<i16>,
        sort_by: &ReviewSortBy,
        sort_order: &SortOrder,
        page: i32,
        page_size: i32,
    ) -> ReviewResult<PaginatedReviews> {
        let offset = (page - 1) * page_size;

        // Build ORDER BY clause
        let order_by = match (sort_by, sort_order) {
            (ReviewSortBy::MostRecent, SortOrder::Desc) => "r.created_at DESC",
            (ReviewSortBy::MostRecent, SortOrder::Asc) => "r.created_at ASC",
            (ReviewSortBy::MostHelpful, SortOrder::Desc) => "r.helpful_count DESC, r.created_at DESC",
            (ReviewSortBy::MostHelpful, SortOrder::Asc) => "r.helpful_count ASC, r.created_at DESC",
            (ReviewSortBy::HighestRated, _) => "r.rating DESC, r.created_at DESC",
            (ReviewSortBy::LowestRated, _) => "r.rating ASC, r.created_at DESC",
        };

        // Get total count
        let total_row: (i64,) = if let Some(rating) = rating_filter {
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM reviews
                WHERE course_id = $1 AND status = 'published' AND rating = $2
                "#,
            )
            .bind(course_id)
            .bind(rating)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM reviews
                WHERE course_id = $1 AND status = 'published'
                "#,
            )
            .bind(course_id)
            .fetch_one(&self.pool)
            .await?
        };
        let total = total_row.0;

        // Build and execute query with dynamic ORDER BY
        let query = format!(
            r#"
            SELECT
                r.review_id, r.course_id, r.user_id,
                COALESCE(u.first_name || ' ' || u.last_name, 'Anonymous') as user_name,
                u.avatar_url as user_avatar_url,
                r.rating, r.title, r.content,
                r.helpful_count,
                r.status::text,
                r.instructor_response, r.instructor_response_at,
                r.created_at, r.updated_at,
                COALESCE(hv.vote_id IS NOT NULL, false) as user_found_helpful
            FROM reviews r
            LEFT JOIN users u ON r.user_id = u.id
            LEFT JOIN helpful_votes hv ON r.review_id = hv.review_id AND hv.user_id = $4
            WHERE r.course_id = $1 AND r.status = 'published'
            {}
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            if rating_filter.is_some() {
                "AND r.rating = $5"
            } else {
                ""
            },
            order_by
        );

        let reviews: Vec<ReviewWithUser> = if let Some(rating) = rating_filter {
            sqlx::query_as(&query)
                .bind(course_id)
                .bind(page_size)
                .bind(offset)
                .bind(current_user_id)
                .bind(rating)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as(&query)
                .bind(course_id)
                .bind(page_size)
                .bind(offset)
                .bind(current_user_id)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(PaginatedReviews::new(reviews, total, page, page_size))
    }

    /// Get all reviews by a user.
    pub async fn get_user_reviews(
        &self,
        user_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> ReviewResult<PaginatedReviews> {
        let offset = (page - 1) * page_size;

        let total_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM reviews
            WHERE user_id = $1 AND status != 'deleted'
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        let total = total_row.0;

        let reviews: Vec<ReviewWithUser> = sqlx::query_as(
            r#"
            SELECT
                r.review_id, r.course_id, r.user_id,
                COALESCE(u.first_name || ' ' || u.last_name, 'Anonymous') as user_name,
                u.avatar_url as user_avatar_url,
                r.rating, r.title, r.content,
                r.helpful_count,
                r.status::text,
                r.instructor_response, r.instructor_response_at,
                r.created_at, r.updated_at,
                false as user_found_helpful
            FROM reviews r
            LEFT JOIN users u ON r.user_id = u.id
            WHERE r.user_id = $1 AND r.status != 'deleted'
            ORDER BY r.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(PaginatedReviews::new(reviews, total, page, page_size))
    }

    /// Get reviews for an instructor's courses.
    pub async fn get_instructor_reviews(
        &self,
        instructor_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> ReviewResult<PaginatedReviews> {
        let offset = (page - 1) * page_size;

        let total_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM reviews r
            JOIN courses c ON r.course_id = c.id
            WHERE c.instructor_id = $1 AND r.status = 'published'
            "#,
        )
        .bind(instructor_id)
        .fetch_one(&self.pool)
        .await?;
        let total = total_row.0;

        let reviews: Vec<ReviewWithUser> = sqlx::query_as(
            r#"
            SELECT
                r.review_id, r.course_id, r.user_id,
                COALESCE(u.first_name || ' ' || u.last_name, 'Anonymous') as user_name,
                u.avatar_url as user_avatar_url,
                r.rating, r.title, r.content,
                r.helpful_count,
                r.status::text,
                r.instructor_response, r.instructor_response_at,
                r.created_at, r.updated_at,
                false as user_found_helpful
            FROM reviews r
            JOIN courses c ON r.course_id = c.id
            LEFT JOIN users u ON r.user_id = u.id
            WHERE c.instructor_id = $1 AND r.status = 'published'
            ORDER BY r.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(instructor_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(PaginatedReviews::new(reviews, total, page, page_size))
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get rating statistics for a course.
    pub async fn get_course_rating_stats(&self, course_id: Uuid) -> ReviewResult<CourseRatingStats> {
        let stats: CourseRatingStats = sqlx::query_as(
            r#"
            SELECT
                $1::uuid as course_id,
                COALESCE(AVG(rating)::decimal, 0) as average_rating,
                COUNT(*) as total_reviews,
                COUNT(*) FILTER (WHERE rating = 5) as rating_5_count,
                COUNT(*) FILTER (WHERE rating = 4) as rating_4_count,
                COUNT(*) FILTER (WHERE rating = 3) as rating_3_count,
                COUNT(*) FILTER (WHERE rating = 2) as rating_2_count,
                COUNT(*) FILTER (WHERE rating = 1) as rating_1_count
            FROM reviews
            WHERE course_id = $1 AND status = 'published'
            "#,
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    /// Get rating statistics for an instructor.
    pub async fn get_instructor_rating_stats(
        &self,
        instructor_id: Uuid,
    ) -> ReviewResult<InstructorRatingStats> {
        let stats: InstructorRatingStats = sqlx::query_as(
            r#"
            SELECT
                $1::uuid as instructor_id,
                COALESCE(AVG(r.rating)::decimal, 0) as average_rating,
                COUNT(r.review_id) as total_reviews,
                COUNT(DISTINCT c.id) as total_courses
            FROM courses c
            LEFT JOIN reviews r ON c.id = r.course_id AND r.status = 'published'
            WHERE c.instructor_id = $1
            "#,
        )
        .bind(instructor_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    // ========================================================================
    // Helpful Votes
    // ========================================================================

    /// Check if user has voted on a review.
    pub async fn get_user_vote(
        &self,
        review_id: Uuid,
        user_id: Uuid,
    ) -> ReviewResult<Option<HelpfulVote>> {
        let vote: Option<HelpfulVote> = sqlx::query_as(
            r#"
            SELECT vote_id, review_id, user_id, created_at
            FROM helpful_votes
            WHERE review_id = $1 AND user_id = $2
            "#,
        )
        .bind(review_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(vote)
    }

    /// Add a helpful vote on a review.
    pub async fn add_vote(
        &self,
        review_id: Uuid,
        user_id: Uuid,
    ) -> ReviewResult<HelpfulVote> {
        let vote: HelpfulVote = sqlx::query_as(
            r#"
            INSERT INTO helpful_votes (review_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (review_id, user_id) DO NOTHING
            RETURNING vote_id, review_id, user_id, created_at
            "#,
        )
        .bind(review_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Update vote count on the review
        self.update_vote_count(review_id).await?;

        Ok(vote)
    }

    /// Remove a vote from a review.
    pub async fn remove_vote(&self, review_id: Uuid, user_id: Uuid) -> ReviewResult<()> {
        sqlx::query(
            r#"
            DELETE FROM helpful_votes
            WHERE review_id = $1 AND user_id = $2
            "#,
        )
        .bind(review_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        // Update vote count on the review
        self.update_vote_count(review_id).await?;

        Ok(())
    }

    /// Update vote count on a review.
    async fn update_vote_count(&self, review_id: Uuid) -> ReviewResult<()> {
        sqlx::query(
            r#"
            UPDATE reviews
            SET
                helpful_count = (
                    SELECT COUNT(*) FROM helpful_votes
                    WHERE review_id = $1
                ),
                updated_at = NOW()
            WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get current vote count for a review.
    pub async fn get_vote_count(&self, review_id: Uuid) -> ReviewResult<i32> {
        let count: (i32,) = sqlx::query_as(
            r#"
            SELECT helpful_count
            FROM reviews
            WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ReviewError::NotFound(review_id))?;

        Ok(count.0)
    }

    // ========================================================================
    // Reports
    // ========================================================================

    /// Create a report for a review.
    pub async fn create_report(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        reason: ReportReason,
        description: Option<String>,
    ) -> ReviewResult<ReviewReport> {
        let reason_str = match reason {
            ReportReason::Spam => "spam",
            ReportReason::Inappropriate => "inappropriate",
            ReportReason::Offensive => "offensive",
            ReportReason::FakeReview => "fake_review",
            ReportReason::Harassment => "harassment",
            ReportReason::Other => "other",
        };

        let report: ReviewReport = sqlx::query_as(
            r#"
            INSERT INTO review_reports (review_id, reporter_id, reason, description)
            VALUES ($1, $2, $3::text::report_reason, $4)
            RETURNING
                report_id, review_id, reporter_id,
                reason,
                description,
                status,
                resolved_by, resolved_at, created_at
            "#,
        )
        .bind(review_id)
        .bind(user_id)
        .bind(reason_str)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(report)
    }

    /// Check if user has already reported a review.
    pub async fn user_has_reported(
        &self,
        review_id: Uuid,
        user_id: Uuid,
    ) -> ReviewResult<bool> {
        let row: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM review_reports
                WHERE review_id = $1 AND reporter_id = $2
            )
            "#,
        )
        .bind(review_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    // ========================================================================
    // Enrollment Check (for validation)
    // ========================================================================

    /// Check if user is enrolled in a course.
    pub async fn is_user_enrolled(&self, user_id: Uuid, course_id: Uuid) -> ReviewResult<bool> {
        let row: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM enrollments
                WHERE user_id = $1 AND course_id = $2 AND status = 'active'
            )
            "#,
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    /// Check if user is the instructor of a course.
    pub async fn is_course_instructor(&self, user_id: Uuid, course_id: Uuid) -> ReviewResult<bool> {
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
