// =============================================================================
// ACC LMS - Local Filesystem Storage Backend
// =============================================================================
// Implementación de StorageBackend usando sistema de archivos local
// Ideal para desarrollo y producción de bajo presupuesto
// =============================================================================

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};

use super::backend::{
    StorageBackend, StorageError, StorageResult,
    StoredFileInfo, UploadUrl, DownloadUrl, UploadOptions,
};

/// Configuración para LocalStorage
#[derive(Debug, Clone)]
pub struct LocalStorageConfig {
    /// Directorio base para almacenamiento
    pub base_path: PathBuf,
    /// URL base para servir archivos (ej: http://localhost:8083/files)
    pub base_url: String,
    /// URL del endpoint de upload (ej: http://localhost:8083/api/v1/content/upload)
    pub upload_endpoint: String,
    /// Tamaño máximo de archivo por defecto (bytes)
    pub max_file_size: u64,
}

impl Default for LocalStorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./uploads"),
            base_url: "http://localhost:8083/files".to_string(),
            upload_endpoint: "http://localhost:8083/api/v1/content/upload".to_string(),
            max_file_size: 2 * 1024 * 1024 * 1024, // 2GB
        }
    }
}

/// Backend de almacenamiento en sistema de archivos local
pub struct LocalStorage {
    config: LocalStorageConfig,
}

impl LocalStorage {
    pub fn new(config: LocalStorageConfig) -> StorageResult<Self> {
        // Crear directorio base si no existe (sync para simplificar startup)
        if !config.base_path.exists() {
            std::fs::create_dir_all(&config.base_path)?;
            info!("Created storage directory: {:?}", config.base_path);
        }

        Ok(Self { config })
    }

    /// Construye el path completo para un key
    fn full_path(&self, key: &str) -> PathBuf {
        self.config.base_path.join(key)
    }

    /// Calcula el checksum SHA-256 de datos
    fn calculate_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Asegura que el directorio padre existe
    async fn ensure_parent_dir(&self, path: &Path) -> StorageResult<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
            }
        }
        Ok(())
    }

    /// Obtiene el content-type de un archivo por extensión
    fn guess_content_type(path: &Path) -> String {
        mime_guess::from_path(path)
            .first()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string())
    }
}

#[async_trait]
impl StorageBackend for LocalStorage {
    fn name(&self) -> &str {
        "local-filesystem"
    }

    async fn create_upload_url(
        &self,
        key: &str,
        options: UploadOptions,
        expires_in_secs: u64,
    ) -> StorageResult<UploadUrl> {
        // Para local storage, usamos un endpoint de upload con el key como query param
        // El "token" es simplemente el key codificado (en producción usar HMAC)
        let token = base64_url_encode(key);
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in_secs as i64);

        let url = format!(
            "{}?token={}&expires={}",
            self.config.upload_endpoint,
            token,
            expires_at.timestamp()
        );

        let mut headers = vec![
            ("X-Upload-Key".to_string(), key.to_string()),
        ];

        if let Some(ct) = options.content_type {
            headers.push(("Content-Type".to_string(), ct));
        }

        if let Some(max) = options.max_size {
            headers.push(("X-Max-Size".to_string(), max.to_string()));
        }

        debug!("Created upload URL for key: {}", key);

