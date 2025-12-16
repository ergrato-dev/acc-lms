//! # Payments Domain Value Objects
//!
//! Strongly-typed identifiers and value objects.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// =============================================================================
// TYPED IDENTIFIERS
// =============================================================================

/// Strongly-typed Order ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for OrderId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<OrderId> for Uuid {
    fn from(id: OrderId) -> Self {
        id.0
    }
}

/// Strongly-typed Transaction ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TransactionId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<TransactionId> for Uuid {
    fn from(id: TransactionId) -> Self {
        id.0
    }
}

/// Strongly-typed Discount Code ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiscountCodeId(pub Uuid);

impl DiscountCodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DiscountCodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DiscountCodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for DiscountCodeId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<DiscountCodeId> for Uuid {
    fn from(id: DiscountCodeId) -> Self {
        id.0
    }
}

/// Strongly-typed Review ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReviewId(pub Uuid);

impl ReviewId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ReviewId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ReviewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ReviewId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<ReviewId> for Uuid {
    fn from(id: ReviewId) -> Self {
        id.0
    }
}

// =============================================================================
// VALUE OBJECTS
// =============================================================================

/// Money value object with amount in cents and currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the smallest currency unit (e.g., cents for USD)
    pub amount_cents: i32,
    /// ISO 4217 currency code
    pub currency: [char; 3],
}

impl Money {
    /// Creates a new Money value.
    pub fn new(amount_cents: i32, currency: &str) -> Result<Self, String> {
        if currency.len() != 3 {
            return Err("Currency code must be exactly 3 characters".into());
        }

        let mut chars = [' '; 3];
        for (i, c) in currency.chars().take(3).enumerate() {
            chars[i] = c.to_ascii_uppercase();
        }

        Ok(Self {
            amount_cents,
            currency: chars,
        })
    }

    /// Creates a USD money value.
    pub fn usd(amount_cents: i32) -> Self {
        Self {
            amount_cents,
            currency: ['U', 'S', 'D'],
        }
    }

    /// Returns the currency code as a string.
    pub fn currency_code(&self) -> String {
        self.currency.iter().collect()
    }

    /// Returns the amount formatted as a string (e.g., "$19.99").
    pub fn formatted(&self) -> String {
        let symbol = match self.currency_code().as_str() {
            "USD" => "$",
            "EUR" => "€",
            "GBP" => "£",
            _ => "",
        };
        let dollars = self.amount_cents as f64 / 100.0;
        format!("{}{:.2}", symbol, dollars)
    }

    /// Adds two money values (must be same currency).
    pub fn add(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot add money with different currencies".into());
        }
        Ok(Money {
            amount_cents: self.amount_cents + other.amount_cents,
            currency: self.currency,
        })
    }

    /// Subtracts money (must be same currency).
    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("Cannot subtract money with different currencies".into());
        }
        Ok(Money {
            amount_cents: self.amount_cents - other.amount_cents,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

/// Order number value object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderNumber(pub String);

impl OrderNumber {
    /// Validates and creates an order number.
    pub fn new(number: impl Into<String>) -> Result<Self, String> {
        let number = number.into();
        // Expected format: ORD-YYYY-NNNNNN
        if !number.starts_with("ORD-") {
            return Err("Order number must start with 'ORD-'".into());
        }
        if number.len() < 15 {
            return Err("Invalid order number format".into());
        }
        Ok(Self(number))
    }

    /// Returns the year from the order number.
    pub fn year(&self) -> Option<i32> {
        self.0.get(4..8)?.parse().ok()
    }

    /// Returns the sequence number.
    pub fn sequence(&self) -> Option<i32> {
        self.0.get(9..)?.parse().ok()
    }
}

impl fmt::Display for OrderNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrderNumber {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for OrderNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
