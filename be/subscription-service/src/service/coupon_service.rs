// =============================================================================
// Coupon Service - Business logic for coupons and discounts
// =============================================================================

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::{Coupon, CouponRedemption, DiscountType};
use crate::repository::CouponRepository;

#[derive(Debug, Error)]
pub enum CouponError {
    #[error("Coupon not found: {0}")]
    CouponNotFound(String),

    #[error("Coupon is not active")]
    NotActive,

    #[error("Coupon has expired")]
    Expired,

    #[error("Coupon has reached maximum redemptions")]
    MaxRedemptionsReached,

    #[error("User has already redeemed this coupon")]
    AlreadyRedeemed,

    #[error("Coupon does not apply to this plan")]
    NotApplicableToPlan,

    #[error("Amount does not meet minimum requirement")]
    BelowMinimumAmount,

    #[error("Coupon is only for first-time users")]
    FirstTimeOnly,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct CouponService {
    repository: CouponRepository,
}

impl CouponService {
    pub fn new(repository: CouponRepository) -> Self {
        Self { repository }
    }

    pub async fn create_coupon(&self, coupon: Coupon) -> Result<Coupon, CouponError> {
        self.repository.create_coupon(&coupon).await?;
        Ok(coupon)
    }

    pub async fn get_coupon_by_code(&self, code: &str) -> Result<Coupon, CouponError> {
        self.repository
            .get_coupon_by_code(code)
            .await?
            .ok_or_else(|| CouponError::CouponNotFound(code.to_string()))
    }

    pub async fn get_coupon(&self, coupon_id: Uuid) -> Result<Coupon, CouponError> {
        self.repository
            .get_coupon_by_id(coupon_id)
            .await?
            .ok_or_else(|| CouponError::CouponNotFound(coupon_id.to_string()))
    }

    pub async fn list_active_coupons(&self) -> Result<Vec<Coupon>, CouponError> {
        self.repository.list_active_coupons().await.map_err(Into::into)
    }

    pub async fn validate_coupon(
        &self,
        code: &str,
        plan_id: Option<Uuid>,
        user_id: Uuid,
        amount_cents: i64,
        is_first_subscription: bool,
    ) -> Result<Coupon, CouponError> {
        let coupon = self.get_coupon_by_code(code).await?;

        // Check if active
        if !coupon.is_active {
            return Err(CouponError::NotActive);
        }

        // Check valid_from
        let now = Utc::now();
        if let Some(valid_from) = coupon.valid_from {
            if now < valid_from {
                return Err(CouponError::NotActive);
            }
        }

        // Check valid_until
        if let Some(valid_until) = coupon.valid_until {
            if now > valid_until {
                return Err(CouponError::Expired);
            }
        }

        // Check max redemptions
        if let Some(max) = coupon.max_redemptions {
            if coupon.current_redemptions >= max {
                return Err(CouponError::MaxRedemptionsReached);
            }
        }

        // Check minimum amount
        if let Some(min_amount) = coupon.min_amount_cents {
            if amount_cents < min_amount {
                return Err(CouponError::BelowMinimumAmount);
            }
        }

        // Check first-time only
        if coupon.first_time_only && !is_first_subscription {
            return Err(CouponError::FirstTimeOnly);
        }

        // Check if user already redeemed
        if self.repository.has_user_redeemed(user_id, coupon.coupon_id).await? {
            return Err(CouponError::AlreadyRedeemed);
        }

        // Check plan applicability
        if let Some(plan_id) = plan_id {
            if let Some(ref applies_to_plans) = coupon.applies_to_plans {
                let plan_ids: Vec<Uuid> = serde_json::from_value(applies_to_plans.clone())
                    .unwrap_or_default();
                if !plan_ids.is_empty() && !plan_ids.contains(&plan_id) {
                    return Err(CouponError::NotApplicableToPlan);
                }
            }
        }

        Ok(coupon)
    }

    pub async fn calculate_discount(&self, coupon: &Coupon, amount_cents: i64) -> i64 {
        match coupon.discount_type {
            DiscountType::Percentage => {
                // discount_value is stored as percentage (e.g., 20 for 20%)
                (amount_cents * coupon.discount_value as i64) / 100
            }
            DiscountType::FixedAmount => {
                coupon.discount_value as i64
            }
        }
    }

    pub async fn redeem_coupon(
        &self,
        coupon_id: Uuid,
        user_id: Uuid,
        subscription_id: Uuid,
    ) -> Result<CouponRedemption, CouponError> {
        let _coupon = self.get_coupon(coupon_id).await?;

        // Increment redemption count
        self.repository.increment_redemptions(coupon_id).await?;

        // Record the redemption
        let redemption = CouponRedemption {
            redemption_id: Uuid::new_v4(),
            coupon_id,
            user_id,
            subscription_id,
            redeemed_at: Utc::now(),
        };

        self.repository.record_redemption(&redemption).await?;
        Ok(redemption)
    }

    pub async fn update_coupon(&self, coupon: Coupon) -> Result<Coupon, CouponError> {
        self.repository.update_coupon(&coupon).await?;
        Ok(coupon)
    }

    pub async fn deactivate_coupon(&self, coupon_id: Uuid) -> Result<(), CouponError> {
        self.repository.deactivate_coupon(coupon_id).await.map_err(Into::into)
    }

    pub async fn get_redemptions(&self, coupon_id: Uuid) -> Result<Vec<CouponRedemption>, CouponError> {
        self.repository.get_redemptions(coupon_id).await.map_err(Into::into)
    }
}
