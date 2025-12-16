//! # Notification Domain Value Objects
//!
//! Value objects for strong typing and validation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// TYPED IDENTIFIERS
// =============================================================================

/// Strongly typed notification ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationId(pub Uuid);

impl NotificationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for NotificationId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for NotificationId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<NotificationId> for Uuid {
    fn from(id: NotificationId) -> Self {
        id.0
    }
}

/// Strongly typed template ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemplateId(pub Uuid);

impl TemplateId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for TemplateId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TemplateId> for Uuid {
    fn from(id: TemplateId) -> Self {
        id.0
    }
}

// =============================================================================
// COMPLEX VALUE OBJECTS
// =============================================================================

/// Validated email address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Creates a new validated email address.
    pub fn new(email: &str) -> Result<Self, &'static str> {
        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err("Email cannot be empty");
        }

        if !email.contains('@') {
            return Err("Email must contain @");
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format");
        }

        if parts[0].is_empty() || parts[1].is_empty() {
            return Err("Invalid email format");
        }

        if !parts[1].contains('.') {
            return Err("Domain must contain a dot");
        }

        Ok(Self(email))
    }

    /// Returns the email as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the email domain.
    pub fn domain(&self) -> &str {
        self.0.split('@').last().unwrap_or("")
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Notification priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Priority(i32);

impl Priority {
    /// Low priority.
    pub const LOW: Priority = Priority(1);
    /// Normal priority.
    pub const NORMAL: Priority = Priority(5);
    /// High priority.
    pub const HIGH: Priority = Priority(10);
    /// Urgent priority.
    pub const URGENT: Priority = Priority(20);

    /// Creates a new priority (clamped 1-20).
    pub fn new(value: i32) -> Self {
        Self(value.clamp(1, 20))
    }

    /// Returns the numeric value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Checks if this is high priority.
    pub fn is_high(&self) -> bool {
        self.0 >= Self::HIGH.0
    }

    /// Checks if this is urgent priority.
    pub fn is_urgent(&self) -> bool {
        self.0 >= Self::URGENT.0
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

/// Quiet hours for notification delivery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuietHours {
    /// Start hour (0-23).
    pub start_hour: u8,
    /// End hour (0-23).
    pub end_hour: u8,
}

impl QuietHours {
    /// Creates new quiet hours configuration.
    pub fn new(start_hour: u8, end_hour: u8) -> Self {
        Self {
            start_hour: start_hour.min(23),
            end_hour: end_hour.min(23),
        }
    }

    /// Checks if a given hour falls within the quiet period.
    pub fn is_quiet_hour(&self, hour: u8) -> bool {
        let hour = hour.min(23);

        if self.start_hour <= self.end_hour {
            // Normal period (e.g., 9-17 doesn't cross midnight)
            hour >= self.start_hour && hour < self.end_hour
        } else {
            // Overnight period (e.g., 22-08 crosses midnight)
            hour >= self.start_hour || hour < self.end_hour
        }
    }

    /// Converts to string format "HH-HH".
    pub fn to_string_format(&self) -> String {
        format!("{:02}-{:02}", self.start_hour, self.end_hour)
    }

    /// Parses from string format "HH-HH".
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return None;
        }

        let start: u8 = parts[0].parse().ok()?;
        let end: u8 = parts[1].parse().ok()?;

        Some(Self::new(start, end))
    }
}

impl Default for QuietHours {
    fn default() -> Self {
        Self::new(22, 8) // 10pm to 8am by default
    }
}

/// Device token for push notifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceToken {
    /// The device token string.
    pub token: String,
    /// Platform (ios, android, web).
    pub platform: DevicePlatform,
}

/// Device platform type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DevicePlatform {
    Ios,
    Android,
    Web,
}

impl DeviceToken {
    pub fn new(token: String, platform: DevicePlatform) -> Self {
        Self { token, platform }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address_valid() {
        let email = EmailAddress::new("test@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().domain(), "example.com");
    }

    #[test]
    fn test_email_address_invalid() {
        assert!(EmailAddress::new("invalid").is_err());
        assert!(EmailAddress::new("@example.com").is_err());
        assert!(EmailAddress::new("test@").is_err());
    }

    #[test]
    fn test_priority_clamping() {
        assert_eq!(Priority::new(0).value(), 1);
        assert_eq!(Priority::new(100).value(), 20);
        assert_eq!(Priority::new(5).value(), 5);
    }

    #[test]
    fn test_quiet_hours_normal() {
        let qh = QuietHours::new(9, 17); // 9am-5pm
        assert!(qh.is_quiet_hour(12));
        assert!(!qh.is_quiet_hour(8));
        assert!(!qh.is_quiet_hour(18));
    }

    #[test]
    fn test_quiet_hours_overnight() {
        let qh = QuietHours::new(22, 8); // 10pm-8am
        assert!(qh.is_quiet_hour(23));
        assert!(qh.is_quiet_hour(6));
        assert!(!qh.is_quiet_hour(12));
    }
}
