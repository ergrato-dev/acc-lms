// =============================================================================
// ACC LMS - Knowledge Base Service
// =============================================================================
// Capa de servicio con lógica de negocio para KB
// =============================================================================

use std::sync::Arc;
use tracing::{info, error};
use uuid::Uuid;

use crate::domain::{
    Article, ArticleFilters, ArticleFeedback, ArticleListResult, ArticleStatus,
    ArticleVersion, ArticleVisibility, Category, CategoryListResult, ContentType,
    CreateArticleDto, CreateCategoryDto, CreateFaqDto, CreateFeedbackDto,
    FaqItem, KbStats, RelatedArticle, SearchFilters, SearchResultPage,
    TopArticle, UpdateArticleDto, UpdateCategoryDto,
};
use crate::repository::{KbRepository, RepositoryError};

pub type Result<T> = std::result::Result<T, KbError>;

#[derive(Debug, thiserror::Error)]
pub enum KbError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Duplicate: {0}")]
    Duplicate(String),
}

pub struct KbService {
    repository: Arc<KbRepository>,
}

impl KbService {
    pub fn new(repository: Arc<KbRepository>) -> Self {
        Self { repository }
    }

    // =========================================================================
    // CATEGORIES
    // =========================================================================

    /// Crear categoría
    pub async fn create_category(
        &self,
        dto: CreateCategoryDto,
        tenant_id: Option<Uuid>,
    ) -> Result<Category> {
        // Validar nombre
        if dto.name.trim().is_empty() {
            return Err(KbError::Validation("Category name is required".to_string()));
        }

        // Verificar padre existe si se especifica
        if let Some(parent_id) = dto.parent_id {
            self.repository.get_category(parent_id).await
                .map_err(|_| KbError::NotFound(format!("Parent category {} not found", parent_id)))?;
        }

        let category = self.repository.create_category(dto, tenant_id).await?;
        info!("Created category: {} ({})", category.name, category.category_id);
        Ok(category)
    }

