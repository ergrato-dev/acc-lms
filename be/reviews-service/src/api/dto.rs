//! # API Data Transfer Objects
//!
//! Request and response DTOs for the reviews API.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::{
    CourseRatingStats, InstructorRatingStats, Review, ReviewStatus,
    ReviewWithUser,
};

// ============================================================================
// Request DTOs
// ============================================================================

/// Create review request.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewRequest {
    /// Course ID being reviewed
    pub course_id: Uuid,

    /// Rating from 1 to 5
    #[validate(range(min = 1, max = 5))]
    pub rating: i16,

    /// Review title (optional)
    #[validate(length(max = 200))]
    pub title: Option<String>,

    /// Review content
    #[validate(length(min = 10, max = 5000))]
    pub content: String,
}

/// Update review request.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateReviewRequest {
    /// New rating from 1 to 5
    #[validate(range(min = 1, max = 5))]
    pub rating: Option<i16>,

    /// New title
    #[validate(length(max = 200))]
    pub title: Option<String>,

    /// New content
    #[validate(length(min = 10, max = 5000))]
    pub content: Option<String>,
}

/// Report review request.
#[derive(Debug, Deserialize, Validate)]
pub struct ReportReviewRequest {
    /// Reason for reporting
    pub reason: ReportReasonDto,

    /// Additional details
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

/// Report reason DTO.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportReasonDto {
    Spam,
    Inappropriate,
    Offensive,
    FakeReview,
    Harassment,
    Other,
}

impl From<ReportReasonDto> for crate::domain::entities::ReportReason {
    fn from(dto: ReportReasonDto) -> Self {
        match dto {
            ReportReasonDto::Spam => Self::Spam,
            ReportReasonDto::Inappropriate => Self::Inappropriate,
            ReportReasonDto::Offensive => Self::Offensive,
            ReportReasonDto::FakeReview => Self::FakeReview,
            ReportReasonDto::Harassment => Self::Harassment,
            ReportReasonDto::Other => Self::Other,
        }
    }
}

/// Mark review as helpful request.
#[derive(Debug, Deserialize)]
pub struct MarkHelpfulRequest {
    /// Whether to mark as helpful (true) or remove the mark (false)
    pub helpful: bool,
}

/// Query parameters for listing reviews.
#[derive(Debug, Deserialize, Default)]
pub struct ListReviewsQuery {
    /// Filter by rating (1-5)
    pub rating: Option<i16>,

    /// Sort field
    #[serde(default)]
    pub sort_by: ReviewSortBy,

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

/// Review sort options.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSortBy {
    #[default]
    MostRecent,
    MostHelpful,
    HighestRated,
    LowestRated,
}

/// Sort order.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

/// Query for instructor reviews.
#[derive(Debug, Deserialize, Default)]
pub struct InstructorReviewsQuery {
    /// Page number
    #[serde(default = "default_page")]
    pub page: i32,

    /// Items per page
    #[serde(default = "default_page_size")]
    pub per_page: i32,
}

/// Moderate review request (admin only).
#[derive(Debug, Deserialize)]
pub struct ModerateReviewRequest {
    /// New status
    pub status: ReviewStatusDto,

    /// Moderation note
    pub note: Option<String>,
}

/// Review status DTO.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatusDto {
    Published,
    PendingModeration,
    Hidden,
    Deleted,
}

impl From<ReviewStatusDto> for ReviewStatus {
    fn from(dto: ReviewStatusDto) -> Self {
        match dto {
            ReviewStatusDto::Published => Self::Published,
            ReviewStatusDto::PendingModeration => Self::PendingModeration,
            ReviewStatusDto::Hidden => Self::Hidden,
            ReviewStatusDto::Deleted => Self::Deleted,
        }
    }
}

impl From<ReviewStatus> for ReviewStatusDto {
    fn from(status: ReviewStatus) -> Self {
        match status {
            ReviewStatus::Published => Self::Published,
            ReviewStatus::PendingModeration => Self::PendingModeration,
            ReviewStatus::Hidden => Self::Hidden,
            ReviewStatus::Deleted => Self::Deleted,
        }
    }
}

