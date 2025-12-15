//! # Authentication Middleware for Actix-web
//!
//! Extracts and validates JWT tokens from HTTP requests.
//!
//! ## Overview
//!
//! This module provides:
//!
//! | Component | Purpose | Usage |
//! |-----------|---------|-------|
//! | [`AuthMiddleware`] | Extract and validate JWT | Service-level authentication |
//! | [`AuthenticatedUser`] | Extractor for handlers | Get current user in handlers |
//! | [`RequireRole`] | Route guards | Restrict routes by role |
//! | [`UserRole`] | Role enum | Define user permissions |
//!
//! ## Authentication Flow
//!
//! ```text
//! ┌─────────┐     ┌──────────────┐     ┌─────────────┐     ┌─────────┐
//! │ Request │────►│ AuthMiddleware│────►│ Extract JWT │────►│ Validate│
//! └─────────┘     └──────────────┘     └─────────────┘     └────┬────┘
//!                                                                │
//!                 ┌──────────────┐     ┌─────────────┐          │
//!                 │   Handler    │◄────│ Insert User │◄─────────┘
//!                 │(AuthenticUser)│     │ Extension   │     (if valid)
//!                 └──────────────┘     └─────────────┘
//! ```
//!
//! ## Role Hierarchy
//!
//! Roles have a hierarchical permission model:
//!
//! ```text
//! Admin ─────────────────────────────────────►  Can do everything
//!   │
//!   └─► Instructor ──────────────────────────►  Can manage courses, view students
//!         │
//!         └─► Student ───────────────────────►  Basic access
//! ```
//!
//! ## Usage Example
//!
//! ### Setup Middleware
//!
//! ```rust,ignore
//! use shared::auth::{AuthMiddleware, JwtService};
//! use std::sync::Arc;
//!
//! let jwt_service = Arc::new(JwtService::new(config.jwt));
//! let auth_middleware = AuthMiddleware::new(jwt_service);
//!
//! // Apply to protected routes
//! App::new()
//!     .service(
//!         web::scope("/api")
//!             .wrap(auth_middleware)
//!             .route("/profile", web::get().to(get_profile))
//!     )
//! ```
//!
//! ### Use in Handlers
//!
//! ```rust,ignore
//! use shared::auth::AuthenticatedUser;
//!
//! // The user is automatically extracted from the validated token
//! async fn get_profile(user: AuthenticatedUser) -> impl Responder {
//!     format!("Hello, {}!", user.email)
//! }
//! ```
//!
//! ### Role-Based Guards
//!
//! ```rust,ignore
//! use shared::auth::RequireRole;
//!
//! // Only admins can access this route
//! #[get("/admin/users", guard = "RequireRole::admin")]
//! async fn list_users(user: AuthenticatedUser) -> impl Responder {
//!     // Only reaches here if user is admin
//! }
//! ```
//!
//! ## Related Documentation
//!
//! - [`crate::auth::jwt`] - JWT token validation
//! - [`crate::errors::ApiError`] - Authentication errors
//! - [`_docs/business/functional-requirements.md`] - RF-AUTH-003 (RBAC)

use crate::auth::jwt::{Claims, JwtService};
use crate::errors::ApiError;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use std::sync::Arc;
use uuid::Uuid;

// =============================================================================
// Authenticated User
// =============================================================================

/// Represents an authenticated user extracted from a JWT.
///
/// This struct is inserted into request extensions after successful
/// authentication and can be extracted in handlers.
///
/// ## Fields
///
/// - `user_id`: The user's unique identifier (from JWT `sub` claim)
/// - `email`: User's email address (for display/logging)
/// - `role`: User's role for authorization checks
///
/// ## Example
///
/// ```rust,ignore
/// async fn handler(user: AuthenticatedUser) -> impl Responder {
///     if user.role.has_permission(UserRole::Instructor) {
///         // User is instructor or admin
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// The user's unique identifier (UUID)
    pub user_id: Uuid,
    /// User's email address
    pub email: String,
    /// User's role for authorization
    pub role: UserRole,
}

// =============================================================================
// User Roles
// =============================================================================

/// System user roles with hierarchical permissions.
///
/// ## Permission Hierarchy
///
/// | Role | Can Access |
/// |------|------------|
/// | Admin | Everything |
/// | Instructor | Instructor + Student resources |
/// | Student | Student resources only |
///
/// ## Usage
///
/// ```rust,ignore
/// // Check if user has required permission
/// if user.role.has_permission(UserRole::Instructor) {
///     // User is instructor OR admin
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    /// Basic user role - can access their own resources
    Student,
    /// Course creator - can manage courses and view enrolled students
    Instructor,
    /// Full system access - can manage users, courses, settings
    Admin,
}

