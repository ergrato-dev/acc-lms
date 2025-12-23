import { cn } from '@/utils';
import { type ReactNode } from 'react';
import { Outlet } from 'react-router-dom';
import { Footer } from '../Footer';
import { Header } from '../Header';

export interface MainLayoutProps {
  children?: ReactNode;
  showHeader?: boolean;
  showFooter?: boolean;
  className?: string;
}

/**
 * Main layout component with header and footer
 * Use Outlet for nested routes or children for direct content
 */
function MainLayout({
  children,
  showHeader = true,
  showFooter = true,
  className,
}: MainLayoutProps) {
  return (
    <div className="flex min-h-screen flex-col">
      {showHeader && <Header />}

      <main className={cn('flex-1', className)}>{children || <Outlet />}</main>

      {showFooter && <Footer />}
    </div>
  );
}

export default MainLayout;
