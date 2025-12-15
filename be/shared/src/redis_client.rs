//! # Redis Client for Caching and Session Management
//!
//! High-level Redis client for caching, session management, and rate limiting.
//!
//! ## Why Redis?
//!
//! Redis is an in-memory data store used for:
//!
//! | Use Case | Description |
//! |----------|-------------|
//! | **Caching** | Store frequently accessed data to reduce DB load |
//! | **Sessions** | Store refresh tokens and session data |
//! | **Rate Limiting** | Track request counts per IP/user |
//! | **Token Blacklist** | Invalidate JWTs on logout |
//!
//! ## Connection Management
//!
//! We use a `ConnectionManager` which automatically reconnects on failure.
//! This is more resilient than a simple connection for long-running services.
//!
//! ```text
//! ┌───────────────────────────────────────────────────────────────────┐
//! │                     Redis Client                                   │
//! ├───────────────────────────────────────────────────────────────────┤
//! │                                                                    │
//! │  ┌──────────────────┐                                             │
//! │  │ ConnectionManager │ ──── Auto-reconnect on failure             │
//! │  └────────┬─────────┘                                             │
//! │           │                                                        │
//! │           ▼                                                        │
//! │  ┌─────────────────┐                                              │
//! │  │   Redis Server   │                                              │
//! │  └─────────────────┘                                              │
//! │                                                                    │
//! └───────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Naming Convention
//!
//! We use a hierarchical naming pattern for keys:
//!
//! | Pattern | Example | Purpose |
//! |---------|---------|---------|
//! | `blacklist:token:{jti}` | `blacklist:token:abc-123` | Invalidated JWTs |
//! | `refresh:{user_id}:{token_id}` | `refresh:user-1:token-1` | Refresh tokens |
//! | `failed_login:{identifier}` | `failed_login:user@example.com` | Brute force protection |
//! | `cache:{entity}:{id}` | `cache:user:123` | Entity caching |
//!
//! ## TTL (Time To Live)
//!
//! All session-related keys should have a TTL:
//!
//! | Key Type | Recommended TTL |
//! |----------|-----------------|
//! | Access token blacklist | Same as access token TTL (15 min) |
//! | Refresh token | Same as refresh token TTL (7 days) |
//! | Failed login counter | 15 minutes |
//! | Cache entries | Varies by use case |
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::redis_client::RedisClient;
//! use shared::config::AppConfig;
//! use std::time::Duration;
//!
//! let config = AppConfig::from_env()?;
//! let redis = RedisClient::new(&config.redis).await?;
//!
//! // Store a value with TTL
//! redis.set("key", &"value", Some(Duration::from_secs(3600))).await?;
//!
//! // Retrieve a value
//! let value: Option<String> = redis.get("key").await?;
//!
//! // Token blacklisting for logout
//! redis.blacklist_token(&jti, Duration::from_secs(900)).await?;
//! ```
//!
//! ## Related Documentation
//!
//! - [`crate::config::RedisConfig`] - Connection configuration
//! - [`crate::auth::jwt`] - JWT token management
//! - [`_docs/business/functional-requirements.md`] - RF-AUTH-004 (logout)

use crate::config::RedisConfig;
use crate::errors::ApiError;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::time::Duration;
use tracing::info;

// =============================================================================
// Redis Client
// =============================================================================

/// Async Redis client with automatic reconnection.
///
/// This client wraps a `ConnectionManager` which handles connection
/// failures automatically. It's safe to clone and share across tasks.
///
/// ## Thread Safety
///
/// The client is `Clone + Send + Sync`. Cloning is cheap (just an Arc clone).
/// Each operation uses an async connection from the manager.
#[derive(Clone)]
pub struct RedisClient {
    /// Connection manager for automatic reconnection
    conn: ConnectionManager,
}

