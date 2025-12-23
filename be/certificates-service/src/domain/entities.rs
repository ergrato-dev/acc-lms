//! # Domain Entities
//!
//! Core entities for certificates.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// Certificate Entity
// =============================================================================

/// A certificate issued to a student upon course completion.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Certificate {
    pub certificate_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    /// Unique verification code (e.g., "ACC-2024-XXXX-XXXX")
    pub verification_code: String,
    /// Student name at time of issuance
    pub student_name: String,
    /// Course title at time of issuance
    pub course_title: String,
    /// Instructor name at time of issuance
    pub instructor_name: String,
    /// Date the course was completed
    pub completion_date: DateTime<Utc>,
    /// Date the certificate was issued
    pub issued_at: DateTime<Utc>,
    /// Optional expiration date
    pub expires_at: Option<DateTime<Utc>>,
    /// PDF blob stored in database (or path to storage)
    pub pdf_url: Option<String>,
    /// Certificate status
    pub status: String,
    /// Template used for this certificate
    pub template_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Certificate status enum values.
pub mod certificate_status {
    pub const ACTIVE: &str = "active";
    pub const REVOKED: &str = "revoked";
    pub const EXPIRED: &str = "expired";
}

// =============================================================================
// Certificate Template Entity
// =============================================================================

/// Template for generating certificates.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CertificateTemplate {
    pub template_id: Uuid,
    /// Optional: specific to a course
    pub course_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    /// Background image URL
    pub background_url: Option<String>,
    /// Logo URL
    pub logo_url: Option<String>,
    /// Primary color (hex)
    pub primary_color: String,
    /// Secondary color (hex)
    pub secondary_color: String,
    /// Font family name
    pub font_family: String,
    /// Whether this is the default template
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// Certificate Verification
// =============================================================================

/// Result of verifying a certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateVerification {
    pub is_valid: bool,
    pub certificate_id: Option<Uuid>,
    pub verification_code: String,
    pub student_name: Option<String>,
    pub course_title: Option<String>,
    pub instructor_name: Option<String>,
    pub completion_date: Option<DateTime<Utc>>,
    pub issued_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub message: String,
}

impl CertificateVerification {
    /// Create a valid verification result.
    pub fn valid(cert: &Certificate) -> Self {
        Self {
            is_valid: true,
            certificate_id: Some(cert.certificate_id),
            verification_code: cert.verification_code.clone(),
            student_name: Some(cert.student_name.clone()),
            course_title: Some(cert.course_title.clone()),
            instructor_name: Some(cert.instructor_name.clone()),
            completion_date: Some(cert.completion_date),
            issued_at: Some(cert.issued_at),
            status: Some(cert.status.clone()),
            message: "Certificate is valid".to_string(),
        }
    }

    /// Create an invalid verification result.
    pub fn invalid(code: &str, message: &str) -> Self {
        Self {
            is_valid: false,
            certificate_id: None,
            verification_code: code.to_string(),
            student_name: None,
            course_title: None,
            instructor_name: None,
            completion_date: None,
            issued_at: None,
            status: None,
            message: message.to_string(),
        }
    }
}

// =============================================================================
// Certificate Generation Request
// =============================================================================

/// Data needed to generate a certificate.
#[derive(Debug, Clone)]
pub struct CertificateGenerationData {
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub student_name: String,
    pub course_title: String,
    pub instructor_name: String,
    pub completion_date: DateTime<Utc>,
    pub template_id: Option<Uuid>,
}

// =============================================================================
// User Certificate Summary
// =============================================================================

/// Summary of certificates for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCertificatesSummary {
    pub user_id: Uuid,
    pub total_certificates: i64,
    pub certificates: Vec<CertificateSummary>,
}

/// Brief certificate info for lists.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CertificateSummary {
    pub certificate_id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub completion_date: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub verification_code: String,
    pub status: String,
}

// =============================================================================
// Pagination
// =============================================================================

/// Paginated certificates response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCertificates {
    pub items: Vec<CertificateSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}
