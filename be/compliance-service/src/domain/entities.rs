// =============================================================================
// ACC LMS - Compliance Service Domain Entities
// =============================================================================
// Entidades para cumplimiento normativo: GDPR, CCPA, LGPD, Habeas Data
// Multi-jurisdicción: Colombia, UE, US California, Brasil
// =============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// ENUMS - Jurisdicciones y Regulaciones
// =============================================================================

/// Jurisdicción/regulación aplicable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Jurisdiction {
    /// Colombia - Ley 1581/2012 (Habeas Data)
    Colombia,
    /// Unión Europea - GDPR
    Gdpr,
    /// California - CCPA/CPRA
    Ccpa,
    /// Brasil - LGPD
    Lgpd,
    /// General (sin jurisdicción específica)
    General,
}

impl std::fmt::Display for Jurisdiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Jurisdiction::Colombia => write!(f, "colombia"),
            Jurisdiction::Gdpr => write!(f, "gdpr"),
            Jurisdiction::Ccpa => write!(f, "ccpa"),
            Jurisdiction::Lgpd => write!(f, "lgpd"),
            Jurisdiction::General => write!(f, "general"),
        }
    }
}

// =============================================================================
// ENUMS - Tipos de Solicitud
// =============================================================================

/// Tipo de derecho ejercido (unificado para todas las jurisdicciones)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DataRightType {
    // ARCO (Colombia)
    /// Acceso - Conocer qué datos tenemos
    Access,
    /// Rectificación - Corregir datos inexactos
    Rectification,
    /// Cancelación/Supresión - Eliminar datos (derecho al olvido)
    Erasure,
    /// Oposición - Oponerse a tratamiento
    Objection,

    // GDPR adicionales
    /// Restricción de procesamiento
    Restriction,
    /// Portabilidad de datos
    Portability,
    /// Objeción a decisiones automatizadas
    AutomatedDecision,

    // CCPA específicos
    /// Opt-out de venta de datos
    OptOutSale,
    /// Opt-out de compartir datos
    OptOutSharing,
    /// Limitar uso de datos sensibles
    LimitSensitive,

    // LGPD adicionales
    /// Confirmación de existencia de tratamiento
    Confirmation,
    /// Anonimización
    Anonymization,
    /// Revocación de consentimiento
    RevokeConsent,
}

impl std::fmt::Display for DataRightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataRightType::Access => write!(f, "access"),
            DataRightType::Rectification => write!(f, "rectification"),
            DataRightType::Erasure => write!(f, "erasure"),
            DataRightType::Objection => write!(f, "objection"),
            DataRightType::Restriction => write!(f, "restriction"),
            DataRightType::Portability => write!(f, "portability"),
            DataRightType::AutomatedDecision => write!(f, "automated_decision"),
            DataRightType::OptOutSale => write!(f, "opt_out_sale"),
            DataRightType::OptOutSharing => write!(f, "opt_out_sharing"),
            DataRightType::LimitSensitive => write!(f, "limit_sensitive"),
            DataRightType::Confirmation => write!(f, "confirmation"),
            DataRightType::Anonymization => write!(f, "anonymization"),
            DataRightType::RevokeConsent => write!(f, "revoke_consent"),
        }
    }
}

/// Estado de una solicitud de derechos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RequestStatus {
    /// Recibida
    Received,
    /// Pendiente verificación de identidad
    IdentityPending,
    /// En proceso
    InProgress,
    /// Requiere información adicional
    AwaitingInfo,
    /// Completada/Resuelta
    Resolved,
    /// Rechazada (con justificación legal)
    Denied,
    /// Apelada ante autoridad
    Appealed,
    /// Expirada
    Expired,
}

impl std::fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestStatus::Received => write!(f, "received"),
            RequestStatus::IdentityPending => write!(f, "identity_pending"),
            RequestStatus::InProgress => write!(f, "in_progress"),
            RequestStatus::AwaitingInfo => write!(f, "awaiting_info"),
            RequestStatus::Resolved => write!(f, "resolved"),
            RequestStatus::Denied => write!(f, "denied"),
            RequestStatus::Appealed => write!(f, "appealed"),
            RequestStatus::Expired => write!(f, "expired"),
        }
    }
}

/// Decisión de respuesta
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseDecision {
    /// Aprobada completamente
    Approved,
    /// Parcialmente aprobada
    Partial,
    /// Rechazada
    Denied,
}

// =============================================================================
// ENUMS - Consentimiento
// =============================================================================

/// Tipo de consentimiento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ConsentType {
    /// Términos y condiciones
    Terms,
    /// Política de privacidad
    Privacy,
    /// Cookies esenciales (siempre activas)
    CookiesEssential,
    /// Cookies funcionales
    CookiesFunctional,
    /// Cookies analytics
    CookiesAnalytics,
    /// Cookies marketing
    CookiesMarketing,
    /// Comunicaciones de marketing
    Marketing,
    /// Newsletter
    Newsletter,
    /// Compartir datos con terceros
    ThirdPartySharing,
    /// Perfilado y personalización
    Profiling,
}

