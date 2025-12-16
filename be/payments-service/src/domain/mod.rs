//! # Payments Domain Module
//!
//! Core domain entities, events, and value objects for the payments service.

pub mod entities;
pub mod events;
pub mod value_objects;

// Re-export commonly used types
pub use entities::{
    Order, NewOrder, UpdateOrder, OrderStatus,
    Transaction, NewTransaction, TransactionType, TransactionStatus,
    DiscountCode, NewDiscountCode, UpdateDiscountCode, DiscountType,
    Review, NewReview, UpdateReview,
    OrderWithTransactions,
};

pub use events::{OrderEvent, TransactionEvent, ReviewEvent};

pub use value_objects::{
    OrderId, TransactionId, DiscountCodeId, ReviewId,
    Money, OrderNumber,
};
