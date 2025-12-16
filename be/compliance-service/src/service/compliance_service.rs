// =============================================================================
// ACC LMS - Compliance Service Business Logic
// =============================================================================
// Lógica de negocio para cumplimiento normativo
// GDPR, CCPA, LGPD, Habeas Data (Colombia)
// =============================================================================

use std::sync::Arc;
use thiserror::Error;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::domain::{
    ComplianceAuditLog, ComplianceStats, ConsentRecord, CookiePreferences,
    CreateDataRequestDto, CreateDeletionRequestDto, CreateExportRequestDto,
    DataCategory, DataExport, DataRequest, DataRequestFilters,
    DataRequestListResponse, DataRightType, DeletionRequest, ExportFormat,
    ExportStatus, Jurisdiction, LegalDeadlines, RequestStatus,
    SaveCookieConsentDto,
};
use crate::repository::{ComplianceRepository, RepositoryError};

#[derive(Debug, Error)]
pub enum ComplianceError {
    #[error("Request not found: {0}")]
    NotFound(Uuid),

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Invalid request: {0}")]
    Validation(String),

    #[error("Request expired")]
    Expired,

    #[error("Identity not verified")]
    IdentityNotVerified,

    #[error("Export in progress")]
    ExportInProgress,

    #[error("Deletion already scheduled")]
    DeletionAlreadyScheduled,

    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
}

pub type Result<T> = std::result::Result<T, ComplianceError>;

/// Servicio de compliance
pub struct ComplianceService {
    repository: Arc<ComplianceRepository>,
    grace_period_days: i64,
}

impl ComplianceService {
    pub fn new(repository: Arc<ComplianceRepository>) -> Self {
        Self {
            repository,
            grace_period_days: 30, // 30 días para cancelar eliminación
        }
    }

    pub fn with_grace_period(mut self, days: i64) -> Self {
        self.grace_period_days = days;
        self
    }

    // =========================================================================
    // Data Rights Requests (ARCO/GDPR/CCPA/LGPD)
    // =========================================================================

