//! Subscription Service - LMS Subscription & Billing Management
//!
//! This service handles all subscription-related functionality:
//! - Subscription plan management
//! - User subscription lifecycle (create, upgrade, cancel)
//! - Billing and invoicing
//! - Payment method management
//! - Usage tracking and metering
//! - Coupon/discount management

pub mod domain;
pub mod repository;
pub mod service;
pub mod api;

pub use domain::*;
pub use repository::*;
pub use service::*;
pub use api::*;
