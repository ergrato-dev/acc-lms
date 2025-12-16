-- Migration: 005_analytics.sql
-- Description: Analytics tables for event tracking, sessions and metrics
-- Author: System
-- Date: 2025-12-15
--
-- PREREQUISITE: Run 000_schema_setup.sql first
--
-- This migration creates tables for analytics:
-- - analytics.events         : Raw event tracking data
-- - analytics.sessions       : User sessions
-- - analytics.metrics        : Aggregated metrics
-- - analytics.daily_stats    : Daily aggregated statistics

-- ========================================
-- ANALYTICS SCHEMA SETUP
-- ========================================

-- Create analytics schema if not exists
CREATE SCHEMA IF NOT EXISTS analytics;

-- Grant permissions
GRANT USAGE ON SCHEMA analytics TO PUBLIC;
GRANT ALL ON ALL TABLES IN SCHEMA analytics TO PUBLIC;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT ALL ON TABLES TO PUBLIC;

-- ========================================
-- EVENT TRACKING
-- ========================================

-- Event types enum for better performance
CREATE TYPE analytics.event_type AS ENUM (
    'page_view',
    'click',
    'form_submit',
    'course_enroll',
    'course_complete',
    'lesson_start',
    'lesson_complete',
    'quiz_start',
    'quiz_complete',
    'assignment_submit',
    'video_play',
    'video_pause',
    'video_complete',
    'search',
    'download',
    'login',
    'logout',
    'error',
    'custom'
);

-- Platform enum
CREATE TYPE analytics.platform AS ENUM (
    'web',
    'android',
    'ios',
    'desktop',
    'api',
    'unknown'
);

-- Raw events table (append-only, partitioned by month)
CREATE TABLE analytics.events (
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    event_type analytics.event_type NOT NULL,
    custom_event_name TEXT, -- For 'custom' event_type
    tenant_id UUID,
    user_id UUID,
    session_id UUID NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Page/content context
    page_url TEXT,
    page_title TEXT,
    referrer TEXT,

    -- Platform/device info
    platform analytics.platform NOT NULL DEFAULT 'unknown',
    user_agent TEXT,
    browser TEXT,
    browser_version TEXT,
    os TEXT,
    os_version TEXT,
    device_type TEXT,
    screen_width INTEGER,
    screen_height INTEGER,

    -- Geo info
    ip_address INET,
    country TEXT,
    country_code CHAR(2),
    region TEXT,
    city TEXT,
    timezone TEXT,

    -- Event-specific data
    properties JSONB DEFAULT '{}'::jsonb,
    duration_ms BIGINT,
    entity_type TEXT, -- 'course', 'lesson', 'quiz', etc.
    entity_id UUID,

    -- Partition key
    created_month DATE NOT NULL DEFAULT DATE_TRUNC('month', NOW()),

    PRIMARY KEY (event_id, created_month)
) PARTITION BY RANGE (created_month);

-- Create partitions for current and next 12 months
CREATE TABLE analytics.events_2025_12 PARTITION OF analytics.events
    FOR VALUES FROM ('2025-12-01') TO ('2026-01-01');
CREATE TABLE analytics.events_2026_01 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-01-01') TO ('2026-02-01');
CREATE TABLE analytics.events_2026_02 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-02-01') TO ('2026-03-01');
CREATE TABLE analytics.events_2026_03 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-03-01') TO ('2026-04-01');
CREATE TABLE analytics.events_2026_04 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-04-01') TO ('2026-05-01');
CREATE TABLE analytics.events_2026_05 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-05-01') TO ('2026-06-01');
CREATE TABLE analytics.events_2026_06 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');
CREATE TABLE analytics.events_2026_07 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
CREATE TABLE analytics.events_2026_08 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');
CREATE TABLE analytics.events_2026_09 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-09-01') TO ('2026-10-01');
CREATE TABLE analytics.events_2026_10 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-10-01') TO ('2026-11-01');
CREATE TABLE analytics.events_2026_11 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-11-01') TO ('2026-12-01');
CREATE TABLE analytics.events_2026_12 PARTITION OF analytics.events
    FOR VALUES FROM ('2026-12-01') TO ('2027-01-01');

