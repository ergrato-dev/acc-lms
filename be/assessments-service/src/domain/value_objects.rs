//! # Assessment Value Objects
//!
//! Strongly-typed identifiers and value objects for the assessments domain.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// TYPED IDENTIFIERS
// =============================================================================

/// Strongly-typed Quiz ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuizId(pub Uuid);

impl QuizId {
    /// Creates a new random QuizId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a QuizId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for QuizId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for QuizId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for QuizId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<QuizId> for Uuid {
    fn from(id: QuizId) -> Self {
        id.0
    }
}

/// Strongly-typed Question ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionId(pub Uuid);

impl QuestionId {
    /// Creates a new random QuestionId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a QuestionId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for QuestionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for QuestionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<QuestionId> for Uuid {
    fn from(id: QuestionId) -> Self {
        id.0
    }
}

/// Strongly-typed Submission ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubmissionId(pub Uuid);

impl SubmissionId {
    /// Creates a new random SubmissionId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a SubmissionId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for SubmissionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubmissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for SubmissionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<SubmissionId> for Uuid {
    fn from(id: SubmissionId) -> Self {
        id.0
    }
}

/// Strongly-typed Response ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResponseId(pub Uuid);

impl ResponseId {
    /// Creates a new random ResponseId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a ResponseId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the inner UUID.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for ResponseId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ResponseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ResponseId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ResponseId> for Uuid {
    fn from(id: ResponseId) -> Self {
        id.0
    }
}

// =============================================================================
// SCORE VALUE OBJECTS
// =============================================================================

/// Represents a quiz score with validation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub earned: f64,
    pub max: f64,
}

impl Score {
    /// Creates a new Score with validation.
    pub fn new(earned: f64, max: f64) -> Result<Self, String> {
        if max <= 0.0 {
            return Err("Max score must be positive".into());
        }
        if earned < 0.0 {
            return Err("Earned score cannot be negative".into());
        }
        if earned > max {
            return Err("Earned score cannot exceed max score".into());
        }
        Ok(Self { earned, max })
    }

    /// Returns the score as a percentage (0-100).
    pub fn percentage(&self) -> f64 {
        (self.earned / self.max) * 100.0
    }

    /// Returns true if the score meets or exceeds the passing threshold.
    pub fn passes(&self, passing_percentage: f64) -> bool {
        self.percentage() >= passing_percentage
    }

    /// Returns a letter grade based on percentage.
    pub fn letter_grade(&self) -> &'static str {
        let pct = self.percentage();
        if pct >= 90.0 {
            "A"
        } else if pct >= 80.0 {
            "B"
        } else if pct >= 70.0 {
            "C"
        } else if pct >= 60.0 {
            "D"
        } else {
            "F"
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1} ({:.1}%)", self.earned, self.max, self.percentage())
    }
}
