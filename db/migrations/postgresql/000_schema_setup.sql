-- Migration: 000_schema_setup.sql
-- Description: Schema separation and service users setup
-- Author: System
-- Date: 2025-12-15
--
-- This migration implements Schema-per-Service pattern for microservices architecture.
-- Each service gets its own PostgreSQL schema with a dedicated database user.
--
-- Benefits:
-- - Logical isolation between services
-- - Principle of least privilege (each service accesses only its schema)
-- - Single PostgreSQL instance (reduced operational complexity)
-- - ACID transactions when needed across schemas
-- - Clear ownership boundaries

-- ========================================
-- CLEANUP (for re-runs in development)
-- ========================================

-- Drop users if exist (for idempotency)
DO $$
BEGIN
    -- Revoke all and drop users if they exist
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'auth_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM auth_svc;
        DROP OWNED BY auth_svc CASCADE;
        DROP USER auth_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'users_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM users_svc;
        DROP OWNED BY users_svc CASCADE;
        DROP USER users_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'courses_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM courses_svc;
        DROP OWNED BY courses_svc CASCADE;
        DROP USER courses_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'enrollments_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM enrollments_svc;
        DROP OWNED BY enrollments_svc CASCADE;
        DROP USER enrollments_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'assessments_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM assessments_svc;
        DROP OWNED BY assessments_svc CASCADE;
        DROP USER assessments_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'payments_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM payments_svc;
        DROP OWNED BY payments_svc CASCADE;
        DROP USER payments_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ai_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM ai_svc;
        DROP OWNED BY ai_svc CASCADE;
        DROP USER ai_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'notifications_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM notifications_svc;
        DROP OWNED BY notifications_svc CASCADE;
        DROP USER notifications_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'analytics_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM analytics_svc;
        DROP OWNED BY analytics_svc CASCADE;
        DROP USER analytics_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'chatbot_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM chatbot_svc;
        DROP OWNED BY chatbot_svc CASCADE;
        DROP USER chatbot_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'content_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM content_svc;
        DROP OWNED BY content_svc CASCADE;
        DROP USER content_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'compliance_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM compliance_svc;
        DROP OWNED BY compliance_svc CASCADE;
        DROP USER compliance_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'kb_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM kb_svc;
        DROP OWNED BY kb_svc CASCADE;
        DROP USER kb_svc;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'subscriptions_svc') THEN
        REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM subscriptions_svc;
        DROP OWNED BY subscriptions_svc CASCADE;
        DROP USER subscriptions_svc;
    END IF;
END $$;

-- ========================================
-- CREATE SCHEMAS
-- ========================================

-- Auth schema: authentication, tokens, sessions
CREATE SCHEMA IF NOT EXISTS auth;
COMMENT ON SCHEMA auth IS 'Authentication domain: users, tokens, sessions';

-- Users schema: user profiles, preferences, stats
CREATE SCHEMA IF NOT EXISTS users;
COMMENT ON SCHEMA users IS 'Users domain: profiles, preferences, statistics';

-- Courses schema: courses, sections, lessons, categories
CREATE SCHEMA IF NOT EXISTS courses;
COMMENT ON SCHEMA courses IS 'Courses domain: course content, sections, lessons';

-- Enrollments schema: enrollments, progress tracking
CREATE SCHEMA IF NOT EXISTS enrollments;
COMMENT ON SCHEMA enrollments IS 'Enrollments domain: student enrollments, lesson progress';

-- Assessments schema: quizzes, assignments, grades
CREATE SCHEMA IF NOT EXISTS assessments;
COMMENT ON SCHEMA assessments IS 'Assessments domain: quizzes, questions, submissions, grades';

-- Payments schema: orders, transactions, invoices
CREATE SCHEMA IF NOT EXISTS payments;
COMMENT ON SCHEMA payments IS 'Payments domain: orders, transactions, discounts, reviews';

