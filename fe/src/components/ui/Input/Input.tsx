import { cn } from '@/utils';
import {
  forwardRef,
  type InputHTMLAttributes,
  type ReactNode,
  useId,
} from 'react';

export type InputSize = 'sm' | 'md' | 'lg';

export interface InputProps extends Omit<
  InputHTMLAttributes<HTMLInputElement>,
  'size'
> {
  label?: string;
  helperText?: string;
  error?: string;
  size?: InputSize;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
  fullWidth?: boolean;
}

const sizeStyles: Record<InputSize, string> = {
  sm: 'h-8 text-sm px-3',
  md: 'h-10 px-4',
  lg: 'h-12 text-lg px-4',
};

const iconSizeStyles: Record<InputSize, string> = {
  sm: 'h-4 w-4',
  md: 'h-5 w-5',
  lg: 'h-6 w-6',
};

/**
 * Input component with label, helper text, and error states
 *
 * @example
 * ```tsx
 * <Input label="Email" placeholder="Enter your email" />
 * <Input label="Password" type="password" error="Invalid password" />
 * <Input leftIcon={<SearchIcon />} placeholder="Search..." />
 * ```
 */
const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      label,
      helperText,
      error,
      size = 'md',
      leftIcon,
      rightIcon,
      fullWidth = false,
      disabled,
      id: providedId,
      ...props
    },
    ref
  ) => {
    const generatedId = useId();
    const id = providedId || generatedId;
    const hasError = !!error;

    return (
      <div className={cn('flex flex-col gap-1.5', fullWidth && 'w-full')}>
        {label && (
          <label
            htmlFor={id}
            className={cn(
              'text-sm font-medium',
              hasError ? 'text-destructive' : 'text-foreground'
            )}
          >
            {label}
          </label>
        )}
        <div className="relative">
          {leftIcon && (
            <div
              className={cn(
                'text-muted-foreground absolute top-1/2 left-3 -translate-y-1/2',
                iconSizeStyles[size]
              )}
            >
              {leftIcon}
            </div>
          )}
          <input
            ref={ref}
            id={id}
            disabled={disabled}
            aria-invalid={hasError}
            aria-describedby={
              hasError ? `${id}-error` : helperText ? `${id}-helper` : undefined
            }
            className={cn(
              // Base styles
              'bg-background w-full rounded-md border',
              'text-foreground placeholder:text-muted-foreground',
              'transition-colors duration-200',
              'focus:ring-ring focus:ring-2 focus:ring-offset-2 focus:outline-none',
              'disabled:cursor-not-allowed disabled:opacity-50',
              // Size styles
              sizeStyles[size],
              // Icon padding
              leftIcon && 'pl-10',
              rightIcon && 'pr-10',
              // Error styles
              hasError
                ? 'border-destructive focus:ring-destructive'
                : 'border-input',
              // Custom classes
              className
            )}
            {...props}
          />
          {rightIcon && (
            <div
              className={cn(
                'text-muted-foreground absolute top-1/2 right-3 -translate-y-1/2',
                iconSizeStyles[size]
              )}
            >
              {rightIcon}
            </div>
          )}
        </div>
        {hasError ? (
          <p
            id={`${id}-error`}
            className="text-destructive text-sm"
            role="alert"
          >
            {error}
          </p>
        ) : helperText ? (
          <p id={`${id}-helper`} className="text-muted-foreground text-sm">
            {helperText}
          </p>
        ) : null}
      </div>
    );
  }
);

Input.displayName = 'Input';

export default Input;
