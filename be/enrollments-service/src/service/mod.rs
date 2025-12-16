//! # Service Layer
//!
//! Business logic for enrollments and progress tracking.

pub mod enrollment_service;

pub use enrollment_service::{EnrollmentError, EnrollmentResult, EnrollmentService};