impl std::fmt::Display for ConsentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsentType::Terms => write!(f, "terms"),
            ConsentType::Privacy => write!(f, "privacy"),
            ConsentType::CookiesEssential => write!(f, "cookies_essential"),
            ConsentType::CookiesFunctional => write!(f, "cookies_functional"),
            ConsentType::CookiesAnalytics => write!(f, "cookies_analytics"),
            ConsentType::CookiesMarketing => write!(f, "cookies_marketing"),
            ConsentType::Marketing => write!(f, "marketing"),
            ConsentType::Newsletter => write!(f, "newsletter"),
            ConsentType::ThirdPartySharing => write!(f, "third_party_sharing"),
            ConsentType::Profiling => write!(f, "profiling"),
        }
    }
}

/// Estado del consentimiento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    /// Otorgado
    Granted,
    /// Denegado
    Denied,
    /// Revocado (previamente otorgado)
    Revoked,
    /// Pendiente (no ha decidido)
    Pending,
}

// =============================================================================
// ENUMS - Exportación de Datos
// =============================================================================

/// Formato de exportación de datos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
    Zip, // Contiene múltiples formatos
}

/// Estado de exportación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ExportStatus {
    /// En cola
    Queued,
    /// Procesando
    Processing,
    /// Listo para descarga
    Ready,
    /// Expirado
    Expired,
    /// Error
    Failed,
}

impl std::fmt::Display for ExportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportStatus::Queued => write!(f, "queued"),
            ExportStatus::Processing => write!(f, "processing"),
            ExportStatus::Ready => write!(f, "ready"),
            ExportStatus::Expired => write!(f, "expired"),
            ExportStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Categoría de datos para exportación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataCategory {
    Profile,
    Preferences,
    Enrollments,
    Certificates,
    Purchases,
    Communications,
    ActivityLogs,
    ContentCreated,
    Consents,
}

// =============================================================================
// ENTIDADES PRINCIPALES
// =============================================================================

/// Solicitud de derechos de datos (ARCO/GDPR/CCPA/LGPD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequest {
    pub request_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Jurisdicción y tipo
    pub jurisdiction: Jurisdiction,
    pub right_type: DataRightType,

    // Solicitante
    pub user_id: Option<Uuid>,          // Si está autenticado
    pub requester_email: String,
    pub requester_name: String,
    pub document_type: Option<String>,  // CC, CE, passport, etc.
    pub document_number: Option<String>,

    // Representante (si aplica)
    pub is_representative: bool,
    pub represented_name: Option<String>,
    pub authorization_document_url: Option<String>,

    // Detalles
    pub data_categories: Option<Vec<String>>,
    pub specific_request: String,
    pub reason: Option<String>,
    pub supporting_documents: Option<Vec<String>>,

    // Estado
    pub status: RequestStatus,
    pub response_deadline: DateTime<Utc>,

    // Respuesta
    pub response_decision: Option<String>,
    pub response_explanation: Option<String>,
    pub response_actions: Option<Vec<String>>,
    pub export_url: Option<String>,
    pub export_expires_at: Option<DateTime<Utc>>,

    // Verificación
    pub identity_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verification_method: Option<String>,

    // Tracking
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Registro de consentimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub consent_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Usuario (puede ser anónimo para cookies)
    pub user_id: Option<Uuid>,
    pub anonymous_id: Option<String>, // Fingerprint/cookie ID

    // Consentimiento
    pub consent_type: ConsentType,
    pub granted: bool,
    pub policy_version: String,

    // Contexto
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub source: String, // banner, registration, settings

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// Preferencias de cookies (agregado por usuario/sesión)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookiePreferences {
    pub preference_id: Uuid,
    pub user_id: Option<Uuid>,
    pub anonymous_id: Option<String>,

    // Preferencias por categoría
    pub essential: bool,      // Siempre true
    pub functional: bool,
    pub analytics: bool,
    pub marketing: bool,
    pub social_media: bool,

    // Metadata
    pub policy_version: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Exportación de datos del usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExport {
    pub export_id: Uuid,
    pub user_id: Uuid,
    pub request_id: Option<Uuid>, // Si viene de una solicitud formal

    // Configuración
    pub format: String,
    pub categories: Vec<String>,

    // Estado
    pub status: ExportStatus,
    pub progress_percent: i32,

    // Resultado
    pub download_url: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub checksum: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub processing_started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,

    // Error info
    pub error_message: Option<String>,
}

