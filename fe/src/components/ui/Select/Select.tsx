import { cn } from '@/utils';
import { forwardRef, type SelectHTMLAttributes, useId } from 'react';

export type SelectSize = 'sm' | 'md' | 'lg';

export interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface SelectProps extends Omit<
  SelectHTMLAttributes<HTMLSelectElement>,
  'size'
> {
  label?: string;
  helperText?: string;
  error?: string;
  size?: SelectSize;
  options: SelectOption[];
  placeholder?: string;
  fullWidth?: boolean;
}

const sizeStyles: Record<SelectSize, string> = {
  sm: 'h-8 text-sm px-3',
  md: 'h-10 px-4',
  lg: 'h-12 text-lg px-4',
};

/**
 * Select component with label and error states
 *
 * @example
 * ```tsx
 * <Select
 *   label="Country"
 *   options={[
 *     { value: 'us', label: 'United States' },
 *     { value: 'mx', label: 'Mexico' },
 *   ]}
 * />
 * ```
 */
const Select = forwardRef<HTMLSelectElement, SelectProps>(
  (
    {
      className,
      label,
      helperText,
      error,
      size = 'md',
      options,
      placeholder,
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
          <select
            ref={ref}
            id={id}
            disabled={disabled}
            aria-invalid={hasError}
            aria-describedby={
              hasError ? `${id}-error` : helperText ? `${id}-helper` : undefined
            }
            className={cn(
              // Base styles
              'bg-background w-full appearance-none rounded-md border pr-10',
              'text-foreground',
              'transition-colors duration-200',
              'focus:ring-ring focus:ring-2 focus:ring-offset-2 focus:outline-none',
              'disabled:cursor-not-allowed disabled:opacity-50',
              // Size styles
              sizeStyles[size],
              // Error styles
              hasError
                ? 'border-destructive focus:ring-destructive'
                : 'border-input',
              // Custom classes
              className
            )}
            {...props}
          >
            {placeholder && (
              <option value="" disabled>
                {placeholder}
              </option>
            )}
            {options.map((option) => (
              <option
                key={option.value}
                value={option.value}
                disabled={option.disabled}
              >
                {option.label}
              </option>
            ))}
          </select>
          {/* Dropdown arrow */}
          <div className="text-muted-foreground pointer-events-none absolute top-1/2 right-3 -translate-y-1/2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <polyline points="6 9 12 15 18 9" />
            </svg>
          </div>
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

Select.displayName = 'Select';

export default Select;
