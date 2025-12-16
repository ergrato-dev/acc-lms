// =============================================================================
// ACC LMS - Compliance Service Routes
// =============================================================================
// Configuración de rutas para la API de compliance
// =============================================================================

use actix_web::web;

use crate::api::handlers;

/// Configura todas las rutas del servicio de compliance
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/compliance")
            // -----------------------------------------------------------------
            // Data Rights Requests (Solicitudes de derechos)
            // -----------------------------------------------------------------

            // Crear solicitud de derechos (ARCO/GDPR/CCPA/LGPD)
            .route("/requests", web::post().to(handlers::create_data_request))

            // Listar solicitudes (admin - con filtros)
            .route("/requests", web::get().to(handlers::list_data_requests))

            // Obtener mis solicitudes (usuario actual)
            .route("/my-requests", web::get().to(handlers::get_my_requests))

            // Obtener solicitud específica
            .route("/requests/{request_id}", web::get().to(handlers::get_data_request))

            // Procesar solicitud (admin)
            .route("/requests/{request_id}/process", web::post().to(handlers::process_request))

            // -----------------------------------------------------------------
            // Cookie Consent (Consentimiento de cookies)
            // -----------------------------------------------------------------

            // Guardar preferencias de cookies
            .route("/consent/cookies", web::post().to(handlers::save_cookie_consent))

            // Obtener preferencias de cookies
            .route("/consent/cookies", web::get().to(handlers::get_cookie_consent))

            // Registrar consentimiento individual
            .route("/consent", web::post().to(handlers::record_consent))

            // -----------------------------------------------------------------
            // Data Export (Portabilidad de datos)
            // -----------------------------------------------------------------

            // Solicitar exportación de datos
            .route("/export", web::post().to(handlers::create_data_export))

            // Obtener estado de exportación
            .route("/export/{export_id}", web::get().to(handlers::get_data_export))

            // -----------------------------------------------------------------
            // Account Deletion (Derecho al olvido)
            // -----------------------------------------------------------------

            // Solicitar eliminación de cuenta
            .route("/delete-account", web::post().to(handlers::request_account_deletion))

            // Cancelar solicitud de eliminación
            .route("/delete-account/{deletion_id}", web::delete().to(handlers::cancel_account_deletion))

            // -----------------------------------------------------------------
            // Statistics & Info
            // -----------------------------------------------------------------

            // Estadísticas de compliance (admin)
            .route("/stats", web::get().to(handlers::get_compliance_stats))

            // Plazos legales por jurisdicción
            .route("/deadlines/{jurisdiction}", web::get().to(handlers::get_legal_deadlines))
    )
    .route("/health", web::get().to(handlers::health_check));
}
