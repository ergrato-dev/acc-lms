// =============================================================================
// Domain Entities - Subscription Service
// =============================================================================
// Core domain types for subscription management
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// Subscription Plan
// =============================================================================

/// Billing interval for subscription plans
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BillingInterval {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    Lifetime,
}

impl BillingInterval {
    /// Returns the number of days in this billing interval (approximate)
    pub fn days(&self) -> Option<i64> {
        match self {
            BillingInterval::Daily => Some(1),
            BillingInterval::Weekly => Some(7),
            BillingInterval::Monthly => Some(30),
            BillingInterval::Quarterly => Some(90),
            BillingInterval::SemiAnnual => Some(180),
            BillingInterval::Annual => Some(365),
            BillingInterval::Lifetime => None, // No recurrence
        }
    }
}

impl std::fmt::Display for BillingInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillingInterval::Daily => write!(f, "daily"),
            BillingInterval::Weekly => write!(f, "weekly"),
            BillingInterval::Monthly => write!(f, "monthly"),
            BillingInterval::Quarterly => write!(f, "quarterly"),
            BillingInterval::SemiAnnual => write!(f, "semi_annual"),
            BillingInterval::Annual => write!(f, "annual"),
            BillingInterval::Lifetime => write!(f, "lifetime"),
        }
    }
}

impl std::str::FromStr for BillingInterval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "daily" => Ok(BillingInterval::Daily),
            "weekly" => Ok(BillingInterval::Weekly),
            "monthly" => Ok(BillingInterval::Monthly),
            "quarterly" => Ok(BillingInterval::Quarterly),
            "semi_annual" => Ok(BillingInterval::SemiAnnual),
            "annual" => Ok(BillingInterval::Annual),
            "lifetime" => Ok(BillingInterval::Lifetime),
            _ => Err(()),
        }
    }
}

/// Plan tier/category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PlanTier {
    Free,
    Basic,
    Professional,
    Enterprise,
    Custom,
}

impl std::fmt::Display for PlanTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanTier::Free => write!(f, "free"),
            PlanTier::Basic => write!(f, "basic"),
            PlanTier::Professional => write!(f, "professional"),
            PlanTier::Enterprise => write!(f, "enterprise"),
            PlanTier::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for PlanTier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free" => Ok(PlanTier::Free),
            "basic" => Ok(PlanTier::Basic),
            "professional" => Ok(PlanTier::Professional),
            "enterprise" => Ok(PlanTier::Enterprise),
            "custom" => Ok(PlanTier::Custom),
            _ => Err(()),
        }
    }
}

/// Subscription plan definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
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
    pub stripe_price_id: Option<String>,
    pub is_active: bool,
    pub is_public: bool,
    pub sort_order: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Feature included in a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanFeature {
    pub feature_id: Uuid,
    pub plan_id: Uuid,
    pub feature_key: String,
    pub feature_name: String,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub limit_value: Option<i64>,
    pub limit_unit: Option<String>,
}

/// Resource limits for a plan
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanLimits {
    pub max_courses: Option<i32>,
    pub max_students: Option<i32>,
    pub max_storage_mb: Option<i64>,
    pub max_video_hours: Option<i32>,
    pub max_instructors: Option<i32>,
    pub max_certificates: Option<i32>,
    pub api_rate_limit: Option<i32>,
    pub support_level: Option<String>,
    pub custom_domain: bool,
    pub white_label: bool,
    pub analytics_retention_days: Option<i32>,
}

// =============================================================================
// Subscription
// =============================================================================

/// Subscription status lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription created but not yet active
    Pending,
    /// In trial period
    Trialing,
    /// Active and paid
    Active,
    /// Payment failed, grace period
    PastDue,
    /// User cancelled, still active until period end
    Cancelled,
    /// Subscription ended (after cancellation period)
    Expired,
    /// Permanently suspended (e.g., fraud)
    Suspended,
    /// Incomplete setup (payment method needed)
    Incomplete,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionStatus::Pending => write!(f, "pending"),
            SubscriptionStatus::Trialing => write!(f, "trialing"),
            SubscriptionStatus::Active => write!(f, "active"),
            SubscriptionStatus::PastDue => write!(f, "past_due"),
            SubscriptionStatus::Cancelled => write!(f, "cancelled"),
            SubscriptionStatus::Expired => write!(f, "expired"),
            SubscriptionStatus::Suspended => write!(f, "suspended"),
            SubscriptionStatus::Incomplete => write!(f, "incomplete"),
        }
    }
}