impl UserRole {
    /// Parses a role from a string (case-insensitive).
    ///
    /// ## Returns
    ///
    /// - `Some(role)` if the string matches a known role
    /// - `None` if the string is not recognized
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// assert_eq!(UserRole::from_str("student"), Some(UserRole::Student));
    /// assert_eq!(UserRole::from_str("ADMIN"), Some(UserRole::Admin));
    /// assert_eq!(UserRole::from_str("unknown"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "student" => Some(Self::Student),
            "instructor" => Some(Self::Instructor),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }

    /// Returns the string representation of the role.
    ///
    /// Used when storing roles in JWT or database.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Student => "student",
            Self::Instructor => "instructor",
            Self::Admin => "admin",
        }
    }

    /// Checks if this role has at least the required permission level.
    ///
    /// ## Permission Hierarchy
    ///
    /// - Admin can do everything
    /// - Instructor can do instructor and student tasks
    /// - Student can only do student tasks
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let admin = UserRole::Admin;
    /// assert!(admin.has_permission(UserRole::Admin));
    /// assert!(admin.has_permission(UserRole::Instructor));
    /// assert!(admin.has_permission(UserRole::Student));
    ///
    /// let student = UserRole::Student;
    /// assert!(!student.has_permission(UserRole::Admin));
    /// assert!(!student.has_permission(UserRole::Instructor));
    /// assert!(student.has_permission(UserRole::Student));
    /// ```
    pub fn has_permission(&self, required: Self) -> bool {
        match (self, required) {
            // Admin has access to everything
            (Self::Admin, _) => true,
            // Instructor has access to instructor and student resources
            (Self::Instructor, Self::Student | Self::Instructor) => true,
            // Student only has access to student resources
            (Self::Student, Self::Student) => true,
            // All other combinations are denied
            _ => false,
        }
    }
}

/// Converts JWT claims to an authenticated user.
impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            email: claims.email,
            // Default to Student if role is unknown
            role: UserRole::from_str(&claims.role).unwrap_or(UserRole::Student),
        }
    }
}

// =============================================================================
// Auth Middleware
// =============================================================================

/// Middleware for JWT-based authentication.
///
/// This middleware:
/// 1. Extracts the JWT from the `Authorization` header
/// 2. Validates the token signature and claims
/// 3. Inserts the authenticated user into request extensions
///
/// ## Setup
///
/// ```rust,ignore
/// let jwt_service = Arc::new(JwtService::new(config.jwt));
/// let auth = AuthMiddleware::new(jwt_service);
///
/// App::new()
///     .service(
///         web::scope("/api")
///             .wrap(auth)
///             .route("/protected", web::get().to(handler))
///     )
/// ```
#[derive(Clone)]
pub struct AuthMiddleware {
    /// JWT service for token validation
    jwt_service: Arc<JwtService>,
}

impl AuthMiddleware {
    /// Creates a new authentication middleware.
    ///
    /// ## Parameters
    ///
    /// - `jwt_service`: Shared JWT service for token validation
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }

    /// Extracts and validates the user from a request.
    ///
    /// This is the core authentication logic:
    /// 1. Get the `Authorization` header
    /// 2. Extract the Bearer token
    /// 3. Validate the JWT
    /// 4. Convert claims to `AuthenticatedUser`
    ///
    /// ## Errors
    ///
    /// - `ApiError::MissingAuth` - No Authorization header
    /// - `ApiError::InvalidToken` - Malformed token or invalid signature
    /// - `ApiError::TokenExpired` - Token has expired
    pub fn extract_user(&self, req: &ServiceRequest) -> Result<AuthenticatedUser, ApiError> {
        // Get Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(ApiError::MissingAuth)?;

        // Extract token from "Bearer <token>"
        let token = JwtService::extract_from_header(auth_header)?;
        
        // Validate token and get claims
        let claims = self.jwt_service.validate_access_token(token)?;

        Ok(AuthenticatedUser::from(claims))
    }

    /// Authenticates a request and stores the user in extensions.
    ///
    /// Call this from middleware to authenticate the request.
    /// The user will be available via the `AuthenticatedUser` extractor.
    ///
    /// ## Errors
    ///
    /// Returns authentication errors if the token is missing or invalid.
    pub fn authenticate(&self, req: &ServiceRequest) -> Result<(), ApiError> {
        let user = self.extract_user(req)?;
        // Store in request extensions for later extraction
        req.extensions_mut().insert(user);
        Ok(())
    }
}

// =============================================================================
// Actix-web Extractor
// =============================================================================

/// Extractor for getting the authenticated user in handlers.
///
/// This implements Actix-web's `FromRequest` trait, allowing you to
/// simply add `AuthenticatedUser` as a handler parameter.
///
/// ## Example
///
/// ```rust,ignore
/// // User is automatically extracted from request extensions
/// async fn handler(user: AuthenticatedUser) -> impl Responder {
///     format!("Hello, {}!", user.email)
/// }
/// ```
///
/// ## Errors
///
/// Returns `ApiError::MissingAuth` if no user is in request extensions.
/// This happens if authentication middleware wasn't applied.
impl actix_web::FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // Try to get the user from request extensions
        let result = req
            .extensions()
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or_else(|| ApiError::MissingAuth.into());

        std::future::ready(result)
    }
}

