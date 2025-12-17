//! # API Data Transfer Objects
//!
//! Request and response DTOs for AI Service endpoints.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    ContentGenerationType, ContentReference, DifficultyLevel,
    EmbeddingContentType, GenerationStatus, MessageRole,
    QuestionOption, QuestionType, TutorSessionStatus,
};

// =============================================================================
// COMMON RESPONSES
// =============================================================================

/// Standard API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorResponse {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }
}

/// Error response structure.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

/// Paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub has_more: bool,
}

// =============================================================================
// TUTOR SESSION DTOs
// =============================================================================

/// Request to create a new tutor session.
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTutorSessionRequest {
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
}

/// Response for tutor session.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorSessionResponse {
    pub session_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub status: TutorSessionStatus,
    pub message_count: i32,
    pub created_at: DateTime<Utc>,
    pub last_message_at: Option<DateTime<Utc>>,
}

/// Request to send a message to tutor.
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendTutorMessageRequest {
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    pub lesson_id: Option<Uuid>,
}

/// Response for tutor message.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorMessageResponse {
    pub message_id: Uuid,
    pub session_id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub references: Option<Vec<ContentReferenceResponse>>,
    pub created_at: DateTime<Utc>,
}

/// Content reference in tutor response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentReferenceResponse {
    pub lesson_id: Uuid,
    pub lesson_title: String,
    pub timestamp_seconds: Option<i32>,
    pub snippet: String,
    pub relevance_score: f32,
}

impl From<ContentReference> for ContentReferenceResponse {
    fn from(r: ContentReference) -> Self {
        Self {
            lesson_id: r.lesson_id,
            lesson_title: r.lesson_title,
            timestamp_seconds: r.timestamp_seconds,
            snippet: r.snippet,
            relevance_score: r.relevance_score,
        }
    }
}

/// Streaming message chunk for SSE.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamChunk {
    #[serde(rename = "type")]
    pub chunk_type: StreamChunkType,
    pub content: Option<String>,
    pub references: Option<Vec<ContentReferenceResponse>>,
    pub done: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamChunkType {
    Text,
    Reference,
    Done,
    Error,
}

// =============================================================================
// SEMANTIC SEARCH DTOs
// =============================================================================

/// Request for semantic search.
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchRequest {
    #[validate(length(min = 3, max = 500))]
    pub query: String,
    pub course_ids: Option<Vec<Uuid>>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<i32>,
    #[validate(range(min = 0.0, max = 1.0))]
    pub min_score: Option<f32>,
}

/// Response for semantic search result.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticSearchResultResponse {
    pub embedding_id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub lesson_id: Option<Uuid>,
    pub lesson_title: Option<String>,
    pub content_type: EmbeddingContentType,
    pub snippet: String,
    pub similarity_score: f32,
    pub has_access: bool,
}

/// Request to index course content.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexCourseRequest {
    pub force_reindex: Option<bool>,
}

/// Response for indexing operation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingResponse {
    pub course_id: Uuid,
    pub chunks_indexed: i32,
    pub status: String,
}

// =============================================================================
// CONTENT GENERATION DTOs
// =============================================================================

/// Request to generate content (summary, key points, glossary).
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    #[validate(length(min = 2, max = 5))]
    pub language: Option<String>,
}

/// Response for content generation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentGenerationResponse {
    pub request_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub generation_type: ContentGenerationType,
    pub status: GenerationStatus,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Response for summary generation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryResponse {
    pub summary_id: Uuid,
    pub lesson_id: Uuid,
    pub summary: String,
    pub language: String,
    pub generated_at: DateTime<Utc>,
}

/// Response for key points generation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPointsResponse {
    pub lesson_id: Uuid,
    pub key_points: Vec<KeyPointResponse>,
    pub language: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPointResponse {
    pub order_index: i32,
    pub title: String,
    pub description: String,
    pub timestamp_seconds: Option<i32>,
}

/// Response for glossary generation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryResponse {
    pub lesson_id: Uuid,
    pub terms: Vec<GlossaryTermResponse>,
    pub language: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryTermResponse {
    pub term: String,
    pub definition: String,
    pub related_terms: Vec<String>,
}

// =============================================================================
// QUIZ GENERATION DTOs
// =============================================================================

/// Request to generate quiz.
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GenerateQuizRequest {
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    #[validate(range(min = 1, max = 20))]
    pub question_count: Option<i32>,
    pub difficulty: Option<DifficultyLevel>,
    pub question_types: Option<Vec<QuestionType>>,
    #[validate(length(min = 2, max = 5))]
    pub language: Option<String>,
    pub include_explanations: Option<bool>,
}

/// Response for quiz generation request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizGenerationResponse {
    pub request_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub status: GenerationStatus,
    pub config: QuizConfigResponse,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizConfigResponse {
    pub question_count: i32,
    pub difficulty: DifficultyLevel,
    pub question_types: Vec<QuestionType>,
    pub language: String,
    pub include_explanations: bool,
}

/// Response for generated questions.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedQuestionsResponse {
    pub request_id: Uuid,
    pub questions: Vec<GeneratedQuestionResponse>,
    pub total: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedQuestionResponse {
    pub question_id: Uuid,
    pub question_type: QuestionType,
    pub difficulty: DifficultyLevel,
    pub question_text: String,
    pub options: Option<Vec<QuestionOptionResponse>>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub points: i32,
    pub source_reference: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOptionResponse {
    pub option_id: String,
    pub text: String,
    pub is_correct: bool,
}

impl From<QuestionOption> for QuestionOptionResponse {
    fn from(opt: QuestionOption) -> Self {
        Self {
            option_id: opt.option_id,
            text: opt.text,
            is_correct: opt.is_correct,
        }
    }
}

// =============================================================================
// USAGE/QUOTA DTOs
// =============================================================================

/// Response for usage quota.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageQuotaResponse {
    pub tutor_chat: FeatureQuotaResponse,
    pub semantic_search: FeatureQuotaResponse,
    pub summary_generation: FeatureQuotaResponse,
    pub quiz_generation: FeatureQuotaResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureQuotaResponse {
    pub daily_limit: i32,
    pub daily_used: i32,
    pub daily_remaining: i32,
    pub monthly_limit: i32,
    pub monthly_used: i32,
    pub monthly_remaining: i32,
    pub reset_daily_at: DateTime<Utc>,
    pub reset_monthly_at: DateTime<Utc>,
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}
