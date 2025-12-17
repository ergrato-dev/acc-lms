//! # Transcript Service
//!
//! Business logic for academic transcripts.

use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{
    GradeError, LetterGrade, Transcript, TranscriptEntryStatus,
    TranscriptSummary,
};
use crate::repository::{GradeRepository, TranscriptRepository};

/// Service for transcript operations.
pub struct TranscriptService {
    transcript_repo: Arc<TranscriptRepository>,
    grade_repo: Arc<GradeRepository>,
}

impl TranscriptService {
    /// Create a new transcript service.
    pub fn new(
        transcript_repo: Arc<TranscriptRepository>,
        grade_repo: Arc<GradeRepository>,
    ) -> Self {
        Self {
            transcript_repo,
            grade_repo,
        }
    }

    /// Generate complete transcript for a user.
    pub async fn get_transcript(&self, user_id: Uuid) -> Result<Transcript, GradeError> {
        // Get user info
        let user_info = self.transcript_repo.get_user_info(user_id).await?;

        // Get raw transcript entries
        let entry_rows = self.transcript_repo.get_transcript_entries(user_id).await?;

        // Enrich entries with grade information
        let mut entries = Vec::new();
        let mut total_gpa_points = Decimal::ZERO;
        let mut courses_with_grades = 0;
        let mut courses_completed = 0;
        let mut courses_in_progress = 0;
        let mut total_certificates = 0;

        for row in entry_rows {
            let course_id = row.course_id;
            let certificate_issued = row.certificate_issued;

            // Get grade summary for this course
            let grade_summary = self.transcript_repo
                .get_enrollment_grade_summary(user_id, course_id)
                .await?;

            let grade_percentage = grade_summary.percentage();
            let entry = row.to_transcript_entry(grade_percentage);

            // Track summary stats
            match entry.status {
                TranscriptEntryStatus::Completed => courses_completed += 1,
                TranscriptEntryStatus::InProgress => courses_in_progress += 1,
                _ => {}
            }

            if certificate_issued {
                total_certificates += 1;
            }

            // Add to GPA calculation if graded
            if grade_summary.graded_count > 0 {
                total_gpa_points += entry.letter_grade.gpa_points();
                courses_with_grades += 1;
            }

            entries.push(entry);
        }

        // Calculate overall GPA
        let overall_gpa = if courses_with_grades > 0 {
            total_gpa_points / Decimal::new(courses_with_grades, 0)
        } else {
            Decimal::ZERO
        };

        let overall_letter_grade = if courses_with_grades > 0 {
            // Map GPA back to letter grade
            Self::gpa_to_letter_grade(overall_gpa)
        } else {
            LetterGrade::F
        };

        let summary = TranscriptSummary {
            total_courses_enrolled: entries.len() as i32,
            courses_completed,
            courses_in_progress,
            overall_gpa,
            overall_letter_grade,
            total_certificates,
            member_since: user_info.member_since,
        };

        Ok(Transcript {
            user_id,
            user_name: user_info.full_name,
            user_email: user_info.email,
            generated_at: Utc::now(),
            summary,
            entries,
        })
    }

    /// Convert GPA to letter grade.
    fn gpa_to_letter_grade(gpa: Decimal) -> LetterGrade {
        let gpa_f = gpa.to_string().parse::<f64>().unwrap_or(0.0);
        match gpa_f {
            g if g >= 4.0 => LetterGrade::A,
            g if g >= 3.7 => LetterGrade::AMinus,
            g if g >= 3.3 => LetterGrade::BPlus,
            g if g >= 3.0 => LetterGrade::B,
            g if g >= 2.7 => LetterGrade::BMinus,
            g if g >= 2.3 => LetterGrade::CPlus,
            g if g >= 2.0 => LetterGrade::C,
            g if g >= 1.7 => LetterGrade::CMinus,
            g if g >= 1.3 => LetterGrade::DPlus,
            g if g >= 1.0 => LetterGrade::D,
            g if g >= 0.7 => LetterGrade::DMinus,
            _ => LetterGrade::F,
        }
    }
}
