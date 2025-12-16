//! # Chatbot API Handlers
//!
//! HTTP handlers for the chatbot service.

use actix_web::{web, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::domain::{NewConversation, NewMessage, NewKBArticle, ConversationStatus};
use crate::service::ChatbotService;
use crate::api::dto::*;

/// Application state shared across handlers.
pub struct AppState {
    pub chatbot_service: Arc<ChatbotService>,
}

// =============================================================================
// CONVERSATION HANDLERS
// =============================================================================

/// POST /api/v1/chatbot/conversations - Start new conversation
pub async fn start_conversation(
    state: web::Data<AppState>,
    body: web::Json<StartConversationRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    let request = NewConversation {
        tenant_id: body.tenant_id,
        user_id: body.user_id,
        user_role: body.user_role,
        context: body.context.clone().map(|c| c.into()),
    };

    match state.chatbot_service.start_conversation(request).await {
        Ok(conversation) => {
            let messages = state.chatbot_service
                .get_history(conversation.conversation_id, 10, 0)
                .await
                .unwrap_or_default();

            HttpResponse::Created().json(ApiResponse {
                success: true,
                data: ConversationResponse::from((conversation, messages)),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "CONVERSATION_ERROR".to_string(),
        }),
    }
}

/// POST /api/v1/chatbot/conversations/{id}/messages - Send message
pub async fn send_message(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<SendMessageRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    let conversation_id = path.into_inner();
    let request = NewMessage {
        conversation_id,
        content: body.content.clone(),
        context_update: body.context_update.clone().map(|c| c.into()),
    };

    match state.chatbot_service.process_message(request).await {
        Ok((user_msg, bot_msg)) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: SendMessageResponse {
                    user_message: MessageResponse::from(user_msg),
                    bot_message: MessageResponse::from(bot_msg),
                },
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "MESSAGE_ERROR".to_string(),
        }),
    }
}

/// GET /api/v1/chatbot/conversations/{id}/history - Get message history
pub async fn get_history(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let conversation_id = path.into_inner();
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match state.chatbot_service.get_history(conversation_id, limit, offset).await {
        Ok(messages) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: messages.into_iter().map(MessageResponse::from).collect::<Vec<_>>(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "HISTORY_ERROR".to_string(),
        }),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// POST /api/v1/chatbot/messages/{id}/feedback - Add feedback to message
pub async fn add_feedback(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<FeedbackRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    let message_id = path.into_inner();

    match state.chatbot_service.add_feedback(
        message_id,
        body.feedback_type.clone().into(),
        body.comment.clone(),
    ).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: serde_json::json!({"message": "Feedback recorded"}),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "FEEDBACK_ERROR".to_string(),
        }),
    }
}

/// POST /api/v1/chatbot/conversations/{id}/escalate - Escalate to human
pub async fn escalate_conversation(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<EscalateRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    let conversation_id = path.into_inner();

    match state.chatbot_service.escalate(
        conversation_id,
        body.reason.clone().into(),
        body.notes.clone(),
    ).await {
        Ok(conversation) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: EscalationResponse {
                    conversation_id: conversation.conversation_id,
                    status: "escalated".to_string(),
                    ticket_id: conversation.escalation.and_then(|e| e.ticket_id),
                    message: "Tu conversación ha sido escalada a un agente humano. Te contactaremos pronto.".to_string(),
                },
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "ESCALATION_ERROR".to_string(),
        }),
    }
}

/// PUT /api/v1/chatbot/conversations/{id}/end - End conversation
pub async fn end_conversation(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let conversation_id = path.into_inner();

    match state.chatbot_service.end_conversation(conversation_id, ConversationStatus::Resolved).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: serde_json::json!({"message": "Conversation ended"}),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "END_ERROR".to_string(),
        }),
    }
}

// =============================================================================
// KNOWLEDGE BASE HANDLERS
// =============================================================================

/// GET /api/v1/kb/search - Search knowledge base
pub async fn search_kb(
    state: web::Data<AppState>,
    query: web::Query<KBSearchRequest>,
) -> HttpResponse {
    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    match state.chatbot_service.search_knowledge_base(&query.query, query.role, &query.language).await {
        Ok(results) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: results.into_iter().map(KBSearchResultResponse::from).collect::<Vec<_>>(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "SEARCH_ERROR".to_string(),
        }),
    }
}

