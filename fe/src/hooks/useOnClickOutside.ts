import { useEffect, useRef } from 'react';

/**
 * Hook to handle clicks outside of a component
 * Useful for dropdowns, modals, and menus
 *
 * @example
 * ```tsx
 * const ref = useOnClickOutside<HTMLDivElement>(() => {
 *   setIsOpen(false);
 * });
 *
 * return <div ref={ref}>...</div>;
 * ```
 */
export function useOnClickOutside<T extends HTMLElement = HTMLElement>(
  handler: () => void,
  enabled = true
): React.RefObject<T | null> {
  const ref = useRef<T>(null);

  useEffect(() => {
    if (!enabled) return;

    const listener = (event: MouseEvent | TouchEvent) => {
      const element = ref.current;
      if (!element || element.contains(event.target as Node)) {
        return;
      }
      handler();
    };

    document.addEventListener('mousedown', listener);
    document.addEventListener('touchstart', listener);

    return () => {
      document.removeEventListener('mousedown', listener);
      document.removeEventListener('touchstart', listener);
    };
  }, [handler, enabled]);

  return ref;
}

export default useOnClickOutside;
