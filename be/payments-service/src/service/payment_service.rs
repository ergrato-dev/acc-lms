//! # Payment Service
//!
//! Business logic for order processing, discount validation, and review management.

use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    DiscountCode, DiscountType, NewDiscountCode, NewOrder, NewReview, NewTransaction,
    Order, OrderEvent, OrderStatus, Review, ReviewEvent, Transaction,
    TransactionStatus, TransactionType, UpdateDiscountCode, UpdateOrder, UpdateReview,
};
use crate::repository::{OrderStats, PaymentRepository, ReviewStats};

/// Errors that can occur in the payment service.
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Order not found: {0}")]
    OrderNotFound(Uuid),

    #[error("Transaction not found: {0}")]
    TransactionNotFound(Uuid),

    #[error("Discount code not found: {0}")]
    DiscountCodeNotFound(String),

    #[error("Review not found: {0}")]
    ReviewNotFound(Uuid),

    #[error("Invalid discount code: {0}")]
    InvalidDiscountCode(String),

    #[error("Discount code expired")]
    DiscountCodeExpired,

    #[error("Discount code max uses exceeded")]
    DiscountCodeMaxUsesExceeded,

    #[error("Minimum order not met: required {required} cents, got {actual} cents")]
    MinimumOrderNotMet { required: i32, actual: i32 },

    #[error("Order cannot be modified in status: {0}")]
    OrderCannotBeModified(OrderStatus),

    #[error("Order cannot be refunded")]
    OrderCannotBeRefunded,

    #[error("Order cannot be cancelled")]
    OrderCannotBeCancelled,

    #[error("Review already exists for this course")]
    ReviewAlreadyExists,

    #[error("Invalid rating: must be between 1 and 5")]
    InvalidRating,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Payment service for business logic.
#[derive(Debug, Clone)]
pub struct PaymentService {
    repository: Arc<PaymentRepository>,
}

impl PaymentService {
    /// Creates a new payment service.
    pub fn new(repository: Arc<PaymentRepository>) -> Self {
        Self { repository }
    }

    // =========================================================================
    // ORDER OPERATIONS
    // =========================================================================