/// GET /api/v1/kb/articles/{slug} - Get article by slug
pub async fn get_article(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let slug = path.into_inner();

    match state.chatbot_service.get_article(&slug).await {
        Ok(article) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: KBArticleResponse::from(article),
            })
        }
        Err(e) => HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "NOT_FOUND".to_string(),
        }),
    }
}

/// GET /api/v1/kb/articles/popular - Get popular articles
pub async fn get_popular_articles(
    state: web::Data<AppState>,
    query: web::Query<PopularArticlesQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).min(50);

    match state.chatbot_service.get_popular_articles(query.role, limit).await {
        Ok(articles) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: articles.into_iter().map(KBArticleResponse::from).collect::<Vec<_>>(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "KB_ERROR".to_string(),
        }),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PopularArticlesQuery {
    #[serde(default)]
    pub role: crate::domain::UserRole,
    pub limit: Option<i64>,
}

/// GET /api/v1/kb/categories/{category} - Get articles by category
pub async fn get_articles_by_category(
    state: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<CategoryArticlesQuery>,
) -> HttpResponse {
    let category = path.into_inner();
    let limit = query.limit.unwrap_or(20).min(50);

    match state.chatbot_service.get_articles_by_category(&category, query.role, limit).await {
        Ok(articles) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: articles.into_iter().map(KBArticleResponse::from).collect::<Vec<_>>(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "KB_ERROR".to_string(),
        }),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CategoryArticlesQuery {
    #[serde(default)]
    pub role: crate::domain::UserRole,
    pub limit: Option<i64>,
}

/// POST /api/v1/kb/articles/{id}/feedback - Record article feedback
pub async fn record_article_feedback(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<ArticleFeedbackRequest>,
) -> HttpResponse {
    let article_id = path.into_inner();

    match state.chatbot_service.record_article_feedback(article_id, body.helpful).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: serde_json::json!({"message": "Feedback recorded"}),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "FEEDBACK_ERROR".to_string(),
        }),
    }
}

/// POST /api/v1/kb/articles - Create article (admin only)
pub async fn create_article(
    state: web::Data<AppState>,
    body: web::Json<CreateArticleRequest>,
    // In production, extract author_id from JWT
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        });
    }

    let article = NewKBArticle {
        slug: body.slug.clone(),
        title: body.title.clone(),
        content: body.content.clone(),
        summary: body.summary.clone(),
        category: body.category.clone(),
        subcategory: body.subcategory.clone(),
        tags: body.tags.clone(),
        keywords: body.keywords.clone(),
        intent_triggers: body.intent_triggers.clone(),
        target_roles: body.target_roles.clone(),
        language: body.language.clone(),
        author_id: Uuid::nil(), // Should come from JWT
    };

    match state.chatbot_service.create_article(article).await {
        Ok(article) => {
            HttpResponse::Created().json(ApiResponse {
                success: true,
                data: KBArticleResponse::from(article),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "CREATE_ERROR".to_string(),
        }),
    }
}

// =============================================================================
// SUGGESTIONS HANDLERS
// =============================================================================

/// GET /api/v1/chatbot/suggestions - Get contextual suggestions
pub async fn get_suggestions(
    state: web::Data<AppState>,
    query: web::Query<SuggestionsRequest>,
) -> HttpResponse {
    match state.chatbot_service.get_suggestions(query.role, query.page.as_deref()).await {
        Ok(suggestions) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: suggestions.into_iter().map(SuggestionResponse::from).collect::<Vec<_>>(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "SUGGESTIONS_ERROR".to_string(),
        }),
    }
}

// =============================================================================
// ANALYTICS HANDLERS
// =============================================================================

/// GET /api/v1/chatbot/analytics - Get chatbot analytics (admin only)
pub async fn get_analytics(
    state: web::Data<AppState>,
    query: web::Query<AnalyticsQuery>,
) -> HttpResponse {
    match state.chatbot_service.get_analytics(query.start, query.end).await {
        Ok(analytics) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: AnalyticsResponse::from(analytics),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: e.to_string(),
            code: "ANALYTICS_ERROR".to_string(),
        }),
    }
}

// =============================================================================
// HEALTH HANDLERS
// =============================================================================

/// GET /health - Health check
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "chatbot-service"
    }))
}

/// GET /ready - Readiness check
pub async fn readiness_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "service": "chatbot-service"
    }))
}
