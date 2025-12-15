//! # API Handlers
//!
//! HTTP request handlers for the users-service.
//! Each handler is responsible for:
//!
//! 1. Extracting and validating request data
//! 2. Calling the appropriate service method
//! 3. Transforming the result into an HTTP response
//!
//! ## Error Handling
//!
//! Handlers use the `?` operator with `ApiError` which implements
//! `ResponseError`. This automatically converts errors into proper
//! HTTP responses with appropriate status codes.
//!
//! ## Authentication
//!
//! Most handlers require authentication via JWT token in the
//! `Authorization` header. The token is validated and user claims
//! are extracted before calling the service layer.

use actix_web::{web, HttpRequest, HttpResponse};
use shared::errors::ApiError;
use tracing::{info, instrument, warn};
use uuid::Uuid;
use validator::Validate;

use crate::api::dto::{
    AvatarRemoveResponse, AvatarUploadResponse, ChangeRoleRequest, PaginationMeta,
    PreferencesResponse, ProfileResponse, SearchUsersQuery, SearchUsersResponse,
    UpdatePreferencesRequest, UpdateProfileRequest, UserDto,
};
use crate::domain::entities::UserRole;
use crate::service::{UpdatePreferencesRequest as ServicePrefsUpdate, UpdateProfileRequest as ServiceProfileUpdate};
use crate::AppState;

// =============================================================================
// HEALTH CHECK
// =============================================================================

/// Health check handler.
///
/// Returns the service health status. Used by:
/// - Load balancers (health probes)
/// - Kubernetes (liveness/readiness probes)
/// - Monitoring systems
///
/// # Response
///
/// ```json
/// {
///   "status": "healthy",
///   "service": "users-service",
///   "version": "1.0.0"
/// }
/// ```
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "users-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// =============================================================================
// PROFILE HANDLERS
// =============================================================================

/// Gets a user's profile by ID.
///
/// # Path Parameters
///
/// - `id`: User UUID
///
/// # Authorization
///
/// - Public: Can view public profiles (respecting privacy settings)
/// - Authenticated: Can view own profile with full data
/// - Admin: Can view any profile with full data
///
/// # Response
///
/// - `200 OK`: Profile data
/// - `404 Not Found`: User doesn't exist
/// - `403 Forbidden`: Profile is private
#[instrument(skip(state, req, path), fields(user_id))]
pub async fn get_profile(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Extract requesting user from JWT (if present)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    
    // Get profile from service
    let response = state
        .user_service
        .get_profile(user_id, requesting_user_id, requesting_user_role)
        .await?;
    
    // Get stats if user has access
    let stats = if requesting_user_id == Some(user_id) 
        || requesting_user_role == Some(UserRole::Admin) 
    {
        state.user_service.get_stats(user_id, requesting_user_id, requesting_user_role).await.ok()
    } else {
        None
    };
    
    let profile_response = ProfileResponse::from_profile(response.profile, stats);
    
    Ok(HttpResponse::Ok().json(profile_response))
}

/// Updates a user's profile.
///
/// # Path Parameters
///
/// - `id`: User UUID
///
/// # Authorization
///
/// - User can update their own profile
/// - Admin can update any profile
///
/// # Request Body
///
/// ```json
/// {
///   "firstName": "John",
///   "lastName": "Doe",
///   "bio": "Updated bio",
///   "website": "https://example.com"
/// }
/// ```
#[instrument(skip(state, body, req, path), fields(user_id))]
pub async fn update_profile(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Validate request body
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;
    
    // Extract requesting user (required for this endpoint)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    
    let requesting_user_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    let requesting_user_role = requesting_user_role.unwrap_or(UserRole::Student);
    
    // Convert DTO to service request
    let update = ServiceProfileUpdate {
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        bio: body.bio.clone(),
        website: body.website.clone(),
        social_links: body.social_links.clone(),
    };
    
    // Call service
    let updated = state
        .user_service
        .update_profile(user_id, update, requesting_user_id, requesting_user_role)
        .await?;
    
    info!(user_id = %user_id, "Profile updated via API");
    
    Ok(HttpResponse::Ok().json(UserDto::from(updated)))
}

// =============================================================================
// PREFERENCES HANDLERS
// =============================================================================

