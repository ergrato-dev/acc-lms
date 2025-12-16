// =============================================================================
// HTTP Handlers - Request handlers for subscription endpoints
// =============================================================================

use actix_web::{web, HttpResponse};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::dto::*;
use crate::domain::entities::*;
use crate::service::{BillingService, CouponService, SubscriptionError, SubscriptionService};

/// Application state shared across handlers
pub struct AppState {
    pub subscription_service: Arc<SubscriptionService>,
    pub billing_service: Arc<BillingService>,
    pub coupon_service: Arc<CouponService>,
}

// =============================================================================
// PLAN HANDLERS
// =============================================================================

/// Get all public plans
pub async fn get_plans(state: web::Data<AppState>) -> HttpResponse {
    match state.subscription_service.list_public_plans().await {
        Ok(plans) => {
            let response: Vec<PlanResponse> = plans.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get all plans (admin)
pub async fn get_all_plans(state: web::Data<AppState>) -> HttpResponse {
    match state.subscription_service.list_all_plans().await {
        Ok(plans) => {
            let response: Vec<PlanResponse> = plans.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get plan by ID
pub async fn get_plan(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let plan_id = path.into_inner();

    match state.subscription_service.get_plan(plan_id).await {
        Ok(plan) => HttpResponse::Ok().json(ApiResponse::success(PlanResponse::from(plan))),
        Err(SubscriptionError::PlanNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Plan not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get plan by slug
pub async fn get_plan_by_slug(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let slug = path.into_inner();

    match state.subscription_service.get_plan_by_slug(&slug).await {
        Ok(Some(plan)) => HttpResponse::Ok().json(ApiResponse::success(PlanResponse::from(plan))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error("Plan not found")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Create a new plan (admin only)
pub async fn create_plan(state: web::Data<AppState>, body: web::Json<CreatePlanRequest>) -> HttpResponse {
    let now = Utc::now();

    let plan = SubscriptionPlan {
        plan_id: Uuid::new_v4(),
        name: body.name.clone(),
        slug: body.slug.clone(),
        description: body.description.clone(),
        tier: body.tier,
        billing_interval: body.billing_interval,
        price_cents: body.price_cents,
        currency: body.currency.clone(),
        trial_days: body.trial_days.unwrap_or(0),
        features: body.features.clone(),
        limits: body.limits.clone(),
        stripe_price_id: body.stripe_price_id.clone(),
        is_active: true,
        is_public: body.is_public.unwrap_or(true),
        sort_order: body.sort_order.unwrap_or(0),
        metadata: serde_json::json!({}),
        created_at: now,
        updated_at: now,
    };

    match state.subscription_service.create_plan(plan).await {
        Ok(plan) => HttpResponse::Created().json(ApiResponse::success(PlanResponse::from(plan))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Update a plan (admin only)
pub async fn update_plan(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePlanRequest>,
) -> HttpResponse {
    let plan_id = path.into_inner();

    let plan = match state.subscription_service.get_plan(plan_id).await {
        Ok(p) => p,
        Err(SubscriptionError::PlanNotFound(_)) => {
            return HttpResponse::NotFound().json(ApiResponse::<()>::error("Plan not found"));
        }
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    };

    let updated_plan = SubscriptionPlan {
        plan_id: plan.plan_id,
        name: body.name.clone().unwrap_or(plan.name),
        slug: plan.slug,
        description: body.description.clone().or(plan.description),
        tier: plan.tier,
        billing_interval: plan.billing_interval,
        price_cents: body.price_cents.unwrap_or(plan.price_cents),
        currency: plan.currency,
        trial_days: body.trial_days.unwrap_or(plan.trial_days),
        features: body.features.clone().unwrap_or(plan.features),
        limits: body.limits.clone().unwrap_or(plan.limits),
        stripe_price_id: plan.stripe_price_id,
        is_active: body.is_active.unwrap_or(plan.is_active),
        is_public: body.is_public.unwrap_or(plan.is_public),
        sort_order: body.sort_order.unwrap_or(plan.sort_order),
        metadata: plan.metadata,
        created_at: plan.created_at,
        updated_at: Utc::now(),
    };

    match state.subscription_service.update_plan(updated_plan).await {
        Ok(plan) => HttpResponse::Ok().json(ApiResponse::success(PlanResponse::from(plan))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// SUBSCRIPTION HANDLERS
// =============================================================================

/// Create subscription
pub async fn create_subscription(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<CreateSubscriptionRequest>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.subscription_service.create_subscription(
        user,
        body.plan_id,
        body.organization_id,
    ).await {
        Ok(sub) => HttpResponse::Created().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Err(SubscriptionError::AlreadySubscribed) => {
            HttpResponse::Conflict().json(ApiResponse::<()>::error("User already has an active subscription"))
        }
        Err(SubscriptionError::PlanNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Plan not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get current subscription
pub async fn get_my_subscription(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.subscription_service.get_active_subscription(user).await {
        Ok(Some(sub)) => HttpResponse::Ok().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error("No active subscription")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get subscription by ID
pub async fn get_subscription(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let subscription_id = path.into_inner();

    match state.subscription_service.get_subscription(subscription_id).await {
        Ok(sub) => HttpResponse::Ok().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Err(SubscriptionError::SubscriptionNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Subscription not found"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// List user subscriptions
pub async fn list_subscriptions(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.subscription_service.list_subscriptions(user, query.limit(), query.offset()).await {
        Ok(subs) => {
            let response: Vec<SubscriptionResponse> = subs.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Change subscription plan
pub async fn change_plan(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<ChangePlanRequest>,
) -> HttpResponse {
    let subscription_id = path.into_inner();

    match state.subscription_service.change_plan(subscription_id, body.new_plan_id).await {
        Ok(sub) => HttpResponse::Ok().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Err(SubscriptionError::SubscriptionNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Subscription not found"))
        }
        Err(SubscriptionError::SamePlan) => {
            HttpResponse::BadRequest().json(ApiResponse::<()>::error("Cannot change to the same plan"))
        }
        Err(SubscriptionError::NotActive) => {
            HttpResponse::BadRequest().json(ApiResponse::<()>::error("Subscription is not active"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Cancel subscription
pub async fn cancel_subscription(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<CancelSubscriptionRequest>,
) -> HttpResponse {
    let subscription_id = path.into_inner();

    // Parse reason if provided
    let reason = body.reason.as_ref().and_then(|r| match r.as_str() {
        "user_requested" => Some(CancellationReason::UserRequested),
        "payment_failed" => Some(CancellationReason::PaymentFailed),
        "fraud" => Some(CancellationReason::Fraud),
        "policy_violation" => Some(CancellationReason::PolicyViolation),
        "downgrade_to_plan" => Some(CancellationReason::DowngradeToPlan),
        "migration" => Some(CancellationReason::Migration),
        _ => Some(CancellationReason::Other),
    });

    match state.subscription_service.cancel_subscription(
        subscription_id,
        reason,
        body.feedback.clone(),
        body.cancel_immediately.unwrap_or(false),
    ).await {
        Ok(sub) => HttpResponse::Ok().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Err(SubscriptionError::SubscriptionNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Subscription not found"))
        }
        Err(SubscriptionError::NotActive) => {
            HttpResponse::BadRequest().json(ApiResponse::<()>::error("Subscription is not active"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Reactivate subscription
pub async fn reactivate_subscription(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let subscription_id = path.into_inner();

    match state.subscription_service.reactivate_subscription(subscription_id).await {
        Ok(sub) => HttpResponse::Ok().json(ApiResponse::success(SubscriptionResponse::from(sub))),
        Err(SubscriptionError::SubscriptionNotFound(_)) => {
            HttpResponse::NotFound().json(ApiResponse::<()>::error("Subscription not found"))
        }
        Err(SubscriptionError::NotActive) => {
            HttpResponse::BadRequest().json(ApiResponse::<()>::error("Cannot reactivate this subscription"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// INVOICE HANDLERS
// =============================================================================

/// Get invoice by ID
pub async fn get_invoice(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let invoice_id = path.into_inner();

    match state.billing_service.get_invoice(invoice_id).await {
        Ok(invoice) => HttpResponse::Ok().json(ApiResponse::success(InvoiceResponse::from(invoice))),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::<()>::error("Invoice not found")),
    }
}

/// List user invoices
pub async fn list_invoices(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.billing_service.list_invoices_for_user(user, query.limit(), query.offset()).await {
        Ok(invoices) => {
            let response: Vec<InvoiceResponse> = invoices.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get invoice line items
pub async fn get_invoice_line_items(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let invoice_id = path.into_inner();

    match state.billing_service.get_line_items(invoice_id).await {
        Ok(items) => {
            let response: Vec<InvoiceLineItemResponse> = items.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// PAYMENT METHOD HANDLERS
// =============================================================================

/// List payment methods
pub async fn list_payment_methods(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.billing_service.list_payment_methods(user).await {
        Ok(methods) => {
            let response: Vec<PaymentMethodResponse> = methods.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Add payment method
pub async fn add_payment_method(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<AddPaymentMethodRequest>,
) -> HttpResponse {
    let user = user_id.into_inner();
    let now = Utc::now();

    let method = PaymentMethod {
        payment_method_id: Uuid::new_v4(),
        user_id: user,
        method_type: PaymentMethodType::Card,
        stripe_payment_method_id: Some(body.stripe_payment_method_id.clone()),
        is_default: body.set_as_default.unwrap_or(false),
        card_brand: None,
        card_last_four: None,
        card_exp_month: None,
        card_exp_year: None,
        billing_name: None,
        billing_email: None,
        created_at: now,
        updated_at: now,
    };

    let response = PaymentMethodResponse::from(method.clone());
    match state.billing_service.add_payment_method(&method).await {
        Ok(()) => HttpResponse::Created().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Delete payment method
pub async fn delete_payment_method(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user = user_id.into_inner();
    let payment_method_id = path.into_inner();

    match state.billing_service.delete_payment_method(user, payment_method_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json(ApiResponse::<()>::error("Payment method not found")),
    }
}

// =============================================================================
// USAGE HANDLERS
// =============================================================================

/// Record usage
pub async fn record_usage(
    state: web::Data<AppState>,
    body: web::Json<RecordUsageRequest>,
) -> HttpResponse {
    let action = body.action.clone().unwrap_or_else(|| "increment".to_string());

    match state.billing_service.record_usage(
        body.subscription_id,
        body.feature_key.clone(),
        body.quantity,
        action,
    ).await {
        Ok(record) => HttpResponse::Created().json(ApiResponse::success(UsageRecordResponse::from(record))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// COUPON HANDLERS
// =============================================================================

/// Validate coupon
pub async fn validate_coupon(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    body: web::Json<ValidateCouponRequest>,
) -> HttpResponse {
    let user = user_id.into_inner();

    // Check if user has existing subscription (for first-time check)
    let is_first = match state.subscription_service.get_active_subscription(user).await {
        Ok(None) => true,
        _ => false,
    };

    match state.coupon_service.validate_coupon(
        &body.code,
        Some(body.plan_id),
        user,
        0, // amount_cents - would need plan price
        is_first,
    ).await {
        Ok(coupon) => {
            HttpResponse::Ok().json(ApiResponse::success(CouponResponse::from(coupon)))
        }
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// List active coupons (admin)
pub async fn get_coupons(state: web::Data<AppState>) -> HttpResponse {
    match state.coupon_service.list_active_coupons().await {
        Ok(coupons) => {
            let response: Vec<CouponResponse> = coupons.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Create coupon (admin)
pub async fn create_coupon(
    state: web::Data<AppState>,
    body: web::Json<CreateCouponRequest>,
) -> HttpResponse {
    let now = Utc::now();

    let coupon = Coupon {
        coupon_id: Uuid::new_v4(),
        code: body.code.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        discount_type: body.discount_type,
        discount_value: body.discount_value,
        max_redemptions: body.max_redemptions,
        current_redemptions: 0,
        valid_from: body.valid_from,
        valid_until: body.valid_until,
        min_amount_cents: None,
        applies_to_plans: body.applies_to_plans.as_ref().map(|p| serde_json::to_value(p).unwrap_or_default()),
        first_time_only: body.first_time_only.unwrap_or(false),
        is_active: true,
        metadata: serde_json::json!({}),
        created_at: now,
    };

    match state.coupon_service.create_coupon(coupon).await {
        Ok(c) => HttpResponse::Created().json(ApiResponse::success(CouponResponse::from(c))),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Deactivate coupon (admin)
pub async fn deactivate_coupon(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let coupon_id = path.into_inner();

    match state.coupon_service.deactivate_coupon(coupon_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// BILLING EVENTS HANDLERS
// =============================================================================

/// List billing events
pub async fn list_billing_events(
    state: web::Data<AppState>,
    user_id: web::ReqData<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let user = user_id.into_inner();

    match state.billing_service.list_events_for_user(user, query.limit(), query.offset()).await {
        Ok(events) => {
            let response: Vec<BillingEventResponse> = events.into_iter().map(Into::into).collect();
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

// =============================================================================
// WEBHOOK HANDLERS
// =============================================================================

/// Stripe webhook handler
pub async fn stripe_webhook(_body: web::Bytes) -> HttpResponse {
    // TODO: Implement Stripe webhook processing
    HttpResponse::Ok().finish()
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "subscription-service"
    }))
}
