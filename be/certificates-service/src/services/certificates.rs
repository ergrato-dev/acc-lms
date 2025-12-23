//! # Certificates Service
//!
//! Business logic for certificate generation and verification.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api::dto::*;
use crate::domain::entities::*;
use crate::domain::errors::CertificatesError;
use crate::repository::CertificatesRepository;

/// Service for certificates business logic.
#[derive(Clone)]
pub struct CertificatesService {
    repository: CertificatesRepository,
    base_url: String,
}

impl CertificatesService {
    /// Create a new service instance.
    pub fn new(repository: CertificatesRepository, base_url: String) -> Self {
        Self { repository, base_url }
    }

    /// Generate a unique verification code.
    fn generate_verification_code(&self) -> String {
        let year = Utc::now().format("%Y");
        let random_part = Uuid::new_v4().to_string().replace("-", "").to_uppercase();
        format!("ACC-{}-{}-{}", year, &random_part[..4], &random_part[4..8])
    }

    /// Build the verification URL.
    fn build_verification_url(&self, code: &str) -> String {
        format!("{}/verify/{}", self.base_url, code)
    }

    // =========================================================================
    // Certificate Operations
    // =========================================================================

    /// Generate a new certificate.
    pub async fn generate_certificate(
        &self,
        user_id: Uuid,
        course_id: Uuid,
        student_name: &str,
        course_title: &str,
        instructor_name: &str,
        completion_date: DateTime<Utc>,
        template_id: Option<Uuid>,
    ) -> Result<CertificateResponse, CertificatesError> {
        // Check if certificate already exists
        if let Some(existing) = self.repository.find_certificate_by_user_and_course(
            user_id, course_id
        ).await? {
            return Ok(self.certificate_to_response(&existing));
        }

        // Generate unique verification code
        let mut verification_code = self.generate_verification_code();
        let mut attempts = 0;
        while !self.repository.is_verification_code_unique(&verification_code).await? {
            verification_code = self.generate_verification_code();
            attempts += 1;
            if attempts > 10 {
                return Err(CertificatesError::Internal("Failed to generate unique verification code".into()));
            }
        }

        // Determine template to use
        let effective_template_id = if let Some(tid) = template_id {
            Some(tid)
        } else if let Some(course_template) = self.repository.find_template_for_course(course_id).await? {
            Some(course_template.template_id)
        } else if let Some(default_template) = self.repository.find_default_template().await? {
            Some(default_template.template_id)
        } else {
            None
        };

        // Create certificate record
        let certificate = self.repository.create_certificate(
            user_id,
            course_id,
            &verification_code,
            student_name,
            course_title,
            instructor_name,
            completion_date,
            effective_template_id,
        ).await?;

        // Generate PDF in background (for now, we'll skip actual PDF generation)
        // In production, this would trigger an async job or generate the PDF here
        // let pdf_url = self.generate_pdf(&certificate, effective_template_id).await?;
        // self.repository.update_certificate_pdf_url(certificate.certificate_id, &pdf_url).await?;

        Ok(self.certificate_to_response(&certificate))
    }

    /// Get a certificate by ID.
    pub async fn get_certificate(
        &self,
        certificate_id: Uuid,
    ) -> Result<CertificateResponse, CertificatesError> {
        let certificate = self.repository.find_certificate_by_id(certificate_id).await?
            .ok_or(CertificatesError::CertificateNotFound(certificate_id))?;

        Ok(self.certificate_to_response(&certificate))
    }

