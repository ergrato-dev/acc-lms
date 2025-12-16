-- Migration: 006_chatbot.sql
-- Description: Chatbot tables for conversations, messages, and knowledge base
-- Author: System
-- Date: 2025-12-15
--
-- This migration creates tables for the chatbot system:
-- - chatbot.conversations : Chat sessions with context
-- - chatbot.messages      : Individual messages with feedback
-- - chatbot.kb_articles   : Knowledge base articles
-- - chatbot.suggestions   : Contextual suggestions

-- ========================================
-- CHATBOT SCHEMA SETUP
-- ========================================

CREATE SCHEMA IF NOT EXISTS chatbot;

GRANT USAGE ON SCHEMA chatbot TO PUBLIC;
GRANT ALL ON ALL TABLES IN SCHEMA chatbot TO PUBLIC;
ALTER DEFAULT PRIVILEGES IN SCHEMA chatbot GRANT ALL ON TABLES TO PUBLIC;

-- ========================================
-- ENUMS
-- ========================================

CREATE TYPE chatbot.user_role AS ENUM (
    'anonymous',
    'student',
    'instructor',
    'admin'
);

CREATE TYPE chatbot.conversation_status AS ENUM (
    'active',
    'escalated',
    'resolved',
    'abandoned'
);

CREATE TYPE chatbot.message_sender AS ENUM (
    'user',
    'bot',
    'system',
    'agent'
);

CREATE TYPE chatbot.article_status AS ENUM (
    'draft',
    'published',
    'archived'
);

-- ========================================
-- CONVERSATIONS TABLE
-- ========================================

CREATE TABLE chatbot.conversations (
    conversation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,
    user_id UUID,
    user_role VARCHAR(20) NOT NULL DEFAULT 'anonymous',
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    message_count INTEGER NOT NULL DEFAULT 0,
    context JSONB NOT NULL DEFAULT '{}',
    escalation JSONB,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for conversations
CREATE INDEX idx_conversations_user_id ON chatbot.conversations(user_id);
CREATE INDEX idx_conversations_tenant_id ON chatbot.conversations(tenant_id);
CREATE INDEX idx_conversations_status ON chatbot.conversations(status);
CREATE INDEX idx_conversations_started_at ON chatbot.conversations(started_at);
CREATE INDEX idx_conversations_last_activity ON chatbot.conversations(last_activity_at);

-- ========================================
-- MESSAGES TABLE
-- ========================================

CREATE TABLE chatbot.messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES chatbot.conversations(conversation_id) ON DELETE CASCADE,
    sender VARCHAR(20) NOT NULL,
    content JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    intent JSONB,
    confidence FLOAT,
    feedback JSONB,
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- Indexes for messages
CREATE INDEX idx_messages_conversation ON chatbot.messages(conversation_id);
CREATE INDEX idx_messages_timestamp ON chatbot.messages(timestamp);
CREATE INDEX idx_messages_sender ON chatbot.messages(sender);
CREATE INDEX idx_messages_intent ON chatbot.messages USING GIN ((intent->>'name') gin_trgm_ops);

-- ========================================
-- KNOWLEDGE BASE ARTICLES
-- ========================================

CREATE TABLE chatbot.kb_articles (
    article_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(100) UNIQUE NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,
    category VARCHAR(50) NOT NULL,
    subcategory VARCHAR(50),
    tags TEXT[] NOT NULL DEFAULT '{}',
    keywords TEXT[] NOT NULL DEFAULT '{}',
    intent_triggers TEXT[] NOT NULL DEFAULT '{}',
    target_roles TEXT[] NOT NULL DEFAULT '{"anonymous"}',
    language VARCHAR(10) NOT NULL DEFAULT 'es',
    status VARCHAR(20) NOT NULL DEFAULT 'draft',
    view_count BIGINT NOT NULL DEFAULT 0,
    helpful_count BIGINT NOT NULL DEFAULT 0,
    not_helpful_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    author_id UUID NOT NULL,
    -- Full-text search vector
    search_vector tsvector GENERATED ALWAYS AS (
        setweight(to_tsvector('spanish', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('spanish', coalesce(summary, '')), 'B') ||
        setweight(to_tsvector('spanish', coalesce(content, '')), 'C') ||
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(summary, '')), 'B')
    ) STORED
);

