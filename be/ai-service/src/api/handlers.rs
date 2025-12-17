//! # API Handlers
//!
//! HTTP request handlers for AI Service endpoints.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::api::{
    AppState, ApiResponse, CreateTutorSessionRequest, SendTutorMessageRequest,
    SemanticSearchRequest, GenerateContentRequest, GenerateQuizRequest,
    IndexCourseRequest, HealthResponse, TutorSessionResponse, TutorMessageResponse,
    SemanticSearchResultResponse, ContentGenerationResponse, QuizGenerationResponse,
    GeneratedQuestionsResponse, SummaryResponse, KeyPointsResponse,
    GlossaryResponse, IndexingResponse,
};
use crate::domain::AIError;

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        service: "ai-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    })
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Extracts user_id from request headers (set by API gateway).
fn extract_user_id(req: &HttpRequest) -> Result<Uuid, AIError> {
    req.headers()
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AIError::InvalidRequest("Missing or invalid X-User-Id header".to_string()))
}

/// Converts AIError to HttpResponse.
fn error_response(error: AIError) -> HttpResponse {
    let status = match error.status_code() {
        400 => actix_web::http::StatusCode::BAD_REQUEST,
        403 => actix_web::http::StatusCode::FORBIDDEN,
        404 => actix_web::http::StatusCode::NOT_FOUND,
        410 => actix_web::http::StatusCode::GONE,
        429 => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
        _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    };

    HttpResponse::build(status).json(ApiResponse::<()>::error(
        error.error_code(),
        &error.to_string(),
    ))
}

// =============================================================================
// TUTOR HANDLERS
// =============================================================================

/// Create a new tutor session.
pub async fn create_tutor_session(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateTutorSessionRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    match state.tutor_service.create_session(user_id, body.course_id, body.lesson_id).await {
        Ok(session) => HttpResponse::Created().json(ApiResponse::success(TutorSessionResponse {
            session_id: session.session_id,
            course_id: session.course_id,
            lesson_id: session.lesson_id,
            status: session.status,
            message_count: session.message_count,
            created_at: session.created_at,
            last_message_at: session.last_message_at,
        })),
        Err(e) => error_response(e),
    }
}

/// Get tutor session details.
pub async fn get_tutor_session(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let session_id = path.into_inner();

    match state.tutor_service.get_session(session_id, user_id).await {
        Ok(session) => HttpResponse::Ok().json(ApiResponse::success(TutorSessionResponse {
            session_id: session.session_id,
            course_id: session.course_id,
            lesson_id: session.lesson_id,
            status: session.status,
            message_count: session.message_count,
            created_at: session.created_at,
            last_message_at: session.last_message_at,
        })),
        Err(e) => error_response(e),
    }
}

