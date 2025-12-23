import { DashboardLayout, MainLayout } from '@/components/layout';
import { Spinner } from '@/components/ui';
import { ROUTES } from '@/utils/constants';
import { lazy, Suspense } from 'react';
import {
  createBrowserRouter,
  Navigate,
  Outlet,
  type RouteObject,
} from 'react-router-dom';

// ============================================
// Lazy loaded pages
// ============================================

// Public pages
const HomePage = lazy(() => import('@/pages/Home'));
const CoursesPage = lazy(() => import('@/pages/Courses'));
const CourseDetailPage = lazy(() => import('@/pages/CourseDetail'));
const AboutPage = lazy(() => import('@/pages/About'));
const ContactPage = lazy(() => import('@/pages/Contact'));

// Auth pages
const LoginPage = lazy(() => import('@/pages/auth/Login'));
const RegisterPage = lazy(() => import('@/pages/auth/Register'));
const ForgotPasswordPage = lazy(() => import('@/pages/auth/ForgotPassword'));
const ResetPasswordPage = lazy(() => import('@/pages/auth/ResetPassword'));
const VerifyEmailPage = lazy(() => import('@/pages/auth/VerifyEmail'));

// Dashboard pages
const DashboardPage = lazy(() => import('@/pages/dashboard/Dashboard'));
const MyCoursesPage = lazy(() => import('@/pages/dashboard/MyCourses'));
const CoursePlayerPage = lazy(() => import('@/pages/dashboard/CoursePlayer'));
const ProfilePage = lazy(() => import('@/pages/dashboard/Profile'));
const SettingsPage = lazy(() => import('@/pages/dashboard/Settings'));
const CertificatesPage = lazy(() => import('@/pages/dashboard/Certificates'));

// Checkout pages
const CheckoutPage = lazy(() => import('@/pages/checkout/Checkout'));
const PaymentSuccessPage = lazy(
  () => import('@/pages/checkout/PaymentSuccess')
);

// Instructor pages
const InstructorDashboardPage = lazy(
  () => import('@/pages/instructor/Dashboard')
);
const InstructorCoursesPage = lazy(() => import('@/pages/instructor/Courses'));
const CourseEditorPage = lazy(() => import('@/pages/instructor/CourseEditor'));
const InstructorAnalyticsPage = lazy(
  () => import('@/pages/instructor/Analytics')
);

// Admin pages
const AdminDashboardPage = lazy(() => import('@/pages/admin/Dashboard'));
const AdminUsersPage = lazy(() => import('@/pages/admin/Users'));
const AdminCoursesPage = lazy(() => import('@/pages/admin/Courses'));
const AdminPaymentsPage = lazy(() => import('@/pages/admin/Payments'));

// Legal pages
const TermsPage = lazy(() => import('@/pages/legal/Terms'));
const PrivacyPage = lazy(() => import('@/pages/legal/Privacy'));
const CookiesPage = lazy(() => import('@/pages/legal/Cookies'));

// Error pages
const NotFoundPage = lazy(() => import('@/pages/errors/NotFound'));
const ServerErrorPage = lazy(() => import('@/pages/errors/ServerError'));

// ============================================
// Loading component
// ============================================

function PageLoader() {
  return (
    <div className="flex min-h-[50vh] items-center justify-center">
      <Spinner size="lg" />
    </div>
  );
}

// ============================================
// Route protection components
// ============================================

import { useAuth } from '@/hooks';

function ProtectedRoute() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return <PageLoader />;
  }

  if (!isAuthenticated) {
    return <Navigate to={ROUTES.AUTH.LOGIN} replace />;
  }

  return <Outlet />;
}

function GuestRoute() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return <PageLoader />;
  }

  if (isAuthenticated) {
    return <Navigate to={ROUTES.DASHBOARD.ROOT} replace />;
  }

  return <Outlet />;
}

function RoleRoute({ allowedRoles }: { allowedRoles: string[] }) {
  const { user, isLoading } = useAuth();

  if (isLoading) {
    return <PageLoader />;
  }

  if (!user || !allowedRoles.includes(user.role)) {
    return <Navigate to={ROUTES.DASHBOARD.ROOT} replace />;
  }

  return <Outlet />;
}

// ============================================
// Dashboard sidebar items
// ============================================

const studentSidebarItems = [
  {
    id: 'dashboard',
    label: 'Dashboard',
    href: ROUTES.DASHBOARD.ROOT,
    icon: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <rect width="7" height="9" x="3" y="3" rx="1" />
        <rect width="7" height="5" x="14" y="3" rx="1" />
        <rect width="7" height="9" x="14" y="12" rx="1" />
        <rect width="7" height="5" x="3" y="16" rx="1" />
      </svg>
    ),
  },
  {
    id: 'my-courses',
    label: 'Mis Cursos',
    href: ROUTES.DASHBOARD.MY_COURSES,
    icon: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
      </svg>
    ),
  },
  {
    id: 'certificates',
    label: 'Certificados',
    href: ROUTES.DASHBOARD.CERTIFICATES,
    icon: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <circle cx="12" cy="8" r="6" />
        <path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" />
      </svg>
    ),
  },
  {
    id: 'profile',
    label: 'Perfil',
    href: ROUTES.DASHBOARD.PROFILE,
    icon: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <circle cx="12" cy="8" r="5" />
        <path d="M20 21a8 8 0 0 0-16 0" />
      </svg>
    ),
  },
  {
    id: 'settings',
    label: 'Configuración',
    href: ROUTES.DASHBOARD.SETTINGS,
    icon: (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
        <circle cx="12" cy="12" r="3" />
      </svg>
    ),
  },
];

// ============================================
// Route definitions
// ============================================

