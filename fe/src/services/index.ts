// API client
export { default as api, clearAuthTokens, setAuthTokens } from './api';

// Auth service
export {
  default as authService,
  forgotPassword,
  getCurrentUser,
  login,
  logout,
  register,
  resendVerification,
  resetPassword,
  verifyEmail,
} from './authService';

// Course service
export {
  default as courseService,
  getCategories,
  getCourseById,
  getCourseBySlug,
  getCourses,
  getCoursesByInstructor,
  getFeaturedCourses,
  getPopularCourses,
  searchCourses,
} from './courseService';

// Enrollment service
export {
  completeLesson,
  enrollInCourse,
  default as enrollmentService,
  getCertificate,
  getEnrollmentById,
  getEnrollmentProgress,
  getMyEnrollments,
} from './enrollmentService';

// User service
export {
  changePassword,
  deleteAccount,
  getNotificationSettings,
  getPreferences,
  getProfile,
  updateNotificationSettings,
  updatePreferences,
  updateProfile,
  uploadAvatar,
  default as userService,
} from './userService';
export type { NotificationSettings, UserPreferences } from './userService';

// Payment service
export {
  confirmPayment,
  createOrder,
  createPaymentIntent,
  getMyOrders,
  getOrderById,
  getPaymentMethods,
  default as paymentService,
  validateCoupon,
} from './paymentService';
export type {
  CouponValidation,
  CreateOrderRequest,
  PaymentIntentRequest,
} from './paymentService';
