// API Configuration
export const API_CONFIG = {
  BASE_URL: import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1',
  TIMEOUT: 30000, // 30 seconds
} as const;

// Authentication
export const AUTH_CONFIG = {
  ACCESS_TOKEN_EXPIRY: 15 * 60 * 1000, // 15 minutes
  REFRESH_TOKEN_EXPIRY: 7 * 24 * 60 * 60 * 1000, // 7 days
  MAX_LOGIN_ATTEMPTS: 5,
  LOCKOUT_DURATION: 15 * 60 * 1000, // 15 minutes
} as const;

// Pagination
export const PAGINATION = {
  DEFAULT_PAGE: 1,
  DEFAULT_PAGE_SIZE: 20,
  MAX_PAGE_SIZE: 100,
} as const;

// File Upload
export const FILE_UPLOAD = {
  MAX_IMAGE_SIZE: 2 * 1024 * 1024, // 2MB
  MAX_VIDEO_SIZE: 500 * 1024 * 1024, // 500MB
  ALLOWED_IMAGE_TYPES: ['image/jpeg', 'image/png', 'image/webp', 'image/gif'],
  ALLOWED_VIDEO_TYPES: ['video/mp4', 'video/webm', 'video/quicktime'],
  ALLOWED_DOCUMENT_TYPES: ['application/pdf'],
} as const;

// Rating
export const RATING = {
  MIN: 1,
  MAX: 5,
} as const;

// Course Levels
export const COURSE_LEVELS = ['beginner', 'intermediate', 'advanced', 'all_levels'] as const;
export type CourseLevel = (typeof COURSE_LEVELS)[number];

// User Roles
export const USER_ROLES = ['student', 'instructor', 'admin'] as const;
export type UserRole = (typeof USER_ROLES)[number];

// Order Status
export const ORDER_STATUS = ['pending', 'paid', 'failed', 'refunded', 'cancelled'] as const;
export type OrderStatus = (typeof ORDER_STATUS)[number];

// Enrollment Status
export const ENROLLMENT_STATUS = ['active', 'completed', 'expired', 'cancelled'] as const;
export type EnrollmentStatus = (typeof ENROLLMENT_STATUS)[number];

// Lesson Types
export const LESSON_TYPES = ['video', 'article', 'quiz'] as const;
export type LessonType = (typeof LESSON_TYPES)[number];

// Payment Providers
export const PAYMENT_PROVIDERS = ['stripe', 'mercadopago'] as const;
export type PaymentProvider = (typeof PAYMENT_PROVIDERS)[number];

// Contact Categories (RF-SUPPORT-002)
export const CONTACT_CATEGORIES = [
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
] as const;
export type ContactCategory = (typeof CONTACT_CATEGORIES)[number];

// Breakpoints (matching Tailwind)
export const BREAKPOINTS = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1536,
} as const;

// Routes
export const ROUTES = {
  HOME: '/',
  COURSES: '/courses',
  COURSE_DETAIL: '/course/:slug',
  LOGIN: '/login',
  REGISTER: '/register',
  FORGOT_PASSWORD: '/forgot-password',
  RESET_PASSWORD: '/reset-password',
  CHECKOUT: '/checkout',
  CHECKOUT_SUCCESS: '/checkout/success',
  LEARN: '/learn/:courseId',
  ACCOUNT: '/account',
  ACCOUNT_PROFILE: '/account/profile',
  ACCOUNT_COURSES: '/account/courses',
  ACCOUNT_ORDERS: '/account/orders',
  ACCOUNT_CERTIFICATES: '/account/certificates',
  ACCOUNT_WISHLIST: '/account/wishlist',
  INSTRUCTOR: '/instructor',
  INSTRUCTOR_COURSES: '/instructor/courses',
  INSTRUCTOR_COURSE_NEW: '/instructor/courses/new',
  INSTRUCTOR_COURSE_EDIT: '/instructor/courses/:id/edit',
  INSTRUCTOR_ANALYTICS: '/instructor/analytics',
  ADMIN: '/admin',
  ADMIN_USERS: '/admin/users',
  ADMIN_COURSES: '/admin/courses',
  CONTACT: '/contact',
  SUPPORT: '/support',
  TERMS: '/terms',
  PRIVACY: '/privacy',
  ACCESSIBILITY: '/accessibility',
} as const;

// Keyboard shortcuts
export const KEYBOARD_SHORTCUTS = {
  SEARCH: 'ctrl+k',
  ESCAPE: 'Escape',
  ENTER: 'Enter',
  SPACE: ' ',
} as const;
