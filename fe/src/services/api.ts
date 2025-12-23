import type { ApiError, AuthTokens } from '@/types';
import { API_CONFIG, STORAGE_KEYS } from '@utils/constants';
import {
  getStorageItem,
  removeStorageItem,
  setStorageItem,
} from '@utils/storage';
import axios, {
  type AxiosError,
  type AxiosInstance,
  type InternalAxiosRequestConfig,
} from 'axios';

// Create axios instance
const api: AxiosInstance = axios.create({
  baseURL: API_CONFIG.BASE_URL,
  timeout: API_CONFIG.TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Flag to prevent multiple refresh token requests
let isRefreshing = false;
let failedQueue: Array<{
  resolve: (token: string) => void;
  reject: (error: unknown) => void;
}> = [];

/**
 * Process queued requests after token refresh
 */
function processQueue(error: unknown, token: string | null = null): void {
  failedQueue.forEach((promise) => {
    if (error) {
      promise.reject(error);
    } else {
      promise.resolve(token!);
    }
  });
  failedQueue = [];
}

/**
 * Request interceptor - Add auth token
 */
api.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const token = getStorageItem<string>(STORAGE_KEYS.ACCESS_TOKEN);
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }

    // Add correlation ID for tracing
    config.headers['X-Correlation-ID'] = crypto.randomUUID();

    return config;
  },
  (error) => Promise.reject(error)
);

/**
 * Response interceptor - Handle errors and token refresh
 */
api.interceptors.response.use(
  (response) => response,
  async (error: AxiosError<ApiError>) => {
    const originalRequest = error.config as InternalAxiosRequestConfig & {
      _retry?: boolean;
    };

    // Handle 401 Unauthorized - Try to refresh token
    if (error.response?.status === 401 && !originalRequest._retry) {
      if (isRefreshing) {
        // Queue the request while refreshing
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject });
        }).then((token) => {
          originalRequest.headers.Authorization = `Bearer ${token}`;
          return api(originalRequest);
        });
      }

      originalRequest._retry = true;
      isRefreshing = true;

      try {
        const refreshToken = getStorageItem<string>(STORAGE_KEYS.REFRESH_TOKEN);
        if (!refreshToken) {
          throw new Error('No refresh token available');
        }

        // Call refresh endpoint
        const response = await axios.post<{ tokens: AuthTokens }>(
          `${API_CONFIG.BASE_URL}/auth/refresh`,
          { refreshToken },
          { headers: { 'Content-Type': 'application/json' } }
        );

        const { tokens } = response.data;

        // Store new tokens
        setStorageItem(STORAGE_KEYS.ACCESS_TOKEN, tokens.accessToken);
        setStorageItem(STORAGE_KEYS.REFRESH_TOKEN, tokens.refreshToken);

        // Update header for retry
        originalRequest.headers.Authorization = `Bearer ${tokens.accessToken}`;

        // Process queued requests
        processQueue(null, tokens.accessToken);

        return api(originalRequest);
      } catch (refreshError) {
        // Refresh failed - clear tokens and redirect to login
        processQueue(refreshError, null);
        removeStorageItem(STORAGE_KEYS.ACCESS_TOKEN);
        removeStorageItem(STORAGE_KEYS.REFRESH_TOKEN);
        removeStorageItem(STORAGE_KEYS.USER);

        // Dispatch event for auth context to handle
        window.dispatchEvent(new CustomEvent('auth:logout'));

        return Promise.reject(refreshError);
      } finally {
        isRefreshing = false;
      }
    }

    // Transform error to standard format
    const apiError: ApiError = error.response?.data || {
      code: 'UNKNOWN',
      message: error.message || 'An unexpected error occurred',
    };

    return Promise.reject(apiError);
  }
);

export default api;

// ============================================
// API Helper Functions
// ============================================

/**
 * Set auth tokens after login
 */
export function setAuthTokens(tokens: AuthTokens): void {
  setStorageItem(STORAGE_KEYS.ACCESS_TOKEN, tokens.accessToken);
  setStorageItem(STORAGE_KEYS.REFRESH_TOKEN, tokens.refreshToken);
}

/**
 * Clear auth tokens on logout
 */
export function clearAuthTokens(): void {
  removeStorageItem(STORAGE_KEYS.ACCESS_TOKEN);
  removeStorageItem(STORAGE_KEYS.REFRESH_TOKEN);
  removeStorageItem(STORAGE_KEYS.USER);
}

/**
 * Check if user is authenticated (has tokens)
 */
export function isAuthenticated(): boolean {
  return !!getStorageItem<string>(STORAGE_KEYS.ACCESS_TOKEN);
}

/**
 * Get current access token
 */
export function getAccessToken(): string | null {
  return getStorageItem<string>(STORAGE_KEYS.ACCESS_TOKEN);
}
