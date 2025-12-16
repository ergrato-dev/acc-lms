// =============================================================================
// Data Transfer Objects (DTOs) - API request/response structures
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{
    BillingEvent, BillingInterval, Coupon, DiscountType, Invoice, InvoiceLineItem, InvoiceStatus,
    PaymentMethod, PlanFeature, PlanLimits, PlanTier, Subscription, SubscriptionPlan,
    SubscriptionStatus, UsageRecord,
};

// =============================================================================
// Generic API Response
// =============================================================================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

// =============================================================================
// Plan DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct PlanResponse {
    pub plan_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub tier: PlanTier,
    pub billing_interval: BillingInterval,
    pub price_cents: i32,
    pub currency: String,
    pub trial_days: i32,
    pub features: Vec<PlanFeature>,
    pub limits: PlanLimits,
    pub is_active: bool,
    pub is_public: bool,
}

impl From<SubscriptionPlan> for PlanResponse {
    fn from(p: SubscriptionPlan) -> Self {
        Self {
            plan_id: p.plan_id,
            name: p.name,
            slug: p.slug,
            description: p.description,
            tier: p.tier,
            billing_interval: p.billing_interval,
            price_cents: p.price_cents,
            currency: p.currency,
            trial_days: p.trial_days,
            features: p.features,
            limits: p.limits,
            is_active: p.is_active,
            is_public: p.is_public,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub tier: PlanTier,
    pub billing_interval: BillingInterval,
    pub price_cents: i32,
    pub currency: String,
    pub trial_days: Option<i32>,
    pub features: Vec<PlanFeature>,
    pub limits: PlanLimits,
    pub stripe_price_id: Option<String>,
    pub is_public: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_cents: Option<i32>,
    pub trial_days: Option<i32>,
    pub features: Option<Vec<PlanFeature>>,
    pub limits: Option<PlanLimits>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub sort_order: Option<i32>,
}

// =============================================================================
// Subscription DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

impl From<Subscription> for SubscriptionResponse {
    fn from(s: Subscription) -> Self {
        Self {
            subscription_id: s.subscription_id,
            user_id: s.user_id,
            organization_id: s.organization_id,
            plan_id: s.plan_id,
            status: s.status,
            current_period_start: s.current_period_start,
            current_period_end: s.current_period_end,
            trial_start: s.trial_start,
            trial_end: s.trial_end,
            cancelled_at: s.cancelled_at,
            cancel_at_period_end: s.cancel_at_period_end,
            quantity: s.quantity,
            created_at: s.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: Uuid,
    pub organization_id: Option<Uuid>,
    pub coupon_code: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePlanRequest {
    pub new_plan_id: Uuid,
    pub prorate: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    pub reason: Option<String>,
    pub feedback: Option<String>,
    pub cancel_immediately: Option<bool>,
}

// =============================================================================
// Invoice DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
    pub invoice_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub discount_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub billing_period_start: DateTime<Utc>,
    pub billing_period_end: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Invoice> for InvoiceResponse {
    fn from(i: Invoice) -> Self {
        Self {
            invoice_id: i.invoice_id,
            subscription_id: i.subscription_id,
            invoice_number: i.invoice_number,
            status: i.status,
            subtotal_cents: i.subtotal_cents,
            tax_cents: i.tax_cents,
            discount_cents: i.discount_cents,
            total_cents: i.total_cents,
            currency: i.currency,
            billing_period_start: i.billing_period_start,
            billing_period_end: i.billing_period_end,
            due_date: i.due_date,
            paid_at: i.paid_at,
            pdf_url: i.pdf_url,
            created_at: i.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InvoiceLineItemResponse {
    pub line_item_id: Uuid,
    pub description: String,
    pub quantity: i32,
    pub unit_price_cents: i32,
    pub total_cents: i32,
}

impl From<InvoiceLineItem> for InvoiceLineItemResponse {
    fn from(item: InvoiceLineItem) -> Self {
        Self {
            line_item_id: item.line_item_id,
            description: item.description,
            quantity: item.quantity,
            unit_price_cents: item.unit_price_cents,
            total_cents: item.total_cents,
        }
    }
}

// =============================================================================
// Payment Method DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct PaymentMethodResponse {
    pub payment_method_id: Uuid,
    pub method_type: String,
    pub card_brand: Option<String>,
    pub card_last_four: Option<String>,
    pub card_exp_month: Option<i32>,
    pub card_exp_year: Option<i32>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

impl From<PaymentMethod> for PaymentMethodResponse {
    fn from(pm: PaymentMethod) -> Self {
        Self {
            payment_method_id: pm.payment_method_id,
            method_type: pm.method_type.to_string(),
            card_brand: pm.card_brand.map(|b| b.to_string()),
            card_last_four: pm.card_last_four,
            card_exp_month: pm.card_exp_month,
            card_exp_year: pm.card_exp_year,
            is_default: pm.is_default,
            created_at: pm.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddPaymentMethodRequest {
    pub stripe_payment_method_id: String,
    pub set_as_default: Option<bool>,
}

// =============================================================================
// Usage DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct UsageRecordResponse {
    pub record_id: Uuid,
    pub feature_key: String,
    pub quantity: i64,
    pub timestamp: DateTime<Utc>,
    pub action: String,
}

impl From<UsageRecord> for UsageRecordResponse {
    fn from(r: UsageRecord) -> Self {
        Self {
            record_id: r.record_id,
            feature_key: r.feature_key,
            quantity: r.quantity,
            timestamp: r.timestamp,
            action: r.action,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RecordUsageRequest {
    pub subscription_id: Uuid,
    pub feature_key: String,
    pub quantity: i64,
    pub action: Option<String>,
}

// =============================================================================
// Coupon DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct CouponResponse {
    pub coupon_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: i32,
    pub max_redemptions: Option<i32>,
    pub current_redemptions: i32,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl From<Coupon> for CouponResponse {
    fn from(c: Coupon) -> Self {
        Self {
            coupon_id: c.coupon_id,
            code: c.code,
            name: c.name,
            description: c.description,
            discount_type: c.discount_type,
            discount_value: c.discount_value,
            max_redemptions: c.max_redemptions,
            current_redemptions: c.current_redemptions,
            valid_from: c.valid_from,
            valid_until: c.valid_until,
            is_active: c.is_active,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: i32,
    pub max_redemptions: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub applies_to_plans: Option<Vec<Uuid>>,
    pub first_time_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateCouponRequest {
    pub code: String,
    pub plan_id: Uuid,
}

// =============================================================================
// Billing Event DTOs
// =============================================================================

#[derive(Debug, Serialize)]
pub struct BillingEventResponse {
    pub event_id: Uuid,
    pub event_type: String,
    pub description: String,
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<BillingEvent> for BillingEventResponse {
    fn from(e: BillingEvent) -> Self {
        Self {
            event_id: e.event_id,
            event_type: e.event_type.to_string(),
            description: e.description,
            amount_cents: e.amount_cents,
            currency: e.currency,
            created_at: e.created_at,
        }
    }
}

// =============================================================================
// Pagination
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationQuery {
    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(20).min(100)
    }

    pub fn offset(&self) -> i64 {
        (self.page.unwrap_or(1) - 1).max(0) * self.limit()
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, query: &PaginationQuery) -> Self {
        let per_page = query.limit();
        let page = query.page.unwrap_or(1);
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