        Ok(UploadUrl {
            url,
            method: "POST".to_string(),
            headers,
            expires_at,
        })
    }

    async fn create_download_url(
        &self,
        key: &str,
        expires_in_secs: u64,
    ) -> StorageResult<DownloadUrl> {
        // Verificar que el archivo existe
        let path = self.full_path(key);
        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }

        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in_secs as i64);
        let token = base64_url_encode(&format!("{}:{}", key, expires_at.timestamp()));

        let url = format!(
            "{}/{}?token={}",
            self.config.base_url,
            key,
            token
        );

        debug!("Created download URL for key: {}", key);

        Ok(DownloadUrl { url, expires_at })
    }

    async fn upload(
        &self,
        key: &str,
        data: Bytes,
        content_type: &str,
    ) -> StorageResult<StoredFileInfo> {
        let path = self.full_path(key);

        // Verificar tamaño
        if data.len() as u64 > self.config.max_file_size {
            return Err(StorageError::FileTooLarge {
                size: data.len() as u64,
                max: self.config.max_file_size,
            });
        }

        // Crear directorio padre si no existe
        self.ensure_parent_dir(&path).await?;

        // Calcular checksum
        let checksum = Self::calculate_checksum(&data);

        // Escribir archivo
        let mut file = fs::File::create(&path).await?;
        file.write_all(&data).await?;
        file.flush().await?;

        let now = Utc::now();

        info!("Uploaded file: {} ({} bytes)", key, data.len());

        Ok(StoredFileInfo {
            key: key.to_string(),
            size_bytes: data.len() as u64,
            content_type: content_type.to_string(),
            checksum,
            created_at: now,
            modified_at: now,
        })
    }

    async fn download(&self, key: &str) -> StorageResult<Bytes> {
        let path = self.full_path(key);

        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }

        let data = fs::read(&path).await?;
        debug!("Downloaded file: {} ({} bytes)", key, data.len());

        Ok(Bytes::from(data))
    }

    async fn delete(&self, key: &str) -> StorageResult<()> {
        let path = self.full_path(key);

        if !path.exists() {
            warn!("Attempted to delete non-existent file: {}", key);
            return Ok(()); // Idempotente
        }

        fs::remove_file(&path).await?;
        info!("Deleted file: {}", key);

        // Limpiar directorios vacíos
        if let Some(parent) = path.parent() {
            let _ = self.cleanup_empty_dirs(parent).await;
        }

        Ok(())
    }

    async fn get_info(&self, key: &str) -> StorageResult<StoredFileInfo> {
        let path = self.full_path(key);

        if !path.exists() {
            return Err(StorageError::NotFound(key.to_string()));
        }

        let metadata = fs::metadata(&path).await?;
        let data = fs::read(&path).await?;
        let checksum = Self::calculate_checksum(&data);

        let modified = metadata.modified()
            .map(DateTime::<Utc>::from)
            .unwrap_or_else(|_| Utc::now());

        let created = metadata.created()
            .map(DateTime::<Utc>::from)
            .unwrap_or(modified);

        Ok(StoredFileInfo {
            key: key.to_string(),
            size_bytes: metadata.len(),
            content_type: Self::guess_content_type(&path),
            checksum,
            created_at: created,
            modified_at: modified,
        })
    }

    async fn exists(&self, key: &str) -> StorageResult<bool> {
        let path = self.full_path(key);
        Ok(path.exists())
    }

    async fn list(&self, prefix: &str) -> StorageResult<Vec<StoredFileInfo>> {
        let base = self.full_path(prefix);
        let mut results = Vec::new();

        if !base.exists() {
            return Ok(results);
        }

        let mut stack = vec![base];

        while let Some(dir) = stack.pop() {
            let mut entries = fs::read_dir(&dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    stack.push(path);
                } else {
                    let key = path
                        .strip_prefix(&self.config.base_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();

                    let modified = metadata.modified()
                        .map(DateTime::<Utc>::from)
                        .unwrap_or_else(|_| Utc::now());

                    let created = metadata.created()
                        .map(DateTime::<Utc>::from)
                        .unwrap_or(modified);

                    results.push(StoredFileInfo {
                        key,
                        size_bytes: metadata.len(),
                        content_type: Self::guess_content_type(&path),
                        checksum: String::new(), // No calculamos para list (costoso)
                        created_at: created,
                        modified_at: modified,
                    });
                }
            }
        }

        Ok(results)
    }

    async fn copy(&self, source_key: &str, dest_key: &str) -> StorageResult<StoredFileInfo> {
        let source = self.full_path(source_key);
        let dest = self.full_path(dest_key);

        if !source.exists() {
            return Err(StorageError::NotFound(source_key.to_string()));
        }

        self.ensure_parent_dir(&dest).await?;
        fs::copy(&source, &dest).await?;

        info!("Copied file: {} -> {}", source_key, dest_key);
        self.get_info(dest_key).await
    }

    fn base_url(&self) -> &str {
        &self.config.base_url
    }
}

impl LocalStorage {
    /// Limpia directorios vacíos recursivamente hacia arriba
    async fn cleanup_empty_dirs(&self, dir: &Path) -> StorageResult<()> {
        if dir == self.config.base_path {
            return Ok(());
        }

        let mut entries = fs::read_dir(dir).await?;
        if entries.next_entry().await?.is_none() {
            fs::remove_dir(dir).await?;
            if let Some(parent) = dir.parent() {
                Box::pin(self.cleanup_empty_dirs(parent)).await?;
            }
        }

        Ok(())
    }
}

/// Codifica en base64 URL-safe
fn base64_url_encode(input: &str) -> String {
    use base64_light::base64_encode;
    base64_encode(input.as_bytes())
        .replace('+', "-")
        .replace('/', "_")
        .replace('=', "")
}

// Mini implementación de base64 para evitar dependencia extra
mod base64_light {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn base64_encode(input: &[u8]) -> String {
        let mut result = String::new();

        for chunk in input.chunks(3) {
            let b0 = chunk[0] as usize;
            let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
            let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

            result.push(ALPHABET[b0 >> 2] as char);
            result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

            if chunk.len() > 1 {
                result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(ALPHABET[b2 & 0x3f] as char);
            } else {
                result.push('=');
            }
        }

        result
    }
}