const routes: RouteObject[] = [
  // Public routes with main layout
  {
    element: <MainLayout />,
    children: [
      {
        path: ROUTES.HOME,
        element: (
          <Suspense fallback={<PageLoader />}>
            <HomePage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.COURSES.LIST,
        element: (
          <Suspense fallback={<PageLoader />}>
            <CoursesPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.COURSES.DETAIL,
        element: (
          <Suspense fallback={<PageLoader />}>
            <CourseDetailPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.ABOUT,
        element: (
          <Suspense fallback={<PageLoader />}>
            <AboutPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.CONTACT,
        element: (
          <Suspense fallback={<PageLoader />}>
            <ContactPage />
          </Suspense>
        ),
      },
      // Legal pages
      {
        path: ROUTES.LEGAL.TERMS,
        element: (
          <Suspense fallback={<PageLoader />}>
            <TermsPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.LEGAL.PRIVACY,
        element: (
          <Suspense fallback={<PageLoader />}>
            <PrivacyPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.LEGAL.COOKIES,
        element: (
          <Suspense fallback={<PageLoader />}>
            <CookiesPage />
          </Suspense>
        ),
      },
    ],
  },

  // Auth routes (guest only)
  {
    element: <GuestRoute />,
    children: [
      {
        element: <MainLayout showFooter={false} />,
        children: [
          {
            path: ROUTES.AUTH.LOGIN,
            element: (
              <Suspense fallback={<PageLoader />}>
                <LoginPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.AUTH.REGISTER,
            element: (
              <Suspense fallback={<PageLoader />}>
                <RegisterPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.AUTH.FORGOT_PASSWORD,
            element: (
              <Suspense fallback={<PageLoader />}>
                <ForgotPasswordPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.AUTH.RESET_PASSWORD,
            element: (
              <Suspense fallback={<PageLoader />}>
                <ResetPasswordPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.AUTH.VERIFY_EMAIL,
            element: (
              <Suspense fallback={<PageLoader />}>
                <VerifyEmailPage />
              </Suspense>
            ),
          },
        ],
      },
    ],
  },

  // Protected student dashboard routes
  {
    element: <ProtectedRoute />,
    children: [
      {
        element: <DashboardLayout sidebarItems={studentSidebarItems} />,
        children: [
          {
            path: ROUTES.DASHBOARD.ROOT,
            element: (
              <Suspense fallback={<PageLoader />}>
                <DashboardPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.DASHBOARD.MY_COURSES,
            element: (
              <Suspense fallback={<PageLoader />}>
                <MyCoursesPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.DASHBOARD.PROFILE,
            element: (
              <Suspense fallback={<PageLoader />}>
                <ProfilePage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.DASHBOARD.SETTINGS,
            element: (
              <Suspense fallback={<PageLoader />}>
                <SettingsPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.DASHBOARD.CERTIFICATES,
            element: (
              <Suspense fallback={<PageLoader />}>
                <CertificatesPage />
              </Suspense>
            ),
          },
        ],
      },
      // Course player (full screen, no sidebar)
      {
        path: ROUTES.DASHBOARD.COURSE_PLAYER,
        element: (
          <Suspense fallback={<PageLoader />}>
            <CoursePlayerPage />
          </Suspense>
        ),
      },
      // Checkout routes
      {
        element: <MainLayout />,
        children: [
          {
            path: ROUTES.CHECKOUT.ROOT,
            element: (
              <Suspense fallback={<PageLoader />}>
                <CheckoutPage />
              </Suspense>
            ),
          },
          {
            path: ROUTES.CHECKOUT.SUCCESS,
            element: (
              <Suspense fallback={<PageLoader />}>
                <PaymentSuccessPage />
              </Suspense>
            ),
          },
        ],
      },
    ],
  },

  // Instructor routes
  {
    element: <RoleRoute allowedRoles={['instructor', 'admin']} />,
    children: [
      {
        path: ROUTES.INSTRUCTOR.ROOT,
        element: (
          <Suspense fallback={<PageLoader />}>
            <InstructorDashboardPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.INSTRUCTOR.COURSES,
        element: (
          <Suspense fallback={<PageLoader />}>
            <InstructorCoursesPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.INSTRUCTOR.COURSE_EDITOR,
        element: (
          <Suspense fallback={<PageLoader />}>
            <CourseEditorPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.INSTRUCTOR.ANALYTICS,
        element: (
          <Suspense fallback={<PageLoader />}>
            <InstructorAnalyticsPage />
          </Suspense>
        ),
      },
    ],
  },

  // Admin routes
  {
    element: <RoleRoute allowedRoles={['admin']} />,
    children: [
      {
        path: ROUTES.ADMIN.ROOT,
        element: (
          <Suspense fallback={<PageLoader />}>
            <AdminDashboardPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.ADMIN.USERS,
        element: (
          <Suspense fallback={<PageLoader />}>
            <AdminUsersPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.ADMIN.COURSES,
        element: (
          <Suspense fallback={<PageLoader />}>
            <AdminCoursesPage />
          </Suspense>
        ),
      },
      {
        path: ROUTES.ADMIN.PAYMENTS,
        element: (
          <Suspense fallback={<PageLoader />}>
            <AdminPaymentsPage />
          </Suspense>
        ),
      },
    ],
  },

  // Error routes
  {
    path: '/500',
    element: (
      <Suspense fallback={<PageLoader />}>
        <ServerErrorPage />
      </Suspense>
    ),
  },
  {
    path: '*',
    element: (
      <Suspense fallback={<PageLoader />}>
        <NotFoundPage />
      </Suspense>
    ),
  },
];

// ============================================
// Router instance
// ============================================

export const router = createBrowserRouter(routes);

export default router;
