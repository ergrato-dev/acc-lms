-- =============================================================================
-- Migration: 010_subscriptions.sql
-- Description: Subscription management tables for LMS billing
-- =============================================================================

-- Set search path to subscriptions schema
SET search_path TO subscriptions, public;

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- ENUM TYPES (in subscriptions schema)
-- =============================================================================

-- Billing interval options
CREATE TYPE subscriptions.billing_interval AS ENUM (
    'daily',
    'weekly',
    'monthly',
    'quarterly',
    'semi_annual',
    'annual',
    'lifetime'
);

-- Plan tier levels
CREATE TYPE subscriptions.plan_tier AS ENUM (
    'free',
    'basic',
    'professional',
    'enterprise',
    'custom'
);

-- Subscription status
CREATE TYPE subscriptions.subscription_status AS ENUM (
    'pending',
    'trialing',
    'active',
    'past_due',
    'cancelled',
    'expired',
    'suspended',
    'incomplete'
);

-- Invoice status
CREATE TYPE subscriptions.invoice_status AS ENUM (
    'draft',
    'pending',
    'open',
    'paid',
    'void',
    'uncollectible'
);

-- Payment method types
CREATE TYPE subscriptions.payment_method_type AS ENUM (
    'card',
    'bank_transfer',
    'paypal',
    'crypto',
    'invoice'
);

-- Card brands
CREATE TYPE subscriptions.card_brand AS ENUM (
    'visa',
    'mastercard',
    'amex',
    'discover',
    'jcb',
    'diners',
    'unionpay',
    'unknown'
);

-- Usage action types
CREATE TYPE subscriptions.usage_action AS ENUM (
    'api_call',
    'storage_upload',
    'video_minutes',
    'ai_tokens',
    'email_sent',
    'student_enrolled',
    'course_created',
    'assessment_graded',
    'certificate_issued'
);

-- Billing event types
CREATE TYPE subscriptions.billing_event_type AS ENUM (
    'subscription_created',
    'subscription_activated',
    'subscription_renewed',
    'subscription_upgraded',
    'subscription_downgraded',
    'subscription_cancelled',
    'subscription_expired',
    'payment_succeeded',
    'payment_failed',
    'invoice_created',
    'invoice_paid',
    'refund_issued',
    'coupon_applied'
);

-- Discount types
CREATE TYPE subscriptions.discount_type AS ENUM (
    'percentage',
    'fixed_amount'
);

-- Coupon duration
CREATE TYPE subscriptions.coupon_duration AS ENUM (
    'once',
    'repeating',
    'forever'
);

-- =============================================================================
-- SUBSCRIPTION PLANS
-- =============================================================================

CREATE TABLE subscriptions.plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    tier plan_tier NOT NULL DEFAULT 'basic',
    billing_interval billing_interval NOT NULL DEFAULT 'monthly',
    price_cents BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    trial_days INTEGER NOT NULL DEFAULT 0,
    features JSONB NOT NULL DEFAULT '[]'::JSONB,
    limits JSONB NOT NULL DEFAULT '{}'::JSONB,
    stripe_price_id VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for plans
CREATE INDEX idx_subscription_plans_tier ON subscriptions.plans(tier);
CREATE INDEX idx_subscription_plans_active ON subscriptions.plans(is_active);
CREATE INDEX idx_subscription_plans_stripe ON subscriptions.plans(stripe_price_id) WHERE stripe_price_id IS NOT NULL;

-- =============================================================================
-- SUBSCRIPTIONS
-- =============================================================================

CREATE TABLE subscriptions.subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    plan_id UUID NOT NULL REFERENCES subscriptions.plans(id),
    status subscription_status NOT NULL DEFAULT 'pending',
    current_period_start TIMESTAMPTZ NOT NULL,
    current_period_end TIMESTAMPTZ NOT NULL,
    trial_start TIMESTAMPTZ,
    trial_end TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    cancel_at_period_end BOOLEAN NOT NULL DEFAULT false,
    stripe_subscription_id VARCHAR(255),
    stripe_customer_id VARCHAR(255),
    payment_method_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for subscriptions
