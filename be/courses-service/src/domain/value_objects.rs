//! # Course Domain Value Objects
//!
//! Type-safe wrappers for course-related identifiers and primitives.
//!
//! These value objects provide:
//! - Type safety (can't mix CourseId with LessonId)
//! - Validation at construction time
//! - Serialization/deserialization support

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// =============================================================================
// ID WRAPPERS
// =============================================================================

/// Type-safe wrapper for course identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CourseId(pub Uuid);

impl CourseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for CourseId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CourseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for CourseId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<CourseId> for Uuid {
    fn from(id: CourseId) -> Self {
        id.0
    }
}

/// Type-safe wrapper for section identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SectionId(pub Uuid);

impl SectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for SectionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type-safe wrapper for lesson identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LessonId(pub Uuid);

impl LessonId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for LessonId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LessonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// SLUG
// =============================================================================

/// URL-friendly course identifier.
///
/// Generated from the course title, must be unique.
///
/// # Validation Rules
///
/// - Lowercase alphanumeric and hyphens only
/// - No leading/trailing hyphens
/// - No consecutive hyphens
/// - Length: 3-100 characters
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Slug(String);

impl Slug {
    /// Creates a new slug from a string, validating format.
    pub fn new(s: impl Into<String>) -> Result<Self, SlugError> {
        let s = s.into();
        Self::validate(&s)?;
        Ok(Self(s))
    }

    /// Creates a slug from a title by converting to lowercase,
    /// replacing spaces with hyphens, and removing invalid chars.
    pub fn from_title(title: &str) -> Self {
        let slug = slug::slugify(title);
        // Ensure minimum length
        let slug = if slug.len() < 3 {
            format!("{}-course", slug)
        } else {
            slug
        };
        // Truncate to max length
        let slug = if slug.len() > 100 {
            slug[..100].to_string()
        } else {
            slug
        };
        Self(slug)
    }

    /// Validates a slug string.
    fn validate(s: &str) -> Result<(), SlugError> {
        if s.len() < 3 {
            return Err(SlugError::TooShort);
        }
        if s.len() > 100 {
            return Err(SlugError::TooLong);
        }
        if !s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(SlugError::InvalidCharacters);
        }
        if s.starts_with('-') || s.ends_with('-') {
            return Err(SlugError::InvalidFormat);
        }
        if s.contains("--") {
            return Err(SlugError::InvalidFormat);
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Slug> for String {
    fn from(slug: Slug) -> Self {
        slug.0
    }
}

/// Errors when creating a Slug.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SlugError {
    #[error("Slug must be at least 3 characters")]
    TooShort,
    #[error("Slug must be at most 100 characters")]
    TooLong,
    #[error("Slug can only contain lowercase letters, numbers, and hyphens")]
    InvalidCharacters,
    #[error("Slug cannot start/end with hyphen or have consecutive hyphens")]
    InvalidFormat,
}

// =============================================================================
// PRICE
// =============================================================================

/// Represents a monetary price.
///
/// Stores amount in smallest currency unit (cents/centavos) to avoid
/// floating-point precision issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Price {
    /// Amount in smallest currency unit
    pub amount_cents: i32,
    /// ISO 4217 currency code
    pub currency: Currency,
}

impl Price {
    /// Creates a new price.
    pub fn new(amount_cents: i32, currency: Currency) -> Self {
        Self {
            amount_cents: amount_cents.max(0),
            currency,
        }
    }

    /// Creates a free price.
    pub fn free(currency: Currency) -> Self {
        Self {
            amount_cents: 0,
            currency,
        }
    }

    /// Returns true if the price is free.
    pub fn is_free(&self) -> bool {
        self.amount_cents == 0
    }

    /// Formats for display (e.g., "$49.99").
    pub fn format(&self) -> String {
        if self.is_free() {
            return "Free".to_string();
        }
        let amount = self.amount_cents as f64 / 100.0;
        match self.currency {
            Currency::USD => format!("${:.2}", amount),
            Currency::EUR => format!("€{:.2}", amount),
            Currency::MXN => format!("${:.2} MXN", amount),
            Currency::BRL => format!("R${:.2}", amount),
        }
    }
}

impl Default for Price {
    fn default() -> Self {
        Self::free(Currency::USD)
    }
}

/// Supported currencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    USD,
    EUR,
    MXN,
    BRL,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::MXN => write!(f, "MXN"),
            Currency::BRL => write!(f, "BRL"),
        }
    }
}

impl std::str::FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "MXN" => Ok(Currency::MXN),
            "BRL" => Ok(Currency::BRL),
            _ => Err(format!("Unknown currency: {}", s)),
        }
    }
}
