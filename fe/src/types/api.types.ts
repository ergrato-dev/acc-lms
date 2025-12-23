// ============================================
// API Response Types
// ============================================

/**
 * Standard API response wrapper
 */
export interface ApiResponse<T> {
  data: T;
  message?: string;
}

/**
 * Paginated response
 */
export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    totalItems: number;
    totalPages: number;
    hasNext: boolean;
    hasPrevious: boolean;
  };
}

/**
 * API Error response (matching backend format from RF requirements)
 */
export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, string[]>;
}

/**
 * API Query parameters for lists
 */
export interface QueryParams {
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
  search?: string;
}

// ============================================
// Auth Types
// ============================================

/**
 * JWT Token payload
 */
export interface JwtPayload {
  sub: string; // user ID
  email: string;
  role: 'student' | 'instructor' | 'admin';
  exp: number;
  iat: number;
}

/**
 * Auth tokens response
 */
export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}

/**
 * Login request
 */
export interface LoginRequest {
  email: string;
  password: string;
  rememberMe?: boolean;
}

/**
 * Register request
 */
export interface RegisterRequest {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
}

/**
 * Login/Register response
 */
export interface AuthResponse {
  user: User;
  tokens: AuthTokens;
}
