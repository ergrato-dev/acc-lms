//! # Enrollments Repository Module
//!
//! PostgreSQL data access layer for enrollments.

pub mod enrollment_repository;

pub use enrollment_repository::{
    EnrollmentRepository,
    CourseEnrollmentStats,
    UserLearningStats,
};
