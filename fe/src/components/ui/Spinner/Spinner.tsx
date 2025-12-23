import { cn } from '@/utils';
import { type HTMLAttributes } from 'react';

export type SpinnerSize = 'sm' | 'md' | 'lg' | 'xl';

export interface SpinnerProps extends HTMLAttributes<HTMLDivElement> {
  size?: SpinnerSize;
  color?: string;
}

const sizeStyles: Record<SpinnerSize, string> = {
  sm: 'h-4 w-4 border-2',
  md: 'h-6 w-6 border-2',
  lg: 'h-8 w-8 border-3',
  xl: 'h-12 w-12 border-4',
};

/**
 * Loading spinner component
 *
 * @example
 * ```tsx
 * <Spinner size="md" />
 * <Spinner size="lg" className="text-primary" />
 * ```
 */
function Spinner({ size = 'md', color, className, ...props }: SpinnerProps) {
  return (
    <div
      role="status"
      aria-label="Loading"
      className={cn(
        'animate-spin rounded-full',
        'border-current border-t-transparent',
        sizeStyles[size],
        className
      )}
      style={
        color
          ? { borderColor: color, borderTopColor: 'transparent' }
          : undefined
      }
      {...props}
    >
      <span className="sr-only">Loading...</span>
    </div>
  );
}

export default Spinner;
