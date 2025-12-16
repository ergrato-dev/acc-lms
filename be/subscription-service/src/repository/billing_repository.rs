// =============================================================================
// Billing Repository - PostgreSQL persistence for billing entities
// =============================================================================

use chrono::{DateTime, Utc};
use rand::Rng;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{
    BillingEvent, BillingEventType, CardBrand, Invoice, InvoiceLineItem,
    InvoiceStatus, PaymentMethod, PaymentMethodType, UsageRecord,
};

/// Repository for billing-related persistence
pub struct BillingRepository {
    pool: PgPool,
}

impl BillingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // INVOICES
    // =========================================================================

    /// Create a new invoice
    pub async fn create_invoice(&self, invoice: &Invoice) -> Result<Invoice, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.invoices (
                invoice_id, subscription_id, user_id, invoice_number, status,
                subtotal_cents, tax_cents, discount_cents, total_cents, currency,
                billing_period_start, billing_period_end, due_date, paid_at,
                stripe_invoice_id, pdf_url, hosted_invoice_url, notes,
                metadata, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            "#,
        )
        .bind(invoice.invoice_id)
        .bind(invoice.subscription_id)
        .bind(invoice.user_id)
        .bind(&invoice.invoice_number)
        .bind(invoice.status.to_string())
        .bind(invoice.subtotal_cents)
        .bind(invoice.tax_cents)
        .bind(invoice.discount_cents)
        .bind(invoice.total_cents)
        .bind(&invoice.currency)
        .bind(invoice.billing_period_start)
        .bind(invoice.billing_period_end)
        .bind(invoice.due_date)
        .bind(invoice.paid_at)
        .bind(&invoice.stripe_invoice_id)
        .bind(&invoice.pdf_url)
        .bind(&invoice.hosted_invoice_url)
        .bind(&invoice.notes)
        .bind(&invoice.metadata)
        .bind(invoice.created_at)
        .bind(invoice.updated_at)
        .execute(&self.pool)
        .await?;

        // Return the invoice as-is since we just inserted it
        Ok(invoice.clone())
    }

    /// Get invoice by ID
    pub async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Option<Invoice>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.invoices WHERE invoice_id = $1"#,
        )
        .bind(invoice_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_invoice(&r)?)),
            None => Ok(None),
        }
    }

    /// List invoices for user
    pub async fn list_invoices_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Invoice>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.invoices
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_invoice(r)).collect()
    }

    /// List invoices for subscription
    pub async fn list_invoices_for_subscription(&self, subscription_id: Uuid) -> Result<Vec<Invoice>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.invoices
            WHERE subscription_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(subscription_id)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_invoice(r)).collect()
    }

    /// Update invoice status
    pub async fn update_invoice_status(
        &self,
        invoice_id: Uuid,
        status: InvoiceStatus,
        paid_at: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE subscriptions.invoices SET
                status = $2, paid_at = $3, updated_at = $4
            WHERE invoice_id = $1
            "#,
        )
        .bind(invoice_id)
        .bind(status.to_string())
        .bind(paid_at)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Find overdue invoices
    pub async fn find_overdue_invoices(&self) -> Result<Vec<Invoice>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.invoices
            WHERE status IN ('draft', 'open') AND due_date < NOW()
            ORDER BY due_date
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_invoice(r)).collect()
    }

    /// Convert row to Invoice
    fn row_to_invoice(&self, row: &sqlx::postgres::PgRow) -> Result<Invoice, sqlx::Error> {
        let status_str: String = row.try_get("status")?;

        Ok(Invoice {
            invoice_id: row.try_get("invoice_id")?,
            subscription_id: row.try_get("subscription_id")?,
            user_id: row.try_get("user_id")?,
            invoice_number: row.try_get("invoice_number")?,
            status: status_str.parse().unwrap_or(InvoiceStatus::Draft),
            subtotal_cents: row.try_get("subtotal_cents")?,
            tax_cents: row.try_get("tax_cents")?,
            discount_cents: row.try_get("discount_cents")?,
            total_cents: row.try_get("total_cents")?,
            currency: row.try_get("currency")?,
            billing_period_start: row.try_get("billing_period_start")?,
            billing_period_end: row.try_get("billing_period_end")?,
            due_date: row.try_get("due_date")?,
            paid_at: row.try_get("paid_at")?,
            stripe_invoice_id: row.try_get("stripe_invoice_id")?,
            pdf_url: row.try_get("pdf_url")?,
            hosted_invoice_url: row.try_get("hosted_invoice_url")?,
            notes: row.try_get("notes")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    // =========================================================================
    // INVOICE LINE ITEMS
    // =========================================================================

    /// Add line item to invoice
    pub async fn add_line_item(&self, item: &InvoiceLineItem) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.invoice_line_items (
                line_item_id, invoice_id, description, quantity, unit_price_cents,
                total_cents, period_start, period_end, proration
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(item.line_item_id)
        .bind(item.invoice_id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(item.unit_price_cents)
        .bind(item.total_cents)
        .bind(item.period_start)
        .bind(item.period_end)
        .bind(item.proration)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get line items for invoice
    pub async fn get_line_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceLineItem>, sqlx::Error> {
        let rows = sqlx::query(
            r#"SELECT * FROM subscriptions.invoice_line_items WHERE invoice_id = $1"#,
        )
        .bind(invoice_id)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| {
            Ok(InvoiceLineItem {
                line_item_id: r.try_get("line_item_id")?,
                invoice_id: r.try_get("invoice_id")?,
                description: r.try_get("description")?,
                quantity: r.try_get("quantity")?,
                unit_price_cents: r.try_get("unit_price_cents")?,
                total_cents: r.try_get("total_cents")?,
                period_start: r.try_get("period_start")?,
                period_end: r.try_get("period_end")?,
                proration: r.try_get("proration")?,
            })
        }).collect()
    }

    // =========================================================================
    // PAYMENT METHODS
    // =========================================================================

    /// Add payment method
    pub async fn add_payment_method(&self, pm: &PaymentMethod) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.payment_methods (
                payment_method_id, user_id, method_type, stripe_payment_method_id,
                card_brand, card_last_four, card_exp_month, card_exp_year,
                is_default, billing_name, billing_email,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(pm.payment_method_id)
        .bind(pm.user_id)
        .bind(pm.method_type.to_string())
        .bind(&pm.stripe_payment_method_id)
        .bind(pm.card_brand.as_ref().map(|b| b.to_string()))
        .bind(&pm.card_last_four)
        .bind(pm.card_exp_month)
        .bind(pm.card_exp_year)
        .bind(pm.is_default)
        .bind(&pm.billing_name)
        .bind(&pm.billing_email)
        .bind(pm.created_at)
        .bind(pm.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List payment methods for user
    pub async fn list_payment_methods(&self, user_id: Uuid) -> Result<Vec<PaymentMethod>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.payment_methods
            WHERE user_id = $1
            ORDER BY is_default DESC, created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_payment_method(r)).collect()
    }

    /// Get payment method by ID
    pub async fn get_payment_method(&self, payment_method_id: Uuid) -> Result<Option<PaymentMethod>, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT * FROM subscriptions.payment_methods WHERE payment_method_id = $1"#,
        )
        .bind(payment_method_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(self.row_to_payment_method(&r)?)),
            None => Ok(None),
        }
    }

    /// Set default payment method
    pub async fn set_default_payment_method(&self, user_id: Uuid, payment_method_id: Uuid) -> Result<(), sqlx::Error> {
        // First, unset all defaults for user
        sqlx::query(
            r#"
            UPDATE subscriptions.payment_methods SET is_default = false, updated_at = $2
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        // Set the specified one as default
        sqlx::query(
            r#"
            UPDATE subscriptions.payment_methods SET is_default = true, updated_at = $2
            WHERE payment_method_id = $1
            "#,
        )
        .bind(payment_method_id)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete payment method
    pub async fn delete_payment_method(&self, payment_method_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"DELETE FROM subscriptions.payment_methods WHERE payment_method_id = $1"#,
        )
        .bind(payment_method_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Convert row to PaymentMethod
    fn row_to_payment_method(&self, row: &sqlx::postgres::PgRow) -> Result<PaymentMethod, sqlx::Error> {
        let method_type_str: String = row.try_get("method_type")?;
        let card_brand_str: Option<String> = row.try_get("card_brand")?;

        let card_brand = card_brand_str.and_then(|s| match s.as_str() {
            "visa" => Some(CardBrand::Visa),
            "mastercard" => Some(CardBrand::Mastercard),
            "amex" => Some(CardBrand::Amex),
            "discover" => Some(CardBrand::Discover),
            "diners" => Some(CardBrand::Diners),
            "jcb" => Some(CardBrand::Jcb),
            "unionpay" => Some(CardBrand::UnionPay),
            _ => Some(CardBrand::Unknown),
        });

        let method_type = match method_type_str.as_str() {
            "card" => PaymentMethodType::Card,
            "bank_account" => PaymentMethodType::BankAccount,
            "sepa" => PaymentMethodType::Sepa,
            "ideal" => PaymentMethodType::Ideal,
            "boleto" => PaymentMethodType::Boleto,
            "pix" => PaymentMethodType::Pix,
            "oxxo" => PaymentMethodType::Oxxo,
            _ => PaymentMethodType::Card,
        };

        Ok(PaymentMethod {
            payment_method_id: row.try_get("payment_method_id")?,
            user_id: row.try_get("user_id")?,
            method_type,
            stripe_payment_method_id: row.try_get("stripe_payment_method_id")?,
            card_brand,
            card_last_four: row.try_get("card_last_four")?,
            card_exp_month: row.try_get("card_exp_month")?,
            card_exp_year: row.try_get("card_exp_year")?,
            is_default: row.try_get("is_default")?,
            billing_name: row.try_get("billing_name")?,
            billing_email: row.try_get("billing_email")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    // =========================================================================
    // USAGE RECORDS
    // =========================================================================

    /// Record usage
    pub async fn record_usage(&self, record: &UsageRecord) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.usage_records (
                record_id, subscription_id, feature_key, quantity, timestamp,
                action, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(record.record_id)
        .bind(record.subscription_id)
        .bind(&record.feature_key)
        .bind(record.quantity)
        .bind(record.timestamp)
        .bind(&record.action)
        .bind(&record.metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get usage summary for period
    pub async fn get_usage_summary(
        &self,
        subscription_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Vec<(String, i64)>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                feature_key,
                SUM(quantity) as total_quantity
            FROM subscriptions.usage_records
            WHERE subscription_id = $1 AND timestamp >= $2 AND timestamp < $3
            GROUP BY feature_key
            "#,
        )
        .bind(subscription_id)
        .bind(period_start)
        .bind(period_end)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| {
            Ok((
                r.try_get::<String, _>("feature_key")?,
                r.try_get::<i64, _>("total_quantity")?,
            ))
        }).collect()
    }

    // =========================================================================
    // BILLING EVENTS
    // =========================================================================

    /// Log billing event
    pub async fn log_event(&self, event: &BillingEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO subscriptions.billing_events (
                event_id, subscription_id, user_id, event_type, description,
                amount_cents, currency, invoice_id, stripe_event_id, metadata,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(event.event_id)
        .bind(event.subscription_id)
        .bind(event.user_id)
        .bind(event.event_type.to_string())
        .bind(&event.description)
        .bind(event.amount_cents)
        .bind(&event.currency)
        .bind(event.invoice_id)
        .bind(&event.stripe_event_id)
        .bind(&event.metadata)
        .bind(event.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List billing events for user
    pub async fn list_events_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BillingEvent>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM subscriptions.billing_events
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(|r| self.row_to_billing_event(r)).collect()
    }

    /// Count billing events for user
    pub async fn count_events_for_user(&self, user_id: Uuid) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"SELECT COUNT(*) as count FROM subscriptions.billing_events WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get::<i64, _>("count")?)
    }

    /// Convert row to BillingEvent
    fn row_to_billing_event(&self, row: &sqlx::postgres::PgRow) -> Result<BillingEvent, sqlx::Error> {
        let event_type_str: String = row.try_get("event_type")?;

        let event_type = match event_type_str.as_str() {
            "subscription_created" => BillingEventType::SubscriptionCreated,
            "subscription_activated" => BillingEventType::SubscriptionActivated,
            "subscription_trial_started" => BillingEventType::SubscriptionTrialStarted,
            "subscription_trial_ended" => BillingEventType::SubscriptionTrialEnded,
            "subscription_renewed" => BillingEventType::SubscriptionRenewed,
            "subscription_upgraded" => BillingEventType::SubscriptionUpgraded,
            "subscription_downgraded" => BillingEventType::SubscriptionDowngraded,
            "subscription_cancelled" => BillingEventType::SubscriptionCancelled,
            "subscription_expired" => BillingEventType::SubscriptionExpired,
            "subscription_reactivated" => BillingEventType::SubscriptionReactivated,
            "payment_succeeded" => BillingEventType::PaymentSucceeded,
            "payment_failed" => BillingEventType::PaymentFailed,
            "payment_refunded" => BillingEventType::PaymentRefunded,
            "invoice_created" => BillingEventType::InvoiceCreated,
            "invoice_paid" => BillingEventType::InvoicePaid,
            "invoice_voided" => BillingEventType::InvoiceVoided,
            "payment_method_added" => BillingEventType::PaymentMethodAdded,
            "payment_method_updated" => BillingEventType::PaymentMethodUpdated,
            "payment_method_removed" => BillingEventType::PaymentMethodRemoved,
            _ => BillingEventType::SubscriptionCreated,
        };

        Ok(BillingEvent {
            event_id: row.try_get("event_id")?,
            subscription_id: row.try_get("subscription_id")?,
            user_id: row.try_get("user_id")?,
            event_type,
            description: row.try_get("description")?,
            amount_cents: row.try_get("amount_cents")?,
            currency: row.try_get("currency")?,
            invoice_id: row.try_get("invoice_id")?,
            stripe_event_id: row.try_get("stripe_event_id")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
        })
    }

    // =========================================================================
    // UTILITY FUNCTIONS
    // =========================================================================

    /// Generate unique invoice number
    pub fn generate_invoice_number(&self) -> String {
        let mut rng = rand::thread_rng();
        let random: u32 = rng.gen_range(100000..999999);
        let timestamp = Utc::now().format("%Y%m");
        format!("INV-{}-{}", timestamp, random)
    }
}
