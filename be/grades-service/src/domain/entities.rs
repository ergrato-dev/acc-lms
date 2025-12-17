//! # Domain Entities
//!
//! Core business entities for grades and transcripts.
//!
//! ## Entity Overview
//!
//! - `GradeEntry`: Individual grade from a quiz submission
//! - `CourseGrade`: Aggregated grade for a course (weighted average)
//! - `StudentGradeSummary`: Overall summary across all courses
//! - `TranscriptEntry`: Single line item in academic transcript
//! - `Transcript`: Complete academic record
//! - `CourseStats`: Statistics for instructor view
//! - `QuestionStats`: Per-question performance data

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// =============================================================================
// ENUMS
// =============================================================================

/// Status of a grade entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradeStatus {
    /// Automatically graded (multiple choice, etc.)
    Graded,
    /// Waiting for instructor review (essay, code)
    PendingReview,
    /// Submitted but not yet processed
    Processing,
}

impl Default for GradeStatus {
    fn default() -> Self {
        Self::Processing
    }
}

/// Letter grade representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LetterGrade {
    APlus,
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    DMinus,
    F,
}

impl LetterGrade {
    /// Calculate letter grade from percentage.
    pub fn from_percentage(percentage: Decimal) -> Self {
        let pct = percentage.to_string().parse::<f64>().unwrap_or(0.0);
        match pct {
            p if p >= 97.0 => LetterGrade::APlus,
            p if p >= 93.0 => LetterGrade::A,
            p if p >= 90.0 => LetterGrade::AMinus,
            p if p >= 87.0 => LetterGrade::BPlus,
            p if p >= 83.0 => LetterGrade::B,
            p if p >= 80.0 => LetterGrade::BMinus,
            p if p >= 77.0 => LetterGrade::CPlus,
            p if p >= 73.0 => LetterGrade::C,
            p if p >= 70.0 => LetterGrade::CMinus,
            p if p >= 67.0 => LetterGrade::DPlus,
            p if p >= 63.0 => LetterGrade::D,
            p if p >= 60.0 => LetterGrade::DMinus,
            _ => LetterGrade::F,
        }
    }

    /// Get GPA points for this grade.
    pub fn gpa_points(&self) -> Decimal {
        match self {
            LetterGrade::APlus => Decimal::new(40, 1),  // 4.0
            LetterGrade::A => Decimal::new(40, 1),     // 4.0
            LetterGrade::AMinus => Decimal::new(37, 1), // 3.7
            LetterGrade::BPlus => Decimal::new(33, 1), // 3.3
            LetterGrade::B => Decimal::new(30, 1),     // 3.0
            LetterGrade::BMinus => Decimal::new(27, 1), // 2.7
            LetterGrade::CPlus => Decimal::new(23, 1), // 2.3
            LetterGrade::C => Decimal::new(20, 1),     // 2.0
            LetterGrade::CMinus => Decimal::new(17, 1), // 1.7
            LetterGrade::DPlus => Decimal::new(13, 1), // 1.3
            LetterGrade::D => Decimal::new(10, 1),     // 1.0
            LetterGrade::DMinus => Decimal::new(7, 1), // 0.7
            LetterGrade::F => Decimal::ZERO,
        }
    }

    /// Display string.
    pub fn as_str(&self) -> &'static str {
        match self {
            LetterGrade::APlus => "A+",
            LetterGrade::A => "A",
            LetterGrade::AMinus => "A-",
            LetterGrade::BPlus => "B+",
            LetterGrade::B => "B",
            LetterGrade::BMinus => "B-",
            LetterGrade::CPlus => "C+",
            LetterGrade::C => "C",
            LetterGrade::CMinus => "C-",
            LetterGrade::DPlus => "D+",
            LetterGrade::D => "D",
            LetterGrade::DMinus => "D-",
            LetterGrade::F => "F",
        }
    }
}

impl std::fmt::Display for LetterGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Export format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
    Pdf,
}

impl Default for ExportFormat {
    fn default() -> Self {
        Self::Csv
    }
}

// =============================================================================
// GRADE ENTITIES
// =============================================================================

/// Individual grade from a quiz submission.
/// Aggregates data from assessments.quiz_submissions.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GradeEntry {
    pub submission_id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub course_id: Uuid,
    pub quiz_title: String,
    pub score: Decimal,
    pub max_score: Decimal,
    pub percentage: Decimal,
    pub passed: Option<bool>,
    pub attempt_number: i32,
    pub status: String,  // 'in_progress', 'submitted', 'graded'
    pub submitted_at: Option<DateTime<Utc>>,
    pub graded_at: Option<DateTime<Utc>>,
    pub instructor_feedback: Option<String>,
}

