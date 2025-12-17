//! # Conversation Repository
//!
//! Data access for tutor conversation sessions and messages.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, ContentReference, CreateTutorSession,
    MessageRole, TutorMessage, TutorSession, TutorSessionStatus,
};

/// Repository for tutor conversation operations.
#[derive(Clone)]
pub struct ConversationRepository {
    pool: PgPool,
}

impl ConversationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new tutor session.
    pub async fn create_session(&self, request: CreateTutorSession) -> AIResult<TutorSession> {
        let session_id = Uuid::now_v7();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO ai.tutor_sessions (
                session_id, user_id, course_id, lesson_id, status,
                message_count, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(session_id)
        .bind(request.user_id)
        .bind(request.course_id)
        .bind(request.lesson_id)
        .bind(TutorSessionStatus::Active.to_string())
        .bind(0i32)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(TutorSession {
            session_id,
            user_id: request.user_id,
            course_id: request.course_id,
            lesson_id: request.lesson_id,
            status: TutorSessionStatus::Active,
            message_count: 0,
            context_summary: None,
            created_at: now,
            updated_at: now,
            last_message_at: None,
        })
    }

    /// Gets a session by ID.
    pub async fn get_session(&self, session_id: Uuid) -> AIResult<TutorSession> {
        let row = sqlx::query(
            r#"
            SELECT session_id, user_id, course_id, lesson_id, status,
                   message_count, context_summary, created_at, updated_at, last_message_at
            FROM ai.tutor_sessions
            WHERE session_id = $1
            "#
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?
        .ok_or(AIError::SessionNotFound(session_id))?;

        Ok(self.map_session_row(&row))
    }

    /// Gets active session for user and course.
    pub async fn get_active_session(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> AIResult<Option<TutorSession>> {
        let row = sqlx::query(
            r#"
            SELECT session_id, user_id, course_id, lesson_id, status,
                   message_count, context_summary, created_at, updated_at, last_message_at
            FROM ai.tutor_sessions
            WHERE user_id = $1 AND course_id = $2 AND status = 'active'
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(row.map(|r| self.map_session_row(&r)))
    }

    /// Updates session's last message timestamp and count.
    pub async fn update_session_activity(&self, session_id: Uuid) -> AIResult<()> {
        sqlx::query(
            r#"
            UPDATE ai.tutor_sessions
            SET message_count = message_count + 1,
                last_message_at = $2,
                updated_at = $2
            WHERE session_id = $1
            "#
        )
        .bind(session_id)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Updates session context summary.
    pub async fn update_context_summary(&self, session_id: Uuid, summary: &str) -> AIResult<()> {
        sqlx::query(
            r#"
            UPDATE ai.tutor_sessions
            SET context_summary = $2, updated_at = $3
            WHERE session_id = $1
            "#
        )
        .bind(session_id)
        .bind(summary)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Saves a message.
    pub async fn save_message(&self, message: &TutorMessage) -> AIResult<()> {
        let references_json = message.references.as_ref()
            .map(|refs| serde_json::to_value(refs).ok())
            .flatten();

        sqlx::query(
            r#"
            INSERT INTO ai.tutor_messages (
                message_id, session_id, role, content, tokens_used,
                references_json, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(message.message_id)
        .bind(message.session_id)
        .bind(message.role.to_string())
        .bind(&message.content)
        .bind(message.tokens_used)
        .bind(references_json)
        .bind(message.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets messages for a session.
    pub async fn get_messages(
        &self,
        session_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AIResult<Vec<TutorMessage>> {
        let rows = sqlx::query(
            r#"
            SELECT message_id, session_id, role, content, tokens_used,
                   references_json, created_at
            FROM ai.tutor_messages
            WHERE session_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(session_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(rows.iter().map(|r| self.map_message_row(r)).collect())
    }

    /// Gets recent messages for context.
    pub async fn get_recent_messages(&self, session_id: Uuid, count: i32) -> AIResult<Vec<TutorMessage>> {
        let rows = sqlx::query(
            r#"
            SELECT message_id, session_id, role, content, tokens_used,
                   references_json, created_at
            FROM ai.tutor_messages
            WHERE session_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#
        )
        .bind(session_id)
        .bind(count)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        // Reverse to get chronological order
        let mut messages: Vec<TutorMessage> = rows.iter().map(|r| self.map_message_row(r)).collect();
        messages.reverse();
        Ok(messages)
    }

    // Helper methods
    fn map_session_row(&self, row: &sqlx::postgres::PgRow) -> TutorSession {
        use sqlx::Row;

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "active" => TutorSessionStatus::Active,
            "paused" => TutorSessionStatus::Paused,
            "completed" => TutorSessionStatus::Completed,
            "expired" => TutorSessionStatus::Expired,
            _ => TutorSessionStatus::Active,
        };

        TutorSession {
            session_id: row.get("session_id"),
            user_id: row.get("user_id"),
            course_id: row.get("course_id"),
            lesson_id: row.get("lesson_id"),
            status,
            message_count: row.get("message_count"),
            context_summary: row.get("context_summary"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_message_at: row.get("last_message_at"),
        }
    }

    fn map_message_row(&self, row: &sqlx::postgres::PgRow) -> TutorMessage {
        use sqlx::Row;

        let role_str: String = row.get("role");
        let role = match role_str.as_str() {
            "user" => MessageRole::User,
            "assistant" => MessageRole::Assistant,
            "system" => MessageRole::System,
            _ => MessageRole::User,
        };

        let references_json: Option<serde_json::Value> = row.get("references_json");
        let references = references_json
            .and_then(|v| serde_json::from_value::<Vec<ContentReference>>(v).ok());

        TutorMessage {
            message_id: row.get("message_id"),
            session_id: row.get("session_id"),
            role,
            content: row.get("content"),
            tokens_used: row.get("tokens_used"),
            references,
            created_at: row.get("created_at"),
        }
    }
}