CREATE INDEX idx_subscriptions_user ON subscriptions.subscriptions(user_id);
CREATE INDEX idx_subscriptions_plan ON subscriptions.subscriptions(plan_id);
CREATE INDEX idx_subscriptions_status ON subscriptions.subscriptions(status);
CREATE INDEX idx_subscriptions_user_active ON subscriptions.subscriptions(user_id, status)
    WHERE status IN ('active', 'trialing');
CREATE INDEX idx_subscriptions_stripe ON subscriptions.subscriptions(stripe_subscription_id)
    WHERE stripe_subscription_id IS NOT NULL;
CREATE INDEX idx_subscriptions_period_end ON subscriptions.subscriptions(current_period_end)
    WHERE status = 'active';

-- =============================================================================
-- INVOICES
-- =============================================================================

CREATE TABLE subscriptions.invoices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions.subscriptions(id),
    user_id UUID NOT NULL,
    invoice_number VARCHAR(50) NOT NULL UNIQUE,
    status invoice_status NOT NULL DEFAULT 'draft',
    subtotal_cents BIGINT NOT NULL DEFAULT 0,
    discount_cents BIGINT NOT NULL DEFAULT 0,
    tax_cents BIGINT NOT NULL DEFAULT 0,
    total_cents BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    due_date TIMESTAMPTZ NOT NULL,
    paid_at TIMESTAMPTZ,
    stripe_invoice_id VARCHAR(255),
    stripe_payment_intent_id VARCHAR(255),
    pdf_url TEXT,
    line_items JSONB NOT NULL DEFAULT '[]'::JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for invoices
CREATE INDEX idx_invoices_subscription ON subscriptions.invoices(subscription_id);
CREATE INDEX idx_invoices_user ON subscriptions.invoices(user_id);
CREATE INDEX idx_invoices_status ON subscriptions.invoices(status);
CREATE INDEX idx_invoices_due_date ON subscriptions.invoices(due_date) WHERE status IN ('pending', 'open');
CREATE INDEX idx_invoices_stripe ON subscriptions.invoices(stripe_invoice_id) WHERE stripe_invoice_id IS NOT NULL;

-- =============================================================================
-- PAYMENT METHODS
-- =============================================================================

CREATE TABLE subscriptions.payment_methods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    method_type payment_method_type NOT NULL DEFAULT 'card',
    is_default BOOLEAN NOT NULL DEFAULT false,
    card_brand card_brand,
    card_last4 VARCHAR(4),
    card_exp_month INTEGER,
    card_exp_year INTEGER,
    billing_address JSONB,
    stripe_payment_method_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for payment methods
CREATE INDEX idx_payment_methods_user ON subscriptions.payment_methods(user_id);
CREATE INDEX idx_payment_methods_default ON subscriptions.payment_methods(user_id, is_default) WHERE is_default = true;
CREATE INDEX idx_payment_methods_stripe ON subscriptions.payment_methods(stripe_payment_method_id)
    WHERE stripe_payment_method_id IS NOT NULL;

-- =============================================================================
-- USAGE RECORDS
-- =============================================================================

CREATE TABLE subscriptions.usage_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions.subscriptions(id),
    user_id UUID NOT NULL,
    action usage_action NOT NULL,
    quantity BIGINT NOT NULL DEFAULT 1,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for usage records (partitioned by time for large datasets)
CREATE INDEX idx_usage_records_subscription ON subscriptions.usage_records(subscription_id);
CREATE INDEX idx_usage_records_user ON subscriptions.usage_records(user_id);
CREATE INDEX idx_usage_records_action ON subscriptions.usage_records(action);
CREATE INDEX idx_usage_records_time ON subscriptions.usage_records(subscription_id, recorded_at);

