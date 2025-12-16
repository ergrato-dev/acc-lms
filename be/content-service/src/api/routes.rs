// =============================================================================
// ACC LMS - Content Service Routes
// =============================================================================
// Configuración de rutas HTTP para el servicio de contenido
// =============================================================================

use actix_web::web;
use crate::api::handlers;

/// Configura todas las rutas del servicio de contenido
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(handlers::health_check))

        // API v1
        .service(
            web::scope("/api/v1/content")
                // Upload endpoints
                .route("/presign-upload", web::post().to(handlers::create_upload_url))
                .route("/upload", web::post().to(handlers::direct_upload))

                // Asset management
                .route("/assets", web::get().to(handlers::list_assets))
                .route("/assets/{asset_id}", web::get().to(handlers::get_asset))
                .route("/assets/{asset_id}/confirm", web::post().to(handlers::confirm_upload))
                .route("/assets/{asset_id}/metadata", web::patch().to(handlers::update_metadata))
                .route("/assets/{asset_id}", web::delete().to(handlers::delete_asset))
                .route("/assets/{asset_id}/stream", web::get().to(handlers::get_stream_url))

                // Download with key (human-readable)
                .route("/download/{key:.*}", web::get().to(handlers::create_download_url))

                // Course/Lesson assets
                .route("/courses/{course_id}/assets", web::get().to(handlers::get_course_assets))
                .route("/lessons/{lesson_id}/assets", web::get().to(handlers::get_lesson_assets))

                // Stats
                .route("/stats", web::get().to(handlers::get_storage_stats))
        );
}

/// Rutas públicas (sin autenticación)
pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get().to(handlers::health_check))
        .route("/ready", web::get().to(health_ready));
}

/// GET /ready - Readiness probe
async fn health_ready() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "service": "content-service"
    }))
}
