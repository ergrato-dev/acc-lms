//! # Chatbot Repository
//!
//! Repository implementation for chatbot operations using PostgreSQL and Redis.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::{
    Conversation, ConversationContext, ConversationStatus, EscalationInfo, EscalationReason,
    FeedbackType, KBArticle, ArticleStatus, KBSearchResult, Message, MessageContent,
    MessageFeedback, MessageSender, NewConversation, NewKBArticle, UserRole, ContentType,
    ChatbotAnalytics, IntentStat, UnansweredQuery, ContextualSuggestion,
};

// =============================================================================
// REPOSITORY ERRORS
// =============================================================================

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

// =============================================================================
// CHATBOT REPOSITORY
// =============================================================================

/// Repository for chatbot operations.
#[derive(Clone)]
pub struct ChatbotRepository {
    pool: PgPool,
}

impl ChatbotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // CONVERSATION OPERATIONS
    // =========================================================================

    /// Creates a new conversation.
    pub async fn create_conversation(&self, request: NewConversation) -> Result<Conversation> {
        let conversation = Conversation::new(request);
        let context_json = serde_json::to_value(&conversation.context)?;
        let metadata_json = serde_json::to_value(&conversation.metadata)?;

        sqlx::query(r#"
            INSERT INTO chatbot.conversations (
                conversation_id, tenant_id, user_id, user_role, status,
                started_at, last_activity_at, message_count, context, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#)
        .bind(conversation.conversation_id)
        .bind(conversation.tenant_id)
        .bind(conversation.user_id)
        .bind(conversation.user_role.to_string())
        .bind("active")
        .bind(conversation.started_at)
        .bind(conversation.last_activity_at)
        .bind(conversation.message_count)
        .bind(&context_json)
        .bind(&metadata_json)
        .execute(&self.pool)
        .await?;

        Ok(conversation)
    }

    /// Gets a conversation by ID.
    pub async fn get_conversation(&self, conversation_id: Uuid) -> Result<Conversation> {
        let maybe_row: Option<PgRow> = sqlx::query(r#"
            SELECT
                conversation_id, tenant_id, user_id, user_role, status,
                started_at, last_activity_at, ended_at, message_count,
                context, escalation, metadata
            FROM chatbot.conversations
            WHERE conversation_id = $1
        "#)
        .bind(conversation_id)
        .fetch_optional(&self.pool)
        .await?;

        let row = maybe_row.ok_or_else(|| {
            RepositoryError::NotFound(format!("Conversation {}", conversation_id))
        })?;

        Ok(row_to_conversation(&row))
    }

    /// Updates conversation last activity.
    pub async fn update_conversation_activity(&self, conversation_id: Uuid) -> Result<()> {
        sqlx::query(r#"
            UPDATE chatbot.conversations
            SET last_activity_at = NOW(), message_count = message_count + 1
            WHERE conversation_id = $1
        "#)
        .bind(conversation_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Updates conversation context.
    pub async fn update_conversation_context(
        &self,
        conversation_id: Uuid,
        context: &ConversationContext,
    ) -> Result<()> {
        let context_json = serde_json::to_value(context)?;

        sqlx::query(r#"
            UPDATE chatbot.conversations
            SET context = $2, last_activity_at = NOW()
            WHERE conversation_id = $1
        "#)
        .bind(conversation_id)
        .bind(&context_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Escalates a conversation.
    pub async fn escalate_conversation(
        &self,
        conversation_id: Uuid,
        reason: EscalationReason,
        notes: Option<String>,
    ) -> Result<Conversation> {
        let escalation = EscalationInfo {
            escalated_at: Utc::now(),
            reason,
            agent_id: None,
            ticket_id: None,
            notes,
        };
        let escalation_json = serde_json::to_value(&escalation)?;

        sqlx::query(r#"
            UPDATE chatbot.conversations
            SET status = 'escalated', escalation = $2, last_activity_at = NOW()
            WHERE conversation_id = $1
        "#)
        .bind(conversation_id)
        .bind(&escalation_json)
        .execute(&self.pool)
        .await?;

        self.get_conversation(conversation_id).await
    }

    /// Ends a conversation.
    pub async fn end_conversation(
        &self,
        conversation_id: Uuid,
        status: ConversationStatus,
    ) -> Result<Conversation> {
        let status_str = match status {
            ConversationStatus::Resolved => "resolved",
            ConversationStatus::Abandoned => "abandoned",
            _ => "resolved",
        };

        sqlx::query(r#"
            UPDATE chatbot.conversations
            SET status = $2, ended_at = NOW()
            WHERE conversation_id = $1
        "#)
        .bind(conversation_id)
        .bind(status_str)
        .execute(&self.pool)
        .await?;

        self.get_conversation(conversation_id).await
    }

    /// Gets user's recent conversations.
    pub async fn get_user_conversations(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Conversation>> {
        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                conversation_id, tenant_id, user_id, user_role, status,
                started_at, last_activity_at, ended_at, message_count,
                context, escalation, metadata
            FROM chatbot.conversations
            WHERE user_id = $1
            ORDER BY last_activity_at DESC
            LIMIT $2
        "#)
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(row_to_conversation).collect())
    }

    // =========================================================================
    // MESSAGE OPERATIONS
    // =========================================================================

    /// Saves a message.
    pub async fn save_message(&self, message: &Message) -> Result<()> {
        let content_json = serde_json::to_value(&message.content)?;
        let intent_json = message.intent.as_ref()
            .map(|i| serde_json::to_value(i))
            .transpose()?;
        let feedback_json = message.feedback.as_ref()
            .map(|f| serde_json::to_value(f))
            .transpose()?;
        let metadata_json = serde_json::to_value(&message.metadata)?;

        let sender_str = match message.sender {
            MessageSender::User => "user",
            MessageSender::Bot => "bot",
            MessageSender::System => "system",
            MessageSender::Agent => "agent",
        };

        sqlx::query(r#"
            INSERT INTO chatbot.messages (
                message_id, conversation_id, sender, content, timestamp,
                intent, confidence, feedback, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#)
        .bind(message.message_id)
        .bind(message.conversation_id)
        .bind(sender_str)
        .bind(&content_json)
        .bind(message.timestamp)
        .bind(&intent_json)
        .bind(message.confidence)
        .bind(&feedback_json)
        .bind(&metadata_json)
        .execute(&self.pool)
        .await?;

        // Update conversation activity
        self.update_conversation_activity(message.conversation_id).await?;

        Ok(())
    }

    /// Gets conversation messages.
    pub async fn get_messages(
        &self,
        conversation_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>> {
        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                message_id, conversation_id, sender, content, timestamp,
                intent, confidence, feedback, metadata
            FROM chatbot.messages
            WHERE conversation_id = $1
            ORDER BY timestamp ASC
            LIMIT $2 OFFSET $3
        "#)
        .bind(conversation_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(row_to_message).collect())
    }

    /// Adds feedback to a message.
    pub async fn add_message_feedback(
        &self,
        message_id: Uuid,
        feedback_type: FeedbackType,
        comment: Option<String>,
    ) -> Result<()> {
        let feedback = MessageFeedback {
            feedback_type,
            submitted_at: Utc::now(),
            comment,
        };
        let feedback_json = serde_json::to_value(&feedback)?;

        sqlx::query(r#"
            UPDATE chatbot.messages
            SET feedback = $2
            WHERE message_id = $1
        "#)
        .bind(message_id)
        .bind(&feedback_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // KNOWLEDGE BASE OPERATIONS
    // =========================================================================

    /// Creates a KB article.
    pub async fn create_article(&self, article: NewKBArticle) -> Result<KBArticle> {
        let article_id = Uuid::new_v4();
        let now = Utc::now();
        let target_roles: Vec<String> = article.target_roles.iter()
            .map(|r| r.to_string())
            .collect();

        sqlx::query(r#"
            INSERT INTO chatbot.kb_articles (
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        "#)
        .bind(article_id)
        .bind(&article.slug)
        .bind(&article.title)
        .bind(&article.content)
        .bind(&article.summary)
        .bind(&article.category)
        .bind(&article.subcategory)
        .bind(&article.tags)
        .bind(&article.keywords)
        .bind(&article.intent_triggers)
        .bind(&target_roles)
        .bind(&article.language)
        .bind("draft")
        .bind(0i64)
        .bind(0i64)
        .bind(0i64)
        .bind(now)
        .bind(now)
        .bind(article.author_id)
        .execute(&self.pool)
        .await?;

        self.get_article(article_id).await
    }

    /// Gets a KB article by ID.
    pub async fn get_article(&self, article_id: Uuid) -> Result<KBArticle> {
        let maybe_row: Option<PgRow> = sqlx::query(r#"
            SELECT
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id
            FROM chatbot.kb_articles
            WHERE article_id = $1
        "#)
        .bind(article_id)
        .fetch_optional(&self.pool)
        .await?;

        let row = maybe_row.ok_or_else(|| {
            RepositoryError::NotFound(format!("Article {}", article_id))
        })?;

        Ok(row_to_article(&row))
    }

    /// Gets a KB article by slug.
    pub async fn get_article_by_slug(&self, slug: &str) -> Result<KBArticle> {
        let maybe_row: Option<PgRow> = sqlx::query(r#"
            SELECT
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id
            FROM chatbot.kb_articles
            WHERE slug = $1 AND status = 'published'
        "#)
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        let row = maybe_row.ok_or_else(|| {
            RepositoryError::NotFound(format!("Article with slug {}", slug))
        })?;

        // Increment view count
        sqlx::query("UPDATE chatbot.kb_articles SET view_count = view_count + 1 WHERE slug = $1")
            .bind(slug)
            .execute(&self.pool)
            .await?;

        Ok(row_to_article(&row))
    }

    /// Searches KB articles.
    pub async fn search_articles(
        &self,
        query: &str,
        role: UserRole,
        language: &str,
        limit: i64,
    ) -> Result<Vec<KBSearchResult>> {
        let role_str = role.to_string();

        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id,
                ts_rank(search_vector, plainto_tsquery('spanish', $1)) +
                ts_rank(search_vector, plainto_tsquery('english', $1)) as relevance
            FROM chatbot.kb_articles
            WHERE status = 'published'
              AND language = $2
              AND ($3 = 'anonymous' OR $3 = ANY(target_roles))
              AND (
                  search_vector @@ plainto_tsquery('spanish', $1) OR
                  search_vector @@ plainto_tsquery('english', $1) OR
                  $1 = ANY(keywords) OR
                  $1 = ANY(intent_triggers)
              )
            ORDER BY relevance DESC
            LIMIT $4
        "#)
        .bind(query)
        .bind(language)
        .bind(&role_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row| {
            let article = row_to_article(row);
            let relevance: f64 = row.get("relevance");

            // Create snippet from content
            let snippet = article.summary.clone()
                .unwrap_or_else(|| {
                    let content = &article.content;
                    if content.len() > 200 {
                        format!("{}...", &content[..200])
                    } else {
                        content.clone()
                    }
                });

            KBSearchResult {
                article,
                relevance_score: relevance,
                matched_keywords: vec![query.to_string()],
                snippet,
            }
        }).collect())
    }

    /// Gets articles by category.
    pub async fn get_articles_by_category(
        &self,
        category: &str,
        role: UserRole,
        limit: i64,
    ) -> Result<Vec<KBArticle>> {
        let role_str = role.to_string();

        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id
            FROM chatbot.kb_articles
            WHERE status = 'published'
              AND category = $1
              AND ($2 = 'anonymous' OR $2 = ANY(target_roles))
            ORDER BY view_count DESC
            LIMIT $3
        "#)
        .bind(category)
        .bind(&role_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(row_to_article).collect())
    }

    /// Gets popular articles.
    pub async fn get_popular_articles(
        &self,
        role: UserRole,
        limit: i64,
    ) -> Result<Vec<KBArticle>> {
        let role_str = role.to_string();

        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                article_id, slug, title, content, summary, category, subcategory,
                tags, keywords, intent_triggers, target_roles, language,
                status, view_count, helpful_count, not_helpful_count,
                created_at, updated_at, author_id
            FROM chatbot.kb_articles
            WHERE status = 'published'
              AND ($1 = 'anonymous' OR $1 = ANY(target_roles))
            ORDER BY view_count DESC, helpful_count DESC
            LIMIT $2
        "#)
        .bind(&role_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(row_to_article).collect())
    }

    /// Records article feedback.
    pub async fn record_article_feedback(
        &self,
        article_id: Uuid,
        helpful: bool,
    ) -> Result<()> {
        let column = if helpful { "helpful_count" } else { "not_helpful_count" };

        sqlx::query(&format!(
            "UPDATE chatbot.kb_articles SET {} = {} + 1 WHERE article_id = $1",
            column, column
        ))
        .bind(article_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // SUGGESTIONS OPERATIONS
    // =========================================================================

    /// Gets contextual suggestions.
    pub async fn get_suggestions(
        &self,
        role: UserRole,
        page: Option<&str>,
    ) -> Result<Vec<ContextualSuggestion>> {
        let role_str = role.to_string();

        let rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                suggestion_id, text, intent, target_roles, context_conditions, priority
            FROM chatbot.suggestions
            WHERE is_active = TRUE
              AND $1 = ANY(target_roles)
              AND ($2 IS NULL OR $2 = ANY(context_conditions->'pages'))
            ORDER BY priority DESC
            LIMIT 5
        "#)
        .bind(&role_str)
        .bind(page)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(row_to_suggestion).collect())
    }

    // =========================================================================
    // ANALYTICS OPERATIONS
    // =========================================================================

    /// Gets chatbot analytics.
    pub async fn get_analytics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<ChatbotAnalytics> {
        // Get conversation stats
        let conv_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) as total_conversations,
                COUNT(DISTINCT user_id) as unique_users,
                AVG(message_count)::float8 as avg_messages,
                COUNT(*) FILTER (WHERE status = 'resolved')::float8 / NULLIF(COUNT(*)::float8, 0) as resolution_rate,
                COUNT(*) FILTER (WHERE status = 'escalated')::float8 / NULLIF(COUNT(*)::float8, 0) as escalation_rate
            FROM chatbot.conversations
            WHERE started_at >= $1 AND started_at <= $2
        "#)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;

        // Get message stats
        let msg_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) as total_messages,
                AVG(
                    CASE WHEN sender = 'bot' THEN
                        EXTRACT(EPOCH FROM (timestamp - LAG(timestamp) OVER (PARTITION BY conversation_id ORDER BY timestamp))) * 1000
                    END
                )::float8 as avg_response_time
            FROM chatbot.messages
            WHERE timestamp >= $1 AND timestamp <= $2
        "#)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;

        // Get satisfaction rate from feedback
        let feedback_row: PgRow = sqlx::query(r#"
            SELECT
                COUNT(*) FILTER (WHERE feedback->>'feedback_type' = 'thumbs_up')::float8 /
                NULLIF(COUNT(*) FILTER (WHERE feedback IS NOT NULL)::float8, 0) as satisfaction_rate
            FROM chatbot.messages
            WHERE timestamp >= $1 AND timestamp <= $2
        "#)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;

        // Get top intents
        let intent_rows: Vec<PgRow> = sqlx::query(r#"
            SELECT
                intent->>'name' as intent_name,
                COUNT(*) as count,
                AVG((intent->>'confidence')::float8) as avg_confidence
            FROM chatbot.messages
            WHERE timestamp >= $1 AND timestamp <= $2
              AND intent IS NOT NULL
            GROUP BY intent->>'name'
            ORDER BY count DESC
            LIMIT 10
        "#)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        let top_intents: Vec<IntentStat> = intent_rows.iter().map(|row| {
            IntentStat {
                intent: row.get("intent_name"),
                count: row.get("count"),
                avg_confidence: row.get("avg_confidence"),
            }
        }).collect();

        Ok(ChatbotAnalytics {
            period_start: start,
            period_end: end,
            total_conversations: conv_row.get("total_conversations"),
            total_messages: msg_row.get("total_messages"),
            unique_users: conv_row.get("unique_users"),
            avg_messages_per_conversation: conv_row.get::<Option<f64>, _>("avg_messages").unwrap_or(0.0),
            avg_response_time_ms: msg_row.get::<Option<f64>, _>("avg_response_time").unwrap_or(0.0),
            resolution_rate: conv_row.get::<Option<f64>, _>("resolution_rate").unwrap_or(0.0),
            escalation_rate: conv_row.get::<Option<f64>, _>("escalation_rate").unwrap_or(0.0),
            satisfaction_rate: feedback_row.get::<Option<f64>, _>("satisfaction_rate").unwrap_or(0.0),
            top_intents,
            unanswered_queries: vec![], // Would need separate tracking
        })
    }
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

fn row_to_conversation(row: &PgRow) -> Conversation {
    let context: serde_json::Value = row.get("context");
    let escalation: Option<serde_json::Value> = row.get("escalation");
    let metadata: serde_json::Value = row.get("metadata");
    let status_str: String = row.get("status");
    let role_str: String = row.get("user_role");

    Conversation {
        conversation_id: row.get("conversation_id"),
        tenant_id: row.get("tenant_id"),
        user_id: row.get("user_id"),
        user_role: str_to_role(&role_str),
        status: str_to_status(&status_str),
        started_at: row.get("started_at"),
        last_activity_at: row.get("last_activity_at"),
        ended_at: row.get("ended_at"),
        message_count: row.get("message_count"),
        context: serde_json::from_value(context).unwrap_or_default(),
        escalation: escalation.and_then(|e| serde_json::from_value(e).ok()),
        metadata: serde_json::from_value(metadata).unwrap_or_default(),
    }
}

fn row_to_message(row: &PgRow) -> Message {
    let content: serde_json::Value = row.get("content");
    let intent: Option<serde_json::Value> = row.get("intent");
    let feedback: Option<serde_json::Value> = row.get("feedback");
    let metadata: serde_json::Value = row.get("metadata");
    let sender_str: String = row.get("sender");

    Message {
        message_id: row.get("message_id"),
        conversation_id: row.get("conversation_id"),
        sender: str_to_sender(&sender_str),
        content: serde_json::from_value(content).unwrap_or(MessageContent {
            content_type: ContentType::Text,
            text: String::new(),
            rich_content: None,
        }),
        timestamp: row.get("timestamp"),
        intent: intent.and_then(|i| serde_json::from_value(i).ok()),
        confidence: row.get("confidence"),
        feedback: feedback.and_then(|f| serde_json::from_value(f).ok()),
        metadata: serde_json::from_value(metadata).unwrap_or_default(),
    }
}

fn row_to_article(row: &PgRow) -> KBArticle {
    let target_roles: Vec<String> = row.get("target_roles");
    let status_str: String = row.get("status");

    KBArticle {
        article_id: row.get("article_id"),
        slug: row.get("slug"),
        title: row.get("title"),
        content: row.get("content"),
        summary: row.get("summary"),
        category: row.get("category"),
        subcategory: row.get("subcategory"),
        tags: row.get("tags"),
        keywords: row.get("keywords"),
        intent_triggers: row.get("intent_triggers"),
        target_roles: target_roles.iter().map(|r| str_to_role(r)).collect(),
        language: row.get("language"),
        status: str_to_article_status(&status_str),
        view_count: row.get("view_count"),
        helpful_count: row.get("helpful_count"),
        not_helpful_count: row.get("not_helpful_count"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        author_id: row.get("author_id"),
    }
}

fn row_to_suggestion(row: &PgRow) -> ContextualSuggestion {
    let target_roles: Vec<String> = row.get("target_roles");
    let conditions: serde_json::Value = row.get("context_conditions");

    ContextualSuggestion {
        suggestion_id: row.get("suggestion_id"),
        text: row.get("text"),
        intent: row.get("intent"),
        target_roles: target_roles.iter().map(|r| str_to_role(r)).collect(),
        context_conditions: serde_json::from_value(conditions).unwrap_or_default(),
        priority: row.get("priority"),
        is_active: true,
    }
}

fn str_to_role(s: &str) -> UserRole {
    match s {
        "student" => UserRole::Student,
        "instructor" => UserRole::Instructor,
        "admin" => UserRole::Admin,
        _ => UserRole::Anonymous,
    }
}

fn str_to_status(s: &str) -> ConversationStatus {
    match s {
        "active" => ConversationStatus::Active,
        "escalated" => ConversationStatus::Escalated,
        "resolved" => ConversationStatus::Resolved,
        "abandoned" => ConversationStatus::Abandoned,
        _ => ConversationStatus::Active,
    }
}

fn str_to_sender(s: &str) -> MessageSender {
    match s {
        "user" => MessageSender::User,
        "bot" => MessageSender::Bot,
        "system" => MessageSender::System,
        "agent" => MessageSender::Agent,
        _ => MessageSender::User,
    }
}

fn str_to_article_status(s: &str) -> ArticleStatus {
    match s {
        "draft" => ArticleStatus::Draft,
        "published" => ArticleStatus::Published,
        "archived" => ArticleStatus::Archived,
        _ => ArticleStatus::Draft,
    }
}
