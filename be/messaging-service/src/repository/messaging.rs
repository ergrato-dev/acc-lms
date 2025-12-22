//! # Messaging Repository
//!
//! Database operations for conversations and messages.

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::*;
use crate::domain::errors::MessagingError;

/// Repository for messaging database operations.
#[derive(Clone)]
pub struct MessagingRepository {
    pool: PgPool,
}

impl MessagingRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // Conversation Operations
    // =========================================================================

    /// Find conversations for a user with pagination.
    pub async fn find_conversations_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
        conversation_type: Option<&str>,
        include_archived: bool,
    ) -> Result<Vec<Conversation>, MessagingError> {
        let mut query = String::from(
            r#"
            SELECT c.*
            FROM conversations c
            INNER JOIN conversation_participants cp ON c.conversation_id = cp.conversation_id
            WHERE cp.user_id = $1 AND cp.left_at IS NULL
            "#
        );

        if !include_archived {
            query.push_str(" AND cp.is_archived = false");
        }

        if conversation_type.is_some() {
            query.push_str(" AND c.conversation_type = $4");
        }

        query.push_str(" ORDER BY c.last_message_at DESC NULLS LAST LIMIT $2 OFFSET $3");

        let conversations = if let Some(conv_type) = conversation_type {
            sqlx::query_as::<_, Conversation>(&query)
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .bind(conv_type)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Conversation>(
                r#"
                SELECT c.*
                FROM conversations c
                INNER JOIN conversation_participants cp ON c.conversation_id = cp.conversation_id
                WHERE cp.user_id = $1 AND cp.left_at IS NULL
                AND ($4::boolean OR cp.is_archived = false)
                ORDER BY c.last_message_at DESC NULLS LAST
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .bind(include_archived)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(conversations)
    }

    /// Count conversations for a user.
    pub async fn count_conversations_for_user(
        &self,
        user_id: Uuid,
        conversation_type: Option<&str>,
        include_archived: bool,
    ) -> Result<i64, MessagingError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(DISTINCT c.conversation_id)
            FROM conversations c
            INNER JOIN conversation_participants cp ON c.conversation_id = cp.conversation_id
            WHERE cp.user_id = $1 AND cp.left_at IS NULL
            AND ($2::text IS NULL OR c.conversation_type = $2)
            AND ($3::boolean OR cp.is_archived = false)
            "#
        )
            .bind(user_id)
            .bind(conversation_type)
            .bind(include_archived)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    /// Find a conversation by ID.
    pub async fn find_conversation_by_id(
        &self,
        conversation_id: Uuid,
    ) -> Result<Option<Conversation>, MessagingError> {
        let conversation = sqlx::query_as::<_, Conversation>(
            "SELECT * FROM conversations WHERE conversation_id = $1"
        )
            .bind(conversation_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(conversation)
    }

    /// Create a new conversation.
    pub async fn create_conversation(
        &self,
        conversation_type: &str,
        title: Option<&str>,
        course_id: Option<Uuid>,
        creator_id: Uuid,
    ) -> Result<Conversation, MessagingError> {
        let conversation = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversations (conversation_type, title, course_id, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
            .bind(conversation_type)
            .bind(title)
            .bind(course_id)
            .bind(creator_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(conversation)
    }

    /// Add a participant to a conversation.
    pub async fn add_participant(
        &self,
        conversation_id: Uuid,
        user_id: Uuid,
        role: &str,
    ) -> Result<ConversationParticipant, MessagingError> {
        let participant = sqlx::query_as::<_, ConversationParticipant>(
            r#"
            INSERT INTO conversation_participants (conversation_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (conversation_id, user_id) DO UPDATE SET left_at = NULL
            RETURNING *
            "#
        )
            .bind(conversation_id)
            .bind(user_id)
            .bind(role)
            .fetch_one(&self.pool)
            .await?;

        Ok(participant)
    }

    /// Find participants for a conversation.
    pub async fn find_participants(
        &self,
        conversation_id: Uuid,
    ) -> Result<Vec<ConversationParticipant>, MessagingError> {
        let participants = sqlx::query_as::<_, ConversationParticipant>(
            r#"
            SELECT * FROM conversation_participants
            WHERE conversation_id = $1 AND left_at IS NULL
            "#
        )
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(participants)
    }

    /// Check if user is a participant in a conversation.
    pub async fn is_participant(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<bool, MessagingError> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM conversation_participants
                WHERE conversation_id = $1 AND user_id = $2 AND left_at IS NULL
            )
            "#
        )
            .bind(conversation_id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(exists.0)
    }

    /// Update participant settings (mute/archive).
    pub async fn update_participant_settings(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        is_muted: Option<bool>,
        is_archived: Option<bool>,
    ) -> Result<(), MessagingError> {
        let mut updates = Vec::new();
        let mut param_count = 2;

        if is_muted.is_some() {
            param_count += 1;
            updates.push(format!("is_muted = ${}", param_count));
        }
        if is_archived.is_some() {
            param_count += 1;
            updates.push(format!("is_archived = ${}", param_count));
        }

        if updates.is_empty() {
            return Ok(());
        }

        let query = format!(
            "UPDATE conversation_participants SET {} WHERE conversation_id = $1 AND user_id = $2",
            updates.join(", ")
        );

        let mut query_builder = sqlx::query(&query)
            .bind(conversation_id)
            .bind(user_id);

        if let Some(muted) = is_muted {
            query_builder = query_builder.bind(muted);
        }
        if let Some(archived) = is_archived {
            query_builder = query_builder.bind(archived);
        }

        query_builder.execute(&self.pool).await?;

        Ok(())
    }

    /// Get participant settings.
    pub async fn get_participant_settings(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Option<ConversationParticipant>, MessagingError> {
        let participant = sqlx::query_as::<_, ConversationParticipant>(
            r#"
            SELECT * FROM conversation_participants
            WHERE conversation_id = $1 AND user_id = $2
            "#
        )
            .bind(conversation_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(participant)
    }

    // =========================================================================
    // Message Operations
    // =========================================================================

    /// Find messages for a conversation with pagination.
    pub async fn find_messages(
        &self,
        conversation_id: Uuid,
        limit: i32,
        offset: i32,
        before: Option<Uuid>,
        after: Option<Uuid>,
    ) -> Result<Vec<Message>, MessagingError> {
        let messages = if let Some(before_id) = before {
            sqlx::query_as::<_, Message>(
                r#"
                SELECT * FROM messages
                WHERE conversation_id = $1
                AND created_at < (SELECT created_at FROM messages WHERE message_id = $4)
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(conversation_id)
                .bind(limit)
                .bind(offset)
                .bind(before_id)
                .fetch_all(&self.pool)
                .await?
        } else if let Some(after_id) = after {
            sqlx::query_as::<_, Message>(
                r#"
                SELECT * FROM messages
                WHERE conversation_id = $1
                AND created_at > (SELECT created_at FROM messages WHERE message_id = $4)
                ORDER BY created_at ASC
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(conversation_id)
                .bind(limit)
                .bind(offset)
                .bind(after_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Message>(
                r#"
                SELECT * FROM messages
                WHERE conversation_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(conversation_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(messages)
    }

    /// Count messages in a conversation.
    pub async fn count_messages(
        &self,
        conversation_id: Uuid,
    ) -> Result<i64, MessagingError> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE conversation_id = $1"
        )
            .bind(conversation_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    /// Find a message by ID.
    pub async fn find_message_by_id(
        &self,
        message_id: Uuid,
    ) -> Result<Option<Message>, MessagingError> {
        let message = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE message_id = $1"
        )
            .bind(message_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(message)
    }

    /// Create a new message.
    pub async fn create_message(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        content: &str,
        message_type: &str,
        reply_to_id: Option<Uuid>,
    ) -> Result<Message, MessagingError> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (conversation_id, sender_id, content, message_type, reply_to_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
            .bind(conversation_id)
            .bind(sender_id)
            .bind(content)
            .bind(message_type)
            .bind(reply_to_id)
            .fetch_one(&self.pool)
            .await?;

        // Update conversation's last_message_at
        sqlx::query(
            "UPDATE conversations SET last_message_at = NOW(), updated_at = NOW() WHERE conversation_id = $1"
        )
            .bind(conversation_id)
            .execute(&self.pool)
            .await?;

        Ok(message)
    }

    /// Update a message.
    pub async fn update_message(
        &self,
        message_id: Uuid,
        content: &str,
    ) -> Result<Message, MessagingError> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            UPDATE messages
            SET content = $2, is_edited = true, updated_at = NOW()
            WHERE message_id = $1
            RETURNING *
            "#
        )
            .bind(message_id)
            .bind(content)
            .fetch_one(&self.pool)
            .await?;

        Ok(message)
    }

    /// Soft delete a message.
    pub async fn soft_delete_message(
        &self,
        message_id: Uuid,
    ) -> Result<(), MessagingError> {
        sqlx::query(
            r#"
            UPDATE messages
            SET is_deleted = true, content = '[Message deleted]', updated_at = NOW()
            WHERE message_id = $1
            "#
        )
            .bind(message_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Update last read timestamp.
    pub async fn update_last_read(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), MessagingError> {
        sqlx::query(
            r#"
            UPDATE conversation_participants
            SET last_read_at = NOW()
            WHERE conversation_id = $1 AND user_id = $2
            "#
        )
            .bind(conversation_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get unread count for a user in a conversation.
    pub async fn get_unread_count(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<i64, MessagingError> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM messages m
            INNER JOIN conversation_participants cp
                ON m.conversation_id = cp.conversation_id AND cp.user_id = $1
            WHERE m.conversation_id = $2
            AND m.sender_id != $1
            AND (cp.last_read_at IS NULL OR m.created_at > cp.last_read_at)
            "#
        )
            .bind(user_id)
            .bind(conversation_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    /// Get all unread counts for a user.
    pub async fn get_all_unread_counts(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ConversationUnreadCount>, MessagingError> {
        let counts = sqlx::query_as::<_, ConversationUnreadCount>(
            r#"
            SELECT
                cp.conversation_id,
                COUNT(m.message_id)::bigint AS unread_count
            FROM conversation_participants cp
            LEFT JOIN messages m ON m.conversation_id = cp.conversation_id
                AND m.sender_id != $1
                AND (cp.last_read_at IS NULL OR m.created_at > cp.last_read_at)
            WHERE cp.user_id = $1 AND cp.left_at IS NULL
            GROUP BY cp.conversation_id
            HAVING COUNT(m.message_id) > 0
            "#
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(counts)
    }

    // =========================================================================
    // Search Operations
    // =========================================================================

    /// Search messages across conversations.
    pub async fn search_messages(
        &self,
        user_id: Uuid,
        query: &str,
        conversation_id: Option<Uuid>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Message>, MessagingError> {
        let search_pattern = format!("%{}%", query);

        let messages = if let Some(conv_id) = conversation_id {
            sqlx::query_as::<_, Message>(
                r#"
                SELECT m.*
                FROM messages m
                INNER JOIN conversation_participants cp
                    ON m.conversation_id = cp.conversation_id
                WHERE cp.user_id = $1
                AND m.conversation_id = $4
                AND m.content ILIKE $2
                AND m.is_deleted = false
                ORDER BY m.created_at DESC
                LIMIT $3 OFFSET $5
                "#
            )
                .bind(user_id)
                .bind(&search_pattern)
                .bind(limit)
                .bind(conv_id)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Message>(
                r#"
                SELECT m.*
                FROM messages m
                INNER JOIN conversation_participants cp
                    ON m.conversation_id = cp.conversation_id
                WHERE cp.user_id = $1
                AND m.content ILIKE $2
                AND m.is_deleted = false
                ORDER BY m.created_at DESC
                LIMIT $3 OFFSET $4
                "#
            )
                .bind(user_id)
                .bind(&search_pattern)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(messages)
    }

    /// Count search results.
    pub async fn count_search_results(
        &self,
        user_id: Uuid,
        query: &str,
        conversation_id: Option<Uuid>,
    ) -> Result<i64, MessagingError> {
        let search_pattern = format!("%{}%", query);

        let count: (i64,) = if let Some(conv_id) = conversation_id {
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM messages m
                INNER JOIN conversation_participants cp
                    ON m.conversation_id = cp.conversation_id
                WHERE cp.user_id = $1
                AND m.conversation_id = $3
                AND m.content ILIKE $2
                AND m.is_deleted = false
                "#
            )
                .bind(user_id)
                .bind(&search_pattern)
                .bind(conv_id)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM messages m
                INNER JOIN conversation_participants cp
                    ON m.conversation_id = cp.conversation_id
                WHERE cp.user_id = $1
                AND m.content ILIKE $2
                AND m.is_deleted = false
                "#
            )
                .bind(user_id)
                .bind(&search_pattern)
                .fetch_one(&self.pool)
                .await?
        };

        Ok(count.0)
    }

    /// Find existing direct conversation between two users.
    pub async fn find_direct_conversation(
        &self,
        user1_id: Uuid,
        user2_id: Uuid,
    ) -> Result<Option<Conversation>, MessagingError> {
        let conversation = sqlx::query_as::<_, Conversation>(
            r#"
            SELECT c.*
            FROM conversations c
            INNER JOIN conversation_participants cp1
                ON c.conversation_id = cp1.conversation_id AND cp1.user_id = $1
            INNER JOIN conversation_participants cp2
                ON c.conversation_id = cp2.conversation_id AND cp2.user_id = $2
            WHERE c.conversation_type = 'direct'
            AND cp1.left_at IS NULL
            AND cp2.left_at IS NULL
            LIMIT 1
            "#
        )
            .bind(user1_id)
            .bind(user2_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(conversation)
    }

    /// Leave a conversation (set left_at timestamp).
    pub async fn leave_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), MessagingError> {
        sqlx::query(
            r#"
            UPDATE conversation_participants
            SET left_at = NOW()
            WHERE conversation_id = $1 AND user_id = $2
            "#
        )
            .bind(conversation_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