-- Indexes for KB articles
CREATE INDEX idx_kb_articles_slug ON chatbot.kb_articles(slug);
CREATE INDEX idx_kb_articles_category ON chatbot.kb_articles(category);
CREATE INDEX idx_kb_articles_status ON chatbot.kb_articles(status);
CREATE INDEX idx_kb_articles_language ON chatbot.kb_articles(language);
CREATE INDEX idx_kb_articles_search ON chatbot.kb_articles USING GIN(search_vector);
CREATE INDEX idx_kb_articles_keywords ON chatbot.kb_articles USING GIN(keywords);
CREATE INDEX idx_kb_articles_intent_triggers ON chatbot.kb_articles USING GIN(intent_triggers);
CREATE INDEX idx_kb_articles_target_roles ON chatbot.kb_articles USING GIN(target_roles);

-- ========================================
-- CONTEXTUAL SUGGESTIONS
-- ========================================

CREATE TABLE chatbot.suggestions (
    suggestion_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text VARCHAR(200) NOT NULL,
    intent VARCHAR(100) NOT NULL,
    target_roles TEXT[] NOT NULL DEFAULT '{"anonymous"}',
    context_conditions JSONB NOT NULL DEFAULT '{}',
    priority INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for suggestions
CREATE INDEX idx_suggestions_active ON chatbot.suggestions(is_active);
CREATE INDEX idx_suggestions_target_roles ON chatbot.suggestions USING GIN(target_roles);
CREATE INDEX idx_suggestions_priority ON chatbot.suggestions(priority DESC);

-- ========================================
-- TRIGGER: Update updated_at on KB articles
-- ========================================

CREATE OR REPLACE FUNCTION chatbot.update_kb_article_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_kb_articles_updated
    BEFORE UPDATE ON chatbot.kb_articles
    FOR EACH ROW
    EXECUTE FUNCTION chatbot.update_kb_article_timestamp();

-- ========================================
-- SEED DATA: Initial suggestions
-- ========================================

INSERT INTO chatbot.suggestions (text, intent, target_roles, context_conditions, priority) VALUES
-- Anonymous suggestions
('¿Cómo puedo registrarme?', 'register', '{"anonymous"}', '{}', 10),
('¿Qué cursos tienen disponibles?', 'browse_courses', '{"anonymous"}', '{}', 9),
('¿Cuánto cuestan los cursos?', 'pricing', '{"anonymous"}', '{}', 8),

-- Student suggestions
('¿Cómo veo mi progreso?', 'check_progress', '{"student"}', '{}', 10),
('¿Cómo descargo mi certificado?', 'get_certificate', '{"student"}', '{}', 9),
('Tengo un problema con un video', 'technical_issue', '{"student"}', '{}', 8),
('¿Cómo contacto al instructor?', 'contact_instructor', '{"student"}', '{}', 7),

-- Instructor suggestions
('¿Cómo creo un nuevo curso?', 'create_course', '{"instructor"}', '{}', 10),
('¿Dónde veo mis ganancias?', 'view_earnings', '{"instructor"}', '{}', 9),
('¿Cómo veo las estadísticas de mis cursos?', 'view_analytics', '{"instructor"}', '{}', 8),

-- Admin suggestions
('Ver estado del sistema', 'system_health', '{"admin"}', '{}', 10),
('Ver reportes de la plataforma', 'view_reports', '{"admin"}', '{}', 9),
('Gestionar usuarios', 'manage_users', '{"admin"}', '{}', 8);

-- ========================================
-- SEED DATA: Initial KB articles
-- ========================================

INSERT INTO chatbot.kb_articles (
    slug, title, content, summary, category, tags, keywords, intent_triggers, target_roles, language, status, author_id
) VALUES
(
    'como-registrarse',
    '¿Cómo me registro en la plataforma?',
    E'# Registro en ACC LMS\n\nPara registrarte en nuestra plataforma, sigue estos pasos:\n\n1. Haz clic en el botón "Registrarse" en la esquina superior derecha\n2. Completa el formulario con tus datos\n3. Verifica tu correo electrónico\n4. ¡Listo! Ya puedes explorar nuestros cursos\n\n## Requisitos\n\n- Correo electrónico válido\n- Contraseña de al menos 8 caracteres\n\n## Problemas comunes\n\n- Si no recibes el correo de verificación, revisa tu carpeta de spam\n- Si el correo ya está registrado, prueba recuperar tu contraseña',
    'Guía paso a paso para registrarte en la plataforma',
    'cuenta',
    '{"registro", "cuenta", "inicio"}',
    '{"registrarse", "crear cuenta", "nuevo usuario"}',
    '{"register", "signup", "crear_cuenta"}',
    '{"anonymous"}',
    'es',
    'published',
    '00000000-0000-0000-0000-000000000000'
),
(
    'ver-progreso',
    '¿Cómo veo mi progreso en un curso?',
    E'# Ver tu progreso\n\nPuedes ver tu progreso de varias formas:\n\n## Desde el Dashboard\n\n1. Inicia sesión en tu cuenta\n2. Ve a "Mi Dashboard"\n3. Verás un resumen de todos tus cursos con el porcentaje completado\n\n## Desde un curso específico\n\n1. Entra al curso\n2. En la barra lateral verás las lecciones completadas con ✓\n3. En la parte superior verás la barra de progreso general\n\n## Detalles del progreso\n\nEl progreso se calcula basándose en:\n- Lecciones completadas\n- Quizzes aprobados\n- Tareas entregadas',
    'Cómo ver y entender tu progreso en los cursos',
    'estudiante',
    '{"progreso", "avance", "porcentaje"}',
    '{"progreso", "avance", "completado", "porcentaje"}',
    '{"check_progress", "my_progress", "course_progress"}',
    '{"student"}',
    'es',
    'published',
    '00000000-0000-0000-0000-000000000000'
),
(
    'obtener-certificado',
    '¿Cómo obtengo mi certificado?',
    E'# Certificados\n\n## Requisitos para obtener certificado\n\nPara obtener tu certificado necesitas:\n\n1. Completar el 100% del contenido del curso\n2. Aprobar todos los quizzes requeridos\n3. Entregar todas las tareas obligatorias\n\n## Descargar certificado\n\n1. Ve a "Mis Certificados" en tu perfil\n2. Busca el curso completado\n3. Haz clic en "Descargar PDF"\n\n## Verificar certificado\n\nCada certificado tiene un código único que puede ser verificado en nuestra página de verificación.',
    'Guía para obtener y descargar tu certificado de curso',
    'estudiante',
    '{"certificado", "diploma", "acreditación"}',
    '{"certificado", "diploma", "descargar", "completado"}',
    '{"get_certificate", "download_certificate", "certificate"}',
    '{"student"}',
    'es',
    'published',
    '00000000-0000-0000-0000-000000000000'
),
(
    'crear-curso-instructor',
    '¿Cómo creo un curso como instructor?',
    E'# Crear un curso\n\n## Pasos para crear tu curso\n\n1. Accede a tu Dashboard de Instructor\n2. Haz clic en "Nuevo Curso"\n3. Completa la información básica:\n   - Título del curso\n   - Descripción\n   - Categoría\n   - Nivel de dificultad\n4. Añade el contenido:\n   - Crea secciones/módulos\n   - Sube videos y materiales\n   - Crea quizzes\n5. Configura el precio\n6. Envía a revisión\n\n## Consejos\n\n- Usa videos de buena calidad\n- Estructura el contenido en secciones claras\n- Incluye recursos descargables\n- Responde las preguntas de los estudiantes',
    'Guía completa para crear tu primer curso en la plataforma',
    'instructor',
    '{"crear curso", "publicar", "instructor"}',
    '{"crear curso", "nuevo curso", "publicar", "contenido"}',
    '{"create_course", "new_course", "publish_course"}',
    '{"instructor"}',
    'es',
    'published',
    '00000000-0000-0000-0000-000000000000'
);

-- ========================================
-- COMMENTS
-- ========================================

COMMENT ON TABLE chatbot.conversations IS 'Chat conversation sessions with users';
COMMENT ON TABLE chatbot.messages IS 'Individual chat messages within conversations';
COMMENT ON TABLE chatbot.kb_articles IS 'Knowledge base articles for chatbot responses';
COMMENT ON TABLE chatbot.suggestions IS 'Contextual suggestions based on user role and page';

COMMENT ON COLUMN chatbot.conversations.context IS 'Conversation context including current page, course, language';
COMMENT ON COLUMN chatbot.conversations.escalation IS 'Escalation info if conversation was escalated to human';
COMMENT ON COLUMN chatbot.messages.intent IS 'Detected intent from NLU analysis';
COMMENT ON COLUMN chatbot.messages.feedback IS 'User feedback (thumbs up/down) for the message';
COMMENT ON COLUMN chatbot.kb_articles.search_vector IS 'Full-text search vector for Spanish and English';
COMMENT ON COLUMN chatbot.kb_articles.intent_triggers IS 'Intent names that should return this article';
