//! # Assessments Repository Module
//!
//! PostgreSQL data access layer for assessments.

pub mod assessment_repository;

pub use assessment_repository::{
    AssessmentRepository,
    QuizStats,
    SubmissionStats,
};
