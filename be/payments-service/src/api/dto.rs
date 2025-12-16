//! # DTOs
//!
//! Request and response data transfer objects for the payments API.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{DiscountCode, Order, Review, Transaction};
use crate::repository::{OrderStats, ReviewStats};

// =============================================================================
// ORDER DTOs
// =============================================================================

/// Request to create a new order.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub course_id: Uuid,
    #[validate(range(min = 0))]
    pub subtotal_cents: i32,
    pub tax_cents: Option<i32>,
    pub discount_code: Option<String>,
    pub currency: Option<String>,
}

/// Request to update an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateOrderRequest {
    pub status: Option<String>,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Request to initiate payment.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InitiatePaymentRequest {
    #[validate(length(min = 1))]
    pub provider: String,
    #[validate(length(min = 1))]
    pub payment_intent_id: String,
}

/// Request to complete payment.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CompletePaymentRequest {
    #[validate(length(min = 1))]
    pub provider: String,
    #[validate(length(min = 1))]
    pub provider_transaction_id: String,
    #[validate(range(min = 0))]
    pub amount_cents: i32,
    pub currency: String,
    pub provider_fee_cents: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

/// Request to process a refund.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RefundRequest {
    #[validate(range(min = 1))]
    pub amount_cents: i32,
    #[validate(length(min = 1))]
    pub provider_transaction_id: String,
    pub reason: Option<String>,
}

/// Request to cancel an order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelOrderRequest {
    pub reason: Option<String>,
}

/// Order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub order_number: String,
    pub status: String,
    pub subtotal_cents: i32,
    pub tax_cents: i32,
    pub discount_cents: i32,
    pub total_cents: i32,
    pub currency: String,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub discount_code: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Order> for OrderResponse {
    fn from(o: Order) -> Self {
        Self {
            order_id: o.order_id,
            user_id: o.user_id,
            course_id: o.course_id,
            order_number: o.order_number,
            status: o.status.to_string(),
            subtotal_cents: o.subtotal_cents,
            tax_cents: o.tax_cents,
            discount_cents: o.discount_cents,
            total_cents: o.total_cents,
            currency: o.currency,
            payment_provider: o.payment_provider,
            payment_intent_id: o.payment_intent_id,
            discount_code: o.discount_code,
            metadata: o.metadata,
            created_at: o.created_at,
            updated_at: o.updated_at,
        }
    }
}

/// Order list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub orders: Vec<OrderResponse>,
    pub total: usize,
}

// =============================================================================
// TRANSACTION DTOs
// =============================================================================

/// Transaction response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub transaction_id: Uuid,
    pub order_id: Uuid,
    pub provider: String,
    pub provider_transaction_id: String,
    pub transaction_type: String,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub provider_fee_cents: Option<i32>,
    pub metadata: serde_json::Value,
    pub processed_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(t: Transaction) -> Self {
        Self {
            transaction_id: t.transaction_id,
            order_id: t.order_id,
            provider: t.provider,
            provider_transaction_id: t.provider_transaction_id,
            transaction_type: t.transaction_type.to_string(),
            amount_cents: t.amount_cents,
            currency: t.currency,
            status: t.status,
            provider_fee_cents: t.provider_fee_cents,
            metadata: t.metadata,
            processed_at: t.processed_at,
        }
    }
}

// =============================================================================
// DISCOUNT CODE DTOs
// =============================================================================

/// Request to create a discount code.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDiscountCodeRequest {
    #[validate(length(min = 3, max = 50))]
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub minimum_order_cents: Option<i32>,
    pub max_uses: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}

/// Request to update a discount code.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDiscountCodeRequest {
    pub description: Option<String>,
    pub discount_type: Option<String>,
    pub discount_value: Option<Decimal>,
    pub minimum_order_cents: Option<i32>,
    pub max_uses: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

/// Request to validate a discount code.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ValidateDiscountCodeRequest {
    #[validate(length(min = 1))]
    pub code: String,
    #[validate(range(min = 0))]
    pub subtotal_cents: i32,
}

