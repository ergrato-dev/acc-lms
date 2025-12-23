import { useThemeContext } from '@/context/ThemeContext';

/**
 * Hook to access theme state and methods
 * Convenience wrapper around useThemeContext
 *
 * @example
 * ```tsx
 * const { theme, setTheme, toggleTheme } = useTheme();
 * ```
 */
export function useTheme() {
  return useThemeContext();
}

export default useTheme;