-- =============================================================================
-- BILLING EVENTS (Audit trail)
-- =============================================================================

CREATE TABLE subscriptions.billing_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscription_id UUID NOT NULL REFERENCES subscriptions.subscriptions(id),
    event_type billing_event_type NOT NULL,
    description TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for billing events
CREATE INDEX idx_billing_events_subscription ON subscriptions.billing_events(subscription_id);
CREATE INDEX idx_billing_events_type ON subscriptions.billing_events(event_type);
CREATE INDEX idx_billing_events_time ON subscriptions.billing_events(created_at);

-- =============================================================================
-- COUPONS
-- =============================================================================

CREATE TABLE subscriptions.coupons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    discount_type discount_type NOT NULL DEFAULT 'percentage',
    discount_value DECIMAL(10,2) NOT NULL,
    duration coupon_duration NOT NULL DEFAULT 'once',
    duration_months INTEGER,
    max_redemptions INTEGER,
    times_redeemed INTEGER NOT NULL DEFAULT 0,
    applies_to_plans UUID[] DEFAULT '{}',
    min_amount_cents BIGINT,
    valid_from TIMESTAMPTZ,
    valid_until TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    stripe_coupon_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for coupons
CREATE UNIQUE INDEX idx_coupons_code_upper ON subscriptions.coupons(UPPER(code));
CREATE INDEX idx_coupons_active ON subscriptions.coupons(is_active) WHERE is_active = true;
CREATE INDEX idx_coupons_validity ON subscriptions.coupons(valid_from, valid_until);

-- =============================================================================
-- COUPON REDEMPTIONS
-- =============================================================================

CREATE TABLE subscriptions.coupon_redemptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    coupon_id UUID NOT NULL REFERENCES subscriptions.coupons(id),
    user_id UUID NOT NULL,
    subscription_id UUID REFERENCES subscriptions.subscriptions(id),
    discount_applied_cents BIGINT NOT NULL,
    redeemed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for coupon redemptions
CREATE INDEX idx_coupon_redemptions_coupon ON subscriptions.coupon_redemptions(coupon_id);
CREATE INDEX idx_coupon_redemptions_user ON subscriptions.coupon_redemptions(user_id);
CREATE UNIQUE INDEX idx_coupon_redemptions_unique ON subscriptions.coupon_redemptions(coupon_id, user_id);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_subscription_plans_updated_at
    BEFORE UPDATE ON subscriptions.plans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_subscriptions_updated_at
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_invoices_updated_at
    BEFORE UPDATE ON subscriptions.invoices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_payment_methods_updated_at
    BEFORE UPDATE ON subscriptions.payment_methods
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_coupons_updated_at
    BEFORE UPDATE ON subscriptions.coupons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- SEED DATA: Default Plans
-- =============================================================================

