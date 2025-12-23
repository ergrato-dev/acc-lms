import { useCallback, useEffect, useRef, useState } from 'react';

interface UseAsyncState<T> {
  data: T | null;
  error: Error | null;
  isLoading: boolean;
}

interface UseAsyncReturn<T, P extends unknown[]> extends UseAsyncState<T> {
  execute: (...args: P) => Promise<T | null>;
  reset: () => void;
}

/**
 * Hook for handling async operations with loading and error states
 *
 * @example
 * ```tsx
 * const { data, isLoading, error, execute } = useAsync(fetchUser);
 *
 * useEffect(() => {
 *   execute(userId);
 * }, [execute, userId]);
 * ```
 */
export function useAsync<T, P extends unknown[] = []>(
  asyncFunction: (...args: P) => Promise<T>,
  immediate = false
): UseAsyncReturn<T, P> {
  const [state, setState] = useState<UseAsyncState<T>>({
    data: null,
    error: null,
    isLoading: immediate,
  });

  // Track if component is mounted
  const mountedRef = useRef(true);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
    };
  }, []);

  const execute = useCallback(
    async (...args: P): Promise<T | null> => {
      setState((prev) => ({ ...prev, isLoading: true, error: null }));

      try {
        const result = await asyncFunction(...args);
        if (mountedRef.current) {
          setState({ data: result, error: null, isLoading: false });
        }
        return result;
      } catch (error) {
        if (mountedRef.current) {
          setState({
            data: null,
            error: error instanceof Error ? error : new Error(String(error)),
            isLoading: false,
          });
        }
        return null;
      }
    },
    [asyncFunction]
  );

  const reset = useCallback(() => {
    setState({ data: null, error: null, isLoading: false });
  }, []);

  return {
    ...state,
    execute,
    reset,
  };
}

export default useAsync;
