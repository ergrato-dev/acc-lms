-- Migration: 001_initial_schema.sql
-- Description: Core tables for auth, users, courses, and enrollments schemas
-- Author: System
-- Date: 2025-08-08
-- Updated: 2025-12-15 (Schema separation)
--
-- PREREQUISITE: Run 000_schema_setup.sql first
--
-- This migration creates tables in their respective schemas:
-- - auth.*     : Authentication (users core, tokens)
-- - users.*    : User profiles and preferences
-- - courses.*  : Course content management
-- - enrollments.* : Student enrollments and progress

-- ========================================
-- AUTH SCHEMA: Core user identity
-- ========================================

-- Main users table (identity source of truth)
CREATE TABLE auth.users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('student', 'instructor', 'admin')),
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verification_token TEXT,
    password_reset_token TEXT,
    password_reset_expires TIMESTAMP,
    last_login_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP -- soft delete
);

-- Performance indexes
CREATE INDEX idx_auth_users_email ON auth.users(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_auth_users_role ON auth.users(role) WHERE deleted_at IS NULL;
CREATE INDEX idx_auth_users_created_at ON auth.users(created_at);

-- JWT refresh tokens
CREATE TABLE auth.refresh_tokens (
    token_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(user_id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    device_fingerprint TEXT,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMP
);

CREATE INDEX idx_auth_refresh_tokens_user_id ON auth.refresh_tokens(user_id);
CREATE INDEX idx_auth_refresh_tokens_expires_at ON auth.refresh_tokens(expires_at);

-- Trigger for updated_at
CREATE TRIGGER auth_users_updated_at
    BEFORE UPDATE ON auth.users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- USERS SCHEMA: User profiles and preferences
-- ========================================

-- User profile (extended info, separate from auth)
CREATE TABLE users.profiles (
    user_id UUID PRIMARY KEY, -- References auth.users(user_id) - no FK for isolation
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    avatar_url TEXT,
    bio TEXT,
    website TEXT,
    social_links JSONB DEFAULT '{}',
    timezone TEXT DEFAULT 'America/Mexico_City',
    language TEXT DEFAULT 'es' CHECK (language IN ('es', 'en', 'pt')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_profiles_language ON users.profiles(language);

-- User preferences
CREATE TABLE users.preferences (
    user_id UUID PRIMARY KEY, -- References auth.users(user_id)
    email_notifications JSONB NOT NULL DEFAULT '{
        "marketing": true,
        "course_updates": true,
        "reminders": true,
        "weekly_digest": true
    }'::jsonb,
    privacy JSONB NOT NULL DEFAULT '{
        "show_email": false,
        "show_profile": true,
        "show_progress": true
    }'::jsonb,
    accessibility JSONB NOT NULL DEFAULT '{
        "high_contrast": false,
        "font_size": "normal",
        "reduce_motion": false
    }'::jsonb,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_preferences_notifications ON users.preferences USING GIN (email_notifications);

-- User statistics (for gamification)
CREATE TABLE users.stats (
    user_id UUID PRIMARY KEY, -- References auth.users(user_id)
    courses_enrolled INTEGER NOT NULL DEFAULT 0,
    courses_completed INTEGER NOT NULL DEFAULT 0,
    certificates_earned INTEGER NOT NULL DEFAULT 0,
    total_learning_time_minutes BIGINT NOT NULL DEFAULT 0,
    average_completion_rate DECIMAL(5,4) NOT NULL DEFAULT 0.0000,
    current_streak_days INTEGER NOT NULL DEFAULT 0,
    longest_streak_days INTEGER NOT NULL DEFAULT 0,
    last_activity_at TIMESTAMP,
    calculated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_stats_courses_completed ON users.stats(courses_completed DESC);
CREATE INDEX idx_users_stats_streak ON users.stats(current_streak_days DESC);

-- Triggers
CREATE TRIGGER users_profiles_updated_at
    BEFORE UPDATE ON users.profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER users_preferences_updated_at
    BEFORE UPDATE ON users.preferences
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- COURSES SCHEMA: Course content
-- ========================================

-- Course categories
CREATE TABLE courses.categories (
    category_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    icon_url TEXT,
    parent_category_id UUID REFERENCES courses.categories(category_id),
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Main courses table
CREATE TABLE courses.courses (
    course_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instructor_id UUID NOT NULL, -- References auth.users(user_id) - no FK for isolation
    category_id UUID REFERENCES courses.categories(category_id),
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    short_description TEXT NOT NULL,
    full_description TEXT,
    thumbnail_url TEXT,
    trailer_video_url TEXT,
    price_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    difficulty_level TEXT NOT NULL CHECK (difficulty_level IN ('beginner', 'intermediate', 'advanced')),
    estimated_duration_hours INTEGER NOT NULL DEFAULT 1,
    language TEXT NOT NULL DEFAULT 'es',
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMP,
    average_rating DECIMAL(3,2) DEFAULT 0.00,
    total_ratings INTEGER DEFAULT 0,
    total_enrollments INTEGER DEFAULT 0,
    requirements JSONB DEFAULT '[]'::jsonb,
    learning_objectives JSONB DEFAULT '[]'::jsonb,
    target_audience JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
);

-- Course performance indexes
CREATE INDEX idx_courses_instructor_id ON courses.courses(instructor_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_courses_category_id ON courses.courses(category_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_courses_published ON courses.courses(is_published, published_at) WHERE deleted_at IS NULL;
CREATE INDEX idx_courses_price ON courses.courses(price_cents) WHERE deleted_at IS NULL;
CREATE INDEX idx_courses_rating ON courses.courses(average_rating) WHERE deleted_at IS NULL;
CREATE INDEX idx_courses_text_search ON courses.courses USING gin(to_tsvector('spanish', title || ' ' || short_description));

-- Course sections
CREATE TABLE courses.sections (
    section_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES courses.courses(course_id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Lessons
CREATE TABLE courses.lessons (
    lesson_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    section_id UUID NOT NULL REFERENCES courses.sections(section_id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses.courses(course_id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content_type TEXT NOT NULL CHECK (content_type IN ('video', 'article', 'quiz', 'assignment', 'live_session')),
    content_ref TEXT,
    duration_seconds INTEGER DEFAULT 0,
    is_preview BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_courses_lessons_section_id ON courses.lessons(section_id);
CREATE INDEX idx_courses_lessons_course_id ON courses.lessons(course_id);

-- Triggers
CREATE TRIGGER courses_courses_updated_at
    BEFORE UPDATE ON courses.courses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER courses_lessons_updated_at
    BEFORE UPDATE ON courses.lessons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- ENROLLMENTS SCHEMA: Student progress
-- ========================================

-- Student enrollments
CREATE TABLE enrollments.enrollments (
    enrollment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL, -- References auth.users(user_id)
    course_id UUID NOT NULL, -- References courses.courses(course_id)
    status TEXT NOT NULL CHECK (status IN ('active', 'completed', 'paused', 'refunded', 'expired')),
    progress_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    last_accessed_at TIMESTAMP,
    certificate_issued_at TIMESTAMP,
    enrollment_source TEXT DEFAULT 'purchase',
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, course_id)
);

CREATE INDEX idx_enrollments_user_id ON enrollments.enrollments(user_id);
CREATE INDEX idx_enrollments_course_id ON enrollments.enrollments(course_id);
CREATE INDEX idx_enrollments_status ON enrollments.enrollments(status);

-- Lesson progress tracking
CREATE TABLE enrollments.lesson_progress (
    progress_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    enrollment_id UUID NOT NULL REFERENCES enrollments.enrollments(enrollment_id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL, -- References courses.lessons(lesson_id)
    user_id UUID NOT NULL, -- References auth.users(user_id)
    status TEXT NOT NULL CHECK (status IN ('not_started', 'in_progress', 'completed')),
    completion_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    time_spent_seconds INTEGER NOT NULL DEFAULT 0,
    last_position_seconds INTEGER DEFAULT 0,
    completed_at TIMESTAMP,
    first_accessed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_accessed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(enrollment_id, lesson_id)
);

CREATE INDEX idx_enrollments_progress_enrollment_id ON enrollments.lesson_progress(enrollment_id);
CREATE INDEX idx_enrollments_progress_user_id ON enrollments.lesson_progress(user_id);

-- Trigger
CREATE TRIGGER enrollments_enrollments_updated_at
    BEFORE UPDATE ON enrollments.enrollments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- SEED DATA
-- ========================================

-- Insert default categories
INSERT INTO courses.categories (name, slug, description, sort_order) VALUES
    ('Programación', 'programacion', 'Cursos de desarrollo de software y programación', 1),
    ('Ciencia de Datos', 'ciencia-de-datos', 'Análisis de datos y machine learning', 2),
    ('Negocios', 'negocios', 'Negocios y emprendimiento', 3),
    ('Diseño', 'diseno', 'Diseño UI/UX y gráfico', 4);

-- Create default admin user (password: admin123)
INSERT INTO auth.users (user_id, email, hashed_password, role, email_verified) VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin@acc-lms.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewcehzXQ5KbR.zPy', 'admin', true);

-- Create admin profile
INSERT INTO users.profiles (user_id, first_name, last_name, timezone, language) VALUES
    ('00000000-0000-0000-0000-000000000001', 'System', 'Admin', 'America/Mexico_City', 'es');

-- Create admin preferences
INSERT INTO users.preferences (user_id) VALUES
    ('00000000-0000-0000-0000-000000000001');

-- Create admin stats
INSERT INTO users.stats (user_id) VALUES
    ('00000000-0000-0000-0000-000000000001');
