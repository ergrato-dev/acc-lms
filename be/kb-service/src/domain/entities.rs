// =============================================================================
// ACC LMS - Knowledge Base Service Domain Entities
// =============================================================================
// Entidades de dominio para la base de conocimiento
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// CATEGORIES (Categorías de artículos)
// =============================================================================

/// Categoría de artículos de KB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub category_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Información básica
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,         // Nombre de ícono (e.g., "book", "help-circle")
    pub color: Option<String>,        // Color hex para UI

    // Jerarquía
    pub parent_id: Option<Uuid>,
    pub path: Vec<Uuid>,              // Ancestros ordenados (para breadcrumbs)
    pub depth: i32,

    // Orden y visibilidad
    pub order_index: i32,
    pub is_visible: bool,
    pub is_featured: bool,

    // Estadísticas
    pub article_count: i32,
    pub view_count: i64,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO para crear categoría
#[derive(Debug, Clone)]
pub struct CreateCategoryDto {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub order_index: Option<i32>,
}

/// DTO para actualizar categoría
#[derive(Debug, Clone, Default)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<Uuid>,
    pub order_index: Option<i32>,
    pub is_visible: Option<bool>,
    pub is_featured: Option<bool>,
}

// =============================================================================
// ARTICLES (Artículos de KB)
// =============================================================================

/// Estado del artículo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArticleStatus {
    Draft,
    InReview,
    Published,
    Archived,
}

impl std::fmt::Display for ArticleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::InReview => write!(f, "in_review"),
            Self::Published => write!(f, "published"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl std::str::FromStr for ArticleStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "in_review" => Ok(Self::InReview),
            "published" => Ok(Self::Published),
            "archived" => Ok(Self::Archived),
            _ => Err(format!("Unknown article status: {}", s)),
        }
    }
}

/// Tipo de contenido del artículo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Markdown,
    Html,
    RichText,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Markdown => write!(f, "markdown"),
            Self::Html => write!(f, "html"),
            Self::RichText => write!(f, "rich_text"),
        }
    }
}

impl std::str::FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            "rich_text" | "richtext" => Ok(Self::RichText),
            _ => Err(format!("Unknown content type: {}", s)),
        }
    }
}

/// Artículo de KB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub article_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Información básica
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,       // Resumen corto para listados

    // Contenido
    pub content: String,               // Contenido principal
    pub content_type: ContentType,
    pub rendered_html: Option<String>, // HTML renderizado (si es markdown)

    // Categorización
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,

    // SEO
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Vec<String>,

    // Estado y visibilidad
    pub status: ArticleStatus,
    pub is_featured: bool,
    pub is_pinned: bool,               // Fijado al inicio

    // Control de acceso
    pub visibility: ArticleVisibility,
    pub allowed_roles: Vec<String>,    // Roles que pueden ver (si visibility = restricted)

    // Autor
    pub author_id: Uuid,
    pub author_name: Option<String>,   // Denormalizado para performance

    // Versioning
    pub version: i32,
    pub previous_version_id: Option<Uuid>,

    // Estadísticas
    pub view_count: i64,
    pub helpful_count: i32,
    pub not_helpful_count: i32,

    // Timestamps
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Visibilidad del artículo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArticleVisibility {
    Public,      // Visible para todos (incluso anónimos)
    Authenticated, // Solo usuarios autenticados
    Restricted,  // Solo roles específicos
    Internal,    // Solo admins/staff
}

impl std::fmt::Display for ArticleVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Authenticated => write!(f, "authenticated"),
            Self::Restricted => write!(f, "restricted"),
            Self::Internal => write!(f, "internal"),
        }
    }
}

impl std::str::FromStr for ArticleVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(Self::Public),
            "authenticated" => Ok(Self::Authenticated),
            "restricted" => Ok(Self::Restricted),
            "internal" => Ok(Self::Internal),
            _ => Err(format!("Unknown visibility: {}", s)),
        }
    }
}

impl Default for ArticleVisibility {
    fn default() -> Self {
        Self::Public
    }
}

/// DTO para crear artículo
#[derive(Debug, Clone)]
pub struct CreateArticleDto {
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub excerpt: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub visibility: ArticleVisibility,
    pub allowed_roles: Vec<String>,
    pub status: ArticleStatus,
}

/// DTO para actualizar artículo
#[derive(Debug, Clone, Default)]
pub struct UpdateArticleDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub content_type: Option<ContentType>,
    pub excerpt: Option<String>,
    pub category_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub visibility: Option<ArticleVisibility>,
    pub allowed_roles: Option<Vec<String>>,
    pub status: Option<ArticleStatus>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
}