// ============================================================================
// Response DTOs
// ============================================================================

/// Review response.
#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub review_id: Uuid,
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub rating: i16,
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: ReviewStatusDto,
    pub helpful_count: i32,
    pub instructor_response: Option<String>,
    pub instructor_response_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Review> for ReviewResponse {
    fn from(review: Review) -> Self {
        Self {
            review_id: review.review_id,
            course_id: review.course_id,
            user_id: review.user_id,
            rating: review.rating,
            title: review.title,
            content: review.content,
            status: review.status.into(),
            helpful_count: review.helpful_count,
            instructor_response: review.instructor_response,
            instructor_response_at: review.instructor_response_at,
            created_at: review.created_at,
            updated_at: review.updated_at,
        }
    }
}

/// Review with user info response.
#[derive(Debug, Serialize)]
pub struct ReviewWithUserResponse {
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
    pub instructor_response_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether current user found this helpful
    pub user_found_helpful: bool,
}

impl From<ReviewWithUser> for ReviewWithUserResponse {
    fn from(review: ReviewWithUser) -> Self {
        Self {
            review_id: review.review_id,
            course_id: review.course_id,
            user_id: review.user_id,
            user_name: review.user_name,
            user_avatar_url: review.user_avatar_url,
            rating: review.rating,
            title: review.title,
            content: review.content,
            helpful_count: review.helpful_count,
            status: review.status,
            instructor_response: review.instructor_response,
            instructor_response_at: review.instructor_response_at,
            created_at: review.created_at,
            updated_at: review.updated_at,
            user_found_helpful: review.user_found_helpful,
        }
    }
}

/// Paginated reviews response.
#[derive(Debug, Serialize)]
pub struct PaginatedReviewsResponse {
    pub reviews: Vec<ReviewWithUserResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Course rating stats response.
#[derive(Debug, Serialize)]
pub struct CourseRatingStatsResponse {
    pub course_id: Uuid,
    pub average_rating: Decimal,
    pub total_reviews: i64,
    pub distribution: RatingDistributionResponse,
}

impl From<CourseRatingStats> for CourseRatingStatsResponse {
    fn from(stats: CourseRatingStats) -> Self {
        Self {
            course_id: stats.course_id,
            average_rating: stats.average_rating,
            total_reviews: stats.total_reviews,
            distribution: RatingDistributionResponse {
                five_star: stats.rating_5_count,
                four_star: stats.rating_4_count,
                three_star: stats.rating_3_count,
                two_star: stats.rating_2_count,
                one_star: stats.rating_1_count,
            },
        }
    }
}

/// Rating distribution response.
#[derive(Debug, Serialize)]
pub struct RatingDistributionResponse {
    pub five_star: i64,
    pub four_star: i64,
    pub three_star: i64,
    pub two_star: i64,
    pub one_star: i64,
}

/// Instructor rating stats response.
#[derive(Debug, Serialize)]
pub struct InstructorRatingStatsResponse {
    pub instructor_id: Uuid,
    pub average_rating: Decimal,
    pub total_reviews: i64,
    pub total_courses: i64,
}

impl From<InstructorRatingStats> for InstructorRatingStatsResponse {
    fn from(stats: InstructorRatingStats) -> Self {
        Self {
            instructor_id: stats.instructor_id,
            average_rating: stats.average_rating,
            total_reviews: stats.total_reviews,
            total_courses: stats.total_courses,
        }
    }
}

/// User's review for a course response.
#[derive(Debug, Serialize)]
pub struct UserCourseReviewResponse {
    pub has_reviewed: bool,
    pub review: Option<ReviewResponse>,
}

/// Helpful count response.
#[derive(Debug, Serialize)]
pub struct HelpfulCountResponse {
    pub review_id: Uuid,
    pub helpful_count: i32,
}

/// Success message response.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
