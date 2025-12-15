-- Migration: 005_user_profile_extensions.sql
-- Description: Extended user profile and preferences tables for users-service
-- Author: System
-- Date: 2024-01-20
--
-- This migration adds:
-- 1. Extended user_preferences table with JSON columns for flexible settings
-- 2. User statistics table for learning progress
-- 3. Social links support in users table
-- 4. Additional indexes for performance

-- ========================================
-- PREREQUISITE CHECK
-- ========================================
-- Ensure users table exists (from 001_initial_schema.sql)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'users') THEN
        RAISE EXCEPTION 'Table "users" does not exist. Run 001_initial_schema.sql first.';
    END IF;
END $$;

-- ========================================
-- USERS TABLE EXTENSIONS
-- ========================================

-- Add social_links JSONB column if not exists
-- Structure: {"twitter": "url", "linkedin": "url", "github": "url", "youtube": "url"}
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'social_links'
    ) THEN
        ALTER TABLE users ADD COLUMN social_links JSONB DEFAULT '{}';
    END IF;
END $$;

-- Add website column if not exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'users' AND column_name = 'website'
    ) THEN
        ALTER TABLE users ADD COLUMN website TEXT;
    END IF;
END $$;

-- Rename user_id to id for consistency with our Rust code
-- Note: This requires updating foreign keys in dependent tables
-- For now, we'll create an alias view instead
CREATE OR REPLACE VIEW users_v AS
SELECT 
    user_id AS id,
    email,
    hashed_password,
    first_name,
    last_name,
    role,
    avatar_url,
    bio,
    website,
    social_links,
    timezone,
    language_preference AS language,
    email_verified,
    email_verification_token,
    password_reset_token,
    password_reset_expires,
    last_login_at,
    created_at,
    updated_at,
    deleted_at
FROM users;

-- ========================================
-- EXTENDED USER PREFERENCES
-- ========================================

-- Drop existing user_preferences if it has old structure
-- and recreate with JSONB columns for flexibility
DROP TABLE IF EXISTS user_preferences CASCADE;

CREATE TABLE user_preferences (
    -- Primary key referencing users
    user_id UUID PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE,
    
    -- Localization settings
    -- language: ISO 639-1 code (es, en, pt)
    language TEXT NOT NULL DEFAULT 'es',
    
    -- timezone: IANA timezone string (America/Mexico_City)
    timezone TEXT NOT NULL DEFAULT 'America/Mexico_City',
    
    -- Email notification settings as JSON
    -- Structure: {
    --   "marketing": true/false,
    --   "course_updates": true/false,
    --   "reminders": true/false,
    --   "weekly_digest": true/false
    -- }
    email_notifications JSONB NOT NULL DEFAULT '{
        "marketing": true,
        "course_updates": true,
        "reminders": true,
        "weekly_digest": true
    }'::jsonb,
    
    -- Privacy settings as JSON
    -- Structure: {
    --   "show_email": true/false,
    --   "show_profile": true/false,
    --   "show_progress": true/false
    -- }
    privacy JSONB NOT NULL DEFAULT '{
        "show_email": false,
        "show_profile": true,
        "show_progress": true
    }'::jsonb,
    
    -- Accessibility settings as JSON
    -- Structure: {
    --   "high_contrast": true/false,
    --   "font_size": "normal"|"large"|"x-large",
    --   "reduce_motion": true/false
    -- }
    accessibility JSONB NOT NULL DEFAULT '{
        "high_contrast": false,
        "font_size": "normal",
        "reduce_motion": false
    }'::jsonb,
    
    -- Audit timestamp
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for language filtering (useful for marketing)
CREATE INDEX idx_user_preferences_language ON user_preferences(language);

-- GIN index for JSONB queries (e.g., find users with marketing emails enabled)
CREATE INDEX idx_user_preferences_email_notifications 
    ON user_preferences USING GIN (email_notifications);

-- Comment on table
COMMENT ON TABLE user_preferences IS 
'User preferences for localization, notifications, privacy, and accessibility settings';

