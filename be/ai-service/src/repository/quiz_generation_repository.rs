//! # Quiz Generation Repository
//!
//! Data access for quiz generation requests and results.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    AIError, AIResult, DifficultyLevel, GeneratedQuestion, GenerationStatus,
    QuestionOption, QuestionType, QuizGenerationConfig, QuizGenerationRequest,
};

/// Repository for quiz generation operations.
#[derive(Clone)]
pub struct QuizGenerationRepository {
    pool: PgPool,
}

impl QuizGenerationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new quiz generation request.
    pub async fn create_request(&self, request: &QuizGenerationRequest) -> AIResult<()> {
        let config_json = serde_json::to_value(&request.config)
            .map_err(|e| AIError::Database(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO ai.quiz_generation_requests (
                request_id, course_id, lesson_id, instructor_id,
                config, status, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(request.request_id)
        .bind(request.course_id)
        .bind(request.lesson_id)
        .bind(request.instructor_id)
        .bind(config_json)
        .bind(gen_status_to_str(&request.status))
        .bind(request.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Gets a generation request by ID.
    pub async fn get_request(&self, request_id: Uuid) -> AIResult<QuizGenerationRequest> {
        let row = sqlx::query(
            r#"
            SELECT request_id, course_id, lesson_id, instructor_id,
                   config, status, created_at, completed_at
            FROM ai.quiz_generation_requests
            WHERE request_id = $1
            "#
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?
        .ok_or(AIError::GenerationNotFound(request_id))?;

        Ok(self.map_request_row(&row))
    }

    /// Updates request status.
    pub async fn update_status(
        &self,
        request_id: Uuid,
        status: GenerationStatus,
    ) -> AIResult<()> {
        let completed_at = if status == GenerationStatus::Completed || status == GenerationStatus::Failed {
            Some(Utc::now())
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE ai.quiz_generation_requests
            SET status = $2, completed_at = $3
            WHERE request_id = $1
            "#
        )
        .bind(request_id)
        .bind(gen_status_to_str(&status))
        .bind(completed_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Saves a generated question.
    pub async fn save_question(&self, question: &GeneratedQuestion) -> AIResult<()> {
        let options_json = question.options.as_ref()
            .map(|opts| serde_json::to_value(opts).ok())
            .flatten();

        sqlx::query(
            r#"
            INSERT INTO ai.generated_questions (
                question_id, request_id, question_type, difficulty,
                question_text, options, correct_answer, explanation,
                points, source_reference
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(question.question_id)
        .bind(question.request_id)
        .bind(question_type_to_str(&question.question_type))
        .bind(difficulty_to_str(&question.difficulty))
        .bind(&question.question_text)
        .bind(options_json)
        .bind(&question.correct_answer)
        .bind(&question.explanation)
        .bind(question.points)
        .bind(&question.source_reference)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    /// Saves multiple questions in batch.
    pub async fn save_questions_batch(&self, questions: &[GeneratedQuestion]) -> AIResult<()> {
        for question in questions {
            self.save_question(question).await?;
        }
        Ok(())
    }

    /// Gets generated questions for a request.
    pub async fn get_questions(&self, request_id: Uuid) -> AIResult<Vec<GeneratedQuestion>> {
        let rows = sqlx::query(
            r#"
            SELECT question_id, request_id, question_type, difficulty,
                   question_text, options, correct_answer, explanation,
                   points, source_reference
            FROM ai.generated_questions
            WHERE request_id = $1
            ORDER BY question_id
            "#
        )
        .bind(request_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(rows.iter().map(|r| self.map_question_row(r)).collect())
    }

    /// Deletes questions for a request.
    pub async fn delete_questions(&self, request_id: Uuid) -> AIResult<()> {
        sqlx::query(
            "DELETE FROM ai.generated_questions WHERE request_id = $1"
        )
        .bind(request_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AIError::Database(e.to_string()))?;

        Ok(())
    }

    // Helper methods
    fn map_request_row(&self, row: &sqlx::postgres::PgRow) -> QuizGenerationRequest {
        use sqlx::Row;

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "pending" => GenerationStatus::Pending,
            "processing" => GenerationStatus::Processing,
            "completed" => GenerationStatus::Completed,
            "failed" => GenerationStatus::Failed,
            _ => GenerationStatus::Pending,
        };

        let config_json: serde_json::Value = row.get("config");
        let config: QuizGenerationConfig = serde_json::from_value(config_json)
            .unwrap_or_default();

        QuizGenerationRequest {
            request_id: row.get("request_id"),
            course_id: row.get("course_id"),
            lesson_id: row.get("lesson_id"),
            instructor_id: row.get("instructor_id"),
            config,
            status,
            created_at: row.get("created_at"),
            completed_at: row.get("completed_at"),
        }
    }

    fn map_question_row(&self, row: &sqlx::postgres::PgRow) -> GeneratedQuestion {
        use sqlx::Row;

        let qtype_str: String = row.get("question_type");
        let question_type = match qtype_str.as_str() {
            "single_choice" => QuestionType::SingleChoice,
            "multiple_choice" => QuestionType::MultipleChoice,
            "true_false" => QuestionType::TrueFalse,
            "short_answer" => QuestionType::ShortAnswer,
            "code" => QuestionType::Code,
            _ => QuestionType::SingleChoice,
        };

        let diff_str: String = row.get("difficulty");
        let difficulty = match diff_str.as_str() {
            "easy" => DifficultyLevel::Easy,
            "medium" => DifficultyLevel::Medium,
            "hard" => DifficultyLevel::Hard,
            _ => DifficultyLevel::Medium,
        };

        let options_json: Option<serde_json::Value> = row.get("options");
        let options = options_json
            .and_then(|v| serde_json::from_value::<Vec<QuestionOption>>(v).ok());

        GeneratedQuestion {
            question_id: row.get("question_id"),
            request_id: row.get("request_id"),
            question_type,
            difficulty,
            question_text: row.get("question_text"),
            options,
            correct_answer: row.get("correct_answer"),
            explanation: row.get("explanation"),
            points: row.get("points"),
            source_reference: row.get("source_reference"),
        }
    }
}

fn gen_status_to_str(s: &GenerationStatus) -> &'static str {
    match s {
        GenerationStatus::Pending => "pending",
        GenerationStatus::Processing => "processing",
        GenerationStatus::Completed => "completed",
        GenerationStatus::Failed => "failed",
    }
}

fn difficulty_to_str(d: &DifficultyLevel) -> &'static str {
    match d {
        DifficultyLevel::Easy => "easy",
        DifficultyLevel::Medium => "medium",
        DifficultyLevel::Hard => "hard",
    }
}

fn question_type_to_str(t: &QuestionType) -> &'static str {
    match t {
        QuestionType::SingleChoice => "single_choice",
        QuestionType::MultipleChoice => "multiple_choice",
        QuestionType::TrueFalse => "true_false",
        QuestionType::ShortAnswer => "short_answer",
        QuestionType::Code => "code",
    }
}
