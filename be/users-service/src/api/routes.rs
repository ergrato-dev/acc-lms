//! # API Routes Configuration
//!
//! Defines the HTTP routes for the users-service.
//! Routes are organized by resource and follow RESTful conventions.
//!
//! ## Route Structure
//!
//! ```text
//! /api/v1/users
//! ├── GET    /search              - Search users (instructor/admin)
//! ├── GET    /:id                 - Get user profile
//! ├── PATCH  /:id                 - Update user profile
//! ├── GET    /:id/preferences     - Get user preferences
//! ├── PATCH  /:id/preferences     - Update user preferences
//! ├── GET    /:id/stats           - Get user statistics
//! ├── POST   /:id/avatar          - Upload avatar
//! ├── DELETE /:id/avatar          - Remove avatar
//! └── PATCH  /:id/role            - Change user role (admin only)
//!
//! /health
//! └── GET    /                    - Health check
//! ```
//!
//! ## Versioning
//!
//! The API uses URL-based versioning (/api/v1/).
//! This allows breaking changes in future versions while maintaining
//! backward compatibility.

use actix_web::web;

use super::handlers;

/// Configures all routes for the users-service.
///
/// This function is called during application startup to register
/// all HTTP routes with the Actix-web router.
///
/// # Arguments
///
/// * `cfg` - Actix-web service configuration
///
/// # Example
///
/// ```rust
/// App::new()
///     .configure(configure_routes)
/// ```
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check endpoint (no version prefix)
        .route("/health", web::get().to(handlers::health_check))
        
        // API v1 routes
        .service(
            web::scope("/api/v1/users")
                // Search users (must come before /:id to avoid conflict)
                .route("/search", web::get().to(handlers::search_users))
                
                // User profile routes
                .route("/{id}", web::get().to(handlers::get_profile))
                .route("/{id}", web::patch().to(handlers::update_profile))
                
                // Preferences routes
                .route("/{id}/preferences", web::get().to(handlers::get_preferences))
                .route("/{id}/preferences", web::patch().to(handlers::update_preferences))
                
                // Stats routes
                .route("/{id}/stats", web::get().to(handlers::get_stats))
                
                // Avatar routes
                .route("/{id}/avatar", web::post().to(handlers::upload_avatar))
                .route("/{id}/avatar", web::delete().to(handlers::remove_avatar))
                
                // Admin routes
                .route("/{id}/role", web::patch().to(handlers::change_role))
        );
}

// =============================================================================
// ROUTE DOCUMENTATION
// =============================================================================