-- AI schema: embeddings, conversations, AI features
CREATE SCHEMA IF NOT EXISTS ai;
COMMENT ON SCHEMA ai IS 'AI domain: embeddings, conversations, AI-powered features';

-- Notifications schema: templates, user notifications
CREATE SCHEMA IF NOT EXISTS notifications;
COMMENT ON SCHEMA notifications IS 'Notifications domain: templates, delivery queue';

-- Analytics schema: events, metrics, dashboards
CREATE SCHEMA IF NOT EXISTS analytics;
COMMENT ON SCHEMA analytics IS 'Analytics domain: events, metrics, reporting';

-- Chatbot schema: conversations, messages, AI interactions
CREATE SCHEMA IF NOT EXISTS chatbot;
COMMENT ON SCHEMA chatbot IS 'Chatbot domain: AI conversations, messages';

-- Content schema: media, files, assets management
CREATE SCHEMA IF NOT EXISTS content;
COMMENT ON SCHEMA content IS 'Content domain: media files, assets, storage';

-- Compliance schema: GDPR, data rights, consent management
CREATE SCHEMA IF NOT EXISTS compliance;
COMMENT ON SCHEMA compliance IS 'Compliance domain: GDPR, CCPA, data rights, consent';

-- Knowledge Base schema: articles, FAQs, documentation
CREATE SCHEMA IF NOT EXISTS kb;
COMMENT ON SCHEMA kb IS 'Knowledge Base domain: help articles, FAQs, docs';

-- Subscriptions schema: plans, billing, invoices
CREATE SCHEMA IF NOT EXISTS subscriptions;
COMMENT ON SCHEMA subscriptions IS 'Subscriptions domain: plans, billing, invoices, usage';

-- ========================================
-- CREATE SERVICE USERS
-- ========================================
-- NOTE: In production, use stronger passwords from secrets manager

CREATE USER auth_svc WITH PASSWORD 'auth_svc_dev_password';
CREATE USER users_svc WITH PASSWORD 'users_svc_dev_password';
CREATE USER courses_svc WITH PASSWORD 'courses_svc_dev_password';
CREATE USER enrollments_svc WITH PASSWORD 'enrollments_svc_dev_password';
CREATE USER assessments_svc WITH PASSWORD 'assessments_svc_dev_password';
CREATE USER payments_svc WITH PASSWORD 'payments_svc_dev_password';
CREATE USER ai_svc WITH PASSWORD 'ai_svc_dev_password';
CREATE USER notifications_svc WITH PASSWORD 'notifications_svc_dev_password';
CREATE USER analytics_svc WITH PASSWORD 'analytics_svc_dev_password';
CREATE USER chatbot_svc WITH PASSWORD 'chatbot_svc_dev_password';
CREATE USER content_svc WITH PASSWORD 'content_svc_dev_password';
CREATE USER compliance_svc WITH PASSWORD 'compliance_svc_dev_password';
CREATE USER kb_svc WITH PASSWORD 'kb_svc_dev_password';
CREATE USER subscriptions_svc WITH PASSWORD 'subscriptions_svc_dev_password';

-- ========================================
-- GRANT SCHEMA PERMISSIONS
-- ========================================

-- Auth service: full access to auth schema, read users.users for validation
GRANT USAGE ON SCHEMA auth TO auth_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA auth TO auth_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA auth TO auth_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA auth GRANT ALL ON TABLES TO auth_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA auth GRANT ALL ON SEQUENCES TO auth_svc;

-- Users service: full access to users schema
GRANT USAGE ON SCHEMA users TO users_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA users TO users_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA users TO users_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA users GRANT ALL ON TABLES TO users_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA users GRANT ALL ON SEQUENCES TO users_svc;

-- Courses service: full access to courses schema
GRANT USAGE ON SCHEMA courses TO courses_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA courses TO courses_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA courses TO courses_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA courses GRANT ALL ON TABLES TO courses_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA courses GRANT ALL ON SEQUENCES TO courses_svc;

