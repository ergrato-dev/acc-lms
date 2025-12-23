import { useAuthContext } from '@/context/AuthContext';

/**
 * Hook to access authentication state and methods
 * Convenience wrapper around useAuthContext
 *
 * @example
 * ```tsx
 * const { user, isAuthenticated, login, logout } = useAuth();
 * ```
 */
export function useAuth() {
  return useAuthContext();
}

export default useAuth;