    /// Lists orders for a user.
    pub async fn list_user_orders(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, PaymentError> {
        self.repository
            .list_orders_by_user(user_id, limit, offset)
            .await
            .map_err(PaymentError::Database)
    }

    /// Lists all orders (admin only).
    pub async fn list_orders(
        &self,
        status: Option<OrderStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, PaymentError> {
        self.repository
            .list_orders(status, limit, offset)
            .await
            .map_err(PaymentError::Database)
    }

    /// Gets an order by ID.
    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, PaymentError> {
        self.repository
            .find_order_by_id(order_id)
            .await?
            .ok_or(PaymentError::OrderNotFound(order_id))
    }

    /// Gets an order by order number.
    pub async fn get_order_by_number(&self, order_number: &str) -> Result<Order, PaymentError> {
        self.repository
            .find_order_by_number(order_number)
            .await?
            .ok_or_else(|| PaymentError::OrderNotFound(Uuid::nil()))
    }

    /// Creates a new order with optional discount code validation.
    pub async fn create_order(&self, data: NewOrder) -> Result<(Order, OrderEvent), PaymentError> {
        let mut order_data = data;

        // Validate and apply discount code if provided
        if let Some(ref code) = order_data.discount_code {
            let discount = self.validate_discount_code(code, order_data.subtotal_cents).await?;
            let discount_cents = discount.calculate_discount(order_data.subtotal_cents);
            order_data.discount_cents = Some(discount_cents);
        }

        let order = self.repository.create_order(order_data).await?;

        let event = OrderEvent::Created {
            order_id: order.order_id,
            user_id: order.user_id,
            course_id: order.course_id,
            total_cents: order.total_cents,
            currency: order.currency.clone(),
            timestamp: Utc::now(),
        };

        Ok((order, event))
    }

    /// Updates an order.
    pub async fn update_order(
        &self,
        order_id: Uuid,
        data: UpdateOrder,
    ) -> Result<(Order, Option<OrderEvent>), PaymentError> {
        let existing = self.get_order(order_id).await?;

        // Check if order can be modified
        if existing.is_final() {
            return Err(PaymentError::OrderCannotBeModified(existing.status));
        }

        let order = self.repository.update_order(order_id, data.clone()).await?;

        let event = if data.status.is_some() {
            Some(OrderEvent::StatusChanged {
                order_id,
                user_id: order.user_id,
                previous_status: existing.status,
                new_status: order.status,
                timestamp: Utc::now(),
            })
        } else {
            None
        };

        Ok((order, event))
    }

    /// Processes order payment initiation.
    pub async fn initiate_payment(
        &self,
        order_id: Uuid,
        provider: &str,
        payment_intent_id: &str,
    ) -> Result<Order, PaymentError> {
        let order = self.get_order(order_id).await?;

        if order.status != OrderStatus::Pending {
            return Err(PaymentError::OrderCannotBeModified(order.status));
        }

        let update = UpdateOrder {
            status: Some(OrderStatus::Processing),
            payment_provider: Some(provider.to_string()),
            payment_intent_id: Some(payment_intent_id.to_string()),
            metadata: None,
        };

        self.repository.update_order(order_id, update).await.map_err(PaymentError::Database)
    }

    /// Completes order payment.
    pub async fn complete_payment(
        &self,
        order_id: Uuid,
        transaction_data: NewTransaction,
    ) -> Result<(Order, Transaction, OrderEvent), PaymentError> {
        let order = self.get_order(order_id).await?;

        // Create the transaction
        let transaction = self.repository.create_transaction(transaction_data).await?;

        // Update order status to paid
        let updated_order = self.repository
            .update_order_status(order_id, OrderStatus::Paid)
            .await?;

        // Increment discount code usage if applicable
        if let Some(ref code) = order.discount_code {
            let _ = self.repository.increment_discount_code_usage(code).await;
        }

        let event = OrderEvent::Paid {
            order_id,
            user_id: order.user_id,
            course_id: order.course_id,
            amount_cents: order.total_cents,
            currency: order.currency.clone(),
            transaction_id: transaction.transaction_id,
            timestamp: Utc::now(),
        };

        Ok((updated_order, transaction, event))
    }

    /// Processes a refund.
    pub async fn process_refund(
        &self,
        order_id: Uuid,
        refund_amount_cents: i32,
        provider_transaction_id: &str,
        reason: Option<String>,
    ) -> Result<(Order, Transaction, OrderEvent), PaymentError> {
        let order = self.get_order(order_id).await?;

        if !order.can_refund() {
            return Err(PaymentError::OrderCannotBeRefunded);
        }

        let transaction_data = NewTransaction {
            order_id,
            provider: order.payment_provider.clone().unwrap_or_default(),
            provider_transaction_id: provider_transaction_id.to_string(),
            transaction_type: TransactionType::Refund,
            amount_cents: refund_amount_cents,
            currency: order.currency.clone(),
            status: TransactionStatus::Succeeded.to_string(),
            provider_fee_cents: None,
            metadata: None,
        };

        let transaction = self.repository.create_transaction(transaction_data).await?;

        let updated_order = self.repository
            .update_order_status(order_id, OrderStatus::Refunded)
            .await?;

        let event = OrderEvent::Refunded {
            order_id,
            user_id: order.user_id,
            course_id: order.course_id,
            amount_cents: refund_amount_cents,
            reason,
            timestamp: Utc::now(),
        };

        Ok((updated_order, transaction, event))
    }

    /// Cancels an order.
    pub async fn cancel_order(&self, order_id: Uuid, reason: Option<String>) -> Result<(Order, OrderEvent), PaymentError> {
        let order = self.get_order(order_id).await?;

        if !order.can_cancel() {
            return Err(PaymentError::OrderCannotBeCancelled);
        }

        let updated_order = self.repository
            .update_order_status(order_id, OrderStatus::Cancelled)
            .await?;

        let event = OrderEvent::Cancelled {
            order_id,
            user_id: order.user_id,
            reason,
            timestamp: Utc::now(),
        };

        Ok((updated_order, event))
    }

    // =========================================================================
    // DISCOUNT CODE OPERATIONS
    // =========================================================================

    /// Validates a discount code for use.
    pub async fn validate_discount_code(
        &self,
        code: &str,
        subtotal_cents: i32,
    ) -> Result<DiscountCode, PaymentError> {
        let discount = self.repository
            .find_discount_code(code)
            .await?
            .ok_or_else(|| PaymentError::DiscountCodeNotFound(code.to_string()))?;

        if !discount.is_active {
            return Err(PaymentError::InvalidDiscountCode("Code is not active".to_string()));
        }

        if !discount.is_valid() {
            return Err(PaymentError::DiscountCodeExpired);
        }

        if let Some(min) = discount.minimum_order_cents {
            if subtotal_cents < min {
                return Err(PaymentError::MinimumOrderNotMet {
                    required: min,
                    actual: subtotal_cents,
                });
            }
        }

        Ok(discount)
    }

    /// Lists discount codes.
    pub async fn list_discount_codes(
        &self,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DiscountCode>, PaymentError> {
        self.repository
            .list_discount_codes(active_only, limit, offset)
            .await
            .map_err(PaymentError::Database)
    }

    /// Creates a discount code.
    pub async fn create_discount_code(
        &self,
        data: NewDiscountCode,
    ) -> Result<DiscountCode, PaymentError> {
        // Validate discount value
        if let DiscountType::Percentage = data.discount_type {
            if data.discount_value < Decimal::ZERO || data.discount_value > Decimal::from(100) {
                return Err(PaymentError::InvalidDiscountCode(
                    "Percentage must be between 0 and 100".to_string(),
                ));
            }
        }

        self.repository
            .create_discount_code(data)
            .await
            .map_err(PaymentError::Database)
    }

    /// Updates a discount code.
    pub async fn update_discount_code(
        &self,
        code_id: Uuid,
        data: UpdateDiscountCode,
    ) -> Result<DiscountCode, PaymentError> {
        self.repository
            .update_discount_code(code_id, data)
            .await
            .map_err(PaymentError::Database)
    }

    /// Gets a discount code by code string.
    pub async fn get_discount_code(&self, code: &str) -> Result<DiscountCode, PaymentError> {
        self.repository
            .find_discount_code(code)
            .await?
            .ok_or_else(|| PaymentError::DiscountCodeNotFound(code.to_string()))
    }

    // =========================================================================
    // REVIEW OPERATIONS
    // =========================================================================

    /// Lists reviews for a course.
    pub async fn list_course_reviews(
        &self,
        course_id: Uuid,
        public_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, PaymentError> {
        self.repository
            .list_reviews_by_course(course_id, public_only, limit, offset)
            .await
            .map_err(PaymentError::Database)
    }

    /// Gets a review by ID.
    pub async fn get_review(&self, review_id: Uuid) -> Result<Review, PaymentError> {
        self.repository
            .find_review_by_id(review_id)
            .await?
            .ok_or(PaymentError::ReviewNotFound(review_id))
    }

    /// Creates a review.
    pub async fn create_review(
        &self,
        data: NewReview,
    ) -> Result<(Review, ReviewEvent), PaymentError> {
        // Validate rating
        if data.rating < 1 || data.rating > 5 {
            return Err(PaymentError::InvalidRating);
        }

        // Check if user already reviewed this course
        let existing = self.repository
            .find_user_review(data.course_id, data.user_id)
            .await?;

        if existing.is_some() {
            return Err(PaymentError::ReviewAlreadyExists);
        }

        let review = self.repository.create_review(data).await?;

        let event = ReviewEvent::Created {
            review_id: review.review_id,
            course_id: review.course_id,
            user_id: review.user_id,
            rating: review.rating,
            timestamp: Utc::now(),
        };

        Ok((review, event))
    }

    /// Updates a review.
    pub async fn update_review(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        data: UpdateReview,
    ) -> Result<(Review, ReviewEvent), PaymentError> {
        let existing = self.get_review(review_id).await?;

        // Verify ownership
        if existing.user_id != user_id {
            return Err(PaymentError::Unauthorized);
        }

        // Validate rating if provided
        if let Some(rating) = data.rating {
            if rating < 1 || rating > 5 {
                return Err(PaymentError::InvalidRating);
            }
        }

        let review = self.repository.update_review(review_id, data).await?;

        let event = ReviewEvent::Updated {
            review_id,
            course_id: review.course_id,
            previous_rating: existing.rating,
            new_rating: review.rating,
            timestamp: Utc::now(),
        };

        Ok((review, event))
    }

    /// Deletes a review.
    pub async fn delete_review(
        &self,
        review_id: Uuid,
        user_id: Uuid,
        is_admin: bool,
    ) -> Result<ReviewEvent, PaymentError> {
        let review = self.get_review(review_id).await?;

        // Verify ownership or admin
        if review.user_id != user_id && !is_admin {
            return Err(PaymentError::Unauthorized);
        }

        self.repository.delete_review(review_id).await?;

        Ok(ReviewEvent::Deleted {
            review_id,
            course_id: review.course_id,
            user_id: review.user_id,
            timestamp: Utc::now(),
        })
    }

    /// Votes a review as helpful.
    pub async fn vote_helpful(&self, review_id: Uuid, voter_id: Uuid) -> Result<(Review, ReviewEvent), PaymentError> {
        let review = self.repository.increment_helpful_votes(review_id).await?;

        let event = ReviewEvent::HelpfulVote {
            review_id,
            voter_id,
            timestamp: Utc::now(),
        };

        Ok((review, event))
    }

    /// Gets review statistics for a course.
    pub async fn get_review_stats(&self, course_id: Uuid) -> Result<ReviewStats, PaymentError> {
        self.repository
            .get_review_stats(course_id)
            .await
            .map_err(PaymentError::Database)
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Gets order statistics.
    pub async fn get_order_stats(&self) -> Result<OrderStats, PaymentError> {
        self.repository
            .get_order_stats()
            .await
            .map_err(PaymentError::Database)
    }
}
