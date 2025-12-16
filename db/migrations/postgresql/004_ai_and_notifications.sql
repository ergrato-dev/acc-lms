-- Migration: 004_ai_and_notifications.sql
-- Description: AI embeddings, conversations and notification system
-- Author: System
-- Date: 2025-08-08
-- Updated: 2025-12-15 (Schema separation)
--
-- PREREQUISITE: Run 000_schema_setup.sql and 001_initial_schema.sql first
--
-- This migration creates tables in two schemas:
-- - ai.* : Embeddings, conversations, AI features
-- - notifications.* : Templates, delivery queue

-- Enable pgvector extension (required for embeddings)
CREATE EXTENSION IF NOT EXISTS vector;

-- ========================================
-- AI SCHEMA: Embeddings & Conversations
-- ========================================

-- Embeddings for semantic search
CREATE TABLE ai.content_embeddings (
    embedding_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type TEXT NOT NULL CHECK (content_type IN ('course', 'lesson', 'quiz', 'user_query')),
    content_id UUID NOT NULL,
    text_content TEXT NOT NULL,
    embedding vector(1536), -- OpenAI ada-002 dimension
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_embeddings_type ON ai.content_embeddings(content_type);
CREATE INDEX idx_ai_embeddings_content_id ON ai.content_embeddings(content_id);
-- Note: IVFFlat index requires data to be present first, create after initial data load
-- CREATE INDEX idx_ai_embeddings_vector ON ai.content_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

-- AI chat conversations
CREATE TABLE ai.conversations (
    conversation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL, -- References auth.users(user_id)
    course_id UUID, -- References courses.courses(course_id), optional for general AI chat
    title TEXT,
    status TEXT NOT NULL CHECK (status IN ('active', 'archived')) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_conversations_user_id ON ai.conversations(user_id);
CREATE INDEX idx_ai_conversations_course_id ON ai.conversations(course_id);
CREATE INDEX idx_ai_conversations_status ON ai.conversations(status);

-- Individual chat messages
CREATE TABLE ai.messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES ai.conversations(conversation_id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    tokens_used INTEGER DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_messages_conversation_id ON ai.messages(conversation_id);
CREATE INDEX idx_ai_messages_role ON ai.messages(role);

-- Trigger
CREATE TRIGGER ai_conversations_updated_at
    BEFORE UPDATE ON ai.conversations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- NOTIFICATIONS SCHEMA: Templates & Queue
-- ========================================

-- Notification templates
CREATE TABLE notifications.templates (
    template_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('email', 'push', 'in_app', 'sms')),
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_templates_name ON notifications.templates(name);
CREATE INDEX idx_notifications_templates_type ON notifications.templates(type);

-- User notification queue
CREATE TABLE notifications.queue (
    notification_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL, -- References auth.users(user_id)
    template_id UUID NOT NULL REFERENCES notifications.templates(template_id),
    type TEXT NOT NULL,
    subject TEXT,
    content TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'sent', 'failed', 'read')) DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 1 CHECK (priority >= 1 AND priority <= 5),
    scheduled_for TIMESTAMP NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMP,
    read_at TIMESTAMP,
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_queue_user_id ON notifications.queue(user_id);
CREATE INDEX idx_notifications_queue_status ON notifications.queue(status);
CREATE INDEX idx_notifications_queue_scheduled ON notifications.queue(scheduled_for) WHERE status = 'pending';
CREATE INDEX idx_notifications_queue_priority ON notifications.queue(priority DESC, scheduled_for ASC) WHERE status = 'pending';

-- User notification preferences (what types they want to receive)
CREATE TABLE notifications.user_settings (
    user_id UUID PRIMARY KEY, -- References auth.users(user_id)
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    push_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    sms_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    quiet_hours_start TIME,
    quiet_hours_end TIME,
    timezone TEXT DEFAULT 'America/Mexico_City',
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Trigger
CREATE TRIGGER notifications_templates_updated_at
    BEFORE UPDATE ON notifications.templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER notifications_user_settings_updated_at
    BEFORE UPDATE ON notifications.user_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ========================================
-- SEED DATA: Default notification templates
-- ========================================

INSERT INTO notifications.templates (name, type, subject_template, body_template, variables) VALUES
    ('welcome_email', 'email', '¡Bienvenido a ACC LMS!', 'Hola {{firstName}}, bienvenido a nuestra plataforma de aprendizaje.', '["firstName"]'),
    ('course_enrollment', 'email', 'Inscripción confirmada', 'Te has inscrito exitosamente en {{courseTitle}}', '["courseTitle"]'),
    ('course_enrollment_push', 'push', NULL, '¡Inscrito en {{courseTitle}}!', '["courseTitle"]'),
    ('quiz_completed', 'in_app', NULL, 'Quiz completado con puntuación: {{score}}%', '["score"]'),
    ('course_completed', 'email', '¡Felicidades! Curso completado', 'Has completado {{courseTitle}}. ¡Descarga tu certificado!', '["courseTitle"]'),
    ('payment_success', 'email', 'Pago recibido', 'Tu pago de {{amount}} {{currency}} ha sido procesado exitosamente.', '["amount", "currency"]'),
    ('password_reset', 'email', 'Restablecer contraseña', 'Haz clic en el siguiente enlace para restablecer tu contraseña: {{resetLink}}', '["resetLink"]'),
    ('lesson_reminder', 'push', NULL, 'Continúa tu progreso en {{courseTitle}}', '["courseTitle"]');
