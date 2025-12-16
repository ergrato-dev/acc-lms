// =============================================================================
// ACC LMS - Compliance Service Repository
// =============================================================================
// Capa de persistencia para datos de compliance en PostgreSQL
// Maneja: solicitudes ARCO/GDPR/CCPA/LGPD, consentimientos, exportaciones
// =============================================================================

use chrono::{Duration, Utc};
use sqlx::{PgPool, Row};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    ComplianceAuditLog, ConsentRecord, CookiePreferences, DataExport, DataRequest,
    DataRequestFilters, DataRequestListResponse, DeletionRequest, ExportStatus,
    Jurisdiction, DataRightType, RequestStatus, LegalDeadlines,
    ComplianceStats, JurisdictionStats, TypeStats, StatusStats,
    LegalDocument,
};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

pub struct ComplianceRepository {
    pool: PgPool,
}

impl ComplianceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // Data Requests (ARCO/GDPR/CCPA/LGPD)
    // =========================================================================

    /// Crea una nueva solicitud de derechos
    pub async fn create_data_request(
        &self,
        jurisdiction: Jurisdiction,
        right_type: DataRightType,
        user_id: Option<Uuid>,
        email: &str,
        name: &str,
        document_type: Option<&str>,
        document_number: Option<&str>,
        is_representative: bool,
        represented_name: Option<&str>,
        specific_request: &str,
        reason: Option<&str>,
        data_categories: Option<Vec<String>>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        tenant_id: Option<Uuid>,
    ) -> Result<DataRequest> {
        let request_id = Uuid::new_v4();
        let now = Utc::now();

        // Calcular deadline según jurisdicción
        let deadlines = LegalDeadlines::for_jurisdiction(jurisdiction);
        let response_deadline = now + Duration::days(deadlines.response_days as i64);

        let categories_json: Option<serde_json::Value> = data_categories.as_ref()
            .map(|c| serde_json::to_value(&c).unwrap_or_default());

        sqlx::query(
            r#"
            INSERT INTO data_requests (
                request_id, tenant_id, jurisdiction, right_type,
                user_id, requester_email, requester_name,
                document_type, document_number,
                is_representative, represented_name,
                specific_request, reason, data_categories,
                status, response_deadline,
                ip_address, user_agent, identity_verified,
                created_at, updated_at, acknowledged_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                'received', $15, $16, $17, FALSE, $18, $18, $18
            )
            "#,
        )
        .bind(request_id)
        .bind(tenant_id)
        .bind(jurisdiction.to_string())
        .bind(right_type.to_string())
        .bind(user_id)
        .bind(email)
        .bind(name)
        .bind(document_type)
        .bind(document_number)
        .bind(is_representative)
        .bind(represented_name)
        .bind(specific_request)
        .bind(reason)
        .bind(&categories_json)
        .bind(response_deadline)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(DataRequest {
            request_id,
            tenant_id,
            jurisdiction,
            right_type,
            user_id,
            requester_email: email.to_string(),
            requester_name: name.to_string(),
            document_type: document_type.map(String::from),
            document_number: document_number.map(String::from),
            is_representative,
            represented_name: represented_name.map(String::from),
            authorization_document_url: None,
            data_categories: data_categories.clone(),
            specific_request: specific_request.to_string(),
            reason: reason.map(String::from),
            supporting_documents: None,
            status: RequestStatus::Received,
            response_deadline,
            response_decision: None,
            response_explanation: None,
            response_actions: None,
            export_url: None,
            export_expires_at: None,
            identity_verified: false,
            verified_at: None,
            verification_method: None,
            ip_address: ip_address.map(String::from),
            user_agent: user_agent.map(String::from),
            created_at: now,
            acknowledged_at: Some(now),
            updated_at: now,
            resolved_at: None,
        })
    }

    /// Obtiene una solicitud por ID
    pub async fn get_data_request(&self, request_id: Uuid) -> Result<DataRequest> {
        let row = sqlx::query(
            r#"
            SELECT
                request_id, tenant_id, jurisdiction, right_type,
                user_id, requester_email, requester_name,
                document_type, document_number,
                is_representative, represented_name, authorization_document_url,
                specific_request, reason, data_categories, supporting_documents,
                status, response_deadline,
                response_decision, response_explanation, response_actions,
                export_url, export_expires_at,
                identity_verified, verified_at, verification_method,
                ip_address, user_agent,
                created_at, acknowledged_at, updated_at, resolved_at
            FROM data_requests
            WHERE request_id = $1
            "#,
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Request {}", request_id)))?;

        self.row_to_data_request(row)
    }

    /// Lista solicitudes con filtros
    pub async fn list_data_requests(&self, filters: DataRequestFilters) -> Result<DataRequestListResponse> {
        let page = filters.page.unwrap_or(1).max(1);
        let page_size = filters.page_size.unwrap_or(20).min(100);
        let offset = ((page - 1) * page_size) as i64;

        // Count total
        let count_row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM data_requests
            WHERE
                ($1::text IS NULL OR jurisdiction = $1)
                AND ($2::text IS NULL OR right_type = $2)
                AND ($3::text IS NULL OR status = $3)
                AND ($4::uuid IS NULL OR user_id = $4)
                AND ($5::text IS NULL OR requester_email ILIKE '%' || $5 || '%')
                AND ($6::timestamptz IS NULL OR created_at >= $6)
                AND ($7::timestamptz IS NULL OR created_at <= $7)
            "#,
        )
        .bind(filters.jurisdiction.map(|j| j.to_string()))
        .bind(filters.right_type.map(|r| r.to_string()))
        .bind(filters.status.map(|s| s.to_string()))
        .bind(filters.user_id)
        .bind(&filters.email)
        .bind(filters.from_date)
        .bind(filters.to_date)
        .fetch_one(&self.pool)
        .await?;

        let total: i64 = count_row.get("count");
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        // Fetch requests
        let rows = sqlx::query(
            r#"
            SELECT
                request_id, tenant_id, jurisdiction, right_type,
                user_id, requester_email, requester_name,
                document_type, document_number,
                is_representative, represented_name, authorization_document_url,
                specific_request, reason, data_categories, supporting_documents,
                status, response_deadline,
                response_decision, response_explanation, response_actions,
                export_url, export_expires_at,
                identity_verified, verified_at, verification_method,
                ip_address, user_agent,
                created_at, acknowledged_at, updated_at, resolved_at
            FROM data_requests
            WHERE
                ($1::text IS NULL OR jurisdiction = $1)
                AND ($2::text IS NULL OR right_type = $2)
                AND ($3::text IS NULL OR status = $3)
                AND ($4::uuid IS NULL OR user_id = $4)
                AND ($5::text IS NULL OR requester_email ILIKE '%' || $5 || '%')
                AND ($6::timestamptz IS NULL OR created_at >= $6)
                AND ($7::timestamptz IS NULL OR created_at <= $7)
            ORDER BY created_at DESC
            LIMIT $8 OFFSET $9
            "#,
        )
        .bind(filters.jurisdiction.map(|j| j.to_string()))
        .bind(filters.right_type.map(|r| r.to_string()))
        .bind(filters.status.map(|s| s.to_string()))
        .bind(filters.user_id)
        .bind(&filters.email)
        .bind(filters.from_date)
        .bind(filters.to_date)
        .bind(page_size as i64)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut requests = Vec::new();
        for row in rows {
            requests.push(self.row_to_data_request(row)?);
        }

        Ok(DataRequestListResponse {
            requests,
            total,
            page,
            page_size,
            total_pages,
        })
    }

    /// Actualiza el estado de una solicitud
    pub async fn update_request_status(
        &self,
        request_id: Uuid,
        status: RequestStatus,
        decision: Option<&str>,
        explanation: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now();
        let resolved_at = if status == RequestStatus::Resolved || status == RequestStatus::Denied {
            Some(now)
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE data_requests
            SET status = $2, response_decision = $3, response_explanation = $4,
                updated_at = $5, resolved_at = $6
            WHERE request_id = $1
            "#,
        )
        .bind(request_id)
        .bind(status.to_string())
        .bind(decision)
        .bind(explanation)
        .bind(now)
        .bind(resolved_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Verifica la identidad de un solicitante
    pub async fn verify_identity(
        &self,
        request_id: Uuid,
        method: &str,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE data_requests
            SET identity_verified = TRUE, verified_at = $2, verification_method = $3,
                status = CASE WHEN status = 'identity_pending' THEN 'in_progress' ELSE status END,
                updated_at = $2
            WHERE request_id = $1
            "#,
        )
        .bind(request_id)
        .bind(now)
        .bind(method)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    fn row_to_data_request(&self, row: sqlx::postgres::PgRow) -> Result<DataRequest> {
        let jurisdiction_str: String = row.get("jurisdiction");
        let jurisdiction = match jurisdiction_str.as_str() {
            "colombia" => Jurisdiction::Colombia,
            "gdpr" => Jurisdiction::Gdpr,
            "ccpa" => Jurisdiction::Ccpa,
            "lgpd" => Jurisdiction::Lgpd,
            _ => Jurisdiction::General,
        };

        let right_type_str: String = row.get("right_type");
        let right_type = match right_type_str.as_str() {
            "access" => DataRightType::Access,
            "rectification" => DataRightType::Rectification,
            "erasure" => DataRightType::Erasure,
            "objection" => DataRightType::Objection,
            "restriction" => DataRightType::Restriction,
            "portability" => DataRightType::Portability,
            "automated_decision" => DataRightType::AutomatedDecision,
            "opt_out_sale" => DataRightType::OptOutSale,
            "opt_out_sharing" => DataRightType::OptOutSharing,
            "limit_sensitive" => DataRightType::LimitSensitive,
            "confirmation" => DataRightType::Confirmation,
            "anonymization" => DataRightType::Anonymization,
            "revoke_consent" => DataRightType::RevokeConsent,
            _ => DataRightType::Access,
        };

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "received" => RequestStatus::Received,
            "identity_pending" => RequestStatus::IdentityPending,
            "in_progress" => RequestStatus::InProgress,
            "awaiting_info" => RequestStatus::AwaitingInfo,
            "resolved" => RequestStatus::Resolved,
            "denied" => RequestStatus::Denied,
            "appealed" => RequestStatus::Appealed,
            "expired" => RequestStatus::Expired,
            _ => RequestStatus::Received,
        };

        let data_categories: Option<Vec<String>> = row.get::<Option<serde_json::Value>, _>("data_categories")
            .and_then(|v| serde_json::from_value(v).ok());

        let supporting_documents: Option<Vec<String>> = row.get::<Option<serde_json::Value>, _>("supporting_documents")
            .and_then(|v| serde_json::from_value(v).ok());

        let response_actions: Option<Vec<String>> = row.get::<Option<serde_json::Value>, _>("response_actions")
            .and_then(|v| serde_json::from_value(v).ok());

        Ok(DataRequest {
            request_id: row.get("request_id"),
            tenant_id: row.get("tenant_id"),
            jurisdiction,
            right_type,
            user_id: row.get("user_id"),
            requester_email: row.get("requester_email"),
            requester_name: row.get("requester_name"),
            document_type: row.get("document_type"),
            document_number: row.get("document_number"),
            is_representative: row.get("is_representative"),
            represented_name: row.get("represented_name"),
            authorization_document_url: row.get("authorization_document_url"),
            data_categories,
            specific_request: row.get("specific_request"),
            reason: row.get("reason"),
            supporting_documents,
            status,
            response_deadline: row.get("response_deadline"),
            response_decision: row.get("response_decision"),
            response_explanation: row.get("response_explanation"),
            response_actions,
            export_url: row.get("export_url"),
            export_expires_at: row.get("export_expires_at"),
            identity_verified: row.get("identity_verified"),
            verified_at: row.get("verified_at"),
            verification_method: row.get("verification_method"),
            ip_address: row.get("ip_address"),
            user_agent: row.get("user_agent"),
            created_at: row.get("created_at"),
            acknowledged_at: row.get("acknowledged_at"),
            updated_at: row.get("updated_at"),
            resolved_at: row.get("resolved_at"),
        })
    }

    // =========================================================================
    // Consentimientos
    // =========================================================================

    /// Guarda o actualiza preferencias de cookies
    pub async fn save_cookie_preferences(
        &self,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
        functional: bool,
        analytics: bool,
        marketing: bool,
        social_media: bool,
        policy_version: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<CookiePreferences> {
        let preference_id = Uuid::new_v4();
        let now = Utc::now();

        // Upsert basado en user_id o anonymous_id
        sqlx::query(
            r#"
            INSERT INTO cookie_preferences (
                preference_id, user_id, anonymous_id,
                essential, functional, analytics, marketing, social_media,
                policy_version, ip_address, user_agent,
                created_at, updated_at
            ) VALUES ($1, $2, $3, TRUE, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            ON CONFLICT (COALESCE(user_id, '00000000-0000-0000-0000-000000000000'::uuid), COALESCE(anonymous_id, ''))
            DO UPDATE SET
                functional = $4, analytics = $5, marketing = $6, social_media = $7,
                policy_version = $8, ip_address = $9, user_agent = $10, updated_at = $11
            "#,
        )
        .bind(preference_id)
        .bind(user_id)
        .bind(anonymous_id)
        .bind(functional)
        .bind(analytics)
        .bind(marketing)
        .bind(social_media)
        .bind(policy_version)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(CookiePreferences {
            preference_id,
            user_id,
            anonymous_id: anonymous_id.map(String::from),
            essential: true,
            functional,
            analytics,
            marketing,
            social_media,
            policy_version: policy_version.to_string(),
            ip_address: ip_address.map(String::from),
            user_agent: user_agent.map(String::from),
            created_at: now,
            updated_at: now,
        })
    }

    /// Obtiene preferencias de cookies
    pub async fn get_cookie_preferences(
        &self,
        user_id: Option<Uuid>,
        anonymous_id: Option<&str>,
    ) -> Result<Option<CookiePreferences>> {
        let row = sqlx::query(
            r#"
            SELECT
                preference_id, user_id, anonymous_id,
                essential, functional, analytics, marketing, social_media,
                policy_version, ip_address, user_agent,
                created_at, updated_at
            FROM cookie_preferences
            WHERE
                (user_id = $1 OR ($1 IS NULL AND anonymous_id = $2))
            ORDER BY updated_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(anonymous_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CookiePreferences {
            preference_id: r.get("preference_id"),
            user_id: r.get("user_id"),
            anonymous_id: r.get("anonymous_id"),
            essential: r.get("essential"),
            functional: r.get("functional"),
            analytics: r.get("analytics"),
            marketing: r.get("marketing"),
            social_media: r.get("social_media"),
            policy_version: r.get("policy_version"),
            ip_address: r.get("ip_address"),
            user_agent: r.get("user_agent"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
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
        let consent_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO consent_records (
                consent_id, tenant_id, user_id, anonymous_id,
                consent_type, granted, policy_version, source,
                ip_address, user_agent,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            "#,
        )
        .bind(consent_id)
        .bind(tenant_id)
        .bind(user_id)
        .bind(anonymous_id)
        .bind(consent_type)
        .bind(granted)
        .bind(policy_version)
        .bind(source)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(ConsentRecord {
            consent_id,
            tenant_id,
            user_id,
            anonymous_id: anonymous_id.map(String::from),
            consent_type: crate::domain::ConsentType::Terms, // Default, se parsea después
            granted,
            policy_version: policy_version.to_string(),
            ip_address: ip_address.map(String::from),
            user_agent: user_agent.map(String::from),
            source: source.to_string(),
            created_at: now,
            updated_at: now,
            expires_at: None,
            revoked_at: None,
        })
    }

    // =========================================================================
    // Data Exports
    // =========================================================================

    /// Crea una solicitud de exportación de datos
    pub async fn create_data_export(
        &self,
        user_id: Uuid,
        request_id: Option<Uuid>,
        format: &str,
        categories: Vec<String>,
    ) -> Result<DataExport> {
        let export_id = Uuid::new_v4();
        let now = Utc::now();
        let categories_json = serde_json::to_value(&categories)
            .map_err(|e| RepositoryError::Serialization(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO data_exports (
                export_id, user_id, request_id, format, categories,
                status, progress_percent, created_at
            ) VALUES ($1, $2, $3, $4, $5, 'queued', 0, $6)
            "#,
        )
        .bind(export_id)
        .bind(user_id)
        .bind(request_id)
        .bind(format)
        .bind(&categories_json)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(DataExport {
            export_id,
            user_id,
            request_id,
            format: format.to_string(),
            categories,
            status: ExportStatus::Queued,
            progress_percent: 0,
            download_url: None,
            file_size_bytes: None,
            checksum: None,
            created_at: now,
            processing_started_at: None,
            completed_at: None,
            expires_at: None,
            error_message: None,
        })
    }

    /// Actualiza el estado de una exportación
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
        let now = Utc::now();
        let completed_at = if status == ExportStatus::Ready { Some(now) } else { None };
        let expires_at = if status == ExportStatus::Ready {
            Some(now + Duration::hours(48))
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE data_exports
            SET status = $2, progress_percent = $3, download_url = $4,
                file_size_bytes = $5, checksum = $6, error_message = $7,
                completed_at = $8, expires_at = $9
            WHERE export_id = $1
            "#,
        )
        .bind(export_id)
        .bind(status.to_string())
        .bind(progress)
        .bind(download_url)
        .bind(file_size)
        .bind(checksum)
        .bind(error)
        .bind(completed_at)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Obtiene una exportación por ID
    pub async fn get_data_export(&self, export_id: Uuid) -> Result<DataExport> {
        let row = sqlx::query(
            r#"
            SELECT
                export_id, user_id, request_id, format, categories,
                status, progress_percent, download_url, file_size_bytes, checksum,
                created_at, processing_started_at, completed_at, expires_at, error_message
            FROM data_exports
            WHERE export_id = $1
            "#,
        )
        .bind(export_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| RepositoryError::NotFound(format!("Export {}", export_id)))?;

        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "queued" => ExportStatus::Queued,
            "processing" => ExportStatus::Processing,
            "ready" => ExportStatus::Ready,
            "expired" => ExportStatus::Expired,
            "failed" => ExportStatus::Failed,
            _ => ExportStatus::Queued,
        };

        let categories: Vec<String> = row.get::<Option<serde_json::Value>, _>("categories")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        Ok(DataExport {
            export_id: row.get("export_id"),
            user_id: row.get("user_id"),
            request_id: row.get("request_id"),
            format: row.get("format"),
            categories,
            status,
            progress_percent: row.get("progress_percent"),
            download_url: row.get("download_url"),
            file_size_bytes: row.get("file_size_bytes"),
            checksum: row.get("checksum"),
            created_at: row.get("created_at"),
            processing_started_at: row.get("processing_started_at"),
            completed_at: row.get("completed_at"),
            expires_at: row.get("expires_at"),
            error_message: row.get("error_message"),
        })
    }

    // =========================================================================
    // Deletion Requests
    // =========================================================================

    /// Crea una solicitud de eliminación de cuenta
    pub async fn create_deletion_request(
        &self,
        user_id: Uuid,
        request_id: Option<Uuid>,
        reason: Option<&str>,
        grace_period_days: i64,
    ) -> Result<DeletionRequest> {
        let deletion_id = Uuid::new_v4();
        let now = Utc::now();
        let scheduled_at = now + Duration::days(grace_period_days);

        sqlx::query(
            r#"
            INSERT INTO deletion_requests (
                deletion_id, user_id, request_id, reason,
                status, scheduled_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, 'received', $5, $6, $6)
            "#,
        )
        .bind(deletion_id)
        .bind(user_id)
        .bind(request_id)
        .bind(reason)
        .bind(scheduled_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(DeletionRequest {
            deletion_id,
            user_id,
            request_id,
            reason: reason.map(String::from),
            status: RequestStatus::Received,
            scheduled_at,
            services_notified: vec![],
            services_completed: vec![],
            services_failed: vec![],
            completed_at: None,
            cancelled_at: None,
            cancellation_reason: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Cancela una solicitud de eliminación
    pub async fn cancel_deletion_request(
        &self,
        deletion_id: Uuid,
        reason: &str,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE deletion_requests
            SET status = 'resolved', cancelled_at = $2, cancellation_reason = $3, updated_at = $2
            WHERE deletion_id = $1 AND status NOT IN ('resolved', 'denied')
            "#,
        )
        .bind(deletion_id)
        .bind(now)
        .bind(reason)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Audit Logs
    // =========================================================================

    /// Registra un evento de auditoría
    pub async fn log_audit_event(
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
        let log_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO compliance_audit_logs (
                log_id, tenant_id, actor_id, actor_type, actor_ip,
                action, resource_type, resource_id,
                subject_id, subject_email, details, jurisdiction,
                created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(log_id)
        .bind(tenant_id)
        .bind(actor_id)
        .bind(actor_type)
        .bind(actor_ip)
        .bind(action)
        .bind(resource_type)
        .bind(resource_id)
        .bind(subject_id)
        .bind(subject_email)
        .bind(&details)
        .bind(jurisdiction.map(|j| j.to_string()))
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(ComplianceAuditLog {
            log_id,
            tenant_id,
            actor_id,
            actor_type: actor_type.to_string(),
            actor_ip: actor_ip.map(String::from),
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id,
            subject_id,
            subject_email: subject_email.map(String::from),
            details,
            jurisdiction,
            created_at: now,
        })
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Obtiene estadísticas de compliance
    pub async fn get_compliance_stats(&self, tenant_id: Option<Uuid>) -> Result<ComplianceStats> {
        // Total y pendientes
        let totals = sqlx::query(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status NOT IN ('resolved', 'denied', 'expired')) as pending
            FROM data_requests
            WHERE ($1::uuid IS NULL OR tenant_id = $1)
            "#,
        )
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        let total_requests: i64 = totals.get("total");
        let pending_requests: i64 = totals.get("pending");

        // Por jurisdicción
        let by_jurisdiction_rows = sqlx::query(
            r#"
            SELECT jurisdiction, COUNT(*) as count
            FROM data_requests
            WHERE ($1::uuid IS NULL OR tenant_id = $1)
            GROUP BY jurisdiction
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let requests_by_jurisdiction: Vec<JurisdictionStats> = by_jurisdiction_rows
            .iter()
            .map(|r| JurisdictionStats {
                jurisdiction: r.get("jurisdiction"),
                count: r.get("count"),
            })
            .collect();

        // Por tipo
        let by_type_rows = sqlx::query(
            r#"
            SELECT right_type, COUNT(*) as count
            FROM data_requests
            WHERE ($1::uuid IS NULL OR tenant_id = $1)
            GROUP BY right_type
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let requests_by_type: Vec<TypeStats> = by_type_rows
            .iter()
            .map(|r| TypeStats {
                right_type: r.get("right_type"),
                count: r.get("count"),
            })
            .collect();

        // Por estado
        let by_status_rows = sqlx::query(
            r#"
            SELECT status, COUNT(*) as count
            FROM data_requests
            WHERE ($1::uuid IS NULL OR tenant_id = $1)
            GROUP BY status
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let requests_by_status: Vec<StatusStats> = by_status_rows
            .iter()
            .map(|r| StatusStats {
                status: r.get("status"),
                count: r.get("count"),
            })
            .collect();

        // Tiempo promedio de respuesta
        let avg_row = sqlx::query(
            r#"
            SELECT COALESCE(AVG(EXTRACT(DAY FROM (resolved_at - created_at))), 0) as avg_days
            FROM data_requests
            WHERE resolved_at IS NOT NULL
            AND ($1::uuid IS NULL OR tenant_id = $1)
            "#,
        )
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        let avg_response_days: f64 = avg_row.get("avg_days");

        // SLA compliance (resueltos dentro del plazo)
        let sla_row = sqlx::query(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE resolved_at <= response_deadline) as on_time,
                COUNT(*) as total_resolved
            FROM data_requests
            WHERE resolved_at IS NOT NULL
            AND ($1::uuid IS NULL OR tenant_id = $1)
            "#,
        )
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        let on_time: i64 = sla_row.get("on_time");
        let total_resolved: i64 = sla_row.get("total_resolved");
        let sla_compliance_percent = if total_resolved > 0 {
            (on_time as f64 / total_resolved as f64) * 100.0
        } else {
            100.0
        };

        Ok(ComplianceStats {
            total_requests,
            pending_requests,
            requests_by_jurisdiction,
            requests_by_type,
            requests_by_status,
            avg_response_days,
            sla_compliance_percent,
        })
    }
}