-- Indexes for common queries
CREATE INDEX idx_events_timestamp ON analytics.events(timestamp);
CREATE INDEX idx_events_user_id ON analytics.events(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_events_session_id ON analytics.events(session_id);
CREATE INDEX idx_events_tenant_id ON analytics.events(tenant_id) WHERE tenant_id IS NOT NULL;
CREATE INDEX idx_events_event_type ON analytics.events(event_type);
CREATE INDEX idx_events_entity ON analytics.events(entity_type, entity_id) WHERE entity_id IS NOT NULL;
CREATE INDEX idx_events_page_url ON analytics.events(page_url) WHERE page_url IS NOT NULL;

-- BRIN index for time-range queries (very efficient for append-only data)
CREATE INDEX idx_events_timestamp_brin ON analytics.events USING BRIN (timestamp);

-- ========================================
-- SESSION TRACKING
-- ========================================

CREATE TABLE analytics.sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,
    user_id UUID,

    -- Session timing
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    duration_seconds BIGINT,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    -- Navigation
    entry_page TEXT,
    exit_page TEXT,
    page_views INTEGER NOT NULL DEFAULT 0,
    events_count INTEGER NOT NULL DEFAULT 0,

    -- Platform/device
    platform analytics.platform NOT NULL DEFAULT 'unknown',
    user_agent TEXT,
    browser TEXT,
    browser_version TEXT,
    os TEXT,
    os_version TEXT,
    device_type TEXT,
    screen_width INTEGER,
    screen_height INTEGER,

    -- Geo info
    ip_address INET,
    country TEXT,
    country_code CHAR(2),
    region TEXT,
    city TEXT,
    timezone TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_sessions_user_id ON analytics.sessions(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_sessions_tenant_id ON analytics.sessions(tenant_id) WHERE tenant_id IS NOT NULL;
CREATE INDEX idx_sessions_started_at ON analytics.sessions(started_at);
CREATE INDEX idx_sessions_is_active ON analytics.sessions(is_active) WHERE is_active = TRUE;

-- ========================================
-- AGGREGATED METRICS
-- ========================================

-- Pre-aggregated metrics for fast dashboard queries
CREATE TABLE analytics.metrics (
    metric_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    tenant_id UUID,

    -- Aggregation period
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    granularity TEXT NOT NULL CHECK (granularity IN ('hourly', 'daily', 'weekly', 'monthly')),

    -- Metric values
    value DOUBLE PRECISION NOT NULL,
    count BIGINT,

    -- Dimensions for drill-down
    dimensions JSONB DEFAULT '{}'::jsonb,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Prevent duplicate aggregations
    UNIQUE (name, tenant_id, period_start, granularity, dimensions)
);

-- Indexes
CREATE INDEX idx_metrics_name ON analytics.metrics(name);
CREATE INDEX idx_metrics_tenant_id ON analytics.metrics(tenant_id) WHERE tenant_id IS NOT NULL;
CREATE INDEX idx_metrics_period ON analytics.metrics(period_start, period_end);
CREATE INDEX idx_metrics_granularity ON analytics.metrics(granularity);

-- ========================================
-- DAILY STATISTICS (materialized view alternative)
-- ========================================

CREATE TABLE analytics.daily_stats (
    stats_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stats_date DATE NOT NULL,
    tenant_id UUID,

    -- User metrics
    total_users BIGINT NOT NULL DEFAULT 0,
    active_users BIGINT NOT NULL DEFAULT 0,
    new_users BIGINT NOT NULL DEFAULT 0,
    returning_users BIGINT NOT NULL DEFAULT 0,

    -- Session metrics
    total_sessions BIGINT NOT NULL DEFAULT 0,
    avg_session_duration_seconds DOUBLE PRECISION,
    bounce_rate DOUBLE PRECISION,

    -- Page metrics
    total_page_views BIGINT NOT NULL DEFAULT 0,
    unique_page_views BIGINT NOT NULL DEFAULT 0,

    -- Course metrics
    course_enrollments BIGINT NOT NULL DEFAULT 0,
    course_completions BIGINT NOT NULL DEFAULT 0,
    lesson_starts BIGINT NOT NULL DEFAULT 0,
    lesson_completions BIGINT NOT NULL DEFAULT 0,
    quiz_completions BIGINT NOT NULL DEFAULT 0,

    -- Platform breakdown (stored as JSONB)
    platform_breakdown JSONB DEFAULT '{}'::jsonb,

    -- Top content (stored as JSONB arrays)
    top_pages JSONB DEFAULT '[]'::jsonb,
    top_courses JSONB DEFAULT '[]'::jsonb,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (stats_date, tenant_id)
);

-- Indexes
CREATE INDEX idx_daily_stats_date ON analytics.daily_stats(stats_date);
CREATE INDEX idx_daily_stats_tenant_id ON analytics.daily_stats(tenant_id) WHERE tenant_id IS NOT NULL;

-- ========================================
-- FUNCTIONS
-- ========================================

-- Function to auto-create event partitions
CREATE OR REPLACE FUNCTION analytics.create_event_partition(partition_date DATE)
RETURNS VOID AS $$
DECLARE
    partition_name TEXT;
    start_date DATE;
    end_date DATE;
BEGIN
    start_date := DATE_TRUNC('month', partition_date);
    end_date := start_date + INTERVAL '1 month';
    partition_name := 'events_' || TO_CHAR(start_date, 'YYYY_MM');

    -- Check if partition exists
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'analytics' AND c.relname = partition_name
    ) THEN
        EXECUTE format(
            'CREATE TABLE analytics.%I PARTITION OF analytics.events FOR VALUES FROM (%L) TO (%L)',
            partition_name, start_date, end_date
        );

        RAISE NOTICE 'Created partition: analytics.%', partition_name;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to update session on event
CREATE OR REPLACE FUNCTION analytics.update_session_stats()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE analytics.sessions
    SET
        events_count = events_count + 1,
        page_views = CASE
            WHEN NEW.event_type = 'page_view' THEN page_views + 1
            ELSE page_views
        END,
        exit_page = COALESCE(NEW.page_url, exit_page),
        last_activity_at = NOW()
    WHERE session_id = NEW.session_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update session on new event
CREATE TRIGGER trg_update_session_on_event
    AFTER INSERT ON analytics.events
    FOR EACH ROW
    EXECUTE FUNCTION analytics.update_session_stats();

-- ========================================
-- COMMENTS
-- ========================================

COMMENT ON TABLE analytics.events IS 'Raw event tracking data, partitioned by month for efficient querying and archival';
COMMENT ON TABLE analytics.sessions IS 'User session tracking with device and geo information';
COMMENT ON TABLE analytics.metrics IS 'Pre-aggregated metrics for fast dashboard queries';
COMMENT ON TABLE analytics.daily_stats IS 'Daily aggregated statistics for reporting';
COMMENT ON FUNCTION analytics.create_event_partition IS 'Auto-creates monthly partitions for the events table';
