//! # Certificates Repository
//!
//! Database operations for certificates and templates.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::*;
use crate::domain::errors::CertificatesError;

/// Repository for certificates database operations.
#[derive(Clone)]
pub struct CertificatesRepository {
    pool: PgPool,
}

impl CertificatesRepository {
    /// Create a new repository instance.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // Certificate Operations
    // =========================================================================

    /// Find a certificate by ID.
    pub async fn find_certificate_by_id(
        &self,
        certificate_id: Uuid,
    ) -> Result<Option<Certificate>, CertificatesError> {
        let certificate = sqlx::query_as::<_, Certificate>(
            "SELECT * FROM certificates WHERE certificate_id = $1"
        )
            .bind(certificate_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(certificate)
    }

    /// Find a certificate by verification code.
    pub async fn find_certificate_by_code(
        &self,
        verification_code: &str,
    ) -> Result<Option<Certificate>, CertificatesError> {
        let certificate = sqlx::query_as::<_, Certificate>(
            "SELECT * FROM certificates WHERE verification_code = $1"
        )
            .bind(verification_code)
            .fetch_optional(&self.pool)
            .await?;

        Ok(certificate)
    }

    /// Find certificate for a specific user and course.
    pub async fn find_certificate_by_user_and_course(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<Option<Certificate>, CertificatesError> {
        let certificate = sqlx::query_as::<_, Certificate>(
            r#"
            SELECT * FROM certificates
            WHERE user_id = $1 AND course_id = $2 AND status = 'active'
            "#
        )
            .bind(user_id)
            .bind(course_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(certificate)
    }

    /// Find certificates for a user with pagination.
    pub async fn find_certificates_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
        status: Option<&str>,
    ) -> Result<Vec<CertificateSummary>, CertificatesError> {
        let certificates = if let Some(status_filter) = status {
            sqlx::query_as::<_, CertificateSummary>(
                r#"
                SELECT
                    certificate_id, course_id, course_title,
                    completion_date, issued_at, verification_code, status
                FROM certificates
                WHERE user_id = $1 AND status = $4
                ORDER BY issued_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .bind(status_filter)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, CertificateSummary>(
                r#"
                SELECT
                    certificate_id, course_id, course_title,
                    completion_date, issued_at, verification_code, status
                FROM certificates
                WHERE user_id = $1
                ORDER BY issued_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
                .bind(user_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(certificates)
    }

    /// Count certificates for a user.
    pub async fn count_certificates_for_user(
        &self,
        user_id: Uuid,
        status: Option<&str>,
    ) -> Result<i64, CertificatesError> {
        let count: (i64,) = if let Some(status_filter) = status {
            sqlx::query_as(
                "SELECT COUNT(*) FROM certificates WHERE user_id = $1 AND status = $2"
            )
                .bind(user_id)
                .bind(status_filter)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_as(
                "SELECT COUNT(*) FROM certificates WHERE user_id = $1"
            )
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?
        };

        Ok(count.0)
    }

    /// Create a new certificate.
    pub async fn create_certificate(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        verification_code: &str,
        student_name: &str,
        course_title: &str,
        instructor_name: &str,
        completion_date: DateTime<Utc>,
        template_id: Option<Uuid>,
    ) -> Result<Certificate, CertificatesError> {
        let certificate = sqlx::query_as::<_, Certificate>(
            r#"
            INSERT INTO certificates (
                user_id, course_id, verification_code, student_name,
                course_title, instructor_name, completion_date,
                issued_at, status, template_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), 'active', $8)
            RETURNING *
            "#
        )
            .bind(user_id)
            .bind(course_id)
            .bind(verification_code)
            .bind(student_name)
            .bind(course_title)
            .bind(instructor_name)
            .bind(completion_date)
            .bind(template_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(certificate)
    }

    /// Update certificate PDF URL.
    pub async fn update_certificate_pdf_url(
        &self,
        certificate_id: Uuid,
        pdf_url: &str,
    ) -> Result<(), CertificatesError> {
        sqlx::query(
            r#"
            UPDATE certificates
            SET pdf_url = $2, updated_at = NOW()
            WHERE certificate_id = $1
            "#
        )
            .bind(certificate_id)
            .bind(pdf_url)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Revoke a certificate.
    pub async fn revoke_certificate(
        &self,
        certificate_id: Uuid,
    ) -> Result<(), CertificatesError> {
        sqlx::query(
            r#"
            UPDATE certificates
            SET status = 'revoked', updated_at = NOW()
            WHERE certificate_id = $1
            "#
        )
            .bind(certificate_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // =========================================================================
    // Template Operations
    // =========================================================================

    /// Find a template by ID.
    pub async fn find_template_by_id(
        &self,
        template_id: Uuid,
    ) -> Result<Option<CertificateTemplate>, CertificatesError> {
        let template = sqlx::query_as::<_, CertificateTemplate>(
            "SELECT * FROM certificate_templates WHERE template_id = $1"
        )
            .bind(template_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(template)
    }

    /// Find the default template.
    pub async fn find_default_template(
        &self,
    ) -> Result<Option<CertificateTemplate>, CertificatesError> {
        let template = sqlx::query_as::<_, CertificateTemplate>(
            "SELECT * FROM certificate_templates WHERE is_default = true AND is_active = true LIMIT 1"
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(template)
    }

    /// Find template for a specific course.
    pub async fn find_template_for_course(
        &self,
        course_id: Uuid,
    ) -> Result<Option<CertificateTemplate>, CertificatesError> {
        let template = sqlx::query_as::<_, CertificateTemplate>(
            "SELECT * FROM certificate_templates WHERE course_id = $1 AND is_active = true LIMIT 1"
        )
            .bind(course_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(template)
    }

    /// List all active templates.
    pub async fn list_templates(
        &self,
    ) -> Result<Vec<CertificateTemplate>, CertificatesError> {
        let templates = sqlx::query_as::<_, CertificateTemplate>(
            "SELECT * FROM certificate_templates WHERE is_active = true ORDER BY is_default DESC, name ASC"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(templates)
    }

    /// Create a new template.
    pub async fn create_template(
        &self,
        name: &str,
        description: Option<&str>,
        course_id: Option<Uuid>,
        background_url: Option<&str>,
        logo_url: Option<&str>,
        primary_color: &str,
        secondary_color: &str,
        font_family: &str,
        is_default: bool,
    ) -> Result<CertificateTemplate, CertificatesError> {
        // If this is set as default, unset other defaults first
        if is_default {
            sqlx::query("UPDATE certificate_templates SET is_default = false WHERE is_default = true")
                .execute(&self.pool)
                .await?;
        }

        let template = sqlx::query_as::<_, CertificateTemplate>(
            r#"
            INSERT INTO certificate_templates (
                name, description, course_id, background_url, logo_url,
                primary_color, secondary_color, font_family, is_default, is_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true)
            RETURNING *
            "#
        )
            .bind(name)
            .bind(description)
            .bind(course_id)
            .bind(background_url)
            .bind(logo_url)
            .bind(primary_color)
            .bind(secondary_color)
            .bind(font_family)
            .bind(is_default)
            .fetch_one(&self.pool)
            .await?;

        Ok(template)
    }

    /// Update a template.
    pub async fn update_template(
        &self,
        template_id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        background_url: Option<&str>,
        logo_url: Option<&str>,
        primary_color: Option<&str>,
        secondary_color: Option<&str>,
        font_family: Option<&str>,
        is_default: Option<bool>,
        is_active: Option<bool>,
    ) -> Result<CertificateTemplate, CertificatesError> {
        // If setting as default, unset other defaults
        if let Some(true) = is_default {
            sqlx::query("UPDATE certificate_templates SET is_default = false WHERE is_default = true")
                .execute(&self.pool)
                .await?;
        }

        let template = sqlx::query_as::<_, CertificateTemplate>(
            r#"
            UPDATE certificate_templates SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                background_url = COALESCE($4, background_url),
                logo_url = COALESCE($5, logo_url),
                primary_color = COALESCE($6, primary_color),
                secondary_color = COALESCE($7, secondary_color),
                font_family = COALESCE($8, font_family),
                is_default = COALESCE($9, is_default),
                is_active = COALESCE($10, is_active),
                updated_at = NOW()
            WHERE template_id = $1
            RETURNING *
            "#
        )
            .bind(template_id)
            .bind(name)
            .bind(description)
            .bind(background_url)
            .bind(logo_url)
            .bind(primary_color)
            .bind(secondary_color)
            .bind(font_family)
            .bind(is_default)
            .bind(is_active)
            .fetch_one(&self.pool)
            .await?;

        Ok(template)
    }

    /// Soft delete a template.
    pub async fn delete_template(
        &self,
        template_id: Uuid,
    ) -> Result<(), CertificatesError> {
        sqlx::query(
            r#"
            UPDATE certificate_templates
            SET is_active = false, updated_at = NOW()
            WHERE template_id = $1
            "#
        )
            .bind(template_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Check if verification code is unique.
    pub async fn is_verification_code_unique(
        &self,
        code: &str,
    ) -> Result<bool, CertificatesError> {
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM certificates WHERE verification_code = $1)"
        )
            .bind(code)
            .fetch_one(&self.pool)
            .await?;

        Ok(!exists.0)
    }
}
