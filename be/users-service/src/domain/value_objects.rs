//! # Value Objects
//!
//! Value objects are immutable objects that describe characteristics of a thing.
//! Unlike entities, they have no identity - two value objects with the same
//! attributes are considered equal.
//!
//! ## Benefits of Value Objects
//!
//! 1. **Type Safety**: Prevents passing wrong types (e.g., using a String where
//!    a UserId is expected)
//! 2. **Validation**: Ensures values are always valid upon creation
//! 3. **Self-Documenting**: Code clearly shows what type of data is expected
//! 4. **Encapsulation**: Business rules for values are in one place

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// =============================================================================
// USER ID
// =============================================================================

/// A unique identifier for a user.
///
/// Wraps a UUID v4 to provide type safety. This prevents accidentally
/// using a course ID where a user ID is expected.
///
/// # Example
///
/// ```rust
/// use users_service::domain::value_objects::UserId;
///
/// let user_id = UserId::new();
/// let from_string = UserId::from_str("550e8400-e29b-41d4-a716-446655440000")?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    /// Creates a new random UserId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Creates a UserId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Returns the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
    
    /// Returns a reference to the inner UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl std::str::FromStr for UserId {
    type Err = uuid::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

// =============================================================================
// LANGUAGE
// =============================================================================

/// Supported languages in the platform.
///
/// The platform supports three languages:
/// - Spanish (es) - Primary language, default for new users
/// - English (en) - Secondary language
/// - Portuguese (pt) - Tertiary language
///
/// # Validation
///
/// Only these three values are accepted. Any other value will result
/// in an error during parsing.
///
/// # Example
///
/// ```rust
/// use users_service::domain::value_objects::Language;
///
/// let lang = Language::Spanish;
/// assert_eq!(lang.code(), "es");
/// assert_eq!(lang.native_name(), "Español");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Spanish (Español) - Primary language
    #[serde(rename = "es")]
    Spanish,
    
    /// English - Secondary language
    #[serde(rename = "en")]
    English,
    
    /// Portuguese (Português) - Tertiary language
    #[serde(rename = "pt")]
    Portuguese,
}

impl Language {
    /// Returns all supported languages.
    pub fn all() -> &'static [Language] {
        &[Language::Spanish, Language::English, Language::Portuguese]
    }
    
    /// Returns the ISO 639-1 language code.
    ///
    /// # Returns
    ///
    /// - `"es"` for Spanish
    /// - `"en"` for English
    /// - `"pt"` for Portuguese
    pub fn code(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
            Language::Portuguese => "pt",
        }
    }
    
    /// Returns the language name in English.
    pub fn english_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Spanish",
            Language::English => "English",
            Language::Portuguese => "Portuguese",
        }
    }
    
    /// Returns the language name in its native form.
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
            Language::English => "English",
            Language::Portuguese => "Português",
        }
    }
    
    /// Parses a language from its code.
    ///
    /// # Arguments
    ///
    /// * `code` - The ISO 639-1 language code
    ///
    /// # Returns
    ///
    /// - `Some(Language)` if the code is valid
    /// - `None` if the code is not recognized
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "es" | "spanish" | "español" => Some(Language::Spanish),
            "en" | "english" => Some(Language::English),
            "pt" | "portuguese" | "português" => Some(Language::Portuguese),
            _ => None,
        }
    }
}

impl Default for Language {
    /// Default language is Spanish (primary market).
    fn default() -> Self {
        Language::Spanish
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl std::str::FromStr for Language {
    type Err = LanguageParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::from_code(s).ok_or_else(|| LanguageParseError {
            invalid_code: s.to_string(),
        })
    }
}

/// Error when parsing an invalid language code.
#[derive(Debug, Clone)]
pub struct LanguageParseError {
    /// The invalid code that was provided
    pub invalid_code: String,
}

impl fmt::Display for LanguageParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid language code '{}'. Supported: es, en, pt",
            self.invalid_code
        )
    }
}

impl std::error::Error for LanguageParseError {}

