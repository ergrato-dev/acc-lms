// ============================================
// UI-specific Types
// ============================================

import type { ReactNode } from 'react';

/**
 * Common component sizes
 */
export type Size = 'xs' | 'sm' | 'md' | 'lg' | 'xl';

/**
 * Common component variants
 */
export type Variant = 'primary' | 'secondary' | 'outline' | 'ghost' | 'link';

/**
 * Status variants for badges, alerts, etc.
 */
export type StatusVariant = 'info' | 'success' | 'warning' | 'error';

/**
 * Base props for polymorphic components
 */
export interface AsChildProps {
  asChild?: boolean;
}

/**
 * Common disabled state props
 */
export interface DisabledProps {
  disabled?: boolean;
}

/**
 * Common loading state props
 */
export interface LoadingProps {
  isLoading?: boolean;
  loadingText?: string;
}

/**
 * Props for components with children
 */
export interface ChildrenProps {
  children?: ReactNode;
}

/**
 * Props for components with className
 */
export interface ClassNameProps {
  className?: string;
}

/**
 * Combined base props
 */
export interface BaseComponentProps
  extends ClassNameProps,
    ChildrenProps,
    DisabledProps {}

/**
 * Form field props
 */
export interface FormFieldProps {
  label?: string;
  error?: string;
  hint?: string;
  required?: boolean;
}

/**
 * Select option
 */
export interface SelectOption<T = string> {
  value: T;
  label: string;
  disabled?: boolean;
}

/**
 * Tab item
 */
export interface TabItem {
  id: string;
  label: string;
  content: ReactNode;
  disabled?: boolean;
  icon?: ReactNode;
}

/**
 * Breadcrumb item
 */
export interface BreadcrumbItem {
  label: string;
  href?: string;
  isCurrent?: boolean;
}

/**
 * Menu item (for dropdowns, navigation)
 */
export interface MenuItem {
  id: string;
  label: string;
  href?: string;
  onClick?: () => void;
  icon?: ReactNode;
  disabled?: boolean;
  children?: MenuItem[];
}

/**
 * Toast notification
 */
export interface Toast {
  id: string;
  type: StatusVariant;
  title: string;
  message?: string;
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}

/**
 * Modal/Dialog props
 */
export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  description?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
}

/**
 * Pagination state
 */
export interface PaginationState {
  page: number;
  pageSize: number;
  totalItems: number;
  totalPages: number;
}

/**
 * Sort state
 */
export interface SortState {
  sortBy: string;
  sortOrder: 'asc' | 'desc';
}

/**
 * Filter state (generic)
 */
export interface FilterState {
  [key: string]: string | number | boolean | string[] | undefined;
}

/**
 * Theme mode
 */
export type ThemeMode = 'light' | 'dark' | 'system';

/**
 * Theme context value
 */
export interface ThemeContextValue {
  theme: ThemeMode;
  resolvedTheme: 'light' | 'dark';
  setTheme: (theme: ThemeMode) => void;
}

/**
 * Video player state
 */
export interface VideoPlayerState {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  isMuted: boolean;
  playbackRate: number;
  quality: string;
  isFullscreen: boolean;
  buffered: number;
}

/**
 * Video player controls
 */
export interface VideoPlayerControls {
  play: () => void;
  pause: () => void;
  seek: (time: number) => void;
  setVolume: (volume: number) => void;
  toggleMute: () => void;
  setPlaybackRate: (rate: number) => void;
  setQuality: (quality: string) => void;
  toggleFullscreen: () => void;
}
