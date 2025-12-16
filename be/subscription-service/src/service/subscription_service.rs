// =============================================================================
// Subscription Service - Business logic for subscriptions and plans
// =============================================================================

use chrono::{Duration, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::{
    PlanTier, Subscription, SubscriptionPlan, SubscriptionStatus,
};
use crate::repository::SubscriptionRepository;

#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Plan not found: {0}")]
    PlanNotFound(Uuid),

    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(Uuid),

    #[error("User already has an active subscription")]
    AlreadySubscribed,

    #[error("Cannot change to the same plan")]
    SamePlan,

    #[error("Subscription is not active")]
    NotActive,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct SubscriptionService {
    repository: SubscriptionRepository,
}

impl SubscriptionService {
    pub fn new(repository: SubscriptionRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // PLAN OPERATIONS
    // =========================================================================

    pub async fn create_plan(&self, plan: SubscriptionPlan) -> Result<SubscriptionPlan, SubscriptionError> {
        self.repository.create_plan(&plan).await.map_err(Into::into)
    }

    pub async fn get_plan(&self, plan_id: Uuid) -> Result<SubscriptionPlan, SubscriptionError> {
        self.repository
            .get_plan_by_id(plan_id)
            .await?
            .ok_or(SubscriptionError::PlanNotFound(plan_id))
    }

    pub async fn get_plan_by_slug(&self, slug: &str) -> Result<Option<SubscriptionPlan>, SubscriptionError> {
        self.repository.get_plan_by_slug(slug).await.map_err(Into::into)
    }

    pub async fn list_public_plans(&self) -> Result<Vec<SubscriptionPlan>, SubscriptionError> {
        self.repository.list_public_plans().await.map_err(Into::into)
    }

    pub async fn list_all_plans(&self) -> Result<Vec<SubscriptionPlan>, SubscriptionError> {
        self.repository.list_all_plans().await.map_err(Into::into)
    }

    pub async fn list_plans_by_tier(&self, tier: PlanTier) -> Result<Vec<SubscriptionPlan>, SubscriptionError> {
        self.repository.list_plans_by_tier(tier).await.map_err(Into::into)
    }

    pub async fn update_plan(&self, plan: SubscriptionPlan) -> Result<SubscriptionPlan, SubscriptionError> {
        self.repository.update_plan(&plan).await.map_err(Into::into)
    }

    // =========================================================================
    // SUBSCRIPTION OPERATIONS
    // =========================================================================

    pub async fn create_subscription(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
        organization_id: Option<Uuid>,
    ) -> Result<Subscription, SubscriptionError> {
        // Check if user already has active subscription
        if let Some(_) = self.repository.get_active_subscription_for_user(user_id).await? {
            return Err(SubscriptionError::AlreadySubscribed);
        }

        // Get plan details
        let plan = self.get_plan(plan_id).await?;

        let now = Utc::now();
        let (status, trial_start, trial_end) = if plan.trial_days > 0 {
            (
                SubscriptionStatus::Trialing,
                Some(now),
                Some(now + Duration::days(plan.trial_days as i64)),
            )
        } else {
            (SubscriptionStatus::Active, None, None)
        };

        let period_end = match plan.billing_interval {
            crate::domain::entities::BillingInterval::Daily => now + Duration::days(1),
            crate::domain::entities::BillingInterval::Weekly => now + Duration::days(7),
            crate::domain::entities::BillingInterval::Monthly => now + Duration::days(30),
            crate::domain::entities::BillingInterval::Quarterly => now + Duration::days(90),
            crate::domain::entities::BillingInterval::SemiAnnual => now + Duration::days(180),
            crate::domain::entities::BillingInterval::Annual => now + Duration::days(365),
            crate::domain::entities::BillingInterval::Lifetime => now + Duration::days(36500),
        };

        let subscription = Subscription {
            subscription_id: Uuid::new_v4(),
            user_id,
            organization_id,
            plan_id,
            status,
            current_period_start: now,
            current_period_end: period_end,
            trial_start,
            trial_end,
            cancelled_at: None,
            cancel_at_period_end: false,
            cancellation_reason: None,
            cancellation_feedback: None,
            stripe_subscription_id: None,
            stripe_customer_id: None,
            quantity: 1,
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.repository.create_subscription(&subscription).await.map_err(Into::into)
    }

    pub async fn get_subscription(&self, subscription_id: Uuid) -> Result<Subscription, SubscriptionError> {
        self.repository
            .get_subscription_by_id(subscription_id)
            .await?
            .ok_or(SubscriptionError::SubscriptionNotFound(subscription_id))
    }

    pub async fn get_active_subscription(&self, user_id: Uuid) -> Result<Option<Subscription>, SubscriptionError> {
        self.repository.get_active_subscription_for_user(user_id).await.map_err(Into::into)
    }

    pub async fn list_subscriptions(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Subscription>, SubscriptionError> {
        self.repository.list_subscriptions_for_user(user_id, limit, offset).await.map_err(Into::into)
    }

    pub async fn change_plan(
        &self,
        subscription_id: Uuid,
        new_plan_id: Uuid,
    ) -> Result<Subscription, SubscriptionError> {
        let mut subscription = self.get_subscription(subscription_id).await?;

        if subscription.plan_id == new_plan_id {
            return Err(SubscriptionError::SamePlan);
        }

        if !subscription.is_active() {
            return Err(SubscriptionError::NotActive);
        }

        // Verify new plan exists
        let _ = self.get_plan(new_plan_id).await?;

        subscription.plan_id = new_plan_id;
        subscription.updated_at = Utc::now();

        self.repository.update_subscription(&subscription).await.map_err(Into::into)
    }

    pub async fn cancel_subscription(
        &self,
        subscription_id: Uuid,
        reason: Option<crate::domain::entities::CancellationReason>,
        feedback: Option<String>,
        immediate: bool,
    ) -> Result<Subscription, SubscriptionError> {
        let mut subscription = self.get_subscription(subscription_id).await?;

        if !subscription.is_active() {
            return Err(SubscriptionError::NotActive);
        }

        let now = Utc::now();
        subscription.cancelled_at = Some(now);
        subscription.cancellation_reason = reason;
        subscription.cancellation_feedback = feedback;
        subscription.updated_at = now;

        if immediate {
            subscription.status = SubscriptionStatus::Cancelled;
        } else {
            subscription.cancel_at_period_end = true;
        }

        self.repository.update_subscription(&subscription).await.map_err(Into::into)
    }

    pub async fn reactivate_subscription(&self, subscription_id: Uuid) -> Result<Subscription, SubscriptionError> {
        let mut subscription = self.get_subscription(subscription_id).await?;

        if subscription.status != SubscriptionStatus::Cancelled {
            return Err(SubscriptionError::NotActive);
        }

        // Can only reactivate if period hasn't ended
        if subscription.current_period_end < Utc::now() {
            return Err(SubscriptionError::NotActive);
        }

        subscription.status = SubscriptionStatus::Active;
        subscription.cancelled_at = None;
        subscription.cancel_at_period_end = false;
        subscription.cancellation_reason = None;
        subscription.cancellation_feedback = None;
        subscription.updated_at = Utc::now();

        self.repository.update_subscription(&subscription).await.map_err(Into::into)
    }

    pub async fn update_subscription_status(
        &self,
        subscription_id: Uuid,
        status: SubscriptionStatus,
    ) -> Result<Subscription, SubscriptionError> {
        self.repository.update_subscription_status(subscription_id, status).await.map_err(Into::into)
    }
}