-- ========================================
-- USER STATISTICS TABLE
-- ========================================

CREATE TABLE IF NOT EXISTS user_stats (
    -- Primary key referencing users
    user_id UUID PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE,
    
    -- Course metrics
    courses_enrolled INTEGER NOT NULL DEFAULT 0,
    courses_completed INTEGER NOT NULL DEFAULT 0,
    certificates_earned INTEGER NOT NULL DEFAULT 0,
    
    -- Learning time metrics
    total_learning_time_minutes BIGINT NOT NULL DEFAULT 0,
    
    -- Progress metrics
    average_completion_rate DECIMAL(5,4) NOT NULL DEFAULT 0.0000,
    
    -- Streak tracking
    current_streak_days INTEGER NOT NULL DEFAULT 0,
    longest_streak_days INTEGER NOT NULL DEFAULT 0,
    
    -- Activity tracking
    last_activity_at TIMESTAMP WITH TIME ZONE,
    
    -- Computation timestamp (for cache invalidation)
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for leaderboard queries
CREATE INDEX idx_user_stats_courses_completed ON user_stats(courses_completed DESC);
CREATE INDEX idx_user_stats_learning_time ON user_stats(total_learning_time_minutes DESC);
CREATE INDEX idx_user_stats_streak ON user_stats(current_streak_days DESC);

-- Comment on table
COMMENT ON TABLE user_stats IS 
'Aggregated user statistics for learning progress and gamification';

-- ========================================
-- TRIGGER: Update updated_at on user_preferences
-- ========================================

CREATE OR REPLACE FUNCTION update_user_preferences_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS user_preferences_updated_at ON user_preferences;
CREATE TRIGGER user_preferences_updated_at
    BEFORE UPDATE ON user_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_user_preferences_timestamp();

-- ========================================
-- TRIGGER: Auto-create preferences for new users
-- ========================================

CREATE OR REPLACE FUNCTION create_default_user_preferences()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_preferences (user_id, language, timezone)
    VALUES (
        NEW.user_id,
        COALESCE(NEW.language_preference, 'es'),
        COALESCE(NEW.timezone, 'America/Mexico_City')
    )
    ON CONFLICT (user_id) DO NOTHING;
    
    INSERT INTO user_stats (user_id)
    VALUES (NEW.user_id)
    ON CONFLICT (user_id) DO NOTHING;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS create_user_preferences ON users;
CREATE TRIGGER create_user_preferences
    AFTER INSERT ON users
    FOR EACH ROW
    EXECUTE FUNCTION create_default_user_preferences();

-- ========================================
-- DATA MIGRATION: Create preferences for existing users
-- ========================================

INSERT INTO user_preferences (user_id, language, timezone)
SELECT 
    user_id,
    COALESCE(language_preference, 'es'),
    COALESCE(timezone, 'America/Mexico_City')
FROM users
WHERE NOT EXISTS (
    SELECT 1 FROM user_preferences WHERE user_preferences.user_id = users.user_id
)
ON CONFLICT (user_id) DO NOTHING;

INSERT INTO user_stats (user_id)
SELECT user_id FROM users
WHERE NOT EXISTS (
    SELECT 1 FROM user_stats WHERE user_stats.user_id = users.user_id
)
ON CONFLICT (user_id) DO NOTHING;

-- ========================================
-- VALIDATION CHECKS
-- ========================================

-- Check constraint for valid language codes
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.check_constraints 
        WHERE constraint_name = 'user_preferences_language_check'
    ) THEN
        ALTER TABLE user_preferences 
        ADD CONSTRAINT user_preferences_language_check 
        CHECK (language IN ('es', 'en', 'pt'));
    END IF;
END $$;

-- ========================================
-- GRANT PERMISSIONS (if using roles)
-- ========================================
-- GRANT SELECT, INSERT, UPDATE, DELETE ON user_preferences TO app_user;
-- GRANT SELECT, INSERT, UPDATE, DELETE ON user_stats TO app_user;
