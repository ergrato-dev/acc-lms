// ============================================
// Domain Entity Types
// Based on database schema and API contracts
// ============================================

/**
 * User entity
 */
export interface User {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
  avatarUrl?: string;
  bio?: string;
  role: 'student' | 'instructor' | 'admin';
  isActive: boolean;
  emailVerified: boolean;
  createdAt: string;
  updatedAt: string;
}

/**
 * User preferences
 */
export interface UserPreferences {
  locale: 'es' | 'en' | 'pt';
  theme: 'light' | 'dark' | 'system';
  emailNotifications: boolean;
  marketingEmails: boolean;
}

/**
 * Course entity
 */
export interface Course {
  id: string;
  slug: string;
  title: string;
  description: string;
  shortDescription?: string;
  thumbnailUrl?: string;
  previewVideoUrl?: string;
  priceAmount: number;
  priceCurrency: string;
  level: 'beginner' | 'intermediate' | 'advanced' | 'all_levels';
  language: string;
  category: string;
  subcategory?: string;
  tags: string[];
  requirements: string[];
  learningOutcomes: string[];
  status: 'draft' | 'published' | 'archived';
  instructor: Instructor;
  totalLessons: number;
  totalDurationSeconds: number;
  totalStudents: number;
  averageRating?: number;
  totalReviews: number;
  createdAt: string;
  updatedAt: string;
  publishedAt?: string;
}

/**
 * Course summary (for lists/cards)
 */
export interface CourseSummary {
  id: string;
  slug: string;
  title: string;
  shortDescription?: string;
  thumbnailUrl?: string;
  priceAmount: number;
  priceCurrency: string;
  level: string;
  instructorName: string;
  totalLessons: number;
  totalDurationSeconds: number;
  totalStudents: number;
  averageRating?: number;
  totalReviews: number;
  isBestseller?: boolean;
  isNew?: boolean;
}

/**
 * Instructor info
 */
export interface Instructor {
  id: string;
  firstName: string;
  lastName: string;
  avatarUrl?: string;
  bio?: string;
  headline?: string;
  totalCourses: number;
  totalStudents: number;
  averageRating?: number;
}

/**
 * Lesson entity
 */
export interface Lesson {
  id: string;
  courseId: string;
  title: string;
  description?: string;
  type: 'video' | 'article' | 'quiz';
  durationSeconds: number;
  orderIndex: number;
  isPreview: boolean;
  contentUrl?: string;
  articleContent?: string;
  quizId?: string;
}

/**
 * Lesson with progress (for enrolled students)
 */
export interface LessonWithProgress extends Lesson {
  isCompleted: boolean;
  progressPercent: number;
  lastWatchedAt?: string;
  lastPosition?: number;
}

/**
 * Section (lesson grouping)
 */
export interface Section {
  id: string;
  courseId: string;
  title: string;
  orderIndex: number;
  lessons: Lesson[];
}

/**
 * Enrollment entity
 */
export interface Enrollment {
  id: string;
  userId: string;
  courseId: string;
  course: CourseSummary;
  status: 'active' | 'completed' | 'expired' | 'cancelled';
  progressPercent: number;
  completedLessons: number;
  totalLessons: number;
  startedAt: string;
  completedAt?: string;
  lastAccessedAt?: string;
}

/**
 * Order entity
 */
export interface Order {
  id: string;
  userId: string;
  courseId: string;
  course: CourseSummary;
  amountCents: number;
  currency: string;
  status: 'pending' | 'paid' | 'failed' | 'refunded' | 'cancelled';
  paymentProvider: 'stripe' | 'mercadopago';
  paymentIntentId?: string;
  couponCode?: string;
  discountCents?: number;
  createdAt: string;
  paidAt?: string;
}

/**
 * Review entity
 */
export interface Review {
  id: string;
  courseId: string;
  userId: string;
  userName: string;
  userAvatarUrl?: string;
  rating: number;
  content: string;
  helpfulCount: number;
  createdAt: string;
  updatedAt: string;
}

/**
 * Quiz entity
 */
export interface Quiz {
  id: string;
  lessonId: string;
  title: string;
  description?: string;
  passingScore: number;
  timeLimit?: number;
  questions: QuizQuestion[];
}

/**
 * Quiz question
 */
export interface QuizQuestion {
  id: string;
  type: 'multiple_choice' | 'true_false' | 'short_answer';
  question: string;
  options?: string[];
  correctAnswer?: string | string[];
  explanation?: string;
  points: number;
}

/**
 * Quiz submission
 */
export interface QuizSubmission {
  id: string;
  quizId: string;
  userId: string;
  answers: Record<string, string | string[]>;
  score: number;
  passed: boolean;
  submittedAt: string;
}

/**
 * Certificate entity
 */
export interface Certificate {
  id: string;
  userId: string;
  courseId: string;
  courseName: string;
  userName: string;
  issuedAt: string;
  downloadUrl: string;
  verificationCode: string;
}

/**
 * Wishlist item
 */
export interface WishlistItem {
  id: string;
  userId: string;
  courseId: string;
  course: CourseSummary;
  addedAt: string;
}

/**
 * Cart item
 */
export interface CartItem {
  courseId: string;
  course: CourseSummary;
  addedAt: string;
}

/**
 * Notification entity
 */
export interface Notification {
  id: string;
  userId: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  isRead: boolean;
  actionUrl?: string;
  createdAt: string;
}

/**
 * Course category
 */
export interface CourseCategory {
  id: string;
  name: string;
  slug: string;
  description?: string;
  iconUrl?: string;
  parentId?: string;
  courseCount: number;
  subcategories?: CourseCategory[];
}

/**
 * Course list query parameters
 */
export interface CourseListParams {
  page?: number;
  limit?: number;
  search?: string;
  category?: string;
  level?: string;
  priceMin?: number;
  priceMax?: number;
  rating?: number;
  language?: string;
  sortBy?: 'newest' | 'popular' | 'rating' | 'price_low' | 'price_high';
}

/**
 * User profile (extended user data)
 */
export interface UserProfile extends User {
  headline?: string;
  website?: string;
  socialLinks?: {
    linkedin?: string;
    twitter?: string;
    youtube?: string;
    github?: string;
  };
  completedCourses: number;
  totalWatchTime: number;
  certificatesEarned: number;
}

/**
 * Update profile request
 */
export interface UpdateProfileRequest {
  firstName?: string;
  lastName?: string;
  bio?: string;
  headline?: string;
  website?: string;
  socialLinks?: {
    linkedin?: string;
    twitter?: string;
    youtube?: string;
    github?: string;
  };
}

/**
 * Payment method (saved card)
 */
export interface PaymentMethod {
  id: string;
  type: 'card';
  brand: string;
  last4: string;
  expiryMonth: number;
  expiryYear: number;
  isDefault: boolean;
}

/**
 * Payment intent (for processing payment)
 */
export interface PaymentIntent {
  id: string;
  clientSecret: string;
  amount: number;
  currency: string;
  status:
    | 'requires_payment_method'
    | 'requires_confirmation'
    | 'processing'
    | 'succeeded'
    | 'canceled';
}

/**
 * Progress tracking
 */
export interface Progress {
  enrollmentId: string;
  courseId: string;
  completedLessons: string[];
  totalLessons: number;
  progressPercent: number;
  lastLessonId?: string;
  lastPosition?: number;
  totalWatchTime: number;
}