// =============================================================================
// TIMEZONE
// =============================================================================

/// A validated IANA timezone string.
///
/// Wraps a timezone string and validates it against common IANA timezones.
/// This provides type safety and ensures only valid timezones are used.
///
/// # Common Timezones
///
/// - Americas: `America/Mexico_City`, `America/New_York`, `America/Los_Angeles`, `America/Sao_Paulo`
/// - Europe: `Europe/Madrid`, `Europe/London`, `Europe/Paris`
/// - Asia: `Asia/Tokyo`, `Asia/Shanghai`
///
/// # Example
///
/// ```rust
/// use users_service::domain::value_objects::Timezone;
///
/// let tz = Timezone::from_str("America/Mexico_City")?;
/// assert!(tz.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timezone(String);

impl Timezone {
    /// Common IANA timezones for quick validation.
    const COMMON_TIMEZONES: &'static [&'static str] = &[
        // Americas
        "America/Mexico_City",
        "America/Bogota",
        "America/Lima",
        "America/Santiago",
        "America/Buenos_Aires",
        "America/Sao_Paulo",
        "America/Caracas",
        "America/New_York",
        "America/Chicago",
        "America/Denver",
        "America/Los_Angeles",
        "America/Phoenix",
        "America/Toronto",
        "America/Vancouver",
        // Europe
        "Europe/Madrid",
        "Europe/London",
        "Europe/Paris",
        "Europe/Berlin",
        "Europe/Rome",
        "Europe/Amsterdam",
        "Europe/Lisbon",
        // Asia
        "Asia/Tokyo",
        "Asia/Shanghai",
        "Asia/Hong_Kong",
        "Asia/Singapore",
        "Asia/Seoul",
        "Asia/Mumbai",
        "Asia/Dubai",
        // Oceania
        "Australia/Sydney",
        "Australia/Melbourne",
        "Pacific/Auckland",
        // UTC
        "UTC",
    ];
    
    /// Creates a new Timezone if valid.
    ///
    /// # Arguments
    ///
    /// * `tz` - IANA timezone string
    ///
    /// # Returns
    ///
    /// - `Ok(Timezone)` if the timezone is valid
    /// - `Err(TimezoneError)` if the timezone is not recognized
    pub fn new(tz: impl Into<String>) -> Result<Self, TimezoneError> {
        let tz = tz.into();
        
        // Check against common timezones
        if Self::COMMON_TIMEZONES.contains(&tz.as_str()) {
            return Ok(Self(tz));
        }
        
        // Also accept any timezone that looks like a valid IANA format
        // (Region/City or UTC±offset)
        if Self::looks_valid(&tz) {
            return Ok(Self(tz));
        }
        
        Err(TimezoneError {
            invalid_timezone: tz,
        })
    }
    
    /// Checks if a timezone string looks like a valid IANA format.
    fn looks_valid(tz: &str) -> bool {
        // UTC is always valid
        if tz == "UTC" {
            return true;
        }
        
        // Must contain a slash (Region/City format)
        if !tz.contains('/') {
            return false;
        }
        
        // Split and validate parts
        let parts: Vec<&str> = tz.split('/').collect();
        if parts.len() < 2 {
            return false;
        }
        
        // Region should be capitalized
        let region = parts[0];
        if !region.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            return false;
        }
        
        true
    }
    
    /// Returns the timezone string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Returns true if this is a valid timezone.
    pub fn is_valid(&self) -> bool {
        Self::COMMON_TIMEZONES.contains(&self.0.as_str()) || Self::looks_valid(&self.0)
    }
    
    /// Returns the default timezone (America/Mexico_City).
    pub fn default_timezone() -> Self {
        Self("America/Mexico_City".to_string())
    }
    
    /// Returns a list of common timezones for UI selection.
    pub fn common_timezones() -> &'static [&'static str] {
        Self::COMMON_TIMEZONES
    }
}