/// Discount code response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCodeResponse {
    pub code_id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub minimum_order_cents: Option<i32>,
    pub max_uses: Option<i32>,
    pub current_uses: i32,
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<DiscountCode> for DiscountCodeResponse {
    fn from(d: DiscountCode) -> Self {
        Self {
            code_id: d.code_id,
            code: d.code,
            description: d.description,
            discount_type: d.discount_type.to_string(),
            discount_value: d.discount_value,
            minimum_order_cents: d.minimum_order_cents,
            max_uses: d.max_uses,
            current_uses: d.current_uses,
            valid_from: d.valid_from,
            valid_until: d.valid_until,
            is_active: d.is_active,
            created_by: d.created_by,
            created_at: d.created_at,
        }
    }
}

/// Discount validation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountValidationResponse {
    pub valid: bool,
    pub code: String,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub discount_amount_cents: i32,
    pub message: Option<String>,
}

// =============================================================================
// REVIEW DTOs
// =============================================================================

/// Request to create a review.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateReviewRequest {
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    #[validate(range(min = 1, max = 5))]
    pub rating: i32,
    #[validate(length(max = 200))]
    pub review_title: Option<String>,
    #[validate(length(max = 5000))]
    pub review_text: Option<String>,
    pub is_public: Option<bool>,
}

/// Request to update a review.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct UpdateReviewRequest {
    #[validate(range(min = 1, max = 5))]
    pub rating: Option<i32>,
    #[validate(length(max = 200))]
    pub review_title: Option<String>,
    #[validate(length(max = 5000))]
    pub review_text: Option<String>,
    pub is_public: Option<bool>,
}

/// Review response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub review_id: Uuid,
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub rating: i32,
    pub review_title: Option<String>,
    pub review_text: Option<String>,
    pub is_public: bool,
    pub is_verified_purchase: bool,
    pub helpful_votes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Review> for ReviewResponse {
    fn from(r: Review) -> Self {
        Self {
            review_id: r.review_id,
            course_id: r.course_id,
            user_id: r.user_id,
            enrollment_id: r.enrollment_id,
            rating: r.rating,
            review_title: r.review_title,
            review_text: r.review_text,
            is_public: r.is_public,
            is_verified_purchase: r.is_verified_purchase,
            helpful_votes: r.helpful_votes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// Review list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewListResponse {
    pub reviews: Vec<ReviewResponse>,
    pub total: usize,
}

// =============================================================================
// STATISTICS DTOs
// =============================================================================

/// Order statistics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatsResponse {
    pub total_orders: i64,
    pub paid_orders: i64,
    pub total_revenue_cents: i64,
    pub avg_order_value_cents: f64,
}

impl From<OrderStats> for OrderStatsResponse {
    fn from(s: OrderStats) -> Self {
        Self {
            total_orders: s.total_orders,
            paid_orders: s.paid_orders,
            total_revenue_cents: s.total_revenue_cents,
            avg_order_value_cents: s.avg_order_value_cents,
        }
    }
}

/// Review statistics response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStatsResponse {
    pub total_reviews: i64,
    pub average_rating: f64,
    pub rating_distribution: RatingDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingDistribution {
    pub one_star: i64,
    pub two_star: i64,
    pub three_star: i64,
    pub four_star: i64,
    pub five_star: i64,
}

impl From<ReviewStats> for ReviewStatsResponse {
    fn from(s: ReviewStats) -> Self {
        Self {
            total_reviews: s.total_reviews,
            average_rating: s.average_rating,
            rating_distribution: RatingDistribution {
                one_star: s.one_star,
                two_star: s.two_star,
                three_star: s.three_star,
                four_star: s.four_star,
                five_star: s.five_star,
            },
        }
    }
}

// =============================================================================
// COMMON DTOs
// =============================================================================

/// Query parameters for pagination.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PaginationQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
}

/// Query parameters for order listing.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OrderQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for review listing.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ReviewQuery {
    pub public_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for discount code listing.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DiscountCodeQuery {
    pub active_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