// =============================================================================
// Role Guards
// =============================================================================

/// Route guards for role-based access control.
///
/// Use these as guards in route definitions to restrict access
/// based on user role.
///
/// ## Example
///
/// ```rust,ignore
/// use actix_web::{get, Responder};
/// use shared::auth::{RequireRole, AuthenticatedUser};
///
/// // Only admins can access
/// #[get("/admin/dashboard", guard = "RequireRole::admin")]
/// async fn admin_dashboard(user: AuthenticatedUser) -> impl Responder {
///     "Admin dashboard"
/// }
///
/// // Instructors and admins can access
/// #[get("/courses/create", guard = "RequireRole::instructor")]
/// async fn create_course(user: AuthenticatedUser) -> impl Responder {
///     "Create course form"
/// }
/// ```
pub struct RequireRole;

impl RequireRole {
    /// Guard that requires at least Student role.
    ///
    /// Effectively means "any authenticated user" since Student is the
    /// lowest role.
    pub fn student(req: &actix_web::guard::GuardContext) -> bool {
        Self::check_role(req, UserRole::Student)
    }

    /// Guard that requires at least Instructor role.
    ///
    /// Allows Instructors and Admins.
    pub fn instructor(req: &actix_web::guard::GuardContext) -> bool {
        Self::check_role(req, UserRole::Instructor)
    }

    /// Guard that requires Admin role.
    ///
    /// Only allows Admins.
    pub fn admin(req: &actix_web::guard::GuardContext) -> bool {
        Self::check_role(req, UserRole::Admin)
    }

    /// Internal helper to check if user has required role.
    fn check_role(req: &actix_web::guard::GuardContext, required: UserRole) -> bool {
        req.req_data()
            .get::<AuthenticatedUser>()
            .map(|user| user.role.has_permission(required))
            .unwrap_or(false)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_from_str() {
        // Case insensitive parsing
        assert_eq!(UserRole::from_str("student"), Some(UserRole::Student));
        assert_eq!(UserRole::from_str("INSTRUCTOR"), Some(UserRole::Instructor));
        assert_eq!(UserRole::from_str("Admin"), Some(UserRole::Admin));
        
        // Unknown role returns None
        assert_eq!(UserRole::from_str("unknown"), None);
        assert_eq!(UserRole::from_str(""), None);
    }

    #[test]
    fn test_user_role_as_str() {
        assert_eq!(UserRole::Student.as_str(), "student");
        assert_eq!(UserRole::Instructor.as_str(), "instructor");
        assert_eq!(UserRole::Admin.as_str(), "admin");
    }

    #[test]
    fn test_admin_has_all_permissions() {
        assert!(UserRole::Admin.has_permission(UserRole::Admin));
        assert!(UserRole::Admin.has_permission(UserRole::Instructor));
        assert!(UserRole::Admin.has_permission(UserRole::Student));
    }

    #[test]
    fn test_instructor_permissions() {
        // Instructor cannot do admin tasks
        assert!(!UserRole::Instructor.has_permission(UserRole::Admin));
        // Instructor can do instructor tasks
        assert!(UserRole::Instructor.has_permission(UserRole::Instructor));
        // Instructor can do student tasks
        assert!(UserRole::Instructor.has_permission(UserRole::Student));
    }

    #[test]
    fn test_student_permissions() {
        // Student cannot do admin tasks
        assert!(!UserRole::Student.has_permission(UserRole::Admin));
        // Student cannot do instructor tasks
        assert!(!UserRole::Student.has_permission(UserRole::Instructor));
        // Student can do student tasks
        assert!(UserRole::Student.has_permission(UserRole::Student));
    }

    #[test]
    fn test_authenticated_user_from_claims() {
        let claims = Claims {
            sub: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "instructor".to_string(),
            iss: "test".to_string(),
            aud: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: Uuid::new_v4(),
            token_type: crate::auth::jwt::TokenType::Access,
        };

        let user = AuthenticatedUser::from(claims.clone());

        assert_eq!(user.user_id, claims.sub);
        assert_eq!(user.email, claims.email);
        assert_eq!(user.role, UserRole::Instructor);
    }

    #[test]
    fn test_unknown_role_defaults_to_student() {
        let claims = Claims {
            sub: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: "unknown_role".to_string(),
            iss: "test".to_string(),
            aud: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: Uuid::new_v4(),
            token_type: crate::auth::jwt::TokenType::Access,
        };

        let user = AuthenticatedUser::from(claims);

        // Unknown role defaults to Student for safety
        assert_eq!(user.role, UserRole::Student);
    }
}