-- Enrollments service: full access to enrollments, read courses for validation
GRANT USAGE ON SCHEMA enrollments TO enrollments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA enrollments TO enrollments_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA enrollments TO enrollments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA enrollments GRANT ALL ON TABLES TO enrollments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA enrollments GRANT ALL ON SEQUENCES TO enrollments_svc;
-- Read access to courses for FK validation
GRANT USAGE ON SCHEMA courses TO enrollments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO enrollments_svc;

-- Assessments service: full access to assessments
GRANT USAGE ON SCHEMA assessments TO assessments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA assessments TO assessments_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA assessments TO assessments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA assessments GRANT ALL ON TABLES TO assessments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA assessments GRANT ALL ON SEQUENCES TO assessments_svc;
-- Read access to courses for FK validation
GRANT USAGE ON SCHEMA courses TO assessments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO assessments_svc;
-- Read access to enrollments for submission validation
GRANT USAGE ON SCHEMA enrollments TO assessments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA enrollments TO assessments_svc;

-- Payments service: full access to payments
GRANT USAGE ON SCHEMA payments TO payments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA payments TO payments_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA payments TO payments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA payments GRANT ALL ON TABLES TO payments_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA payments GRANT ALL ON SEQUENCES TO payments_svc;
-- Read access to courses for order creation
GRANT USAGE ON SCHEMA courses TO payments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO payments_svc;

-- AI service: full access to ai schema
GRANT USAGE ON SCHEMA ai TO ai_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA ai TO ai_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA ai TO ai_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA ai GRANT ALL ON TABLES TO ai_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA ai GRANT ALL ON SEQUENCES TO ai_svc;
-- Read access to courses for context
GRANT USAGE ON SCHEMA courses TO ai_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO ai_svc;

-- Notifications service: full access to notifications
GRANT USAGE ON SCHEMA notifications TO notifications_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA notifications TO notifications_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA notifications TO notifications_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA notifications GRANT ALL ON TABLES TO notifications_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA notifications GRANT ALL ON SEQUENCES TO notifications_svc;

-- Analytics service: full access to analytics
GRANT USAGE ON SCHEMA analytics TO analytics_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA analytics TO analytics_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA analytics TO analytics_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT ALL ON TABLES TO analytics_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT ALL ON SEQUENCES TO analytics_svc;

-- Chatbot service: full access to chatbot schema
GRANT USAGE ON SCHEMA chatbot TO chatbot_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA chatbot TO chatbot_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA chatbot TO chatbot_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA chatbot GRANT ALL ON TABLES TO chatbot_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA chatbot GRANT ALL ON SEQUENCES TO chatbot_svc;
-- Read access to courses for context
GRANT USAGE ON SCHEMA courses TO chatbot_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO chatbot_svc;

-- Content service: full access to content schema
GRANT USAGE ON SCHEMA content TO content_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA content TO content_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA content TO content_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA content GRANT ALL ON TABLES TO content_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA content GRANT ALL ON SEQUENCES TO content_svc;

-- Compliance service: full access to compliance schema
GRANT USAGE ON SCHEMA compliance TO compliance_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA compliance TO compliance_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA compliance TO compliance_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA compliance GRANT ALL ON TABLES TO compliance_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA compliance GRANT ALL ON SEQUENCES TO compliance_svc;

-- Knowledge Base service: full access to kb schema
GRANT USAGE ON SCHEMA kb TO kb_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA kb TO kb_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA kb TO kb_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA kb GRANT ALL ON TABLES TO kb_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA kb GRANT ALL ON SEQUENCES TO kb_svc;

-- Subscriptions service: full access to subscriptions schema
GRANT USAGE ON SCHEMA subscriptions TO subscriptions_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA subscriptions TO subscriptions_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA subscriptions TO subscriptions_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA subscriptions GRANT ALL ON TABLES TO subscriptions_svc;
ALTER DEFAULT PRIVILEGES IN SCHEMA subscriptions GRANT ALL ON SEQUENCES TO subscriptions_svc;

