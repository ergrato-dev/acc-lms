// =============================================================================
// ACC LMS - Content Service DTOs
// =============================================================================
// Data Transfer Objects para la API
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{ContentMetadata, ContentType, ProcessingStatus};

// =============================================================================
// Request DTOs
// =============================================================================

/// Request para crear URL de upload
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUploadDto {
    #[validate(length(min = 1, max = 255, message = "Filename must be 1-255 characters"))]
    pub filename: String,

    #[validate(length(min = 1, max = 100, message = "Content-Type required"))]
    pub content_type: String,

    #[validate(range(min = 1, max = 2147483647, message = "Size must be 1 byte to 2GB"))]
    pub size_bytes: i64,

    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,
    pub metadata: Option<MetadataDto>,
}

/// Request para confirmar upload
#[derive(Debug, Deserialize)]
pub struct ConfirmUploadDto {
    pub checksum: Option<String>,
    pub metadata: Option<MetadataDto>,
}

/// Request para actualizar metadata
#[derive(Debug, Deserialize)]
pub struct UpdateMetadataDto {
    pub duration_seconds: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Filtros para listar assets
#[derive(Debug, Default, Deserialize)]
pub struct ListAssetsQuery {
    pub owner_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,
    pub content_type: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// Respuesta de URL de upload
#[derive(Debug, Serialize)]
pub struct UploadUrlResponse {
    pub asset_id: Uuid,
    pub upload_url: String,
    pub expires_at: DateTime<Utc>,
    pub max_size_bytes: u64,
    pub allowed_content_types: Vec<String>,
    pub instructions: UploadInstructions,
}

#[derive(Debug, Serialize)]
pub struct UploadInstructions {
    pub method: String,
    pub headers: Vec<HeaderInfo>,
    pub confirm_endpoint: String,
}

#[derive(Debug, Serialize)]
pub struct HeaderInfo {
    pub name: String,
    pub value: String,
    pub required: bool,
}

/// Respuesta de URL de descarga
#[derive(Debug, Serialize)]
pub struct DownloadUrlResponse {
    pub asset_id: Uuid,
    pub download_url: String,
    pub expires_at: DateTime<Utc>,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
}

/// Asset completo
#[derive(Debug, Serialize)]
pub struct AssetResponse {
    pub asset_id: Uuid,
    pub owner_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub metadata: MetadataDto,
    pub course_id: Option<Uuid>,
    pub lesson_id: Option<Uuid>,
    pub view_count: i64,
    pub download_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

impl From<crate::domain::ContentAsset> for AssetResponse {
    fn from(asset: crate::domain::ContentAsset) -> Self {
        Self {
            asset_id: asset.asset_id,
            owner_id: asset.owner_id,
            filename: asset.filename,
            original_filename: asset.original_filename,
            content_type: asset.content_type.to_string(),
            mime_type: asset.mime_type,
            size_bytes: asset.size_bytes,
            checksum: asset.checksum,
            status: asset.status.to_string(),
            error_message: asset.error_message,
            metadata: asset.metadata.into(),
            course_id: asset.course_id,
            lesson_id: asset.lesson_id,
            view_count: asset.view_count,
            download_count: asset.download_count,
            created_at: asset.created_at,
            updated_at: asset.updated_at,
            uploaded_at: asset.uploaded_at,
        }
    }
}

/// Metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MetadataDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl From<ContentMetadata> for MetadataDto {
    fn from(m: ContentMetadata) -> Self {
        Self {
            duration_seconds: m.duration_seconds,
            width: m.width,
            height: m.height,
            bitrate: m.bitrate,
            codec: m.codec,
            frame_rate: m.frame_rate,
            thumbnail_url: m.thumbnail_url,
            description: m.description,
            tags: m.tags,
        }
    }
}

impl From<MetadataDto> for ContentMetadata {
    fn from(d: MetadataDto) -> Self {
        Self {
            duration_seconds: d.duration_seconds,
            width: d.width,
            height: d.height,
            bitrate: d.bitrate,
            codec: d.codec,
            frame_rate: d.frame_rate,
            thumbnail_url: d.thumbnail_url,
            description: d.description,
            tags: d.tags,
            ..Default::default()
        }
    }
}

/// Lista paginada de assets
#[derive(Debug, Serialize)]
pub struct AssetListDto {
    pub assets: Vec<AssetResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl From<crate::domain::AssetListResponse> for AssetListDto {
    fn from(r: crate::domain::AssetListResponse) -> Self {
        Self {
            assets: r.assets.into_iter().map(Into::into).collect(),
            total: r.total,
            page: r.page,
            page_size: r.page_size,
            total_pages: r.total_pages,
        }
    }
}

/// Estadísticas de storage
#[derive(Debug, Serialize)]
pub struct StorageStatsDto {
    pub total_assets: i64,
    pub total_size_bytes: i64,
    pub total_size_human: String,
    pub by_type: Vec<TypeStatsDto>,
    pub by_status: Vec<StatusStatsDto>,
    pub limit_bytes: Option<i64>,
    pub usage_percent: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct TypeStatsDto {
    pub content_type: String,
    pub count: i64,
    pub size_bytes: i64,
    pub size_human: String,
}

#[derive(Debug, Serialize)]
pub struct StatusStatsDto {
    pub status: String,
    pub count: i64,
}

impl From<crate::domain::StorageStats> for StorageStatsDto {
    fn from(s: crate::domain::StorageStats) -> Self {
        Self {
            total_assets: s.total_assets,
            total_size_bytes: s.total_size_bytes,
            total_size_human: human_size(s.total_size_bytes),
            by_type: s.by_type.into_iter().map(|t| TypeStatsDto {
                content_type: t.content_type,
                count: t.count,
                size_bytes: t.size_bytes,
                size_human: human_size(t.size_bytes),
            }).collect(),
            by_status: s.by_status.into_iter().map(|t| StatusStatsDto {
                status: t.status,
                count: t.count,
            }).collect(),
            limit_bytes: s.limit_bytes,
            usage_percent: s.usage_percent,
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

// =============================================================================
// Helpers
// =============================================================================

fn human_size(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;
    const TB: i64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
