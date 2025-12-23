import { z } from 'zod';

// ============================================
// Validation Schemas using Zod
// Based on RF-AUTH requirements
// ============================================

/**
 * Password requirements (RF-AUTH-001):
 * - At least 10 characters
 * - At least one uppercase letter
 * - At least one lowercase letter
 * - At least one number
 * - At least one special character
 */
export const passwordSchema = z
  .string()
  .min(10, 'validation.min_length')
  .regex(/[A-Z]/, 'validation.password_weak')
  .regex(/[a-z]/, 'validation.password_weak')
  .regex(/[0-9]/, 'validation.password_weak')
  .regex(/[!@#$%^&*(),.?":{}|<>]/, 'validation.password_weak');

/**
 * Email validation schema
 */
export const emailSchema = z
  .string()
  .min(1, 'validation.required')
  .email('validation.email');

/**
 * Name validation schema
 */
export const nameSchema = z
  .string()
  .min(1, 'validation.required')
  .min(2, 'validation.min_length')
  .max(50, 'validation.max_length');

/**
 * Login form schema
 */
export const loginSchema = z.object({
  email: emailSchema,
  password: z.string().min(1, 'validation.required'),
  rememberMe: z.boolean().optional(),
});

export type LoginFormData = z.infer<typeof loginSchema>;

/**
 * Registration form schema
 */
export const registerSchema = z
  .object({
    firstName: nameSchema,
    lastName: nameSchema,
    email: emailSchema,
    password: passwordSchema,
    confirmPassword: z.string().min(1, 'validation.required'),
    acceptTerms: z.literal(true, {
      errorMap: () => ({ message: 'validation.required' }),
    }),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'validation.passwords_match',
    path: ['confirmPassword'],
  });

export type RegisterFormData = z.infer<typeof registerSchema>;

/**
 * Forgot password form schema
 */
export const forgotPasswordSchema = z.object({
  email: emailSchema,
});

export type ForgotPasswordFormData = z.infer<typeof forgotPasswordSchema>;

/**
 * Reset password form schema
 */
export const resetPasswordSchema = z
  .object({
    password: passwordSchema,
    confirmPassword: z.string().min(1, 'validation.required'),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'validation.passwords_match',
    path: ['confirmPassword'],
  });

export type ResetPasswordFormData = z.infer<typeof resetPasswordSchema>;

/**
 * Profile update schema
 */
export const profileSchema = z.object({
  firstName: nameSchema,
  lastName: nameSchema,
  bio: z.string().max(500, 'validation.max_length').optional(),
  avatarUrl: z.string().url().optional().or(z.literal('')),
});

export type ProfileFormData = z.infer<typeof profileSchema>;

/**
 * Contact form schema (RF-SUPPORT-002)
 */
export const contactSchema = z.object({
  category: z.enum([
    'technical_support',
    'billing_inquiry',
    'course_question',
    'refund_request',
    'partnership',
    'bug_report',
    'feature_request',
    'account_issue',
    'content_report',
    'other',
  ]),
  senderEmail: emailSchema,
  senderName: nameSchema,
  courseId: z.string().uuid().optional(),
  orderId: z.string().uuid().optional(),
  subject: z.string().min(1, 'validation.required').max(200, 'validation.max_length'),
  message: z.string().min(1, 'validation.required').max(5000, 'validation.max_length'),
});

export type ContactFormData = z.infer<typeof contactSchema>;

/**
 * Review form schema
 */
export const reviewSchema = z.object({
  rating: z.number().min(1).max(5),
  content: z.string().min(10, 'validation.min_length').max(2000, 'validation.max_length'),
});

export type ReviewFormData = z.infer<typeof reviewSchema>;

// ============================================
// Validation Helper Functions
// ============================================

/**
 * Calculate password strength (0-4)
 */
export function getPasswordStrength(password: string): number {
  let strength = 0;
  if (password.length >= 10) strength++;
  if (/[A-Z]/.test(password)) strength++;
  if (/[a-z]/.test(password)) strength++;
  if (/[0-9]/.test(password)) strength++;
  if (/[!@#$%^&*(),.?":{}|<>]/.test(password)) strength++;
  return Math.min(strength, 4);
}

/**
 * Get password strength label
 */
export function getPasswordStrengthLabel(
  strength: number
): 'weak' | 'fair' | 'good' | 'strong' {
  switch (strength) {
    case 0:
    case 1:
      return 'weak';
    case 2:
      return 'fair';
    case 3:
      return 'good';
    default:
      return 'strong';
  }
}

/**
 * Validate email format
 */
export function isValidEmail(email: string): boolean {
  return emailSchema.safeParse(email).success;
}

/**
 * Validate URL format
 */
export function isValidUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

/**
 * Validate UUID format
 */
export function isValidUuid(value: string): boolean {
  const uuidRegex =
    /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  return uuidRegex.test(value);
}
