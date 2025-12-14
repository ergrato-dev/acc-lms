-- Migration: 004_ai_and_notifications.sql  
-- Description: AI embeddings, conversations and notification system
-- Author: System
-- Date: 2025-08-08

-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- ========================================
-- AI & VECTORS DOMAIN
-- ========================================

-- Embeddings for semantic search
CREATE TABLE content_embeddings (
    embedding_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_type TEXT NOT NULL CHECK (content_type IN ('course', 'lesson', 'quiz', 'user_query')),
    content_id UUID NOT NULL,
    text_content TEXT NOT NULL,
    embedding vector(1536), -- OpenAI ada-002 dimension
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_content_embeddings_type ON content_embeddings(content_type);
CREATE INDEX idx_content_embeddings_vector ON content_embeddings USING ivfflat (embedding vector_cosine_ops);

-- AI chat conversations
CREATE TABLE ai_conversations (
    conversation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id),
    course_id UUID REFERENCES courses(course_id),
    title TEXT,
    status TEXT NOT NULL CHECK (status IN ('active', 'archived')) DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Individual chat messages
CREATE TABLE ai_messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES ai_conversations(conversation_id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    tokens_used INTEGER DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- ========================================
-- NOTIFICATIONS SYSTEM  
-- ========================================

-- Notification templates
CREATE TABLE notification_templates (
    template_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT UNIQUE NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('email', 'push', 'in_app')),
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- User notification queue
CREATE TABLE user_notifications (
    notification_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id),
    template_id UUID NOT NULL REFERENCES notification_templates(template_id),
    type TEXT NOT NULL,
    subject TEXT,
    content TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'sent', 'failed', 'read')) DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 1,
    scheduled_for TIMESTAMP NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMP,
    read_at TIMESTAMP,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_notifications_user_id ON user_notifications(user_id);
CREATE INDEX idx_user_notifications_status ON user_notifications(status);
CREATE INDEX idx_user_notifications_scheduled ON user_notifications(scheduled_for);

-- Performance indexes for AI
CREATE INDEX idx_ai_conversations_user_id ON ai_conversations(user_id);
CREATE INDEX idx_ai_messages_conversation_id ON ai_messages(conversation_id);

-- Default notification templates
INSERT INTO notification_templates (name, type, subject_template, body_template, variables) VALUES
    ('welcome_email', 'email', 'Welcome to ACC LMS!', 'Hi {{firstName}}, welcome to our learning platform!', '["firstName"]'),
    ('course_enrollment', 'email', 'Course Enrollment Confirmed', 'You have successfully enrolled in {{courseTitle}}', '["courseTitle"]'),
    ('quiz_completed', 'in_app', null, 'Quiz completed with score: {{score}}%', '["score"]'),
    ('course_completed', 'email', 'Congratulations! Course Completed', 'You have completed {{courseTitle}}. Download your certificate!', '["courseTitle"]');
