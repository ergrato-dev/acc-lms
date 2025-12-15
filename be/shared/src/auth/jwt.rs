//! # JWT Token Service
//!
//! Handles generation and validation of JSON Web Tokens (JWT) for authentication.
//!
//! ## What is JWT?
//!
//! JWT (JSON Web Token) is an open standard ([RFC 7519](https://tools.ietf.org/html/rfc7519))
//! for securely transmitting information between parties as a JSON object. This information
//! can be verified because it is digitally signed.
//!
//! ## Token Structure
//!
//! A JWT consists of three parts separated by dots:
//!
//! ```text
//! xxxxx.yyyyy.zzzzz
//! ├────┼─────┼────┤
//! │    │     │    └── Signature (verifies integrity)
//! │    │     └─────── Payload (claims - the actual data)
//! │    └───────────── Header (algorithm and token type)
//! ```
//!
//! ## Our Token Implementation
//!
//! We use two types of tokens:
//!
//! | Token Type | TTL | Purpose | Storage |
//! |------------|-----|---------|---------|
//! | Access Token | 15 min | API authorization | Memory only |
//! | Refresh Token | 7 days | Get new access token | HttpOnly cookie |
//!
//! ### Why Two Tokens?
//!
//! - **Access tokens** are short-lived to minimize damage if leaked
//! - **Refresh tokens** allow session continuity without re-login
//! - Refresh tokens are stored in HttpOnly cookies (XSS protection)
//!
//! ## Claims (Token Payload)
//!
//! Our tokens contain these claims:
//!
//! | Claim | Description | Example |
//! |-------|-------------|---------|
//! | `sub` | Subject (user ID) | `550e8400-e29b-...` |
//! | `email` | User's email | `user@example.com` |
//! | `role` | User's role | `student` |
//! | `iss` | Issuer | `acc-lms` |
//! | `aud` | Audience | `acc-lms-api` |
//! | `exp` | Expiration time | Unix timestamp |
//! | `iat` | Issued at | Unix timestamp |
//! | `jti` | JWT ID (unique) | UUID |
//! | `type` | Token type | `access` or `refresh` |
//!
//! ## Security Notes
//!
//! - We use **HS256** (HMAC-SHA256) for signing
//! - Secret key must be at least 32 characters
//! - Tokens are validated for: signature, expiration, issuer, audience
//! - The `jti` claim enables token blacklisting (for logout)
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::auth::jwt::{JwtService, TokenType};
//! use shared::config::JwtConfig;
//!
//! // Create service with configuration
//! let config = JwtConfig { /* ... */ };
//! let jwt_service = JwtService::new(config);
//!
//! // Generate tokens for a user
//! let tokens = jwt_service.generate_tokens(
//!     user_id,
//!     "user@example.com",
//!     "student"
//! )?;
//!
//! // Validate an access token
//! let claims = jwt_service.validate_access_token(&tokens.access_token)?;
//!
//! // Extract token from Authorization header
//! let token = JwtService::extract_from_header("Bearer eyJhbGc...")?;
//! ```
//!
//! ## Related Documentation
//!
//! - [`crate::config::JwtConfig`] - Configuration options
//! - [`crate::auth::middleware`] - Request authentication
//! - [`_docs/business/functional-requirements.md`] - RF-AUTH-001

use crate::config::JwtConfig;
use crate::errors::ApiError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =============================================================================
// Claims Structure
// =============================================================================

/// JWT Claims (token payload).
///
/// These are the data fields embedded in the token. All fields are verified
/// during token validation.
///
/// ## Standard Claims (RFC 7519)
///
/// - `sub`: Subject - identifies the principal (our user ID)
/// - `iss`: Issuer - who created the token
/// - `aud`: Audience - who the token is intended for
/// - `exp`: Expiration - when the token becomes invalid
/// - `iat`: Issued At - when the token was created
///
/// ## Custom Claims
///
/// - `email`: User's email for display/logging
/// - `role`: User's role for authorization
/// - `jti`: Unique token ID for blacklisting
/// - `token_type`: Differentiates access from refresh tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - the user's unique identifier (UUID).
    /// This is the primary way to identify which user owns this token.
    pub sub: Uuid,
    
    /// User's email address.
    /// Included for convenience in logging and display.
    pub email: String,
    
    /// User's role for authorization decisions.
    /// One of: "student", "instructor", "admin"
    pub role: String,
    
    /// Issuer - identifies who created the token.
    /// Should match the configured issuer in validation.
    pub iss: String,
    
    /// Audience - identifies who the token is intended for.
    /// Should match the configured audience in validation.
    pub aud: String,
    
    /// Expiration time as Unix timestamp (seconds since epoch).
    /// Token is invalid after this time.
    pub exp: i64,
    
    /// Issued at time as Unix timestamp.
    /// When the token was created.
    pub iat: i64,
    
    /// JWT ID - unique identifier for this specific token.
    /// Used for token blacklisting (e.g., on logout).
    pub jti: Uuid,
    
    /// Token type to distinguish access from refresh tokens.
    /// Prevents using a refresh token as an access token.
    #[serde(rename = "type")]
    pub token_type: TokenType,
}