-- ========================================
-- SECURITY HARDENING
-- ========================================

-- Revoke public schema access (prevent accidental use)
REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM auth_svc, users_svc, courses_svc, enrollments_svc, assessments_svc, payments_svc, ai_svc, notifications_svc, analytics_svc, chatbot_svc, content_svc, compliance_svc, kb_svc, subscriptions_svc;

-- Ensure service users cannot create databases or roles
ALTER USER auth_svc NOCREATEDB NOCREATEROLE;
ALTER USER users_svc NOCREATEDB NOCREATEROLE;
ALTER USER courses_svc NOCREATEDB NOCREATEROLE;
ALTER USER enrollments_svc NOCREATEDB NOCREATEROLE;
ALTER USER assessments_svc NOCREATEDB NOCREATEROLE;
ALTER USER payments_svc NOCREATEDB NOCREATEROLE;
ALTER USER ai_svc NOCREATEDB NOCREATEROLE;
ALTER USER notifications_svc NOCREATEDB NOCREATEROLE;
ALTER USER analytics_svc NOCREATEDB NOCREATEROLE;
ALTER USER chatbot_svc NOCREATEDB NOCREATEROLE;
ALTER USER content_svc NOCREATEDB NOCREATEROLE;
ALTER USER compliance_svc NOCREATEDB NOCREATEROLE;
ALTER USER kb_svc NOCREATEDB NOCREATEROLE;
ALTER USER subscriptions_svc NOCREATEDB NOCREATEROLE;

-- Set connection limits per service user (prevent resource exhaustion)
ALTER USER auth_svc CONNECTION LIMIT 20;
ALTER USER users_svc CONNECTION LIMIT 20;
ALTER USER courses_svc CONNECTION LIMIT 20;
ALTER USER enrollments_svc CONNECTION LIMIT 20;
ALTER USER assessments_svc CONNECTION LIMIT 20;
ALTER USER payments_svc CONNECTION LIMIT 20;
ALTER USER ai_svc CONNECTION LIMIT 20;
ALTER USER notifications_svc CONNECTION LIMIT 20;
ALTER USER analytics_svc CONNECTION LIMIT 20;
ALTER USER chatbot_svc CONNECTION LIMIT 20;
ALTER USER content_svc CONNECTION LIMIT 20;
ALTER USER compliance_svc CONNECTION LIMIT 20;
ALTER USER kb_svc CONNECTION LIMIT 20;
ALTER USER subscriptions_svc CONNECTION LIMIT 20;

-- ========================================
-- CROSS-SCHEMA REFERENCES (Minimal)
-- ========================================
-- Note: We use UUIDs for cross-schema references.
-- Foreign keys are NOT enforced across schemas to maintain isolation.
-- Referential integrity is handled at the application/service level.

-- Common extensions needed
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ========================================
-- UTILITY FUNCTIONS (Shared)
-- ========================================

-- Timestamp update trigger function (shared across schemas)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Grant execute to all service users
GRANT EXECUTE ON FUNCTION update_updated_at_column() TO auth_svc, users_svc, courses_svc, enrollments_svc, assessments_svc, payments_svc, ai_svc, notifications_svc, analytics_svc, chatbot_svc, content_svc, compliance_svc, kb_svc, subscriptions_svc;

-- ========================================
-- VERIFICATION QUERIES
-- ========================================
-- Run these to verify schema setup:
--
-- List all schemas:
-- SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast');
--
-- List all users:
-- SELECT usename FROM pg_user WHERE usename LIKE '%_svc';
--
-- Check permissions:
-- SELECT grantee, table_schema, privilege_type FROM information_schema.role_table_grants WHERE grantee LIKE '%_svc';
--
-- Verify no service user has superuser privileges:
-- SELECT usename, usesuper, usecreatedb, usecreaterole FROM pg_user WHERE usename LIKE '%_svc';
-- Expected: all false