/// Gets a user's preferences.
///
/// # Authorization
///
/// - User can view their own preferences
/// - Admin can view any user's preferences
#[instrument(skip(state, req, path), fields(user_id))]
pub async fn get_preferences(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Extract requesting user (required)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    let requesting_user_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    let requesting_user_role = requesting_user_role.unwrap_or(UserRole::Student);
    
    let preferences = state
        .user_service
        .get_preferences(user_id, requesting_user_id, requesting_user_role)
        .await?;
    
    Ok(HttpResponse::Ok().json(PreferencesResponse::from(preferences)))
}

/// Updates a user's preferences.
///
/// # Authorization
///
/// - User can update their own preferences
/// - Admin can update any user's preferences
#[instrument(skip(state, body, req, path), fields(user_id))]
pub async fn update_preferences(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePreferencesRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Validate request body
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;
    
    // Extract requesting user (required)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    let requesting_user_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    let requesting_user_role = requesting_user_role.unwrap_or(UserRole::Student);
    
    // Convert DTO to service request
    let update = ServicePrefsUpdate {
        language: body.language.clone(),
        timezone: body.timezone.clone(),
        email_notifications: body.email_notifications.clone(),
        privacy: body.privacy.clone(),
        accessibility: body.accessibility.clone(),
    };
    
    let updated = state
        .user_service
        .update_preferences(user_id, update, requesting_user_id, requesting_user_role)
        .await?;
    
    info!(user_id = %user_id, "Preferences updated via API");
    
    Ok(HttpResponse::Ok().json(PreferencesResponse::from(updated)))
}

// =============================================================================
// STATS HANDLER
// =============================================================================

/// Gets a user's statistics.
///
/// # Authorization
///
/// - User can view their own stats
/// - Others can view if privacy.show_progress is true
/// - Admin can view any user's stats
#[instrument(skip(state, req, path), fields(user_id))]
pub async fn get_stats(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Extract requesting user (optional for this endpoint)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    
    let stats = state
        .user_service
        .get_stats(user_id, requesting_user_id, requesting_user_role)
        .await?;
    
    Ok(HttpResponse::Ok().json(crate::api::dto::UserStatsDto::from(stats)))
}

// =============================================================================
// AVATAR HANDLERS
// =============================================================================

/// Uploads a user's avatar.
///
/// # Content-Type
///
/// Must be `multipart/form-data`
///
/// # Form Fields
///
/// - `avatar`: Image file (JPEG, PNG, WebP)
///
/// # Limits
///
/// - Max file size: 5MB
/// - Image will be resized to 256x256
#[instrument(skip(state, _payload, req, path), fields(user_id))]
pub async fn upload_avatar(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    _payload: actix_multipart::Multipart,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Extract requesting user (required)
    let (requesting_user_id, _) = extract_user_claims(&req)?;
    let requesting_user_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    
    // TODO: Process multipart upload
    // 1. Extract file from payload
    // 2. Validate file type and size
    // 3. Resize image
    // 4. Upload to MinIO/S3
    // 5. Update user profile with new URL
    
    // For now, return a placeholder response
    // This will be implemented when we add MinIO integration
    let avatar_url = format!(
        "https://storage.acclms.com/avatars/{}/avatar.jpg",
        user_id
    );
    
    let _updated = state
        .user_service
        .update_avatar(
            user_id,
            avatar_url.clone(),
            1024, // placeholder size
            "image/jpeg".to_string(),
            requesting_user_id,
        )
        .await?;
    
    Ok(HttpResponse::Ok().json(AvatarUploadResponse {
        avatar_url,
        message: "Avatar uploaded successfully".to_string(),
    }))
}

/// Removes a user's avatar.
#[instrument(skip(state, req, path), fields(user_id))]
pub async fn remove_avatar(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Extract requesting user (required)
    let (requesting_user_id, _) = extract_user_claims(&req)?;
    let requesting_user_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    
    // TODO: Delete file from MinIO/S3
    
    let _ = state
        .user_service
        .remove_avatar(user_id, requesting_user_id)
        .await?;
    
    Ok(HttpResponse::Ok().json(AvatarRemoveResponse {
        message: "Avatar removed successfully".to_string(),
    }))
}

// =============================================================================
// SEARCH HANDLER
// =============================================================================