impl GradeEntry {
    /// Calculate letter grade for this entry.
    pub fn letter_grade(&self) -> LetterGrade {
        LetterGrade::from_percentage(self.percentage)
    }

    /// Check if this grade is pending review.
    pub fn is_pending_review(&self) -> bool {
        self.status == "submitted" && self.graded_at.is_none()
    }

    /// Get display status.
    pub fn display_status(&self) -> GradeStatus {
        match self.status.as_str() {
            "graded" => GradeStatus::Graded,
            "submitted" if self.graded_at.is_none() => GradeStatus::PendingReview,
            _ => GradeStatus::Processing,
        }
    }
}

/// Aggregated grade for a single course.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseGrade {
    pub course_id: Uuid,
    pub course_title: String,
    pub user_id: Uuid,
    pub enrollment_id: Uuid,
    pub total_quizzes: i32,
    pub completed_quizzes: i32,
    pub pending_review: i32,
    pub total_points_earned: Decimal,
    pub total_points_possible: Decimal,
    pub weighted_average: Decimal,
    pub letter_grade: LetterGrade,
    pub passed_quizzes: i32,
    pub failed_quizzes: i32,
    pub grades: Vec<GradeEntry>,
    pub last_activity: Option<DateTime<Utc>>,
}

impl CourseGrade {
    /// Create a new course grade from entries.
    pub fn from_entries(
        course_id: Uuid,
        course_title: String,
        user_id: Uuid,
        enrollment_id: Uuid,
        entries: Vec<GradeEntry>,
    ) -> Self {
        let total_quizzes = entries.len() as i32;
        let completed_quizzes = entries.iter()
            .filter(|e| e.status == "graded")
            .count() as i32;
        let pending_review = entries.iter()
            .filter(|e| e.is_pending_review())
            .count() as i32;

        let graded_entries: Vec<_> = entries.iter()
            .filter(|e| e.status == "graded")
            .collect();

        let total_points_earned: Decimal = graded_entries.iter()
            .map(|e| e.score)
            .sum();
        let total_points_possible: Decimal = graded_entries.iter()
            .map(|e| e.max_score)
            .sum();

        let weighted_average = if total_points_possible > Decimal::ZERO {
            (total_points_earned / total_points_possible) * Decimal::new(100, 0)
        } else {
            Decimal::ZERO
        };

        let passed_quizzes = entries.iter()
            .filter(|e| e.passed == Some(true))
            .count() as i32;
        let failed_quizzes = entries.iter()
            .filter(|e| e.passed == Some(false))
            .count() as i32;

        let last_activity = entries.iter()
            .filter_map(|e| e.submitted_at)
            .max();

        CourseGrade {
            course_id,
            course_title,
            user_id,
            enrollment_id,
            total_quizzes,
            completed_quizzes,
            pending_review,
            total_points_earned,
            total_points_possible,
            weighted_average,
            letter_grade: LetterGrade::from_percentage(weighted_average),
            passed_quizzes,
            failed_quizzes,
            grades: entries,
            last_activity,
        }
    }
}

/// Summary of all grades for a student.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentGradeSummary {
    pub user_id: Uuid,
    pub total_courses: i32,
    pub courses_completed: i32,
    pub overall_average: Decimal,
    pub overall_letter_grade: LetterGrade,
    pub gpa: Decimal,
    pub total_quizzes_taken: i32,
    pub total_quizzes_passed: i32,
    pub pending_reviews: i32,
    pub course_grades: Vec<CourseGrade>,
    pub calculated_at: DateTime<Utc>,
}

impl StudentGradeSummary {
    /// Create summary from course grades.
    pub fn from_course_grades(user_id: Uuid, course_grades: Vec<CourseGrade>) -> Self {
        let total_courses = course_grades.len() as i32;
        let courses_completed = course_grades.iter()
            .filter(|c| c.completed_quizzes == c.total_quizzes && c.total_quizzes > 0)
            .count() as i32;

        // Calculate weighted average across all courses
        let total_points_earned: Decimal = course_grades.iter()
            .map(|c| c.total_points_earned)
            .sum();
        let total_points_possible: Decimal = course_grades.iter()
            .map(|c| c.total_points_possible)
            .sum();

        let overall_average = if total_points_possible > Decimal::ZERO {
            (total_points_earned / total_points_possible) * Decimal::new(100, 0)
        } else {
            Decimal::ZERO
        };

        // Calculate GPA
        let gpa = Self::calculate_gpa(&course_grades);

        let total_quizzes_taken: i32 = course_grades.iter()
            .map(|c| c.completed_quizzes)
            .sum();
        let total_quizzes_passed: i32 = course_grades.iter()
            .map(|c| c.passed_quizzes)
            .sum();
        let pending_reviews: i32 = course_grades.iter()
            .map(|c| c.pending_review)
            .sum();

        StudentGradeSummary {
            user_id,
            total_courses,
            courses_completed,
            overall_average,
            overall_letter_grade: LetterGrade::from_percentage(overall_average),
            gpa,
            total_quizzes_taken,
            total_quizzes_passed,
            pending_reviews,
            course_grades,
            calculated_at: Utc::now(),
        }
    }

