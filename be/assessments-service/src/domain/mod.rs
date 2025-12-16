//! # Assessments Domain Module
//!
//! Contains core domain entities, value objects, and events for the assessments service.
//!
//! ## Entities
//!
//! - [`Quiz`]: Quiz/assessment definition
//! - [`QuizQuestion`]: Questions within a quiz
//! - [`QuizSubmission`]: Student quiz attempt
//! - [`QuizResponse`]: Individual question response
//!
//! ## Events
//!
//! Domain events emitted for cross-service communication:
//! - `quiz.created`, `quiz.published`
//! - `submission.started`, `submission.submitted`, `submission.graded`

pub mod entities;
pub mod events;
pub mod value_objects;

pub use entities::{
    NewQuiz, NewQuizQuestion, NewQuizResponse, NewQuizSubmission,
    Quiz, QuizQuestion, QuestionType, QuizResponse, QuizSubmission,
    SubmissionStatus, UpdateQuiz, UpdateQuizQuestion, UpdateQuizSubmission,
    QuizWithQuestions, SubmissionWithResponses,
};
pub use events::{QuizEvent, SubmissionEvent};
pub use value_objects::{QuizId, QuestionId, SubmissionId, ResponseId};
