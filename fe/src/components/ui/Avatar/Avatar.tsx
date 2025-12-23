import { cn } from '@/utils';
import { type HTMLAttributes } from 'react';

export type AvatarSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

export interface AvatarProps extends HTMLAttributes<HTMLDivElement> {
  src?: string;
  alt?: string;
  name?: string;
  size?: AvatarSize;
  fallback?: string;
}

const sizeStyles: Record<AvatarSize, string> = {
  xs: 'h-6 w-6 text-xs',
  sm: 'h-8 w-8 text-sm',
  md: 'h-10 w-10 text-base',
  lg: 'h-12 w-12 text-lg',
  xl: 'h-16 w-16 text-xl',
};

/**
 * Get initials from name
 */
function getInitials(name: string): string {
  const parts = name.trim().split(/\s+/);
  if (parts.length === 1) {
    return parts[0].slice(0, 2).toUpperCase();
  }
  return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
}

/**
 * Generate consistent color based on name
 */
function getColorFromName(name: string): string {
  const colors = [
    'bg-red-500',
    'bg-orange-500',
    'bg-amber-500',
    'bg-yellow-500',
    'bg-lime-500',
    'bg-green-500',
    'bg-emerald-500',
    'bg-teal-500',
    'bg-cyan-500',
    'bg-sky-500',
    'bg-blue-500',
    'bg-indigo-500',
    'bg-violet-500',
    'bg-purple-500',
    'bg-fuchsia-500',
    'bg-pink-500',
    'bg-rose-500',
  ];

  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  return colors[Math.abs(hash) % colors.length];
}

/**
 * Avatar component with image or initials fallback
 *
 * @example
 * ```tsx
 * <Avatar src="/avatar.jpg" alt="John Doe" />
 * <Avatar name="John Doe" size="lg" />
 * ```
 */
function Avatar({
  src,
  alt,
  name,
  size = 'md',
  fallback,
  className,
  ...props
}: AvatarProps) {
  const initials = fallback || (name ? getInitials(name) : '?');
  const bgColor = name ? getColorFromName(name) : 'bg-muted';

  return (
    <div
      className={cn(
        'relative inline-flex shrink-0 items-center justify-center overflow-hidden rounded-full',
        sizeStyles[size],
        className
      )}
      {...props}
    >
      {src ? (
        <img
          src={src}
          alt={alt || name || 'Avatar'}
          className="h-full w-full object-cover"
          onError={(e) => {
            // Hide broken image and show fallback
            e.currentTarget.style.display = 'none';
          }}
        />
      ) : null}
      <div
        className={cn(
          'absolute inset-0 flex items-center justify-center font-medium text-white',
          bgColor,
          src && 'hidden'
        )}
        aria-hidden={!!src}
      >
        {initials}
      </div>
    </div>
  );
}

export default Avatar;