/// Send a message to the tutor.
pub async fn send_tutor_message(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<SendTutorMessageRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let session_id = path.into_inner();

    match state.tutor_service.send_message(session_id, user_id, &body.content, body.lesson_id).await {
        Ok((_user_msg, assistant_msg)) => {
            let response = TutorMessageResponse {
                message_id: assistant_msg.message_id,
                session_id: assistant_msg.session_id,
                role: assistant_msg.role,
                content: assistant_msg.content,
                references: assistant_msg.references.map(|refs| {
                    refs.into_iter().map(Into::into).collect()
                }),
                created_at: assistant_msg.created_at,
            };
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// Send a message to the tutor with streaming response.
pub async fn send_tutor_message_stream(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<SendTutorMessageRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let session_id = path.into_inner();

    // For now, return a simple response - streaming will be implemented with SSE
    match state.tutor_service.send_message(session_id, user_id, &body.content, body.lesson_id).await {
        Ok((_, assistant_msg)) => {
            let response = TutorMessageResponse {
                message_id: assistant_msg.message_id,
                session_id: assistant_msg.session_id,
                role: assistant_msg.role,
                content: assistant_msg.content,
                references: assistant_msg.references.map(|refs| {
                    refs.into_iter().map(Into::into).collect()
                }),
                created_at: assistant_msg.created_at,
            };
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => error_response(e),
    }
}

/// Get messages from a tutor session.
pub async fn get_session_messages(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let session_id = path.into_inner();
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match state.tutor_service.get_messages(session_id, user_id, limit, offset).await {
        Ok(messages) => {
            let responses: Vec<TutorMessageResponse> = messages
                .into_iter()
                .map(|m| TutorMessageResponse {
                    message_id: m.message_id,
                    session_id: m.session_id,
                    role: m.role,
                    content: m.content,
                    references: m.references.map(|refs| {
                        refs.into_iter().map(Into::into).collect()
                    }),
                    created_at: m.created_at,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::success(responses))
        }
        Err(e) => error_response(e),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// =============================================================================
// SEMANTIC SEARCH HANDLERS
// =============================================================================

/// Perform semantic search.
pub async fn semantic_search(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<SemanticSearchRequest>,
) -> HttpResponse {
    let user_id = extract_user_id(&req).ok();
    let limit = body.limit.unwrap_or(10);
    let min_score = body.min_score.unwrap_or(0.7);

    match state.search_service.search(
        &body.query,
        user_id,
        body.course_ids.clone(),
        limit,
        min_score,
    ).await {
        Ok(results) => {
            let responses: Vec<SemanticSearchResultResponse> = results
                .into_iter()
                .map(|r| SemanticSearchResultResponse {
                    embedding_id: r.embedding_id,
                    course_id: r.course_id,
                    course_title: r.course_title,
                    lesson_id: r.lesson_id,
                    lesson_title: r.lesson_title,
                    content_type: r.content_type,
                    snippet: r.snippet,
                    similarity_score: r.similarity_score,
                    has_access: r.has_access,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::success(responses))
        }
        Err(e) => error_response(e),
    }
}

/// Index course content for semantic search.
pub async fn index_course_content(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<IndexCourseRequest>,
) -> HttpResponse {
    // This endpoint requires admin or instructor role - simplified check
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let course_id = path.into_inner();
    let force_reindex = body.force_reindex.unwrap_or(false);

    match state.search_service.index_course(course_id, force_reindex).await {
        Ok(chunks_indexed) => {
            HttpResponse::Ok().json(ApiResponse::success(IndexingResponse {
                course_id,
                chunks_indexed,
                status: "completed".to_string(),
            }))
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// CONTENT GENERATION HANDLERS
// =============================================================================

/// Generate lesson summary.
pub async fn generate_summary(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<GenerateContentRequest>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let language = body.language.clone().unwrap_or_else(|| "es".to_string());

    match state.summary_service.generate_summary(body.course_id, body.lesson_id, &language).await {
        Ok(summary) => {
            HttpResponse::Ok().json(ApiResponse::success(SummaryResponse {
                summary_id: summary.summary_id,
                lesson_id: summary.lesson_id,
                summary: summary.content,
                language: summary.language,
                generated_at: summary.created_at,
            }))
        }
        Err(e) => error_response(e),
    }
}

/// Generate key points from lesson.
pub async fn generate_key_points(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<GenerateContentRequest>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let language = body.language.clone().unwrap_or_else(|| "es".to_string());

    match state.summary_service.generate_key_points(body.course_id, body.lesson_id, &language).await {
        Ok(key_points) => {
            HttpResponse::Ok().json(ApiResponse::success(KeyPointsResponse {
                lesson_id: body.lesson_id,
                key_points: key_points.into_iter().map(|kp| crate::api::KeyPointResponse {
                    order_index: kp.order_index,
                    title: kp.title,
                    description: kp.description,
                    timestamp_seconds: kp.timestamp_seconds,
                }).collect(),
                language,
            }))
        }
        Err(e) => error_response(e),
    }
}

/// Generate glossary from lesson.
pub async fn generate_glossary(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<GenerateContentRequest>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let language = body.language.clone().unwrap_or_else(|| "es".to_string());

    match state.summary_service.generate_glossary(body.course_id, body.lesson_id, &language).await {
        Ok(terms) => {
            HttpResponse::Ok().json(ApiResponse::success(GlossaryResponse {
                lesson_id: body.lesson_id,
                terms: terms.into_iter().map(|t| crate::api::GlossaryTermResponse {
                    term: t.term,
                    definition: t.definition,
                    related_terms: t.related_terms,
                }).collect(),
                language,
            }))
        }
        Err(e) => error_response(e),
    }
}

/// Get generation status.
pub async fn get_generation_status(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let request_id = path.into_inner();

    match state.summary_service.get_generation_status(request_id).await {
        Ok(summary) => {
            HttpResponse::Ok().json(ApiResponse::success(ContentGenerationResponse {
                request_id,
                course_id: summary.course_id,
                lesson_id: summary.lesson_id,
                generation_type: summary.generation_type,
                status: summary.status,
                content: Some(summary.content),
                created_at: summary.created_at,
                completed_at: Some(summary.updated_at),
            }))
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// QUIZ GENERATION HANDLERS
// =============================================================================

/// Generate quiz from lesson content.
pub async fn generate_quiz(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<GenerateQuizRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    match state.quiz_generator_service.generate_quiz(
        body.course_id,
        body.lesson_id,
        user_id,
        body.question_count.unwrap_or(5),
        body.difficulty.clone(),
        body.question_types.clone(),
        body.language.clone(),
        body.include_explanations.unwrap_or(true),
    ).await {
        Ok(generation) => {
            HttpResponse::Accepted().json(ApiResponse::success(QuizGenerationResponse {
                request_id: generation.request_id,
                course_id: generation.course_id,
                lesson_id: generation.lesson_id,
                status: generation.status,
                config: crate::api::QuizConfigResponse {
                    question_count: generation.config.question_count,
                    difficulty: generation.config.difficulty,
                    question_types: generation.config.question_types,
                    language: generation.config.language,
                    include_explanations: generation.config.include_explanations,
                },
                created_at: generation.created_at,
                completed_at: generation.completed_at,
            }))
        }
        Err(e) => error_response(e),
    }
}

/// Get generated quiz status.
pub async fn get_generated_quiz(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let request_id = path.into_inner();

    match state.quiz_generator_service.get_generation_status(request_id).await {
        Ok(generation) => {
            HttpResponse::Ok().json(ApiResponse::success(QuizGenerationResponse {
                request_id: generation.request_id,
                course_id: generation.course_id,
                lesson_id: generation.lesson_id,
                status: generation.status,
                config: crate::api::QuizConfigResponse {
                    question_count: generation.config.question_count,
                    difficulty: generation.config.difficulty,
                    question_types: generation.config.question_types,
                    language: generation.config.language,
                    include_explanations: generation.config.include_explanations,
                },
                created_at: generation.created_at,
                completed_at: generation.completed_at,
            }))
        }
        Err(e) => error_response(e),
    }
}

/// Get generated questions.
pub async fn get_generated_questions(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let _user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };
    let request_id = path.into_inner();

    match state.quiz_generator_service.get_generated_questions(request_id).await {
        Ok(questions) => {
            let total = questions.len() as i32;
            let responses: Vec<crate::api::GeneratedQuestionResponse> = questions
                .into_iter()
                .map(|q| crate::api::GeneratedQuestionResponse {
                    question_id: q.question_id,
                    question_type: q.question_type,
                    difficulty: q.difficulty,
                    question_text: q.question_text,
                    options: q.options.map(|opts| opts.into_iter().map(Into::into).collect()),
                    correct_answer: q.correct_answer,
                    explanation: q.explanation,
                    points: q.points,
                    source_reference: q.source_reference,
                })
                .collect();

            HttpResponse::Ok().json(ApiResponse::success(GeneratedQuestionsResponse {
                request_id,
                questions: responses,
                total,
            }))
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// USAGE HANDLERS
// =============================================================================

/// Get user's AI usage quota.
pub async fn get_usage_quota(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Ok(id) => id,
        Err(e) => return error_response(e),
    };

    match state.tutor_service.get_usage_quota(user_id).await {
        Ok(quota) => HttpResponse::Ok().json(ApiResponse::success(quota)),
        Err(e) => error_response(e),
    }
}
