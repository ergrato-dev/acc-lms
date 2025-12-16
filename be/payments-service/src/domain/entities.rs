//! # Payments Domain Entities
//!
//! Core domain entities for the payments service.
//!
//! ## Entity Hierarchy
//!
//! ```text
//! Order (aggregate root)
//!     └── Transaction
//!
//! DiscountCode (standalone)
//! Review (standalone)
//! ```

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Order status enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// Order created, awaiting payment
    Pending,
    /// Payment is being processed
    Processing,
    /// Payment successful
    Paid,
    /// Payment failed
    Failed,
    /// Order cancelled by user or system
    Cancelled,
    /// Order refunded
    Refunded,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Pending
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Paid => write!(f, "paid"),
            OrderStatus::Failed => write!(f, "failed"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Refunded => write!(f, "refunded"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(OrderStatus::Pending),
            "processing" => Ok(OrderStatus::Processing),
            "paid" => Ok(OrderStatus::Paid),
            "failed" => Ok(OrderStatus::Failed),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "refunded" => Ok(OrderStatus::Refunded),
            _ => Err(format!("Invalid order status: {}", s)),
        }
    }
}

/// Transaction type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    /// Payment transaction
    Payment,
    /// Refund transaction
    Refund,
    /// Chargeback transaction
    Chargeback,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Payment => write!(f, "payment"),
            TransactionType::Refund => write!(f, "refund"),
            TransactionType::Chargeback => write!(f, "chargeback"),
        }
    }
}

/// Transaction status enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    /// Transaction pending
    Pending,
    /// Transaction succeeded
    Succeeded,
    /// Transaction failed
    Failed,
    /// Transaction refunded
    Refunded,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "pending"),
            TransactionStatus::Succeeded => write!(f, "succeeded"),
            TransactionStatus::Failed => write!(f, "failed"),
            TransactionStatus::Refunded => write!(f, "refunded"),
        }
    }
}

/// Discount type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    /// Percentage discount (e.g., 20% off)
    Percentage,
    /// Fixed amount discount (e.g., $10 off)
    FixedAmount,
}

impl std::fmt::Display for DiscountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscountType::Percentage => write!(f, "percentage"),
            DiscountType::FixedAmount => write!(f, "fixed_amount"),
        }
    }
}

// =============================================================================
// ORDER
// =============================================================================

/// Purchase order entity.
///
/// # Database Mapping
///
/// Maps to `payments.orders` table.
///
/// # Cross-Schema References
///
/// - `user_id`: References `auth.users(user_id)`
/// - `course_id`: References `courses.courses(course_id)`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    /// Unique identifier
    pub order_id: Uuid,
    /// Customer placing the order
    pub user_id: Uuid,
    /// Course being purchased
    pub course_id: Uuid,
    /// Human-readable order number (e.g., ORD-2024-000001)
    pub order_number: String,
    /// Current order status
    pub status: OrderStatus,
    /// Subtotal before tax and discounts (in cents)
    pub subtotal_cents: i32,
    /// Tax amount (in cents)
    pub tax_cents: i32,
    /// Discount amount (in cents)
    pub discount_cents: i32,
    /// Final total (in cents)
    pub total_cents: i32,
    /// Currency code (e.g., "USD")
    pub currency: String,
    /// Payment provider (e.g., "stripe")
    pub payment_provider: Option<String>,
    /// External payment intent ID
    pub payment_intent_id: Option<String>,
    /// Applied discount code
    pub discount_code: Option<String>,
    /// Additional metadata as JSON
    pub metadata: serde_json::Value,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Order {
    /// Returns true if the order is in a final state.
    pub fn is_final(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Paid | OrderStatus::Failed | OrderStatus::Cancelled | OrderStatus::Refunded
        )
    }

    /// Returns true if the order can be refunded.
    pub fn can_refund(&self) -> bool {
        self.status == OrderStatus::Paid
    }

    /// Returns true if the order can be cancelled.
    pub fn can_cancel(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::Processing)
    }

    /// Returns the total as a formatted string (e.g., "$19.99").
    pub fn formatted_total(&self) -> String {
        let dollars = self.total_cents as f64 / 100.0;
        format!("${:.2}", dollars)
    }
}

