//! # Payments Repository
//!
//! PostgreSQL data access for orders, transactions, discount codes, and reviews.
//!
//! ## Schema
//!
//! Uses the `payments` schema with tables:
//! - `payments.orders`
//! - `payments.transactions`
//! - `payments.discount_codes`
//! - `payments.reviews`

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    DiscountCode, NewDiscountCode, NewOrder, NewReview, NewTransaction,
    Order, OrderStatus, Review, Transaction, UpdateDiscountCode, UpdateOrder, UpdateReview,
};

/// Repository for payments data access.
#[derive(Debug, Clone)]
pub struct PaymentRepository {
    pool: PgPool,
}

impl PaymentRepository {
    /// Creates a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // ORDER OPERATIONS
    // =========================================================================

    /// Lists orders for a user.
    pub async fn list_orders_by_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            FROM payments.orders
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Lists all orders with optional status filter.
    pub async fn list_orders(
        &self,
        status: Option<OrderStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>, sqlx::Error> {
        if let Some(status) = status {
            sqlx::query_as::<_, Order>(
                r#"
                SELECT
                    order_id, user_id, course_id, order_number, status,
                    subtotal_cents, tax_cents, discount_cents, total_cents,
                    currency, payment_provider, payment_intent_id, discount_code,
                    metadata, created_at, updated_at
                FROM payments.orders
                WHERE status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(status.to_string())
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Order>(
                r#"
                SELECT
                    order_id, user_id, course_id, order_number, status,
                    subtotal_cents, tax_cents, discount_cents, total_cents,
                    currency, payment_provider, payment_intent_id, discount_code,
                    metadata, created_at, updated_at
                FROM payments.orders
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Finds an order by ID.
    pub async fn find_order_by_id(&self, order_id: Uuid) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            FROM payments.orders
            WHERE order_id = $1
            "#,
        )
        .bind(order_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Finds an order by order number.
    pub async fn find_order_by_number(&self, order_number: &str) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            FROM payments.orders
            WHERE order_number = $1
            "#,
        )
        .bind(order_number)
        .fetch_optional(&self.pool)
        .await
    }

    /// Finds an order by payment intent ID.
    pub async fn find_order_by_payment_intent(&self, payment_intent_id: &str) -> Result<Option<Order>, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            FROM payments.orders
            WHERE payment_intent_id = $1
            "#,
        )
        .bind(payment_intent_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates a new order.
    pub async fn create_order(&self, data: NewOrder) -> Result<Order, sqlx::Error> {
        let tax_cents = data.tax_cents.unwrap_or(0);
        let discount_cents = data.discount_cents.unwrap_or(0);
        let total_cents = data.subtotal_cents + tax_cents - discount_cents;
        let currency = data.currency.unwrap_or_else(|| "USD".to_string());
        let metadata = data.metadata.unwrap_or_else(|| serde_json::json!({}));

        sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO payments.orders (
                user_id, course_id, status, subtotal_cents, tax_cents,
                discount_cents, total_cents, currency, discount_code, metadata
            )
            VALUES ($1, $2, 'pending', $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            "#,
        )
        .bind(data.user_id)
        .bind(data.course_id)
        .bind(data.subtotal_cents)
        .bind(tax_cents)
        .bind(discount_cents)
        .bind(total_cents)
        .bind(&currency)
        .bind(&data.discount_code)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates an order.
    pub async fn update_order(&self, order_id: Uuid, data: UpdateOrder) -> Result<Order, sqlx::Error> {
        let mut query = String::from("UPDATE payments.orders SET updated_at = NOW()");
        let mut param_count = 1;

        if data.status.is_some() {
            param_count += 1;
            query.push_str(&format!(", status = ${}", param_count));
        }
        if data.payment_provider.is_some() {
            param_count += 1;
            query.push_str(&format!(", payment_provider = ${}", param_count));
        }
        if data.payment_intent_id.is_some() {
            param_count += 1;
            query.push_str(&format!(", payment_intent_id = ${}", param_count));
        }
        if data.metadata.is_some() {
            param_count += 1;
            query.push_str(&format!(", metadata = ${}", param_count));
        }

        query.push_str(" WHERE order_id = $1 RETURNING
            order_id, user_id, course_id, order_number, status,
            subtotal_cents, tax_cents, discount_cents, total_cents,
            currency, payment_provider, payment_intent_id, discount_code,
            metadata, created_at, updated_at");

        let mut query_builder = sqlx::query_as::<_, Order>(&query).bind(order_id);

        if let Some(status) = &data.status {
            query_builder = query_builder.bind(status.to_string());
        }
        if let Some(provider) = &data.payment_provider {
            query_builder = query_builder.bind(provider);
        }
        if let Some(intent_id) = &data.payment_intent_id {
            query_builder = query_builder.bind(intent_id);
        }
        if let Some(metadata) = &data.metadata {
            query_builder = query_builder.bind(metadata);
        }

        query_builder.fetch_one(&self.pool).await
    }

    /// Updates order status.
    pub async fn update_order_status(
        &self,
        order_id: Uuid,
        status: OrderStatus,
    ) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"
            UPDATE payments.orders
            SET status = $2, updated_at = NOW()
            WHERE order_id = $1
            RETURNING
                order_id, user_id, course_id, order_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents,
                currency, payment_provider, payment_intent_id, discount_code,
                metadata, created_at, updated_at
            "#,
        )
        .bind(order_id)
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // TRANSACTION OPERATIONS
    // =========================================================================

    /// Lists transactions for an order.
    pub async fn list_transactions_by_order(&self, order_id: Uuid) -> Result<Vec<Transaction>, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT
                transaction_id, order_id, provider, provider_transaction_id,
                transaction_type, amount_cents, currency, status,
                provider_fee_cents, metadata, processed_at
            FROM payments.transactions
            WHERE order_id = $1
            ORDER BY processed_at DESC
            "#,
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await
    }

    /// Finds a transaction by ID.
    pub async fn find_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT
                transaction_id, order_id, provider, provider_transaction_id,
                transaction_type, amount_cents, currency, status,
                provider_fee_cents, metadata, processed_at
            FROM payments.transactions
            WHERE transaction_id = $1
            "#,
        )
        .bind(transaction_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates a new transaction.
    pub async fn create_transaction(&self, data: NewTransaction) -> Result<Transaction, sqlx::Error> {
        let metadata = data.metadata.unwrap_or_else(|| serde_json::json!({}));

        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO payments.transactions (
                order_id, provider, provider_transaction_id, transaction_type,
                amount_cents, currency, status, provider_fee_cents, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                transaction_id, order_id, provider, provider_transaction_id,
                transaction_type, amount_cents, currency, status,
                provider_fee_cents, metadata, processed_at
            "#,
        )
        .bind(data.order_id)
        .bind(&data.provider)
        .bind(&data.provider_transaction_id)
        .bind(data.transaction_type.to_string())
        .bind(data.amount_cents)
        .bind(&data.currency)
        .bind(&data.status)
        .bind(data.provider_fee_cents)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // DISCOUNT CODE OPERATIONS
    // =========================================================================

    /// Lists all discount codes.
    pub async fn list_discount_codes(
        &self,
        active_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DiscountCode>, sqlx::Error> {
        if active_only {
            sqlx::query_as::<_, DiscountCode>(
                r#"
                SELECT
                    code_id, code, description, discount_type, discount_value,
                    minimum_order_cents, max_uses, current_uses, valid_from,
                    valid_until, is_active, created_by, created_at
                FROM payments.discount_codes
                WHERE is_active = TRUE
                    AND valid_from <= NOW()
                    AND (valid_until IS NULL OR valid_until > NOW())
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, DiscountCode>(
                r#"
                SELECT
                    code_id, code, description, discount_type, discount_value,
                    minimum_order_cents, max_uses, current_uses, valid_from,
                    valid_until, is_active, created_by, created_at
                FROM payments.discount_codes
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Finds a discount code by code string.
    pub async fn find_discount_code(&self, code: &str) -> Result<Option<DiscountCode>, sqlx::Error> {
        sqlx::query_as::<_, DiscountCode>(
            r#"
            SELECT
                code_id, code, description, discount_type, discount_value,
                minimum_order_cents, max_uses, current_uses, valid_from,
                valid_until, is_active, created_by, created_at
            FROM payments.discount_codes
            WHERE UPPER(code) = UPPER($1)
            "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates a new discount code.
    pub async fn create_discount_code(&self, data: NewDiscountCode) -> Result<DiscountCode, sqlx::Error> {
        let valid_from = data.valid_from.unwrap_or_else(chrono::Utc::now);

        sqlx::query_as::<_, DiscountCode>(
            r#"
            INSERT INTO payments.discount_codes (
                code, description, discount_type, discount_value,
                minimum_order_cents, max_uses, valid_from, valid_until, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                code_id, code, description, discount_type, discount_value,
                minimum_order_cents, max_uses, current_uses, valid_from,
                valid_until, is_active, created_by, created_at
            "#,
        )
        .bind(&data.code.to_uppercase())
        .bind(&data.description)
        .bind(data.discount_type.to_string())
        .bind(data.discount_value)
        .bind(data.minimum_order_cents)
        .bind(data.max_uses)
        .bind(valid_from)
        .bind(data.valid_until)
        .bind(data.created_by)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates a discount code.
    pub async fn update_discount_code(
        &self,
        code_id: Uuid,
        data: UpdateDiscountCode,
    ) -> Result<DiscountCode, sqlx::Error> {
        // Build dynamic query
        let mut updates = Vec::new();
        let mut param_idx = 2;

        if data.description.is_some() {
            updates.push(format!("description = ${}", param_idx));
            param_idx += 1;
        }
        if data.discount_type.is_some() {
            updates.push(format!("discount_type = ${}", param_idx));
            param_idx += 1;
        }
        if data.discount_value.is_some() {
            updates.push(format!("discount_value = ${}", param_idx));
            param_idx += 1;
        }
        if data.minimum_order_cents.is_some() {
            updates.push(format!("minimum_order_cents = ${}", param_idx));
            param_idx += 1;
        }
        if data.max_uses.is_some() {
            updates.push(format!("max_uses = ${}", param_idx));
            param_idx += 1;
        }
        if data.valid_from.is_some() {
            updates.push(format!("valid_from = ${}", param_idx));
            param_idx += 1;
        }
        if data.valid_until.is_some() {
            updates.push(format!("valid_until = ${}", param_idx));
            param_idx += 1;
        }
        if data.is_active.is_some() {
            updates.push(format!("is_active = ${}", param_idx));
        }

        let query = format!(
            r#"
            UPDATE payments.discount_codes
            SET {}
            WHERE code_id = $1
            RETURNING
                code_id, code, description, discount_type, discount_value,
                minimum_order_cents, max_uses, current_uses, valid_from,
                valid_until, is_active, created_by, created_at
            "#,
            updates.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, DiscountCode>(&query).bind(code_id);

        if let Some(desc) = &data.description {
            query_builder = query_builder.bind(desc.as_ref());
        }
        if let Some(dt) = &data.discount_type {
            query_builder = query_builder.bind(dt.to_string());
        }
        if let Some(dv) = &data.discount_value {
            query_builder = query_builder.bind(dv);
        }
        if let Some(min) = &data.minimum_order_cents {
            query_builder = query_builder.bind(*min);
        }
        if let Some(max) = &data.max_uses {
            query_builder = query_builder.bind(*max);
        }
        if let Some(vf) = &data.valid_from {
            query_builder = query_builder.bind(vf);
        }
        if let Some(vu) = &data.valid_until {
            query_builder = query_builder.bind(vu.as_ref());
        }
        if let Some(active) = &data.is_active {
            query_builder = query_builder.bind(active);
        }

        query_builder.fetch_one(&self.pool).await
    }

    /// Increments the use count for a discount code.
    pub async fn increment_discount_code_usage(&self, code: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE payments.discount_codes
            SET current_uses = current_uses + 1
            WHERE UPPER(code) = UPPER($1)
            "#,
        )
        .bind(code)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // =========================================================================
    // REVIEW OPERATIONS
    // =========================================================================

    /// Lists reviews for a course.
    pub async fn list_reviews_by_course(
        &self,
        course_id: Uuid,
        public_only: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, sqlx::Error> {
        if public_only {
            sqlx::query_as::<_, Review>(
                r#"
                SELECT
                    review_id, course_id, user_id, enrollment_id, rating,
                    review_title, review_text, is_public, is_verified_purchase,
                    helpful_votes, created_at, updated_at
                FROM payments.reviews
                WHERE course_id = $1 AND is_public = TRUE
                ORDER BY helpful_votes DESC, created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(course_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, Review>(
                r#"
                SELECT
                    review_id, course_id, user_id, enrollment_id, rating,
                    review_title, review_text, is_public, is_verified_purchase,
                    helpful_votes, created_at, updated_at
                FROM payments.reviews
                WHERE course_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(course_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
        }
    }

    /// Finds a review by ID.
    pub async fn find_review_by_id(&self, review_id: Uuid) -> Result<Option<Review>, sqlx::Error> {
        sqlx::query_as::<_, Review>(
            r#"
            SELECT
                review_id, course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public, is_verified_purchase,
                helpful_votes, created_at, updated_at
            FROM payments.reviews
            WHERE review_id = $1
            "#,
        )
        .bind(review_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Finds a user's review for a course.
    pub async fn find_user_review(
        &self,
        course_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Review>, sqlx::Error> {
        sqlx::query_as::<_, Review>(
            r#"
            SELECT
                review_id, course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public, is_verified_purchase,
                helpful_votes, created_at, updated_at
            FROM payments.reviews
            WHERE course_id = $1 AND user_id = $2
            "#,
        )
        .bind(course_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Creates a new review.
    pub async fn create_review(&self, data: NewReview) -> Result<Review, sqlx::Error> {
        let is_public = data.is_public.unwrap_or(true);

        sqlx::query_as::<_, Review>(
            r#"
            INSERT INTO payments.reviews (
                course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                review_id, course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public, is_verified_purchase,
                helpful_votes, created_at, updated_at
            "#,
        )
        .bind(data.course_id)
        .bind(data.user_id)
        .bind(data.enrollment_id)
        .bind(data.rating)
        .bind(&data.review_title)
        .bind(&data.review_text)
        .bind(is_public)
        .fetch_one(&self.pool)
        .await
    }

    /// Updates a review.
    pub async fn update_review(&self, review_id: Uuid, data: UpdateReview) -> Result<Review, sqlx::Error> {
        let mut updates = vec!["updated_at = NOW()".to_string()];
        let mut param_idx = 2;

        if data.rating.is_some() {
            updates.push(format!("rating = ${}", param_idx));
            param_idx += 1;
        }
        if data.review_title.is_some() {
            updates.push(format!("review_title = ${}", param_idx));
            param_idx += 1;
        }
        if data.review_text.is_some() {
            updates.push(format!("review_text = ${}", param_idx));
            param_idx += 1;
        }
        if data.is_public.is_some() {
            updates.push(format!("is_public = ${}", param_idx));
        }

        let query = format!(
            r#"
            UPDATE payments.reviews
            SET {}
            WHERE review_id = $1
            RETURNING
                review_id, course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public, is_verified_purchase,
                helpful_votes, created_at, updated_at
            "#,
            updates.join(", ")
        );

        let mut query_builder = sqlx::query_as::<_, Review>(&query).bind(review_id);

        if let Some(rating) = &data.rating {
            query_builder = query_builder.bind(rating);
        }
        if let Some(title) = &data.review_title {
            query_builder = query_builder.bind(title.as_ref());
        }
        if let Some(text) = &data.review_text {
            query_builder = query_builder.bind(text.as_ref());
        }
        if let Some(public) = &data.is_public {
            query_builder = query_builder.bind(public);
        }

        query_builder.fetch_one(&self.pool).await
    }

    /// Deletes a review.
    pub async fn delete_review(&self, review_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM payments.reviews WHERE review_id = $1"
        )
        .bind(review_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Increments helpful votes for a review.
    pub async fn increment_helpful_votes(&self, review_id: Uuid) -> Result<Review, sqlx::Error> {
        sqlx::query_as::<_, Review>(
            r#"
            UPDATE payments.reviews
            SET helpful_votes = helpful_votes + 1, updated_at = NOW()
            WHERE review_id = $1
            RETURNING
                review_id, course_id, user_id, enrollment_id, rating,
                review_title, review_text, is_public, is_verified_purchase,
                helpful_votes, created_at, updated_at
            "#,
        )
        .bind(review_id)
        .fetch_one(&self.pool)
        .await
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Gets order statistics.
    pub async fn get_order_stats(&self) -> Result<OrderStats, sqlx::Error> {
        sqlx::query_as::<_, OrderStats>(
            r#"
            SELECT
                COUNT(*) as total_orders,
                COUNT(*) FILTER (WHERE status = 'paid') as paid_orders,
                COALESCE(SUM(total_cents) FILTER (WHERE status = 'paid'), 0) as total_revenue_cents,
                COALESCE(AVG(total_cents) FILTER (WHERE status = 'paid'), 0) as avg_order_value_cents
            FROM payments.orders
            "#,
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Gets review statistics for a course.
    pub async fn get_review_stats(&self, course_id: Uuid) -> Result<ReviewStats, sqlx::Error> {
        sqlx::query_as::<_, ReviewStats>(
            r#"
            SELECT
                COUNT(*) as total_reviews,
                COALESCE(AVG(rating), 0) as average_rating,
                COUNT(*) FILTER (WHERE rating = 1) as one_star,
                COUNT(*) FILTER (WHERE rating = 2) as two_star,
                COUNT(*) FILTER (WHERE rating = 3) as three_star,
                COUNT(*) FILTER (WHERE rating = 4) as four_star,
                COUNT(*) FILTER (WHERE rating = 5) as five_star
            FROM payments.reviews
            WHERE course_id = $1 AND is_public = TRUE
            "#,
        )
        .bind(course_id)
        .fetch_one(&self.pool)
        .await
    }
}

/// Order statistics.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderStats {
    pub total_orders: i64,
    pub paid_orders: i64,
    pub total_revenue_cents: i64,
    pub avg_order_value_cents: f64,
}

/// Review statistics for a course.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ReviewStats {
    pub total_reviews: i64,
    pub average_rating: f64,
    pub one_star: i64,
    pub two_star: i64,
    pub three_star: i64,
    pub four_star: i64,
    pub five_star: i64,
}
