// =============================================================================
// ACC LMS - Content Service Business Logic
// =============================================================================
// Lógica de negocio para gestión de contenido multimedia
// =============================================================================

use std::sync::Arc;
use thiserror::Error;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::domain::{
    ContentAsset, ContentMetadata, ContentType, ProcessingStatus,
    CreateUploadRequest, PresignedUpload, PresignedDownload,
    AssetFilters, AssetListResponse, StorageStats,
    AccessCheckRequest, AccessCheckResponse, AccessAction,
};
use crate::repository::{ContentRepository, RepositoryError};
use crate::storage::{StorageBackend, StorageError, UploadOptions};

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Asset not found: {0}")]
    NotFound(Uuid),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<StorageError> for ContentError {
    fn from(err: StorageError) -> Self {
        ContentError::Storage(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ContentError>;

/// Servicio de gestión de contenido
pub struct ContentService {
    repository: Arc<ContentRepository>,
    storage: Arc<dyn StorageBackend>,
}

impl ContentService {
    pub fn new(
        repository: Arc<ContentRepository>,
        storage: Arc<dyn StorageBackend>,
    ) -> Self {
        Self {
            repository,
            storage,
        }
    }

    // =========================================================================
    // Upload Flow
    // =========================================================================

    /// Crea una URL presignada para upload
    pub async fn create_upload_url(
        &self,
        owner_id: Uuid,
        request: CreateUploadRequest,
        tenant_id: Option<Uuid>,
    ) -> Result<PresignedUpload> {
        // Validar request
        self.validate_upload_request(&request)?;

        // Determinar tipo de contenido
        let content_type = ContentType::from_mime(&request.content_type);

        // Validar tamaño
        let max_size = content_type.max_size_bytes();
        if request.size_bytes as u64 > max_size {
            return Err(ContentError::FileTooLarge {
                size: request.size_bytes as u64,
                max: max_size,
            });
        }

        // Generar key único
        let prefix = format!("{}/{}", owner_id, content_type);
        let key = self.storage.generate_key(&prefix, &request.filename);

        // Crear registro en DB (status = pending)
        let asset = self.repository.create_asset(
            owner_id,
            &key.split('/').last().unwrap_or(&request.filename),
            &request.filename,
            content_type,
            &request.content_type,
            request.size_bytes,
            "default",  // bucket name
            &key,
            request.course_id,
            request.lesson_id,
            tenant_id,
        ).await?;

        // Generar URL de upload
        let upload_url = self.storage.create_upload_url(
            &key,
            UploadOptions {
                content_type: Some(request.content_type.clone()),
                max_size: Some(max_size),
                metadata: None,
            },
            3600, // 1 hora
        ).await?;

        info!(
            asset_id = %asset.asset_id,
            owner_id = %owner_id,
            content_type = %content_type,
            size = request.size_bytes,
            "Created upload URL"
        );

        // Obtener tipos de contenido permitidos
        let allowed_types: Vec<String> = content_type.allowed_extensions()
            .iter()
            .map(|ext| {
                mime_guess::from_ext(ext)
                    .first()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("application/{}", ext))
            })
            .collect();

        Ok(PresignedUpload {
            asset_id: asset.asset_id,
            upload_url: upload_url.url,
            expires_at: upload_url.expires_at,
            max_size_bytes: max_size,
            allowed_content_types: allowed_types,
            upload_id: None,
            part_size: None,
        })
    }

    /// Confirma que un upload se completó
    pub async fn confirm_upload(
        &self,
        asset_id: Uuid,
        user_id: Uuid,
        checksum: Option<String>,
    ) -> Result<ContentAsset> {
        // Obtener asset
        let asset = self.repository.get_asset(asset_id).await?;

        // Verificar ownership
        if asset.owner_id != user_id {
            return Err(ContentError::AccessDenied(
                "Not the owner of this asset".to_string()
            ));
        }

        // Verificar que el archivo existe en storage
        let exists = self.storage.exists(&asset.key).await?;
        if !exists {
            warn!(asset_id = %asset_id, "Upload confirmation but file not found");
            return Err(ContentError::NotFound(asset_id));
        }

        // Obtener info del archivo
        let file_info = self.storage.get_info(&asset.key).await?;

        // Actualizar DB
        let final_checksum = checksum.unwrap_or(file_info.checksum);
        self.repository.confirm_upload(
            asset_id,
            &final_checksum,
            file_info.size_bytes as i64,
        ).await?;

        info!(
            asset_id = %asset_id,
            size = file_info.size_bytes,
            "Upload confirmed"
        );

        // Retornar asset actualizado
        self.repository.get_asset(asset_id).await.map_err(Into::into)
    }

    // =========================================================================
    // Download Flow
    // =========================================================================

    /// Crea una URL presignada para descarga
    pub async fn create_download_url(
        &self,
        asset_id: Uuid,
        user_id: Uuid,
        user_role: &str,
    ) -> Result<PresignedDownload> {
        // Obtener asset
        let asset = self.repository.get_asset(asset_id).await?;

        // Verificar acceso
        let access = self.check_access(AccessCheckRequest {
            user_id,
            asset_id,
            action: AccessAction::Download,
        }, user_role).await?;

        if !access.allowed {
            return Err(ContentError::AccessDenied(
                access.reason.unwrap_or_else(|| "Access denied".to_string())
            ));
        }

        // Verificar status
        if asset.status != ProcessingStatus::Ready {
            return Err(ContentError::Validation(format!(
                "Asset is not ready for download (status: {:?})",
                asset.status
            )));
        }

        // Generar URL
        let download_url = self.storage.create_download_url(&asset.key, 3600).await?;

        // Incrementar contador
        let _ = self.repository.increment_download_count(asset_id).await;

        info!(
            asset_id = %asset_id,
            user_id = %user_id,
            "Download URL created"
        );

        Ok(PresignedDownload {
            asset_id: asset.asset_id,
            download_url: download_url.url,
            expires_at: download_url.expires_at,
            filename: asset.original_filename,
            content_type: asset.mime_type,
            size_bytes: asset.size_bytes,
        })
    }

    /// Obtiene URL directa para streaming (video)
    pub async fn get_stream_url(
        &self,
        asset_id: Uuid,
        user_id: Uuid,
        user_role: &str,
    ) -> Result<PresignedDownload> {
        let asset = self.repository.get_asset(asset_id).await?;

        // Solo videos
        if asset.content_type != ContentType::Video {
            return Err(ContentError::Validation(
                "Streaming only available for video assets".to_string()
            ));
        }

        // Verificar acceso
        let access = self.check_access(AccessCheckRequest {
            user_id,
            asset_id,
            action: AccessAction::View,
        }, user_role).await?;

        if !access.allowed {
            return Err(ContentError::AccessDenied(
                access.reason.unwrap_or_else(|| "Access denied".to_string())
            ));
        }

        // Incrementar vistas
        let _ = self.repository.increment_view_count(asset_id).await;

        // URL de streaming (más larga)
        let download_url = self.storage.create_download_url(&asset.key, 14400).await?; // 4 horas

        Ok(PresignedDownload {
            asset_id: asset.asset_id,
            download_url: download_url.url,
            expires_at: download_url.expires_at,
            filename: asset.original_filename,
            content_type: asset.mime_type,
            size_bytes: asset.size_bytes,
        })
    }

    // =========================================================================
    // Asset Management
    // =========================================================================

    /// Obtiene un asset por ID
    pub async fn get_asset(&self, asset_id: Uuid) -> Result<ContentAsset> {
        self.repository.get_asset(asset_id).await.map_err(Into::into)
    }

    /// Obtiene un asset por key
    pub async fn get_asset_by_key(&self, key: &str) -> Result<ContentAsset> {
        self.repository.get_asset_by_key(key).await.map_err(Into::into)
    }

    /// Lista assets con filtros
    pub async fn list_assets(&self, filters: AssetFilters) -> Result<AssetListResponse> {
        self.repository.list_assets(filters).await.map_err(Into::into)
    }

    /// Actualiza metadata de un asset
    pub async fn update_metadata(
        &self,
        asset_id: Uuid,
        user_id: Uuid,
        metadata: ContentMetadata,
    ) -> Result<ContentAsset> {
        let asset = self.repository.get_asset(asset_id).await?;

        if asset.owner_id != user_id {
            return Err(ContentError::AccessDenied(
                "Not the owner of this asset".to_string()
            ));
        }

        self.repository.update_metadata(asset_id, &metadata).await?;
        self.repository.get_asset(asset_id).await.map_err(Into::into)
    }

    /// Elimina un asset
    pub async fn delete_asset(
        &self,
        asset_id: Uuid,
        user_id: Uuid,
        user_role: &str,
    ) -> Result<()> {
        let asset = self.repository.get_asset(asset_id).await?;

        // Solo owner o admin puede eliminar
        if asset.owner_id != user_id && user_role != "admin" {
            return Err(ContentError::AccessDenied(
                "Not authorized to delete this asset".to_string()
            ));
        }

        // Eliminar de storage
        if let Err(e) = self.storage.delete(&asset.key).await {
            warn!(
                asset_id = %asset_id,
                error = %e,
                "Failed to delete from storage, continuing with DB deletion"
            );
        }

        // Eliminar de DB
        self.repository.delete_asset(asset_id).await?;

        info!(
            asset_id = %asset_id,
            deleted_by = %user_id,
            "Asset deleted"
        );

        Ok(())
    }

    /// Obtiene estadísticas de storage
    pub async fn get_storage_stats(&self, owner_id: Uuid) -> Result<StorageStats> {
        self.repository.get_storage_stats(owner_id).await.map_err(Into::into)
    }

    /// Obtiene assets de un curso
    pub async fn get_course_assets(&self, course_id: Uuid) -> Result<Vec<ContentAsset>> {
        self.repository.get_course_assets(course_id).await.map_err(Into::into)
    }

    /// Obtiene assets de una lección
    pub async fn get_lesson_assets(&self, lesson_id: Uuid) -> Result<Vec<ContentAsset>> {
        self.repository.get_lesson_assets(lesson_id).await.map_err(Into::into)
    }

    // =========================================================================
    // Access Control
    // =========================================================================

    /// Verifica acceso a un asset
    pub async fn check_access(
        &self,
        request: AccessCheckRequest,
        user_role: &str,
    ) -> Result<AccessCheckResponse> {
        let asset = self.repository.get_asset(request.asset_id).await?;

        // Admin tiene acceso completo
        if user_role == "admin" {
            return Ok(AccessCheckResponse {
                allowed: true,
                reason: None,
            });
        }

        // Owner tiene acceso completo
        if asset.owner_id == request.user_id {
            return Ok(AccessCheckResponse {
                allowed: true,
                reason: None,
            });
        }

        // Para View/Download, verificar enrollment (simplificado)
        // TODO: Integrar con enrollments-service para verificar matrícula
        match request.action {
            AccessAction::View | AccessAction::Download => {
                // Por ahora, permitir si tiene course_id asociado
                // En producción: llamar a enrollments-service
                if asset.course_id.is_some() {
                    // Asumimos que hay enrollment válido si llegó aquí
                    // El middleware de autenticación ya validó el token
                    Ok(AccessCheckResponse {
                        allowed: true,
                        reason: Some("Enrollment verified".to_string()),
                    })
                } else {
                    Ok(AccessCheckResponse {
                        allowed: false,
                        reason: Some("No enrollment found".to_string()),
                    })
                }
            }
            AccessAction::Edit | AccessAction::Delete => {
                Ok(AccessCheckResponse {
                    allowed: false,
                    reason: Some("Only owner or admin can modify".to_string()),
                })
            }
        }
    }

    // =========================================================================
    // Validaciones
    // =========================================================================

    fn validate_upload_request(&self, request: &CreateUploadRequest) -> Result<()> {
        if request.filename.is_empty() {
            return Err(ContentError::Validation("Filename is required".to_string()));
        }

        if request.filename.len() > 255 {
            return Err(ContentError::Validation("Filename too long".to_string()));
        }

        if request.content_type.is_empty() {
            return Err(ContentError::Validation("Content-Type is required".to_string()));
        }

        if request.size_bytes <= 0 {
            return Err(ContentError::Validation("Invalid file size".to_string()));
        }

        // Validar extensión
        let ext = std::path::PathBuf::from(&request.filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let content_type = ContentType::from_mime(&request.content_type);
        let allowed = content_type.allowed_extensions();

        if !allowed.contains(&ext.as_str()) {
            return Err(ContentError::InvalidFileType(format!(
                "Extension '{}' not allowed for {} (allowed: {:?})",
                ext, content_type, allowed
            )));
        }

        Ok(())
    }
}
