//! # API Handlers
//!
//! Request handlers for certificates endpoints.

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::errors::CertificatesError;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub certificates_service: std::sync::Arc<crate::services::certificates::CertificatesService>,
}

// =============================================================================
// Certificate Handlers
// =============================================================================

/// Generate a new certificate.
pub async fn generate_certificate(
    state: web::Data<AppState>,
    body: web::Json<GenerateCertificateRequest>,
) -> Result<HttpResponse, CertificatesError> {
    body.validate().map_err(|e| CertificatesError::Validation(e.to_string()))?;

    let result = state.certificates_service.generate_certificate(
        body.user_id,
        body.course_id,
        &body.student_name,
        &body.course_title,
        &body.instructor_name,
        body.completion_date,
        body.template_id,
    ).await?;

    Ok(HttpResponse::Created().json(result))
}

/// List certificates for a user.
pub async fn list_user_certificates(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    query: web::Query<ListCertificatesQuery>,
) -> Result<HttpResponse, CertificatesError> {
    let result = state.certificates_service.list_user_certificates(
        *user_id,
        query.page,
        query.per_page,
        query.status.as_deref(),
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Get a specific certificate.
pub async fn get_certificate(
    state: web::Data<AppState>,
    certificate_id: web::Path<Uuid>,
) -> Result<HttpResponse, CertificatesError> {
    let result = state.certificates_service.get_certificate(*certificate_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Download certificate as PDF.
pub async fn download_certificate_pdf(
    state: web::Data<AppState>,
    certificate_id: web::Path<Uuid>,
) -> Result<HttpResponse, CertificatesError> {
    let pdf_data = state.certificates_service.get_certificate_pdf(*certificate_id).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/pdf")
        .insert_header(("Content-Disposition", format!(
            "attachment; filename=\"certificate-{}.pdf\"",
            certificate_id
        )))
        .body(pdf_data))
}

/// Revoke a certificate.
pub async fn revoke_certificate(
    state: web::Data<AppState>,
    certificate_id: web::Path<Uuid>,
    body: web::Json<RevokeCertificateRequest>,
) -> Result<HttpResponse, CertificatesError> {
    state.certificates_service.revoke_certificate(
        *certificate_id,
        body.reason.as_deref(),
    ).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Verify a certificate by verification code (public endpoint).
pub async fn verify_certificate(
    state: web::Data<AppState>,
    verification_code: web::Path<String>,
) -> Result<HttpResponse, CertificatesError> {
    let result = state.certificates_service.verify_certificate(&verification_code).await;
    Ok(HttpResponse::Ok().json(result))
}

/// Get certificate for a specific user and course.
pub async fn get_user_course_certificate(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, CertificatesError> {
    let (user_id, course_id) = path.into_inner();

    let result = state.certificates_service.get_user_course_certificate(
        user_id,
        course_id,
    ).await?;

    match result {
        Some(cert) => Ok(HttpResponse::Ok().json(cert)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "message": "No certificate found for this user and course"
        }))),
    }
}

// =============================================================================
// Template Handlers
// =============================================================================

/// List all templates.
pub async fn list_templates(
    state: web::Data<AppState>,
) -> Result<HttpResponse, CertificatesError> {
    let result = state.certificates_service.list_templates().await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Create a new template.
pub async fn create_template(
    state: web::Data<AppState>,
    body: web::Json<CreateTemplateRequest>,
) -> Result<HttpResponse, CertificatesError> {
    body.validate().map_err(|e| CertificatesError::Validation(e.to_string()))?;

    let result = state.certificates_service.create_template(
        &body.name,
        body.description.as_deref(),
        body.course_id,
        body.background_url.as_deref(),
        body.logo_url.as_deref(),
        &body.primary_color,
        &body.secondary_color,
        &body.font_family,
        body.is_default,
    ).await?;

    Ok(HttpResponse::Created().json(result))
}

/// Get a specific template.
pub async fn get_template(
    state: web::Data<AppState>,
    template_id: web::Path<Uuid>,
) -> Result<HttpResponse, CertificatesError> {
    let result = state.certificates_service.get_template(*template_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Update a template.
pub async fn update_template(
    state: web::Data<AppState>,
    template_id: web::Path<Uuid>,
    body: web::Json<UpdateTemplateRequest>,
) -> Result<HttpResponse, CertificatesError> {
    body.validate().map_err(|e| CertificatesError::Validation(e.to_string()))?;

    let result = state.certificates_service.update_template(
        *template_id,
        body.name.as_deref(),
        body.description.as_deref(),
        body.background_url.as_deref(),
        body.logo_url.as_deref(),
        body.primary_color.as_deref(),
        body.secondary_color.as_deref(),
        body.font_family.as_deref(),
        body.is_default,
        body.is_active,
    ).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Delete a template.
pub async fn delete_template(
    state: web::Data<AppState>,
    template_id: web::Path<Uuid>,
) -> Result<HttpResponse, CertificatesError> {
    state.certificates_service.delete_template(*template_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// =============================================================================
// Health Check
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "certificates-service"
    }))
}
