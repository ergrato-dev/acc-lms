import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
  User,
} from '@/types';
import api, { clearAuthTokens, setAuthTokens } from './api';

const AUTH_ENDPOINTS = {
  LOGIN: '/auth/login',
  REGISTER: '/auth/register',
  LOGOUT: '/auth/logout',
  REFRESH: '/auth/refresh',
  ME: '/auth/me',
  FORGOT_PASSWORD: '/auth/forgot-password',
  RESET_PASSWORD: '/auth/reset-password',
  VERIFY_EMAIL: '/auth/verify-email',
  RESEND_VERIFICATION: '/auth/resend-verification',
} as const;

/**
 * Login user
 */
export async function login(data: LoginRequest): Promise<AuthResponse> {
  const response = await api.post<AuthResponse>(AUTH_ENDPOINTS.LOGIN, data);
  setAuthTokens(response.data.tokens);
  return response.data;
}

/**
 * Register new user
 */
export async function register(data: RegisterRequest): Promise<AuthResponse> {
  const response = await api.post<AuthResponse>(AUTH_ENDPOINTS.REGISTER, data);
  setAuthTokens(response.data.tokens);
  return response.data;
}

/**
 * Logout user
 */
export async function logout(): Promise<void> {
  try {
    await api.post(AUTH_ENDPOINTS.LOGOUT);
  } finally {
    clearAuthTokens();
  }
}

/**
 * Get current user
 */
export async function getCurrentUser(): Promise<User> {
  const response = await api.get<{ data: User }>(AUTH_ENDPOINTS.ME);
  return response.data.data;
}

/**
 * Request password reset
 */
export async function forgotPassword(email: string): Promise<void> {
  await api.post(AUTH_ENDPOINTS.FORGOT_PASSWORD, { email });
}

/**
 * Reset password with token
 */
export async function resetPassword(
  token: string,
  password: string
): Promise<void> {
  await api.post(AUTH_ENDPOINTS.RESET_PASSWORD, { token, password });
}

/**
 * Verify email with token
 */
export async function verifyEmail(token: string): Promise<void> {
  await api.post(AUTH_ENDPOINTS.VERIFY_EMAIL, { token });
}

/**
 * Resend email verification
 */
export async function resendVerification(email: string): Promise<void> {
  await api.post(AUTH_ENDPOINTS.RESEND_VERIFICATION, { email });
}

export const authService = {
  login,
  register,
  logout,
  getCurrentUser,
  forgotPassword,
  resetPassword,
  verifyEmail,
  resendVerification,
};

export default authService;
