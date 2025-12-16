-- =============================================================================
-- ACC LMS - Compliance Database Schema
-- =============================================================================
-- Esquema para compliance multi-jurisdiccional:
-- - Colombia: Ley 1581/2012 (Habeas Data), derechos ARCO
-- - EU: GDPR (Arts. 15-22)
-- - California: CCPA/CPRA
-- - Brasil: LGPD
-- =============================================================================

-- Set search path to compliance schema
SET search_path TO compliance, public;

-- =============================================================================
-- ENUMS (in compliance schema)
-- =============================================================================

-- Jurisdicciones soportadas
CREATE TYPE compliance.jurisdiction AS ENUM (
    'colombia',     -- Ley 1581/2012 (Habeas Data)
    'gdpr',         -- EU General Data Protection Regulation
    'ccpa',         -- California Consumer Privacy Act / CPRA
    'lgpd',         -- Lei Geral de Proteção de Dados (Brasil)
    'general'       -- Jurisdicción genérica
);

-- Tipos de derechos de datos (unificado todas las jurisdicciones)
CREATE TYPE compliance.data_right_type AS ENUM (
    -- ARCO (Colombia)
    'access',           -- Conocer/Acceder a datos
    'rectification',    -- Actualizar/Rectificar
    'erasure',          -- Cancelación/Supresión/Eliminación
    'objection',        -- Oposición

    -- GDPR adicionales
    'restriction',      -- Limitación del tratamiento
    'portability',      -- Portabilidad
    'automated_decision', -- Decisiones automatizadas

    -- CCPA adicionales
    'opt_out_sale',     -- No vender datos
    'opt_out_sharing',  -- No compartir datos
    'limit_sensitive',  -- Limitar uso de datos sensibles

    -- LGPD adicionales
    'confirmation',     -- Confirmación de tratamiento
    'anonymization',    -- Anonimización

    -- General
    'revoke_consent'    -- Revocar consentimiento
);

-- Estados de solicitud de derechos
CREATE TYPE compliance.request_status AS ENUM (
    'received',         -- Recibida
    'identity_pending', -- Pendiente verificación de identidad
    'in_progress',      -- En procesamiento
    'awaiting_info',    -- Esperando información adicional
    'resolved',         -- Resuelta/Completada
    'denied',           -- Denegada (con justificación)
    'appealed',         -- Apelada
    'expired'           -- Expirada
);

-- Tipos de consentimiento
CREATE TYPE compliance.consent_type AS ENUM (
    'terms_of_service',
    'privacy_policy',
    'cookies_functional',
    'cookies_analytics',
    'cookies_marketing',
    'newsletter',
    'third_party_sharing',
    'profiling',
    'international_transfer',
    'data_sale'         -- CCPA opt-in
);

-- Estados de exportación
CREATE TYPE compliance.export_status AS ENUM (
    'pending',
    'processing',
    'completed',
    'failed',
    'expired'
);

-- Categorías de datos para exportación
CREATE TYPE compliance.data_category AS ENUM (
    'profile',
    'preferences',
    'enrollments',
    'certificates',
    'purchases',
    'communications',
    'activity_logs',
    'content_created',
    'consents'
);

-- Estados de eliminación de cuenta
CREATE TYPE compliance.deletion_status AS ENUM (
    'scheduled',        -- Programada
    'in_progress',      -- En proceso
    'completed',        -- Completada
    'cancelled'         -- Cancelada por usuario
);

-- =============================================================================
-- DATA RIGHTS REQUESTS (Solicitudes de derechos ARCO/GDPR/CCPA/LGPD)
-- =============================================================================

