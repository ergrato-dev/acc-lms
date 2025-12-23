import { cn } from '@/utils';
import { forwardRef, type TextareaHTMLAttributes, useId } from 'react';

export interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  helperText?: string;
  error?: string;
  fullWidth?: boolean;
}

/**
 * Textarea component with label and error states
 *
 * @example
 * ```tsx
 * <Textarea label="Description" rows={4} />
 * <Textarea error="Field is required" />
 * ```
 */
const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  (
    {
      className,
      label,
      helperText,
      error,
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
        <textarea
          ref={ref}
          id={id}
          disabled={disabled}
          aria-invalid={hasError}
          aria-describedby={
            hasError ? `${id}-error` : helperText ? `${id}-helper` : undefined
          }
          className={cn(
            // Base styles
            'bg-background w-full rounded-md border px-4 py-3',
            'text-foreground placeholder:text-muted-foreground',
            'transition-colors duration-200',
            'focus:ring-ring focus:ring-2 focus:ring-offset-2 focus:outline-none',
            'disabled:cursor-not-allowed disabled:opacity-50',
            'min-h-[80px] resize-y',
            // Error styles
            hasError
              ? 'border-destructive focus:ring-destructive'
              : 'border-input',
            // Custom classes
            className
          )}
          {...props}
        />
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

Textarea.displayName = 'Textarea';

export default Textarea;
