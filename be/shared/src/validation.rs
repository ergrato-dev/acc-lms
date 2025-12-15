//! # Request Validation Helpers
//!
//! Utilities for validating incoming request data using the `validator` crate.
//!
//! ## Overview
//!
//! This module provides:
//!
//! | Component | Purpose |
//! |-----------|---------|
//! | [`validate_request`] | Validate a struct implementing `Validate` |
//! | [`parse_and_validate`] | Parse JSON and validate in one step |
//! | [`validators`] | Custom validation functions |
//!
//! ## How Validation Works
//!
//! We use the [`validator`](https://docs.rs/validator/) crate which provides
//! derive macros for declarative validation:
//!
//! ```rust,ignore
//! use validator::Validate;
//!
//! #[derive(Validate)]
//! struct CreateUser {
//!     #[validate(email)]
//!     email: String,
//!     
//!     #[validate(length(min = 10))]
//!     password: String,
//!     
//!     #[validate(custom(function = "validators::not_blank"))]
//!     name: String,
//! }
//! ```
//!
//! ## Validation Flow
//!
//! ```text
//! ┌──────────────┐     ┌───────────────┐     ┌──────────────┐
//! │ JSON Request │────►│ Deserialize   │────►│   Validate   │
//! │              │     │ (serde)       │     │  (validator) │
//! └──────────────┘     └───────────────┘     └──────┬───────┘
//!                                                   │
//!         ┌─────────────────────────────────────────┴───────┐
//!         │                                                 │
//!         ▼                                                 ▼
//! ┌───────────────┐                               ┌─────────────────┐
//! │   Success     │                               │ ValidationError │
//! │ (continue)    │                               │   (400 + details)│
//! └───────────────┘                               └─────────────────┘
//! ```
//!
//! ## Error Response Format
//!
//! When validation fails, the API returns a 400 Bad Request with details:
//!
//! ```json
//! {
//!   "code": "VALIDATION_ERROR",
//!   "message": "Validation error",
//!   "details": {
//!     "email": [
//!       { "code": "email", "message": "Invalid email format" }
//!     ],
//!     "password": [
//!       { "code": "length", "message": "Must be at least 10 characters" }
//!     ]
//!   }
//! }
//! ```
//!
//! ## Custom Validators
//!
//! The [`validators`] module provides common validation functions:
//!
//! | Validator | Purpose |
//! |-----------|---------|
//! | `not_blank` | String is not empty after trimming |
//! | `valid_slug` | URL-safe slug format |
//! | `valid_price_cents` | Non-negative integer for prices |
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::validation::{validate_request, validators};
//! use validator::Validate;
//!
//! #[derive(Validate)]
//! struct CreateCourse {
//!     #[validate(length(min = 1, max = 200))]
//!     title: String,
//!     
//!     #[validate(custom(function = "validators::valid_slug"))]
//!     slug: String,
//!     
//!     #[validate(custom(function = "validators::valid_price_cents"))]
//!     price_cents: i32,
//! }
//!
//! async fn handler(body: Json<CreateCourse>) -> Result<impl Responder, ApiError> {
//!     validate_request(&body)?;
//!     // Validation passed, continue...
//! }
//! ```
//!
//! ## Related Documentation
//!
//! - [`validator` crate](https://docs.rs/validator/)
//! - [`crate::errors::ApiError`] - Error handling
//! - [`_docs/development/development-standards.md`] - Validation guidelines

use crate::errors::ApiError;
use serde::de::DeserializeOwned;
use validator::Validate;

// =============================================================================
// Core Validation Functions
// =============================================================================

/// Validates a request DTO that implements `Validate`.
///
/// This is the primary validation function. Use it in handlers to
/// validate incoming data.
///
/// ## Example
///
/// ```rust,ignore
/// async fn create_user(body: Json<CreateUserDto>) -> Result<impl Responder, ApiError> {
///     validate_request(&body)?;  // Returns early if invalid
///     
///     // Continue with validated data
/// }
/// ```
///
/// ## Errors
///
/// Returns `ApiError::ValidationError` with field-level details if validation fails.
pub fn validate_request<T: Validate>(data: &T) -> Result<(), ApiError> {
    data.validate().map_err(ApiError::ValidationError)
}

/// Parses JSON string and validates in a single step.
///
/// Useful when you have raw JSON and need to parse + validate.
///
/// ## Example
///
/// ```rust,ignore
/// let json = r#"{"email": "user@example.com", "password": "secret"}"#;
/// let user: CreateUserDto = parse_and_validate(json)?;
/// ```
///
/// ## Errors
///
/// - `ApiError::BadRequest` if JSON parsing fails
/// - `ApiError::ValidationError` if validation fails
pub fn parse_and_validate<T: DeserializeOwned + Validate>(json: &str) -> Result<T, ApiError> {
    // Parse JSON
    let data: T = serde_json::from_str(json)
        .map_err(|e| ApiError::BadRequest { message: e.to_string() })?;

    // Validate
    validate_request(&data)?;

    Ok(data)
}

