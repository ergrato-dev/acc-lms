// =============================================================================
// Coupon Repository - PostgreSQL persistence for coupons
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{Coupon, CouponRedemption, DiscountType};

/// Repository for coupon persistence
pub struct CouponRepository {
    pool: PgPool,
}

impl CouponRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new coupon
    pub async fn create_coupon(&self, coupon: &Coupon) -> Result<(), sqlx::Error> {
        let applies_to_plans: Option<Vec<String>> = coupon.applies_to_plans.as_ref().map(|v| {
            v.iter().map(|u| u.to_string()).collect()
        });

        sqlx::query(
            r#"
            INSERT INTO subscriptions.coupons (
                coupon_id, code, name, description, discount_type, discount_value,
                max_redemptions, current_redemptions, valid_from, valid_until,
                min_amount_cents, applies_to_plans, first_time_only, is_active,
                metadata, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
        )
        .bind(coupon.coupon_id)
        .bind(&coupon.code)
        .bind(&coupon.name)
        .bind(&coupon.description)
        .bind(coupon.discount_type.to_string())
        .bind(coupon.discount_value)
        .bind(coupon.max_redemptions)
        .bind(coupon.current_redemptions)
        .bind(coupon.valid_from)
        .bind(coupon.valid_until)
        .bind(coupon.min_amount_cents)
        .bind(&applies_to_plans)
        .bind(coupon.first_time_only)
        .bind(coupon.is_active)
        .bind(&coupon.metadata)
        .bind(coupon.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get coupon by code
    pub async fn get_coupon_by_code(&self, code: &str) -> Result<Option<Coupon>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.coupons WHERE UPPER(code) = UPPER($1)"#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_coupon(&r)?)),
            None => Ok(None),
        }
    }

    /// Get coupon by ID
    pub async fn get_coupon_by_id(&self, coupon_id: Uuid) -> Result<Option<Coupon>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.coupons WHERE coupon_id = $1"#,
        )
        .bind(coupon_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_coupon(&r)?)),
            None => Ok(None),
        }
    }

    /// List active coupons
    pub async fn list_active_coupons(&self) -> Result<Vec<Coupon>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.coupons
            WHERE is_active = true
            AND (valid_from IS NULL OR valid_from <= NOW())
            AND (valid_until IS NULL OR valid_until >= NOW())
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_coupon(r)).collect()
    }

    /// Validate coupon for plan
    pub async fn validate_coupon_for_plan(
        &self,
        code: &str,
        plan_id: Uuid,
    ) -> Result<Option<Coupon>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM subscriptions.coupons
            WHERE UPPER(code) = UPPER($1)
            AND is_active = true
            AND (valid_from IS NULL OR valid_from <= NOW())
            AND (valid_until IS NULL OR valid_until >= NOW())
            AND (max_redemptions IS NULL OR current_redemptions < max_redemptions)
            AND (applies_to_plans IS NULL OR $2::TEXT = ANY(applies_to_plans))
            "#,
        )
        .bind(code)
        .bind(plan_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_coupon(&r)?)),
            None => Ok(None),
        }
    }

    /// Increment redemption count
    pub async fn increment_redemptions(&self, coupon_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE subscriptions.coupons
            SET current_redemptions = current_redemptions + 1
            WHERE coupon_id = $1
            "#,
        )
        .bind(coupon_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update coupon
    pub async fn update_coupon(&self, coupon: &Coupon) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE subscriptions.coupons SET
                name = $2, description = $3, max_redemptions = $4,
                valid_until = $5, is_active = $6
            WHERE coupon_id = $1
            "#,
        )
        .bind(coupon.coupon_id)
        .bind(&coupon.name)
        .bind(&coupon.description)
        .bind(coupon.max_redemptions)
        .bind(coupon.valid_until)
        .bind(coupon.is_active)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Deactivate coupon
    pub async fn deactivate_coupon(&self, coupon_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE subscriptions.coupons SET is_active = false WHERE coupon_id = $1"#,
        )
        .bind(coupon_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Record redemption
    pub async fn record_redemption(&self, redemption: &CouponRedemption) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.coupon_redemptions (
                redemption_id, coupon_id, subscription_id, user_id, redeemed_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(redemption.redemption_id)
        .bind(redemption.coupon_id)
        .bind(redemption.subscription_id)
        .bind(redemption.user_id)
        .bind(redemption.redeemed_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Check if user has redeemed coupon
    pub async fn has_user_redeemed(&self, user_id: Uuid, coupon_id: Uuid) -> Result<bool, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM subscriptions.coupon_redemptions
                WHERE user_id = $1 AND coupon_id = $2
            ) as exists
            "#,
        )
        .bind(user_id)
        .bind(coupon_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get::<bool, _>("exists")?)
    }

    /// Get redemptions for coupon
    pub async fn get_redemptions(&self, coupon_id: Uuid) -> Result<Vec<CouponRedemption>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.coupon_redemptions
            WHERE coupon_id = $1
            ORDER BY redeemed_at DESC
            "#,
        )
        .bind(coupon_id)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| {
            Ok(CouponRedemption {
                redemption_id: r.try_get("redemption_id")?,
                coupon_id: r.try_get("coupon_id")?,
                subscription_id: r.try_get("subscription_id")?,
                user_id: r.try_get("user_id")?,
                redeemed_at: r.try_get("redeemed_at")?,
            })
        }).collect()
    }

    /// Convert row to Coupon
    fn row_to_coupon(&self, row: &sqlx::postgres::PgRow) -> Result<Coupon, sqlx::Error> {
        let discount_type_str: String = row.try_get("discount_type")?;
        let applies_to_plans_str: Option<Vec<String>> = row.try_get("applies_to_plans")?;

        let discount_type = match discount_type_str.as_str() {
            "percentage" => DiscountType::Percentage,
            "fixed_amount" => DiscountType::FixedAmount,
            _ => DiscountType::Percentage,
        };

        let applies_to_plans = applies_to_plans_str.map(|plans| {
            let uuids: Vec<String> = plans.iter()
                .filter_map(|s| Uuid::parse_str(s).ok())
                .map(|u| u.to_string())
                .collect();
            serde_json::json!(uuids)
        });

        Ok(Coupon {
            coupon_id: row.try_get("coupon_id")?,
            code: row.try_get("code")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            discount_type,
            discount_value: row.try_get("discount_value")?,
            max_redemptions: row.try_get("max_redemptions")?,
            current_redemptions: row.try_get("current_redemptions")?,
            valid_from: row.try_get("valid_from")?,
            valid_until: row.try_get("valid_until")?,
            min_amount_cents: row.try_get("min_amount_cents")?,
            applies_to_plans,
            first_time_only: row.try_get("first_time_only")?,
            is_active: row.try_get("is_active")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
        })
    }
}
