//! # ACC LMS - Chatbot Service
//!
//! AI-powered chatbot service with contextual support for all user roles.
//!
//! ## Features
//!
//! - Multi-role support (Anonymous, Student, Instructor, Admin)
//! - Contextual conversations with session management
//! - Knowledge base integration with semantic search
//! - Intelligent escalation to human agents
//! - Feedback collection and analytics
//! - Rich content support (cards, carousels, code snippets)
//!
//! ## API Endpoints
//!
//! ### Chatbot
//! - POST   /api/v1/chatbot/conversations           - Start conversation
//! - POST   /api/v1/chatbot/conversations/{id}/messages - Send message
//! - GET    /api/v1/chatbot/conversations/{id}/history  - Get history
//! - POST   /api/v1/chatbot/conversations/{id}/escalate - Escalate to human
//! - PUT    /api/v1/chatbot/conversations/{id}/end      - End conversation
//! - POST   /api/v1/chatbot/messages/{id}/feedback      - Add feedback
//! - GET    /api/v1/chatbot/suggestions                 - Get suggestions
//! - GET    /api/v1/chatbot/analytics                   - Get analytics
//!
//! ### Knowledge Base
//! - GET    /api/v1/kb/search                    - Search KB
//! - GET    /api/v1/kb/articles/popular          - Popular articles
//! - GET    /api/v1/kb/articles/{slug}           - Get article
//! - POST   /api/v1/kb/articles                  - Create article
//! - POST   /api/v1/kb/articles/{id}/feedback    - Article feedback
//! - GET    /api/v1/kb/categories/{category}     - Articles by category
//!
//! ### Health
//! - GET    /health - Health check
//! - GET    /ready  - Readiness check

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod repository;
mod service;

use api::{configure_routes, AppState};
use repository::ChatbotRepository;
use service::{ChatbotService, AIClient, AIResponse};

/// Server configuration.
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    database_url: String,
    max_connections: u32,
    openai_api_key: Option<String>,
}

impl Config {
    fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8087".to_string())
                .parse()
                .expect("PORT must be a number"),
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
        }
    }
}

// =============================================================================
// SIMPLE AI CLIENT (for initial implementation)
// =============================================================================

/// Simple rule-based AI client (can be replaced with OpenAI integration).
struct SimpleAIClient;

#[async_trait::async_trait]
impl AIClient for SimpleAIClient {
    async fn generate_response(
        &self,
        message: &str,
        context: &domain::ConversationContext,
        role: domain::UserRole,
        _history: &[domain::Message],
    ) -> std::result::Result<AIResponse, String> {
        let message_lower = message.to_lowercase();

        // Simple keyword-based responses (can be enhanced with actual AI)
        let (text, confidence) = if message_lower.contains("hola") || message_lower.contains("buenos") {
            ("¡Hola! ¿En qué puedo ayudarte?".to_string(), 0.95)
        } else if message_lower.contains("gracias") {
            ("¡De nada! ¿Hay algo más en lo que pueda ayudarte?".to_string(), 0.95)
        } else if message_lower.contains("progreso") || message_lower.contains("avance") {
            match role {
                domain::UserRole::Student => {
                    ("Puedes ver tu progreso en la sección 'Mi Progreso' del dashboard. Ahí verás el porcentaje completado de cada curso y tus logros.".to_string(), 0.85)
                }
                _ => ("Para ver el progreso necesitas estar registrado como estudiante.".to_string(), 0.7)
            }
        } else if message_lower.contains("certificado") {
            ("Los certificados se generan automáticamente cuando completas un curso al 100%. Los encontrarás en la sección 'Mis Certificados'.".to_string(), 0.85)
        } else if message_lower.contains("pago") || message_lower.contains("precio") {
            ("Para temas de pagos, te recomiendo revisar nuestra sección de FAQ o contactar a soporte para ayuda personalizada.".to_string(), 0.75)
        } else if message_lower.contains("curso") && message_lower.contains("crear") {
            match role {
                domain::UserRole::Instructor | domain::UserRole::Admin => {
                    ("Para crear un curso, ve a tu dashboard de instructor y haz clic en 'Nuevo Curso'. Te guiaré paso a paso en el proceso.".to_string(), 0.85)
                }
                _ => ("Solo los instructores pueden crear cursos. Si deseas convertirte en instructor, revisa nuestra sección de registro de instructores.".to_string(), 0.8)
            }
        } else if message_lower.contains("problema") || message_lower.contains("error") || message_lower.contains("no funciona") {
            ("Lamento que estés teniendo problemas. ¿Podrías darme más detalles sobre lo que está pasando? Si prefieres, puedo escalarlo a un agente humano.".to_string(), 0.6)
        } else if message_lower.contains("humano") || message_lower.contains("agente") || message_lower.contains("persona") {
            return Ok(AIResponse {
                text: "Entendido, te voy a conectar con un agente humano que podrá ayudarte mejor.".to_string(),
                confidence: 1.0,
                intent: None,
                suggested_articles: vec![],
                should_escalate: true,
                rich_content: None,
            });
        } else {
            ("No estoy seguro de entender tu consulta. ¿Podrías reformularla o ser más específico?".to_string(), 0.4)
        };

        Ok(AIResponse {
            text,
            confidence,
            intent: None,
            suggested_articles: vec![],
            should_escalate: false,
            rich_content: None,
        })
    }

    async fn detect_intent(
        &self,
        message: &str,
        _role: domain::UserRole,
    ) -> std::result::Result<domain::DetectedIntent, String> {
        let message_lower = message.to_lowercase();

        let (name, category, confidence) = if message_lower.contains("hola") || message_lower.contains("buenos") {
            ("greeting", domain::IntentCategory::Greeting, 0.95)
        } else if message_lower.contains("gracias") {
            ("thanks", domain::IntentCategory::Thanks, 0.95)
        } else if message_lower.contains("adiós") || message_lower.contains("chao") {
            ("farewell", domain::IntentCategory::Farewell, 0.95)
        } else if message_lower.contains("progreso") {
            ("check_progress", domain::IntentCategory::CourseProgress, 0.85)
        } else if message_lower.contains("certificado") {
            ("get_certificate", domain::IntentCategory::Certificate, 0.85)
        } else if message_lower.contains("pago") {
            ("payment_help", domain::IntentCategory::Payment, 0.8)
        } else if message_lower.contains("crear") && message_lower.contains("curso") {
            ("create_course", domain::IntentCategory::CourseCreation, 0.85)
        } else if message_lower.contains("problema") || message_lower.contains("error") {
            ("technical_issue", domain::IntentCategory::TechnicalIssue, 0.7)
        } else {
            ("unknown", domain::IntentCategory::Unknown, 0.3)
        };

        Ok(domain::DetectedIntent {
            name: name.to_string(),
            category,
            confidence,
            entities: vec![],
        })
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,chatbot_service=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    info!("Starting Chatbot Service...");
    info!("Host: {}:{}", config.host, config.port);

    // Create database pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database pool");

    info!("Database connection pool created");

    // Create repository
    let repository = ChatbotRepository::new(pool);

    // Create AI client (can be replaced with OpenAI integration)
    let ai_client: Arc<dyn AIClient> = Arc::new(SimpleAIClient);

    // Create service
    let service = Arc::new(
        ChatbotService::new(repository, ai_client)
            .with_escalation_threshold(0.6)
    );

    // Create app state
    let app_state = web::Data::new(AppState {
        chatbot_service: service,
    });

    info!("Starting HTTP server on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
