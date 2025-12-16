// =============================================================================
// ACC LMS - Knowledge Base Service Routes
// =============================================================================
// Configuración de rutas para la API de KB
// =============================================================================

use actix_web::web;

use crate::api::handlers;

/// Configura todas las rutas del servicio de KB
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/kb")
            // -----------------------------------------------------------------
            // Categories
            // -----------------------------------------------------------------

            // Listar categorías
            .route("/categories", web::get().to(handlers::list_categories))

            // Crear categoría (admin)
            .route("/categories", web::post().to(handlers::create_category))

            // Obtener categoría por slug
            .route("/categories/slug/{slug}", web::get().to(handlers::get_category_by_slug))

            // Obtener categoría por ID
            .route("/categories/{category_id}", web::get().to(handlers::get_category))

            // Actualizar categoría (admin)
            .route("/categories/{category_id}", web::patch().to(handlers::update_category))

            // Eliminar categoría (admin)
            .route("/categories/{category_id}", web::delete().to(handlers::delete_category))

            // -----------------------------------------------------------------
            // Articles
            // -----------------------------------------------------------------

            // Listar artículos
            .route("/articles", web::get().to(handlers::list_articles))

            // Crear artículo (admin/author)
            .route("/articles", web::post().to(handlers::create_article))

            // Obtener artículo por slug
            .route("/articles/slug/{slug}", web::get().to(handlers::get_article_by_slug))

            // Obtener artículo por ID
            .route("/articles/{article_id}", web::get().to(handlers::get_article))

            // Actualizar artículo (admin/author)
            .route("/articles/{article_id}", web::patch().to(handlers::update_article))

            // Eliminar artículo (admin)
            .route("/articles/{article_id}", web::delete().to(handlers::delete_article))

            // Publicar artículo (admin)
            .route("/articles/{article_id}/publish", web::post().to(handlers::publish_article))

            // Archivar artículo (admin)
            .route("/articles/{article_id}/archive", web::post().to(handlers::archive_article))

            // Historial de versiones
            .route("/articles/{article_id}/versions", web::get().to(handlers::get_article_versions))

            // Artículos relacionados
            .route("/articles/{article_id}/related", web::get().to(handlers::get_related_articles))

            // Feedback de artículo
            .route("/articles/{article_id}/feedback", web::post().to(handlers::submit_feedback))

            // -----------------------------------------------------------------
            // Search
            // -----------------------------------------------------------------

            // Búsqueda de artículos
            .route("/search", web::get().to(handlers::search))

            // -----------------------------------------------------------------
            // FAQ
            // -----------------------------------------------------------------

            // Listar FAQs
            .route("/faqs", web::get().to(handlers::list_faqs))

            // Crear FAQ (admin)
            .route("/faqs", web::post().to(handlers::create_faq))

            // -----------------------------------------------------------------
            // Statistics (admin)
            // -----------------------------------------------------------------

            // Estadísticas de KB
            .route("/stats", web::get().to(handlers::get_stats))
    )
    .route("/health", web::get().to(handlers::health_check));
}