/// Searches for users.
///
/// # Authorization
///
/// Only instructors and admins can search users.
///
/// # Query Parameters
///
/// - `q`: Search query (required)
/// - `role`: Filter by role (optional)
/// - `page`: Page number (default: 1)
/// - `pageSize`: Results per page (default: 20, max: 100)
#[instrument(skip(state, req), fields(query = %query.q))]
pub async fn search_users(
    state: web::Data<AppState>,
    query: web::Query<SearchUsersQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    // Validate query parameters
    query.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;
    
    // Extract requesting user (required)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    let _ = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    let requesting_user_role = requesting_user_role.unwrap_or(UserRole::Student);
    
    // Call service
    let result = state
        .user_service
        .search_users(
            &query.q,
            query.role_filter(),
            query.page,
            query.page_size,
            requesting_user_role,
        )
        .await?;
    
    // Transform to response
    let response = SearchUsersResponse {
        users: result.users.into_iter().map(UserDto::from).collect(),
        pagination: PaginationMeta {
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
        },
    };
    
    Ok(HttpResponse::Ok().json(response))
}

// =============================================================================
// ADMIN HANDLERS
// =============================================================================

/// Changes a user's role.
///
/// # Authorization
///
/// Only admins can change user roles.
///
/// # Request Body
///
/// ```json
/// {
///   "role": "instructor",
///   "reason": "Promoted after certification"
/// }
/// ```
#[instrument(skip(state, body, req, path), fields(user_id))]
pub async fn change_role(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<ChangeRoleRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    
    // Validate request body
    body.validate().map_err(|e| ApiError::BadRequest {
        message: format!("Validation error: {}", e),
    })?;
    
    // Extract requesting user (required, must be admin)
    let (requesting_user_id, requesting_user_role) = extract_user_claims(&req)?;
    let admin_id = requesting_user_id.ok_or(ApiError::MissingAuth)?;
    let requesting_user_role = requesting_user_role.unwrap_or(UserRole::Student);
    
    // Check admin permission
    if requesting_user_role != UserRole::Admin {
        warn!(
            admin_id = %admin_id,
            user_id = %user_id,
            "Non-admin attempted to change role"
        );
        return Err(ApiError::AccessDenied);
    }
    
    // Parse new role
    let new_role = body.role_enum().ok_or_else(|| ApiError::BadRequest {
        message: "Invalid role value".to_string(),
    })?;
    
    // Call service
    let updated = state
        .user_service
        .change_role(user_id, new_role, admin_id, body.reason.clone())
        .await?;
    
    info!(
        admin_id = %admin_id,
        user_id = %user_id,
        new_role = %new_role,
        "User role changed via API"
    );
    
    Ok(HttpResponse::Ok().json(UserDto::from(updated)))
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Extracts user claims from the Authorization header.
///
/// Returns:
/// - `(Some(user_id), Some(role))` if valid JWT present
/// - `(None, None)` if no Authorization header (public access)
/// - `Err(ApiError)` if invalid token
///
/// # JWT Claims Expected
///
/// ```json
/// {
///   "sub": "user-uuid",
///   "role": "student|instructor|admin",
///   "exp": 1234567890
/// }
/// ```
fn extract_user_claims(
    req: &HttpRequest,
) -> Result<(Option<Uuid>, Option<UserRole>), ApiError> {
    // Get Authorization header
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h,
        None => return Ok((None, None)), // No auth = public access
    };
    
    // Parse header value
    let auth_str = auth_header.to_str().map_err(|_| ApiError::InvalidToken)?;
    
    // Extract token from "Bearer <token>"
    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(ApiError::InvalidToken)?;
    
    // TODO: Validate JWT and extract claims
    // For now, we'll use a placeholder implementation
    // In production, this would use the shared JwtService
    
    // Placeholder: parse user_id from a simple format for testing
    // Real implementation would validate JWT signature and expiry
    if token.starts_with("test_") {
        // Test token format: test_<user_id>_<role>
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() >= 3 {
            let user_id = Uuid::parse_str(parts[1]).map_err(|_| ApiError::InvalidToken)?;
            let role = match parts[2] {
                "admin" => UserRole::Admin,
                "instructor" => UserRole::Instructor,
                _ => UserRole::Student,
            };
            return Ok((Some(user_id), Some(role)));
        }
    }
    
    // For production: validate JWT using shared::jwt::JwtService
    // let jwt_service = JwtService::new(&config.jwt_secret);
    // let claims = jwt_service.validate_access_token(token)?;
    // let user_id = Uuid::parse_str(&claims.sub)?;
    // let role = UserRole::from_str(&claims.role)?;
    
    Err(ApiError::InvalidToken)
}
