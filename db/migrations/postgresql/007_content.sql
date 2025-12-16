-- =============================================================================
-- ACC LMS - Content Assets Migration
-- =============================================================================
-- Tablas para almacenamiento de contenido multimedia
-- Soporta multi-tenancy y múltiples backends de storage
-- =============================================================================

-- Set search path to content schema
SET search_path TO content, public;

-- Tipos enum para content (in content schema)
CREATE TYPE content.asset_type AS ENUM (
    'video',
    'image',
    'document',
    'audio',
    'subtitle',
    'attachment'
);

CREATE TYPE content.processing_status AS ENUM (
    'pending',
    'uploading',
    'processing',
    'transcoding',
    'ready',
    'failed',
    'deleted'
);

-- =============================================================================
-- Tabla principal de assets
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.assets (
    asset_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Ownership
    owner_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    tenant_id UUID REFERENCES tenants(tenant_id) ON DELETE CASCADE,

    -- Content association
    course_id UUID REFERENCES courses(course_id) ON DELETE SET NULL,
    lesson_id UUID, -- Referencia a futuro módulo de lecciones

    -- File info
    filename VARCHAR(500) NOT NULL,
    original_filename VARCHAR(500) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    size_bytes BIGINT NOT NULL CHECK (size_bytes > 0),
    checksum VARCHAR(128), -- SHA-256 hash

    -- Storage
    storage_key VARCHAR(1000) NOT NULL UNIQUE,
    storage_backend VARCHAR(50) NOT NULL DEFAULT 'local', -- local, s3, minio
    storage_bucket VARCHAR(255),

    -- Classification
    asset_type content_type NOT NULL DEFAULT 'attachment',
    status content.processing_status NOT NULL DEFAULT 'pending',

    -- Metadata (JSON flexible)
    metadata JSONB DEFAULT '{}',

    -- Processing info
    processing_started_at TIMESTAMPTZ,
    processing_completed_at TIMESTAMPTZ,
    processing_error TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- =============================================================================
-- Tabla para variantes de video (múltiples calidades)
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.video_variants (
    variant_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES content.assets(asset_id) ON DELETE CASCADE,

    -- Variant info
    quality VARCHAR(50) NOT NULL, -- 1080p, 720p, 480p, 360p
    codec VARCHAR(50) NOT NULL,   -- h264, h265, vp9
    bitrate_kbps INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,

    -- Storage
    storage_key VARCHAR(1000) NOT NULL,
    size_bytes BIGINT NOT NULL,

    -- Status
    is_ready BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(asset_id, quality, codec)
);

-- =============================================================================
-- Tabla para thumbnails generados
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.thumbnails (
    thumbnail_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES content.assets(asset_id) ON DELETE CASCADE,

    -- Thumbnail info
    thumbnail_type VARCHAR(50) NOT NULL, -- small, medium, large, poster
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    timestamp_seconds INTEGER, -- Para videos: momento del thumbnail

    -- Storage
    storage_key VARCHAR(1000) NOT NULL,
    size_bytes BIGINT NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(asset_id, thumbnail_type)
);

-- =============================================================================
-- Tabla para transcripciones de video/audio
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.transcriptions (
    transcription_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES content.assets(asset_id) ON DELETE CASCADE,

    -- Transcription info
    language VARCHAR(10) NOT NULL, -- es, en, pt, etc.
    format VARCHAR(20) NOT NULL,   -- srt, vtt, txt
    is_auto_generated BOOLEAN NOT NULL DEFAULT FALSE,

    -- Storage
    storage_key VARCHAR(1000) NOT NULL,
    size_bytes BIGINT NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(asset_id, language, format)
);

-- =============================================================================
-- Tabla para tokens de acceso temporal
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.access_tokens (
    token_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES content.assets(asset_id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,

    -- Token info
    token_hash VARCHAR(128) NOT NULL, -- SHA-256 del token
    token_type VARCHAR(50) NOT NULL,  -- download, stream, upload

    -- Expiration
    expires_at TIMESTAMPTZ NOT NULL,

    -- Usage tracking
    max_uses INTEGER DEFAULT 1,
    use_count INTEGER NOT NULL DEFAULT 0,
    last_used_at TIMESTAMPTZ,

    -- Context
    ip_address INET,
    user_agent TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- Tabla para estadísticas de uso
-- =============================================================================
CREATE TABLE IF NOT EXISTS content.usage_stats (
    stat_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES content.assets(asset_id) ON DELETE CASCADE,

    -- Stats by period
    period_start DATE NOT NULL,
    period_type VARCHAR(20) NOT NULL, -- daily, weekly, monthly

    -- Counters
    view_count INTEGER NOT NULL DEFAULT 0,
    download_count INTEGER NOT NULL DEFAULT 0,
    stream_count INTEGER NOT NULL DEFAULT 0,
    bytes_transferred BIGINT NOT NULL DEFAULT 0,
    unique_users INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(asset_id, period_start, period_type)
);

-- =============================================================================
-- Indexes
-- =============================================================================

-- Assets
CREATE INDEX idx_content_assets_owner ON content.assets(owner_id);
CREATE INDEX idx_content_assets_tenant ON content.assets(tenant_id);
CREATE INDEX idx_content_assets_course ON content.assets(course_id);
CREATE INDEX idx_content_assets_lesson ON content.assets(lesson_id);
CREATE INDEX idx_content_assets_status ON content.assets(status);
CREATE INDEX idx_content_assets_type ON content.assets(asset_type);
CREATE INDEX idx_content_assets_storage_key ON content.assets(storage_key);
CREATE INDEX idx_content_assets_created ON content.assets(created_at DESC);
CREATE INDEX idx_content_assets_filename ON content.assets(filename);
CREATE INDEX idx_content_assets_deleted ON content.assets(deleted_at) WHERE deleted_at IS NOT NULL;

-- Full text search on filename
CREATE INDEX idx_content_assets_filename_search ON content.assets USING gin(to_tsvector('spanish', filename));

-- JSONB metadata
CREATE INDEX idx_content_assets_metadata ON content.assets USING gin(metadata);

-- Variants
CREATE INDEX idx_video_variants_asset ON content.video_variants(asset_id);
CREATE INDEX idx_video_variants_ready ON content.video_variants(asset_id) WHERE is_ready = TRUE;

-- Thumbnails
CREATE INDEX idx_asset_thumbnails_asset ON content.thumbnails(asset_id);

-- Transcriptions
CREATE INDEX idx_asset_transcriptions_asset ON content.transcriptions(asset_id);
CREATE INDEX idx_asset_transcriptions_language ON content.transcriptions(language);

-- Access tokens
CREATE INDEX idx_asset_access_tokens_asset ON content.access_tokens(asset_id);
CREATE INDEX idx_asset_access_tokens_expires ON content.access_tokens(expires_at);
CREATE INDEX idx_asset_access_tokens_hash ON content.access_tokens(token_hash);

-- Usage stats
CREATE INDEX idx_asset_usage_stats_asset ON content.usage_stats(asset_id);
CREATE INDEX idx_asset_usage_stats_period ON content.usage_stats(period_start, period_type);

-- =============================================================================
-- Triggers
-- =============================================================================

-- Updated at trigger
CREATE TRIGGER update_content_assets_updated_at
    BEFORE UPDATE ON content.assets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- Funciones auxiliares
-- =============================================================================

-- Función para obtener estadísticas de storage de un usuario
CREATE OR REPLACE FUNCTION get_user_storage_stats(p_user_id UUID)
RETURNS TABLE (
    total_assets BIGINT,
    total_size_bytes BIGINT,
    assets_by_type JSONB,
    assets_by_status JSONB
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*) AS total_assets,
        COALESCE(SUM(ca.size_bytes), 0) AS total_size_bytes,
        jsonb_object_agg(
            DISTINCT ca.asset_type::TEXT,
            (SELECT COUNT(*) FROM content_assets WHERE owner_id = p_user_id AND asset_type = ca.asset_type AND deleted_at IS NULL)
        ) FILTER (WHERE ca.asset_type IS NOT NULL) AS assets_by_type,
        jsonb_object_agg(
            DISTINCT ca.status::TEXT,
            (SELECT COUNT(*) FROM content_assets WHERE owner_id = p_user_id AND status = ca.status AND deleted_at IS NULL)
        ) FILTER (WHERE ca.status IS NOT NULL) AS assets_by_status
    FROM content_assets ca
    WHERE ca.owner_id = p_user_id AND ca.deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;

-- Función para limpiar tokens expirados
CREATE OR REPLACE FUNCTION cleanup_expired_tokens()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM asset_access_tokens
    WHERE expires_at < NOW();

    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- Datos iniciales
-- =============================================================================

-- No hay datos iniciales necesarios para este módulo

COMMENT ON TABLE content_assets IS 'Almacena metadata de archivos multimedia (videos, imágenes, documentos)';
COMMENT ON TABLE video_variants IS 'Variantes de calidad para videos (1080p, 720p, etc.)';
COMMENT ON TABLE asset_thumbnails IS 'Thumbnails generados para videos e imágenes';
COMMENT ON TABLE asset_transcriptions IS 'Transcripciones y subtítulos para videos';
COMMENT ON TABLE asset_access_tokens IS 'Tokens temporales para acceso a archivos';
COMMENT ON TABLE asset_usage_stats IS 'Estadísticas de uso de archivos por período';
