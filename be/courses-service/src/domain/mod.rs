//! # Courses Domain Module
//!
//! Contains the core domain entities, value objects, and events for the courses service.
//!
//! ## Entities
//!
//! - [`Course`]: Main aggregate root representing a course
//! - [`Section`]: A logical grouping of lessons within a course
//! - [`Lesson`]: Individual content unit (video, article, quiz)
//! - [`Category`]: Course categorization
//!
//! ## Events
//!
//! Domain events emitted for cross-service communication:
//! - `course.created`, `course.updated`
//! - `course.published`, `course.unpublished`
//! - `lesson.created`, `lesson.updated`

pub mod entities;
pub mod events;
pub mod value_objects;

pub use entities::{
    Category, Course, CourseStatus, CourseWithContent, DifficultyLevel, Lesson, LessonType,
    NewCourse, NewLesson, NewSection, Section, SectionWithLessons, UpdateCourse, UpdateLesson,
};
pub use events::{CourseEvent, LessonEvent};
pub use value_objects::{CourseId, LessonId, Price, SectionId, Slug};
