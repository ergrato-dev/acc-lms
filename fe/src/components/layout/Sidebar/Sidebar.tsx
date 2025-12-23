import { cn } from '@/utils';
import { useCallback, useState, type ReactNode } from 'react';
import { useTranslation } from 'react-i18next';
import { Link, useLocation } from 'react-router-dom';

export interface SidebarItem {
  id: string;
  label: string;
  href: string;
  icon?: ReactNode;
  badge?: string | number;
  children?: SidebarItem[];
}

export interface SidebarProps {
  items: SidebarItem[];
  className?: string;
  collapsible?: boolean;
  defaultCollapsed?: boolean;
}

/**
 * Sidebar navigation component for dashboard layouts
 */
function Sidebar({
  items,
  className,
  collapsible = true,
  defaultCollapsed = false,
}: SidebarProps) {
  const { t } = useTranslation();
  const location = useLocation();
  const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
  const [expandedItems, setExpandedItems] = useState<Set<string>>(new Set());

  const toggleCollapse = useCallback(() => {
    setIsCollapsed((prev) => !prev);
  }, []);

  const toggleExpanded = useCallback((id: string) => {
    setExpandedItems((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  const isActive = (href: string) => location.pathname === href;
  const isParentActive = (item: SidebarItem) =>
    item.children?.some((child) => isActive(child.href));

  const renderItem = (item: SidebarItem, level = 0) => {
    const hasChildren = item.children && item.children.length > 0;
    const isExpanded = expandedItems.has(item.id);
    const active = isActive(item.href) || isParentActive(item);

    return (
      <li key={item.id}>
        {hasChildren ? (
          <>
            <button
              onClick={() => toggleExpanded(item.id)}
              className={cn(
                'flex w-full items-center justify-between gap-3 rounded-md px-3 py-2 text-sm',
                'transition-colors duration-200',
                active
                  ? 'bg-primary/10 text-primary'
                  : 'text-muted-foreground hover:bg-accent hover:text-foreground',
                isCollapsed && 'justify-center px-2'
              )}
            >
              <div className="flex items-center gap-3">
                {item.icon && (
                  <span className={cn('shrink-0', isCollapsed && 'mx-auto')}>
                    {item.icon}
                  </span>
                )}
                {!isCollapsed && <span>{item.label}</span>}
              </div>
              {!isCollapsed && (
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
                  className={cn(
                    'transition-transform duration-200',
                    isExpanded && 'rotate-180'
                  )}
                >
                  <polyline points="6 9 12 15 18 9" />
                </svg>
              )}
            </button>
            {!isCollapsed && isExpanded && (
              <ul className="mt-1 space-y-1 pl-4">
                {item.children.map((child) => renderItem(child, level + 1))}
              </ul>
            )}
          </>
        ) : (
          <Link
            to={item.href}
            className={cn(
              'flex items-center gap-3 rounded-md px-3 py-2 text-sm',
              'transition-colors duration-200',
              active
                ? 'bg-primary/10 text-primary font-medium'
                : 'text-muted-foreground hover:bg-accent hover:text-foreground',
              isCollapsed && 'justify-center px-2'
            )}
            title={isCollapsed ? item.label : undefined}
          >
            {item.icon && (
              <span className={cn('shrink-0', isCollapsed && 'mx-auto')}>
                {item.icon}
              </span>
            )}
            {!isCollapsed && (
              <>
                <span className="flex-1">{item.label}</span>
                {item.badge && (
                  <span className="bg-primary text-primary-foreground rounded-full px-2 py-0.5 text-xs">
                    {item.badge}
                  </span>
                )}
              </>
            )}
          </Link>
        )}
      </li>
    );
  };

  return (
    <aside
      className={cn(
        'bg-card flex h-full flex-col border-r transition-all duration-300',
        isCollapsed ? 'w-16' : 'w-64',
        className
      )}
    >
      {/* Collapse toggle */}
      {collapsible && (
        <div className="flex items-center justify-end border-b p-2">
          <button
            onClick={toggleCollapse}
            className="text-muted-foreground hover:bg-accent hover:text-foreground rounded-md p-2"
            aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
          >
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
              className={cn(
                'transition-transform duration-200',
                isCollapsed && 'rotate-180'
              )}
            >
              <polyline points="15 18 9 12 15 6" />
            </svg>
          </button>
        </div>
      )}

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto p-2">
        <ul className="space-y-1">{items.map((item) => renderItem(item))}</ul>
      </nav>
    </aside>
  );
}

export default Sidebar;