INSERT INTO subscriptions.plans (id, name, description, tier, billing_interval, price_cents, currency, trial_days, features, limits, display_order) VALUES
-- Free tier
(
    uuid_generate_v4(),
    'Free',
    'Perfect for trying out the platform',
    'free',
    'monthly',
    0,
    'USD',
    0,
    '[
        {"key": "courses", "name": "Course Creation", "value": "Up to 1 course", "included": true},
        {"key": "students", "name": "Students", "value": "Up to 10 students", "included": true},
        {"key": "storage", "name": "Storage", "value": "100MB", "included": true},
        {"key": "support", "name": "Support", "value": "Community", "included": true}
    ]'::JSONB,
    '{"max_courses": 1, "max_students": 10, "max_storage_gb": 0, "support_level": "community", "custom_branding": false}'::JSONB,
    1
),
-- Basic tier
(
    uuid_generate_v4(),
    'Basic',
    'Great for individual instructors',
    'basic',
    'monthly',
    2900,
    'USD',
    14,
    '[
        {"key": "courses", "name": "Course Creation", "value": "Up to 5 courses", "included": true},
        {"key": "students", "name": "Students", "value": "Up to 100 students", "included": true},
        {"key": "storage", "name": "Storage", "value": "5GB", "included": true},
        {"key": "video", "name": "Video Hosting", "value": "10 hours", "included": true},
        {"key": "support", "name": "Support", "value": "Email", "included": true},
        {"key": "analytics", "name": "Analytics", "value": "Basic", "included": true}
    ]'::JSONB,
    '{"max_courses": 5, "max_students": 100, "max_storage_gb": 5, "max_video_hours": 10, "support_level": "email", "custom_branding": false, "analytics_retention_days": 30}'::JSONB,
    2
),
-- Professional tier
(
    uuid_generate_v4(),
    'Professional',
    'For growing education businesses',
    'professional',
    'monthly',
    7900,
    'USD',
    14,
    '[
        {"key": "courses", "name": "Course Creation", "value": "Unlimited courses", "included": true},
        {"key": "students", "name": "Students", "value": "Up to 1,000 students", "included": true},
        {"key": "storage", "name": "Storage", "value": "50GB", "included": true},
        {"key": "video", "name": "Video Hosting", "value": "100 hours", "included": true},
        {"key": "support", "name": "Support", "value": "Priority", "included": true},
        {"key": "analytics", "name": "Analytics", "value": "Advanced", "included": true},
        {"key": "branding", "name": "Custom Branding", "value": "Included", "included": true},
        {"key": "api", "name": "API Access", "value": "Included", "included": true}
    ]'::JSONB,
    '{"max_courses": null, "max_students": 1000, "max_storage_gb": 50, "max_video_hours": 100, "max_instructors": 5, "api_rate_limit": 1000, "support_level": "priority", "custom_branding": true, "analytics_retention_days": 90}'::JSONB,
    3
),
-- Enterprise tier
(
    uuid_generate_v4(),
    'Enterprise',
    'For large organizations and institutions',
    'enterprise',
    'monthly',
    29900,
    'USD',
    30,
    '[
        {"key": "courses", "name": "Course Creation", "value": "Unlimited", "included": true},
        {"key": "students", "name": "Students", "value": "Unlimited", "included": true},
        {"key": "storage", "name": "Storage", "value": "500GB", "included": true},
        {"key": "video", "name": "Video Hosting", "value": "Unlimited", "included": true},
        {"key": "support", "name": "Support", "value": "Dedicated", "included": true},
        {"key": "analytics", "name": "Analytics", "value": "Enterprise", "included": true},
        {"key": "branding", "name": "Custom Branding", "value": "Full White-label", "included": true},
        {"key": "api", "name": "API Access", "value": "Unlimited", "included": true},
        {"key": "sso", "name": "SSO/SAML", "value": "Included", "included": true},
        {"key": "sla", "name": "SLA", "value": "99.9% uptime", "included": true}
    ]'::JSONB,
    '{"max_courses": null, "max_students": null, "max_storage_gb": 500, "max_video_hours": null, "max_instructors": null, "api_rate_limit": 10000, "support_level": "dedicated", "custom_branding": true, "analytics_retention_days": 365}'::JSONB,
    4
);

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE subscriptions.plans IS 'Available subscription plans with pricing and features';
COMMENT ON TABLE subscriptions.subscriptions IS 'User subscriptions to plans';
COMMENT ON TABLE subscriptions.invoices IS 'Billing invoices for subscriptions';
COMMENT ON TABLE subscriptions.payment_methods IS 'User payment methods (cards, bank accounts, etc.)';
COMMENT ON TABLE subscriptions.usage_records IS 'Usage tracking for metered billing';
COMMENT ON TABLE subscriptions.billing_events IS 'Audit trail for billing events';
COMMENT ON TABLE subscriptions.coupons IS 'Discount codes and promotions';
COMMENT ON TABLE subscriptions.coupon_redemptions IS 'Track coupon usage by users';