    /// Crea una nueva solicitud de derechos
    pub async fn create_data_request(
        &self,
        dto: CreateDataRequestDto,
        user_id: Option<Uuid>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<DataRequest> {
        // Validaciones según jurisdicción
        self.validate_request_for_jurisdiction(&dto)?;

        let request = self.repository.create_data_request(
            dto.jurisdiction,
            dto.right_type,
            user_id,
            &dto.email,
            &dto.name,
            dto.document_type.as_deref(),
            dto.document_number.as_deref(),
            dto.is_representative,
            dto.represented_name.as_deref(),
            &dto.specific_request,
            dto.reason.as_deref(),
            dto.data_categories,
            ip_address,
            user_agent,
            tenant_id,
        ).await?;

        // Audit log
        self.log_audit(
            user_id,
            "user",
            ip_address,
            "data_request_created",
            "data_request",
            Some(request.request_id),
            user_id,
            Some(&dto.email),
            Some(serde_json::json!({
                "jurisdiction": dto.jurisdiction.to_string(),
                "right_type": dto.right_type.to_string(),
            })),
            Some(dto.jurisdiction),
            tenant_id,
        ).await?;

        info!(
            request_id = %request.request_id,
            jurisdiction = %dto.jurisdiction,
            right_type = %dto.right_type,
            "Data request created"
        );

        Ok(request)
    }

    /// Obtiene una solicitud por ID
    pub async fn get_data_request(&self, request_id: Uuid) -> Result<DataRequest> {
        self.repository.get_data_request(request_id)
            .await
            .map_err(|e| match e {
                RepositoryError::NotFound(_) => ComplianceError::NotFound(request_id),
                _ => ComplianceError::Repository(e),
            })
    }

    /// Lista solicitudes (admin)
    pub async fn list_data_requests(
        &self,
        filters: DataRequestFilters,
    ) -> Result<DataRequestListResponse> {
        self.repository.list_data_requests(filters).await.map_err(Into::into)
    }

    /// Obtiene solicitudes de un usuario
    pub async fn get_user_requests(&self, user_id: Uuid) -> Result<DataRequestListResponse> {
        self.repository.list_data_requests(DataRequestFilters {
            user_id: Some(user_id),
            ..Default::default()
        }).await.map_err(Into::into)
    }

    /// Procesa una solicitud (admin)
    pub async fn process_request(
        &self,
        request_id: Uuid,
        admin_id: Uuid,
        status: RequestStatus,
        decision: Option<&str>,
        explanation: Option<&str>,
        ip_address: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<DataRequest> {
        let request = self.repository.get_data_request(request_id).await?;

        // Validar que se puede procesar
        match request.status {
            RequestStatus::Resolved | RequestStatus::Denied | RequestStatus::Expired => {
                return Err(ComplianceError::Validation(
                    "Request already processed".to_string()
                ));
            }
            _ => {}
        }

        self.repository.update_request_status(
            request_id,
            status,
            decision,
            explanation,
        ).await?;

        // Audit log
        self.log_audit(
            Some(admin_id),
            "admin",
            ip_address,
            "data_request_processed",
            "data_request",
            Some(request_id),
            request.user_id,
            Some(&request.requester_email),
            Some(serde_json::json!({
                "new_status": status.to_string(),
                "decision": decision,
            })),
            Some(request.jurisdiction),
            tenant_id,
        ).await?;

        info!(
            request_id = %request_id,
            admin_id = %admin_id,
            status = %status,
            "Data request processed"
        );

        self.repository.get_data_request(request_id).await.map_err(Into::into)
    }

    /// Verifica identidad de solicitante
    pub async fn verify_identity(
        &self,
        request_id: Uuid,
        method: &str,
        admin_id: Option<Uuid>,
        ip_address: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<()> {
        let request = self.repository.get_data_request(request_id).await?;

        self.repository.verify_identity(request_id, method).await?;

        self.log_audit(
            admin_id,
            admin_id.map(|_| "admin").unwrap_or("system"),
            ip_address,
            "identity_verified",
            "data_request",
            Some(request_id),
            request.user_id,
            Some(&request.requester_email),
            Some(serde_json::json!({
                "method": method,
            })),
            Some(request.jurisdiction),
            tenant_id,
        ).await?;

        Ok(())
    }

    /// Obtiene los plazos legales para una jurisdicción
    pub fn get_legal_deadlines(&self, jurisdiction: Jurisdiction) -> LegalDeadlines {
        LegalDeadlines::for_jurisdiction(jurisdiction)
    }

    fn validate_request_for_jurisdiction(&self, dto: &CreateDataRequestDto) -> Result<()> {
        // Colombia (ARCO) requiere documento de identidad
        if dto.jurisdiction == Jurisdiction::Colombia {
            if dto.document_type.is_none() || dto.document_number.is_none() {
                return Err(ComplianceError::Validation(
                    "Colombian ARCO requests require document type and number".to_string()
                ));
            }
        }

        // Representantes requieren autorización
        if dto.is_representative && dto.represented_name.is_none() {
            return Err(ComplianceError::Validation(
                "Representatives must provide the name of the represented person".to_string()
            ));
        }

        // Validar tipo de derecho vs jurisdicción
        match dto.jurisdiction {
            Jurisdiction::Colombia => {
                // ARCO solo soporta Access, Rectification, Erasure (Cancelación), Objection
                match dto.right_type {
                    DataRightType::Access | DataRightType::Rectification |
                    DataRightType::Erasure | DataRightType::Objection => {}
                    _ => return Err(ComplianceError::Validation(
                        format!("{:?} is not a valid ARCO right type", dto.right_type)
                    )),
                }
            }
            Jurisdiction::Ccpa => {
                // CCPA tiene tipos específicos
                match dto.right_type {
                    DataRightType::Access | DataRightType::Erasure |
                    DataRightType::OptOutSale | DataRightType::OptOutSharing |
                    DataRightType::Rectification | DataRightType::LimitSensitive => {}
                    _ => return Err(ComplianceError::Validation(
                        format!("{:?} is not a valid CCPA right type", dto.right_type)
                    )),
                }
            }
            _ => {} // GDPR y LGPD soportan todos los tipos
        }

        Ok(())
    }

    // =========================================================================
    // Cookie Consent
    // =========================================================================

    /// Guarda preferencias de cookies
    pub async fn save_cookie_preferences(
        &self,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
        dto: SaveCookieConsentDto,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<CookiePreferences> {
        if user_id.is_none() && anonymous_id.is_none() {
            return Err(ComplianceError::Validation(
                "Either user_id or anonymous_id is required".to_string()
            ));
        }

        let prefs = self.repository.save_cookie_preferences(
            user_id,
            anonymous_id,
            dto.functional,
            dto.analytics,
            dto.marketing,
            dto.social_media,
            &dto.policy_version,
            ip_address,
            user_agent,
        ).await?;

        info!(
            user_id = ?user_id,
            anonymous_id = ?anonymous_id,
            functional = dto.functional,
            analytics = dto.analytics,
            marketing = dto.marketing,
            "Cookie preferences saved"
        );

        Ok(prefs)
    }

    /// Obtiene preferencias de cookies
    pub async fn get_cookie_preferences(
        &self,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
    ) -> Result<Option<CookiePreferences>> {
        self.repository.get_cookie_preferences(user_id, anonymous_id)
            .await
            .map_err(Into::into)
    }

    /// Registra un consentimiento individual
    pub async fn record_consent(
        &self,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
        consent_type: &str,
        granted: bool,
        policy_version: &str,
        source: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<ConsentRecord> {
        let record = self.repository.record_consent(
            user_id,
            anonymous_id,
            consent_type,
            granted,
            policy_version,
            source,
            ip_address,
            user_agent,
            tenant_id,
        ).await?;

        // Audit para consentimientos importantes
        if consent_type == "terms" || consent_type == "privacy" {
            self.log_audit(
                user_id,
                "user",
                ip_address,
                if granted { "consent_granted" } else { "consent_denied" },
                "consent",
                Some(record.consent_id),
                user_id,
                None,
                Some(serde_json::json!({
                    "consent_type": consent_type,
                    "policy_version": policy_version,
                    "source": source,
                })),
                None,
                tenant_id,
            ).await?;
        }

        Ok(record)
    }

    // =========================================================================
    // Data Export (Portability)
    // =========================================================================

    /// Crea una solicitud de exportación de datos
    pub async fn create_data_export(
        &self,
        user_id: Uuid,
        dto: CreateExportRequestDto,
        request_id: Option<Uuid>,
        ip_address: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<DataExport> {
        let format = match dto.format {
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Xml => "xml",
            ExportFormat::Zip => "zip",
        };

        let categories: Vec<String> = dto.categories.iter()
            .map(|c| match c {
                DataCategory::Profile => "profile",
                DataCategory::Preferences => "preferences",
                DataCategory::Enrollments => "enrollments",
                DataCategory::Certificates => "certificates",
                DataCategory::Purchases => "purchases",
                DataCategory::Communications => "communications",
                DataCategory::ActivityLogs => "activity_logs",
                DataCategory::ContentCreated => "content_created",
                DataCategory::Consents => "consents",
            })
            .map(String::from)
            .collect();

        let export = self.repository.create_data_export(
            user_id,
            request_id,
            format,
            categories,
        ).await?;

        self.log_audit(
            Some(user_id),
            "user",
            ip_address,
            "data_export_requested",
            "data_export",
            Some(export.export_id),
            Some(user_id),
            None,
            Some(serde_json::json!({
                "format": format,
                "categories": &export.categories,
            })),
            None,
            tenant_id,
        ).await?;

        info!(
            export_id = %export.export_id,
            user_id = %user_id,
            format = format,
            "Data export requested"
        );

        // TODO: Disparar job de procesamiento en background
        // Para MVP, marcar como en proceso inmediatamente

        Ok(export)
    }

    /// Obtiene estado de una exportación
    pub async fn get_data_export(&self, export_id: Uuid, user_id: Uuid) -> Result<DataExport> {
        let export = self.repository.get_data_export(export_id).await?;

        // Verificar ownership
        if export.user_id != user_id {
            return Err(ComplianceError::AccessDenied(
                "Export belongs to another user".to_string()
            ));
        }

        Ok(export)
    }

    /// Actualiza estado de exportación (llamado por job de background)
    pub async fn update_export_status(
        &self,
        export_id: Uuid,
        status: ExportStatus,
        progress: i32,
        download_url: Option<&str>,
        file_size: Option<i64>,
        checksum: Option<&str>,
        error: Option<&str>,
    ) -> Result<()> {
        self.repository.update_export_status(
            export_id,
            status,
            progress,
            download_url,
            file_size,
            checksum,
            error,
        ).await.map_err(Into::into)
    }

    // =========================================================================
    // Account Deletion (Right to Erasure)
    // =========================================================================

    /// Crea solicitud de eliminación de cuenta
    pub async fn request_account_deletion(
        &self,
        user_id: Uuid,
        dto: CreateDeletionRequestDto,
        user_email: &str,
        request_id: Option<Uuid>,
        ip_address: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<DeletionRequest> {
        // Verificar email
        if dto.confirm_email.to_lowercase() != user_email.to_lowercase() {
            return Err(ComplianceError::Validation(
                "Email confirmation does not match".to_string()
            ));
        }

        // Verificar si ya hay una solicitud pendiente
        let existing = self.repository.list_data_requests(DataRequestFilters {
            user_id: Some(user_id),
            right_type: Some(DataRightType::Erasure),
            status: Some(RequestStatus::InProgress),
            ..Default::default()
        }).await?;

        if existing.total > 0 {
            return Err(ComplianceError::DeletionAlreadyScheduled);
        }

        let deletion = self.repository.create_deletion_request(
            user_id,
            request_id,
            dto.reason.as_deref(),
            self.grace_period_days,
        ).await?;

        self.log_audit(
            Some(user_id),
            "user",
            ip_address,
            "account_deletion_requested",
            "deletion_request",
            Some(deletion.deletion_id),
            Some(user_id),
            Some(user_email),
            Some(serde_json::json!({
                "scheduled_at": deletion.scheduled_at.to_rfc3339(),
                "grace_period_days": self.grace_period_days,
            })),
            None,
            tenant_id,
        ).await?;

        info!(
            deletion_id = %deletion.deletion_id,
            user_id = %user_id,
            scheduled_at = %deletion.scheduled_at,
            "Account deletion scheduled"
        );

        // TODO: Enviar email de confirmación con opción de cancelar
        // TODO: Disparar job para ejecutar eliminación después del período de gracia

        Ok(deletion)
    }

    /// Cancela solicitud de eliminación
    pub async fn cancel_account_deletion(
        &self,
        deletion_id: Uuid,
        user_id: Uuid,
        reason: &str,
        ip_address: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<()> {
        self.repository.cancel_deletion_request(deletion_id, reason).await?;

        self.log_audit(
            Some(user_id),
            "user",
            ip_address,
            "account_deletion_cancelled",
            "deletion_request",
            Some(deletion_id),
            Some(user_id),
            None,
            Some(serde_json::json!({
                "reason": reason,
            })),
            None,
            tenant_id,
        ).await?;

        info!(
            deletion_id = %deletion_id,
            user_id = %user_id,
            "Account deletion cancelled"
        );

        Ok(())
    }

    // =========================================================================
    // Statistics & Reporting
    // =========================================================================

    /// Obtiene estadísticas de compliance
    pub async fn get_compliance_stats(&self, tenant_id: Option<Uuid>) -> Result<ComplianceStats> {
        self.repository.get_compliance_stats(tenant_id).await.map_err(Into::into)
    }

    // =========================================================================
    // Audit Logging
    // =========================================================================

    async fn log_audit(
        &self,
        actor_id: Option<Uuid>,
        actor_type: &str,
        actor_ip: Option<&str>,
        action: &str,
        resource_type: &str,
        resource_id: Option<Uuid>,
        subject_id: Option<Uuid>,
        subject_email: Option<&str>,
        details: Option<serde_json::Value>,
        jurisdiction: Option<Jurisdiction>,
        tenant_id: Option<Uuid>,
    ) -> Result<ComplianceAuditLog> {
        self.repository.log_audit_event(
            actor_id,
            actor_type,
            actor_ip,
            action,
            resource_type,
            resource_id,
            subject_id,
            subject_email,
            details,
            jurisdiction,
            tenant_id,
        ).await.map_err(Into::into)
    }
}
