//! # Service Layer
//!
//! Business logic for assessments, quizzes, and grading.

pub mod assessment_service;

pub use assessment_service::{AssessmentError, AssessmentResult, AssessmentService};