// =============================================================================
// Token Type
// =============================================================================

/// Distinguishes between access and refresh tokens.
///
/// This is stored in the token itself to prevent misuse:
/// - Access tokens cannot be used to refresh
/// - Refresh tokens cannot be used for API access
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    /// Short-lived token for API authorization (default 15 min).
    Access,
    /// Long-lived token for obtaining new access tokens (default 7 days).
    Refresh,
}

// =============================================================================
// Token Pair
// =============================================================================

/// A pair of access and refresh tokens returned after login.
///
/// ## Client Handling
///
/// - **Access token**: Store in memory, send in `Authorization` header
/// - **Refresh token**: Store in HttpOnly cookie (browser) or secure storage (mobile)
///
/// ## Token Rotation
///
/// When refreshing, the client receives a new token pair. The old refresh
/// token becomes invalid (rotation for security).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    /// The access token for API authorization.
    /// Include in requests as: `Authorization: Bearer <token>`
    pub access_token: String,
    
    /// The refresh token for obtaining new access tokens.
    /// Send to `/auth/refresh` when access token expires.
    pub refresh_token: String,
    
    /// Seconds until the access token expires.
    /// Client should refresh before this time.
    pub expires_in: i64,
    
    /// Seconds until the refresh token expires.
    /// User must re-login after this time.
    pub refresh_expires_in: i64,
}

// =============================================================================
// JWT Service
// =============================================================================

/// Service for JWT generation and validation.
///
/// This is the main entry point for all JWT operations. It should be
/// created once at startup and shared across the application.
///
/// ## Thread Safety
///
/// This type is `Clone` and can be wrapped in `Arc` for sharing across
/// async tasks. The underlying keys are immutable after construction.
///
/// ## Example
///
/// ```rust,ignore
/// let config = AppConfig::from_env()?.jwt;
/// let jwt_service = Arc::new(JwtService::new(config));
///
/// // Share across handlers
/// let tokens = jwt_service.generate_tokens(user_id, email, role)?;
/// ```
#[derive(Clone)]
pub struct JwtService {
    /// Key for signing tokens (kept secret)
    encoding_key: EncodingKey,
    /// Key for verifying signatures
    decoding_key: DecodingKey,
    /// Configuration with TTLs, issuer, audience
    config: JwtConfig,
}

impl JwtService {
    /// Creates a new JWT service with the given configuration.
    ///
    /// ## Parameters
    ///
    /// - `config`: JWT configuration including secret key and TTLs
    ///
    /// ## Panics
    ///
    /// Does not panic. Invalid configuration will cause validation errors
    /// at runtime rather than construction time.
    pub fn new(config: JwtConfig) -> Self {
        // Create keys from the secret
        // Using from_secret for HS256 (symmetric algorithm)
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());

