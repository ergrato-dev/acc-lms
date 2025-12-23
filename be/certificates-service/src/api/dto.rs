//! # Data Transfer Objects
//!
//! Request and response DTOs for the certificates API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// Request DTOs
// =============================================================================

/// Request to generate a certificate.
#[derive(Debug, Deserialize, Validate)]
pub struct GenerateCertificateRequest {
    pub user_id: Uuid,
    pub course_id: Uuid,
    #[validate(length(min = 2, max = 200, message = "Student name must be 2-200 characters"))]
    pub student_name: String,
    #[validate(length(min = 2, max = 300, message = "Course title must be 2-300 characters"))]
    pub course_title: String,
    #[validate(length(min = 2, max = 200, message = "Instructor name must be 2-200 characters"))]
    pub instructor_name: String,
    pub completion_date: DateTime<Utc>,
    pub template_id: Option<Uuid>,
}

/// Request to create a certificate template.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTemplateRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub name: String,
    pub description: Option<String>,
    pub course_id: Option<Uuid>,
    pub background_url: Option<String>,
    pub logo_url: Option<String>,
    #[validate(length(min = 4, max = 9, message = "Color must be valid hex format"))]
    #[serde(default = "default_primary_color")]
    pub primary_color: String,
    #[validate(length(min = 4, max = 9, message = "Color must be valid hex format"))]
    #[serde(default = "default_secondary_color")]
    pub secondary_color: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default)]
    pub is_default: bool,
}

/// Request to update a certificate template.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTemplateRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub background_url: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub font_family: Option<String>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}

/// Request to revoke a certificate.
#[derive(Debug, Deserialize)]
pub struct RevokeCertificateRequest {
    pub reason: Option<String>,
    pub revoked_by: Uuid,
}

/// Query parameters for listing certificates.
#[derive(Debug, Deserialize)]
pub struct ListCertificatesQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    pub status: Option<String>,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// Response for a generated certificate.
#[derive(Debug, Serialize)]
pub struct CertificateResponse {
    pub certificate_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub verification_code: String,
    pub verification_url: String,
    pub student_name: String,
    pub course_title: String,
    pub instructor_name: String,
    pub completion_date: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub status: String,
}

/// Response for certificate verification.
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
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

/// Response for a template.
#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub template_id: Uuid,
    pub course_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub background_url: Option<String>,
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Response for paginated certificates.
#[derive(Debug, Serialize)]
pub struct PaginatedCertificatesResponse {
    pub items: Vec<CertificateSummaryResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Brief certificate info for lists.
#[derive(Debug, Serialize)]
pub struct CertificateSummaryResponse {
    pub certificate_id: Uuid,
    pub course_id: Uuid,
    pub course_title: String,
    pub completion_date: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub verification_code: String,
    pub verification_url: String,
    pub status: String,
}

/// Response for list of templates.
#[derive(Debug, Serialize)]
pub struct TemplatesListResponse {
    pub items: Vec<TemplateResponse>,
    pub total: i64,
}

// =============================================================================
// Default Functions
// =============================================================================

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

fn default_primary_color() -> String {
    "#1a365d".to_string()
}

fn default_secondary_color() -> String {
    "#c69c6d".to_string()
}

fn default_font_family() -> String {
    "serif".to_string()
}
