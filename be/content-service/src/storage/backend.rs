// =============================================================================
// ACC LMS - Content Service Storage Backend Trait
// =============================================================================
// Trait abstracto para backends de almacenamiento (Local, S3, MinIO, etc.)
// =============================================================================

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("File not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Storage full")]
    StorageFull,

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

/// Resultado de operación de storage
pub type StorageResult<T> = Result<T, StorageError>;

/// Metadata de un archivo almacenado
#[derive(Debug, Clone)]
pub struct StoredFileInfo {
    pub key: String,
    pub size_bytes: u64,
    pub content_type: String,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

/// URL para upload directo
#[derive(Debug, Clone)]
pub struct UploadUrl {
    pub url: String,
    pub method: String,  // PUT o POST
    pub headers: Vec<(String, String)>,
    pub expires_at: DateTime<Utc>,
}

/// URL para descarga
#[derive(Debug, Clone)]
pub struct DownloadUrl {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

/// Opciones para upload
#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    pub content_type: Option<String>,
    pub max_size: Option<u64>,
    pub metadata: Option<Vec<(String, String)>>,
}

/// Trait para backends de almacenamiento
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Nombre del backend (para logging)
    fn name(&self) -> &str;

    /// Genera una URL para upload directo (presigned URL o endpoint local)
    async fn create_upload_url(
        &self,
        key: &str,
        options: UploadOptions,
        expires_in_secs: u64,
    ) -> StorageResult<UploadUrl>;

    /// Genera una URL para descarga
    async fn create_download_url(
        &self,
        key: &str,
        expires_in_secs: u64,
    ) -> StorageResult<DownloadUrl>;

    /// Sube un archivo directamente (para archivos pequeños)
    async fn upload(
        &self,
        key: &str,
        data: Bytes,
        content_type: &str,
    ) -> StorageResult<StoredFileInfo>;

    /// Descarga un archivo
    async fn download(&self, key: &str) -> StorageResult<Bytes>;

    /// Elimina un archivo
    async fn delete(&self, key: &str) -> StorageResult<()>;

    /// Obtiene metadata de un archivo
    async fn get_info(&self, key: &str) -> StorageResult<StoredFileInfo>;

    /// Verifica si un archivo existe
    async fn exists(&self, key: &str) -> StorageResult<bool>;

    /// Lista archivos con un prefijo
    async fn list(&self, prefix: &str) -> StorageResult<Vec<StoredFileInfo>>;

    /// Copia un archivo
    async fn copy(&self, source_key: &str, dest_key: &str) -> StorageResult<StoredFileInfo>;

    /// Mueve un archivo
    async fn rename(&self, source_key: &str, dest_key: &str) -> StorageResult<StoredFileInfo> {
        let info = self.copy(source_key, dest_key).await?;
        self.delete(source_key).await?;
        Ok(info)
    }

    /// Obtiene el path/URL base del storage
    fn base_url(&self) -> &str;

    /// Genera un key único para un archivo
    fn generate_key(&self, prefix: &str, filename: &str) -> String {
        let id = Uuid::new_v4();
        let path = PathBuf::from(filename);
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if ext.is_empty() {
            format!("{}/{}", prefix, id)
        } else {
            format!("{}/{}.{}", prefix, id, ext)
        }
    }
}