impl RedisClient {
    /// Creates a new Redis client and establishes connection.
    ///
    /// ## Parameters
    ///
    /// - `config`: Redis configuration (URL, pool size)
    ///
    /// ## Errors
    ///
    /// Returns `ApiError::RedisError` if:
    /// - URL is invalid
    /// - Redis server is unreachable
    /// - Authentication fails
    pub async fn new(config: &RedisConfig) -> Result<Self, ApiError> {
        info!(url = %config.url, "Connecting to Redis");

        // Parse connection URL and create client
        let client = redis::Client::open(config.url.as_str())
            .map_err(ApiError::RedisError)?;

        // Create connection manager (handles reconnection automatically)
        let conn = ConnectionManager::new(client)
            .await
            .map_err(ApiError::RedisError)?;

        info!("Redis connection established");

        Ok(Self { conn })
    }

    // =========================================================================
    // Basic Operations
    // =========================================================================

    /// Stores a value with optional TTL (Time To Live).
    ///
    /// The value is serialized to JSON before storage.
    ///
    /// ## Parameters
    ///
    /// - `key`: Redis key
    /// - `value`: Any serializable value
    /// - `ttl`: Optional expiration time (None = no expiration)
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// // Store with 1 hour TTL
    /// redis.set("user:123", &user, Some(Duration::from_secs(3600))).await?;
    ///
    /// // Store without expiration
    /// redis.set("config:feature", &true, None).await?;
    /// ```
    pub async fn set<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<Duration>,
    ) -> Result<(), ApiError> {
        // Serialize value to JSON
        let serialized = serde_json::to_string(value)
            .map_err(|e| ApiError::InternalError { message: e.to_string() })?;

        let mut conn = self.conn.clone();

        match ttl {
            Some(duration) => {
                // SET with expiration (SETEX)
                conn.set_ex::<_, _, ()>(key, &serialized, duration.as_secs())
                    .await
                    .map_err(ApiError::RedisError)?;
            }
            None => {
                // SET without expiration
                conn.set::<_, _, ()>(key, &serialized)
                    .await
                    .map_err(ApiError::RedisError)?;
            }
        }

        Ok(())
    }

    /// Retrieves and deserializes a value.
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(value))` - Key exists and was deserialized
    /// - `Ok(None)` - Key doesn't exist
    /// - `Err(...)` - Redis error or deserialization failed
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let user: Option<User> = redis.get("user:123").await?;
    /// if let Some(user) = user {
    ///     // Cache hit
    /// }
    /// ```
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ApiError> {
        let mut conn = self.conn.clone();

        let value: Option<String> = conn.get(key).await.map_err(ApiError::RedisError)?;

        match value {
            Some(s) => {
                let deserialized = serde_json::from_str(&s)
                    .map_err(|e| ApiError::InternalError { message: e.to_string() })?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    /// Deletes a key.
    ///
    /// ## Returns
    ///
    /// - `true` - Key was deleted
    /// - `false` - Key didn't exist
    pub async fn delete(&self, key: &str) -> Result<bool, ApiError> {
        let mut conn = self.conn.clone();
        let deleted: i64 = conn.del(key).await.map_err(ApiError::RedisError)?;
        Ok(deleted > 0)
    }

    /// Checks if a key exists.
    pub async fn exists(&self, key: &str) -> Result<bool, ApiError> {
        let mut conn = self.conn.clone();
        conn.exists(key).await.map_err(ApiError::RedisError)
    }

    /// Increments a counter (atomic operation).
    ///
    /// Creates the key with value 1 if it doesn't exist.
    /// Useful for rate limiting and counters.
    ///
    /// ## Returns
    ///
    /// The new value after incrementing.
    pub async fn incr(&self, key: &str) -> Result<i64, ApiError> {
        let mut conn = self.conn.clone();
        conn.incr(key, 1).await.map_err(ApiError::RedisError)
    }

    /// Sets TTL on an existing key.
    ///
    /// ## Returns
    ///
    /// - `true` - TTL was set
    /// - `false` - Key doesn't exist
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, ApiError> {
        let mut conn = self.conn.clone();
        conn.expire(key, ttl.as_secs() as i64)
            .await
            .map_err(ApiError::RedisError)
    }

    /// Health check - verifies Redis is responding.
    pub async fn ping(&self) -> Result<(), ApiError> {
        let mut conn = self.conn.clone();
        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .map_err(ApiError::RedisError)?;
        Ok(())
    }

    // =========================================================================
    // Token Management (Authentication)
    // =========================================================================
    // These methods implement RF-AUTH-004: Secure logout
    // See: _docs/business/functional-requirements.md

    /// Adds a JWT to the blacklist (for logout).
    ///
    /// When a user logs out, we add their token's JTI (JWT ID) to a blacklist.
    /// Subsequent requests with that token are rejected.
    ///
    /// ## Parameters
    ///
    /// - `jti`: The JWT ID claim from the token
    /// - `ttl`: Should match the token's remaining lifetime
    ///
    /// ## Implementation Note
    ///
    /// We only need to blacklist until the token would have expired anyway.
    /// After that, the token is invalid regardless of the blacklist.
    pub async fn blacklist_token(&self, jti: &str, ttl: Duration) -> Result<(), ApiError> {
        let key = format!("blacklist:token:{}", jti);
        self.set(&key, &true, Some(ttl)).await
    }

    /// Checks if a token is blacklisted.
    ///
    /// Call this when validating JWTs to ensure they haven't been
    /// invalidated by logout.
    pub async fn is_token_blacklisted(&self, jti: &str) -> Result<bool, ApiError> {
        let key = format!("blacklist:token:{}", jti);
        self.exists(&key).await
    }

    /// Stores a refresh token reference.
    ///
    /// This allows us to track which refresh tokens are valid for a user.
    /// Used for single-device logout and session management.
    pub async fn store_refresh_token(
        &self,
        user_id: &str,
        token_id: &str,
        ttl: Duration,
    ) -> Result<(), ApiError> {
        let key = format!("refresh:{}:{}", user_id, token_id);
        self.set(&key, &true, Some(ttl)).await
    }

    /// Revokes all refresh tokens for a user (logout everywhere).
    ///
    /// This is used when a user wants to sign out of all devices.
    ///
    /// ## Note
    ///
    /// Uses KEYS command which is O(N). For production with many keys,
    /// consider using SCAN or a different data structure (e.g., a set
    /// of tokens per user).
    pub async fn revoke_all_refresh_tokens(&self, user_id: &str) -> Result<(), ApiError> {
        let mut conn = self.conn.clone();
        let pattern = format!("refresh:{}:*", user_id);
        
        // Find all matching keys
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(&pattern)
            .query_async(&mut conn)
            .await
            .map_err(ApiError::RedisError)?;

        // Delete all found keys
        if !keys.is_empty() {
            conn.del::<_, ()>(keys).await.map_err(ApiError::RedisError)?;
        }

        Ok(())
    }

    // =========================================================================
    // Brute Force Protection
    // =========================================================================
    // These methods implement RF-AUTH-004: Account lockout after failed attempts

    /// Increments failed login counter.
    ///
    /// Returns the new count. Use this to implement account lockout:
    ///
    /// ```rust,ignore
    /// let count = redis.increment_failed_login(&email).await?;
    /// if count >= 5 {
    ///     return Err(ApiError::AccountLocked { until: ... });
    /// }
    /// ```
    ///
    /// The counter automatically expires after 15 minutes.
    pub async fn increment_failed_login(&self, identifier: &str) -> Result<i64, ApiError> {
        let key = format!("failed_login:{}", identifier);
        let count = self.incr(&key).await?;
        
        // Set 15-minute expiry on first attempt
        // After 15 minutes with no attempts, the counter resets
        if count == 1 {
            self.expire(&key, Duration::from_secs(15 * 60)).await?;
        }
        
        Ok(count)
    }

    /// Gets the current failed login count.
    ///
    /// Returns 0 if no failed attempts recorded.
    pub async fn get_failed_login_count(&self, identifier: &str) -> Result<i64, ApiError> {
        let key = format!("failed_login:{}", identifier);
        let mut conn = self.conn.clone();
        let count: i64 = conn.get(&key).await.unwrap_or(0);
        Ok(count)
    }

    /// Resets failed login counter (call after successful login).
    pub async fn reset_failed_login(&self, identifier: &str) -> Result<(), ApiError> {
        let key = format!("failed_login:{}", identifier);
        self.delete(&key).await?;
        Ok(())
    }
}

// Hide internal state in Debug output
impl std::fmt::Debug for RedisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisClient").finish_non_exhaustive()
    }
}

