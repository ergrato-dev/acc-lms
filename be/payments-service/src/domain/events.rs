//! # Payments Domain Events
//!
//! Domain events for cross-service communication.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::{OrderStatus, TransactionType};

// =============================================================================
// ORDER EVENTS
// =============================================================================

/// Events related to order lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum OrderEvent {
    /// Order was created
    Created {
        order_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        total_cents: i32,
        currency: String,
        timestamp: DateTime<Utc>,
    },
    /// Order status changed
    StatusChanged {
        order_id: Uuid,
        user_id: Uuid,
        previous_status: OrderStatus,
        new_status: OrderStatus,
        timestamp: DateTime<Utc>,
    },
    /// Order was paid
    Paid {
        order_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        amount_cents: i32,
        currency: String,
        transaction_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Order was refunded
    Refunded {
        order_id: Uuid,
        user_id: Uuid,
        course_id: Uuid,
        amount_cents: i32,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// Order was cancelled
    Cancelled {
        order_id: Uuid,
        user_id: Uuid,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
}

impl OrderEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            OrderEvent::Created { .. } => "order.created",
            OrderEvent::StatusChanged { .. } => "order.status_changed",
            OrderEvent::Paid { .. } => "order.paid",
            OrderEvent::Refunded { .. } => "order.refunded",
            OrderEvent::Cancelled { .. } => "order.cancelled",
        }
    }

    /// Returns the order ID.
    pub fn order_id(&self) -> Uuid {
        match self {
            OrderEvent::Created { order_id, .. } => *order_id,
            OrderEvent::StatusChanged { order_id, .. } => *order_id,
            OrderEvent::Paid { order_id, .. } => *order_id,
            OrderEvent::Refunded { order_id, .. } => *order_id,
            OrderEvent::Cancelled { order_id, .. } => *order_id,
        }
    }

    /// Returns the user ID.
    pub fn user_id(&self) -> Uuid {
        match self {
            OrderEvent::Created { user_id, .. } => *user_id,
            OrderEvent::StatusChanged { user_id, .. } => *user_id,
            OrderEvent::Paid { user_id, .. } => *user_id,
            OrderEvent::Refunded { user_id, .. } => *user_id,
            OrderEvent::Cancelled { user_id, .. } => *user_id,
        }
    }
}

// =============================================================================
// TRANSACTION EVENTS
// =============================================================================

/// Events related to payment transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum TransactionEvent {
    /// Transaction was processed
    Processed {
        transaction_id: Uuid,
        order_id: Uuid,
        transaction_type: TransactionType,
        amount_cents: i32,
        currency: String,
        status: String,
        timestamp: DateTime<Utc>,
    },
    /// Transaction succeeded
    Succeeded {
        transaction_id: Uuid,
        order_id: Uuid,
        amount_cents: i32,
        timestamp: DateTime<Utc>,
    },
    /// Transaction failed
    Failed {
        transaction_id: Uuid,
        order_id: Uuid,
        error_code: Option<String>,
        error_message: Option<String>,
        timestamp: DateTime<Utc>,
    },
}

impl TransactionEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            TransactionEvent::Processed { .. } => "transaction.processed",
            TransactionEvent::Succeeded { .. } => "transaction.succeeded",
            TransactionEvent::Failed { .. } => "transaction.failed",
        }
    }

    /// Returns the transaction ID.
    pub fn transaction_id(&self) -> Uuid {
        match self {
            TransactionEvent::Processed { transaction_id, .. } => *transaction_id,
            TransactionEvent::Succeeded { transaction_id, .. } => *transaction_id,
            TransactionEvent::Failed { transaction_id, .. } => *transaction_id,
        }
    }
}

// =============================================================================
// REVIEW EVENTS
// =============================================================================

/// Events related to reviews.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum ReviewEvent {
    /// Review was created
    Created {
        review_id: Uuid,
        course_id: Uuid,
        user_id: Uuid,
        rating: i32,
        timestamp: DateTime<Utc>,
    },
    /// Review was updated
    Updated {
        review_id: Uuid,
        course_id: Uuid,
        previous_rating: i32,
        new_rating: i32,
        timestamp: DateTime<Utc>,
    },
    /// Review was deleted
    Deleted {
        review_id: Uuid,
        course_id: Uuid,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Review received a helpful vote
    HelpfulVote {
        review_id: Uuid,
        voter_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

impl ReviewEvent {
    /// Returns the event type as a string.
    pub fn event_type(&self) -> &'static str {
        match self {
            ReviewEvent::Created { .. } => "review.created",
            ReviewEvent::Updated { .. } => "review.updated",
            ReviewEvent::Deleted { .. } => "review.deleted",
            ReviewEvent::HelpfulVote { .. } => "review.helpful_vote",
        }
    }

    /// Returns the review ID.
    pub fn review_id(&self) -> Uuid {
        match self {
            ReviewEvent::Created { review_id, .. } => *review_id,
            ReviewEvent::Updated { review_id, .. } => *review_id,
            ReviewEvent::Deleted { review_id, .. } => *review_id,
            ReviewEvent::HelpfulVote { review_id, .. } => *review_id,
        }
    }
}