CREATE TABLE IF NOT EXISTS compliance.data_rights_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Información del solicitante
    user_id UUID,                                      -- NULL si no está registrado
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    document_type VARCHAR(50),                         -- CC, CE, Passport, etc.
    document_number VARCHAR(100),

    -- Representante (si aplica)
    is_representative BOOLEAN NOT NULL DEFAULT FALSE,
    represented_name VARCHAR(255),

    -- Solicitud
    jurisdiction jurisdiction NOT NULL,
    right_type data_right_type NOT NULL,
    status request_status NOT NULL DEFAULT 'received',

    -- Detalles
    data_categories TEXT[],                            -- Categorías de datos afectadas
    specific_request TEXT,                             -- Descripción específica
    reason TEXT,                                       -- Razón/justificación

    -- Plazos legales
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deadline_at TIMESTAMPTZ NOT NULL,                  -- Fecha límite legal
    extended_deadline_at TIMESTAMPTZ,                  -- Si se extendió el plazo

    -- Verificación de identidad
    identity_verified BOOLEAN NOT NULL DEFAULT FALSE,
    identity_verified_at TIMESTAMPTZ,
    identity_verified_by UUID,
    identity_method VARCHAR(100),                      -- email, document, video, etc.

    -- Resolución
    resolved_at TIMESTAMPTZ,
    resolved_by UUID,
    decision TEXT,                                     -- approved, partially_approved, denied
    explanation TEXT,                                  -- Explicación de la decisión

    -- Apelación
    appealed_at TIMESTAMPTZ,
    appeal_reason TEXT,
    appeal_resolved_at TIMESTAMPTZ,
    appeal_decision TEXT,

    -- Trazabilidad
    ip_address INET,
    user_agent TEXT,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Índices para búsqueda
    CONSTRAINT fk_data_rights_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE SET NULL
);

-- Índices para data_rights_requests
CREATE INDEX idx_data_rights_user_id ON compliance.data_rights_requests(user_id);
CREATE INDEX idx_data_rights_email ON compliance.data_rights_requests(email);
CREATE INDEX idx_data_rights_jurisdiction ON compliance.data_rights_requests(jurisdiction);
CREATE INDEX idx_data_rights_right_type ON compliance.data_rights_requests(right_type);
CREATE INDEX idx_data_rights_status ON compliance.data_rights_requests(status);
CREATE INDEX idx_data_rights_deadline ON compliance.data_rights_requests(deadline_at);
CREATE INDEX idx_data_rights_tenant ON compliance.data_rights_requests(tenant_id);
CREATE INDEX idx_data_rights_received ON compliance.data_rights_requests(received_at DESC);

-- =============================================================================
-- CONSENT RECORDS (Registros de consentimiento)
-- =============================================================================

CREATE TABLE IF NOT EXISTS compliance.consent_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Usuario
    user_id UUID,                                      -- NULL si anónimo
    anonymous_id VARCHAR(255),                         -- Para usuarios no registrados

    -- Consentimiento
    consent_type consent_type NOT NULL,
    granted BOOLEAN NOT NULL,

    -- Versiones
    policy_version VARCHAR(50) NOT NULL,              -- Versión de la política
    consent_text TEXT,                                 -- Texto mostrado al usuario

    -- Contexto
    source VARCHAR(100) NOT NULL,                      -- registration, banner, settings, etc.
    ip_address INET,
    user_agent TEXT,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamps
    consented_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,                            -- Algunos consentimientos expiran
    withdrawn_at TIMESTAMPTZ,                          -- Si fue revocado

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Índices para búsqueda
    CONSTRAINT fk_consent_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE SET NULL
);

-- Índices para consent_records
CREATE INDEX idx_consent_user_id ON compliance.consent_records(user_id);
CREATE INDEX idx_consent_anonymous_id ON compliance.consent_records(anonymous_id);
CREATE INDEX idx_consent_type ON compliance.consent_records(consent_type);
CREATE INDEX idx_consent_granted ON compliance.consent_records(granted);
CREATE INDEX idx_consent_tenant ON compliance.consent_records(tenant_id);
CREATE INDEX idx_consent_date ON compliance.consent_records(consented_at DESC);

-- =============================================================================
-- COOKIE PREFERENCES (Preferencias de cookies)
-- =============================================================================

CREATE TABLE IF NOT EXISTS cookie_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Usuario
    user_id UUID,
    anonymous_id VARCHAR(255),

    -- Preferencias por categoría
    essential BOOLEAN NOT NULL DEFAULT TRUE,           -- Siempre true
    functional BOOLEAN NOT NULL DEFAULT FALSE,
    analytics BOOLEAN NOT NULL DEFAULT FALSE,
    marketing BOOLEAN NOT NULL DEFAULT FALSE,
    social_media BOOLEAN NOT NULL DEFAULT FALSE,

    -- Versión de la política
    policy_version VARCHAR(50) NOT NULL,

    -- Trazabilidad
    ip_address INET,
    user_agent TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Único por usuario o anonymous_id
    CONSTRAINT fk_cookie_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE CASCADE
);

