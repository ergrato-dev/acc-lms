//! # AI Service Domain Entities
//!
//! Core entities for the AI-powered educational system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Status of a tutor conversation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TutorSessionStatus {
    #[default]
    Active,
    Paused,
    Completed,
    Expired,
}

impl std::fmt::Display for TutorSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Completed => write!(f, "completed"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

/// Role of message sender in tutor conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
            Self::System => write!(f, "system"),
        }
    }
}

/// Type of generated content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentGenerationType {
    Summary,
    KeyPoints,
    Glossary,
    Objectives,
    Transcription,
}

/// Status of content generation task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GenerationStatus {
    #[default]
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Quiz question type for generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Code,
}

impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleChoice => write!(f, "single_choice"),
            Self::MultipleChoice => write!(f, "multiple_choice"),
            Self::TrueFalse => write!(f, "true_false"),
            Self::ShortAnswer => write!(f, "short_answer"),
            Self::Code => write!(f, "code"),
        }
    }
}

/// Difficulty level for quiz generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyLevel {
    Easy,
    #[default]
    Medium,
    Hard,
}

// =============================================================================
// TUTOR CONVERSATION ENTITIES
// =============================================================================

/// A tutor conversation session contextual to a course.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub status: TutorSessionStatus,
    pub message_count: i32,
    pub context_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_message_at: Option<DateTime<Utc>>,
}

/// A message in a tutor conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorMessage {
    pub message_id: Uuid,
    pub session_id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub tokens_used: Option<i32>,
    pub references: Option<Vec<ContentReference>>,
    pub created_at: DateTime<Utc>,
}

/// Reference to course content in a tutor response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentReference {
    pub lesson_id: Uuid,
    pub lesson_title: String,
    pub timestamp_seconds: Option<i32>,
    pub snippet: String,
    pub relevance_score: f32,
}

// =============================================================================
// SEMANTIC SEARCH ENTITIES
// =============================================================================

/// A content embedding for semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEmbedding {
    pub embedding_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub content_type: EmbeddingContentType,
    pub chunk_index: i32,
    pub content_text: String,
    pub embedding: Vec<f32>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Type of content that was embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingContentType {
    LessonText,
    Transcription,
    Description,
    Resource,
}

/// Result from semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResult {
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

// =============================================================================
// CONTENT GENERATION ENTITIES
// =============================================================================

/// Generated content summary for a lesson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSummary {
    pub summary_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub generation_type: ContentGenerationType,
    pub content: String,
    pub language: String,
    pub status: GenerationStatus,
    pub model_used: Option<String>,
    pub tokens_used: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A key point extracted from content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPoint {
    pub order_index: i32,
    pub title: String,
    pub description: String,
    pub timestamp_seconds: Option<i32>,
}

/// A glossary term extracted from content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryTerm {
    pub term: String,
    pub definition: String,
    pub related_terms: Vec<String>,
}

// =============================================================================
// QUIZ GENERATION ENTITIES
// =============================================================================

/// Request to generate a quiz from content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizGenerationRequest {
    pub request_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub instructor_id: Uuid,
    pub config: QuizGenerationConfig,
    pub status: GenerationStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Configuration for quiz generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizGenerationConfig {
    pub question_count: i32,
    pub difficulty: DifficultyLevel,
    pub question_types: Vec<QuestionType>,
    pub language: String,
    pub include_explanations: bool,
}

impl Default for QuizGenerationConfig {
    fn default() -> Self {
        Self {
            question_count: 5,
            difficulty: DifficultyLevel::Medium,
            question_types: vec![QuestionType::SingleChoice, QuestionType::MultipleChoice],
            language: "es".to_string(),
            include_explanations: true,
        }
    }
}

/// A generated quiz question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedQuestion {
    pub question_id: Uuid,
    pub request_id: Uuid,
    pub question_type: QuestionType,
    pub difficulty: DifficultyLevel,
    pub question_text: String,
    pub options: Option<Vec<QuestionOption>>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub points: i32,
    pub source_reference: Option<String>,
}

/// An option for a multiple choice question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub option_id: String,
    pub text: String,
    pub is_correct: bool,
}

// =============================================================================
// USAGE TRACKING
// =============================================================================

/// AI feature usage for rate limiting and billing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIUsage {
    pub usage_id: Uuid,
    pub user_id: Uuid,
    pub feature: AIFeature,
    pub tokens_input: i32,
    pub tokens_output: i32,
    pub cost_cents: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// AI features for usage tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AIFeature {
    TutorChat,
    SemanticSearch,
    SummaryGeneration,
    QuizGeneration,
    TranscriptionGeneration,
}

/// User's AI usage quota.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageQuota {
    pub user_id: Uuid,
    pub feature: AIFeature,
    pub daily_limit: i32,
    pub monthly_limit: i32,
    pub used_today: i32,
    pub used_this_month: i32,
    pub reset_daily_at: DateTime<Utc>,
    pub reset_monthly_at: DateTime<Utc>,
}

// =============================================================================
// DTO TYPES FOR CREATION
// =============================================================================

/// DTO for creating a new tutor session.
#[derive(Debug, Clone)]
pub struct CreateTutorSession {
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
}

/// DTO for sending a tutor message.
#[derive(Debug, Clone)]
pub struct SendTutorMessage {
    pub session_id: Uuid,
    pub content: String,
    pub lesson_id: Option<Uuid>,
}

/// DTO for semantic search request.
#[derive(Debug, Clone)]
pub struct SemanticSearchQuery {
    pub query: String,
    pub user_id: Option<Uuid>,
    pub course_ids: Option<Vec<Uuid>>,
    pub limit: i32,
    pub min_score: f32,
}

/// DTO for requesting content generation.
#[derive(Debug, Clone)]
pub struct RequestContentGeneration {
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub generation_type: ContentGenerationType,
    pub language: String,
}

/// DTO for requesting quiz generation.
#[derive(Debug, Clone)]
pub struct RequestQuizGeneration {
    pub course_id: Uuid,
    pub lesson_id: Uuid,
    pub instructor_id: Uuid,
    pub config: QuizGenerationConfig,
}
