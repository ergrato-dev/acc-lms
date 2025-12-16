// =============================================================================
// ACC LMS - Content Service HTTP Handlers
// =============================================================================
// Handlers para endpoints de la API de contenido
// =============================================================================

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use bytes::BytesMut;
use futures_util::StreamExt;
use sha2::{Sha256, Digest};
use tracing::{info, warn, error};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::{CreateUploadRequest, AssetFilters, ContentType, ProcessingStatus};
use crate::service::{ContentService, ContentError};

type ServiceData = web::Data<std::sync::Arc<ContentService>>;

// =============================================================================
// Upload Handlers
// =============================================================================

/// POST /api/v1/content/presign-upload
/// Genera URL presignada para upload
pub async fn create_upload_url(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateUploadDto>,
) -> HttpResponse {
    // Validar request
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

    // Obtener user_id del header (inyectado por middleware de auth)
    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let tenant_id = extract_tenant_id(&req);

    // Crear request de dominio
    let request = CreateUploadRequest {
        filename: body.filename.clone(),
        content_type: body.content_type.clone(),
        size_bytes: body.size_bytes,
        course_id: body.course_id,
        lesson_id: body.lesson_id,
        metadata: body.metadata.clone().map(|m| m.into()),
    };

    match service.create_upload_url(user_id, request, tenant_id).await {
        Ok(presigned) => {
            HttpResponse::Created().json(UploadUrlResponse {
                asset_id: presigned.asset_id,
                upload_url: presigned.upload_url.clone(),
                expires_at: presigned.expires_at,
                max_size_bytes: presigned.max_size_bytes,
                allowed_content_types: presigned.allowed_content_types,
                instructions: UploadInstructions {
                    method: "POST".to_string(),
                    headers: vec![
                        HeaderInfo {
                            name: "Content-Type".to_string(),
                            value: body.content_type.clone(),
                            required: true,
                        },
                        HeaderInfo {
                            name: "X-Asset-Id".to_string(),
                            value: presigned.asset_id.to_string(),
                            required: true,
                        },
                    ],
                    confirm_endpoint: format!("/api/v1/content/assets/{}/confirm", presigned.asset_id),
                },
            })
        }
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/content/upload
/// Endpoint para upload directo (usado por LocalStorage)
pub async fn direct_upload(
    service: ServiceData,
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    // Obtener asset_id del query param o header
    let asset_id = req.headers()
        .get("X-Asset-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    let asset_id = match asset_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                code: "MISSING_ASSET_ID".to_string(),
                message: "X-Asset-Id header required".to_string(),
                details: None,
            });
        }
    };

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    // Leer el archivo del multipart
    let mut file_data = BytesMut::new();
    let mut hasher = Sha256::new();

    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            hasher.update(&data);
                            file_data.extend_from_slice(&data);
                        }
                        Err(e) => {
                            error!("Error reading upload chunk: {}", e);
                            return HttpResponse::InternalServerError().json(ErrorResponse {
                                code: "UPLOAD_ERROR".to_string(),
                                message: "Failed to read upload data".to_string(),
                                details: None,
                            });
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error processing multipart: {}", e);
                return HttpResponse::BadRequest().json(ErrorResponse {
                    code: "MULTIPART_ERROR".to_string(),
                    message: format!("Failed to process upload: {}", e),
                    details: None,
                });
            }
        }
    }

    if file_data.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "EMPTY_UPLOAD".to_string(),
            message: "No file data received".to_string(),
            details: None,
        });
    }

    let checksum = hex::encode(hasher.finalize());

    info!(
        asset_id = %asset_id,
        size = file_data.len(),
        checksum = %checksum,
        "Direct upload received"
    );

    // Obtener asset para guardar en storage
    let asset = match service.get_asset(asset_id).await {
        Ok(a) => a,
        Err(e) => return error_response(e),
    };

    // Guardar en storage
    // Nota: En producción esto debería usar el storage directamente
    // Por ahora, confirmamos el upload
    match service.confirm_upload(asset_id, user_id, Some(checksum)).await {
        Ok(asset) => HttpResponse::Ok().json(AssetResponse::from(asset)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/content/assets/{asset_id}/confirm
/// Confirma que un upload se completó
pub async fn confirm_upload(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<ConfirmUploadDto>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    match service.confirm_upload(asset_id, user_id, body.checksum.clone()).await {
        Ok(asset) => HttpResponse::Ok().json(AssetResponse::from(asset)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Download Handlers
// =============================================================================

/// GET /api/v1/content/{key}/presign-download
/// Genera URL presignada para descarga
pub async fn create_download_url(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let key = path.into_inner();

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let user_role = extract_user_role(&req).unwrap_or("student".to_string());

    // Obtener asset por key
    let asset = match service.get_asset_by_key(&key).await {
        Ok(a) => a,
        Err(e) => return error_response(e),
    };

    match service.create_download_url(asset.asset_id, user_id, &user_role).await {
        Ok(download) => HttpResponse::Ok().json(DownloadUrlResponse {
            asset_id: download.asset_id,
            download_url: download.download_url,
            expires_at: download.expires_at,
            filename: download.filename,
            content_type: download.content_type,
            size_bytes: download.size_bytes,
        }),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/content/assets/{asset_id}/stream
/// Obtiene URL para streaming de video
pub async fn get_stream_url(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let user_role = extract_user_role(&req).unwrap_or("student".to_string());

    match service.get_stream_url(asset_id, user_id, &user_role).await {
        Ok(download) => HttpResponse::Ok().json(DownloadUrlResponse {
            asset_id: download.asset_id,
            download_url: download.download_url,
            expires_at: download.expires_at,
            filename: download.filename,
            content_type: download.content_type,
            size_bytes: download.size_bytes,
        }),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Asset Management Handlers
// =============================================================================

/// GET /api/v1/content/assets/{asset_id}
/// Obtiene información de un asset
pub async fn get_asset(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    match service.get_asset(asset_id).await {
        Ok(asset) => HttpResponse::Ok().json(AssetResponse::from(asset)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/content/assets
/// Lista assets con filtros
pub async fn list_assets(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<ListAssetsQuery>,
) -> HttpResponse {
    let user_id = extract_user_id(&req);
    let user_role = extract_user_role(&req).unwrap_or("student".to_string());

    // Non-admin solo puede ver sus propios assets
    let owner_filter = if user_role != "admin" {
        user_id
    } else {
        query.owner_id
    };

    let filters = AssetFilters {
        owner_id: owner_filter,
        course_id: query.course_id,
        lesson_id: query.lesson_id,
        content_type: query.content_type.as_ref().and_then(|s| match s.as_str() {
            "video" => Some(ContentType::Video),
            "image" => Some(ContentType::Image),
            "document" => Some(ContentType::Document),
            "audio" => Some(ContentType::Audio),
            "subtitle" => Some(ContentType::Subtitle),
            "attachment" => Some(ContentType::Attachment),
            _ => None,
        }),
        status: query.status.as_ref().and_then(|s| match s.as_str() {
            "pending" => Some(ProcessingStatus::Pending),
            "uploading" => Some(ProcessingStatus::Uploading),
            "processing" => Some(ProcessingStatus::Processing),
            "ready" => Some(ProcessingStatus::Ready),
            "failed" => Some(ProcessingStatus::Failed),
            _ => None,
        }),
        search: query.search.clone(),
        page: query.page,
        page_size: query.page_size,
    };

    match service.list_assets(filters).await {
        Ok(result) => HttpResponse::Ok().json(AssetListDto::from(result)),
        Err(e) => error_response(e),
    }
}

/// PATCH /api/v1/content/assets/{asset_id}/metadata
/// Actualiza metadata de un asset
pub async fn update_metadata(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateMetadataDto>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let metadata = crate::domain::ContentMetadata {
        duration_seconds: body.duration_seconds,
        width: body.width,
        height: body.height,
        description: body.description.clone(),
        tags: body.tags.clone(),
        ..Default::default()
    };

    match service.update_metadata(asset_id, user_id, metadata).await {
        Ok(asset) => HttpResponse::Ok().json(AssetResponse::from(asset)),
        Err(e) => error_response(e),
    }
}

/// DELETE /api/v1/content/assets/{asset_id}
/// Elimina un asset
pub async fn delete_asset(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let user_role = extract_user_role(&req).unwrap_or("student".to_string());

    match service.delete_asset(asset_id, user_id, &user_role).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/content/stats
/// Obtiene estadísticas de storage del usuario
pub async fn get_storage_stats(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<OwnerQuery>,
) -> HttpResponse {
    let user_id = extract_user_id(&req);
    let user_role = extract_user_role(&req).unwrap_or("student".to_string());

    // Admin puede ver stats de cualquier usuario
    let owner_id = if user_role == "admin" && query.owner_id.is_some() {
        query.owner_id.unwrap()
    } else if let Some(id) = user_id {
        id
    } else {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            code: "UNAUTHORIZED".to_string(),
            message: "Authentication required".to_string(),
            details: None,
        });
    };

    match service.get_storage_stats(owner_id).await {
        Ok(stats) => HttpResponse::Ok().json(StorageStatsDto::from(stats)),
        Err(e) => error_response(e),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct OwnerQuery {
    pub owner_id: Option<Uuid>,
}

/// GET /api/v1/content/courses/{course_id}/assets
/// Lista assets de un curso
pub async fn get_course_assets(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let course_id = path.into_inner();

    match service.get_course_assets(course_id).await {
        Ok(assets) => {
            let response: Vec<AssetResponse> = assets.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/content/lessons/{lesson_id}/assets
/// Lista assets de una lección
pub async fn get_lesson_assets(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let lesson_id = path.into_inner();

    match service.get_lesson_assets(lesson_id).await {
        Ok(assets) => {
            let response: Vec<AssetResponse> = assets.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Health Check
// =============================================================================

/// GET /health
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "content-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// =============================================================================
// Helpers
// =============================================================================

fn extract_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

fn extract_tenant_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-Tenant-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

fn extract_user_role(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-User-Role")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

fn error_response(err: ContentError) -> HttpResponse {
    match &err {
        ContentError::NotFound(_) => HttpResponse::NotFound().json(ErrorResponse {
            code: "NOT_FOUND".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ContentError::AccessDenied(_) => HttpResponse::Forbidden().json(ErrorResponse {
            code: "ACCESS_DENIED".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ContentError::InvalidFileType(_) | ContentError::Validation(_) => {
            HttpResponse::BadRequest().json(ErrorResponse {
                code: "VALIDATION_ERROR".to_string(),
                message: err.to_string(),
                details: None,
            })
        }
        ContentError::FileTooLarge { size, max } => HttpResponse::PayloadTooLarge().json(ErrorResponse {
            code: "FILE_TOO_LARGE".to_string(),
            message: err.to_string(),
            details: Some(serde_json::json!({
                "size": size,
                "max": max
            })),
        }),
        _ => {
            error!("Internal error: {}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: "An unexpected error occurred".to_string(),
                details: None,
            })
        }
    }
}
