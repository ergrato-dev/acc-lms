-- Migration: 003_payments_and_orders.sql
-- Description: Payment processing and order management
-- Author: System
-- Date: 2025-08-08
-- Updated: 2025-12-15 (Schema separation)
--
-- PREREQUISITE: Run 000_schema_setup.sql and 001_initial_schema.sql first
--
-- This migration creates tables in the payments schema:
-- - payments.orders : Purchase orders
-- - payments.transactions : Payment transactions
-- - payments.discount_codes : Promotional codes
-- - payments.reviews : Course reviews (tied to purchases)

-- ========================================
-- PAYMENTS SCHEMA: Orders & Transactions
-- ========================================

-- Order number sequence
CREATE SEQUENCE payments.order_number_seq START 1;

-- Order number generator function
CREATE OR REPLACE FUNCTION payments.generate_order_number()
RETURNS TEXT AS $$
DECLARE
    next_id INTEGER;
BEGIN
    SELECT nextval('payments.order_number_seq') INTO next_id;
    RETURN 'ORD-' || TO_CHAR(NOW(), 'YYYY') || '-' || LPAD(next_id::TEXT, 6, '0');
END;
$$ LANGUAGE plpgsql;

-- Purchase orders
CREATE TABLE payments.orders (
    order_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL, -- References auth.users(user_id)
    course_id UUID NOT NULL, -- References courses.courses(course_id)
    order_number TEXT UNIQUE NOT NULL DEFAULT payments.generate_order_number(),
    status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'paid', 'failed', 'cancelled', 'refunded')),
    subtotal_cents INTEGER NOT NULL,
    tax_cents INTEGER NOT NULL DEFAULT 0,
    discount_cents INTEGER NOT NULL DEFAULT 0,
    total_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    payment_provider TEXT,
    payment_intent_id TEXT,
    discount_code TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_orders_user_id ON payments.orders(user_id);
CREATE INDEX idx_payments_orders_course_id ON payments.orders(course_id);
CREATE INDEX idx_payments_orders_status ON payments.orders(status);
CREATE INDEX idx_payments_orders_created_at ON payments.orders(created_at);

-- Payment transactions
CREATE TABLE payments.transactions (
    transaction_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES payments.orders(order_id),
    provider TEXT NOT NULL,
    provider_transaction_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL CHECK (transaction_type IN ('payment', 'refund', 'chargeback')),
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    provider_fee_cents INTEGER DEFAULT 0,
    metadata JSONB DEFAULT '{}'::jsonb,
    processed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_transactions_order_id ON payments.transactions(order_id);
CREATE INDEX idx_payments_transactions_provider ON payments.transactions(provider);

-- Discount codes
CREATE TABLE payments.discount_codes (
    code_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code TEXT UNIQUE NOT NULL,
    description TEXT,
    discount_type TEXT NOT NULL CHECK (discount_type IN ('percentage', 'fixed_amount')),
    discount_value DECIMAL(10,2) NOT NULL,
    minimum_order_cents INTEGER DEFAULT 0,
    max_uses INTEGER,
    current_uses INTEGER NOT NULL DEFAULT 0,
    valid_from TIMESTAMP NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by UUID NOT NULL, -- References auth.users(user_id)
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_discount_codes_code ON payments.discount_codes(code);
CREATE INDEX idx_payments_discount_codes_active ON payments.discount_codes(is_active, valid_from, valid_until);

-- Course reviews (tied to purchases for verified reviews)
CREATE TABLE payments.reviews (
    review_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL, -- References courses.courses(course_id)
    user_id UUID NOT NULL, -- References auth.users(user_id)
    enrollment_id UUID NOT NULL, -- References enrollments.enrollments(enrollment_id)
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_title TEXT,
    review_text TEXT,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    is_verified_purchase BOOLEAN NOT NULL DEFAULT TRUE,
    helpful_votes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(course_id, user_id)
);

CREATE INDEX idx_payments_reviews_course_id ON payments.reviews(course_id);
CREATE INDEX idx_payments_reviews_user_id ON payments.reviews(user_id);
CREATE INDEX idx_payments_reviews_rating ON payments.reviews(rating);

-- Triggers
CREATE TRIGGER payments_orders_updated_at
    BEFORE UPDATE ON payments.orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER payments_reviews_updated_at
    BEFORE UPDATE ON payments.reviews
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
