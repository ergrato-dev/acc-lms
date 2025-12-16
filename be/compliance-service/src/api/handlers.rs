// =============================================================================
// ACC LMS - Compliance Service HTTP Handlers
// =============================================================================
// Handlers para endpoints de la API de compliance
// =============================================================================

use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{info, error};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::{
    CreateDataRequestDto, CreateDeletionRequestDto, CreateExportRequestDto,
    DataCategory, DataRequestFilters, DataRightType, ExportFormat,
    Jurisdiction, LegalDeadlines, RequestStatus, SaveCookieConsentDto,
};
use crate::service::{ComplianceService, ComplianceError};

type ServiceData = web::Data<std::sync::Arc<ComplianceService>>;

// =============================================================================
// Data Rights Requests
// =============================================================================

/// POST /api/v1/compliance/requests
/// Crea una nueva solicitud de derechos (ARCO/GDPR/CCPA/LGPD)
pub async fn create_data_request(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateDataRequestBody>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

    let user_id = extract_user_id(&req);
    let tenant_id = extract_tenant_id(&req);
    let ip_address = extract_ip(&req);
    let user_agent = extract_user_agent(&req);

    // Parse jurisdiction
    let jurisdiction = match body.jurisdiction.to_lowercase().as_str() {
        "colombia" | "arco" => Jurisdiction::Colombia,
        "gdpr" | "eu" => Jurisdiction::Gdpr,
        "ccpa" | "cpra" | "california" => Jurisdiction::Ccpa,
        "lgpd" | "brazil" => Jurisdiction::Lgpd,
        _ => Jurisdiction::General,
    };

    // Parse right type
    let right_type = match body.right_type.to_lowercase().as_str() {
        "access" => DataRightType::Access,
        "rectification" => DataRightType::Rectification,
        "erasure" | "cancellation" | "deletion" => DataRightType::Erasure,
        "objection" | "opposition" => DataRightType::Objection,
        "restriction" => DataRightType::Restriction,
        "portability" => DataRightType::Portability,
        "automated_decision" => DataRightType::AutomatedDecision,
        "opt_out_sale" => DataRightType::OptOutSale,
        "opt_out_sharing" => DataRightType::OptOutSharing,
        "limit_sensitive" => DataRightType::LimitSensitive,
        "confirmation" => DataRightType::Confirmation,
        "anonymization" => DataRightType::Anonymization,
        "revoke_consent" => DataRightType::RevokeConsent,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                code: "INVALID_RIGHT_TYPE".to_string(),
                message: format!("Unknown right type: {}", body.right_type),
                details: None,
            });
        }
    };

    let dto = CreateDataRequestDto {
        jurisdiction,
        right_type,
        email: body.email.clone(),
        name: body.name.clone(),
        document_type: body.document_type.clone(),
        document_number: body.document_number.clone(),
        is_representative: body.is_representative,
        represented_name: body.represented_name.clone(),
        data_categories: body.data_categories.clone(),
        specific_request: body.specific_request.clone(),
        reason: body.reason.clone(),
    };

    match service.create_data_request(
        dto,
        user_id,
        ip_address.as_deref(),
        user_agent.as_deref(),
        tenant_id,
    ).await {
        Ok(request) => {
            let deadlines = LegalDeadlines::for_jurisdiction(jurisdiction);
            let response_days = deadlines.response_days;
            HttpResponse::Created().json(serde_json::json!({
                "request": DataRequestResponse::from(request),
                "deadlines": LegalDeadlinesResponse::from(deadlines),
                "message": format!("Request received. Response deadline: {} days", response_days)
            }))
        }
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/requests/{request_id}
/// Obtiene una solicitud por ID
pub async fn get_data_request(
    service: ServiceData,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let request_id = path.into_inner();

    match service.get_data_request(request_id).await {
        Ok(request) => HttpResponse::Ok().json(DataRequestResponse::from(request)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/requests
/// Lista solicitudes (admin) con filtros
pub async fn list_data_requests(
    service: ServiceData,
    req: HttpRequest,
    query: web::Query<ListRequestsQuery>,
) -> HttpResponse {
    let user_role = extract_user_role(&req).unwrap_or_default();
    let user_id = extract_user_id(&req);

    // Non-admin solo puede ver sus propias solicitudes
    let filters = if user_role != "admin" {
        DataRequestFilters {
            user_id,
            ..Default::default()
        }
    } else {
        DataRequestFilters {
            jurisdiction: query.jurisdiction.as_ref().map(|j| match j.as_str() {
                "colombia" => Jurisdiction::Colombia,
                "gdpr" => Jurisdiction::Gdpr,
                "ccpa" => Jurisdiction::Ccpa,
                "lgpd" => Jurisdiction::Lgpd,
                _ => Jurisdiction::General,
            }),
            right_type: query.right_type.as_ref().and_then(|r| parse_right_type(r)),
            status: query.status.as_ref().and_then(|s| parse_status(s)),
            email: query.email.clone(),
            from_date: query.from_date,
            to_date: query.to_date,
            page: query.page,
            page_size: query.page_size,
            ..Default::default()
        }
    };

    match service.list_data_requests(filters).await {
        Ok(result) => HttpResponse::Ok().json(DataRequestListResponse::from(result)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/my-requests
/// Obtiene solicitudes del usuario actual
pub async fn get_my_requests(
    service: ServiceData,
    req: HttpRequest,
) -> HttpResponse {
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

    match service.get_user_requests(user_id).await {
        Ok(result) => HttpResponse::Ok().json(DataRequestListResponse::from(result)),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/compliance/requests/{request_id}/process
/// Procesa una solicitud (admin)
pub async fn process_request(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<ProcessRequestBody>,
) -> HttpResponse {
    let request_id = path.into_inner();

    let admin_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Authentication required".to_string(),
                details: None,
            });
        }
    };

    let user_role = extract_user_role(&req).unwrap_or_default();
    if user_role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse {
            code: "FORBIDDEN".to_string(),
            message: "Admin access required".to_string(),
            details: None,
        });
    }

    let status = match parse_status(&body.status) {
        Some(s) => s,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                code: "INVALID_STATUS".to_string(),
                message: format!("Unknown status: {}", body.status),
                details: None,
            });
        }
    };

    let tenant_id = extract_tenant_id(&req);
    let ip_address = extract_ip(&req);

    match service.process_request(
        request_id,
        admin_id,
        status,
        body.decision.as_deref(),
        body.explanation.as_deref(),
        ip_address.as_deref(),
        tenant_id,
    ).await {
        Ok(request) => HttpResponse::Ok().json(DataRequestResponse::from(request)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Cookie Consent
// =============================================================================

/// POST /api/v1/compliance/consent/cookies
/// Guarda preferencias de cookies
pub async fn save_cookie_consent(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<SaveCookieConsentBody>,
) -> HttpResponse {
    let user_id = extract_user_id(&req);
    let anonymous_id = req.headers()
        .get("X-Anonymous-Id")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let ip_address = extract_ip(&req);
    let user_agent = extract_user_agent(&req);

    let dto = SaveCookieConsentDto {
        functional: body.functional,
        analytics: body.analytics,
        marketing: body.marketing,
        social_media: body.social_media,
        policy_version: body.policy_version.clone(),
    };

    match service.save_cookie_preferences(
        user_id,
        anonymous_id.as_deref(),
        dto,
        ip_address.as_deref(),
        user_agent.as_deref(),
    ).await {
        Ok(prefs) => HttpResponse::Ok().json(CookiePreferencesResponse::from(prefs)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/consent/cookies
/// Obtiene preferencias de cookies actuales
pub async fn get_cookie_consent(
    service: ServiceData,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = extract_user_id(&req);
    let anonymous_id = req.headers()
        .get("X-Anonymous-Id")
        .and_then(|v| v.to_str().ok());

    match service.get_cookie_preferences(user_id, anonymous_id).await {
        Ok(Some(prefs)) => HttpResponse::Ok().json(CookiePreferencesResponse::from(prefs)),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({
            "essential": true,
            "functional": false,
            "analytics": false,
            "marketing": false,
            "social_media": false,
            "has_consent": false
        })),
        Err(e) => error_response(e),
    }
}

/// POST /api/v1/compliance/consent
/// Registra un consentimiento individual
pub async fn record_consent(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<RecordConsentBody>,
) -> HttpResponse {
    let user_id = extract_user_id(&req);
    let anonymous_id = req.headers()
        .get("X-Anonymous-Id")
        .and_then(|v| v.to_str().ok());
    let tenant_id = extract_tenant_id(&req);
    let ip_address = extract_ip(&req);
    let user_agent = extract_user_agent(&req);

    match service.record_consent(
        user_id,
        anonymous_id,
        &body.consent_type,
        body.granted,
        &body.policy_version,
        &body.source,
        ip_address.as_deref(),
        user_agent.as_deref(),
        tenant_id,
    ).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            message: "Consent recorded".to_string(),
        }),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Data Export (Portability)
// =============================================================================

/// POST /api/v1/compliance/export
/// Solicita exportación de datos personales
pub async fn create_data_export(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<CreateExportBody>,
) -> HttpResponse {
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

    let format = match body.format.to_lowercase().as_str() {
        "json" => ExportFormat::Json,
        "csv" => ExportFormat::Csv,
        "xml" => ExportFormat::Xml,
        "zip" => ExportFormat::Zip,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                code: "INVALID_FORMAT".to_string(),
                message: format!("Unknown format: {}", body.format),
                details: None,
            });
        }
    };

    let categories: Vec<DataCategory> = body.categories.iter()
        .filter_map(|c| match c.to_lowercase().as_str() {
            "profile" => Some(DataCategory::Profile),
            "preferences" => Some(DataCategory::Preferences),
            "enrollments" => Some(DataCategory::Enrollments),
            "certificates" => Some(DataCategory::Certificates),
            "purchases" => Some(DataCategory::Purchases),
            "communications" => Some(DataCategory::Communications),
            "activity_logs" => Some(DataCategory::ActivityLogs),
            "content_created" => Some(DataCategory::ContentCreated),
            "consents" => Some(DataCategory::Consents),
            _ => None,
        })
        .collect();

    if categories.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "NO_CATEGORIES".to_string(),
            message: "At least one valid data category is required".to_string(),
            details: None,
        });
    }

    let dto = CreateExportRequestDto { format, categories };
    let tenant_id = extract_tenant_id(&req);
    let ip_address = extract_ip(&req);

    match service.create_data_export(user_id, dto, None, ip_address.as_deref(), tenant_id).await {
        Ok(export) => HttpResponse::Created().json(DataExportResponse::from(export)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/export/{export_id}
/// Obtiene estado de una exportación
pub async fn get_data_export(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let export_id = path.into_inner();

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

    match service.get_data_export(export_id, user_id).await {
        Ok(export) => HttpResponse::Ok().json(DataExportResponse::from(export)),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Account Deletion
// =============================================================================

/// POST /api/v1/compliance/delete-account
/// Solicita eliminación de cuenta (derecho al olvido)
pub async fn request_account_deletion(
    service: ServiceData,
    req: HttpRequest,
    body: web::Json<DeleteAccountBody>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Invalid request".to_string(),
            details: Some(serde_json::to_value(errors).unwrap_or_default()),
        });
    }

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

    // Obtener email del usuario (en producción, de users-service)
    let user_email = req.headers()
        .get("X-User-Email")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let dto = CreateDeletionRequestDto {
        reason: body.reason.clone(),
        confirm_email: body.confirm_email.clone(),
    };

    let tenant_id = extract_tenant_id(&req);
    let ip_address = extract_ip(&req);

    match service.request_account_deletion(
        user_id,
        dto,
        user_email,
        None,
        ip_address.as_deref(),
        tenant_id,
    ).await {
        Ok(deletion) => HttpResponse::Created().json(DeletionRequestResponse::from(deletion)),
        Err(e) => error_response(e),
    }
}

/// DELETE /api/v1/compliance/delete-account/{deletion_id}
/// Cancela solicitud de eliminación
pub async fn cancel_account_deletion(
    service: ServiceData,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let deletion_id = path.into_inner();

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
    let ip_address = extract_ip(&req);

    match service.cancel_account_deletion(
        deletion_id,
        user_id,
        "User cancelled",
        ip_address.as_deref(),
        tenant_id,
    ).await {
        Ok(()) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            message: "Account deletion cancelled".to_string(),
        }),
        Err(e) => error_response(e),
    }
}

