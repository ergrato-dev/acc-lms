// =============================================================================
// ACC LMS - Compliance Service DTOs
// =============================================================================
// Data Transfer Objects para la API de compliance
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    ComplianceStats, CookiePreferences, DataCategory, DataExport, DataRequest,
    DataRightType, DeletionRequest, ExportFormat, ExportStatus, Jurisdiction,
    LegalDeadlines, RequestStatus,
};

// =============================================================================
// Request DTOs
// =============================================================================

/// Request para crear solicitud de derechos (ARCO/GDPR/CCPA/LGPD)
#[derive(Debug, Deserialize, Validate)]
pub struct CreateDataRequestBody {
    pub jurisdiction: String,
    pub right_type: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 2, max = 200))]
    pub name: String,

    pub document_type: Option<String>,
    pub document_number: Option<String>,

    #[serde(default)]
    pub is_representative: bool,
    pub represented_name: Option<String>,

    pub data_categories: Option<Vec<String>>,

    #[validate(length(min = 10, max = 5000))]
    pub specific_request: String,

    pub reason: Option<String>,
}

/// Request para guardar preferencias de cookies
#[derive(Debug, Deserialize)]
pub struct SaveCookieConsentBody {
    #[serde(default)]
    pub functional: bool,
    #[serde(default)]
    pub analytics: bool,
    #[serde(default)]
    pub marketing: bool,
    #[serde(default)]
    pub social_media: bool,
    pub policy_version: String,
}

/// Request para registrar consentimiento individual
#[derive(Debug, Deserialize)]
pub struct RecordConsentBody {
    pub consent_type: String,
    pub granted: bool,
    pub policy_version: String,
    pub source: String,
}

/// Request para exportar datos
#[derive(Debug, Deserialize)]
pub struct CreateExportBody {
    pub format: String, // json, csv, xml, zip
    pub categories: Vec<String>,
}

/// Request para solicitar eliminación de cuenta
#[derive(Debug, Deserialize, Validate)]
pub struct DeleteAccountBody {
    pub reason: Option<String>,

    #[validate(email)]
    pub confirm_email: String,
}

/// Request para procesar solicitud (admin)
#[derive(Debug, Deserialize)]
pub struct ProcessRequestBody {
    pub status: String,
    pub decision: Option<String>,
    pub explanation: Option<String>,
}

