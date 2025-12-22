//! # Domain Entities
//!
//! Core business entities for reviews functionality.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// REVIEW
// =============================================================================

/// Course review with rating and comment.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub review_id: Uuid,
    pub course_id: Uuid,
    pub user_id: Uuid,
    /// Star rating 1-5
    pub rating: i16,
    /// Review title (optional)
    pub title: Option<String>,
    /// Review content/comment
    pub content: Option<String>,
    /// Number of helpful votes
    #[sqlx(default)]
    pub helpful_count: i32,
    /// Review status
    pub status: ReviewStatus,
    /// Instructor response (if any)
    pub instructor_response: Option<String>,
    pub instructor_response_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Review with additional user info for display.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReviewWithUser {
    pub review_id: Uuid,
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_avatar_url: Option<String>,
    pub rating: i16,
    pub title: Option<String>,
    pub content: Option<String>,
    pub helpful_count: i32,
    pub status: String,
    pub instructor_response: Option<String>,
    pub instructor_response_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Whether current user found this helpful
    #[sqlx(default)]
    pub user_found_helpful: bool,
}

/// Review status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "review_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Active and visible
    Published,
    /// Under moderation review
    PendingModeration,
    /// Hidden by moderation
    Hidden,
    /// Deleted by user
    Deleted,
}

impl Default for ReviewStatus {
    fn default() -> Self {
        ReviewStatus::Published
    }
}

// =============================================================================
// HELPFUL VOTE
// =============================================================================

/// Record of a user marking a review as helpful.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HelpfulVote {
    pub vote_id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// REVIEW REPORT
// =============================================================================

/// Report of an inappropriate review.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReviewReport {
    pub report_id: Uuid,
    pub review_id: Uuid,
    pub reporter_id: Uuid,
    pub reason: ReportReason,
    pub description: Option<String>,
    pub status: ReportStatus,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Reason for reporting a review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_reason", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReportReason {
    Spam,
    Inappropriate,
    Offensive,
    FakeReview,
    Harassment,
    Other,
}

/// Report resolution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Pending,
    Reviewed,
    ActionTaken,
    Dismissed,
}

impl Default for ReportStatus {
    fn default() -> Self {
        ReportStatus::Pending
    }
}

// =============================================================================
// AGGREGATES
// =============================================================================

/// Course rating statistics.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseRatingStats {
    pub course_id: Uuid,
    pub average_rating: Decimal,
    pub total_reviews: i64,
    pub rating_5_count: i64,
    pub rating_4_count: i64,
    pub rating_3_count: i64,
    pub rating_2_count: i64,
    pub rating_1_count: i64,
}

impl CourseRatingStats {
    /// Calculate rating distribution percentages.
    pub fn distribution(&self) -> RatingDistribution {
        let total = self.total_reviews as f64;
        if total == 0.0 {
            return RatingDistribution::default();
        }
        RatingDistribution {
            five_star: (self.rating_5_count as f64 / total * 100.0) as u8,
            four_star: (self.rating_4_count as f64 / total * 100.0) as u8,
            three_star: (self.rating_3_count as f64 / total * 100.0) as u8,
            two_star: (self.rating_2_count as f64 / total * 100.0) as u8,
            one_star: (self.rating_1_count as f64 / total * 100.0) as u8,
        }
    }
}

/// Rating distribution percentages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RatingDistribution {
    pub five_star: u8,
    pub four_star: u8,
    pub three_star: u8,
    pub two_star: u8,
    pub one_star: u8,
}

/// Instructor rating statistics.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InstructorRatingStats {
    pub instructor_id: Uuid,
    pub average_rating: Decimal,
    pub total_reviews: i64,
    pub total_courses: i64,
}

// =============================================================================
// PAGINATION
// =============================================================================

/// Paginated reviews result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedReviews {
    pub reviews: Vec<ReviewWithUser>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl PaginatedReviews {
    pub fn new(reviews: Vec<ReviewWithUser>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            reviews,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
