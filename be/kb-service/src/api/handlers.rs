// =============================================================================
// ACC LMS - Knowledge Base HTTP Handlers
// =============================================================================
// Handlers para endpoints de la API de KB
// =============================================================================

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{info, error};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::{
    ArticleFilters, ArticleStatus, ArticleVisibility, ContentType,
    CreateArticleDto, CreateCategoryDto, CreateFaqDto, CreateFeedbackDto,
    UpdateArticleDto, UpdateCategoryDto,
};
use crate::service::{KbService, KbError};

type ServiceData = web::Data<std::sync::Arc<KbService>>;

// =============================================================================
// CATEGORIES
// =============================================================================

/// GET /api/v1/kb/categories
/// Lista categorías
pub async fn list_categories(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<ListCategoriesQuery>,
) -> HttpResponse {
    let tenant_id = extract_tenant_id(&req);
    let include_hidden = query.include_hidden.unwrap_or(false);

    match service.list_categories(query.parent_id, include_hidden, tenant_id).await {
        Ok(result) => HttpResponse::Ok().json(CategoryListResponse::from(result)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/categories/{category_id}
/// Obtiene categoría por ID
pub async fn get_category(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let category_id = path.into_inner();

    match service.get_category(category_id).await {
        Ok(category) => HttpResponse::Ok().json(CategoryResponse::from(category)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/categories/slug/{slug}
/// Obtiene categoría por slug
pub async fn get_category_by_slug(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let slug = path.into_inner();
    let tenant_id = extract_tenant_id(&req);

    match service.get_category_by_slug(&slug, tenant_id).await {
        Ok(category) => HttpResponse::Ok().json(CategoryResponse::from(category)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/kb/categories
/// Crea categoría (admin)
pub async fn create_category(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateCategoryBody>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

    let tenant_id = extract_tenant_id(&req);

    let dto = CreateCategoryDto {
        name: body.name.clone(),
        description: body.description.clone(),
        icon: body.icon.clone(),
        color: body.color.clone(),
        parent_id: body.parent_id,
        order_index: body.order_index,
    };

    match service.create_category(dto, tenant_id).await {
        Ok(category) => HttpResponse::Created().json(CategoryResponse::from(category)),
        Err(e) => error_response(e),
    }
}

/// PATCH /api/v1/kb/categories/{category_id}
/// Actualiza categoría (admin)
pub async fn update_category(
    service: ServiceData,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCategoryBody>,
) -> HttpResponse {
    let category_id = path.into_inner();

    let dto = UpdateCategoryDto {
        name: body.name.clone(),
        description: body.description.clone(),
        icon: body.icon.clone(),
        color: body.color.clone(),
        parent_id: body.parent_id,
        order_index: body.order_index,
        is_visible: body.is_visible,
        is_featured: body.is_featured,
    };

    match service.update_category(category_id, dto).await {
        Ok(category) => HttpResponse::Ok().json(CategoryResponse::from(category)),
        Err(e) => error_response(e),
    }
}

/// DELETE /api/v1/kb/categories/{category_id}
/// Elimina categoría (admin)
pub async fn delete_category(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let category_id = path.into_inner();

    match service.delete_category(category_id).await {
        Ok(()) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            message: "Category deleted".to_string(),
        }),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// ARTICLES
// =============================================================================

/// GET /api/v1/kb/articles
/// Lista artículos con filtros
pub async fn list_articles(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<ListArticlesQuery>,
) -> HttpResponse {
    let tenant_id = extract_tenant_id(&req);
    let user_role = extract_user_role(&req);

    // Si no es admin, solo ver publicados
    let filters = if user_role.as_deref() != Some("admin") {
        ArticleFilters {
            category_id: query.category_id,
            status: Some(ArticleStatus::Published),
            visibility: Some(ArticleVisibility::Public),
            page: query.page,
            page_size: query.page_size,
            sort_by: query.sort_by.clone(),
            sort_order: query.sort_order.clone(),
            ..Default::default()
        }
    } else {
        ArticleFilters {
            category_id: query.category_id,
            status: query.status.as_ref().and_then(|s| s.parse().ok()),
            visibility: query.visibility.as_ref().and_then(|v| v.parse().ok()),
            author_id: query.author_id,
            is_featured: query.is_featured,
            is_pinned: query.is_pinned,
            page: query.page,
            page_size: query.page_size,
            sort_by: query.sort_by.clone(),
            sort_order: query.sort_order.clone(),
            ..Default::default()
        }
    };

    match service.list_articles(filters, tenant_id).await {
        Ok(result) => HttpResponse::Ok().json(ArticleListResponse::from(result)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/articles/{article_id}
/// Obtiene artículo por ID
pub async fn get_article(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();
    let user_role = extract_user_role(&req);

    match service.get_article(article_id, user_role.as_deref(), true).await {
        Ok(article) => HttpResponse::Ok().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/articles/slug/{slug}
/// Obtiene artículo por slug
pub async fn get_article_by_slug(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let slug = path.into_inner();
    let tenant_id = extract_tenant_id(&req);
    let user_role = extract_user_role(&req);

    match service.get_article_by_slug(&slug, user_role.as_deref(), true, tenant_id).await {
        Ok(article) => HttpResponse::Ok().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/kb/articles
/// Crea artículo (admin/author)
pub async fn create_article(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateArticleBody>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

    let author_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let tenant_id = extract_tenant_id(&req);
    let author_name = extract_user_name(&req);

    let content_type: ContentType = body.content_type.parse().unwrap_or(ContentType::Markdown);
    let visibility: ArticleVisibility = body.visibility.parse().unwrap_or_default();
    let status: ArticleStatus = body.status.parse().unwrap_or(ArticleStatus::Draft);

    let dto = CreateArticleDto {
        title: body.title.clone(),
        content: body.content.clone(),
        content_type,
        excerpt: body.excerpt.clone(),
        category_id: body.category_id,
        tags: body.tags.clone(),
        meta_title: body.meta_title.clone(),
        meta_description: body.meta_description.clone(),
        visibility,
        allowed_roles: body.allowed_roles.clone(),
        status,
    };

    match service.create_article(dto, author_id, author_name.as_deref(), tenant_id).await {
        Ok(article) => HttpResponse::Created().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// PATCH /api/v1/kb/articles/{article_id}
/// Actualiza artículo (admin/author)
pub async fn update_article(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateArticleBody>,
) -> HttpResponse {
    let article_id = path.into_inner();

    let editor_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let dto = UpdateArticleDto {
        title: body.title.clone(),
        content: body.content.clone(),
        content_type: body.content_type.as_ref().and_then(|ct| ct.parse().ok()),
        excerpt: body.excerpt.clone(),
        category_id: body.category_id,
        tags: body.tags.clone(),
        meta_title: body.meta_title.clone(),
        meta_description: body.meta_description.clone(),
        visibility: body.visibility.as_ref().and_then(|v| v.parse().ok()),
        allowed_roles: body.allowed_roles.clone(),
        status: body.status.as_ref().and_then(|s| s.parse().ok()),
        is_featured: body.is_featured,
        is_pinned: body.is_pinned,
    };

    match service.update_article(article_id, dto, editor_id).await {
        Ok(article) => HttpResponse::Ok().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/kb/articles/{article_id}/publish
/// Publica artículo (admin)
pub async fn publish_article(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();

    let editor_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    match service.publish_article(article_id, editor_id).await {
        Ok(article) => HttpResponse::Ok().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/kb/articles/{article_id}/archive
/// Archiva artículo (admin)
pub async fn archive_article(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();

    let editor_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    match service.archive_article(article_id, editor_id).await {
        Ok(article) => HttpResponse::Ok().json(ArticleResponse::from(article)),
        Err(e) => error_response(e),
    }
}

/// DELETE /api/v1/kb/articles/{article_id}
/// Elimina artículo (admin)
pub async fn delete_article(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();

    match service.delete_article(article_id).await {
        Ok(()) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            message: "Article deleted".to_string(),
        }),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/articles/{article_id}/versions
/// Obtiene historial de versiones
pub async fn get_article_versions(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();

    match service.get_article_versions(article_id).await {
        Ok(versions) => {
            let responses: Vec<ArticleVersionResponse> = versions
                .into_iter()
                .map(ArticleVersionResponse::from)
                .collect();
            HttpResponse::Ok().json(responses)
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/kb/articles/{article_id}/related
/// Obtiene artículos relacionados
pub async fn get_related_articles(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let article_id = path.into_inner();

    match service.get_related_articles(article_id, Some(5)).await {
        Ok(related) => {
            let responses: Vec<RelatedArticleResponse> = related
                .into_iter()
                .map(RelatedArticleResponse::from)
                .collect();
            HttpResponse::Ok().json(responses)
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// FEEDBACK
// =============================================================================

/// POST /api/v1/kb/articles/{article_id}/feedback
/// Registra feedback sobre artículo
pub async fn submit_feedback(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<ArticleFeedbackBody>,
) -> HttpResponse {
    let article_id = path.into_inner();
    let user_id = extract_user_id(&req);
    let anonymous_id = req.headers()
        .get("X-Anonymous-Id")
        .and_then(|v| v.to_str().ok());
    let ip_address = extract_ip(&req);
    let user_agent = extract_user_agent(&req);

    let dto = CreateFeedbackDto {
        is_helpful: body.is_helpful,
        comment: body.comment.clone(),
    };

    match service.submit_feedback(
        article_id,
        user_id,
        anonymous_id,
        dto,
        ip_address.as_deref(),
        user_agent.as_deref(),
    ).await {
        Ok(feedback) => HttpResponse::Created().json(FeedbackResponse::from(feedback)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// SEARCH
// =============================================================================

/// GET /api/v1/kb/search
/// Busca artículos
pub async fn search(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let tenant_id = extract_tenant_id(&req);
    let user_id = extract_user_id(&req);
    let ip_address = extract_ip(&req);
    let user_agent = extract_user_agent(&req);

    match service.search(
        &query.q,
        query.category_id,
        query.page,
        query.page_size,
        user_id,
        ip_address.as_deref(),
        user_agent.as_deref(),
        tenant_id,
    ).await {
        Ok(results) => HttpResponse::Ok().json(SearchResultPageResponse::from(results)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// FAQ
// =============================================================================

/// GET /api/v1/kb/faqs
/// Lista FAQs
pub async fn list_faqs(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<ListFaqsQuery>,
) -> HttpResponse {
    let tenant_id = extract_tenant_id(&req);
    let include_hidden = query.include_hidden.unwrap_or(false);

    match service.list_faqs(query.category_id, include_hidden, tenant_id).await {
        Ok(faqs) => {
            let responses: Vec<FaqResponse> = faqs.into_iter().map(FaqResponse::from).collect();
            HttpResponse::Ok().json(responses)
        }
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/kb/faqs
/// Crea FAQ (admin)
pub async fn create_faq(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateFaqBody>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

    let tenant_id = extract_tenant_id(&req);
    let answer_type: ContentType = body.answer_type.parse().unwrap_or(ContentType::Markdown);

    let dto = CreateFaqDto {
        question: body.question.clone(),
        answer: body.answer.clone(),
        answer_type,
        category_id: body.category_id,
        tags: body.tags.clone(),
        order_index: body.order_index,
    };

    match service.create_faq(dto, tenant_id).await {
        Ok(faq) => HttpResponse::Created().json(FaqResponse::from(faq)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// STATISTICS
// =============================================================================

/// GET /api/v1/kb/stats
/// Obtiene estadísticas (admin)
pub async fn get_stats(
    service: ServiceData,
    req: HttpRequest,
) -> HttpResponse {
    let user_role = extract_user_role(&req).unwrap_or_default();
    if user_role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse {
            code: "FORBIDDEN".to_string(),
            message: "Admin access required".to_string(),
            details: None,
        });
    }

    let tenant_id = extract_tenant_id(&req);

    match service.get_stats(tenant_id).await {
        Ok(stats) => HttpResponse::Ok().json(KbStatsResponse::from(stats)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// HEALTH
// =============================================================================

/// GET /health
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "kb-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// =============================================================================
// HELPERS
// =============================================================================

fn extract_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

fn extract_tenant_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-Tenant-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

fn extract_user_role(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-User-Role")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn extract_user_name(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-User-Name")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn extract_ip(req: &HttpRequest) -> Option<String> {
    req.connection_info()
        .realip_remote_addr()
        .map(String::from)
}

fn extract_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn error_response(err: KbError) -> HttpResponse {
    match &err {
        KbError::NotFound(_) => HttpResponse::NotFound().json(ErrorResponse {
            code: "NOT_FOUND".to_string(),
            message: err.to_string(),
            details: None,
        }),
        KbError::AccessDenied(_) => HttpResponse::Forbidden().json(ErrorResponse {
            code: "ACCESS_DENIED".to_string(),
            message: err.to_string(),
            details: None,
        }),
        KbError::Validation(_) => HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: err.to_string(),
            details: None,
        }),
        KbError::Duplicate(_) => HttpResponse::Conflict().json(ErrorResponse {
            code: "DUPLICATE".to_string(),
            message: err.to_string(),
            details: None,
        }),
        _ => {
            error!("Internal error: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "An unexpected error occurred".to_string(),
                details: None,
            })
        }
    }
}