/// Filtros para listar solicitudes
#[derive(Debug, Default, Deserialize)]
pub struct ListRequestsQuery {
    pub jurisdiction: Option<String>,
    pub right_type: Option<String>,
    pub status: Option<String>,
    pub email: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// Respuesta de solicitud de derechos
#[derive(Debug, Serialize)]
pub struct DataRequestResponse {
    pub request_id: Uuid,
    pub jurisdiction: String,
    pub right_type: String,
    pub status: String,
    pub requester_email: String,
    pub requester_name: String,
    pub specific_request: String,
    pub data_categories: Option<Vec<String>>,
    pub response_deadline: DateTime<Utc>,
    pub response_decision: Option<String>,
    pub response_explanation: Option<String>,
    pub identity_verified: bool,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl From<DataRequest> for DataRequestResponse {
    fn from(r: DataRequest) -> Self {
        Self {
            request_id: r.request_id,
            jurisdiction: r.jurisdiction.to_string(),
            right_type: r.right_type.to_string(),
            status: r.status.to_string(),
            requester_email: r.requester_email,
            requester_name: r.requester_name,
            specific_request: r.specific_request,
            data_categories: r.data_categories,
            response_deadline: r.response_deadline,
            response_decision: r.response_decision,
            response_explanation: r.response_explanation,
            identity_verified: r.identity_verified,
            created_at: r.created_at,
            acknowledged_at: r.acknowledged_at,
            resolved_at: r.resolved_at,
        }
    }
}

/// Lista paginada de solicitudes
#[derive(Debug, Serialize)]
pub struct DataRequestListResponse {
    pub requests: Vec<DataRequestResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl From<crate::domain::DataRequestListResponse> for DataRequestListResponse {
    fn from(r: crate::domain::DataRequestListResponse) -> Self {
        Self {
            requests: r.requests.into_iter().map(Into::into).collect(),
            total: r.total,
            page: r.page,
            page_size: r.page_size,
            total_pages: r.total_pages,
        }
    }
}

/// Respuesta de preferencias de cookies
#[derive(Debug, Serialize)]
pub struct CookiePreferencesResponse {
    pub essential: bool,
    pub functional: bool,
    pub analytics: bool,
    pub marketing: bool,
    pub social_media: bool,
    pub policy_version: String,
    pub updated_at: DateTime<Utc>,
}

impl From<CookiePreferences> for CookiePreferencesResponse {
    fn from(p: CookiePreferences) -> Self {
        Self {
            essential: p.essential,
            functional: p.functional,
            analytics: p.analytics,
            marketing: p.marketing,
            social_media: p.social_media,
            policy_version: p.policy_version,
            updated_at: p.updated_at,
        }
    }
}

/// Respuesta de exportación de datos
#[derive(Debug, Serialize)]
pub struct DataExportResponse {
    pub export_id: Uuid,
    pub status: String,
    pub progress_percent: i32,
    pub format: String,
    pub categories: Vec<String>,
    pub download_url: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

impl From<DataExport> for DataExportResponse {
    fn from(e: DataExport) -> Self {
        Self {
            export_id: e.export_id,
            status: e.status.to_string(),
            progress_percent: e.progress_percent,
            format: e.format,
            categories: e.categories,
            download_url: e.download_url,
            file_size_bytes: e.file_size_bytes,
            expires_at: e.expires_at,
            created_at: e.created_at,
            completed_at: e.completed_at,
            error_message: e.error_message,
        }
    }
}

/// Respuesta de solicitud de eliminación
#[derive(Debug, Serialize)]
pub struct DeletionRequestResponse {
    pub deletion_id: Uuid,
    pub status: String,
    pub scheduled_at: DateTime<Utc>,
    pub can_cancel_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<DeletionRequest> for DeletionRequestResponse {
    fn from(d: DeletionRequest) -> Self {
        Self {
            deletion_id: d.deletion_id,
            status: d.status.to_string(),
            scheduled_at: d.scheduled_at,
            can_cancel_until: d.scheduled_at, // Hasta la fecha programada
            created_at: d.created_at,
        }
    }
}

/// Plazos legales
#[derive(Debug, Serialize)]
pub struct LegalDeadlinesResponse {
    pub jurisdiction: String,
    pub response_days: i32,
    pub extension_days: Option<i32>,
    pub breach_notification_hours: Option<i32>,
}

impl From<LegalDeadlines> for LegalDeadlinesResponse {
    fn from(d: LegalDeadlines) -> Self {
        Self {
            jurisdiction: d.jurisdiction.to_string(),
            response_days: d.response_days,
            extension_days: d.extension_days,
            breach_notification_hours: d.breach_notification_hours,
        }
    }
}

/// Estadísticas de compliance
#[derive(Debug, Serialize)]
pub struct ComplianceStatsResponse {
    pub total_requests: i64,
    pub pending_requests: i64,
    pub requests_by_jurisdiction: Vec<JurisdictionStatsDto>,
    pub requests_by_type: Vec<TypeStatsDto>,
    pub requests_by_status: Vec<StatusStatsDto>,
    pub avg_response_days: f64,
    pub sla_compliance_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct JurisdictionStatsDto {
    pub jurisdiction: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct TypeStatsDto {
    pub right_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatusStatsDto {
    pub status: String,
    pub count: i64,
}

impl From<ComplianceStats> for ComplianceStatsResponse {
    fn from(s: ComplianceStats) -> Self {
        Self {
            total_requests: s.total_requests,
            pending_requests: s.pending_requests,
            requests_by_jurisdiction: s.requests_by_jurisdiction.into_iter().map(|j| JurisdictionStatsDto {
                jurisdiction: j.jurisdiction,
                count: j.count,
            }).collect(),
            requests_by_type: s.requests_by_type.into_iter().map(|t| TypeStatsDto {
                right_type: t.right_type,
                count: t.count,
            }).collect(),
            requests_by_status: s.requests_by_status.into_iter().map(|s| StatusStatsDto {
                status: s.status,
                count: s.count,
            }).collect(),
            avg_response_days: s.avg_response_days,
            sla_compliance_percent: s.sla_compliance_percent,
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

/// Respuesta de confirmación simple
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}
