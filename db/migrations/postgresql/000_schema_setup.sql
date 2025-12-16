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

-- ========================================
-- SECURITY HARDENING
-- ========================================

-- Revoke public schema access (prevent accidental use)
REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM auth_svc, users_svc, courses_svc, enrollments_svc, assessments_svc, payments_svc, ai_svc, notifications_svc;

-- Ensure service users cannot create databases or roles
ALTER USER auth_svc NOCREATEDB NOCREATEROLE;
ALTER USER users_svc NOCREATEDB NOCREATEROLE;
ALTER USER courses_svc NOCREATEDB NOCREATEROLE;
ALTER USER enrollments_svc NOCREATEDB NOCREATEROLE;
ALTER USER assessments_svc NOCREATEDB NOCREATEROLE;
ALTER USER payments_svc NOCREATEDB NOCREATEROLE;
ALTER USER ai_svc NOCREATEDB NOCREATEROLE;
ALTER USER notifications_svc NOCREATEDB NOCREATEROLE;

-- Set connection limits per service user (prevent resource exhaustion)
ALTER USER auth_svc CONNECTION LIMIT 20;
ALTER USER users_svc CONNECTION LIMIT 20;
ALTER USER courses_svc CONNECTION LIMIT 20;
ALTER USER enrollments_svc CONNECTION LIMIT 20;
ALTER USER assessments_svc CONNECTION LIMIT 20;
ALTER USER payments_svc CONNECTION LIMIT 20;
ALTER USER ai_svc CONNECTION LIMIT 20;
ALTER USER notifications_svc CONNECTION LIMIT 20;

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
GRANT EXECUTE ON FUNCTION update_updated_at_column() TO auth_svc, users_svc, courses_svc, enrollments_svc, assessments_svc, payments_svc, ai_svc, notifications_svc;

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
