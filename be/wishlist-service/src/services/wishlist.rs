//! # Wishlist Service
//!
//! Business logic for wishlist operations.

use redis::AsyncCommands;
use uuid::Uuid;

use crate::api::dto::{SortOrder, WishlistSortBy};
use crate::domain::entities::{PaginatedWishlist, WishlistItem, WishlistSummary};
use crate::domain::errors::{WishlistError, WishlistResult};
use crate::repository::WishlistRepository;

/// Cache TTL in seconds (1 hour).
const CACHE_TTL: u64 = 3600;

/// Cache key prefix for wishlist data.
const WISHLIST_CACHE_PREFIX: &str = "wishlist";

/// Wishlist service with business logic.
#[derive(Clone)]
pub struct WishlistService {
    repository: WishlistRepository,
    redis: redis::Client,
}

impl WishlistService {
    /// Create a new service instance.
    pub fn new(repository: WishlistRepository, redis: redis::Client) -> Self {
        Self { repository, redis }
    }

    // ========================================================================
    // Wishlist Management
    // ========================================================================

    /// Add a course to user's wishlist.
    pub async fn add_to_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<WishlistItem> {
        // Check if course exists
        if !self.repository.course_exists(course_id).await? {
            return Err(WishlistError::CourseNotFound(course_id));
        }

        // Check if user is the instructor (can't wishlist own course)
        if self.repository.is_course_instructor(user_id, course_id).await? {
            return Err(WishlistError::CannotAddOwnCourse);
        }

        // Check if already in wishlist
        if self.repository.is_in_wishlist(user_id, course_id).await?.is_some() {
            return Err(WishlistError::AlreadyInWishlist);
        }

        // Add to wishlist
        let item = self.repository.add_to_wishlist(user_id, course_id).await?;

        // Invalidate cache
        self.invalidate_user_wishlist_cache(user_id).await;

        Ok(item)
    }

    /// Remove an item from wishlist by wishlist_id.
    pub async fn remove_from_wishlist(
        &self,
        user_id: Uuid,
        wishlist_id: Uuid,
    ) -> WishlistResult<()> {
        self.repository.remove_from_wishlist(user_id, wishlist_id).await?;
        self.invalidate_user_wishlist_cache(user_id).await;
        Ok(())
    }

    /// Remove a course from wishlist by course_id.
    pub async fn remove_course_from_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<()> {
        self.repository.remove_course_from_wishlist(user_id, course_id).await?;
        self.invalidate_user_wishlist_cache(user_id).await;
        Ok(())
    }

    /// Clear entire wishlist for a user.
    pub async fn clear_wishlist(&self, user_id: Uuid) -> WishlistResult<i64> {
        let count = self.repository.clear_wishlist(user_id).await?;
        self.invalidate_user_wishlist_cache(user_id).await;
        Ok(count)
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Get user's wishlist with course details.
    pub async fn get_wishlist(
        &self,
        user_id: Uuid,
        sort_by: WishlistSortBy,
        sort_order: SortOrder,
        page: i32,
        per_page: i32,
    ) -> WishlistResult<PaginatedWishlist> {
        let cache_key = format!(
            "{}:{}:list:{}:{:?}:{:?}:{}",
            WISHLIST_CACHE_PREFIX, user_id, page, sort_by, sort_order, per_page
        );

        // Try cache first
        if let Some(cached) = self.get_from_cache::<PaginatedWishlist>(&cache_key).await {
            return Ok(cached);
        }

        let result = self.repository.get_wishlist(
            user_id,
            &sort_by,
            &sort_order,
            page,
            per_page,
        ).await?;

        // Cache result
        self.set_cache(&cache_key, &result).await;

        Ok(result)
    }

    /// Get wishlist summary for a user.
    pub async fn get_wishlist_summary(&self, user_id: Uuid) -> WishlistResult<WishlistSummary> {
        let cache_key = format!("{}:{}:summary", WISHLIST_CACHE_PREFIX, user_id);

        // Try cache first
        if let Some(cached) = self.get_from_cache::<WishlistSummary>(&cache_key).await {
            return Ok(cached);
        }

        let summary = self.repository.get_wishlist_summary(user_id).await?;

        // Cache result
        self.set_cache(&cache_key, &summary).await;

        Ok(summary)
    }

    /// Check if a course is in user's wishlist.
    pub async fn is_in_wishlist(
        &self,
        user_id: Uuid,
        course_id: Uuid,
    ) -> WishlistResult<Option<Uuid>> {
        self.repository.is_in_wishlist(user_id, course_id).await
    }

    /// Check multiple courses in wishlist.
    pub async fn check_multiple_in_wishlist(
        &self,
        user_id: Uuid,
        course_ids: Vec<Uuid>,
    ) -> WishlistResult<Vec<(Uuid, Option<Uuid>)>> {
        self.repository.check_multiple_in_wishlist(user_id, &course_ids).await
    }

    // ========================================================================
    // Cache Helpers
    // ========================================================================

    /// Get value from cache.
    async fn get_from_cache<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Redis connection error: {}", e);
                return None;
            }
        };

        let data: Option<String> = match conn.get(key).await {
            Ok(d) => d,
            Err(e) => {
                tracing::warn!("Redis get error: {}", e);
                return None;
            }
        };

        data.and_then(|s| serde_json::from_str(&s).ok())
    }

    /// Set value in cache.
    async fn set_cache<T>(&self, key: &str, value: &T)
    where
        T: serde::Serialize,
    {
        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Redis connection error: {}", e);
                return;
            }
        };

        let data = match serde_json::to_string(value) {
            Ok(d) => d,
            Err(e) => {
                tracing::warn!("JSON serialization error: {}", e);
                return;
            }
        };

        if let Err(e) = conn.set_ex::<_, _, ()>(key, data, CACHE_TTL).await {
            tracing::warn!("Redis set error: {}", e);
        }
    }

    /// Invalidate user's wishlist cache.
    async fn invalidate_user_wishlist_cache(&self, user_id: Uuid) {
        let mut conn = match self.redis.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("Redis connection error: {}", e);
                return;
            }
        };

        let pattern = format!("{}:{}:*", WISHLIST_CACHE_PREFIX, user_id);

        // Get all keys matching the pattern
        let keys: Vec<String> = match conn.keys(&pattern).await {
            Ok(k) => k,
            Err(e) => {
                tracing::warn!("Redis keys error: {}", e);
                return;
            }
        };

        // Delete all matching keys
        for key in keys {
            if let Err(e) = conn.del::<_, ()>(&key).await {
                tracing::warn!("Redis del error: {}", e);
            }
        }
    }
}
