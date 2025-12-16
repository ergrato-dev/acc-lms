// =============================================================================
// ACC LMS - Content Service Domain Entities
// =============================================================================
// Entidades para gestión de archivos multimedia con MinIO/S3
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// ENUMS
// =============================================================================

/// Tipo de contenido/archivo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Video,
    Image,
    Document,
    Audio,
    Subtitle,
    Attachment,
}

impl ContentType {
    pub fn from_mime(mime: &str) -> Self {
        if mime.starts_with("video/") {
            ContentType::Video
        } else if mime.starts_with("image/") {
            ContentType::Image
        } else if mime.starts_with("audio/") {
            ContentType::Audio
        } else if mime == "text/vtt" || mime == "application/x-subrip" {
            ContentType::Subtitle
        } else if mime.starts_with("application/pdf")
            || mime.starts_with("application/msword")
            || mime.contains("document")
            || mime.contains("presentation")
            || mime.contains("spreadsheet")
        {
            ContentType::Document
        } else {
            ContentType::Attachment
        }
    }

    pub fn allowed_extensions(&self) -> Vec<&'static str> {
        match self {
            ContentType::Video => vec!["mp4", "webm", "mov", "avi", "mkv"],
            ContentType::Image => vec!["jpg", "jpeg", "png", "gif", "webp", "svg"],
            ContentType::Audio => vec!["mp3", "wav", "ogg", "m4a", "flac"],
            ContentType::Document => vec!["pdf", "doc", "docx", "ppt", "pptx", "xls", "xlsx"],
            ContentType::Subtitle => vec!["vtt", "srt"],
            ContentType::Attachment => vec!["zip", "rar", "7z", "tar", "gz"],
        }
    }

    pub fn max_size_bytes(&self) -> u64 {
        match self {
            ContentType::Video => 2 * 1024 * 1024 * 1024,    // 2GB
            ContentType::Image => 10 * 1024 * 1024,          // 10MB
            ContentType::Audio => 100 * 1024 * 1024,         // 100MB
            ContentType::Document => 50 * 1024 * 1024,       // 50MB
            ContentType::Subtitle => 1 * 1024 * 1024,        // 1MB
            ContentType::Attachment => 100 * 1024 * 1024,    // 100MB
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Video => write!(f, "video"),
            ContentType::Image => write!(f, "image"),
            ContentType::Document => write!(f, "document"),
            ContentType::Audio => write!(f, "audio"),
            ContentType::Subtitle => write!(f, "subtitle"),
            ContentType::Attachment => write!(f, "attachment"),
        }
    }
}

/// Estado de procesamiento del contenido
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProcessingStatus {
    Pending,        // Esperando upload
    Uploading,      // Upload en progreso
    Processing,     // Procesando (transcode, thumbnails)
    Ready,          // Listo para usar
    Failed,         // Error en procesamiento
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingStatus::Pending => write!(f, "pending"),
            ProcessingStatus::Uploading => write!(f, "uploading"),
            ProcessingStatus::Processing => write!(f, "processing"),
            ProcessingStatus::Ready => write!(f, "ready"),
            ProcessingStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Calidad de video disponible
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoQuality {
    Low,      // 360p
    Medium,   // 720p
    High,     // 1080p
    Original, // Calidad original
}

impl VideoQuality {
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            VideoQuality::Low => (640, 360),
            VideoQuality::Medium => (1280, 720),
            VideoQuality::High => (1920, 1080),
            VideoQuality::Original => (0, 0), // Variable
        }
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            VideoQuality::Low => "_360p",
            VideoQuality::Medium => "_720p",
            VideoQuality::High => "_1080p",
            VideoQuality::Original => "",
        }
    }
}

// =============================================================================
// ENTIDADES PRINCIPALES
// =============================================================================

