//! # Export Service
//!
//! Business logic for exporting grades and transcripts.

use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use crate::api::ExportResponse;
use crate::domain::{ExportFormat, GradeError};
use crate::repository::{GradeRepository, TranscriptRepository};

/// Service for export operations.
pub struct ExportService {
    grade_repo: Arc<GradeRepository>,
    transcript_repo: Arc<TranscriptRepository>,
}

impl ExportService {
    /// Create a new export service.
    pub fn new(
        grade_repo: Arc<GradeRepository>,
        transcript_repo: Arc<TranscriptRepository>,
    ) -> Self {
        Self {
            grade_repo,
            transcript_repo,
        }
    }

    /// Export transcript for a user.
    pub async fn export_transcript(
        &self,
        user_id: Uuid,
        format: ExportFormat,
    ) -> Result<ExportResponse, GradeError> {
        // Get user info
        let user_info = self.transcript_repo.get_user_info(user_id).await?;

        // Get transcript entries
        let entries = self.transcript_repo.get_transcript_entries(user_id).await?;

        match format {
            ExportFormat::Csv => {
                let content = self.transcript_to_csv(&user_info.full_name, &entries)?;
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

                Ok(ExportResponse {
                    filename: format!("transcript_{}_{}.csv", user_id, timestamp),
                    content_type: "text/csv".to_string(),
                    size_bytes: content.len(),
                    content: Some(content),
                })
            }
            ExportFormat::Json => {
                let data = serde_json::json!({
                    "user_id": user_id,
                    "user_name": user_info.full_name,
                    "generated_at": Utc::now(),
                    "entries": entries.iter().map(|e| {
                        serde_json::json!({
                            "course_id": e.course_id,
                            "course_title": e.course_title,
                            "instructor": e.instructor_name,
                            "enrollment_date": e.enrollment_date,
                            "completion_date": e.completion_date,
                            "progress": e.progress_percentage,
                            "status": e.enrollment_status,
                            "certificate_issued": e.certificate_issued
                        })
                    }).collect::<Vec<_>>()
                });
                let content = serde_json::to_string_pretty(&data)
                    .map_err(|e| GradeError::Export(e.to_string()))?;
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

                Ok(ExportResponse {
                    filename: format!("transcript_{}_{}.json", user_id, timestamp),
                    content_type: "application/json".to_string(),
                    size_bytes: content.len(),
                    content: Some(content),
                })
            }
            ExportFormat::Pdf => {
                // PDF generation would require additional library
                Err(GradeError::Export("PDF export not yet implemented".to_string()))
            }
        }
    }

    /// Export all grades for a course.
    pub async fn export_course_grades(
        &self,
        course_id: Uuid,
        format: ExportFormat,
    ) -> Result<ExportResponse, GradeError> {
        let grades = self.grade_repo.get_all_course_grades(course_id).await?;

        match format {
            ExportFormat::Csv => {
                let content = self.course_grades_to_csv(&grades)?;
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

                Ok(ExportResponse {
                    filename: format!("course_grades_{}_{}.csv", course_id, timestamp),
                    content_type: "text/csv".to_string(),
                    size_bytes: content.len(),
                    content: Some(content),
                })
            }
            ExportFormat::Json => {
                let data = serde_json::json!({
                    "course_id": course_id,
                    "exported_at": Utc::now(),
                    "grades": grades.iter().map(|g| {
                        serde_json::json!({
                            "student_name": g.student_name,
                            "student_email": g.student_email,
                            "quiz_title": g.quiz_title,
                            "score": g.score,
                            "max_score": g.max_score,
                            "percentage": g.percentage,
                            "passed": g.passed,
                            "status": g.status,
                            "submitted_at": g.submitted_at
                        })
                    }).collect::<Vec<_>>()
                });
                let content = serde_json::to_string_pretty(&data)
                    .map_err(|e| GradeError::Export(e.to_string()))?;
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

                Ok(ExportResponse {
                    filename: format!("course_grades_{}_{}.json", course_id, timestamp),
                    content_type: "application/json".to_string(),
                    size_bytes: content.len(),
                    content: Some(content),
                })
            }
            ExportFormat::Pdf => {
                Err(GradeError::Export("PDF export not yet implemented".to_string()))
            }
        }
    }

    /// Convert transcript entries to CSV.
    fn transcript_to_csv(
        &self,
        user_name: &str,
        entries: &[crate::repository::transcript_repository::TranscriptEntryRow],
    ) -> Result<String, GradeError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record(&[
            "Student Name",
            "Course",
            "Instructor",
            "Enrollment Date",
            "Completion Date",
            "Progress %",
            "Status",
            "Certificate",
        ])?;

        // Write rows
        for entry in entries {
            wtr.write_record(&[
                user_name.to_string(),
                entry.course_title.clone(),
                entry.instructor_name.clone(),
                entry.enrollment_date.to_rfc3339(),
                entry.completion_date.map(|d| d.to_rfc3339()).unwrap_or_default(),
                entry.progress_percentage.to_string(),
                entry.enrollment_status.clone(),
                if entry.certificate_issued { "Yes" } else { "No" }.to_string(),
            ])?;
        }

        let data = wtr.into_inner()
            .map_err(|e| GradeError::Export(e.to_string()))?;

        String::from_utf8(data)
            .map_err(|e| GradeError::Export(e.to_string()))
    }

    /// Convert course grades to CSV.
    fn course_grades_to_csv(
        &self,
        grades: &[crate::repository::grade_repository::StudentGradeRow],
    ) -> Result<String, GradeError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record(&[
            "Student Name",
            "Email",
            "Quiz",
            "Score",
            "Max Score",
            "Percentage",
            "Passed",
            "Status",
            "Submitted At",
        ])?;

        // Write rows
        for grade in grades {
            wtr.write_record(&[
                grade.student_name.clone(),
                grade.student_email.clone(),
                grade.quiz_title.clone(),
                grade.score.to_string(),
                grade.max_score.to_string(),
                format!("{:.2}%", grade.percentage),
                grade.passed.map(|p| if p { "Yes" } else { "No" }).unwrap_or("Pending").to_string(),
                grade.status.clone(),
                grade.submitted_at.map(|d| d.to_rfc3339()).unwrap_or_default(),
            ])?;
        }

        let data = wtr.into_inner()
            .map_err(|e| GradeError::Export(e.to_string()))?;

        String::from_utf8(data)
            .map_err(|e| GradeError::Export(e.to_string()))
    }
}
