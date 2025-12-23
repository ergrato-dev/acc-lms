import { cn } from '@/utils';
import { forwardRef, type InputHTMLAttributes, useId } from 'react';

export interface CheckboxProps extends Omit<
  InputHTMLAttributes<HTMLInputElement>,
  'type'
> {
  label?: string;
  description?: string;
  error?: string;
}

/**
 * Checkbox component with label and description
 *
 * @example
 * ```tsx
 * <Checkbox label="Accept terms and conditions" />
 * <Checkbox label="Subscribe" description="Get updates by email" />
 * ```
 */
const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(
  (
    {
      className,
      label,
      description,
      error,
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
      <div className="flex flex-col gap-1">
        <div className="flex items-start gap-3">
          <input
            ref={ref}
            type="checkbox"
            id={id}
            disabled={disabled}
            aria-invalid={hasError}
            aria-describedby={
              hasError
                ? `${id}-error`
                : description
                  ? `${id}-description`
                  : undefined
            }
            className={cn(
              'border-input h-4 w-4 shrink-0 rounded border',
              'text-primary focus:ring-ring focus:ring-2 focus:ring-offset-2',
              'disabled:cursor-not-allowed disabled:opacity-50',
              'accent-primary',
              hasError && 'border-destructive',
              className
            )}
            {...props}
          />
          {(label || description) && (
            <div className="flex flex-col gap-0.5">
              {label && (
                <label
                  htmlFor={id}
                  className={cn(
                    'cursor-pointer text-sm leading-none font-medium',
                    disabled && 'cursor-not-allowed opacity-50',
                    hasError ? 'text-destructive' : 'text-foreground'
                  )}
                >
                  {label}
                </label>
              )}
              {description && (
                <p
                  id={`${id}-description`}
                  className="text-muted-foreground text-sm"
                >
                  {description}
                </p>
              )}
            </div>
          )}
        </div>
        {hasError && (
          <p
            id={`${id}-error`}
            className="text-destructive ml-7 text-sm"
            role="alert"
          >
            {error}
          </p>
        )}
      </div>
    );
  }
);

Checkbox.displayName = 'Checkbox';

export default Checkbox;
