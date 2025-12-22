//! # Reviews Service
//!
//! Business logic for review operations.

use redis::AsyncCommands;
use uuid::Uuid;

use crate::api::dto::{ReviewSortBy, SortOrder};
use crate::domain::entities::{
    CourseRatingStats, InstructorRatingStats, PaginatedReviews, ReportReason, Review, ReviewStatus,
};
use crate::domain::errors::{ReviewError, ReviewResult};
use crate::repository::ReviewsRepository;

/// Minimum content length for reviews.
const MIN_CONTENT_LENGTH: usize = 10;
/// Maximum content length for reviews.
const MAX_CONTENT_LENGTH: usize = 5000;
/// Cache TTL for rating stats (5 minutes).
const STATS_CACHE_TTL: u64 = 300;

/// Reviews service for business logic.
#[derive(Clone)]
pub struct ReviewsService {
    repository: ReviewsRepository,
    redis: redis::Client,
}

impl ReviewsService {
    /// Create a new service instance.
    pub fn new(repository: ReviewsRepository, redis: redis::Client) -> Self {
        Self { repository, redis }
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
        // Validate rating
        if !(1..=5).contains(&rating) {
            return Err(ReviewError::InvalidRating { value: rating });
        }

        // Validate content length
        if content.len() < MIN_CONTENT_LENGTH {
            return Err(ReviewError::ContentTooShort {
                min: MIN_CONTENT_LENGTH,
            });
        }
        if content.len() > MAX_CONTENT_LENGTH {
            return Err(ReviewError::ContentTooLong {
                max: MAX_CONTENT_LENGTH,
            });
        }

        // Check if user already reviewed this course
        if self.repository.user_has_reviewed(user_id, course_id).await? {
            return Err(ReviewError::AlreadyReviewed);
        }

        // Check if user is enrolled in the course
        if !self.repository.is_user_enrolled(user_id, course_id).await? {
            return Err(ReviewError::NotEnrolled);
        }

        // Check if user is the instructor (can't review own course)
        if self
            .repository
            .is_course_instructor(user_id, course_id)
            .await?
        {
            return Err(ReviewError::CannotReviewOwnCourse);
        }

        // Create the review
        let review = self
            .repository
            .create_review(user_id, course_id, rating, title, content)
            .await?;

        // Invalidate stats cache
        self.invalidate_course_stats_cache(course_id).await?;

        Ok(review)
    }

    /// Get a review by ID.
    pub async fn get_review(&self, review_id: Uuid) -> ReviewResult<Review> {
        self.repository.get_review(review_id).await
    }

    /// Update an existing review.
    pub async fn update_review(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        rating: Option<i16>,
        title: Option<String>,
        content: Option<String>,
    ) -> ReviewResult<Review> {
        // Get the existing review
        let review = self.repository.get_review(review_id).await?;

        // Check ownership
        if review.user_id != user_id {
            return Err(ReviewError::Unauthorized);
        }

        // Check if review can be modified
        if review.status == ReviewStatus::Hidden || review.status == ReviewStatus::Deleted {
            return Err(ReviewError::CannotModify);
        }

        // Validate rating if provided
        if let Some(r) = rating {
            if !(1..=5).contains(&r) {
                return Err(ReviewError::InvalidRating { value: r });
            }
        }

        // Validate content if provided
        if let Some(ref c) = content {
            if c.len() < MIN_CONTENT_LENGTH {
                return Err(ReviewError::ContentTooShort {
                    min: MIN_CONTENT_LENGTH,
                });
            }
            if c.len() > MAX_CONTENT_LENGTH {
                return Err(ReviewError::ContentTooLong {
                    max: MAX_CONTENT_LENGTH,
                });
            }
        }

        // Update the review
        let updated = self
            .repository
            .update_review(review_id, rating, title, content)
            .await?;

        // Invalidate stats cache if rating changed
        if rating.is_some() {
            self.invalidate_course_stats_cache(review.course_id).await?;
        }

        Ok(updated)
    }

    /// Delete a review.
    pub async fn delete_review(&self, review_id: Uuid, user_id: Uuid) -> ReviewResult<()> {
        // Get the existing review
        let review = self.repository.get_review(review_id).await?;

        // Check ownership
        if review.user_id != user_id {
            return Err(ReviewError::Unauthorized);
        }

        // Delete the review
        self.repository.delete_review(review_id).await?;

        // Invalidate stats cache
        self.invalidate_course_stats_cache(review.course_id).await?;

        Ok(())
    }