// =============================================================================
// ARTICLE VERSIONS (Historial de versiones)
// =============================================================================

/// Versión histórica de un artículo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVersion {
    pub version_id: Uuid,
    pub article_id: Uuid,
    pub version_number: i32,

    // Snapshot del contenido
    pub title: String,
    pub content: String,
    pub content_type: ContentType,

    // Metadata
    pub change_summary: Option<String>,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
}

// =============================================================================
// FEEDBACK (Retroalimentación de artículos)
// =============================================================================

/// Feedback sobre un artículo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleFeedback {
    pub feedback_id: Uuid,
    pub article_id: Uuid,
    pub user_id: Option<Uuid>,
    pub anonymous_id: Option<String>,

    // Feedback
    pub is_helpful: bool,
    pub comment: Option<String>,

    // Contexto
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// DTO para registrar feedback
#[derive(Debug, Clone)]
pub struct CreateFeedbackDto {
    pub is_helpful: bool,
    pub comment: Option<String>,
}

// =============================================================================
// SEARCH (Búsqueda)
// =============================================================================

/// Resultado de búsqueda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub category_name: Option<String>,
    pub tags: Vec<String>,

    // Relevancia
    pub score: f64,
    pub highlights: Vec<String>,      // Fragmentos con match resaltado

    // Metadata
    pub view_count: i64,
    pub helpful_count: i32,
    pub published_at: Option<DateTime<Utc>>,
}

/// Filtros de búsqueda
#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub query: String,
    pub category_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub status: Option<ArticleStatus>,
    pub visibility: Option<ArticleVisibility>,
    pub author_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Resultado paginado de búsqueda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultPage {
    pub results: Vec<SearchResult>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub query: String,
    pub suggestions: Vec<String>,     // Sugerencias de búsqueda
}

// =============================================================================
// ARTICLE LIST (Lista de artículos)
// =============================================================================

/// Filtros para listar artículos
#[derive(Debug, Clone, Default)]
pub struct ArticleFilters {
    pub category_id: Option<Uuid>,
    pub status: Option<ArticleStatus>,
    pub visibility: Option<ArticleVisibility>,
    pub author_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Resultado paginado de artículos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleListResult {
    pub articles: Vec<Article>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Resultado paginado de categorías
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResult {
    pub categories: Vec<Category>,
    pub total: i64,
}

// =============================================================================
// FAQ (Frequently Asked Questions)
// =============================================================================

/// Pregunta frecuente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    pub faq_id: Uuid,
    pub tenant_id: Option<Uuid>,

    pub question: String,
    pub answer: String,
    pub answer_type: ContentType,
    pub rendered_answer: Option<String>,

    // Categorización
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,

    // Orden y visibilidad
    pub order_index: i32,
    pub is_visible: bool,
    pub is_featured: bool,

    // Estadísticas
    pub view_count: i64,
    pub helpful_count: i32,
    pub not_helpful_count: i32,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO para crear FAQ
#[derive(Debug, Clone)]
pub struct CreateFaqDto {
    pub question: String,
    pub answer: String,
    pub answer_type: ContentType,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub order_index: Option<i32>,
}

/// DTO para actualizar FAQ
#[derive(Debug, Clone, Default)]
pub struct UpdateFaqDto {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub answer_type: Option<ContentType>,
    pub category_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub order_index: Option<i32>,
    pub is_visible: Option<bool>,
    pub is_featured: Option<bool>,
}

// =============================================================================
// RELATED ARTICLES (Artículos relacionados)
// =============================================================================

/// Artículo relacionado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedArticle {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub category_name: Option<String>,
    pub relevance_score: f64,
}

// =============================================================================
// STATISTICS
// =============================================================================

/// Estadísticas de KB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbStats {
    pub total_articles: i64,
    pub published_articles: i64,
    pub draft_articles: i64,
    pub total_categories: i64,
    pub total_faqs: i64,

    pub total_views: i64,
    pub total_helpful: i64,
    pub total_not_helpful: i64,
    pub helpfulness_rate: f64,

    pub top_articles: Vec<TopArticle>,
    pub recent_articles: Vec<Article>,

    pub searches_today: i64,
    pub zero_result_searches: i64,
}

/// Artículo top
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArticle {
    pub article_id: Uuid,
    pub title: String,
    pub slug: String,
    pub view_count: i64,
    pub helpful_count: i32,
}

// =============================================================================
// SEARCH LOG (Log de búsquedas)
// =============================================================================

/// Log de búsqueda para analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLog {
    pub log_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub query: String,
    pub results_count: i32,
    pub clicked_article_id: Option<Uuid>,

    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    pub searched_at: DateTime<Utc>,
}