// =============================================================================
// Custom Validators
// =============================================================================

/// Custom validation functions for use with `#[validate(custom)]`.
///
/// These functions follow the validator crate's signature:
/// `fn(&T) -> Result<(), ValidationError>`
///
/// ## Usage
///
/// ```rust,ignore
/// use validator::Validate;
/// use shared::validation::validators;
///
/// #[derive(Validate)]
/// struct MyStruct {
///     #[validate(custom(function = "validators::not_blank"))]
///     name: String,
/// }
/// ```
pub mod validators {
    use validator::ValidationError;

    /// Validates that a string is not blank (empty or whitespace-only).
    ///
    /// ## Valid Values
    ///
    /// - `"hello"` ✓
    /// - `"  hello  "` ✓ (has content)
    ///
    /// ## Invalid Values
    ///
    /// - `""` ✗
    /// - `"   "` ✗ (whitespace only)
    pub fn not_blank(value: &str) -> Result<(), ValidationError> {
        if value.trim().is_empty() {
            return Err(ValidationError::new("blank"));
        }
        Ok(())
    }

    /// Validates a URL-safe slug format.
    ///
    /// ## Rules
    ///
    /// - Only lowercase letters, digits, and hyphens
    /// - Cannot start or end with hyphen
    /// - No consecutive hyphens
    ///
    /// ## Valid Examples
    ///
    /// - `"my-course"` ✓
    /// - `"intro-to-rust-2024"` ✓
    /// - `"a-b-c"` ✓
    ///
    /// ## Invalid Examples
    ///
    /// - `"My-Course"` ✗ (uppercase)
    /// - `"-invalid"` ✗ (starts with hyphen)
    /// - `"invalid-"` ✗ (ends with hyphen)
    /// - `"in--valid"` ✗ (double hyphen)
    /// - `"my_course"` ✗ (underscore not allowed)
    pub fn valid_slug(value: &str) -> Result<(), ValidationError> {
        // Check for valid characters
        if !value.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(ValidationError::new("invalid_slug"));
        }
        
        // Cannot start or end with hyphen
        if value.starts_with('-') || value.ends_with('-') {
            return Err(ValidationError::new("invalid_slug"));
        }
        
        // No consecutive hyphens
        if value.contains("--") {
            return Err(ValidationError::new("invalid_slug"));
        }
        
        Ok(())
    }

    /// Validates a price in cents (smallest currency unit).
    ///
    /// Prices must be non-negative. We store prices in cents to avoid
    /// floating-point precision issues.
    ///
    /// ## Valid Values
    ///
    /// - `0` ✓ (free)
    /// - `999` ✓ ($9.99)
    /// - `9999` ✓ ($99.99)
    ///
    /// ## Invalid Values
    ///
    /// - `-1` ✗ (negative)
    pub fn valid_price_cents(value: i32) -> Result<(), ValidationError> {
        if value < 0 {
            return Err(ValidationError::new("negative_price"));
        }
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::validators::*;

    #[test]
    fn test_not_blank_valid() {
        assert!(not_blank("hello").is_ok());
        assert!(not_blank("  hello  ").is_ok());
        assert!(not_blank("a").is_ok());
    }

    #[test]
    fn test_not_blank_invalid() {
        assert!(not_blank("").is_err());
        assert!(not_blank("   ").is_err());
        assert!(not_blank("\t\n").is_err());
    }

    #[test]
    fn test_valid_slug_valid() {
        assert!(valid_slug("my-course-title").is_ok());
        assert!(valid_slug("course123").is_ok());
        assert!(valid_slug("a-b-c").is_ok());
        assert!(valid_slug("a").is_ok());
        assert!(valid_slug("intro-to-rust").is_ok());
    }

    #[test]
    fn test_valid_slug_invalid_uppercase() {
        assert!(valid_slug("My-Course").is_err());
        assert!(valid_slug("COURSE").is_err());
    }

    #[test]
    fn test_valid_slug_invalid_hyphen_position() {
        assert!(valid_slug("-invalid").is_err());
        assert!(valid_slug("invalid-").is_err());
        assert!(valid_slug("-").is_err());
    }

    #[test]
    fn test_valid_slug_invalid_double_hyphen() {
        assert!(valid_slug("in--valid").is_err());
        assert!(valid_slug("a--b--c").is_err());
    }

    #[test]
    fn test_valid_slug_invalid_characters() {
        assert!(valid_slug("my_course").is_err());  // underscore
        assert!(valid_slug("my course").is_err());  // space
        assert!(valid_slug("my.course").is_err());  // dot
    }

    #[test]
    fn test_valid_price_cents_valid() {
        assert!(valid_price_cents(0).is_ok());
        assert!(valid_price_cents(1).is_ok());
        assert!(valid_price_cents(9999).is_ok());
        assert!(valid_price_cents(1_000_000).is_ok());
    }

    #[test]
    fn test_valid_price_cents_invalid() {
        assert!(valid_price_cents(-1).is_err());
        assert!(valid_price_cents(-100).is_err());
    }
}

