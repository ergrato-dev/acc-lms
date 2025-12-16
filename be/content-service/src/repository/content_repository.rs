// =============================================================================
// ACC LMS - Content Service Repository
// =============================================================================
// Capa de persistencia para metadata de archivos en PostgreSQL
// Usa queries dinámicas (sin macros) para evitar necesidad de DATABASE_URL
// =============================================================================

use chrono::Utc;
use sqlx::{PgPool, Row};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    ContentAsset, ContentMetadata, ContentType, ProcessingStatus,
    AssetFilters, AssetListResponse, StorageStats, TypeStats, StatusStats,
};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Asset not found: {0}")]
    NotFound(Uuid),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

pub struct ContentRepository {
    pool: PgPool,
}

impl ContentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // CRUD de Assets
    // =========================================================================

    /// Crea un nuevo asset (antes del upload)
    pub async fn create_asset(
        &self,
        owner_id: Uuid,
        filename: &str,
        original_filename: &str,
        content_type: ContentType,
        mime_type: &str,
        size_bytes: i64,
        bucket: &str,
        key: &str,
        course_id: Option<Uuid>,
        lesson_id: Option<Uuid>,
        tenant_id: Option<Uuid>,
    ) -> Result<ContentAsset> {
        let asset_id = Uuid::new_v4();
        let now = Utc::now();
        let metadata = ContentMetadata::default();
        let metadata_json = serde_json::to_value(&metadata)
            .map_err(|e| RepositoryError::Serialization(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO content_assets (
                asset_id, tenant_id, owner_id, filename, original_filename,
                asset_type, content_type, size_bytes, storage_bucket, storage_key,
                storage_backend, status, metadata, course_id, lesson_id,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6::content_type, $7, $8, $9, $10,
                'local', 'pending'::processing_status, $11, $12, $13, $14, $14
            )
            "#,
        )
        .bind(asset_id)
        .bind(tenant_id)
        .bind(owner_id)
        .bind(filename)
        .bind(original_filename)
        .bind(content_type.to_string())
        .bind(mime_type)
        .bind(size_bytes)
        .bind(bucket)
        .bind(key)
        .bind(&metadata_json)
        .bind(course_id)
        .bind(lesson_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(ContentAsset {
            asset_id,
            tenant_id,
            owner_id,
            filename: filename.to_string(),
            original_filename: original_filename.to_string(),
            content_type,
            mime_type: mime_type.to_string(),
            size_bytes,
            checksum: None,
            bucket: bucket.to_string(),
            key: key.to_string(),
            storage_class: "STANDARD".to_string(),
            status: ProcessingStatus::Pending,
            error_message: None,
            metadata,
            course_id,
            lesson_id,
            view_count: 0,
            download_count: 0,
            created_at: now,
            updated_at: now,
            uploaded_at: None,
            processed_at: None,
        })
    }

    /// Obtiene un asset por ID
    pub async fn get_asset(&self, asset_id: Uuid) -> Result<ContentAsset> {
        let row = sqlx::query(
            r#"
            SELECT
                asset_id, tenant_id, owner_id, filename, original_filename,
                asset_type::text, content_type, size_bytes, checksum,
                storage_bucket, storage_key, storage_backend,
                status::text, processing_error, metadata,
                course_id, lesson_id,
                created_at, updated_at, processing_started_at, processing_completed_at
            FROM content_assets
            WHERE asset_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(asset_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(RepositoryError::NotFound(asset_id))?;

        self.row_to_asset(row)
    }

    /// Obtiene un asset por key
    pub async fn get_asset_by_key(&self, key: &str) -> Result<ContentAsset> {
        let row = sqlx::query(
            r#"
            SELECT
                asset_id, tenant_id, owner_id, filename, original_filename,
                asset_type::text, content_type, size_bytes, checksum,
                storage_bucket, storage_key, storage_backend,
                status::text, processing_error, metadata,
                course_id, lesson_id,
                created_at, updated_at, processing_started_at, processing_completed_at
            FROM content_assets
            WHERE storage_key = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(Uuid::nil()))?;

        self.row_to_asset(row)
    }

    /// Convierte una fila de DB a ContentAsset
    fn row_to_asset(&self, row: sqlx::postgres::PgRow) -> Result<ContentAsset> {
        let asset_type_str: String = row.get("asset_type");
        let content_type = match asset_type_str.as_str() {
            "video" => ContentType::Video,
            "image" => ContentType::Image,
            "document" => ContentType::Document,
            "audio" => ContentType::Audio,
            "subtitle" => ContentType::Subtitle,
            _ => ContentType::Attachment,
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "pending" => ProcessingStatus::Pending,
            "uploading" => ProcessingStatus::Uploading,
            "processing" => ProcessingStatus::Processing,
            "ready" => ProcessingStatus::Ready,
            "failed" => ProcessingStatus::Failed,
            _ => ProcessingStatus::Pending,
        };

        let metadata_value: Option<serde_json::Value> = row.get("metadata");
        let metadata: ContentMetadata = metadata_value
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        Ok(ContentAsset {
            asset_id: row.get("asset_id"),
            tenant_id: row.get("tenant_id"),
            owner_id: row.get("owner_id"),
            filename: row.get("filename"),
            original_filename: row.get("original_filename"),
            content_type,
            mime_type: row.get("content_type"),
            size_bytes: row.get("size_bytes"),
            checksum: row.get("checksum"),
            bucket: row.get("storage_bucket"),
            key: row.get("storage_key"),
            storage_class: row.get("storage_backend"),
            status,
            error_message: row.get("processing_error"),
            metadata,
            course_id: row.get("course_id"),
            lesson_id: row.get("lesson_id"),
            view_count: 0,
            download_count: 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            uploaded_at: row.get("processing_started_at"),
            processed_at: row.get("processing_completed_at"),
        })
    }

    /// Actualiza el status de un asset
    pub async fn update_status(
        &self,
        asset_id: Uuid,
        status: ProcessingStatus,
        error_message: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE content_assets
            SET status = $2::processing_status, processing_error = $3, updated_at = $4
            WHERE asset_id = $1
            "#,
        )
        .bind(asset_id)
        .bind(status.to_string())
        .bind(error_message)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Confirma que el upload se completó
    pub async fn confirm_upload(
        &self,
        asset_id: Uuid,
        checksum: &str,
        size_bytes: i64,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE content_assets
            SET
                status = 'ready'::processing_status,
                checksum = $2,
                size_bytes = $3,
                processing_completed_at = $4,
                updated_at = $4
            WHERE asset_id = $1
            "#,
        )
        .bind(asset_id)
        .bind(checksum)
        .bind(size_bytes)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Actualiza metadata de un asset
    pub async fn update_metadata(
        &self,
        asset_id: Uuid,
        metadata: &ContentMetadata,
    ) -> Result<()> {
        let now = Utc::now();
        let metadata_json = serde_json::to_value(metadata)
            .map_err(|e| RepositoryError::Serialization(e.to_string()))?;

        sqlx::query(
            r#"
            UPDATE content_assets
            SET metadata = $2, updated_at = $3
            WHERE asset_id = $1
            "#,
        )
        .bind(asset_id)
        .bind(&metadata_json)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Elimina un asset (soft delete)
    pub async fn delete_asset(&self, asset_id: Uuid) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            "UPDATE content_assets SET deleted_at = $2, status = 'deleted'::processing_status WHERE asset_id = $1"
        )
        .bind(asset_id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Listados y búsquedas
    // =========================================================================

    /// Lista assets con filtros
    pub async fn list_assets(&self, filters: AssetFilters) -> Result<AssetListResponse> {
        let page = filters.page.unwrap_or(1).max(1);
        let page_size = filters.page_size.unwrap_or(20).min(100);
        let offset = ((page - 1) * page_size) as i64;
        let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

        // Count total
        let count_row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM content_assets
            WHERE
                deleted_at IS NULL
                AND ($1::uuid IS NULL OR owner_id = $1)
                AND ($2::uuid IS NULL OR course_id = $2)
                AND ($3::uuid IS NULL OR lesson_id = $3)
                AND ($4::text IS NULL OR asset_type::text = $4)
                AND ($5::text IS NULL OR status::text = $5)
                AND ($6::text IS NULL OR filename ILIKE $6 OR original_filename ILIKE $6)
            "#,
        )
        .bind(filters.owner_id)
        .bind(filters.course_id)
        .bind(filters.lesson_id)
        .bind(filters.content_type.map(|ct| ct.to_string()))
        .bind(filters.status.map(|s| s.to_string()))
        .bind(&search_pattern)
        .fetch_one(&self.pool)
        .await?;

        let total: i64 = count_row.get("count");
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        // Fetch assets
        let rows = sqlx::query(
            r#"
            SELECT
                asset_id, tenant_id, owner_id, filename, original_filename,
                asset_type::text, content_type, size_bytes, checksum,
                storage_bucket, storage_key, storage_backend,
                status::text, processing_error, metadata,
                course_id, lesson_id,
                created_at, updated_at, processing_started_at, processing_completed_at
            FROM content_assets
            WHERE
                deleted_at IS NULL
                AND ($1::uuid IS NULL OR owner_id = $1)
                AND ($2::uuid IS NULL OR course_id = $2)
                AND ($3::uuid IS NULL OR lesson_id = $3)
                AND ($4::text IS NULL OR asset_type::text = $4)
                AND ($5::text IS NULL OR status::text = $5)
                AND ($6::text IS NULL OR filename ILIKE $6 OR original_filename ILIKE $6)
            ORDER BY created_at DESC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(filters.owner_id)
        .bind(filters.course_id)
        .bind(filters.lesson_id)
        .bind(filters.content_type.map(|ct| ct.to_string()))
        .bind(filters.status.map(|s| s.to_string()))
        .bind(&search_pattern)
        .bind(page_size as i64)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut assets = Vec::new();
        for row in rows {
            assets.push(self.row_to_asset(row)?);
        }

        Ok(AssetListResponse {
            assets,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// Obtiene estadísticas de storage para un owner
    pub async fn get_storage_stats(&self, owner_id: Uuid) -> Result<StorageStats> {
        let total = sqlx::query(
            r#"
            SELECT
                COUNT(*) as count,
                COALESCE(SUM(size_bytes), 0) as size
            FROM content_assets
            WHERE owner_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(owner_id)
        .fetch_one(&self.pool)
        .await?;

        let total_count: i64 = total.get("count");
        let total_size: i64 = total.get("size");

        let by_type_rows = sqlx::query(
            r#"
            SELECT
                asset_type::text as asset_type,
                COUNT(*) as count,
                COALESCE(SUM(size_bytes), 0) as size
            FROM content_assets
            WHERE owner_id = $1 AND deleted_at IS NULL
            GROUP BY asset_type
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        let by_type: Vec<TypeStats> = by_type_rows.into_iter().map(|row| {
            TypeStats {
                content_type: row.get("asset_type"),
                count: row.get("count"),
                size_bytes: row.get("size"),
            }
        }).collect();

        let by_status_rows = sqlx::query(
            r#"
            SELECT
                status::text as status,
                COUNT(*) as count
            FROM content_assets
            WHERE owner_id = $1 AND deleted_at IS NULL
            GROUP BY status
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        let by_status: Vec<StatusStats> = by_status_rows.into_iter().map(|row| {
            StatusStats {
                status: row.get("status"),
                count: row.get("count"),
            }
        }).collect();

        Ok(StorageStats {
            total_assets: total_count,
            total_size_bytes: total_size,
            by_type,
            by_status,
            limit_bytes: None,
            usage_percent: None,
        })
    }

    /// Lista assets de un curso
    pub async fn get_course_assets(&self, course_id: Uuid) -> Result<Vec<ContentAsset>> {
        let result = self.list_assets(AssetFilters {
            course_id: Some(course_id),
            ..Default::default()
        }).await?;

        Ok(result.assets)
    }

    /// Lista assets de una lección
    pub async fn get_lesson_assets(&self, lesson_id: Uuid) -> Result<Vec<ContentAsset>> {
        let result = self.list_assets(AssetFilters {
            lesson_id: Some(lesson_id),
            ..Default::default()
        }).await?;

        Ok(result.assets)
    }

    /// Incrementa contador de vistas
    pub async fn increment_view_count(&self, asset_id: Uuid) -> Result<()> {
        // Por ahora no hay columna view_count, ignoramos
        // TODO: Agregar columna cuando se necesite tracking
        Ok(())
    }

    /// Incrementa contador de descargas
    pub async fn increment_download_count(&self, asset_id: Uuid) -> Result<()> {
        // Por ahora no hay columna download_count, ignoramos
        // TODO: Agregar columna cuando se necesite tracking
        Ok(())
    }
}
