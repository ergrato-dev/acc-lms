// =============================================================================
// ACC LMS - Knowledge Base API DTOs
// =============================================================================
// Request/Response DTOs para la API de KB
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    Article, ArticleFeedback, ArticleListResult, ArticleStatus, ArticleVersion,
    ArticleVisibility, Category, CategoryListResult, ContentType, FaqItem,
    KbStats, RelatedArticle, SearchResult, SearchResultPage, TopArticle,
};

// =============================================================================
// Request DTOs
// =============================================================================

/// Request para crear categoría
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryBody {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub order_index: Option<i32>,
}

/// Request para actualizar categoría
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub order_index: Option<i32>,
    pub is_visible: Option<bool>,
    pub is_featured: Option<bool>,
}

/// Request para crear artículo
#[derive(Debug, Deserialize, Validate)]
pub struct CreateArticleBody {
    #[validate(length(min = 1, max = 200))]
    pub title: String,

    #[validate(length(min = 1))]
    pub content: String,

    #[serde(default = "default_content_type")]
    pub content_type: String,

    #[validate(length(max = 300))]
    pub excerpt: Option<String>,

    pub category_id: Option<Uuid>,

    #[serde(default)]
    pub tags: Vec<String>,

    pub meta_title: Option<String>,
    pub meta_description: Option<String>,

    #[serde(default = "default_visibility")]
    pub visibility: String,

    #[serde(default)]
    pub allowed_roles: Vec<String>,

    #[serde(default = "default_status")]
    pub status: String,
}

fn default_content_type() -> String {
    "markdown".to_string()
}

fn default_visibility() -> String {
    "public".to_string()
}

fn default_status() -> String {
    "draft".to_string()
}

/// Request para actualizar artículo
#[derive(Debug, Deserialize)]
pub struct UpdateArticleBody {
    pub title: Option<String>,
    pub content: Option<String>,
    pub content_type: Option<String>,
    pub excerpt: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub visibility: Option<String>,
    pub allowed_roles: Option<Vec<String>>,
    pub status: Option<String>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
}

/// Request para feedback de artículo
#[derive(Debug, Deserialize)]
pub struct ArticleFeedbackBody {
    pub is_helpful: bool,
    pub comment: Option<String>,
}

/// Request para crear FAQ
#[derive(Debug, Deserialize, Validate)]
pub struct CreateFaqBody {
    #[validate(length(min = 1, max = 500))]
    pub question: String,

    #[validate(length(min = 1))]
    pub answer: String,

    #[serde(default = "default_content_type")]
    pub answer_type: String,

    pub category_id: Option<Uuid>,

    #[serde(default)]
    pub tags: Vec<String>,

    pub order_index: Option<i32>,
}

