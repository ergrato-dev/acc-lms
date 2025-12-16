// =============================================================================
// Billing Service - Business logic for invoices, payments, and usage
// =============================================================================

use chrono::{Duration, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::{
    BillingEvent, BillingEventType, Invoice, InvoiceLineItem, InvoiceStatus,
    PaymentMethod, Subscription, SubscriptionPlan, UsageRecord,
};
use crate::repository::BillingRepository;

#[derive(Debug, Error)]
pub enum BillingError {
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(Uuid),

    #[error("Payment method not found: {0}")]
    PaymentMethodNotFound(Uuid),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct BillingService {
    repository: BillingRepository,
}

impl BillingService {
    pub fn new(repository: BillingRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // INVOICE OPERATIONS
    // =========================================================================

    pub async fn create_invoice_for_subscription(
        &self,
        subscription: &Subscription,
        plan: &SubscriptionPlan,
    ) -> Result<Invoice, BillingError> {
        let now = Utc::now();
        let invoice_number = self.repository.generate_invoice_number();

        let invoice = Invoice {
            invoice_id: Uuid::new_v4(),
            subscription_id: Some(subscription.subscription_id),
            user_id: subscription.user_id,
            invoice_number,
            status: InvoiceStatus::Draft,
            billing_period_start: subscription.current_period_start,
            billing_period_end: subscription.current_period_end,
            subtotal_cents: plan.price_cents as i64 * subscription.quantity as i64,
            tax_cents: 0,
            discount_cents: 0,
            total_cents: plan.price_cents as i64 * subscription.quantity as i64,
            currency: plan.currency.clone(),
            stripe_invoice_id: None,
            pdf_url: None,
            hosted_invoice_url: None,
            due_date: now + Duration::days(30),
            paid_at: None,
            notes: None,
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.repository.create_invoice(&invoice).await.map_err(Into::into)
    }

    pub async fn get_invoice(&self, invoice_id: Uuid) -> Result<Invoice, BillingError> {
        self.repository
            .get_invoice_by_id(invoice_id)
            .await?
            .ok_or(BillingError::InvoiceNotFound(invoice_id))
    }

    pub async fn list_invoices_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Invoice>, BillingError> {
        self.repository.list_invoices_for_user(user_id, limit, offset).await.map_err(Into::into)
    }

    pub async fn list_invoices_for_subscription(
        &self,
        subscription_id: Uuid,
    ) -> Result<Vec<Invoice>, BillingError> {
        self.repository.list_invoices_for_subscription(subscription_id).await.map_err(Into::into)
    }

    pub async fn mark_invoice_paid(&self, invoice_id: Uuid) -> Result<(), BillingError> {
        let invoice = self.get_invoice(invoice_id).await?;

        if invoice.status == InvoiceStatus::Paid {
            return Ok(());
        }

        self.repository.update_invoice_status(invoice_id, InvoiceStatus::Paid, Some(Utc::now())).await.map_err(Into::into)
    }

    pub async fn void_invoice(&self, invoice_id: Uuid) -> Result<(), BillingError> {
        let invoice = self.get_invoice(invoice_id).await?;

        if invoice.status == InvoiceStatus::Paid {
            return Err(BillingError::InvalidOperation("Cannot void a paid invoice".into()));
        }

        self.repository.update_invoice_status(invoice_id, InvoiceStatus::Void, None).await.map_err(Into::into)
    }

    pub async fn add_line_item(&self, item: &InvoiceLineItem) -> Result<(), BillingError> {
        self.repository.add_line_item(item).await.map_err(Into::into)
    }

    pub async fn get_line_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceLineItem>, BillingError> {
        self.repository.get_line_items(invoice_id).await.map_err(Into::into)
    }

    pub async fn find_overdue_invoices(&self) -> Result<Vec<Invoice>, BillingError> {
        self.repository.find_overdue_invoices().await.map_err(Into::into)
    }

    // =========================================================================
    // PAYMENT METHOD OPERATIONS
    // =========================================================================

    pub async fn add_payment_method(&self, method: &PaymentMethod) -> Result<(), BillingError> {
        self.repository.add_payment_method(method).await.map_err(Into::into)
    }

    pub async fn list_payment_methods(&self, user_id: Uuid) -> Result<Vec<PaymentMethod>, BillingError> {
        self.repository.list_payment_methods(user_id).await.map_err(Into::into)
    }

    pub async fn get_payment_method(&self, payment_method_id: Uuid) -> Result<PaymentMethod, BillingError> {
        self.repository
            .get_payment_method(payment_method_id)
            .await?
            .ok_or(BillingError::PaymentMethodNotFound(payment_method_id))
    }

    pub async fn set_default_payment_method(
        &self,
        user_id: Uuid,
        payment_method_id: Uuid,
    ) -> Result<(), BillingError> {
        // Verify payment method exists and belongs to user
        let method = self.get_payment_method(payment_method_id).await?;
        if method.user_id != user_id {
            return Err(BillingError::InvalidOperation("Payment method does not belong to user".into()));
        }

        self.repository.set_default_payment_method(user_id, payment_method_id).await.map_err(Into::into)
    }

    pub async fn delete_payment_method(
        &self,
        user_id: Uuid,
        payment_method_id: Uuid,
    ) -> Result<(), BillingError> {
        let method = self.get_payment_method(payment_method_id).await?;
        if method.user_id != user_id {
            return Err(BillingError::InvalidOperation("Payment method does not belong to user".into()));
        }

        self.repository.delete_payment_method(payment_method_id).await.map_err(Into::into)
    }

    // =========================================================================
    // USAGE RECORD OPERATIONS
    // =========================================================================

    pub async fn record_usage(
        &self,
        subscription_id: Uuid,
        feature_key: String,
        quantity: i64,
        action: String,
    ) -> Result<UsageRecord, BillingError> {
        let now = Utc::now();

        let record = UsageRecord {
            record_id: Uuid::new_v4(),
            subscription_id,
            feature_key,
            quantity,
            timestamp: now,
            action,
            metadata: serde_json::json!({}),
        };

        self.repository.record_usage(&record).await?;
        Ok(record)
    }

    pub async fn get_usage_summary(
        &self,
        subscription_id: Uuid,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
    ) -> Result<Vec<(String, i64)>, BillingError> {
        self.repository.get_usage_summary(subscription_id, start, end).await.map_err(Into::into)
    }

    // =========================================================================
    // BILLING EVENT OPERATIONS
    // =========================================================================

    pub async fn log_event(
        &self,
        subscription_id: Option<Uuid>,
        user_id: Uuid,
        event_type: BillingEventType,
        description: String,
        amount_cents: Option<i64>,
        currency: Option<String>,
        invoice_id: Option<Uuid>,
    ) -> Result<BillingEvent, BillingError> {
        let now = Utc::now();

        let event = BillingEvent {
            event_id: Uuid::new_v4(),
            subscription_id,
            user_id,
            event_type,
            description,
            amount_cents,
            currency,
            invoice_id,
            stripe_event_id: None,
            metadata: serde_json::json!({}),
            created_at: now,
        };

        self.repository.log_event(&event).await?;
        Ok(event)
    }

    pub async fn list_events_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BillingEvent>, BillingError> {
        self.repository.list_events_for_user(user_id, limit, offset).await.map_err(Into::into)
    }
}