impl Default for Timezone {
    fn default() -> Self {
        Self::default_timezone()
    }
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Timezone {
    type Err = TimezoneError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<Timezone> for String {
    fn from(tz: Timezone) -> Self {
        tz.0
    }
}

/// Error when parsing an invalid timezone.
#[derive(Debug, Clone)]
pub struct TimezoneError {
    /// The invalid timezone that was provided
    pub invalid_timezone: String,
}

impl fmt::Display for TimezoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid timezone '{}'. Use IANA timezone format (e.g., America/Mexico_City)",
            self.invalid_timezone
        )
    }
}

impl std::error::Error for TimezoneError {}

// =============================================================================
// EMAIL ADDRESS
// =============================================================================

/// A validated email address.
///
/// Provides basic email validation and normalization.
///
/// # Validation Rules
///
/// - Must contain exactly one `@` symbol
/// - Must have a non-empty local part (before @)
/// - Must have a valid domain (after @)
/// - Domain must have at least one dot
///
/// # Normalization
///
/// - Converted to lowercase
/// - Whitespace trimmed
///
/// # Example
///
/// ```rust
/// use users_service::domain::value_objects::EmailAddress;
///
/// let email = EmailAddress::new("User@Example.com")?;
/// assert_eq!(email.as_str(), "user@example.com");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Creates a new EmailAddress if valid.
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into().trim().to_lowercase();
        
        // Basic validation
        if !Self::is_valid_format(&email) {
            return Err(EmailError {
                invalid_email: email,
            });
        }
        
        Ok(Self(email))
    }
    
    /// Validates email format.
    fn is_valid_format(email: &str) -> bool {
        // Must contain exactly one @
        let at_count = email.matches('@').count();
        if at_count != 1 {
            return false;
        }
        
        // Split by @
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }
        
        let local = parts[0];
        let domain = parts[1];
        
        // Local part must not be empty
        if local.is_empty() {
            return false;
        }
        
        // Domain must not be empty and must contain a dot
        if domain.is_empty() || !domain.contains('.') {
            return false;
        }
        
        // Domain must not start or end with a dot
        if domain.starts_with('.') || domain.ends_with('.') {
            return false;
        }
        
        true
    }
    
    /// Returns the email as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Returns the local part (before @).
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
    
    /// Returns the domain part (after @).
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = EmailError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Error when parsing an invalid email address.
#[derive(Debug, Clone)]
pub struct EmailError {
    /// The invalid email that was provided
    pub invalid_email: String,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid email address: {}", self.invalid_email)
    }
}

impl std::error::Error for EmailError {}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        let id1 = UserId::new();
        let id2 = UserId::from_uuid(id1.into_inner());
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code("EN"), Some(Language::English));
        assert_eq!(Language::from_code("pt"), Some(Language::Portuguese));
        assert_eq!(Language::from_code("fr"), None);
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Portuguese.code(), "pt");
    }

    #[test]
    fn test_timezone_valid() {
        assert!(Timezone::new("America/Mexico_City").is_ok());
        assert!(Timezone::new("UTC").is_ok());
        assert!(Timezone::new("Europe/Madrid").is_ok());
    }

    #[test]
    fn test_timezone_invalid() {
        assert!(Timezone::new("invalid").is_err());
        assert!(Timezone::new("").is_err());
    }

    #[test]
    fn test_email_valid() {
        assert!(EmailAddress::new("test@example.com").is_ok());
        assert!(EmailAddress::new("User@Example.COM").is_ok());
    }

    #[test]
    fn test_email_invalid() {
        assert!(EmailAddress::new("invalid").is_err());
        assert!(EmailAddress::new("@example.com").is_err());
        assert!(EmailAddress::new("test@").is_err());
        assert!(EmailAddress::new("test@example").is_err());
    }

    #[test]
    fn test_email_normalization() {
        let email = EmailAddress::new("  USER@Example.COM  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_parts() {
        let email = EmailAddress::new("user@example.com").unwrap();
        assert_eq!(email.local_part(), "user");
        assert_eq!(email.domain(), "example.com");
    }
}