-- Índices para cookie_preferences
CREATE INDEX idx_cookie_user_id ON cookie_preferences(user_id);
CREATE INDEX idx_cookie_anonymous_id ON cookie_preferences(anonymous_id);

-- Garantizar un solo registro por usuario/anonymous_id
CREATE UNIQUE INDEX idx_cookie_unique_user ON cookie_preferences(user_id)
    WHERE user_id IS NOT NULL;
CREATE UNIQUE INDEX idx_cookie_unique_anonymous ON cookie_preferences(anonymous_id)
    WHERE anonymous_id IS NOT NULL AND user_id IS NULL;

-- =============================================================================
-- DATA EXPORTS (Exportaciones de datos - Portabilidad)
-- =============================================================================

CREATE TABLE IF NOT EXISTS compliance.data_exports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Usuario
    user_id UUID NOT NULL,

    -- Solicitud relacionada (si viene de una solicitud formal)
    request_id UUID,

    -- Configuración
    format VARCHAR(20) NOT NULL DEFAULT 'json',        -- json, csv, xml, zip
    categories data_category[] NOT NULL,

    -- Estado
    status export_status NOT NULL DEFAULT 'pending',

    -- Archivos generados
    file_path TEXT,
    file_size_bytes BIGINT,
    file_hash VARCHAR(64),                             -- SHA-256 para integridad

    -- URLs de descarga
    download_url TEXT,
    download_expires_at TIMESTAMPTZ,
    download_count INTEGER NOT NULL DEFAULT 0,
    max_downloads INTEGER NOT NULL DEFAULT 5,

    -- Procesamiento
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Trazabilidad
    ip_address INET,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Foreign keys
    CONSTRAINT fk_export_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_export_request FOREIGN KEY (request_id)
        REFERENCES data_rights_requests(id) ON DELETE SET NULL
);

-- Índices para data_exports
CREATE INDEX idx_export_user_id ON compliance.data_exports(user_id);
CREATE INDEX idx_export_status ON compliance.data_exports(status);
CREATE INDEX idx_export_request ON compliance.data_exports(request_id);
CREATE INDEX idx_export_tenant ON compliance.data_exports(tenant_id);

-- =============================================================================
-- DELETION REQUESTS (Solicitudes de eliminación de cuenta)
-- =============================================================================

CREATE TABLE IF NOT EXISTS deletion_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Usuario
    user_id UUID NOT NULL,

    -- Solicitud relacionada (si viene de derecho al olvido)
    request_id UUID,

    -- Detalles
    reason TEXT,
    status deletion_status NOT NULL DEFAULT 'scheduled',

    -- Programación
    scheduled_at TIMESTAMPTZ NOT NULL,                 -- Fecha programada para eliminación
    grace_period_ends TIMESTAMPTZ NOT NULL,            -- Período de gracia para cancelar

    -- Confirmación
    confirmation_token VARCHAR(64),
    confirmed_at TIMESTAMPTZ,

    -- Ejecución
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Cancelación
    cancelled_at TIMESTAMPTZ,
    cancelled_reason TEXT,

    -- Datos eliminados (resumen)
    deleted_data_summary JSONB,                        -- { "profile": true, "enrollments": 5, ... }

    -- Trazabilidad
    ip_address INET,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Foreign keys
    CONSTRAINT fk_deletion_user FOREIGN KEY (user_id)
        REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_deletion_request FOREIGN KEY (request_id)
        REFERENCES data_rights_requests(id) ON DELETE SET NULL
);

-- Índices para deletion_requests
CREATE INDEX idx_deletion_user_id ON deletion_requests(user_id);
CREATE INDEX idx_deletion_status ON deletion_requests(status);
CREATE INDEX idx_deletion_scheduled ON deletion_requests(scheduled_at);
CREATE INDEX idx_deletion_grace ON deletion_requests(grace_period_ends);
CREATE INDEX idx_deletion_tenant ON deletion_requests(tenant_id);

