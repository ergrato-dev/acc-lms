//! # Repository Layer
//!
//! Data access layer for grades and transcripts.

pub mod grade_repository;
pub mod transcript_repository;
pub mod stats_repository;

pub use grade_repository::GradeRepository;
pub use transcript_repository::TranscriptRepository;
pub use stats_repository::StatsRepository;
