import { cn } from '@/utils';
import { type ReactNode } from 'react';
import { Outlet } from 'react-router-dom';
import { Header } from '../Header';
import { Sidebar, type SidebarItem } from '../Sidebar';

export interface DashboardLayoutProps {
  children?: ReactNode;
  sidebarItems: SidebarItem[];
  className?: string;
}

/**
 * Dashboard layout with header and sidebar
 */
function DashboardLayout({
  children,
  sidebarItems,
  className,
}: DashboardLayoutProps) {
  return (
    <div className="flex min-h-screen flex-col">
      <Header />

      <div className="flex flex-1">
        <Sidebar items={sidebarItems} />

        <main className={cn('flex-1 overflow-auto p-6', className)}>
          {children || <Outlet />}
        </main>
      </div>
    </div>
  );
}

export default DashboardLayout;