-- =============================================================================
-- COMPLIANCE AUDIT LOG (Log de auditoría para compliance)
-- =============================================================================

CREATE TABLE IF NOT EXISTS compliance_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Actor
    actor_id UUID,                                     -- Usuario que realizó la acción
    actor_type VARCHAR(50) NOT NULL,                   -- user, admin, system

    -- Target
    target_type VARCHAR(50) NOT NULL,                  -- request, consent, export, deletion
    target_id UUID NOT NULL,

    -- Acción
    action VARCHAR(100) NOT NULL,                      -- created, updated, resolved, etc.
    description TEXT,

    -- Datos cambiados
    old_values JSONB,
    new_values JSONB,

    -- Contexto
    ip_address INET,
    user_agent TEXT,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices para compliance_audit_log
CREATE INDEX idx_audit_actor ON compliance_audit_log(actor_id);
CREATE INDEX idx_audit_target ON compliance_audit_log(target_type, target_id);
CREATE INDEX idx_audit_action ON compliance_audit_log(action);
CREATE INDEX idx_audit_tenant ON compliance_audit_log(tenant_id);
CREATE INDEX idx_audit_date ON compliance_audit_log(created_at DESC);

-- =============================================================================
-- POLICY VERSIONS (Versiones de políticas)
-- =============================================================================

CREATE TABLE IF NOT EXISTS policy_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Tipo de política
    policy_type VARCHAR(50) NOT NULL,                  -- privacy, terms, cookies

    -- Versión
    version VARCHAR(50) NOT NULL,

    -- Contenido
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,                                      -- Resumen de cambios

    -- Jurisdicciones aplicables
    jurisdictions jurisdiction[] NOT NULL DEFAULT '{general}',

    -- Vigencia
    effective_from TIMESTAMPTZ NOT NULL,
    effective_until TIMESTAMPTZ,

    -- Estado
    is_current BOOLEAN NOT NULL DEFAULT FALSE,

    -- Multi-tenant
    tenant_id UUID,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,

    CONSTRAINT unique_policy_version UNIQUE (policy_type, version, tenant_id)
);

-- Índices para policy_versions
CREATE INDEX idx_policy_type ON policy_versions(policy_type);
CREATE INDEX idx_policy_current ON policy_versions(is_current) WHERE is_current = TRUE;
CREATE INDEX idx_policy_tenant ON policy_versions(tenant_id);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Trigger para actualizar updated_at
CREATE OR REPLACE FUNCTION update_compliance_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_data_rights_requests_updated_at
    BEFORE UPDATE ON compliance.data_rights_requests
    FOR EACH ROW EXECUTE FUNCTION update_compliance_updated_at();

CREATE TRIGGER update_cookie_preferences_updated_at
    BEFORE UPDATE ON cookie_preferences
    FOR EACH ROW EXECUTE FUNCTION update_compliance_updated_at();

CREATE TRIGGER update_data_exports_updated_at
    BEFORE UPDATE ON compliance.data_exports
    FOR EACH ROW EXECUTE FUNCTION update_compliance_updated_at();

CREATE TRIGGER update_deletion_requests_updated_at
    BEFORE UPDATE ON deletion_requests
    FOR EACH ROW EXECUTE FUNCTION update_compliance_updated_at();

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE data_rights_requests IS 'Solicitudes de derechos de datos (ARCO/GDPR/CCPA/LGPD)';
COMMENT ON TABLE consent_records IS 'Registros históricos de consentimiento';
COMMENT ON TABLE cookie_preferences IS 'Preferencias actuales de cookies por usuario';
COMMENT ON TABLE data_exports IS 'Solicitudes de exportación de datos (portabilidad)';
COMMENT ON TABLE deletion_requests IS 'Solicitudes de eliminación de cuenta';
COMMENT ON TABLE compliance_audit_log IS 'Log de auditoría para compliance';
COMMENT ON TABLE policy_versions IS 'Versiones históricas de políticas';

COMMENT ON COLUMN data_rights_requests.deadline_at IS 'Plazo legal: Colombia 15d hábiles, GDPR 30d, CCPA 45d, LGPD 15d';
COMMENT ON COLUMN deletion_requests.grace_period_ends IS 'Período de gracia para cancelar (típicamente 7-30 días)';