    /// Obtener categoría por ID
    pub async fn get_category(&self, category_id: Uuid) -> Result<Category> {
        self.repository.get_category(category_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })
    }

    /// Obtener categoría por slug
    pub async fn get_category_by_slug(&self, slug: &str, tenant_id: Option<Uuid>) -> Result<Category> {
        self.repository.get_category_by_slug(slug, tenant_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })
    }

    /// Listar categorías
    pub async fn list_categories(
        &self,
        parent_id: Option<Uuid>,
        include_hidden: bool,
        tenant_id: Option<Uuid>,
    ) -> Result<CategoryListResult> {
        Ok(self.repository.list_categories(parent_id, include_hidden, tenant_id).await?)
    }

    /// Actualizar categoría
    pub async fn update_category(
        &self,
        category_id: Uuid,
        dto: UpdateCategoryDto,
    ) -> Result<Category> {
        // Validar nombre si se proporciona
        if let Some(ref name) = dto.name {
            if name.trim().is_empty() {
                return Err(KbError::Validation("Category name cannot be empty".to_string()));
            }
        }

        // Verificar padre existe si se especifica
        if let Some(parent_id) = dto.parent_id {
            if parent_id == category_id {
                return Err(KbError::Validation("Category cannot be its own parent".to_string()));
            }
            self.repository.get_category(parent_id).await
                .map_err(|_| KbError::NotFound(format!("Parent category {} not found", parent_id)))?;
        }

        let category = self.repository.update_category(category_id, dto).await?;
        info!("Updated category: {}", category_id);
        Ok(category)
    }

    /// Eliminar categoría
    pub async fn delete_category(&self, category_id: Uuid) -> Result<()> {
        // Verificar que no tenga artículos
        let category = self.get_category(category_id).await?;
        if category.article_count > 0 {
            return Err(KbError::Validation(
                format!("Cannot delete category with {} articles", category.article_count)
            ));
        }

        // Verificar que no tenga subcategorías
        let children = self.list_categories(Some(category_id), true, category.tenant_id).await?;
        if children.total > 0 {
            return Err(KbError::Validation(
                format!("Cannot delete category with {} subcategories", children.total)
            ));
        }

        self.repository.delete_category(category_id).await?;
        info!("Deleted category: {}", category_id);
        Ok(())
    }

    // =========================================================================
    // ARTICLES
    // =========================================================================

    /// Crear artículo
    pub async fn create_article(
        &self,
        dto: CreateArticleDto,
        author_id: Uuid,
        author_name: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<Article> {
        // Validaciones
        if dto.title.trim().is_empty() {
            return Err(KbError::Validation("Article title is required".to_string()));
        }
        if dto.content.trim().is_empty() {
            return Err(KbError::Validation("Article content is required".to_string()));
        }

        // Verificar categoría existe si se especifica
        if let Some(category_id) = dto.category_id {
            self.repository.get_category(category_id).await
                .map_err(|_| KbError::NotFound(format!("Category {} not found", category_id)))?;
        }

        let article = self.repository.create_article(dto, author_id, author_name, tenant_id).await?;
        info!("Created article: {} ({})", article.title, article.article_id);
        Ok(article)
    }

    /// Obtener artículo por ID
    pub async fn get_article(
        &self,
        article_id: Uuid,
        user_role: Option<&str>,
        increment_views: bool,
    ) -> Result<Article> {
        let article = self.repository.get_article(article_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })?;

        // Verificar acceso según visibilidad
        self.check_article_access(&article, user_role)?;

        // Incrementar vistas si es publicado y visible
        if increment_views && article.status == ArticleStatus::Published {
            let _ = self.repository.increment_view_count(article_id).await;
        }

        Ok(article)
    }

    /// Obtener artículo por slug
    pub async fn get_article_by_slug(
        &self,
        slug: &str,
        user_role: Option<&str>,
        increment_views: bool,
        tenant_id: Option<Uuid>,
    ) -> Result<Article> {
        let article = self.repository.get_article_by_slug(slug, tenant_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })?;

        // Verificar acceso según visibilidad
        self.check_article_access(&article, user_role)?;

        // Incrementar vistas
        if increment_views && article.status == ArticleStatus::Published {
            let _ = self.repository.increment_view_count(article.article_id).await;
        }

        Ok(article)
    }

    /// Verificar acceso a artículo
    fn check_article_access(&self, article: &Article, user_role: Option<&str>) -> Result<()> {
        match article.visibility {
            ArticleVisibility::Public => Ok(()),
            ArticleVisibility::Authenticated => {
                if user_role.is_some() {
                    Ok(())
                } else {
                    Err(KbError::AccessDenied("Authentication required".to_string()))
                }
            }
            ArticleVisibility::Restricted => {
                if let Some(role) = user_role {
                    if article.allowed_roles.contains(&role.to_string()) || role == "admin" {
                        Ok(())
                    } else {
                        Err(KbError::AccessDenied("Insufficient permissions".to_string()))
                    }
                } else {
                    Err(KbError::AccessDenied("Authentication required".to_string()))
                }
            }
            ArticleVisibility::Internal => {
                if user_role == Some("admin") || user_role == Some("staff") {
                    Ok(())
                } else {
                    Err(KbError::AccessDenied("Internal access only".to_string()))
                }
            }
        }
    }

    /// Listar artículos
    pub async fn list_articles(
        &self,
        filters: ArticleFilters,
        tenant_id: Option<Uuid>,
    ) -> Result<ArticleListResult> {
        Ok(self.repository.list_articles(filters, tenant_id).await?)
    }

    /// Listar artículos publicados (para usuarios finales)
    pub async fn list_published_articles(
        &self,
        category_id: Option<Uuid>,
        page: Option<u32>,
        page_size: Option<u32>,
        tenant_id: Option<Uuid>,
    ) -> Result<ArticleListResult> {
        let filters = ArticleFilters {
            category_id,
            status: Some(ArticleStatus::Published),
            visibility: Some(ArticleVisibility::Public),
            page,
            page_size,
            sort_by: Some("published_at".to_string()),
            sort_order: Some("DESC".to_string()),
            ..Default::default()
        };
        Ok(self.repository.list_articles(filters, tenant_id).await?)
    }

    /// Actualizar artículo
    pub async fn update_article(
        &self,
        article_id: Uuid,
        dto: UpdateArticleDto,
        editor_id: Uuid,
    ) -> Result<Article> {
        // Verificar artículo existe
        let existing = self.repository.get_article(article_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })?;

        // Validaciones
        if let Some(ref title) = dto.title {
            if title.trim().is_empty() {
                return Err(KbError::Validation("Article title cannot be empty".to_string()));
            }
        }

        // Verificar categoría si se cambia
        if let Some(category_id) = dto.category_id {
            self.repository.get_category(category_id).await
                .map_err(|_| KbError::NotFound(format!("Category {} not found", category_id)))?;
        }

        let article = self.repository.update_article(article_id, dto, editor_id).await?;
        info!("Updated article: {} by {}", article_id, editor_id);
        Ok(article)
    }

    /// Publicar artículo
    pub async fn publish_article(&self, article_id: Uuid, editor_id: Uuid) -> Result<Article> {
        let dto = UpdateArticleDto {
            status: Some(ArticleStatus::Published),
            ..Default::default()
        };
        self.update_article(article_id, dto, editor_id).await
    }

    /// Archivar artículo
    pub async fn archive_article(&self, article_id: Uuid, editor_id: Uuid) -> Result<Article> {
        let dto = UpdateArticleDto {
            status: Some(ArticleStatus::Archived),
            ..Default::default()
        };
        self.update_article(article_id, dto, editor_id).await
    }

    /// Eliminar artículo
    pub async fn delete_article(&self, article_id: Uuid) -> Result<()> {
        self.repository.delete_article(article_id).await?;
        info!("Deleted article: {}", article_id);
        Ok(())
    }

    /// Obtener historial de versiones
    pub async fn get_article_versions(&self, article_id: Uuid) -> Result<Vec<ArticleVersion>> {
        Ok(self.repository.get_article_versions(article_id).await?)
    }

    // =========================================================================
    // FEEDBACK
    // =========================================================================

    /// Registrar feedback sobre artículo
    pub async fn submit_feedback(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
        dto: CreateFeedbackDto,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<ArticleFeedback> {
        // Verificar artículo existe
        let _ = self.repository.get_article(article_id).await
            .map_err(|e| match e {
                RepositoryError::NotFound(msg) => KbError::NotFound(msg),
                _ => KbError::Repository(e),
            })?;

        let feedback = self.repository.create_feedback(
            article_id,
            user_id,
            anonymous_id,
            dto,
            ip_address,
            user_agent,
        ).await?;

        info!(
            "Feedback submitted for article {}: helpful={}",
            article_id, feedback.is_helpful
        );

        Ok(feedback)
    }

    // =========================================================================
    // SEARCH
    // =========================================================================

    /// Buscar artículos
    pub async fn search(
        &self,
        query: &str,
        category_id: Option<Uuid>,
        page: Option<u32>,
        page_size: Option<u32>,
        user_id: Option<Uuid>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<SearchResultPage> {
        if query.trim().is_empty() {
            return Err(KbError::Validation("Search query is required".to_string()));
        }

        let filters = SearchFilters {
            query: query.to_string(),
            category_id,
            status: Some(ArticleStatus::Published),
            visibility: Some(ArticleVisibility::Public),
            page,
            page_size,
            ..Default::default()
        };

        let results = self.repository.search_articles(filters, tenant_id).await?;

        // Loguear búsqueda para analytics
        let _ = self.repository.log_search(
            query,
            results.total as i32,
            user_id,
            ip_address,
            user_agent,
            tenant_id,
        ).await;

        Ok(results)
    }

    // =========================================================================
    // FAQ
    // =========================================================================

    /// Crear FAQ
    pub async fn create_faq(
        &self,
        dto: CreateFaqDto,
        tenant_id: Option<Uuid>,
    ) -> Result<FaqItem> {
        // Validaciones
        if dto.question.trim().is_empty() {
            return Err(KbError::Validation("Question is required".to_string()));
        }
        if dto.answer.trim().is_empty() {
            return Err(KbError::Validation("Answer is required".to_string()));
        }

        let faq = self.repository.create_faq(dto, tenant_id).await?;
        info!("Created FAQ: {}", faq.faq_id);
        Ok(faq)
    }

    /// Listar FAQs
    pub async fn list_faqs(
        &self,
        category_id: Option<Uuid>,
        include_hidden: bool,
        tenant_id: Option<Uuid>,
    ) -> Result<Vec<FaqItem>> {
        Ok(self.repository.list_faqs(category_id, include_hidden, tenant_id).await?)
    }

    // =========================================================================
    // RELATED ARTICLES
    // =========================================================================

    /// Obtener artículos relacionados
    pub async fn get_related_articles(
        &self,
        article_id: Uuid,
        limit: Option<i32>,
    ) -> Result<Vec<RelatedArticle>> {
        let limit = limit.unwrap_or(5).min(20);
        Ok(self.repository.get_related_articles(article_id, limit).await?)
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /// Obtener estadísticas de KB (admin)
    pub async fn get_stats(&self, tenant_id: Option<Uuid>) -> Result<KbStats> {
        // Obtener conteos básicos
        let published_filters = ArticleFilters {
            status: Some(ArticleStatus::Published),
            ..Default::default()
        };
        let draft_filters = ArticleFilters {
            status: Some(ArticleStatus::Draft),
            ..Default::default()
        };

        let published = self.repository.list_articles(published_filters, tenant_id).await?;
        let drafts = self.repository.list_articles(draft_filters, tenant_id).await?;
        let categories = self.repository.list_categories(None, true, tenant_id).await?;
        let faqs = self.repository.list_faqs(None, true, tenant_id).await?;

        // Artículos recientes
        let recent_filters = ArticleFilters {
            status: Some(ArticleStatus::Published),
            page: Some(1),
            page_size: Some(5),
            sort_by: Some("published_at".to_string()),
            sort_order: Some("DESC".to_string()),
            ..Default::default()
        };
        let recent = self.repository.list_articles(recent_filters, tenant_id).await?;

        // Calcular totales de feedback
        let total_helpful: i32 = published.articles.iter().map(|a| a.helpful_count).sum();
        let total_not_helpful: i32 = published.articles.iter().map(|a| a.not_helpful_count).sum();
        let total_feedback = total_helpful + total_not_helpful;
        let helpfulness_rate = if total_feedback > 0 {
            (total_helpful as f64) / (total_feedback as f64)
        } else {
            0.0
        };

        // Top artículos por vistas
        let mut top_articles: Vec<TopArticle> = published.articles
            .iter()
            .map(|a| TopArticle {
                article_id: a.article_id,
                title: a.title.clone(),
                slug: a.slug.clone(),
                view_count: a.view_count,
                helpful_count: a.helpful_count,
            })
            .collect();
        top_articles.sort_by(|a, b| b.view_count.cmp(&a.view_count));
        top_articles.truncate(10);

        let total_views: i64 = published.articles.iter().map(|a| a.view_count).sum();

        Ok(KbStats {
            total_articles: published.total + drafts.total,
            published_articles: published.total,
            draft_articles: drafts.total,
            total_categories: categories.total,
            total_faqs: faqs.len() as i64,
            total_views,
            total_helpful: total_helpful as i64,
            total_not_helpful: total_not_helpful as i64,
            helpfulness_rate,
            top_articles,
            recent_articles: recent.articles,
            searches_today: 0, // TODO: Implementar con query a search_logs
            zero_result_searches: 0,
        })
    }
}