/// # Routes Reference
///
/// ## Health Check
///
/// ```
/// GET /health
/// ```
///
/// Returns service health status. Used by load balancers and monitoring.
///
/// Response: 200 OK
/// ```json
/// { "status": "healthy", "service": "users-service" }
/// ```
///
/// ---
///
/// ## Get User Profile
///
/// ```
/// GET /api/v1/users/:id
/// ```
///
/// Returns user profile information. Response varies based on authorization:
/// - Own profile: Full data
/// - Other user (public): Respects privacy settings
/// - Admin: Full data
///
/// Path Parameters:
/// - `id` (UUID): User ID
///
/// Headers:
/// - `Authorization: Bearer <token>` (optional for public profiles)
///
/// Response: 200 OK
/// ```json
/// {
///   "user": {
///     "id": "...",
///     "email": "user@example.com",
///     "firstName": "John",
///     "lastName": "Doe",
///     "role": "student",
///     "avatarUrl": "https://...",
///     "bio": "...",
///     "createdAt": "2024-01-15T10:30:00Z"
///   },
///   "stats": { ... }
/// }
/// ```
///
/// Errors:
/// - 404: User not found
/// - 403: Profile is private
///
/// ---
///
/// ## Update User Profile
///
/// ```
/// PATCH /api/v1/users/:id
/// ```
///
/// Updates user profile fields. Only the authenticated user or an admin
/// can update a profile.
///
/// Path Parameters:
/// - `id` (UUID): User ID
///
/// Headers:
/// - `Authorization: Bearer <token>` (required)
///
/// Request Body:
/// ```json
/// {
///   "firstName": "John",
///   "lastName": "Doe",
///   "bio": "Updated bio",
///   "website": "https://example.com",
///   "socialLinks": { "twitter": "..." }
/// }
/// ```
///
/// All fields are optional.
///
/// Response: 200 OK (updated profile)
///
/// Errors:
/// - 400: Validation error
/// - 401: Not authenticated
/// - 403: Not authorized to update this profile
/// - 404: User not found
///
/// ---
///
/// ## Get User Preferences
///
/// ```
/// GET /api/v1/users/:id/preferences
/// ```
///
/// Returns user preferences. Only the user themselves or an admin
/// can view preferences.
///
/// Response: 200 OK
/// ```json
/// {
///   "language": "es",
///   "timezone": "America/Mexico_City",
///   "emailNotifications": { ... },
///   "privacy": { ... },
///   "accessibility": { ... }
/// }
/// ```
///
/// ---
///
/// ## Update User Preferences
///
/// ```
/// PATCH /api/v1/users/:id/preferences
/// ```
///
/// Updates user preferences. Only the user themselves or an admin
/// can update preferences.
///
/// Request Body:
/// ```json
/// {
///   "language": "en",
///   "timezone": "America/New_York",
///   "emailNotifications": { "marketing": false }
/// }
/// ```
///
/// All fields are optional.
///
/// ---
///
/// ## Get User Statistics
///
/// ```
/// GET /api/v1/users/:id/stats
/// ```
///
/// Returns user learning statistics. Visibility depends on privacy settings.
///
/// Response: 200 OK
/// ```json
/// {
///   "coursesEnrolled": 5,
///   "coursesCompleted": 2,
///   "certificatesEarned": 1,
///   "totalLearningTime": "12h 30m",
///   "currentStreakDays": 5
/// }
/// ```
///
/// ---
///
/// ## Upload Avatar
///
/// ```
/// POST /api/v1/users/:id/avatar
/// Content-Type: multipart/form-data
/// ```
///
/// Uploads a new avatar image. Accepts JPEG, PNG, WebP formats.
/// Maximum file size: 5MB.
/// Image will be resized to 256x256.
///
/// Form Data:
/// - `avatar` (file): Image file
///
/// Response: 200 OK
/// ```json
/// {
///   "avatarUrl": "https://storage.example.com/avatars/...",
///   "message": "Avatar uploaded successfully"
/// }
/// ```
///
/// Errors:
/// - 400: Invalid file format or size
/// - 401: Not authenticated
/// - 403: Cannot update another user's avatar
///
/// ---
///
/// ## Remove Avatar
///
/// ```
/// DELETE /api/v1/users/:id/avatar
/// ```
///
/// Removes the user's avatar.
///
/// Response: 200 OK
/// ```json
/// {
///   "message": "Avatar removed successfully"
/// }
/// ```
///
/// ---
///
/// ## Search Users
///
/// ```
/// GET /api/v1/users/search?q=<query>&role=<role>&page=<page>&pageSize=<size>
/// ```
///
/// Searches users by name or email. Only available to instructors and admins.
///
/// Query Parameters:
/// - `q` (string, required): Search query
/// - `role` (string, optional): Filter by role (student, instructor, admin)
/// - `page` (number, optional): Page number (default: 1)
/// - `pageSize` (number, optional): Results per page (default: 20, max: 100)
///
/// Response: 200 OK
/// ```json
/// {
///   "users": [...],
///   "pagination": {
///     "total": 42,
///     "page": 1,
///     "pageSize": 20,
///     "totalPages": 3
///   }
/// }
/// ```
///
/// ---
///
/// ## Change User Role (Admin Only)
///
/// ```
/// PATCH /api/v1/users/:id/role
/// ```
///
/// Changes a user's role. Only admins can perform this action.
///
/// Request Body:
/// ```json
/// {
///   "role": "instructor",
///   "reason": "Promoted after completing certification"
/// }
/// ```
///
/// Response: 200 OK (updated profile)
///
/// Errors:
/// - 400: Invalid role or trying to change own role
/// - 401: Not authenticated
/// - 403: Not an admin
/// - 404: User not found
#[allow(dead_code)]
fn route_documentation() {}