impl std::str::FromStr for SubscriptionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(SubscriptionStatus::Pending),
            "trialing" => Ok(SubscriptionStatus::Trialing),
            "active" => Ok(SubscriptionStatus::Active),
            "past_due" => Ok(SubscriptionStatus::PastDue),
            "cancelled" => Ok(SubscriptionStatus::Cancelled),
            "expired" => Ok(SubscriptionStatus::Expired),
            "suspended" => Ok(SubscriptionStatus::Suspended),
            "incomplete" => Ok(SubscriptionStatus::Incomplete),
            _ => Err(()),
        }
    }
}

/// Cancellation reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CancellationReason {
    UserRequested,
    PaymentFailed,
    Fraud,
    PolicyViolation,
    DowngradeToPlan,
    Migration,
    Other,
}

impl std::fmt::Display for CancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CancellationReason::UserRequested => write!(f, "user_requested"),
            CancellationReason::PaymentFailed => write!(f, "payment_failed"),
            CancellationReason::Fraud => write!(f, "fraud"),
            CancellationReason::PolicyViolation => write!(f, "policy_violation"),
            CancellationReason::DowngradeToPlan => write!(f, "downgrade_to_plan"),
            CancellationReason::Migration => write!(f, "migration"),
            CancellationReason::Other => write!(f, "other"),
        }
    }
}

/// User subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
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
    pub cancellation_reason: Option<CancellationReason>,
    pub cancellation_feedback: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub quantity: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    /// Check if subscription is currently active (including trial)
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            SubscriptionStatus::Active | SubscriptionStatus::Trialing | SubscriptionStatus::PastDue
        )
    }

    /// Check if in trial period
    pub fn is_trialing(&self) -> bool {
        self.status == SubscriptionStatus::Trialing
    }

    /// Check if subscription has access (active or cancelled but not yet expired)
    pub fn has_access(&self) -> bool {
        match self.status {
            SubscriptionStatus::Active | SubscriptionStatus::Trialing | SubscriptionStatus::PastDue => true,
            SubscriptionStatus::Cancelled => {
                // Still has access until period end
                self.current_period_end > Utc::now()
            }
            _ => false,
        }
    }

    /// Days remaining in current period
    pub fn days_remaining(&self) -> i64 {
        let now = Utc::now();
        if self.current_period_end > now {
            (self.current_period_end - now).num_days()
        } else {
            0
        }
    }
}

// =============================================================================
// Invoice
// =============================================================================

/// Invoice status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceStatus::Draft => write!(f, "draft"),
            InvoiceStatus::Open => write!(f, "open"),
            InvoiceStatus::Paid => write!(f, "paid"),
            InvoiceStatus::Void => write!(f, "void"),
            InvoiceStatus::Uncollectible => write!(f, "uncollectible"),
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(InvoiceStatus::Draft),
            "open" => Ok(InvoiceStatus::Open),
            "paid" => Ok(InvoiceStatus::Paid),
            "void" => Ok(InvoiceStatus::Void),
            "uncollectible" => Ok(InvoiceStatus::Uncollectible),
            _ => Err(()),
        }
    }
}

/// Subscription invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub user_id: Uuid,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub billing_period_start: DateTime<Utc>,
    pub billing_period_end: DateTime<Utc>,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub discount_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub stripe_invoice_id: Option<String>,
    pub pdf_url: Option<String>,
    pub hosted_invoice_url: Option<String>,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub line_item_id: Uuid,
    pub invoice_id: Uuid,
    pub description: String,
    pub quantity: i32,
    pub unit_price_cents: i32,
    pub total_cents: i32,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub proration: bool,
}

// =============================================================================
// Payment Method
// =============================================================================