    /// Get certificate for a user and course.
    pub async fn get_user_course_certificate(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> Result<Option<CertificateResponse>, CertificatesError> {
        let certificate = self.repository.find_certificate_by_user_and_course(
            user_id, course_id
        ).await?;

        Ok(certificate.map(|c| self.certificate_to_response(&c)))
    }

    /// List certificates for a user.
    pub async fn list_user_certificates(
        &self,
        user_id: Uuid,
        page: i32,
        per_page: i32,
        status: Option<&str>,
    ) -> Result<PaginatedCertificatesResponse, CertificatesError> {
        let offset = (page - 1) * per_page;

        let certificates = self.repository.find_certificates_for_user(
            user_id, per_page, offset, status
        ).await?;

        let total = self.repository.count_certificates_for_user(user_id, status).await?;
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

        let items = certificates.into_iter().map(|c| {
            CertificateSummaryResponse {
                certificate_id: c.certificate_id,
                course_id: c.course_id,
                course_title: c.course_title,
                completion_date: c.completion_date,
                issued_at: c.issued_at,
                verification_code: c.verification_code.clone(),
                verification_url: self.build_verification_url(&c.verification_code),
                status: c.status,
            }
        }).collect();

        Ok(PaginatedCertificatesResponse {
            items,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    /// Get certificate PDF.
    pub async fn get_certificate_pdf(
        &self,
        certificate_id: Uuid,
    ) -> Result<Vec<u8>, CertificatesError> {
        let certificate = self.repository.find_certificate_by_id(certificate_id).await?
            .ok_or(CertificatesError::CertificateNotFound(certificate_id))?;

        if certificate.status == certificate_status::REVOKED {
            return Err(CertificatesError::CertificateRevoked(certificate_id));
        }

        // Generate PDF on-the-fly
        self.generate_simple_pdf(&certificate).await
    }

    /// Revoke a certificate.
    pub async fn revoke_certificate(
        &self,
        certificate_id: Uuid,
        _reason: Option<&str>,
    ) -> Result<(), CertificatesError> {
        let certificate = self.repository.find_certificate_by_id(certificate_id).await?
            .ok_or(CertificatesError::CertificateNotFound(certificate_id))?;

        if certificate.status == certificate_status::REVOKED {
            return Err(CertificatesError::CertificateRevoked(certificate_id));
        }

        self.repository.revoke_certificate(certificate_id).await?;

        Ok(())
    }

    /// Verify a certificate by code.
    pub async fn verify_certificate(
        &self,
        verification_code: &str,
    ) -> VerificationResponse {
        // Validate code format
        if !verification_code.starts_with("ACC-") || verification_code.len() < 15 {
            return VerificationResponse {
                is_valid: false,
                certificate_id: None,
                verification_code: verification_code.to_string(),
                student_name: None,
                course_title: None,
                instructor_name: None,
                completion_date: None,
                issued_at: None,
                status: None,
                message: "Invalid verification code format".to_string(),
            };
        }

        match self.repository.find_certificate_by_code(verification_code).await {
            Ok(Some(cert)) => {
                let is_valid = cert.status == certificate_status::ACTIVE;
                let message = match cert.status.as_str() {
                    "active" => "Certificate is valid and active",
                    "revoked" => "Certificate has been revoked",
                    "expired" => "Certificate has expired",
                    _ => "Unknown certificate status",
                };

                VerificationResponse {
                    is_valid,
                    certificate_id: Some(cert.certificate_id),
                    verification_code: cert.verification_code,
                    student_name: Some(cert.student_name),
                    course_title: Some(cert.course_title),
                    instructor_name: Some(cert.instructor_name),
                    completion_date: Some(cert.completion_date),
                    issued_at: Some(cert.issued_at),
                    status: Some(cert.status),
                    message: message.to_string(),
                }
            }
            Ok(None) => VerificationResponse {
                is_valid: false,
                certificate_id: None,
                verification_code: verification_code.to_string(),
                student_name: None,
                course_title: None,
                instructor_name: None,
                completion_date: None,
                issued_at: None,
                status: None,
                message: "Certificate not found".to_string(),
            },
            Err(_) => VerificationResponse {
                is_valid: false,
                certificate_id: None,
                verification_code: verification_code.to_string(),
                student_name: None,
                course_title: None,
                instructor_name: None,
                completion_date: None,
                issued_at: None,
                status: None,
                message: "Error verifying certificate".to_string(),
            },
        }
    }

    // =========================================================================
    // Template Operations
    // =========================================================================

    /// List all templates.
    pub async fn list_templates(
        &self,
    ) -> Result<TemplatesListResponse, CertificatesError> {
        let templates = self.repository.list_templates().await?;
        let total = templates.len() as i64;

        let items = templates.into_iter().map(|t| self.template_to_response(&t)).collect();

        Ok(TemplatesListResponse { items, total })
    }

    /// Create a template.
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
    ) -> Result<TemplateResponse, CertificatesError> {
        let template = self.repository.create_template(
            name, description, course_id, background_url, logo_url,
            primary_color, secondary_color, font_family, is_default
        ).await?;

        Ok(self.template_to_response(&template))
    }

    /// Get a template by ID.
    pub async fn get_template(
        &self,
        template_id: Uuid,
    ) -> Result<TemplateResponse, CertificatesError> {
        let template = self.repository.find_template_by_id(template_id).await?
            .ok_or(CertificatesError::TemplateNotFound(template_id))?;

        Ok(self.template_to_response(&template))
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
    ) -> Result<TemplateResponse, CertificatesError> {
        // Verify template exists
        self.repository.find_template_by_id(template_id).await?
            .ok_or(CertificatesError::TemplateNotFound(template_id))?;

        let template = self.repository.update_template(
            template_id, name, description, background_url, logo_url,
            primary_color, secondary_color, font_family, is_default, is_active
        ).await?;

        Ok(self.template_to_response(&template))
    }

    /// Delete a template.
    pub async fn delete_template(
        &self,
        template_id: Uuid,
    ) -> Result<(), CertificatesError> {
        // Verify template exists
        self.repository.find_template_by_id(template_id).await?
            .ok_or(CertificatesError::TemplateNotFound(template_id))?;

        self.repository.delete_template(template_id).await?;

        Ok(())
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Convert certificate entity to response DTO.
    fn certificate_to_response(&self, cert: &Certificate) -> CertificateResponse {
        CertificateResponse {
            certificate_id: cert.certificate_id,
            user_id: cert.user_id,
            course_id: cert.course_id,
            verification_code: cert.verification_code.clone(),
            verification_url: self.build_verification_url(&cert.verification_code),
            student_name: cert.student_name.clone(),
            course_title: cert.course_title.clone(),
            instructor_name: cert.instructor_name.clone(),
            completion_date: cert.completion_date,
            issued_at: cert.issued_at,
            expires_at: cert.expires_at,
            pdf_url: cert.pdf_url.clone(),
            status: cert.status.clone(),
        }
    }

    /// Convert template entity to response DTO.
    fn template_to_response(&self, template: &CertificateTemplate) -> TemplateResponse {
        TemplateResponse {
            template_id: template.template_id,
            course_id: template.course_id,
            name: template.name.clone(),
            description: template.description.clone(),
            background_url: template.background_url.clone(),
            logo_url: template.logo_url.clone(),
            primary_color: template.primary_color.clone(),
            secondary_color: template.secondary_color.clone(),
            font_family: template.font_family.clone(),
            is_default: template.is_default,
            is_active: template.is_active,
            created_at: template.created_at,
            updated_at: template.updated_at,
        }
    }

    /// Generate a simple PDF certificate.
    async fn generate_simple_pdf(
        &self,
        certificate: &Certificate,
    ) -> Result<Vec<u8>, CertificatesError> {
        use printpdf::*;

        // Create a new PDF document
        let (doc, page1, layer1) = PdfDocument::new(
            "Certificate of Completion",
            Mm(297.0), // A4 landscape width
            Mm(210.0), // A4 landscape height
            "Layer 1",
        );

        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Use built-in font
        let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
            .map_err(|e| CertificatesError::PdfGenerationFailed(e.to_string()))?;

        let bold_font = doc.add_builtin_font(BuiltinFont::TimesBold)
            .map_err(|e| CertificatesError::PdfGenerationFailed(e.to_string()))?;

        // Title
        current_layer.use_text(
            "Certificate of Completion",
            36.0,
            Mm(148.5),
            Mm(180.0),
            &bold_font,
        );

        // This certifies that
        current_layer.use_text(
            "This certifies that",
            14.0,
            Mm(148.5),
            Mm(150.0),
            &font,
        );

        // Student name
        current_layer.use_text(
            &certificate.student_name,
            28.0,
            Mm(148.5),
            Mm(130.0),
            &bold_font,
        );

        // Has successfully completed
        current_layer.use_text(
            "has successfully completed",
            14.0,
            Mm(148.5),
            Mm(110.0),
            &font,
        );

        // Course title
        current_layer.use_text(
            &certificate.course_title,
            22.0,
            Mm(148.5),
            Mm(90.0),
            &bold_font,
        );

        // Instructor
        let instructor_text = format!("Instructor: {}", certificate.instructor_name);
        current_layer.use_text(
            &instructor_text,
            12.0,
            Mm(148.5),
            Mm(60.0),
            &font,
        );

        // Completion date
        let date_text = format!("Completed on: {}", certificate.completion_date.format("%B %d, %Y"));
        current_layer.use_text(
            &date_text,
            12.0,
            Mm(148.5),
            Mm(45.0),
            &font,
        );

        // Verification code
        let code_text = format!("Verification Code: {}", certificate.verification_code);
        current_layer.use_text(
            &code_text,
            10.0,
            Mm(148.5),
            Mm(25.0),
            &font,
        );

        // Verification URL
        let url_text = format!("Verify at: {}", self.build_verification_url(&certificate.verification_code));
        current_layer.use_text(
            &url_text,
            8.0,
            Mm(148.5),
            Mm(15.0),
            &font,
        );

        // Save PDF to bytes
        let pdf_bytes = doc.save_to_bytes()
            .map_err(|e| CertificatesError::PdfGenerationFailed(e.to_string()))?;

        Ok(pdf_bytes)
    }
}
