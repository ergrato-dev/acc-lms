//! # Enrollment Value Objects
//!
//! Type-safe identifiers and domain primitives for the enrollments service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// ENROLLMENT ID
// =============================================================================

/// Type-safe enrollment identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EnrollmentId(Uuid);

impl EnrollmentId {
    /// Creates a new enrollment ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates from an existing UUID.
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for EnrollmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EnrollmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for EnrollmentId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<EnrollmentId> for Uuid {
    fn from(id: EnrollmentId) -> Self {
        id.0
    }
}

// =============================================================================
// PROGRESS ID
// =============================================================================

/// Type-safe progress identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProgressId(Uuid);

impl ProgressId {
    /// Creates a new progress ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates from an existing UUID.
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    /// Returns the inner UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ProgressId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProgressId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ProgressId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<ProgressId> for Uuid {
    fn from(id: ProgressId) -> Self {
        id.0
    }
}

// =============================================================================
// COMPLETION PERCENTAGE
// =============================================================================

/// Type-safe completion percentage (0.0 - 100.0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompletionPercentage(f64);

impl CompletionPercentage {
    /// Creates a new completion percentage.
    ///
    /// # Panics
    ///
    /// Panics if value is not in range 0.0 - 100.0.
    pub fn new(value: f64) -> Self {
        assert!(
            (0.0..=100.0).contains(&value),
            "Completion percentage must be between 0.0 and 100.0"
        );
        Self(value)
    }

    /// Creates a zero percentage.
    pub fn zero() -> Self {
        Self(0.0)
    }

    /// Creates a complete (100%) percentage.
    pub fn complete() -> Self {
        Self(100.0)
    }

    /// Returns the inner value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns true if complete (100%).
    pub fn is_complete(&self) -> bool {
        self.0 >= 100.0
    }

    /// Returns formatted string (e.g., "75%").
    pub fn format(&self) -> String {
        format!("{:.0}%", self.0)
    }
}

impl Default for CompletionPercentage {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<f64> for CompletionPercentage {
    fn from(value: f64) -> Self {
        Self::new(value.clamp(0.0, 100.0))
    }
}

impl From<CompletionPercentage> for f64 {
    fn from(p: CompletionPercentage) -> Self {
        p.0
    }
}
