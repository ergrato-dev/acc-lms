// =============================================================================
// Subscription Repository - PostgreSQL persistence layer
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{
    BillingInterval, CancellationReason, PlanTier, Subscription,
    SubscriptionPlan, SubscriptionStatus,
};

/// Repository for subscription plan and subscription persistence
pub struct SubscriptionRepository {
    pool: PgPool,
}

impl SubscriptionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // SUBSCRIPTION PLANS
    // =========================================================================

    /// Create a new subscription plan
    pub async fn create_plan(&self, plan: &SubscriptionPlan) -> Result<SubscriptionPlan, sqlx::Error> {
        let features_json = serde_json::to_value(&plan.features).unwrap_or_default();
        let limits_json = serde_json::to_value(&plan.limits).unwrap_or_default();

        let row = sqlx::query(
            r#"
            INSERT INTO subscriptions.subscription_plans (
                plan_id, name, slug, description, tier, billing_interval,
                price_cents, currency, trial_days, features, limits,
                stripe_price_id, is_active, is_public, sort_order, metadata,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING *
            "#,
        )
        .bind(plan.plan_id)
        .bind(&plan.name)
        .bind(&plan.slug)
        .bind(&plan.description)
        .bind(plan.tier.to_string())
        .bind(plan.billing_interval.to_string())
        .bind(plan.price_cents)
        .bind(&plan.currency)
        .bind(plan.trial_days)
        .bind(&features_json)
        .bind(&limits_json)
        .bind(&plan.stripe_price_id)
        .bind(plan.is_active)
        .bind(plan.is_public)
        .bind(plan.sort_order)
        .bind(&plan.metadata)
        .bind(plan.created_at)
        .bind(plan.updated_at)
        .fetch_one(&self.pool)
        .await?;

        self.row_to_plan(&row)
    }

    /// Get plan by ID
    pub async fn get_plan_by_id(&self, plan_id: Uuid) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.subscription_plans WHERE plan_id = $1"#,
        )
        .bind(plan_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_plan(&r)?)),
            None => Ok(None),
        }
    }

    /// Get plan by slug
    pub async fn get_plan_by_slug(&self, slug: &str) -> Result<Option<SubscriptionPlan>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.subscription_plans WHERE slug = $1"#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_plan(&r)?)),
            None => Ok(None),
        }
    }

    /// List public plans
    pub async fn list_public_plans(&self) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscription_plans
            WHERE is_active = true AND is_public = true
            ORDER BY sort_order, price_cents
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_plan(r)).collect()
    }

    /// List all plans (admin)
    pub async fn list_all_plans(&self) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscription_plans
            ORDER BY sort_order, price_cents
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_plan(r)).collect()
    }

    /// List plans by tier
    pub async fn list_plans_by_tier(&self, tier: PlanTier) -> Result<Vec<SubscriptionPlan>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscription_plans
            WHERE tier = $1 AND is_active = true
            ORDER BY price_cents
            "#,
        )
        .bind(tier.to_string())
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_plan(r)).collect()
    }

    /// Update a subscription plan
    pub async fn update_plan(&self, plan: &SubscriptionPlan) -> Result<SubscriptionPlan, sqlx::Error> {
        let features_json = serde_json::to_value(&plan.features).unwrap_or_default();
        let limits_json = serde_json::to_value(&plan.limits).unwrap_or_default();

        let row = sqlx::query(
            r#"
            UPDATE subscriptions.subscription_plans SET
                name = $2, description = $3, price_cents = $4, trial_days = $5,
                features = $6, limits = $7, stripe_price_id = $8, is_active = $9,
                is_public = $10, sort_order = $11, metadata = $12, updated_at = $13
            WHERE plan_id = $1
            RETURNING *
            "#,
        )
        .bind(plan.plan_id)
        .bind(&plan.name)
        .bind(&plan.description)
        .bind(plan.price_cents)
        .bind(plan.trial_days)
        .bind(&features_json)
        .bind(&limits_json)
        .bind(&plan.stripe_price_id)
        .bind(plan.is_active)
        .bind(plan.is_public)
        .bind(plan.sort_order)
        .bind(&plan.metadata)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        self.row_to_plan(&row)
    }

    /// Convert a database row to SubscriptionPlan
    fn row_to_plan(&self, row: &sqlx::postgres::PgRow) -> Result<SubscriptionPlan, sqlx::Error> {
        let tier_str: String = row.try_get("tier")?;
        let interval_str: String = row.try_get("billing_interval")?;
        let features_json: serde_json::Value = row.try_get("features")?;
        let limits_json: serde_json::Value = row.try_get("limits")?;

        Ok(SubscriptionPlan {
            plan_id: row.try_get("plan_id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            description: row.try_get("description")?,
            tier: tier_str.parse().unwrap_or(PlanTier::Free),
            billing_interval: interval_str.parse().unwrap_or(BillingInterval::Monthly),
            price_cents: row.try_get("price_cents")?,
            currency: row.try_get("currency")?,
            trial_days: row.try_get("trial_days")?,
            features: serde_json::from_value(features_json).unwrap_or_default(),
            limits: serde_json::from_value(limits_json).unwrap_or_default(),
            stripe_price_id: row.try_get("stripe_price_id")?,
            is_active: row.try_get("is_active")?,
            is_public: row.try_get("is_public")?,
            sort_order: row.try_get("sort_order")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    // =========================================================================
    // SUBSCRIPTIONS
    // =========================================================================

    /// Create a new subscription
    pub async fn create_subscription(&self, sub: &Subscription) -> Result<Subscription, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO subscriptions.subscriptions (
                subscription_id, user_id, organization_id, plan_id, status,
                current_period_start, current_period_end, trial_start, trial_end,
                cancelled_at, cancel_at_period_end, cancellation_reason,
                cancellation_feedback, stripe_subscription_id, stripe_customer_id,
                quantity, metadata, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#,
        )
        .bind(sub.subscription_id)
        .bind(sub.user_id)
        .bind(sub.organization_id)
        .bind(sub.plan_id)
        .bind(sub.status.to_string())
        .bind(sub.current_period_start)
        .bind(sub.current_period_end)
        .bind(sub.trial_start)
        .bind(sub.trial_end)
        .bind(sub.cancelled_at)
        .bind(sub.cancel_at_period_end)
        .bind(sub.cancellation_reason.as_ref().map(|r| r.to_string()))
        .bind(&sub.cancellation_feedback)
        .bind(&sub.stripe_subscription_id)
        .bind(&sub.stripe_customer_id)
        .bind(sub.quantity)
        .bind(&sub.metadata)
        .bind(sub.created_at)
        .bind(sub.updated_at)
        .fetch_one(&self.pool)
        .await?;

        self.row_to_subscription(&row)
    }

    /// Get subscription by ID
    pub async fn get_subscription_by_id(&self, subscription_id: Uuid) -> Result<Option<Subscription>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.subscriptions WHERE subscription_id = $1"#,
        )
        .bind(subscription_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_subscription(&r)?)),
            None => Ok(None),
        }
    }

    /// Get active subscription for user
    pub async fn get_active_subscription_for_user(&self, user_id: Uuid) -> Result<Option<Subscription>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscriptions
            WHERE user_id = $1 AND status IN ('active', 'trialing', 'past_due')
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_subscription(&r)?)),
            None => Ok(None),
        }
    }

    /// List subscriptions for user
    pub async fn list_subscriptions_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscriptions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_subscription(r)).collect()
    }

    /// List subscriptions by status
    pub async fn list_subscriptions_by_status(
        &self,
        status: SubscriptionStatus,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Subscription>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscriptions
            WHERE status = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status.to_string())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_subscription(r)).collect()
    }

    /// Find subscriptions expiring soon
    pub async fn find_expiring_subscriptions(&self, end_date: DateTime<Utc>) -> Result<Vec<Subscription>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.subscriptions
            WHERE status IN ('active', 'trialing')
            AND current_period_end <= $1
            ORDER BY current_period_end
            "#,
        )
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_subscription(r)).collect()
    }

    /// Update subscription
    pub async fn update_subscription(&self, sub: &Subscription) -> Result<Subscription, sqlx::Error> {
        let row = sqlx::query(
            r#"
            UPDATE subscriptions.subscriptions SET
                plan_id = $2, status = $3, current_period_start = $4, current_period_end = $5,
                trial_start = $6, trial_end = $7, cancelled_at = $8, cancel_at_period_end = $9,
                cancellation_reason = $10, cancellation_feedback = $11,
                stripe_subscription_id = $12, stripe_customer_id = $13,
                quantity = $14, metadata = $15, updated_at = $16
            WHERE subscription_id = $1
            RETURNING *
            "#,
        )
        .bind(sub.subscription_id)
        .bind(sub.plan_id)
        .bind(sub.status.to_string())
        .bind(sub.current_period_start)
        .bind(sub.current_period_end)
        .bind(sub.trial_start)
        .bind(sub.trial_end)
        .bind(sub.cancelled_at)
        .bind(sub.cancel_at_period_end)
        .bind(sub.cancellation_reason.as_ref().map(|r| r.to_string()))
        .bind(&sub.cancellation_feedback)
        .bind(&sub.stripe_subscription_id)
        .bind(&sub.stripe_customer_id)
        .bind(sub.quantity)
        .bind(&sub.metadata)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        self.row_to_subscription(&row)
    }

    /// Update subscription status
    pub async fn update_subscription_status(
        &self,
        subscription_id: Uuid,
        status: SubscriptionStatus,
    ) -> Result<Subscription, sqlx::Error> {
        let row = sqlx::query(
            r#"
            UPDATE subscriptions.subscriptions SET
                status = $2, updated_at = $3
            WHERE subscription_id = $1
            RETURNING *
            "#,
        )
        .bind(subscription_id)
        .bind(status.to_string())
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        self.row_to_subscription(&row)
    }

    /// Count subscriptions by status
    pub async fn count_by_status(&self, status: SubscriptionStatus) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT COUNT(*) as count FROM subscriptions.subscriptions WHERE status = $1"#,
        )
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get::<i64, _>("count")?)
    }

    /// Count total subscriptions
    pub async fn count_total(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT COUNT(*) as count FROM subscriptions.subscriptions"#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get::<i64, _>("count")?)
    }

    /// Convert a database row to Subscription
    fn row_to_subscription(&self, row: &sqlx::postgres::PgRow) -> Result<Subscription, sqlx::Error> {
        let status_str: String = row.try_get("status")?;
        let reason_str: Option<String> = row.try_get("cancellation_reason")?;

        let cancellation_reason = reason_str.and_then(|s| match s.as_str() {
            "user_requested" => Some(CancellationReason::UserRequested),
            "payment_failed" => Some(CancellationReason::PaymentFailed),
            "fraud" => Some(CancellationReason::Fraud),
            "policy_violation" => Some(CancellationReason::PolicyViolation),
            "downgrade_to_plan" => Some(CancellationReason::DowngradeToPlan),
            "migration" => Some(CancellationReason::Migration),
            "other" => Some(CancellationReason::Other),
            _ => None,
        });

        Ok(Subscription {
            subscription_id: row.try_get("subscription_id")?,
            user_id: row.try_get("user_id")?,
            organization_id: row.try_get("organization_id")?,
            plan_id: row.try_get("plan_id")?,
            status: status_str.parse().unwrap_or(SubscriptionStatus::Incomplete),
            current_period_start: row.try_get("current_period_start")?,
            current_period_end: row.try_get("current_period_end")?,
            trial_start: row.try_get("trial_start")?,
            trial_end: row.try_get("trial_end")?,
            cancelled_at: row.try_get("cancelled_at")?,
            cancel_at_period_end: row.try_get("cancel_at_period_end")?,
            cancellation_reason,
            cancellation_feedback: row.try_get("cancellation_feedback")?,
            stripe_subscription_id: row.try_get("stripe_subscription_id")?,
            stripe_customer_id: row.try_get("stripe_customer_id")?,
            quantity: row.try_get("quantity")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