    /// Moderate a review (admin action).
    pub async fn moderate_review(
        &self,
        review_id: Uuid,
        _admin_id: Uuid,
        status: ReviewStatus,
        _note: Option<String>,
    ) -> ReviewResult<Review> {
        // Get the existing review (including hidden)
        let review = self.repository.get_review_admin(review_id).await?;

        // Update status
        let updated = self
            .repository
            .update_review_status(review_id, status)
            .await?;

        // Invalidate stats cache
        self.invalidate_course_stats_cache(review.course_id).await?;

        // TODO: Log moderation action, send notification to user, etc.

        Ok(updated)
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Get user's review for a specific course.
    pub async fn get_user_review_for_course(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> ReviewResult<Option<Review>> {
        self.repository
            .get_user_review_for_course(user_id, course_id)
            .await
    }

    /// Get reviews for a course.
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
        // Clamp pagination
        let page = page.max(1);
        let page_size = page_size.clamp(1, 50);

        self.repository
            .get_course_reviews(
                course_id,
                current_user_id,
                rating_filter,
                sort_by,
                sort_order,
                page,
                page_size,
            )
            .await
    }

    /// Get all reviews by a user.
    pub async fn get_user_reviews(
        &self,
        user_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> ReviewResult<PaginatedReviews> {
        let page = page.max(1);
        let page_size = page_size.clamp(1, 50);

        self.repository
            .get_user_reviews(user_id, page, page_size)
            .await
    }

    /// Get reviews for an instructor's courses.
    pub async fn get_instructor_reviews(
        &self,
        instructor_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> ReviewResult<PaginatedReviews> {
        let page = page.max(1);
        let page_size = page_size.clamp(1, 50);

        self.repository
            .get_instructor_reviews(instructor_id, page, page_size)
            .await
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get rating statistics for a course (with caching).
    pub async fn get_course_rating_stats(&self, course_id: Uuid) -> ReviewResult<CourseRatingStats> {
        let cache_key = format!("course_stats:{}", course_id);

        // Try to get from cache
        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            if let Ok(cached) = conn.get::<_, String>(&cache_key).await {
                if let Ok(stats) = serde_json::from_str(&cached) {
                    return Ok(stats);
                }
            }
        }

        // Get from database
        let stats = self.repository.get_course_rating_stats(course_id).await?;

        // Cache the result
        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            if let Ok(json) = serde_json::to_string(&stats) {
                let _: Result<(), _> = conn
                    .set_ex::<_, _, ()>(&cache_key, json, STATS_CACHE_TTL)
                    .await;
            }
        }

        Ok(stats)
    }

    /// Get rating statistics for an instructor.
    pub async fn get_instructor_rating_stats(
        &self,
        instructor_id: Uuid,
    ) -> ReviewResult<InstructorRatingStats> {
        let cache_key = format!("instructor_stats:{}", instructor_id);

        // Try to get from cache
        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            if let Ok(cached) = conn.get::<_, String>(&cache_key).await {
                if let Ok(stats) = serde_json::from_str(&cached) {
                    return Ok(stats);
                }
            }
        }

        // Get from database
        let stats = self
            .repository
            .get_instructor_rating_stats(instructor_id)
            .await?;

        // Cache the result
        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            if let Ok(json) = serde_json::to_string(&stats) {
                let _: Result<(), _> = conn
                    .set_ex::<_, _, ()>(&cache_key, json, STATS_CACHE_TTL)
                    .await;
            }
        }

        Ok(stats)
    }

    // ========================================================================
    // Helpful Votes
    // ========================================================================

    /// Mark a review as helpful or remove the mark.
    pub async fn mark_helpful(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        helpful: bool,
    ) -> ReviewResult<i32> {
        // Check review exists
        self.repository.get_review(review_id).await?;

        if helpful {
            // Check if already voted
            if self.repository.get_user_vote(review_id, user_id).await?.is_some() {
                return Err(ReviewError::AlreadyVoted);
            }
            // Add vote
            self.repository.add_vote(review_id, user_id).await?;
        } else {
            // Remove vote
            self.repository.remove_vote(review_id, user_id).await?;
        }

        // Return updated count
        self.repository.get_vote_count(review_id).await
    }

    // ========================================================================
    // Reporting
    // ========================================================================

    /// Report a review.
    pub async fn report_review(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        reason: ReportReason,
        description: Option<String>,
    ) -> ReviewResult<()> {
        // Check review exists
        self.repository.get_review(review_id).await?;

        // Check if user already reported this review
        if self
            .repository
            .user_has_reported(review_id, user_id)
            .await?
        {
            return Err(ReviewError::AlreadyVoted); // Reusing error for "already acted"
        }

        // Create report
        self.repository
            .create_report(review_id, user_id, reason, description)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Cache Management
    // ========================================================================

    /// Invalidate course stats cache.
    async fn invalidate_course_stats_cache(&self, course_id: Uuid) -> ReviewResult<()> {
        let cache_key = format!("course_stats:{}", course_id);

        if let Ok(mut conn) = self.redis.get_multiplexed_async_connection().await {
            let _: Result<(), _> = conn.del::<_, ()>(&cache_key).await;
        }

        Ok(())
    }
}
