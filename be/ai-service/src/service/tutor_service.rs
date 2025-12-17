//! # Tutor Service
//!
//! Business logic for AI tutor conversations.

use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::api::UsageQuotaResponse;
use crate::domain::{
    AIError, AIResult, ContentReference, CreateTutorSession, MessageRole,
    TutorMessage, TutorSession,
};
use crate::llm::LLMClient;
use crate::repository::{ConversationRepository, EmbeddingRepository};

/// Service for AI tutor operations.
pub struct TutorService {
    conversation_repo: ConversationRepository,
    embedding_repo: EmbeddingRepository,
    llm_client: Arc<dyn LLMClient>,
}

impl TutorService {
    pub fn new(
        conversation_repo: ConversationRepository,
        embedding_repo: EmbeddingRepository,
        llm_client: Arc<dyn LLMClient>,
    ) -> Self {
        Self {
            conversation_repo,
            embedding_repo,
            llm_client,
        }
    }

    /// Creates a new tutor session.
    pub async fn create_session(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        lesson_id: Option<Uuid>,
    ) -> AIResult<TutorSession> {
        // Check if there's an existing active session
        if let Some(session) = self.conversation_repo.get_active_session(user_id, course_id).await? {
            return Ok(session);
        }

        // Create new session
        let request = CreateTutorSession {
            user_id,
            course_id,
            lesson_id,
        };

        self.conversation_repo.create_session(request).await
    }

    /// Gets a session by ID, verifying ownership.
    pub async fn get_session(&self, session_id: Uuid, user_id: Uuid) -> AIResult<TutorSession> {
        let session = self.conversation_repo.get_session(session_id).await?;

        if session.user_id != user_id {
            return Err(AIError::SessionUnauthorized);
        }

        Ok(session)
    }

    /// Sends a message and gets a response.
    pub async fn send_message(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        content: &str,
        lesson_id: Option<Uuid>,
    ) -> AIResult<(TutorMessage, TutorMessage)> {
        // Verify session ownership
        let session = self.get_session(session_id, user_id).await?;

        // Create user message
        let user_message = TutorMessage {
            message_id: Uuid::now_v7(),
            session_id,
            role: MessageRole::User,
            content: content.to_string(),
            tokens_used: None,
            references: None,
            created_at: Utc::now(),
        };

        // Save user message
        self.conversation_repo.save_message(&user_message).await?;

        // Get conversation history for context
        let history = self.conversation_repo.get_recent_messages(session_id, 10).await?;

        // Search for relevant content
        let references = self.find_relevant_content(
            content,
            session.course_id,
            lesson_id,
        ).await?;

        // Build context from references
        let context = self.build_context(&references);

        // Generate response using LLM
        let (response_text, tokens_used) = self.llm_client
            .generate_tutor_response(content, &history, &context, session.course_id)
            .await
            .map_err(|e| AIError::LLMError(e))?;

        // Create assistant message
        let assistant_message = TutorMessage {
            message_id: Uuid::now_v7(),
            session_id,
            role: MessageRole::Assistant,
            content: response_text,
            tokens_used: Some(tokens_used),
            references: if references.is_empty() { None } else { Some(references) },
            created_at: Utc::now(),
        };

        // Save assistant message
        self.conversation_repo.save_message(&assistant_message).await?;

        // Update session activity
        self.conversation_repo.update_session_activity(session_id).await?;

        Ok((user_message, assistant_message))
    }

    /// Gets messages for a session.
    pub async fn get_messages(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AIResult<Vec<TutorMessage>> {
        // Verify ownership
        let _ = self.get_session(session_id, user_id).await?;

        self.conversation_repo.get_messages(session_id, limit, offset).await
    }

    /// Gets user's AI usage quota.
    pub async fn get_usage_quota(&self, _user_id: Uuid) -> AIResult<UsageQuotaResponse> {
        // TODO: Implement actual quota tracking
        let now = Utc::now();
        let tomorrow = now + chrono::Duration::days(1);
        let next_month = now + chrono::Duration::days(30);

        Ok(UsageQuotaResponse {
            tutor_chat: crate::api::FeatureQuotaResponse {
                daily_limit: 50,
                daily_used: 0,
                daily_remaining: 50,
                monthly_limit: 500,
                monthly_used: 0,
                monthly_remaining: 500,
                reset_daily_at: tomorrow,
                reset_monthly_at: next_month,
            },
            semantic_search: crate::api::FeatureQuotaResponse {
                daily_limit: 100,
                daily_used: 0,
                daily_remaining: 100,
                monthly_limit: 1000,
                monthly_used: 0,
                monthly_remaining: 1000,
                reset_daily_at: tomorrow,
                reset_monthly_at: next_month,
            },
            summary_generation: crate::api::FeatureQuotaResponse {
                daily_limit: 20,
                daily_used: 0,
                daily_remaining: 20,
                monthly_limit: 200,
                monthly_used: 0,
                monthly_remaining: 200,
                reset_daily_at: tomorrow,
                reset_monthly_at: next_month,
            },
            quiz_generation: crate::api::FeatureQuotaResponse {
                daily_limit: 10,
                daily_used: 0,
                daily_remaining: 10,
                monthly_limit: 100,
                monthly_used: 0,
                monthly_remaining: 100,
                reset_daily_at: tomorrow,
                reset_monthly_at: next_month,
            },
        })
    }

    /// Finds relevant content for the query.
    async fn find_relevant_content(
        &self,
        query: &str,
        course_id: Uuid,
        _lesson_id: Option<Uuid>,
    ) -> AIResult<Vec<ContentReference>> {
        // Generate embedding for the query
        let query_embedding = self.llm_client
            .generate_embedding(query)
            .await
            .map_err(|e| AIError::EmbeddingService(e))?;

        // Search for similar content
        let results = self.embedding_repo
            .search_similar(&query_embedding, Some(&[course_id]), 3, 0.7)
            .await?;

        // Convert to content references
        let references: Vec<ContentReference> = results
            .into_iter()
            .map(|r| ContentReference {
                lesson_id: r.lesson_id.unwrap_or(course_id),
                lesson_title: r.lesson_title.unwrap_or_else(|| "Course Overview".to_string()),
                timestamp_seconds: None,
                snippet: r.snippet,
                relevance_score: r.similarity_score,
            })
            .collect();

        Ok(references)
    }

    /// Builds context string from references.
    fn build_context(&self, references: &[ContentReference]) -> String {
        if references.is_empty() {
            return String::new();
        }

        let mut context = String::from("Relevant course content:\n\n");
        for (i, ref_) in references.iter().enumerate() {
            context.push_str(&format!(
                "{}. From \"{}\": {}\n\n",
                i + 1,
                ref_.lesson_title,
                ref_.snippet
            ));
        }
        context
    }
}
