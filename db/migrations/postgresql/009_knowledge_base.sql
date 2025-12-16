-- =============================================================================
-- ACC LMS - Knowledge Base Database Schema
-- =============================================================================
-- Esquema para base de conocimiento: artículos de ayuda, FAQ, documentación
-- =============================================================================

-- Set search path to kb schema
SET search_path TO kb, public;

-- =============================================================================
-- ENUMS (in kb schema)
-- =============================================================================

-- Estado del artículo
CREATE TYPE kb.article_status AS ENUM (
    'draft',
    'in_review',
    'published',
    'archived'
);

-- Tipo de contenido
CREATE TYPE kb.kb_content_type AS ENUM (
    'markdown',
    'html',
    'rich_text'
);

-- Visibilidad del artículo
CREATE TYPE kb.article_visibility AS ENUM (
    'public',        -- Visible para todos
    'authenticated', -- Solo usuarios autenticados
    'restricted',    -- Solo roles específicos
    'internal'       -- Solo admins/staff
);

-- =============================================================================
-- KB CATEGORIES (Categorías de artículos)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb.categories (
    category_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,

    -- Información básica
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(50),                              -- Nombre de ícono
    color VARCHAR(7),                              -- Color hex (#RRGGBB)

    -- Jerarquía
    parent_id UUID REFERENCES kb.categories(category_id) ON DELETE SET NULL,
    path JSONB NOT NULL DEFAULT '[]',              -- Array de UUIDs de ancestros
    depth INTEGER NOT NULL DEFAULT 0,

    -- Orden y visibilidad
    order_index INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,

    -- Estadísticas
    article_count INTEGER NOT NULL DEFAULT 0,
    view_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_kb_category_slug UNIQUE (slug, tenant_id)
);

-- Índices para kb_categories
CREATE INDEX idx_kb_categories_tenant ON kb.categories(tenant_id);
CREATE INDEX idx_kb_categories_parent ON kb.categories(parent_id);
CREATE INDEX idx_kb_categories_slug ON kb.categories(slug);
CREATE INDEX idx_kb_categories_visible ON kb.categories(is_visible) WHERE is_visible = TRUE;

-- =============================================================================
-- KB ARTICLES (Artículos de ayuda)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb.articles (
    article_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,

    -- Información básica
    title VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL,
    excerpt TEXT,                                  -- Resumen corto

    -- Contenido
    content TEXT NOT NULL,
    content_type kb_content_type NOT NULL DEFAULT 'markdown',
    rendered_html TEXT,                            -- HTML pre-renderizado

    -- Categorización
    category_id UUID REFERENCES kb.categories(category_id) ON DELETE SET NULL,
    tags JSONB NOT NULL DEFAULT '[]',              -- Array de strings

    -- SEO
    meta_title VARCHAR(70),
    meta_description VARCHAR(160),
    meta_keywords JSONB NOT NULL DEFAULT '[]',

    -- Estado y visibilidad
    status article_status NOT NULL DEFAULT 'draft',
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,

    -- Control de acceso
    visibility article_visibility NOT NULL DEFAULT 'public',
    allowed_roles JSONB NOT NULL DEFAULT '[]',     -- Roles permitidos si visibility = restricted

    -- Autor
    author_id UUID NOT NULL,
    author_name VARCHAR(100),                      -- Denormalizado

    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    previous_version_id UUID,

    -- Estadísticas
    view_count BIGINT NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_kb_article_slug UNIQUE (slug, tenant_id),
    CONSTRAINT fk_kb_article_author FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Índices para kb_articles
CREATE INDEX idx_kb_articles_tenant ON kb.articles(tenant_id);
CREATE INDEX idx_kb_articles_slug ON kb.articles(slug);
CREATE INDEX idx_kb_articles_category ON kb.articles(category_id);
CREATE INDEX idx_kb_articles_author ON kb.articles(author_id);
CREATE INDEX idx_kb_articles_status ON kb.articles(status);
CREATE INDEX idx_kb_articles_visibility ON kb.articles(visibility);
CREATE INDEX idx_kb_articles_featured ON kb.articles(is_featured) WHERE is_featured = TRUE;
CREATE INDEX idx_kb_articles_pinned ON kb.articles(is_pinned) WHERE is_pinned = TRUE;
CREATE INDEX idx_kb_articles_published ON kb.articles(published_at DESC NULLS LAST);
CREATE INDEX idx_kb_articles_tags ON kb.articles USING GIN (tags);

-- Full-text search index
CREATE INDEX idx_kb_articles_search ON kb.articles
    USING GIN (to_tsvector('spanish', title || ' ' || COALESCE(content, '')));

-- =============================================================================
-- KB ARTICLE VERSIONS (Historial de versiones)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb.article_versions (
    version_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES kb.articles(article_id) ON DELETE CASCADE,
    version_number INTEGER NOT NULL,

    -- Snapshot del contenido
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    content_type kb_content_type NOT NULL,

    -- Metadata
    change_summary TEXT,
    changed_by UUID NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_article_version UNIQUE (article_id, version_number)
);

-- Índices
CREATE INDEX idx_kb_versions_article ON kb.article_versions(article_id);
CREATE INDEX idx_kb_versions_changed_by ON kb.article_versions(changed_by);

-- =============================================================================
-- KB ARTICLE FEEDBACK (Retroalimentación)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb.article_feedback (
    feedback_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id UUID NOT NULL REFERENCES kb.articles(article_id) ON DELETE CASCADE,
    user_id UUID,
    anonymous_id VARCHAR(255),

    -- Feedback
    is_helpful BOOLEAN NOT NULL,
    comment TEXT,

    -- Contexto
    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_kb_feedback_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Índices
CREATE INDEX idx_kb_feedback_article ON kb.article_feedback(article_id);
CREATE INDEX idx_kb_feedback_user ON kb.article_feedback(user_id);
CREATE INDEX idx_kb_feedback_helpful ON kb.article_feedback(is_helpful);

-- =============================================================================
-- KB FAQs (Preguntas frecuentes)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb_faqs (
    faq_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,

    -- Contenido
    question VARCHAR(500) NOT NULL,
    answer TEXT NOT NULL,
    answer_type kb_content_type NOT NULL DEFAULT 'markdown',
    rendered_answer TEXT,

    -- Categorización
    category_id UUID REFERENCES kb.categories(category_id) ON DELETE SET NULL,
    tags JSONB NOT NULL DEFAULT '[]',

    -- Orden y visibilidad
    order_index INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT TRUE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,

    -- Estadísticas
    view_count BIGINT NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices
CREATE INDEX idx_kb_faqs_tenant ON kb_faqs(tenant_id);
CREATE INDEX idx_kb_faqs_category ON kb_faqs(category_id);
CREATE INDEX idx_kb_faqs_visible ON kb_faqs(is_visible) WHERE is_visible = TRUE;
CREATE INDEX idx_kb_faqs_featured ON kb_faqs(is_featured) WHERE is_featured = TRUE;
CREATE INDEX idx_kb_faqs_tags ON kb_faqs USING GIN (tags);

-- =============================================================================
-- KB SEARCH LOGS (Log de búsquedas para analytics)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb_search_logs (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,
    user_id UUID,

    query VARCHAR(500) NOT NULL,
    results_count INTEGER NOT NULL DEFAULT 0,
    clicked_article_id UUID,

    -- Contexto
    ip_address INET,
    user_agent TEXT,

    searched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices
CREATE INDEX idx_kb_search_logs_tenant ON kb_search_logs(tenant_id);
CREATE INDEX idx_kb_search_logs_user ON kb_search_logs(user_id);
CREATE INDEX idx_kb_search_logs_query ON kb_search_logs(query);
CREATE INDEX idx_kb_search_logs_date ON kb_search_logs(searched_at DESC);
CREATE INDEX idx_kb_search_logs_zero_results ON kb_search_logs(results_count) WHERE results_count = 0;

-- =============================================================================
-- KB RELATED ARTICLES (Artículos relacionados manualmente)
-- =============================================================================

CREATE TABLE IF NOT EXISTS kb.related_articles (
    article_id UUID NOT NULL REFERENCES kb.articles(article_id) ON DELETE CASCADE,
    related_article_id UUID NOT NULL REFERENCES kb.articles(article_id) ON DELETE CASCADE,
    relevance_score DECIMAL(3,2) NOT NULL DEFAULT 1.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (article_id, related_article_id),
    CONSTRAINT check_different_articles CHECK (article_id != related_article_id)
);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Trigger para actualizar updated_at
CREATE OR REPLACE FUNCTION update_kb_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_kb_categories_updated_at
    BEFORE UPDATE ON kb.categories
    FOR EACH ROW EXECUTE FUNCTION update_kb_updated_at();

CREATE TRIGGER update_kb_articles_updated_at
    BEFORE UPDATE ON kb.articles
    FOR EACH ROW EXECUTE FUNCTION update_kb_updated_at();

CREATE TRIGGER update_kb_faqs_updated_at
    BEFORE UPDATE ON kb_faqs
    FOR EACH ROW EXECUTE FUNCTION update_kb_updated_at();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE kb_categories IS 'Categorías para organizar artículos de KB';
COMMENT ON TABLE kb_articles IS 'Artículos de ayuda y documentación';
COMMENT ON TABLE kb_article_versions IS 'Historial de versiones de artículos';
COMMENT ON TABLE kb_article_feedback IS 'Feedback de usuarios sobre artículos';
COMMENT ON TABLE kb_faqs IS 'Preguntas frecuentes (FAQ)';
COMMENT ON TABLE kb_search_logs IS 'Log de búsquedas para analytics y mejoras';
COMMENT ON TABLE kb_related_articles IS 'Relaciones manuales entre artículos';

COMMENT ON COLUMN kb_articles.rendered_html IS 'HTML pre-renderizado para mejor performance';
COMMENT ON COLUMN kb_articles.allowed_roles IS 'Roles permitidos cuando visibility = restricted';