/// Data required to create a new order.
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrder {
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub subtotal_cents: i32,
    pub tax_cents: Option<i32>,
    pub discount_cents: Option<i32>,
    pub currency: Option<String>,
    pub discount_code: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Data for updating an order.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateOrder {
    pub status: Option<OrderStatus>,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// =============================================================================
// TRANSACTION
// =============================================================================

/// Payment transaction entity.
///
/// # Database Mapping
///
/// Maps to `payments.transactions` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    /// Unique identifier
    pub transaction_id: Uuid,
    /// Parent order
    pub order_id: Uuid,
    /// Payment provider (e.g., "stripe", "paypal")
    pub provider: String,
    /// External transaction ID from provider
    pub provider_transaction_id: String,
    /// Type of transaction
    pub transaction_type: TransactionType,
    /// Transaction amount (in cents)
    pub amount_cents: i32,
    /// Currency code
    pub currency: String,
    /// Transaction status
    pub status: String,
    /// Provider fee (in cents)
    pub provider_fee_cents: Option<i32>,
    /// Additional metadata
    pub metadata: serde_json::Value,
    /// When the transaction was processed
    pub processed_at: DateTime<Utc>,
}

/// Data required to create a new transaction.
#[derive(Debug, Clone, Deserialize)]
pub struct NewTransaction {
    pub order_id: Uuid,
    pub provider: String,
    pub provider_transaction_id: String,
    pub transaction_type: TransactionType,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub provider_fee_cents: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

// =============================================================================
// DISCOUNT CODE
// =============================================================================

/// Discount code entity.
///
/// # Database Mapping
///
/// Maps to `payments.discount_codes` table.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DiscountCode {
    /// Unique identifier
    pub code_id: Uuid,
    /// The discount code string (e.g., "SUMMER20")
    pub code: String,
    /// Description of the discount
    pub description: Option<String>,
    /// Type of discount
    pub discount_type: DiscountType,
    /// Discount value (percentage or fixed amount)
    pub discount_value: Decimal,
    /// Minimum order amount required (in cents)
    pub minimum_order_cents: Option<i32>,
    /// Maximum number of uses allowed
    pub max_uses: Option<i32>,
    /// Current number of uses
    pub current_uses: i32,
    /// When the code becomes valid
    pub valid_from: DateTime<Utc>,
    /// When the code expires
    pub valid_until: Option<DateTime<Utc>>,
    /// Whether the code is active
    pub is_active: bool,
    /// Who created this code
    pub created_by: Uuid,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
}

impl DiscountCode {
    /// Returns true if the code is currently valid.
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        let now = Utc::now();
        if now < self.valid_from {
            return false;
        }

        if let Some(until) = self.valid_until {
            if now > until {
                return false;
            }
        }

        if let Some(max) = self.max_uses {
            if self.current_uses >= max {
                return false;
            }
        }

        true
    }

    /// Calculates the discount amount for a given subtotal.
    pub fn calculate_discount(&self, subtotal_cents: i32) -> i32 {
        if let Some(min) = self.minimum_order_cents {
            if subtotal_cents < min {
                return 0;
            }
        }

        match self.discount_type {
            DiscountType::Percentage => {
                let discount = Decimal::from(subtotal_cents) * (self.discount_value / Decimal::from(100));
                discount.round().to_string().parse::<i32>().unwrap_or(0)
            }
            DiscountType::FixedAmount => {
                // discount_value is in dollars, convert to cents
                let discount_cents = (self.discount_value * Decimal::from(100)).round();
                discount_cents.to_string().parse::<i32>().unwrap_or(0).min(subtotal_cents)
            }
        }
    }
}

/// Data required to create a new discount code.
#[derive(Debug, Clone, Deserialize)]
pub struct NewDiscountCode {
    pub code: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: Decimal,
    pub minimum_order_cents: Option<i32>,
    pub max_uses: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}

/// Data for updating a discount code.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateDiscountCode {
    pub description: Option<Option<String>>,
    pub discount_type: Option<DiscountType>,
    pub discount_value: Option<Decimal>,
    pub minimum_order_cents: Option<Option<i32>>,
    pub max_uses: Option<Option<i32>>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<Option<DateTime<Utc>>>,
    pub is_active: Option<bool>,
}

// =============================================================================
// REVIEW
// =============================================================================

/// Course review entity.
///
/// # Database Mapping
///
/// Maps to `payments.reviews` table.
///
/// # Cross-Schema References
///
/// - `course_id`: References `courses.courses(course_id)`
/// - `user_id`: References `auth.users(user_id)`
/// - `enrollment_id`: References `enrollments.enrollments(enrollment_id)`
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    /// Unique identifier
    pub review_id: Uuid,
    /// Course being reviewed
    pub course_id: Uuid,
    /// User who wrote the review
    pub user_id: Uuid,
    /// Enrollment that verified the purchase
    pub enrollment_id: Uuid,
    /// Rating (1-5 stars)
    pub rating: i32,
    /// Review title
    pub review_title: Option<String>,
    /// Review body text
    pub review_text: Option<String>,
    /// Whether the review is publicly visible
    pub is_public: bool,
    /// Whether this is a verified purchase
    pub is_verified_purchase: bool,
    /// Number of helpful votes
    pub helpful_votes: i32,
    /// Record creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Data required to create a new review.
#[derive(Debug, Clone, Deserialize)]
pub struct NewReview {
    pub course_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub rating: i32,
    pub review_title: Option<String>,
    pub review_text: Option<String>,
    pub is_public: Option<bool>,
}

/// Data for updating a review.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateReview {
    pub rating: Option<i32>,
    pub review_title: Option<Option<String>>,
    pub review_text: Option<Option<String>>,
    pub is_public: Option<bool>,
}

// =============================================================================
// AGGREGATES
// =============================================================================

/// Order with all transactions loaded.
#[derive(Debug, Clone, Serialize)]
pub struct OrderWithTransactions {
    /// The order entity
    #[serde(flatten)]
    pub order: Order,
    /// All transactions for this order
    pub transactions: Vec<Transaction>,
}

impl OrderWithTransactions {
    /// Returns the total amount paid across all successful transactions.
    pub fn total_paid(&self) -> i32 {
        self.transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Payment && t.status == "succeeded")
            .map(|t| t.amount_cents)
            .sum()
    }

    /// Returns the total amount refunded.
    pub fn total_refunded(&self) -> i32 {
        self.transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Refund && t.status == "succeeded")
            .map(|t| t.amount_cents)
            .sum()
    }
}

/// Course review statistics.
#[derive(Debug, Clone, Serialize)]
pub struct CourseReviewStats {
    pub course_id: Uuid,
    pub total_reviews: i64,
    pub average_rating: f64,
    pub rating_distribution: [i64; 5], // [1-star, 2-star, 3-star, 4-star, 5-star]
}