// =============================================================================
// Statistics & Info
// =============================================================================

/// GET /api/v1/compliance/stats
/// Obtiene estadísticas de compliance (admin)
pub async fn get_compliance_stats(
    service: ServiceData,
    req: HttpRequest,
) -> HttpResponse {
    let user_role = extract_user_role(&req).unwrap_or_default();
    if user_role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse {
            code: "FORBIDDEN".to_string(),
            message: "Admin access required".to_string(),
            details: None,
        });
    }

    let tenant_id = extract_tenant_id(&req);

    match service.get_compliance_stats(tenant_id).await {
        Ok(stats) => HttpResponse::Ok().json(ComplianceStatsResponse::from(stats)),
        Err(e) => error_response(e),
    }
}

/// GET /api/v1/compliance/deadlines/{jurisdiction}
/// Obtiene plazos legales para una jurisdicción
pub async fn get_legal_deadlines(
    service: ServiceData,
    path: web::Path<String>,
) -> HttpResponse {
    let jurisdiction_str = path.into_inner();
    let jurisdiction = match jurisdiction_str.to_lowercase().as_str() {
        "colombia" | "arco" => Jurisdiction::Colombia,
        "gdpr" | "eu" => Jurisdiction::Gdpr,
        "ccpa" | "cpra" | "california" => Jurisdiction::Ccpa,
        "lgpd" | "brazil" => Jurisdiction::Lgpd,
        _ => Jurisdiction::General,
    };

    let deadlines = service.get_legal_deadlines(jurisdiction);
    HttpResponse::Ok().json(LegalDeadlinesResponse::from(deadlines))
}

