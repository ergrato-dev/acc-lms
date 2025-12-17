//! # OpenAI Client
//!
//! OpenAI API client implementation.

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateEmbeddingRequestArgs,
    },
    Client,
};
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    DifficultyLevel, GeneratedQuestion, GlossaryTerm, KeyPoint,
    QuestionOption, QuestionType, QuizGenerationConfig, TutorMessage,
};
use crate::llm::LLMClient;

/// OpenAI API client.
pub struct OpenAIClient {
    client: Client<OpenAIConfig>,
    chat_model: String,
    embedding_model: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);

        Self {
            client,
            chat_model: "gpt-4".to_string(),
            embedding_model: "text-embedding-3-small".to_string(),
        }
    }

    pub fn with_models(mut self, chat_model: &str, embedding_model: &str) -> Self {
        self.chat_model = chat_model.to_string();
        self.embedding_model = embedding_model.to_string();
        self
    }

    /// Builds system prompt for tutor.
    fn build_tutor_system_prompt(&self, context: &str) -> String {
        format!(
            r#"Eres un tutor educativo inteligente que ayuda a estudiantes a comprender el contenido de sus cursos.

REGLAS:
1. Solo responde preguntas relacionadas con el contenido del curso proporcionado.
2. Si la pregunta está fuera del tema del curso, responde cortésmente que solo puedes ayudar con el contenido del curso.
3. Usa el contexto proporcionado para dar respuestas precisas y relevantes.
4. Cita las lecciones específicas cuando sea relevante.
5. Mantén un tono amigable y educativo.
6. Si no tienes suficiente información, admítelo honestamente.

CONTEXTO DEL CURSO:
{}"#,
            context
        )
    }

    /// Builds messages for chat completion.
    fn build_chat_messages(
        &self,
        system_prompt: &str,
        history: &[TutorMessage],
        query: &str,
    ) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()
                    .unwrap()
            )
        ];

        // Add history (last few messages)
        for msg in history.iter().take(10) {
            match msg.role {
                crate::domain::MessageRole::User => {
                    messages.push(ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(msg.content.clone())
                            .build()
                            .unwrap()
                    ));
                }
                crate::domain::MessageRole::Assistant => {
                    messages.push(ChatCompletionRequestMessage::Assistant(
                        async_openai::types::ChatCompletionRequestAssistantMessageArgs::default()
                            .content(msg.content.clone())
                            .build()
                            .unwrap()
                    ));
                }
                _ => {}
            }
        }

        // Add current query
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessageArgs::default()
                .content(query)
                .build()
                .unwrap()
        ));

        messages
    }
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.embedding_model)
            .input(text)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let embedding = response.data
            .first()
            .ok_or("No embedding returned")?
            .embedding
            .clone();

        Ok(embedding)
    }

    async fn generate_tutor_response(
        &self,
        query: &str,
        history: &[TutorMessage],
        context: &str,
        _course_id: Uuid,
    ) -> Result<(String, i32), String> {
        let system_prompt = self.build_tutor_system_prompt(context);
        let messages = self.build_chat_messages(&system_prompt, history, query);

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.chat_model)
            .messages(messages)
            .max_tokens(1000u32)
            .temperature(0.7)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let content = response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let tokens = response.usage
            .map(|u| u.total_tokens as i32)
            .unwrap_or(0);

        Ok((content, tokens))
    }

    async fn generate_summary(&self, content: &str, language: &str) -> Result<(String, i32), String> {
        let lang_name = match language {
            "es" => "español",
            "en" => "English",
            "pt" => "português",
            _ => "español",
        };

        let prompt = format!(
            r#"Genera un resumen conciso del siguiente contenido educativo en {}.
El resumen debe:
1. Capturar los puntos principales
2. Ser claro y fácil de entender
3. Tener entre 150-300 palabras

CONTENIDO:
{}"#,
            lang_name, content
        );

        let messages = vec![
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.clone())
                    .build()
                    .unwrap()
            )
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.chat_model)
            .messages(messages)
            .max_tokens(500u32)
            .temperature(0.3)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let summary = response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let tokens = response.usage
            .map(|u| u.total_tokens as i32)
            .unwrap_or(0);

        Ok((summary, tokens))
    }

    async fn generate_key_points(&self, content: &str, language: &str) -> Result<(Vec<KeyPoint>, i32), String> {
        let lang_name = match language {
            "es" => "español",
            "en" => "English",
            "pt" => "português",
            _ => "español",
        };

        let prompt = format!(
            r#"Extrae los puntos clave del siguiente contenido educativo en {}.
Responde SOLO con un JSON array con el siguiente formato:
[{{"order_index": 1, "title": "Título del punto", "description": "Descripción breve"}}]

CONTENIDO:
{}"#,
            lang_name, content
        );

        let messages = vec![
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.clone())
                    .build()
                    .unwrap()
            )
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.chat_model)
            .messages(messages)
            .max_tokens(1000u32)
            .temperature(0.3)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let content_str = response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let key_points: Vec<KeyPoint> = serde_json::from_str(&content_str)
            .unwrap_or_default();

        let tokens = response.usage
            .map(|u| u.total_tokens as i32)
            .unwrap_or(0);

        Ok((key_points, tokens))
    }

    async fn generate_glossary(&self, content: &str, language: &str) -> Result<(Vec<GlossaryTerm>, i32), String> {
        let lang_name = match language {
            "es" => "español",
            "en" => "English",
            "pt" => "português",
            _ => "español",
        };

        let prompt = format!(
            r#"Extrae un glosario de términos técnicos del siguiente contenido educativo en {}.
Responde SOLO con un JSON array con el siguiente formato:
[{{"term": "Término", "definition": "Definición clara", "related_terms": ["término1", "término2"]}}]

CONTENIDO:
{}"#,
            lang_name, content
        );

        let messages = vec![
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.clone())
                    .build()
                    .unwrap()
            )
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.chat_model)
            .messages(messages)
            .max_tokens(1000u32)
            .temperature(0.3)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let content_str = response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let terms: Vec<GlossaryTerm> = serde_json::from_str(&content_str)
            .unwrap_or_default();

        let tokens = response.usage
            .map(|u| u.total_tokens as i32)
            .unwrap_or(0);

        Ok((terms, tokens))
    }

    async fn generate_quiz(&self, content: &str, config: &QuizGenerationConfig) -> Result<Vec<GeneratedQuestion>, String> {
        let lang_name = match config.language.as_str() {
            "es" => "español",
            "en" => "English",
            "pt" => "português",
            _ => "español",
        };

        let difficulty_str = match config.difficulty {
            DifficultyLevel::Easy => "fácil",
            DifficultyLevel::Medium => "intermedia",
            DifficultyLevel::Hard => "difícil",
        };

        let types_str: Vec<&str> = config.question_types.iter().map(|t| match t {
            QuestionType::SingleChoice => "opción única",
            QuestionType::MultipleChoice => "opción múltiple",
            QuestionType::TrueFalse => "verdadero/falso",
            QuestionType::ShortAnswer => "respuesta corta",
            QuestionType::Code => "código",
        }).collect();

        let prompt = format!(
            r#"Genera {} preguntas de quiz basadas en el siguiente contenido educativo.

REQUISITOS:
- Idioma: {}
- Dificultad: {}
- Tipos de pregunta: {}
- Incluir explicaciones: {}

Responde SOLO con un JSON array con el siguiente formato:
[{{
    "question_type": "single_choice|multiple_choice|true_false|short_answer|code",
    "difficulty": "easy|medium|hard",
    "question_text": "Texto de la pregunta",
    "options": [{{"option_id": "a", "text": "Opción A", "is_correct": false}}],
    "correct_answer": "a",
    "explanation": "Explicación de la respuesta correcta",
    "points": 10
}}]

CONTENIDO:
{}"#,
            config.question_count,
            lang_name,
            difficulty_str,
            types_str.join(", "),
            if config.include_explanations { "sí" } else { "no" },
            content
        );

        let messages = vec![
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.clone())
                    .build()
                    .unwrap()
            )
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.chat_model)
            .messages(messages)
            .max_tokens(2000u32)
            .temperature(0.5)
            .build()
            .map_err(|e| e.to_string())?;

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| e.to_string())?;

        let content_str = response.choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        // Parse the JSON response
        let raw_questions: Vec<RawQuestion> = serde_json::from_str(&content_str)
            .map_err(|e| format!("Failed to parse quiz response: {}", e))?;

        // Convert to domain objects
        let questions: Vec<GeneratedQuestion> = raw_questions
            .into_iter()
            .map(|q| GeneratedQuestion {
                question_id: Uuid::now_v7(),
                request_id: Uuid::nil(), // Will be set by caller
                question_type: parse_question_type(&q.question_type),
                difficulty: parse_difficulty(&q.difficulty),
                question_text: q.question_text,
                options: q.options.map(|opts| opts.into_iter().map(|o| QuestionOption {
                    option_id: o.option_id,
                    text: o.text,
                    is_correct: o.is_correct,
                }).collect()),
                correct_answer: q.correct_answer,
                explanation: q.explanation,
                points: q.points.unwrap_or(10),
                source_reference: None,
            })
            .collect();

        Ok(questions)
    }
}

// Helper structs for JSON parsing
#[derive(serde::Deserialize)]
struct RawQuestion {
    question_type: String,
    difficulty: String,
    question_text: String,
    options: Option<Vec<RawOption>>,
    correct_answer: String,
    explanation: Option<String>,
    points: Option<i32>,
}

#[derive(serde::Deserialize)]
struct RawOption {
    option_id: String,
    text: String,
    is_correct: bool,
}

fn parse_question_type(s: &str) -> QuestionType {
    match s {
        "single_choice" => QuestionType::SingleChoice,
        "multiple_choice" => QuestionType::MultipleChoice,
        "true_false" => QuestionType::TrueFalse,
        "short_answer" => QuestionType::ShortAnswer,
        "code" => QuestionType::Code,
        _ => QuestionType::SingleChoice,
    }
}

fn parse_difficulty(s: &str) -> DifficultyLevel {
    match s {
        "easy" => DifficultyLevel::Easy,
        "medium" => DifficultyLevel::Medium,
        "hard" => DifficultyLevel::Hard,
        _ => DifficultyLevel::Medium,
    }
}
