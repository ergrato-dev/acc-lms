//! # AI Service Domain Errors
//!
//! Error types for the AI service domain.

use uuid::Uuid;

/// Errors that can occur in AI operations.
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    // LLM Errors
    #[error("LLM API error: {0}")]
    LLMError(String),

    #[error("LLM rate limit exceeded")]
    LLMRateLimited,

    #[error("LLM context too large: {tokens} tokens exceeds limit of {limit}")]
    ContextTooLarge { tokens: i32, limit: i32 },

    // Session Errors
    #[error("Tutor session not found: {0}")]
    SessionNotFound(Uuid),

    #[error("Session expired: {0}")]
    SessionExpired(Uuid),

    #[error("Session belongs to different user")]
    SessionUnauthorized,

    // Content Errors
    #[error("Course not found: {0}")]
    CourseNotFound(Uuid),

    #[error("Lesson not found: {0}")]
    LessonNotFound(Uuid),

    #[error("Content not indexed for semantic search")]
    ContentNotIndexed,

    #[error("No embeddings found for course: {0}")]
    NoEmbeddings(Uuid),

    // Generation Errors
    #[error("Generation request not found: {0}")]
    GenerationNotFound(Uuid),

    #[error("Content generation failed: {0}")]
    GenerationFailed(String),

    #[error("Quiz generation failed: {0}")]
    QuizGenerationFailed(String),

    // Usage/Quota Errors
    #[error("Daily usage limit exceeded for {feature}")]
    DailyLimitExceeded { feature: String },

    #[error("Monthly usage limit exceeded for {feature}")]
    MonthlyLimitExceeded { feature: String },

    // Access Errors
    #[error("User not enrolled in course: {0}")]
    NotEnrolled(Uuid),

    #[error("Feature not available for user's plan")]
    FeatureNotAvailable,

    // Infrastructure Errors
    #[error("Database error: {0}")]
    Database(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Embedding service error: {0}")]
    EmbeddingService(String),

    // Validation Errors
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Empty query")]
    EmptyQuery,
}

impl AIError {
    /// Returns the HTTP status code for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            Self::SessionNotFound(_)
            | Self::CourseNotFound(_)
            | Self::LessonNotFound(_)
            | Self::GenerationNotFound(_)
            | Self::NoEmbeddings(_) => 404,

            Self::SessionUnauthorized
            | Self::NotEnrolled(_)
            | Self::FeatureNotAvailable => 403,

            Self::SessionExpired(_) => 410,

            Self::LLMRateLimited
            | Self::DailyLimitExceeded { .. }
            | Self::MonthlyLimitExceeded { .. } => 429,

            Self::InvalidRequest(_)
            | Self::EmptyQuery
            | Self::ContextTooLarge { .. } => 400,

            Self::LLMError(_)
            | Self::GenerationFailed(_)
            | Self::QuizGenerationFailed(_)
            | Self::Database(_)
            | Self::Cache(_)
            | Self::EmbeddingService(_)
            | Self::ContentNotIndexed => 500,
        }
    }

    /// Returns the error code for API responses.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::LLMError(_) => "LLM_ERROR",
            Self::LLMRateLimited => "LLM_RATE_LIMITED",
            Self::ContextTooLarge { .. } => "CONTEXT_TOO_LARGE",
            Self::SessionNotFound(_) => "SESSION_NOT_FOUND",
            Self::SessionExpired(_) => "SESSION_EXPIRED",
            Self::SessionUnauthorized => "SESSION_UNAUTHORIZED",
            Self::CourseNotFound(_) => "COURSE_NOT_FOUND",
            Self::LessonNotFound(_) => "LESSON_NOT_FOUND",
            Self::ContentNotIndexed => "CONTENT_NOT_INDEXED",
            Self::NoEmbeddings(_) => "NO_EMBEDDINGS",
            Self::GenerationNotFound(_) => "GENERATION_NOT_FOUND",
            Self::GenerationFailed(_) => "GENERATION_FAILED",
            Self::QuizGenerationFailed(_) => "QUIZ_GENERATION_FAILED",
            Self::DailyLimitExceeded { .. } => "DAILY_LIMIT_EXCEEDED",
            Self::MonthlyLimitExceeded { .. } => "MONTHLY_LIMIT_EXCEEDED",
            Self::NotEnrolled(_) => "NOT_ENROLLED",
            Self::FeatureNotAvailable => "FEATURE_NOT_AVAILABLE",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Cache(_) => "CACHE_ERROR",
            Self::EmbeddingService(_) => "EMBEDDING_ERROR",
            Self::InvalidRequest(_) => "INVALID_REQUEST",
            Self::EmptyQuery => "EMPTY_QUERY",
        }
    }
}

/// Result type alias for AI operations.
pub type AIResult<T> = Result<T, AIError>;
