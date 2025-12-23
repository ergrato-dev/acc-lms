import { cn } from '@/utils';
import { type HTMLAttributes } from 'react';

export interface SkeletonProps extends HTMLAttributes<HTMLDivElement> {
  variant?: 'text' | 'circular' | 'rectangular';
  width?: string | number;
  height?: string | number;
  lines?: number;
}

/**
 * Skeleton component for loading states
 *
 * @example
 * ```tsx
 * <Skeleton variant="text" />
 * <Skeleton variant="circular" width={40} height={40} />
 * <Skeleton variant="rectangular" height={200} />
 * <Skeleton lines={3} /> // Multiple text lines
 * ```
 */
function Skeleton({
  variant = 'text',
  width,
  height,
  lines,
  className,
  style,
  ...props
}: SkeletonProps) {
  const baseStyles = cn(
    'animate-pulse bg-muted',
    variant === 'circular' && 'rounded-full',
    variant === 'rectangular' && 'rounded-md',
    variant === 'text' && 'rounded h-4',
    className
  );

  const computedStyle = {
    width: typeof width === 'number' ? `${width}px` : width,
    height: typeof height === 'number' ? `${height}px` : height,
    ...style,
  };

  // Render multiple lines for text skeleton
  if (lines && lines > 1) {
    return (
      <div className="flex flex-col gap-2">
        {Array.from({ length: lines }).map((_, index) => (
          <div
            key={index}
            className={cn(baseStyles, index === lines - 1 && 'w-4/5')}
            style={computedStyle}
            {...props}
          />
        ))}
      </div>
    );
  }

  return <div className={baseStyles} style={computedStyle} {...props} />;
}

/**
 * Pre-built skeleton for card layouts
 */
export function SkeletonCard({ className }: { className?: string }) {
  return (
    <div className={cn('bg-card rounded-lg border p-4', className)}>
      <Skeleton variant="rectangular" height={160} className="mb-4" />
      <Skeleton variant="text" className="mb-2 w-3/4" />
      <Skeleton variant="text" className="mb-4 w-1/2" />
      <div className="flex items-center gap-2">
        <Skeleton variant="circular" width={32} height={32} />
        <Skeleton variant="text" className="w-24" />
      </div>
    </div>
  );
}

/**
 * Pre-built skeleton for list items
 */
export function SkeletonListItem({ className }: { className?: string }) {
  return (
    <div className={cn('flex items-center gap-4 p-4', className)}>
      <Skeleton variant="circular" width={48} height={48} />
      <div className="flex-1">
        <Skeleton variant="text" className="mb-2 w-1/3" />
        <Skeleton variant="text" className="w-2/3" />
      </div>
    </div>
  );
}

export default Skeleton;