        Self {
            encoding_key,
            decoding_key,
            config,
        }
    }

    /// Generates a token pair (access + refresh) for a user.
    ///
    /// This is called after successful authentication (login).
    ///
    /// ## Parameters
    ///
    /// - `user_id`: The user's unique identifier
    /// - `email`: User's email (included in token for convenience)
    /// - `role`: User's role for authorization
    ///
    /// ## Returns
    ///
    /// A `TokenPair` containing both tokens and their expiration times.
    ///
    /// ## Errors
    ///
    /// Returns `ApiError::InternalError` if token encoding fails
    /// (which shouldn't happen with valid configuration).
    pub fn generate_tokens(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
    ) -> Result<TokenPair, ApiError> {
        let access_token = self.generate_token(user_id, email, role, TokenType::Access)?;
        let refresh_token = self.generate_token(user_id, email, role, TokenType::Refresh)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: self.config.access_token_ttl_seconds as i64,
            refresh_expires_in: self.config.refresh_token_ttl_seconds as i64,
        })
    }

    /// Generates a single token of the specified type.
    ///
    /// Internal method used by `generate_tokens`.
    fn generate_token(
        &self,
        user_id: Uuid,
        email: &str,
        role: &str,
        token_type: TokenType,
    ) -> Result<String, ApiError> {
        let now = Utc::now();
        
        // Select TTL based on token type
        let ttl = match token_type {
            TokenType::Access => Duration::seconds(self.config.access_token_ttl_seconds as i64),
            TokenType::Refresh => Duration::seconds(self.config.refresh_token_ttl_seconds as i64),
        };

        // Build the claims
        let claims = Claims {
            sub: user_id,
            email: email.to_string(),
            role: role.to_string(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            exp: (now + ttl).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4(), // Unique ID for this token
            token_type,
        };

        // Encode with HS256 (default header)
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ApiError::InternalError {
                message: format!("Failed to generate token: {}", e),
            })
    }

    /// Validates and decodes an access token.
    ///
    /// Use this for authenticating API requests.
    ///
    /// ## Validation Steps
    ///
    /// 1. Verify signature (proves token wasn't tampered with)
    /// 2. Check expiration (token is still valid)
    /// 3. Verify issuer (token was created by us)
    /// 4. Verify audience (token is for our API)
    /// 5. Check token type (must be "access")
    ///
    /// ## Errors
    ///
    /// - `ApiError::TokenExpired` - Token has expired
    /// - `ApiError::InvalidToken` - Invalid signature, issuer, audience, or type
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, ApiError> {
        let claims = self.decode_token(token)?;

        // Ensure this is an access token, not a refresh token
        if claims.token_type != TokenType::Access {
            return Err(ApiError::InvalidToken);
        }

        Ok(claims)
    }

    /// Validates and decodes a refresh token.
    ///
    /// Use this when refreshing an expired access token.
    ///
    /// ## Errors
    ///
    /// - `ApiError::TokenExpired` - Refresh token has expired
    /// - `ApiError::InvalidToken` - Invalid signature, issuer, audience, or type
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, ApiError> {
        let claims = self.decode_token(token)?;

        // Ensure this is a refresh token, not an access token
        if claims.token_type != TokenType::Refresh {
            return Err(ApiError::InvalidToken);
        }

        Ok(claims)
    }

    /// Decodes a token without checking the type.
    ///
    /// Internal method that handles common validation.
    fn decode_token(&self, token: &str) -> Result<Claims, ApiError> {
        // Configure validation rules
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);

        // Decode and validate
        let token_data: TokenData<Claims> = decode(token, &self.decoding_key, &validation)
            .map_err(|e| match e.kind() {
                // Map specific JWT errors to our error types
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::TokenExpired,
                _ => ApiError::InvalidToken,
            })?;

        Ok(token_data.claims)
    }

    /// Extracts the token from an Authorization header.
    ///
    /// Expects the format: `Bearer <token>`
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let header = "Bearer eyJhbGciOiJIUzI1NiIs...";
    /// let token = JwtService::extract_from_header(header)?;
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns `ApiError::InvalidToken` if the header doesn't start with "Bearer "
    pub fn extract_from_header(auth_header: &str) -> Result<&str, ApiError> {
        auth_header
            .strip_prefix("Bearer ")
            .ok_or(ApiError::InvalidToken)
    }
}

// Implement Debug manually to avoid exposing keys
impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("config", &self.config)
            .finish_non_exhaustive() // Indicates hidden fields
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a test configuration with valid parameters.
    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "test_secret_key_minimum_32_chars_required".to_string(),
            access_token_ttl_seconds: 900,
            refresh_token_ttl_seconds: 604800,
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
        }
    }

    #[test]
    fn test_generate_and_validate_access_token() {
        let service = JwtService::new(test_config());
        let user_id = Uuid::new_v4();

        let tokens = service
            .generate_tokens(user_id, "test@example.com", "student")
            .unwrap();

        let claims = service.validate_access_token(&tokens.access_token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "student");
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let service = JwtService::new(test_config());
        let user_id = Uuid::new_v4();

        let tokens = service
            .generate_tokens(user_id, "test@example.com", "instructor")
            .unwrap();

        let claims = service.validate_refresh_token(&tokens.refresh_token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_access_token_fails_as_refresh() {
        let service = JwtService::new(test_config());
        let user_id = Uuid::new_v4();

        let tokens = service
            .generate_tokens(user_id, "test@example.com", "student")
            .unwrap();

        // Access token should fail when validated as refresh
        let result = service.validate_refresh_token(&tokens.access_token);
        assert!(matches!(result, Err(ApiError::InvalidToken)));
    }

    #[test]
    fn test_refresh_token_fails_as_access() {
        let service = JwtService::new(test_config());
        let user_id = Uuid::new_v4();

        let tokens = service
            .generate_tokens(user_id, "test@example.com", "student")
            .unwrap();

        // Refresh token should fail when validated as access
        let result = service.validate_access_token(&tokens.refresh_token);
        assert!(matches!(result, Err(ApiError::InvalidToken)));
    }

    #[test]
    fn test_extract_from_header_valid() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let header = format!("Bearer {}", token);

        let extracted = JwtService::extract_from_header(&header).unwrap();
        assert_eq!(extracted, token);
    }

    #[test]
    fn test_extract_from_header_invalid() {
        // Missing "Bearer " prefix
        let result = JwtService::extract_from_header("InvalidHeader token");
        assert!(matches!(result, Err(ApiError::InvalidToken)));
    }

    #[test]
    fn test_token_contains_jti() {
        let service = JwtService::new(test_config());
        let user_id = Uuid::new_v4();

        let tokens1 = service
            .generate_tokens(user_id, "test@example.com", "student")
            .unwrap();
        let tokens2 = service
            .generate_tokens(user_id, "test@example.com", "student")
            .unwrap();

        let claims1 = service.validate_access_token(&tokens1.access_token).unwrap();
        let claims2 = service.validate_access_token(&tokens2.access_token).unwrap();

        // Each token should have a unique JTI
        assert_ne!(claims1.jti, claims2.jti);
    }
}

