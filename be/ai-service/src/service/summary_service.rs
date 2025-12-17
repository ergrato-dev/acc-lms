//! # Summary Service
//!
//! Business logic for generating summaries, key points, and glossaries.

use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, ContentGenerationType, ContentSummary,
    GenerationStatus, GlossaryTerm, KeyPoint,
};
use crate::llm::LLMClient;
use crate::repository::SummaryRepository;

/// Service for content summary generation.
pub struct SummaryService {
    summary_repo: SummaryRepository,
    llm_client: Arc<dyn LLMClient>,
}

impl SummaryService {
    pub fn new(
        summary_repo: SummaryRepository,
        llm_client: Arc<dyn LLMClient>,
    ) -> Self {
        Self {
            summary_repo,
            llm_client,
        }
    }

    /// Generates a summary for a lesson.
    pub async fn generate_summary(
        &self,
        course_id: Uuid,
        lesson_id: Uuid,
        language: &str,
    ) -> AIResult<ContentSummary> {
        // Check for existing summary
        if let Some(existing) = self.summary_repo
            .get_lesson_summary(lesson_id, ContentGenerationType::Summary, language)
            .await?
        {
            return Ok(existing);
        }

        // Fetch lesson content
        let content = self.fetch_lesson_content(lesson_id).await?;

        // Create pending summary record
        let summary_id = Uuid::now_v7();
        let now = Utc::now();
        let summary = ContentSummary {
            summary_id,
            course_id,
            lesson_id,
            generation_type: ContentGenerationType::Summary,
            content: String::new(),
            language: language.to_string(),
            status: GenerationStatus::Processing,
            model_used: Some("gpt-4".to_string()),
            tokens_used: None,
            created_at: now,
            updated_at: now,
        };

        self.summary_repo.create_summary(&summary).await?;

        // Generate summary using LLM
        let (summary_text, tokens_used) = self.llm_client
            .generate_summary(&content, language)
            .await
            .map_err(|e| AIError::GenerationFailed(e))?;

        // Update summary with generated content
        self.summary_repo
            .update_summary(summary_id, &summary_text, GenerationStatus::Completed, Some(tokens_used))
            .await?;

        // Return completed summary
        Ok(ContentSummary {
            summary_id,
            course_id,
            lesson_id,
            generation_type: ContentGenerationType::Summary,
            content: summary_text,
            language: language.to_string(),
            status: GenerationStatus::Completed,
            model_used: Some("gpt-4".to_string()),
            tokens_used: Some(tokens_used),
            created_at: now,
            updated_at: Utc::now(),
        })
    }

    /// Generates key points for a lesson.
    pub async fn generate_key_points(
        &self,
        course_id: Uuid,
        lesson_id: Uuid,
        language: &str,
    ) -> AIResult<Vec<KeyPoint>> {
        // Check for existing key points
        if let Some(existing) = self.summary_repo
            .get_lesson_summary(lesson_id, ContentGenerationType::KeyPoints, language)
            .await?
        {
            // Parse stored JSON
            if let Ok(key_points) = serde_json::from_str::<Vec<KeyPoint>>(&existing.content) {
                return Ok(key_points);
            }
        }

        // Fetch lesson content
        let content = self.fetch_lesson_content(lesson_id).await?;

        // Generate key points using LLM
        let (key_points, tokens_used) = self.llm_client
            .generate_key_points(&content, language)
            .await
            .map_err(|e| AIError::GenerationFailed(e))?;

        // Store key points
        let summary_id = Uuid::now_v7();
        let now = Utc::now();
        let key_points_json = serde_json::to_string(&key_points)
            .map_err(|e| AIError::GenerationFailed(e.to_string()))?;

        let summary = ContentSummary {
            summary_id,
            course_id,
            lesson_id,
            generation_type: ContentGenerationType::KeyPoints,
            content: key_points_json,
            language: language.to_string(),
            status: GenerationStatus::Completed,
            model_used: Some("gpt-4".to_string()),
            tokens_used: Some(tokens_used),
            created_at: now,
            updated_at: now,
        };

        self.summary_repo.create_summary(&summary).await?;

        Ok(key_points)
    }

    /// Generates a glossary for a lesson.
    pub async fn generate_glossary(
        &self,
        course_id: Uuid,
        lesson_id: Uuid,
        language: &str,
    ) -> AIResult<Vec<GlossaryTerm>> {
        // Check for existing glossary
        if let Some(existing) = self.summary_repo
            .get_lesson_summary(lesson_id, ContentGenerationType::Glossary, language)
            .await?
        {
            // Parse stored JSON
            if let Ok(terms) = serde_json::from_str::<Vec<GlossaryTerm>>(&existing.content) {
                return Ok(terms);
            }
        }

        // Fetch lesson content
        let content = self.fetch_lesson_content(lesson_id).await?;

        // Generate glossary using LLM
        let (terms, tokens_used) = self.llm_client
            .generate_glossary(&content, language)
            .await
            .map_err(|e| AIError::GenerationFailed(e))?;

        // Store glossary
        let summary_id = Uuid::now_v7();
        let now = Utc::now();
        let terms_json = serde_json::to_string(&terms)
            .map_err(|e| AIError::GenerationFailed(e.to_string()))?;

        let summary = ContentSummary {
            summary_id,
            course_id,
            lesson_id,
            generation_type: ContentGenerationType::Glossary,
            content: terms_json,
            language: language.to_string(),
            status: GenerationStatus::Completed,
            model_used: Some("gpt-4".to_string()),
            tokens_used: Some(tokens_used),
            created_at: now,
            updated_at: now,
        };

        self.summary_repo.create_summary(&summary).await?;

        Ok(terms)
    }

    /// Gets generation status.
    pub async fn get_generation_status(&self, summary_id: Uuid) -> AIResult<ContentSummary> {
        self.summary_repo.get_summary(summary_id).await
    }

    /// Fetches lesson content.
    async fn fetch_lesson_content(&self, _lesson_id: Uuid) -> AIResult<String> {
        // TODO: Call courses-service to fetch lesson content
        // For now, return placeholder
        Ok("Lesson content would be fetched from courses-service".to_string())
    }
}
