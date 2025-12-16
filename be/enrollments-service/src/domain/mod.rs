//! # Enrollments Domain Module
//!
//! Contains the core domain entities, value objects, and events for the enrollments service.
//!
//! ## Entities
//!
//! - [`Enrollment`]: Main aggregate representing a student's course enrollment
//! - [`LessonProgress`]: Progress tracking for individual lessons
//!
//! ## Events
//!
//! Domain events emitted for cross-service communication:
//! - `enrollment.created`, `enrollment.completed`
//! - `progress.updated`, `lesson.completed`

pub mod entities;
pub mod events;
pub mod value_objects;

pub use entities::{
    Enrollment, EnrollmentStatus, EnrollmentWithProgress, LessonProgress,
    LessonProgressStatus, NewEnrollment, NewLessonProgress, UpdateEnrollment,
    UpdateLessonProgress,
};
pub use events::{EnrollmentEvent, ProgressEvent};
pub use value_objects::{EnrollmentId, ProgressId};