/// Asset de contenido (archivo almacenado)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAsset {
    pub asset_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub owner_id: Uuid,

    // Información del archivo
    pub filename: String,
    pub original_filename: String,
    pub content_type: ContentType,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: Option<String>, // SHA-256

    // Storage
    pub bucket: String,
    pub key: String,
    pub storage_class: String,

    // Estado
    pub status: ProcessingStatus,
    pub error_message: Option<String>,

    // Metadata específica por tipo
    pub metadata: ContentMetadata,

    // Asociación
    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,

    // Uso
    pub view_count: i64,
    pub download_count: i64,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Metadata específica según tipo de contenido
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentMetadata {
    // Video/Audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,

    // Video/Image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,

    // Video específico
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_qualities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,

    // Transcripción
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_language: Option<String>,

    // Documento
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,

    // General
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Presigned URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUpload {
    pub asset_id: Uuid,
    pub upload_url: String,
    pub expires_at: DateTime<Utc>,
    pub max_size_bytes: u64,
    pub allowed_content_types: Vec<String>,

    // Para multipart upload
    pub upload_id: Option<String>,
    pub part_size: Option<u64>,
}

/// Presigned URL para descarga
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedDownload {
    pub asset_id: Uuid,
    pub download_url: String,
    pub expires_at: DateTime<Utc>,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
}

/// Variante de video (diferentes calidades)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoVariant {
    pub variant_id: Uuid,
    pub asset_id: Uuid,
    pub quality: String,
    pub width: i32,
    pub height: i32,
    pub bitrate: i32,
    pub key: String,
    pub size_bytes: i64,
    pub status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
}

/// Thumbnail generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub thumbnail_id: Uuid,
    pub asset_id: Uuid,
    pub timestamp_seconds: i32,
    pub width: i32,
    pub height: i32,
    pub key: String,
    pub url: Option<String>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

/// Transcripción de video/audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub transcription_id: Uuid,
    pub asset_id: Uuid,
    pub language: String,
    pub format: String, // vtt, srt, txt
    pub key: String,
    pub status: ProcessingStatus,
    pub word_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// REQUEST/RESPONSE STRUCTS
// =============================================================================

/// Request para crear presigned upload URL
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUploadRequest {
    #[validate(length(min = 1, max = 255))]
    pub filename: String,

    #[validate(length(min = 1, max = 100))]
    pub content_type: String,

    #[validate(range(min = 1, max = 2147483647))] // Max 2GB
    pub size_bytes: i64,

    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,

    #[serde(default)]
    pub metadata: Option<ContentMetadata>,
}

/// Request para confirmar upload completado
#[derive(Debug, Clone, Deserialize)]
pub struct ConfirmUploadRequest {
    pub asset_id: Uuid,
    pub checksum: Option<String>,
    pub metadata: Option<ContentMetadata>,
}

/// Request para procesar contenido
#[derive(Debug, Clone, Deserialize)]
pub struct ProcessContentRequest {
    pub asset_id: Uuid,
    #[serde(default)]
    pub operations: Vec<ProcessingOperation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProcessingOperation {
    Transcode { qualities: Vec<String> },
    GenerateThumbnail { timestamps: Option<Vec<i32>> },
    Transcribe { language: Option<String> },
    ExtractMetadata,
}

/// Filtros para listar assets
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssetFilters {
    pub owner_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,
    pub content_type: Option<ContentType>,
    pub status: Option<ProcessingStatus>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Respuesta paginada de assets
#[derive(Debug, Clone, Serialize)]
pub struct AssetListResponse {
    pub assets: Vec<ContentAsset>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Estadísticas de uso de storage
#[derive(Debug, Clone, Serialize)]
pub struct StorageStats {
    pub total_assets: i64,
    pub total_size_bytes: i64,
    pub by_type: Vec<TypeStats>,
    pub by_status: Vec<StatusStats>,
    pub limit_bytes: Option<i64>,
    pub usage_percent: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeStats {
    pub content_type: String,
    pub count: i64,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusStats {
    pub status: String,
    pub count: i64,
}

// =============================================================================
// VERIFICACIÓN DE ACCESO
// =============================================================================

/// Request para verificar acceso a contenido
#[derive(Debug, Clone, Deserialize)]
pub struct AccessCheckRequest {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub action: AccessAction,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessAction {
    View,
    Download,
    Edit,
    Delete,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessCheckResponse {
    pub allowed: bool,
    pub reason: Option<String>,
}

// =============================================================================
// WEBHOOK/EVENTOS
// =============================================================================

/// Evento de contenido para notificaciones
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum ContentEvent {
    AssetCreated {
        asset_id: Uuid,
        owner_id: Uuid,
        content_type: ContentType,
    },
    UploadCompleted {
        asset_id: Uuid,
        size_bytes: i64,
        checksum: String,
    },
    ProcessingStarted {
        asset_id: Uuid,
        operations: Vec<String>,
    },
    ProcessingCompleted {
        asset_id: Uuid,
        duration_ms: i64,
    },
    ProcessingFailed {
        asset_id: Uuid,
        error: String,
    },
    AssetDeleted {
        asset_id: Uuid,
        owner_id: Uuid,
    },
}