/// Payment method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    BankAccount,
    Sepa,
    Ideal,
    Boleto,
    Pix,
    Oxxo,
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethodType::Card => write!(f, "card"),
            PaymentMethodType::BankAccount => write!(f, "bank_account"),
            PaymentMethodType::Sepa => write!(f, "sepa"),
            PaymentMethodType::Ideal => write!(f, "ideal"),
            PaymentMethodType::Boleto => write!(f, "boleto"),
            PaymentMethodType::Pix => write!(f, "pix"),
            PaymentMethodType::Oxxo => write!(f, "oxxo"),
        }
    }
}

/// Card brand
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Discover,
    Diners,
    Jcb,
    UnionPay,
    Unknown,
}

impl std::fmt::Display for CardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardBrand::Visa => write!(f, "visa"),
            CardBrand::Mastercard => write!(f, "mastercard"),
            CardBrand::Amex => write!(f, "amex"),
            CardBrand::Discover => write!(f, "discover"),
            CardBrand::Diners => write!(f, "diners"),
            CardBrand::Jcb => write!(f, "jcb"),
            CardBrand::UnionPay => write!(f, "unionpay"),
            CardBrand::Unknown => write!(f, "unknown"),
        }
    }
}

/// User payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub payment_method_id: Uuid,
    pub user_id: Uuid,
    pub method_type: PaymentMethodType,
    pub stripe_payment_method_id: Option<String>,
    pub is_default: bool,
    pub card_brand: Option<CardBrand>,
    pub card_last_four: Option<String>,
    pub card_exp_month: Option<i32>,
    pub card_exp_year: Option<i32>,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Billing address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
}

// =============================================================================
// Usage & Metering
// =============================================================================

/// Usage record for metered billing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub record_id: Uuid,
    pub subscription_id: Uuid,
    pub feature_key: String,
    pub quantity: i64,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub metadata: serde_json::Value,
}

/// Usage action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UsageAction {
    Set,
    Increment,
}

/// Usage summary for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub subscription_id: Uuid,
    pub feature_key: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_usage: i64,
    pub limit: Option<i64>,
    pub percentage_used: f64,
    pub overage: i64,
}

// =============================================================================
// Billing Events
// =============================================================================

/// Billing event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BillingEventType {
    SubscriptionCreated,
    SubscriptionActivated,
    SubscriptionTrialStarted,
    SubscriptionTrialEnded,
    SubscriptionRenewed,
    SubscriptionUpgraded,
    SubscriptionDowngraded,
    SubscriptionCancelled,
    SubscriptionExpired,
    SubscriptionReactivated,
    PaymentSucceeded,
    PaymentFailed,
    PaymentRefunded,
    InvoiceCreated,
    InvoicePaid,
    InvoiceVoided,
    PaymentMethodAdded,
    PaymentMethodUpdated,
    PaymentMethodRemoved,
}

impl std::fmt::Display for BillingEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillingEventType::SubscriptionCreated => write!(f, "subscription_created"),
            BillingEventType::SubscriptionActivated => write!(f, "subscription_activated"),
            BillingEventType::SubscriptionTrialStarted => write!(f, "subscription_trial_started"),
            BillingEventType::SubscriptionTrialEnded => write!(f, "subscription_trial_ended"),
            BillingEventType::SubscriptionRenewed => write!(f, "subscription_renewed"),
            BillingEventType::SubscriptionUpgraded => write!(f, "subscription_upgraded"),
            BillingEventType::SubscriptionDowngraded => write!(f, "subscription_downgraded"),
            BillingEventType::SubscriptionCancelled => write!(f, "subscription_cancelled"),
            BillingEventType::SubscriptionExpired => write!(f, "subscription_expired"),
            BillingEventType::SubscriptionReactivated => write!(f, "subscription_reactivated"),
            BillingEventType::PaymentSucceeded => write!(f, "payment_succeeded"),
            BillingEventType::PaymentFailed => write!(f, "payment_failed"),
            BillingEventType::PaymentRefunded => write!(f, "payment_refunded"),
            BillingEventType::InvoiceCreated => write!(f, "invoice_created"),
            BillingEventType::InvoicePaid => write!(f, "invoice_paid"),
            BillingEventType::InvoiceVoided => write!(f, "invoice_voided"),
            BillingEventType::PaymentMethodAdded => write!(f, "payment_method_added"),
            BillingEventType::PaymentMethodUpdated => write!(f, "payment_method_updated"),
            BillingEventType::PaymentMethodRemoved => write!(f, "payment_method_removed"),
        }
    }
}

