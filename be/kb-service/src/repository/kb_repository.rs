// =============================================================================
// ACC LMS - Knowledge Base Repository
// =============================================================================
// Capa de persistencia para KB usando PostgreSQL
// =============================================================================

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{
    Article, ArticleFilters, ArticleFeedback, ArticleListResult, ArticleStatus,
    ArticleVersion, ArticleVisibility, Category, CategoryListResult, ContentType,
    CreateArticleDto, CreateCategoryDto, CreateFaqDto, CreateFeedbackDto,
    FaqItem, RelatedArticle, SearchFilters, SearchLog, SearchResult,
    SearchResultPage, TopArticle, UpdateArticleDto, UpdateCategoryDto, UpdateFaqDto,
};

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Duplicate: {0}")]
    Duplicate(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub struct KbRepository {
    pool: PgPool,
}

impl KbRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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
        let category_id = Uuid::new_v4();
        let now = Utc::now();
        let slug = slug::slugify(&dto.name);

        // Calcular path y depth
        let (path, depth) = if let Some(parent_id) = dto.parent_id {
            let parent = self.get_category(parent_id).await?;
            let mut path = parent.path.clone();
            path.push(parent_id);
            (path, parent.depth + 1)
        } else {
            (vec![], 0)
        };

        let path_json = serde_json::to_value(&path).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO kb_categories (
                category_id, tenant_id, name, slug, description,
                icon, color, parent_id, path, depth,
                order_index, is_visible, is_featured,
                article_count, view_count, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, TRUE, FALSE, 0, 0, $12, $12
            )
            "#,
        )
        .bind(category_id)
        .bind(tenant_id)
        .bind(&dto.name)
        .bind(&slug)
        .bind(&dto.description)
        .bind(&dto.icon)
        .bind(&dto.color)
        .bind(dto.parent_id)
        .bind(&path_json)
        .bind(depth)
        .bind(dto.order_index.unwrap_or(0))
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Category {
            category_id,
            tenant_id,
            name: dto.name,
            slug,
            description: dto.description,
            icon: dto.icon,
            color: dto.color,
            parent_id: dto.parent_id,
            path,
            depth,
            order_index: dto.order_index.unwrap_or(0),
            is_visible: true,
            is_featured: false,
            article_count: 0,
            view_count: 0,
            created_at: now,
            updated_at: now,
        })
    }

    /// Obtener categoría por ID
    pub async fn get_category(&self, category_id: Uuid) -> Result<Category> {
        let row = sqlx::query(
            r#"
            SELECT category_id, tenant_id, name, slug, description,
                   icon, color, parent_id, path, depth,
                   order_index, is_visible, is_featured,
                   article_count, view_count, created_at, updated_at
            FROM kb_categories
            WHERE category_id = $1
            "#,
        )
        .bind(category_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Category {} not found", category_id)))?;

        self.row_to_category(&row)
    }

    /// Obtener categoría por slug
    pub async fn get_category_by_slug(&self, slug: &str, tenant_id: Option<Uuid>) -> Result<Category> {
        let row = sqlx::query(
            r#"
            SELECT category_id, tenant_id, name, slug, description,
                   icon, color, parent_id, path, depth,
                   order_index, is_visible, is_featured,
                   article_count, view_count, created_at, updated_at
            FROM kb_categories
            WHERE slug = $1 AND (tenant_id = $2 OR tenant_id IS NULL)
            "#,
        )
        .bind(slug)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Category with slug {} not found", slug)))?;

        self.row_to_category(&row)
    }

    /// Listar categorías
    pub async fn list_categories(
        &self,
        parent_id: Option<Uuid>,
        include_hidden: bool,
        tenant_id: Option<Uuid>,
    ) -> Result<CategoryListResult> {
        let mut query = String::from(
            r#"
            SELECT category_id, tenant_id, name, slug, description,
                   icon, color, parent_id, path, depth,
                   order_index, is_visible, is_featured,
                   article_count, view_count, created_at, updated_at
            FROM kb_categories
            WHERE (tenant_id = $1 OR tenant_id IS NULL)
            "#
        );

        if parent_id.is_some() {
            query.push_str(" AND parent_id = $2");
        } else {
            query.push_str(" AND parent_id IS NULL");
        }

        if !include_hidden {
            query.push_str(" AND is_visible = TRUE");
        }

        query.push_str(" ORDER BY order_index ASC, name ASC");

        let rows = if let Some(pid) = parent_id {
            sqlx::query(&query)
                .bind(tenant_id)
                .bind(pid)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query)
                .bind(tenant_id)
                .fetch_all(&self.pool)
                .await?
        };

        let categories: Vec<Category> = rows
            .iter()
            .filter_map(|row| self.row_to_category(row).ok())
            .collect();

        let total = categories.len() as i64;

        Ok(CategoryListResult { categories, total })
    }

    /// Actualizar categoría
    pub async fn update_category(
        &self,
        category_id: Uuid,
        dto: UpdateCategoryDto,
    ) -> Result<Category> {
        let existing = self.get_category(category_id).await?;
        let now = Utc::now();

        let name = dto.name.unwrap_or(existing.name);
        let slug = slug::slugify(&name);

        sqlx::query(
            r#"
            UPDATE kb_categories SET
                name = $2,
                slug = $3,
                description = COALESCE($4, description),
                icon = COALESCE($5, icon),
                color = COALESCE($6, color),
                parent_id = COALESCE($7, parent_id),
                order_index = COALESCE($8, order_index),
                is_visible = COALESCE($9, is_visible),
                is_featured = COALESCE($10, is_featured),
                updated_at = $11
            WHERE category_id = $1
            "#,
        )
        .bind(category_id)
        .bind(&name)
        .bind(&slug)
        .bind(&dto.description)
        .bind(&dto.icon)
        .bind(&dto.color)
        .bind(dto.parent_id)
        .bind(dto.order_index)
        .bind(dto.is_visible)
        .bind(dto.is_featured)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_category(category_id).await
    }

    /// Eliminar categoría
    pub async fn delete_category(&self, category_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM kb_categories WHERE category_id = $1")
            .bind(category_id)
            .execute(&self.pool)
            .await?;
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
        let article_id = Uuid::new_v4();
        let now = Utc::now();
        let slug = slug::slugify(&dto.title);

        // Renderizar HTML si es markdown
        let rendered_html = if dto.content_type == ContentType::Markdown {
            Some(self.render_markdown(&dto.content))
        } else {
            None
        };

        let tags_json = serde_json::to_value(&dto.tags).unwrap_or_default();
        let meta_keywords_json = serde_json::to_value::<Vec<String>>(vec![]).unwrap_or_default();
        let allowed_roles_json = serde_json::to_value(&dto.allowed_roles).unwrap_or_default();

        let published_at = if dto.status == ArticleStatus::Published {
            Some(now)
        } else {
            None
        };

        sqlx::query(
            r#"
            INSERT INTO kb_articles (
                article_id, tenant_id, title, slug, excerpt,
                content, content_type, rendered_html,
                category_id, tags, meta_title, meta_description, meta_keywords,
                status, is_featured, is_pinned,
                visibility, allowed_roles,
                author_id, author_name, version,
                view_count, helpful_count, not_helpful_count,
                published_at, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, FALSE, FALSE, $15, $16,
                $17, $18, 1, 0, 0, 0, $19, $20, $20
            )
            "#,
        )
        .bind(article_id)
        .bind(tenant_id)
        .bind(&dto.title)
        .bind(&slug)
        .bind(&dto.excerpt)
        .bind(&dto.content)
        .bind(dto.content_type.to_string())
        .bind(&rendered_html)
        .bind(dto.category_id)
        .bind(&tags_json)
        .bind(&dto.meta_title)
        .bind(&dto.meta_description)
        .bind(&meta_keywords_json)
        .bind(dto.status.to_string())
        .bind(dto.visibility.to_string())
        .bind(&allowed_roles_json)
        .bind(author_id)
        .bind(author_name)
        .bind(published_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        // Incrementar contador de categoría si aplica
        if let Some(cat_id) = dto.category_id {
            let _ = sqlx::query(
                "UPDATE kb_categories SET article_count = article_count + 1 WHERE category_id = $1"
            )
            .bind(cat_id)
            .execute(&self.pool)
            .await;
        }

        Ok(Article {
            article_id,
            tenant_id,
            title: dto.title,
            slug,
            excerpt: dto.excerpt,
            content: dto.content,
            content_type: dto.content_type,
            rendered_html,
            category_id: dto.category_id,
            tags: dto.tags,
            meta_title: dto.meta_title,
            meta_description: dto.meta_description,
            meta_keywords: vec![],
            status: dto.status,
            is_featured: false,
            is_pinned: false,
            visibility: dto.visibility,
            allowed_roles: dto.allowed_roles,
            author_id,
            author_name: author_name.map(String::from),
            version: 1,
            previous_version_id: None,
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            published_at,
            created_at: now,
            updated_at: now,
        })
    }

    /// Obtener artículo por ID
    pub async fn get_article(&self, article_id: Uuid) -> Result<Article> {
        let row = sqlx::query(
            r#"
            SELECT article_id, tenant_id, title, slug, excerpt,
                   content, content_type, rendered_html,
                   category_id, tags, meta_title, meta_description, meta_keywords,
                   status, is_featured, is_pinned,
                   visibility, allowed_roles,
                   author_id, author_name, version, previous_version_id,
                   view_count, helpful_count, not_helpful_count,
                   published_at, created_at, updated_at
            FROM kb_articles
            WHERE article_id = $1
            "#,
        )
        .bind(article_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Article {} not found", article_id)))?;

        self.row_to_article(&row)
    }

    /// Obtener artículo por slug
    pub async fn get_article_by_slug(&self, slug: &str, tenant_id: Option<Uuid>) -> Result<Article> {
        let row = sqlx::query(
            r#"
            SELECT article_id, tenant_id, title, slug, excerpt,
                   content, content_type, rendered_html,
                   category_id, tags, meta_title, meta_description, meta_keywords,
                   status, is_featured, is_pinned,
                   visibility, allowed_roles,
                   author_id, author_name, version, previous_version_id,
                   view_count, helpful_count, not_helpful_count,
                   published_at, created_at, updated_at
            FROM kb_articles
            WHERE slug = $1 AND (tenant_id = $2 OR tenant_id IS NULL)
            "#,
        )
        .bind(slug)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Article with slug {} not found", slug)))?;

        self.row_to_article(&row)
    }

    /// Listar artículos con filtros
    pub async fn list_articles(
        &self,
        filters: ArticleFilters,
        tenant_id: Option<Uuid>,
    ) -> Result<ArticleListResult> {
        let page = filters.page.unwrap_or(1).max(1);
        let page_size = filters.page_size.unwrap_or(20).min(100);
        let offset = (page - 1) * page_size;

        let mut conditions = vec!["(tenant_id = $1 OR tenant_id IS NULL)".to_string()];
        let mut param_count = 1;

        if filters.category_id.is_some() {
            param_count += 1;
            conditions.push(format!("category_id = ${}", param_count));
        }
        if filters.status.is_some() {
            param_count += 1;
            conditions.push(format!("status = ${}", param_count));
        }
        if filters.visibility.is_some() {
            param_count += 1;
            conditions.push(format!("visibility = ${}", param_count));
        }
        if filters.author_id.is_some() {
            param_count += 1;
            conditions.push(format!("author_id = ${}", param_count));
        }
        if filters.is_featured.is_some() {
            param_count += 1;
            conditions.push(format!("is_featured = ${}", param_count));
        }
        if filters.is_pinned.is_some() {
            param_count += 1;
            conditions.push(format!("is_pinned = ${}", param_count));
        }

        let sort_by = filters.sort_by.as_deref().unwrap_or("created_at");
        let sort_order = filters.sort_order.as_deref().unwrap_or("DESC");

        let query = format!(
            r#"
            SELECT article_id, tenant_id, title, slug, excerpt,
                   content, content_type, rendered_html,
                   category_id, tags, meta_title, meta_description, meta_keywords,
                   status, is_featured, is_pinned,
                   visibility, allowed_roles,
                   author_id, author_name, version, previous_version_id,
                   view_count, helpful_count, not_helpful_count,
                   published_at, created_at, updated_at
            FROM kb_articles
            WHERE {}
            ORDER BY {} {}
            LIMIT {} OFFSET {}
            "#,
            conditions.join(" AND "),
            sort_by, sort_order,
            page_size, offset
        );

        let count_query = format!(
            "SELECT COUNT(*) FROM kb_articles WHERE {}",
            conditions.join(" AND ")
        );

        // Build queries with bindings
        let mut q = sqlx::query(&query).bind(tenant_id);
        let mut cq = sqlx::query_scalar::<_, i64>(&count_query).bind(tenant_id);

        if let Some(cat_id) = filters.category_id {
            q = q.bind(cat_id);
            cq = cq.bind(cat_id);
        }
        if let Some(status) = filters.status {
            q = q.bind(status.to_string());
            cq = cq.bind(status.to_string());
        }
        if let Some(visibility) = filters.visibility {
            q = q.bind(visibility.to_string());
            cq = cq.bind(visibility.to_string());
        }
        if let Some(author_id) = filters.author_id {
            q = q.bind(author_id);
            cq = cq.bind(author_id);
        }
        if let Some(is_featured) = filters.is_featured {
            q = q.bind(is_featured);
            cq = cq.bind(is_featured);
        }
        if let Some(is_pinned) = filters.is_pinned {
            q = q.bind(is_pinned);
            cq = cq.bind(is_pinned);
        }

        let rows = q.fetch_all(&self.pool).await?;
        let total = cq.fetch_one(&self.pool).await.unwrap_or(0);

        let articles: Vec<Article> = rows
            .iter()
            .filter_map(|row| self.row_to_article(row).ok())
            .collect();

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Ok(ArticleListResult {
            articles,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// Actualizar artículo
    pub async fn update_article(
        &self,
        article_id: Uuid,
        dto: UpdateArticleDto,
        editor_id: Uuid,
    ) -> Result<Article> {
        let existing = self.get_article(article_id).await?;
        let now = Utc::now();

        // Guardar versión anterior
        let _ = self.save_article_version(&existing, editor_id, None).await;

        let content = dto.content.clone().unwrap_or_else(|| existing.content.clone());
        let content_type = dto.content_type.unwrap_or(existing.content_type);

        let rendered_html = if content_type == ContentType::Markdown {
            Some(self.render_markdown(&content))
        } else {
            existing.rendered_html
        };

        let title = dto.title.clone().unwrap_or_else(|| existing.title.clone());
        let slug = slug::slugify(&title);

        let tags = dto.tags.clone().unwrap_or(existing.tags.clone());
        let tags_json = serde_json::to_value(&tags).unwrap_or_default();

        let allowed_roles = dto.allowed_roles.clone().unwrap_or(existing.allowed_roles.clone());
        let allowed_roles_json = serde_json::to_value(&allowed_roles).unwrap_or_default();

        let status = dto.status.unwrap_or(existing.status);
        let published_at = if status == ArticleStatus::Published && existing.published_at.is_none() {
            Some(now)
        } else {
            existing.published_at
        };

        sqlx::query(
            r#"
            UPDATE kb_articles SET
                title = $2,
                slug = $3,
                excerpt = COALESCE($4, excerpt),
                content = $5,
                content_type = $6,
                rendered_html = $7,
                category_id = COALESCE($8, category_id),
                tags = $9,
                meta_title = COALESCE($10, meta_title),
                meta_description = COALESCE($11, meta_description),
                status = $12,
                visibility = COALESCE($13, visibility),
                allowed_roles = $14,
                is_featured = COALESCE($15, is_featured),
                is_pinned = COALESCE($16, is_pinned),
                version = version + 1,
                previous_version_id = $17,
                published_at = $18,
                updated_at = $19
            WHERE article_id = $1
            "#,
        )
        .bind(article_id)
        .bind(&title)
        .bind(&slug)
        .bind(&dto.excerpt)
        .bind(&content)
        .bind(content_type.to_string())
        .bind(&rendered_html)
        .bind(dto.category_id)
        .bind(&tags_json)
        .bind(&dto.meta_title)
        .bind(&dto.meta_description)
        .bind(status.to_string())
        .bind(dto.visibility.map(|v| v.to_string()))
        .bind(&allowed_roles_json)
        .bind(dto.is_featured)
        .bind(dto.is_pinned)
        .bind(existing.article_id) // previous_version_id apunta al mismo artículo
        .bind(published_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_article(article_id).await
    }

    /// Eliminar artículo
    pub async fn delete_article(&self, article_id: Uuid) -> Result<()> {
        let article = self.get_article(article_id).await?;

        sqlx::query("DELETE FROM kb_articles WHERE article_id = $1")
            .bind(article_id)
            .execute(&self.pool)
            .await?;

        // Decrementar contador de categoría
        if let Some(cat_id) = article.category_id {
            let _ = sqlx::query(
                "UPDATE kb_categories SET article_count = article_count - 1 WHERE category_id = $1"
            )
            .bind(cat_id)
            .execute(&self.pool)
            .await;
        }

        Ok(())
    }

    /// Incrementar contador de vistas
    pub async fn increment_view_count(&self, article_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE kb_articles SET view_count = view_count + 1 WHERE article_id = $1"
        )
        .bind(article_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // =========================================================================
    // ARTICLE VERSIONS
    // =========================================================================

    async fn save_article_version(
        &self,
        article: &Article,
        changed_by: Uuid,
        change_summary: Option<&str>,
    ) -> Result<ArticleVersion> {
        let version_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO kb_article_versions (
                version_id, article_id, version_number,
                title, content, content_type,
                change_summary, changed_by, changed_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(version_id)
        .bind(article.article_id)
        .bind(article.version)
        .bind(&article.title)
        .bind(&article.content)
        .bind(article.content_type.to_string())
        .bind(change_summary)
        .bind(changed_by)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(ArticleVersion {
            version_id,
            article_id: article.article_id,
            version_number: article.version,
            title: article.title.clone(),
            content: article.content.clone(),
            content_type: article.content_type,
            change_summary: change_summary.map(String::from),
            changed_by,
            changed_at: now,
        })
    }

    /// Obtener historial de versiones de un artículo
    pub async fn get_article_versions(&self, article_id: Uuid) -> Result<Vec<ArticleVersion>> {
        let rows = sqlx::query(
            r#"
            SELECT version_id, article_id, version_number,
                   title, content, content_type,
                   change_summary, changed_by, changed_at
            FROM kb_article_versions
            WHERE article_id = $1
            ORDER BY version_number DESC
            "#,
        )
        .bind(article_id)
        .fetch_all(&self.pool)
        .await?;

        let versions: Vec<ArticleVersion> = rows
            .iter()
            .map(|row| ArticleVersion {
                version_id: row.get("version_id"),
                article_id: row.get("article_id"),
                version_number: row.get("version_number"),
                title: row.get("title"),
                content: row.get("content"),
                content_type: row.get::<String, _>("content_type").parse().unwrap_or(ContentType::Markdown),
                change_summary: row.get("change_summary"),
                changed_by: row.get("changed_by"),
                changed_at: row.get("changed_at"),
            })
            .collect();

        Ok(versions)
    }

    // =========================================================================
    // FEEDBACK
    // =========================================================================

    /// Registrar feedback
    pub async fn create_feedback(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
        dto: CreateFeedbackDto,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<ArticleFeedback> {
        let feedback_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO kb_article_feedback (
                feedback_id, article_id, user_id, anonymous_id,
                is_helpful, comment, ip_address, user_agent, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(feedback_id)
        .bind(article_id)
        .bind(user_id)
        .bind(anonymous_id)
        .bind(dto.is_helpful)
        .bind(&dto.comment)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .execute(&self.pool)
        .await?;

        // Actualizar contadores en el artículo
        if dto.is_helpful {
            sqlx::query(
                "UPDATE kb_articles SET helpful_count = helpful_count + 1 WHERE article_id = $1"
            )
            .bind(article_id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(
                "UPDATE kb_articles SET not_helpful_count = not_helpful_count + 1 WHERE article_id = $1"
            )
            .bind(article_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(ArticleFeedback {
            feedback_id,
            article_id,
            user_id,
            anonymous_id: anonymous_id.map(String::from),
            is_helpful: dto.is_helpful,
            comment: dto.comment,
            ip_address: ip_address.map(String::from),
            user_agent: user_agent.map(String::from),
            created_at: now,
        })
    }

    // =========================================================================
    // SEARCH
    // =========================================================================

    /// Buscar artículos
    pub async fn search_articles(
        &self,
        filters: SearchFilters,
        tenant_id: Option<Uuid>,
    ) -> Result<SearchResultPage> {
        let page = filters.page.unwrap_or(1).max(1);
        let page_size = filters.page_size.unwrap_or(20).min(100);
        let offset = (page - 1) * page_size;

        // Búsqueda full-text con PostgreSQL
        let query = r#"
            SELECT
                a.article_id, a.title, a.slug, a.excerpt,
                c.name as category_name, a.tags,
                ts_rank(
                    to_tsvector('spanish', a.title || ' ' || a.content),
                    plainto_tsquery('spanish', $1)
                ) as score,
                a.view_count, a.helpful_count, a.published_at
            FROM kb_articles a
            LEFT JOIN kb_categories c ON a.category_id = c.category_id
            WHERE (a.tenant_id = $2 OR a.tenant_id IS NULL)
              AND a.status = 'published'
              AND (
                  to_tsvector('spanish', a.title || ' ' || a.content)
                  @@ plainto_tsquery('spanish', $1)
                  OR a.title ILIKE '%' || $1 || '%'
                  OR a.content ILIKE '%' || $1 || '%'
              )
            ORDER BY score DESC, a.view_count DESC
            LIMIT $3 OFFSET $4
        "#;

        let count_query = r#"
            SELECT COUNT(*)
            FROM kb_articles a
            WHERE (a.tenant_id = $2 OR a.tenant_id IS NULL)
              AND a.status = 'published'
              AND (
                  to_tsvector('spanish', a.title || ' ' || a.content)
                  @@ plainto_tsquery('spanish', $1)
                  OR a.title ILIKE '%' || $1 || '%'
                  OR a.content ILIKE '%' || $1 || '%'
              )
        "#;

        let rows = sqlx::query(query)
            .bind(&filters.query)
            .bind(tenant_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let total = sqlx::query_scalar::<_, i64>(count_query)
            .bind(&filters.query)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let results: Vec<SearchResult> = rows
            .iter()
            .map(|row| {
                let tags_json: serde_json::Value = row.get("tags");
                let tags: Vec<String> = serde_json::from_value(tags_json).unwrap_or_default();

                SearchResult {
                    article_id: row.get("article_id"),
                    title: row.get("title"),
                    slug: row.get("slug"),
                    excerpt: row.get("excerpt"),
                    category_name: row.get("category_name"),
                    tags,
                    score: row.get::<f32, _>("score") as f64,
                    highlights: vec![],
                    view_count: row.get("view_count"),
                    helpful_count: row.get("helpful_count"),
                    published_at: row.get("published_at"),
                }
            })
            .collect();

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Ok(SearchResultPage {
            results,
            total,
            page,
            page_size,
            total_pages,
            query: filters.query,
            suggestions: vec![],
        })
    }

    /// Log de búsqueda
    pub async fn log_search(
        &self,
        query: &str,
        results_count: i32,
        user_id: Option<Uuid>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<()> {
        let log_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO kb_search_logs (
                log_id, tenant_id, user_id, query, results_count,
                ip_address, user_agent, searched_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(log_id)
        .bind(tenant_id)
        .bind(user_id)
        .bind(query)
        .bind(results_count)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
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
        let faq_id = Uuid::new_v4();
        let now = Utc::now();

        let rendered_answer = if dto.answer_type == ContentType::Markdown {
            Some(self.render_markdown(&dto.answer))
        } else {
            None
        };

        let tags_json = serde_json::to_value(&dto.tags).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO kb_faqs (
                faq_id, tenant_id, question, answer, answer_type,
                rendered_answer, category_id, tags, order_index,
                is_visible, is_featured, view_count, helpful_count,
                not_helpful_count, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9,
                TRUE, FALSE, 0, 0, 0, $10, $10
            )
            "#,
        )
        .bind(faq_id)
        .bind(tenant_id)
        .bind(&dto.question)
        .bind(&dto.answer)
        .bind(dto.answer_type.to_string())
        .bind(&rendered_answer)
        .bind(dto.category_id)
        .bind(&tags_json)
        .bind(dto.order_index.unwrap_or(0))
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(FaqItem {
            faq_id,
            tenant_id,
            question: dto.question,
            answer: dto.answer,
            answer_type: dto.answer_type,
            rendered_answer,
            category_id: dto.category_id,
            tags: dto.tags,
            order_index: dto.order_index.unwrap_or(0),
            is_visible: true,
            is_featured: false,
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            created_at: now,
            updated_at: now,
        })
    }

    /// Listar FAQs
    pub async fn list_faqs(
        &self,
        category_id: Option<Uuid>,
        include_hidden: bool,
        tenant_id: Option<Uuid>,
    ) -> Result<Vec<FaqItem>> {
        let mut query = String::from(
            r#"
            SELECT faq_id, tenant_id, question, answer, answer_type,
                   rendered_answer, category_id, tags, order_index,
                   is_visible, is_featured, view_count, helpful_count,
                   not_helpful_count, created_at, updated_at
            FROM kb_faqs
            WHERE (tenant_id = $1 OR tenant_id IS NULL)
            "#
        );

        if category_id.is_some() {
            query.push_str(" AND category_id = $2");
        }

        if !include_hidden {
            query.push_str(" AND is_visible = TRUE");
        }

        query.push_str(" ORDER BY order_index ASC, question ASC");

        let rows = if let Some(cat_id) = category_id {
            sqlx::query(&query)
                .bind(tenant_id)
                .bind(cat_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(&query)
                .bind(tenant_id)
                .fetch_all(&self.pool)
                .await?
        };

        let faqs: Vec<FaqItem> = rows
            .iter()
            .filter_map(|row| self.row_to_faq(row).ok())
            .collect();

        Ok(faqs)
    }

    /// Obtener artículos relacionados
    pub async fn get_related_articles(
        &self,
        article_id: Uuid,
        limit: i32,
    ) -> Result<Vec<RelatedArticle>> {
        let article = self.get_article(article_id).await?;

        // Buscar por categoría y tags similares
        let tags_json = serde_json::to_value(&article.tags).unwrap_or_default();

        let rows = sqlx::query(
            r#"
            SELECT a.article_id, a.title, a.slug, a.excerpt,
                   c.name as category_name,
                   CASE
                       WHEN a.category_id = $2 THEN 0.5
                       ELSE 0
                   END +
                   COALESCE((
                       SELECT COUNT(*) * 0.3
                       FROM jsonb_array_elements_text($3::jsonb) AS t(tag)
                       WHERE a.tags::jsonb @> jsonb_build_array(t.tag)
                   ), 0) as relevance_score
            FROM kb_articles a
            LEFT JOIN kb_categories c ON a.category_id = c.category_id
            WHERE a.article_id != $1
              AND a.status = 'published'
              AND (a.category_id = $2 OR a.tags::jsonb ?| ARRAY(SELECT jsonb_array_elements_text($3::jsonb)))
            ORDER BY relevance_score DESC
            LIMIT $4
            "#,
        )
        .bind(article_id)
        .bind(article.category_id)
        .bind(&tags_json)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let related: Vec<RelatedArticle> = rows
            .iter()
            .map(|row| RelatedArticle {
                article_id: row.get("article_id"),
                title: row.get("title"),
                slug: row.get("slug"),
                excerpt: row.get("excerpt"),
                category_name: row.get("category_name"),
                relevance_score: row.get::<f32, _>("relevance_score") as f64,
            })
            .collect();

        Ok(related)
    }

    // =========================================================================
    // HELPERS
    // =========================================================================

    fn row_to_category(&self, row: &sqlx::postgres::PgRow) -> Result<Category> {
        let path_json: serde_json::Value = row.get("path");
        let path: Vec<Uuid> = serde_json::from_value(path_json).unwrap_or_default();

        Ok(Category {
            category_id: row.get("category_id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            icon: row.get("icon"),
            color: row.get("color"),
            parent_id: row.get("parent_id"),
            path,
            depth: row.get("depth"),
            order_index: row.get("order_index"),
            is_visible: row.get("is_visible"),
            is_featured: row.get("is_featured"),
            article_count: row.get("article_count"),
            view_count: row.get("view_count"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn row_to_article(&self, row: &sqlx::postgres::PgRow) -> Result<Article> {
        let tags_json: serde_json::Value = row.get("tags");
        let tags: Vec<String> = serde_json::from_value(tags_json).unwrap_or_default();

        let meta_keywords_json: serde_json::Value = row.get("meta_keywords");
        let meta_keywords: Vec<String> = serde_json::from_value(meta_keywords_json).unwrap_or_default();

        let allowed_roles_json: serde_json::Value = row.get("allowed_roles");
        let allowed_roles: Vec<String> = serde_json::from_value(allowed_roles_json).unwrap_or_default();

        Ok(Article {
            article_id: row.get("article_id"),
            tenant_id: row.get("tenant_id"),
            title: row.get("title"),
            slug: row.get("slug"),
            excerpt: row.get("excerpt"),
            content: row.get("content"),
            content_type: row.get::<String, _>("content_type").parse().unwrap_or(ContentType::Markdown),
            rendered_html: row.get("rendered_html"),
            category_id: row.get("category_id"),
            tags,
            meta_title: row.get("meta_title"),
            meta_description: row.get("meta_description"),
            meta_keywords,
            status: row.get::<String, _>("status").parse().unwrap_or(ArticleStatus::Draft),
            is_featured: row.get("is_featured"),
            is_pinned: row.get("is_pinned"),
            visibility: row.get::<String, _>("visibility").parse().unwrap_or_default(),
            allowed_roles,
            author_id: row.get("author_id"),
            author_name: row.get("author_name"),
            version: row.get("version"),
            previous_version_id: row.get("previous_version_id"),
            view_count: row.get("view_count"),
            helpful_count: row.get("helpful_count"),
            not_helpful_count: row.get("not_helpful_count"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn row_to_faq(&self, row: &sqlx::postgres::PgRow) -> Result<FaqItem> {
        let tags_json: serde_json::Value = row.get("tags");
        let tags: Vec<String> = serde_json::from_value(tags_json).unwrap_or_default();

        Ok(FaqItem {
            faq_id: row.get("faq_id"),
            tenant_id: row.get("tenant_id"),
            question: row.get("question"),
            answer: row.get("answer"),
            answer_type: row.get::<String, _>("answer_type").parse().unwrap_or(ContentType::Markdown),
            rendered_answer: row.get("rendered_answer"),
            category_id: row.get("category_id"),
            tags,
            order_index: row.get("order_index"),
            is_visible: row.get("is_visible"),
            is_featured: row.get("is_featured"),
            view_count: row.get("view_count"),
            helpful_count: row.get("helpful_count"),
            not_helpful_count: row.get("not_helpful_count"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn render_markdown(&self, content: &str) -> String {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