/// Solicitud de eliminación de cuenta (derecho al olvido)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionRequest {
    pub deletion_id: Uuid,
    pub user_id: Uuid,
    pub request_id: Option<Uuid>,

    // Razón
    pub reason: Option<String>,

    // Estado
    pub status: RequestStatus,
    pub scheduled_at: DateTime<Utc>,  // Fecha programada (período de gracia)

    // Progreso
    pub services_notified: Vec<String>,
    pub services_completed: Vec<String>,
    pub services_failed: Vec<String>,

    // Resultado
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Log de auditoría de compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAuditLog {
    pub log_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Quién
    pub actor_id: Option<Uuid>,       // Usuario que realizó la acción
    pub actor_type: String,           // user, admin, system
    pub actor_ip: Option<String>,

    // Qué
    pub action: String,               // consent_granted, data_exported, etc.
    pub resource_type: String,        // consent, data_request, etc.
    pub resource_id: Option<Uuid>,

    // Afectado
    pub subject_id: Option<Uuid>,     // Usuario afectado
    pub subject_email: Option<String>,

    // Detalles
    pub details: Option<serde_json::Value>,
    pub jurisdiction: Option<Jurisdiction>,

    // Timestamp
    pub created_at: DateTime<Utc>,
}

/// Versión de documento legal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDocument {
    pub document_id: Uuid,
    pub tenant_id: Option<Uuid>,

    // Tipo
    pub document_type: String, // terms, privacy, cookies
    pub language: String,
    pub version: String,

    // Contenido
    pub title: String,
    pub content_markdown: String,
    pub content_html: Option<String>,

    // Vigencia
    pub effective_date: DateTime<Utc>,
    pub supersedes_version: Option<String>,

    // Estado
    pub is_current: bool,
    pub is_published: bool,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

/// Request para crear solicitud de derechos
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateDataRequestDto {
    pub jurisdiction: Jurisdiction,
    pub right_type: DataRightType,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 2, max = 200))]
    pub name: String,

    pub document_type: Option<String>,
    pub document_number: Option<String>,

    pub is_representative: bool,
    pub represented_name: Option<String>,

    pub data_categories: Option<Vec<String>>,

    #[validate(length(min = 10, max = 5000))]
    pub specific_request: String,

    pub reason: Option<String>,
}

/// Request para guardar consentimiento de cookies
#[derive(Debug, Clone, Deserialize)]
pub struct SaveCookieConsentDto {
    pub functional: bool,
    pub analytics: bool,
    pub marketing: bool,
    pub social_media: bool,
    pub policy_version: String,
}

/// Request para exportar datos
#[derive(Debug, Clone, Deserialize)]
pub struct CreateExportRequestDto {
    pub format: ExportFormat,
    pub categories: Vec<DataCategory>,
}

/// Request para solicitar eliminación
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDeletionRequestDto {
    pub reason: Option<String>,
    pub confirm_email: String,
}

/// Filtros para listar solicitudes
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DataRequestFilters {
    pub jurisdiction: Option<Jurisdiction>,
    pub right_type: Option<DataRightType>,
    pub status: Option<RequestStatus>,
    pub user_id: Option<Uuid>,
    pub email: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Respuesta paginada de solicitudes
#[derive(Debug, Clone, Serialize)]
pub struct DataRequestListResponse {
    pub requests: Vec<DataRequest>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Estadísticas de compliance
#[derive(Debug, Clone, Serialize)]
pub struct ComplianceStats {
    pub total_requests: i64,
    pub pending_requests: i64,
    pub requests_by_jurisdiction: Vec<JurisdictionStats>,
    pub requests_by_type: Vec<TypeStats>,
    pub requests_by_status: Vec<StatusStats>,
    pub avg_response_days: f64,
    pub sla_compliance_percent: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct JurisdictionStats {
    pub jurisdiction: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeStats {
    pub right_type: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusStats {
    pub status: String,
    pub count: i64,
}

/// Plazos legales por jurisdicción
#[derive(Debug, Clone, Serialize)]
pub struct LegalDeadlines {
    pub jurisdiction: Jurisdiction,
    pub response_days: i32,
    pub extension_days: Option<i32>,
    pub breach_notification_hours: Option<i32>,
}

impl LegalDeadlines {
    pub fn for_jurisdiction(jurisdiction: Jurisdiction) -> Self {
        match jurisdiction {
            Jurisdiction::Colombia => Self {
                jurisdiction,
                response_days: 15, // días hábiles
                extension_days: Some(8),
                breach_notification_hours: None,
            },
            Jurisdiction::Gdpr => Self {
                jurisdiction,
                response_days: 30,
                extension_days: Some(60), // 2 meses adicionales
                breach_notification_hours: Some(72),
            },
            Jurisdiction::Ccpa => Self {
                jurisdiction,
                response_days: 45,
                extension_days: Some(45),
                breach_notification_hours: None,
            },
            Jurisdiction::Lgpd => Self {
                jurisdiction,
                response_days: 15,
                extension_days: None,
                breach_notification_hours: Some(72),
            },
            Jurisdiction::General => Self {
                jurisdiction,
                response_days: 30,
                extension_days: Some(30),
                breach_notification_hours: None,
            },
        }
    }
}
