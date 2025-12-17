//! # Quiz Generator Service
//!
//! Business logic for AI-powered quiz generation.

use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, DifficultyLevel, GeneratedQuestion, GenerationStatus,
    QuestionType, QuizGenerationConfig, QuizGenerationRequest,
};
use crate::llm::LLMClient;
use crate::repository::QuizGenerationRepository;

/// Service for quiz generation.
pub struct QuizGeneratorService {
    quiz_repo: QuizGenerationRepository,
    llm_client: Arc<dyn LLMClient>,
}

impl QuizGeneratorService {
    pub fn new(
        quiz_repo: QuizGenerationRepository,
        llm_client: Arc<dyn LLMClient>,
    ) -> Self {
        Self {
            quiz_repo,
            llm_client,
        }
    }

    /// Generates a quiz from lesson content.
    pub async fn generate_quiz(
        &self,
        course_id: Uuid,
        lesson_id: Uuid,
        instructor_id: Uuid,
        question_count: i32,
        difficulty: Option<DifficultyLevel>,
        question_types: Option<Vec<QuestionType>>,
        language: Option<String>,
        include_explanations: bool,
    ) -> AIResult<QuizGenerationRequest> {
        // Build config
        let config = QuizGenerationConfig {
            question_count,
            difficulty: difficulty.unwrap_or(DifficultyLevel::Medium),
            question_types: question_types.unwrap_or_else(|| vec![
                QuestionType::SingleChoice,
                QuestionType::MultipleChoice,
            ]),
            language: language.unwrap_or_else(|| "es".to_string()),
            include_explanations,
        };

        // Create request record
        let request_id = Uuid::now_v7();
        let now = Utc::now();
        let request = QuizGenerationRequest {
            request_id,
            course_id,
            lesson_id,
            instructor_id,
            config: config.clone(),
            status: GenerationStatus::Processing,
            created_at: now,
            completed_at: None,
        };

        self.quiz_repo.create_request(&request).await?;

        // Fetch lesson content
        let content = self.fetch_lesson_content(lesson_id).await?;

        // Generate quiz using LLM
        match self.llm_client
            .generate_quiz(&content, &config)
            .await
        {
            Ok(questions) => {
                // Save generated questions
                self.quiz_repo.save_questions_batch(&questions).await?;

                // Update status to completed
                self.quiz_repo.update_status(request_id, GenerationStatus::Completed).await?;

                Ok(QuizGenerationRequest {
                    request_id,
                    course_id,
                    lesson_id,
                    instructor_id,
                    config,
                    status: GenerationStatus::Completed,
                    created_at: now,
                    completed_at: Some(Utc::now()),
                })
            }
            Err(e) => {
                // Update status to failed
                self.quiz_repo.update_status(request_id, GenerationStatus::Failed).await?;
                Err(AIError::QuizGenerationFailed(e))
            }
        }
    }

    /// Gets generation status.
    pub async fn get_generation_status(&self, request_id: Uuid) -> AIResult<QuizGenerationRequest> {
        self.quiz_repo.get_request(request_id).await
    }

    /// Gets generated questions.
    pub async fn get_generated_questions(&self, request_id: Uuid) -> AIResult<Vec<GeneratedQuestion>> {
        // Verify request exists and is completed
        let request = self.quiz_repo.get_request(request_id).await?;

        if request.status != GenerationStatus::Completed {
            return Err(AIError::GenerationFailed(
                format!("Quiz generation is still {:?}", request.status)
            ));
        }

        self.quiz_repo.get_questions(request_id).await
    }

    /// Fetches lesson content.
    async fn fetch_lesson_content(&self, _lesson_id: Uuid) -> AIResult<String> {
        // TODO: Call courses-service to fetch lesson content
        // For now, return placeholder
        Ok("Lesson content would be fetched from courses-service".to_string())
    }
}