    fn calculate_gpa(course_grades: &[CourseGrade]) -> Decimal {
        // Assume all courses are equal weight (could be enhanced with credit hours)
        let completed_courses: Vec<_> = course_grades.iter()
            .filter(|c| c.completed_quizzes > 0)
            .collect();

        if completed_courses.is_empty() {
            return Decimal::ZERO;
        }

        let total_points: Decimal = completed_courses.iter()
            .map(|c| c.letter_grade.gpa_points())
            .sum();

        total_points / Decimal::new(completed_courses.len() as i64, 0)
    }
}

// =============================================================================
// TRANSCRIPT ENTITIES
// =============================================================================

/// Single line item in academic transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub course_id: Uuid,
    pub course_title: String,
    pub instructor_name: String,
    pub enrollment_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub progress_percentage: Decimal,
    pub grade_percentage: Decimal,
    pub letter_grade: LetterGrade,
    pub status: TranscriptEntryStatus,
    pub certificate_issued: bool,
    pub certificate_url: Option<String>,
}

/// Status for transcript entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptEntryStatus {
    InProgress,
    Completed,
    Dropped,
    Withdrawn,
}

/// Complete academic transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub generated_at: DateTime<Utc>,
    pub summary: TranscriptSummary,
    pub entries: Vec<TranscriptEntry>,
}

/// Summary section of transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSummary {
    pub total_courses_enrolled: i32,
    pub courses_completed: i32,
    pub courses_in_progress: i32,
    pub overall_gpa: Decimal,
    pub overall_letter_grade: LetterGrade,
    pub total_certificates: i32,
    pub member_since: DateTime<Utc>,
}

// =============================================================================
// STATISTICS ENTITIES (for instructors)
// =============================================================================

/// Overall statistics for a course (instructor view).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseStats {
    pub course_id: Uuid,
    pub course_title: String,
    pub total_students: i32,
    pub students_with_submissions: i32,
    pub total_quizzes: i32,
    pub average_score_percentage: Decimal,
    pub pass_rate: Decimal,
    pub grade_distribution: GradeDistribution,
    pub quiz_stats: Vec<QuizStats>,
    pub calculated_at: DateTime<Utc>,
}

/// Grade distribution breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeDistribution {
    pub a_range: i32,     // A+, A, A-
    pub b_range: i32,     // B+, B, B-
    pub c_range: i32,     // C+, C, C-
    pub d_range: i32,     // D+, D, D-
    pub f_range: i32,     // F
}

impl Default for GradeDistribution {
    fn default() -> Self {
        Self {
            a_range: 0,
            b_range: 0,
            c_range: 0,
            d_range: 0,
            f_range: 0,
        }
    }
}

impl GradeDistribution {
    /// Add a letter grade to distribution.
    pub fn add_grade(&mut self, grade: LetterGrade) {
        match grade {
            LetterGrade::APlus | LetterGrade::A | LetterGrade::AMinus => self.a_range += 1,
            LetterGrade::BPlus | LetterGrade::B | LetterGrade::BMinus => self.b_range += 1,
            LetterGrade::CPlus | LetterGrade::C | LetterGrade::CMinus => self.c_range += 1,
            LetterGrade::DPlus | LetterGrade::D | LetterGrade::DMinus => self.d_range += 1,
            LetterGrade::F => self.f_range += 1,
        }
    }
}

/// Statistics for a single quiz.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizStats {
    pub quiz_id: Uuid,
    pub quiz_title: String,
    pub total_submissions: i32,
    pub unique_students: i32,
    pub average_score: Decimal,
    pub highest_score: Decimal,
    pub lowest_score: Decimal,
    pub pass_rate: Decimal,
    pub average_time_seconds: Option<i32>,
    pub average_attempts: Decimal,
    pub question_stats: Vec<QuestionStats>,
}

/// Per-question performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionStats {
    pub question_id: Uuid,
    pub question_text: String,
    pub question_type: String,
    pub total_responses: i32,
    pub correct_responses: i32,
    pub correct_percentage: Decimal,
    /// Most common incorrect answer (if applicable)
    pub common_mistake: Option<String>,
}

// =============================================================================
// FILTERS AND PAGINATION
// =============================================================================

/// Filters for grade queries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GradeFilter {
    pub course_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub status: Option<String>,
    pub passed: Option<bool>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
}

/// Pagination parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}
