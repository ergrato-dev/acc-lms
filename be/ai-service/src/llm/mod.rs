//! LLM Module
//!
//! LLM client implementations for AI operations

pub mod openai_client;

pub use openai_client::OpenAIClient;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    GeneratedQuestion, GlossaryTerm, KeyPoint, QuizGenerationConfig, TutorMessage,
};

/// Trait for LLM operations.
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Generates an embedding vector for text.
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;

    /// Generates a tutor response.
    async fn generate_tutor_response(
        &self,
        query: &str,
        history: &[TutorMessage],
        context: &str,
        course_id: Uuid,
    ) -> Result<(String, i32), String>;

    /// Generates a summary.
    async fn generate_summary(&self, content: &str, language: &str) -> Result<(String, i32), String>;

    /// Generates key points.
    async fn generate_key_points(&self, content: &str, language: &str) -> Result<(Vec<KeyPoint>, i32), String>;

    /// Generates glossary terms.
    async fn generate_glossary(&self, content: &str, language: &str) -> Result<(Vec<GlossaryTerm>, i32), String>;

    /// Generates quiz questions.
    async fn generate_quiz(&self, content: &str, config: &QuizGenerationConfig) -> Result<Vec<GeneratedQuestion>, String>;
}