/// Query params para listar artículos
#[derive(Debug, Default, Deserialize)]
pub struct ListArticlesQuery {
    pub category_id: Option<Uuid>,
    pub status: Option<String>,
    pub visibility: Option<String>,
    pub author_id: Option<Uuid>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Query params para búsqueda
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub category_id: Option<Uuid>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Query params para listar categorías
#[derive(Debug, Default, Deserialize)]
pub struct ListCategoriesQuery {
    pub parent_id: Option<Uuid>,
    pub include_hidden: Option<bool>,
}

/// Query params para listar FAQs
#[derive(Debug, Default, Deserialize)]
pub struct ListFaqsQuery {
    pub category_id: Option<Uuid>,
    pub include_hidden: Option<bool>,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// Respuesta de categoría
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub depth: i32,
    pub order_index: i32,
    pub is_visible: bool,
    pub is_featured: bool,
    pub article_count: i32,
    pub view_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Category> for CategoryResponse {
    fn from(c: Category) -> Self {
        Self {
            category_id: c.category_id,
            name: c.name,
            slug: c.slug,
            description: c.description,
            icon: c.icon,
            color: c.color,
            parent_id: c.parent_id,
            depth: c.depth,
            order_index: c.order_index,
            is_visible: c.is_visible,
            is_featured: c.is_featured,
            article_count: c.article_count,
            view_count: c.view_count,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

/// Respuesta de lista de categorías
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryListResponse {
    pub categories: Vec<CategoryResponse>,
    pub total: i64,
}

impl From<CategoryListResult> for CategoryListResponse {
    fn from(r: CategoryListResult) -> Self {
        Self {
            categories: r.categories.into_iter().map(CategoryResponse::from).collect(),
            total: r.total,
        }
    }
}

/// Respuesta de artículo
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub content_type: String,
    pub rendered_html: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub status: String,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub visibility: String,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub version: i32,
    pub view_count: i64,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Article> for ArticleResponse {
    fn from(a: Article) -> Self {
        Self {
            article_id: a.article_id,
            title: a.title,
            slug: a.slug,
            excerpt: a.excerpt,
            content: a.content,
            content_type: a.content_type.to_string(),
            rendered_html: a.rendered_html,
            category_id: a.category_id,
            tags: a.tags,
            meta_title: a.meta_title,
            meta_description: a.meta_description,
            status: a.status.to_string(),
            is_featured: a.is_featured,
            is_pinned: a.is_pinned,
            visibility: a.visibility.to_string(),
            author_id: a.author_id,
            author_name: a.author_name,
            version: a.version,
            view_count: a.view_count,
            helpful_count: a.helpful_count,
            not_helpful_count: a.not_helpful_count,
            published_at: a.published_at,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

/// Respuesta simplificada de artículo (para listas)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleSummaryResponse {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub status: String,
    pub is_featured: bool,
    pub author_name: Option<String>,
    pub view_count: i64,
    pub helpful_count: i32,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<Article> for ArticleSummaryResponse {
    fn from(a: Article) -> Self {
        Self {
            article_id: a.article_id,
            title: a.title,
            slug: a.slug,
            excerpt: a.excerpt,
            category_id: a.category_id,
            tags: a.tags,
            status: a.status.to_string(),
            is_featured: a.is_featured,
            author_name: a.author_name,
            view_count: a.view_count,
            helpful_count: a.helpful_count,
            published_at: a.published_at,
        }
    }
}

/// Respuesta de lista de artículos
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleSummaryResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl From<ArticleListResult> for ArticleListResponse {
    fn from(r: ArticleListResult) -> Self {
        Self {
            articles: r.articles.into_iter().map(ArticleSummaryResponse::from).collect(),
            total: r.total,
            page: r.page,
            page_size: r.page_size,
            total_pages: r.total_pages,
        }
    }
}

/// Respuesta de versión de artículo
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleVersionResponse {
    pub version_id: Uuid,
    pub version_number: i32,
    pub title: String,
    pub content_type: String,
    pub change_summary: Option<String>,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
}

impl From<ArticleVersion> for ArticleVersionResponse {
    fn from(v: ArticleVersion) -> Self {
        Self {
            version_id: v.version_id,
            version_number: v.version_number,
            title: v.title,
            content_type: v.content_type.to_string(),
            change_summary: v.change_summary,
            changed_by: v.changed_by,
            changed_at: v.changed_at,
        }
    }
}

/// Respuesta de feedback
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackResponse {
    pub feedback_id: Uuid,
    pub article_id: Uuid,
    pub is_helpful: bool,
    pub created_at: DateTime<Utc>,
}

impl From<ArticleFeedback> for FeedbackResponse {
    fn from(f: ArticleFeedback) -> Self {
        Self {
            feedback_id: f.feedback_id,
            article_id: f.article_id,
            is_helpful: f.is_helpful,
            created_at: f.created_at,
        }
    }
}

/// Respuesta de resultado de búsqueda
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultResponse {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub category_name: Option<String>,
    pub tags: Vec<String>,
    pub score: f64,
    pub highlights: Vec<String>,
    pub view_count: i64,
    pub helpful_count: i32,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<SearchResult> for SearchResultResponse {
    fn from(r: SearchResult) -> Self {
        Self {
            article_id: r.article_id,
            title: r.title,
            slug: r.slug,
            excerpt: r.excerpt,
            category_name: r.category_name,
            tags: r.tags,
            score: r.score,
            highlights: r.highlights,
            view_count: r.view_count,
            helpful_count: r.helpful_count,
            published_at: r.published_at,
        }
    }
}

/// Respuesta de página de búsqueda
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPageResponse {
    pub results: Vec<SearchResultResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub query: String,
    pub suggestions: Vec<String>,
}

impl From<SearchResultPage> for SearchResultPageResponse {
    fn from(r: SearchResultPage) -> Self {
        Self {
            results: r.results.into_iter().map(SearchResultResponse::from).collect(),
            total: r.total,
            page: r.page,
            page_size: r.page_size,
            total_pages: r.total_pages,
            query: r.query,
            suggestions: r.suggestions,
        }
    }
}

/// Respuesta de FAQ
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FaqResponse {
    pub faq_id: Uuid,
    pub question: String,
    pub answer: String,
    pub answer_type: String,
    pub rendered_answer: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub order_index: i32,
    pub is_visible: bool,
    pub is_featured: bool,
    pub view_count: i64,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
}

impl From<FaqItem> for FaqResponse {
    fn from(f: FaqItem) -> Self {
        Self {
            faq_id: f.faq_id,
            question: f.question,
            answer: f.answer,
            answer_type: f.answer_type.to_string(),
            rendered_answer: f.rendered_answer,
            category_id: f.category_id,
            tags: f.tags,
            order_index: f.order_index,
            is_visible: f.is_visible,
            is_featured: f.is_featured,
            view_count: f.view_count,
            helpful_count: f.helpful_count,
            not_helpful_count: f.not_helpful_count,
        }
    }
}

/// Respuesta de artículo relacionado
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedArticleResponse {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub category_name: Option<String>,
    pub relevance_score: f64,
}

impl From<RelatedArticle> for RelatedArticleResponse {
    fn from(r: RelatedArticle) -> Self {
        Self {
            article_id: r.article_id,
            title: r.title,
            slug: r.slug,
            excerpt: r.excerpt,
            category_name: r.category_name,
            relevance_score: r.relevance_score,
        }
    }
}

/// Respuesta de estadísticas
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KbStatsResponse {
    pub total_articles: i64,
    pub published_articles: i64,
    pub draft_articles: i64,
    pub total_categories: i64,
    pub total_faqs: i64,
    pub total_views: i64,
    pub total_helpful: i64,
    pub total_not_helpful: i64,
    pub helpfulness_rate: f64,
    pub top_articles: Vec<TopArticleResponse>,
    pub searches_today: i64,
    pub zero_result_searches: i64,
}

/// Respuesta de artículo top
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArticleResponse {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub view_count: i64,
    pub helpful_count: i32,
}

impl From<TopArticle> for TopArticleResponse {
    fn from(a: TopArticle) -> Self {
        Self {
            article_id: a.article_id,
            title: a.title,
            slug: a.slug,
            view_count: a.view_count,
            helpful_count: a.helpful_count,
        }
    }
}

impl From<KbStats> for KbStatsResponse {
    fn from(s: KbStats) -> Self {
        Self {
            total_articles: s.total_articles,
            published_articles: s.published_articles,
            draft_articles: s.draft_articles,
            total_categories: s.total_categories,
            total_faqs: s.total_faqs,
            total_views: s.total_views,
            total_helpful: s.total_helpful,
            total_not_helpful: s.total_not_helpful,
            helpfulness_rate: s.helpfulness_rate,
            top_articles: s.top_articles.into_iter().map(TopArticleResponse::from).collect(),
            searches_today: s.searches_today,
            zero_result_searches: s.zero_result_searches,
        }
    }
}

/// Respuesta de error
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Respuesta de éxito
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}
