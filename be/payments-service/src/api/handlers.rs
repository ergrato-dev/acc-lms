//! # HTTP Handlers
//!
//! Request handlers for the payments API.

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::*;
use crate::domain::{
    DiscountType, NewDiscountCode, NewOrder, NewReview, NewTransaction,
    OrderStatus, TransactionStatus, TransactionType, UpdateDiscountCode,
    UpdateOrder, UpdateReview,
};
use crate::service::{PaymentError, PaymentService};

/// Application state containing the payment service.
pub struct AppState {
    pub service: PaymentService,
}

// =============================================================================
// ORDER HANDLERS
// =============================================================================

/// Lists orders for a user.
pub async fn list_user_orders(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    match state.service.list_user_orders(*user_id, query.limit(), query.offset()).await {
        Ok(orders) => {
            let total = orders.len();
            let response = OrderListResponse {
                orders: orders.into_iter().map(OrderResponse::from).collect(),
                total,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => handle_error(e),
    }
}

/// Lists all orders (admin).
pub async fn list_orders(
    state: web::Data<AppState>,
    query: web::Query<OrderQuery>,
) -> HttpResponse {
    let status = query.status.as_ref().and_then(|s| parse_order_status(s));
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    match state.service.list_orders(status, limit, offset).await {
        Ok(orders) => {
            let total = orders.len();
            let response = OrderListResponse {
                orders: orders.into_iter().map(OrderResponse::from).collect(),
                total,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => handle_error(e),
    }
}

/// Gets an order by ID.
pub async fn get_order(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
) -> HttpResponse {
    match state.service.get_order(*order_id).await {
        Ok(order) => HttpResponse::Ok().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Gets an order by order number.
pub async fn get_order_by_number(
    state: web::Data<AppState>,
    order_number: web::Path<String>,
) -> HttpResponse {
    match state.service.get_order_by_number(&order_number).await {
        Ok(order) => HttpResponse::Ok().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Creates a new order.
pub async fn create_order(
    state: web::Data<AppState>,
    body: web::Json<CreateOrderRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    let data = NewOrder {
        user_id: body.user_id,
        course_id: body.course_id,
        subtotal_cents: body.subtotal_cents,
        tax_cents: body.tax_cents,
        discount_cents: None,
        discount_code: body.discount_code.clone(),
        currency: body.currency.clone(),
        metadata: None,
    };

    match state.service.create_order(data).await {
        Ok((order, _event)) => HttpResponse::Created().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Updates an order.
pub async fn update_order(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
    body: web::Json<UpdateOrderRequest>,
) -> HttpResponse {
    let data = UpdateOrder {
        status: body.status.as_ref().and_then(|s| parse_order_status(s)),
        payment_provider: body.payment_provider.clone(),
        payment_intent_id: body.payment_intent_id.clone(),
        metadata: body.metadata.clone(),
    };

    match state.service.update_order(*order_id, data).await {
        Ok((order, _event)) => HttpResponse::Ok().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Initiates payment for an order.
pub async fn initiate_payment(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
    body: web::Json<InitiatePaymentRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    match state.service.initiate_payment(*order_id, &body.provider, &body.payment_intent_id).await {
        Ok(order) => HttpResponse::Ok().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Completes payment for an order.
pub async fn complete_payment(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
    body: web::Json<CompletePaymentRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    let transaction_data = NewTransaction {
        order_id: *order_id,
        provider: body.provider.clone(),
        provider_transaction_id: body.provider_transaction_id.clone(),
        transaction_type: TransactionType::Payment,
        amount_cents: body.amount_cents,
        currency: body.currency.clone(),
        status: TransactionStatus::Succeeded.to_string(),
        provider_fee_cents: body.provider_fee_cents,
        metadata: body.metadata.clone(),
    };

    match state.service.complete_payment(*order_id, transaction_data).await {
        Ok((order, transaction, _event)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "order": OrderResponse::from(order),
                "transaction": TransactionResponse::from(transaction)
            }))
        }
        Err(e) => handle_error(e),
    }
}

/// Processes a refund.
pub async fn process_refund(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
    body: web::Json<RefundRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    match state.service.process_refund(*order_id, body.amount_cents, &body.provider_transaction_id, body.reason.clone()).await {
        Ok((order, transaction, _event)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "order": OrderResponse::from(order),
                "transaction": TransactionResponse::from(transaction)
            }))
        }
        Err(e) => handle_error(e),
    }
}

/// Cancels an order.
pub async fn cancel_order(
    state: web::Data<AppState>,
    order_id: web::Path<Uuid>,
    body: web::Json<CancelOrderRequest>,
) -> HttpResponse {
    match state.service.cancel_order(*order_id, body.reason.clone()).await {
        Ok((order, _event)) => HttpResponse::Ok().json(OrderResponse::from(order)),
        Err(e) => handle_error(e),
    }
}

/// Gets order statistics.
pub async fn get_order_stats(state: web::Data<AppState>) -> HttpResponse {
    match state.service.get_order_stats().await {
        Ok(stats) => HttpResponse::Ok().json(OrderStatsResponse::from(stats)),
        Err(e) => handle_error(e),
    }
}

// =============================================================================
// DISCOUNT CODE HANDLERS
// =============================================================================

/// Lists discount codes.
pub async fn list_discount_codes(
    state: web::Data<AppState>,
    query: web::Query<DiscountCodeQuery>,
) -> HttpResponse {
    let active_only = query.active_only.unwrap_or(false);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    match state.service.list_discount_codes(active_only, limit, offset).await {
        Ok(codes) => {
            let response: Vec<DiscountCodeResponse> = codes.into_iter().map(DiscountCodeResponse::from).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => handle_error(e),
    }
}

/// Gets a discount code by code string.
pub async fn get_discount_code(
    state: web::Data<AppState>,
    code: web::Path<String>,
) -> HttpResponse {
    match state.service.get_discount_code(&code).await {
        Ok(discount) => HttpResponse::Ok().json(DiscountCodeResponse::from(discount)),
        Err(e) => handle_error(e),
    }
}

/// Creates a discount code.
pub async fn create_discount_code(
    state: web::Data<AppState>,
    body: web::Json<CreateDiscountCodeRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    let discount_type = match body.discount_type.as_str() {
        "percentage" => DiscountType::Percentage,
        "fixed_amount" => DiscountType::FixedAmount,
        _ => {
            return HttpResponse::BadRequest().json(ErrorResponse::new(
                "INVALID_DISCOUNT_TYPE",
                "Discount type must be 'percentage' or 'fixed_amount'",
            ));
        }
    };

    let data = NewDiscountCode {
        code: body.code.clone(),
        description: body.description.clone(),
        discount_type,
        discount_value: body.discount_value,
        minimum_order_cents: body.minimum_order_cents,
        max_uses: body.max_uses,
        valid_from: body.valid_from,
        valid_until: body.valid_until,
        created_by: body.created_by,
    };

    match state.service.create_discount_code(data).await {
        Ok(code) => HttpResponse::Created().json(DiscountCodeResponse::from(code)),
        Err(e) => handle_error(e),
    }
}

/// Updates a discount code.
pub async fn update_discount_code(
    state: web::Data<AppState>,
    code_id: web::Path<Uuid>,
    body: web::Json<UpdateDiscountCodeRequest>,
) -> HttpResponse {
    let discount_type = body.discount_type.as_ref().map(|dt| {
        match dt.as_str() {
            "percentage" => DiscountType::Percentage,
            _ => DiscountType::FixedAmount,
        }
    });

    let data = UpdateDiscountCode {
        description: body.description.clone().map(Some),
        discount_type,
        discount_value: body.discount_value,
        minimum_order_cents: body.minimum_order_cents.map(Some),
        max_uses: body.max_uses.map(Some),
        valid_from: body.valid_from,
        valid_until: body.valid_until.map(Some),
        is_active: body.is_active,
    };

    match state.service.update_discount_code(*code_id, data).await {
        Ok(code) => HttpResponse::Ok().json(DiscountCodeResponse::from(code)),
        Err(e) => handle_error(e),
    }
}

/// Validates a discount code.
pub async fn validate_discount_code(
    state: web::Data<AppState>,
    body: web::Json<ValidateDiscountCodeRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    match state.service.validate_discount_code(&body.code, body.subtotal_cents).await {
        Ok(discount) => {
            let discount_amount = discount.calculate_discount(body.subtotal_cents);
            HttpResponse::Ok().json(DiscountValidationResponse {
                valid: true,
                code: discount.code,
                discount_type: discount.discount_type.to_string(),
                discount_value: discount.discount_value,
                discount_amount_cents: discount_amount,
                message: None,
            })
        }
        Err(e) => {
            let message = e.to_string();
            HttpResponse::Ok().json(DiscountValidationResponse {
                valid: false,
                code: body.code.clone(),
                discount_type: String::new(),
                discount_value: rust_decimal::Decimal::ZERO,
                discount_amount_cents: 0,
                message: Some(message),
            })
        }
    }
}

// =============================================================================
// REVIEW HANDLERS
// =============================================================================

/// Lists reviews for a course.
pub async fn list_course_reviews(
    state: web::Data<AppState>,
    course_id: web::Path<Uuid>,
    query: web::Query<ReviewQuery>,
) -> HttpResponse {
    let public_only = query.public_only.unwrap_or(true);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    match state.service.list_course_reviews(*course_id, public_only, limit, offset).await {
        Ok(reviews) => {
            let total = reviews.len();
            let response = ReviewListResponse {
                reviews: reviews.into_iter().map(ReviewResponse::from).collect(),
                total,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => handle_error(e),
    }
}

/// Gets a review by ID.
pub async fn get_review(
    state: web::Data<AppState>,
    review_id: web::Path<Uuid>,
) -> HttpResponse {
    match state.service.get_review(*review_id).await {
        Ok(review) => HttpResponse::Ok().json(ReviewResponse::from(review)),
        Err(e) => handle_error(e),
    }
}

/// Creates a review.
pub async fn create_review(
    state: web::Data<AppState>,
    body: web::Json<CreateReviewRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    let data = NewReview {
        course_id: body.course_id,
        user_id: body.user_id,
        enrollment_id: body.enrollment_id,
        rating: body.rating,
        review_title: body.review_title.clone(),
        review_text: body.review_text.clone(),
        is_public: body.is_public,
    };

    match state.service.create_review(data).await {
        Ok((review, _event)) => HttpResponse::Created().json(ReviewResponse::from(review)),
        Err(e) => handle_error(e),
    }
}

/// Updates a review.
pub async fn update_review(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateReviewRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "VALIDATION_ERROR",
            format!("Invalid request: {}", e),
        ));
    }

    let (review_id, user_id) = path.into_inner();

    let data = UpdateReview {
        rating: body.rating,
        review_title: body.review_title.clone().map(Some),
        review_text: body.review_text.clone().map(Some),
        is_public: body.is_public,
    };

    match state.service.update_review(review_id, user_id, data).await {
        Ok((review, _event)) => HttpResponse::Ok().json(ReviewResponse::from(review)),
        Err(e) => handle_error(e),
    }
}

/// Deletes a review.
pub async fn delete_review(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    query: web::Query<DeleteReviewQuery>,
) -> HttpResponse {
    let (review_id, user_id) = path.into_inner();
    let is_admin = query.is_admin.unwrap_or(false);

    match state.service.delete_review(review_id, user_id, is_admin).await {
        Ok(_event) => HttpResponse::NoContent().finish(),
        Err(e) => handle_error(e),
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DeleteReviewQuery {
    pub is_admin: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VoteHelpfulRequest {
    pub voter_id: Uuid,
}

/// Votes a review as helpful.
pub async fn vote_helpful(
    state: web::Data<AppState>,
    review_id: web::Path<Uuid>,
    body: web::Json<VoteHelpfulRequest>,
) -> HttpResponse {
    match state.service.vote_helpful(*review_id, body.voter_id).await {
        Ok((review, _event)) => HttpResponse::Ok().json(ReviewResponse::from(review)),
        Err(e) => handle_error(e),
    }
}

/// Gets review statistics for a course.
pub async fn get_review_stats(
    state: web::Data<AppState>,
    course_id: web::Path<Uuid>,
) -> HttpResponse {
    match state.service.get_review_stats(*course_id).await {
        Ok(stats) => HttpResponse::Ok().json(ReviewStatsResponse::from(stats)),
        Err(e) => handle_error(e),
    }
}

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "payments-service"
    }))
}

// =============================================================================
// HELPERS
// =============================================================================

/// Parses order status from string.
fn parse_order_status(s: &str) -> Option<OrderStatus> {
    match s.to_lowercase().as_str() {
        "pending" => Some(OrderStatus::Pending),
        "processing" => Some(OrderStatus::Processing),
        "paid" => Some(OrderStatus::Paid),
        "failed" => Some(OrderStatus::Failed),
        "cancelled" => Some(OrderStatus::Cancelled),
        "refunded" => Some(OrderStatus::Refunded),
        _ => None,
    }
}

/// Handles service errors and converts them to HTTP responses.
fn handle_error(error: PaymentError) -> HttpResponse {
    match error {
        PaymentError::OrderNotFound(_) => {
            HttpResponse::NotFound().json(ErrorResponse::new("ORDER_NOT_FOUND", error.to_string()))
        }
        PaymentError::TransactionNotFound(_) => {
            HttpResponse::NotFound().json(ErrorResponse::new("TRANSACTION_NOT_FOUND", error.to_string()))
        }
        PaymentError::DiscountCodeNotFound(_) => {
            HttpResponse::NotFound().json(ErrorResponse::new("DISCOUNT_CODE_NOT_FOUND", error.to_string()))
        }
        PaymentError::ReviewNotFound(_) => {
            HttpResponse::NotFound().json(ErrorResponse::new("REVIEW_NOT_FOUND", error.to_string()))
        }
        PaymentError::InvalidDiscountCode(_) | PaymentError::DiscountCodeExpired | PaymentError::DiscountCodeMaxUsesExceeded => {
            HttpResponse::BadRequest().json(ErrorResponse::new("INVALID_DISCOUNT_CODE", error.to_string()))
        }
        PaymentError::MinimumOrderNotMet { .. } => {
            HttpResponse::BadRequest().json(ErrorResponse::new("MINIMUM_ORDER_NOT_MET", error.to_string()))
        }
        PaymentError::OrderCannotBeModified(_) | PaymentError::OrderCannotBeRefunded | PaymentError::OrderCannotBeCancelled => {
            HttpResponse::Conflict().json(ErrorResponse::new("ORDER_STATE_ERROR", error.to_string()))
        }
        PaymentError::ReviewAlreadyExists => {
            HttpResponse::Conflict().json(ErrorResponse::new("REVIEW_EXISTS", error.to_string()))
        }
        PaymentError::InvalidRating => {
            HttpResponse::BadRequest().json(ErrorResponse::new("INVALID_RATING", error.to_string()))
        }
        PaymentError::Unauthorized => {
            HttpResponse::Forbidden().json(ErrorResponse::new("UNAUTHORIZED", error.to_string()))
        }
        PaymentError::Database(e) => {
            tracing::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("DATABASE_ERROR", "An internal error occurred"))
        }
    }
}