// =============================================================================
// Health Check
// =============================================================================

/// GET /health
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "compliance-service",
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
        .map(String::from)
}

fn extract_ip(req: &HttpRequest) -> Option<String> {
    req.connection_info()
        .realip_remote_addr()
        .map(String::from)
}

fn extract_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn parse_right_type(s: &str) -> Option<DataRightType> {
    match s.to_lowercase().as_str() {
        "access" => Some(DataRightType::Access),
        "rectification" => Some(DataRightType::Rectification),
        "erasure" => Some(DataRightType::Erasure),
        "objection" => Some(DataRightType::Objection),
        "restriction" => Some(DataRightType::Restriction),
        "portability" => Some(DataRightType::Portability),
        "automated_decision" => Some(DataRightType::AutomatedDecision),
        "opt_out_sale" => Some(DataRightType::OptOutSale),
        "opt_out_sharing" => Some(DataRightType::OptOutSharing),
        "limit_sensitive" => Some(DataRightType::LimitSensitive),
        "confirmation" => Some(DataRightType::Confirmation),
        "anonymization" => Some(DataRightType::Anonymization),
        "revoke_consent" => Some(DataRightType::RevokeConsent),
        _ => None,
    }
}

fn parse_status(s: &str) -> Option<RequestStatus> {
    match s.to_lowercase().as_str() {
        "received" => Some(RequestStatus::Received),
        "identity_pending" => Some(RequestStatus::IdentityPending),
        "in_progress" => Some(RequestStatus::InProgress),
        "awaiting_info" => Some(RequestStatus::AwaitingInfo),
        "resolved" => Some(RequestStatus::Resolved),
        "denied" => Some(RequestStatus::Denied),
        "appealed" => Some(RequestStatus::Appealed),
        "expired" => Some(RequestStatus::Expired),
        _ => None,
    }
}

fn error_response(err: ComplianceError) -> HttpResponse {
    match &err {
        ComplianceError::NotFound(_) => HttpResponse::NotFound().json(ErrorResponse {
            code: "NOT_FOUND".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ComplianceError::AccessDenied(_) => HttpResponse::Forbidden().json(ErrorResponse {
            code: "ACCESS_DENIED".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ComplianceError::Validation(_) => HttpResponse::BadRequest().json(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ComplianceError::IdentityNotVerified => HttpResponse::Forbidden().json(ErrorResponse {
            code: "IDENTITY_NOT_VERIFIED".to_string(),
            message: err.to_string(),
            details: None,
        }),
        ComplianceError::DeletionAlreadyScheduled => HttpResponse::Conflict().json(ErrorResponse {
            code: "DELETION_ALREADY_SCHEDULED".to_string(),
            message: err.to_string(),
            details: None,
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
