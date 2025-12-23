import { Spinner, ToastProvider } from '@/components/ui';
import { AuthProvider, ThemeProvider } from '@/context';
import '@/i18n';
import { router } from '@/routes';
import { Suspense } from 'react';
import { RouterProvider } from 'react-router-dom';

function AppLoader() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <Spinner size="lg" />
    </div>
  );
}

function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <ToastProvider>
          <Suspense fallback={<AppLoader />}>
            <RouterProvider router={router} />
          </Suspense>
        </ToastProvider>
      </AuthProvider>
    </ThemeProvider>
  );
}

export default App;