/// Billing event for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingEvent {
    pub event_id: Uuid,
    pub subscription_id: Option<Uuid>,
    pub user_id: Uuid,
    pub event_type: BillingEventType,
    pub description: String,
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
    pub invoice_id: Option<Uuid>,
    pub stripe_event_id: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// Coupon / Promo Code
// =============================================================================

/// Coupon discount type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Percentage,
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

/// Coupon duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CouponDuration {
    Once,
    Repeating,
    Forever,
}

/// Subscription coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
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
    pub min_amount_cents: Option<i64>,
    pub applies_to_plans: Option<serde_json::Value>,
    pub first_time_only: bool,
    pub is_active: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl Coupon {
    /// Check if coupon is currently valid
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        self.is_active
            && self.valid_from.map_or(true, |from| now >= from)
            && self.valid_until.map_or(true, |until| now <= until)
            && self.max_redemptions.map_or(true, |max| self.current_redemptions < max)
    }

    /// Calculate discount for a given amount
    pub fn calculate_discount(&self, amount_cents: i32) -> i32 {
        match self.discount_type {
            DiscountType::Percentage => (amount_cents * self.discount_value) / 100,
            DiscountType::FixedAmount => self.discount_value.min(amount_cents),
        }
    }
}

/// Coupon redemption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponRedemption {
    pub redemption_id: Uuid,
    pub coupon_id: Uuid,
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub redeemed_at: DateTime<Utc>,
}

// =============================================================================
// Subscription Stats
// =============================================================================

/// Subscription service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStats {
    pub total_subscriptions: i64,
    pub active_subscriptions: i64,
    pub trialing_subscriptions: i64,
    pub cancelled_subscriptions: i64,
    pub past_due_subscriptions: i64,
    pub mrr_cents: i64,
    pub arr_cents: i64,
    pub average_revenue_per_user_cents: i64,
    pub churn_rate_percent: f64,
    pub trial_conversion_rate_percent: f64,
    pub subscriptions_by_plan: Vec<PlanSubscriptionCount>,
    pub revenue_by_interval: Vec<RevenueByInterval>,
}

/// Subscription count per plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSubscriptionCount {
    pub plan_id: Uuid,
    pub plan_name: String,
    pub count: i64,
    pub percentage: f64,
}

/// Revenue breakdown by billing interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueByInterval {
    pub interval: BillingInterval,
    pub total_cents: i64,
    pub subscription_count: i64,
}

// =============================================================================
// Customer Portal
// =============================================================================

/// Customer portal session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSession {
    pub session_id: String,
    pub url: String,
    pub return_url: String,
    pub expires_at: DateTime<Utc>,
}

/// Checkout session for new subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub session_id: String,
    pub url: String,
    pub expires_at: DateTime<Utc>,
    pub plan_id: Uuid,
    pub user_id: Uuid,
    pub success_url: String,
    pub cancel_url: String,
}

// =============================================================================
// Plan Comparison
// =============================================================================

/// Feature comparison between plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanComparison {
    pub features: Vec<FeatureComparison>,
    pub plans: Vec<PlanSummary>,
}

/// Single feature comparison across plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureComparison {
    pub feature_key: String,
    pub feature_name: String,
    pub category: String,
    pub values: Vec<FeatureValue>,
}

/// Feature value for a specific plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureValue {
    pub plan_id: Uuid,
    pub is_included: bool,
    pub limit_value: Option<i64>,
    pub display_value: String,
}

/// Plan summary for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSummary {
    pub plan_id: Uuid,
    pub name: String,
    pub tier: PlanTier,
    pub price_cents: i32,
    pub currency: String,
    pub billing_interval: BillingInterval,
    pub is_popular: bool,
}
