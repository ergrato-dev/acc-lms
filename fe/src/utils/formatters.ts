import dayjs from 'dayjs';
import 'dayjs/locale/en';
import 'dayjs/locale/es';
import 'dayjs/locale/pt';
import localizedFormat from 'dayjs/plugin/localizedFormat';
import relativeTime from 'dayjs/plugin/relativeTime';

// Load plugins
dayjs.extend(relativeTime);
dayjs.extend(localizedFormat);

// Locale configuration based on RNF-015
export type Locale = 'es' | 'en' | 'pt';

interface LocaleConfig {
  dateFormat: string;
  dateTimeFormat: string;
  timeFormat: string;
  currency: string;
  currencyLocale: string;
  numberLocale: string;
}

const localeConfigs: Record<Locale, LocaleConfig> = {
  es: {
    dateFormat: 'DD/MM/YYYY',
    dateTimeFormat: 'DD/MM/YYYY HH:mm',
    timeFormat: 'HH:mm',
    currency: 'COP',
    currencyLocale: 'es-CO',
    numberLocale: 'es-CO',
  },
  en: {
    dateFormat: 'MM/DD/YYYY',
    dateTimeFormat: 'MM/DD/YYYY h:mm A',
    timeFormat: 'h:mm A',
    currency: 'USD',
    currencyLocale: 'en-US',
    numberLocale: 'en-US',
  },
  pt: {
    dateFormat: 'DD/MM/YYYY',
    dateTimeFormat: 'DD/MM/YYYY HH:mm',
    timeFormat: 'HH:mm',
    currency: 'BRL',
    currencyLocale: 'pt-BR',
    numberLocale: 'pt-BR',
  },
};

/**
 * Set the locale for dayjs
 */
export function setLocale(locale: Locale): void {
  dayjs.locale(locale);
}

/**
 * Get locale configuration
 */
export function getLocaleConfig(locale: Locale): LocaleConfig {
  return localeConfigs[locale];
}

/**
 * Format a date according to locale
 */
export function formatDate(
  date: string | Date | number,
  locale: Locale = 'es'
): string {
  const config = localeConfigs[locale];
  return dayjs(date).locale(locale).format(config.dateFormat);
}

/**
 * Format a date with time according to locale
 */
export function formatDateTime(
  date: string | Date | number,
  locale: Locale = 'es'
): string {
  const config = localeConfigs[locale];
  return dayjs(date).locale(locale).format(config.dateTimeFormat);
}

/**
 * Format time only according to locale
 */
export function formatTime(
  date: string | Date | number,
  locale: Locale = 'es'
): string {
  const config = localeConfigs[locale];
  return dayjs(date).locale(locale).format(config.timeFormat);
}

/**
 * Format relative time (e.g., "2 hours ago")
 */
export function formatRelativeTime(
  date: string | Date | number,
  locale: Locale = 'es'
): string {
  return dayjs(date).locale(locale).fromNow();
}

/**
 * Format currency according to locale
 */
export function formatCurrency(
  amount: number,
  locale: Locale = 'es',
  currency?: string
): string {
  const config = localeConfigs[locale];
  return new Intl.NumberFormat(config.currencyLocale, {
    style: 'currency',
    currency: currency ?? config.currency,
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(amount);
}

/**
 * Format a number according to locale
 */
export function formatNumber(
  value: number,
  locale: Locale = 'es',
  options?: Intl.NumberFormatOptions
): string {
  const config = localeConfigs[locale];
  return new Intl.NumberFormat(config.numberLocale, options).format(value);
}

/**
 * Format a percentage
 */
export function formatPercent(
  value: number,
  locale: Locale = 'es',
  decimals = 0
): string {
  const config = localeConfigs[locale];
  return new Intl.NumberFormat(config.numberLocale, {
    style: 'percent',
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
}

/**
 * Format file size in human-readable format
 */
export function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let unitIndex = 0;
  let size = bytes;

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }

  return `${size.toFixed(unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
}

/**
 * Format duration in HH:MM:SS or MM:SS format
 */
export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Format duration in human-readable format (e.g., "2h 30m")
 */
export function formatDurationHuman(
  seconds: number,
  locale: Locale = 'es'
): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  const translations = {
    es: { h: 'h', m: 'min' },
    en: { h: 'h', m: 'min' },
    pt: { h: 'h', m: 'min' },
  };

  const t = translations[locale];

  if (hours > 0 && minutes > 0) {
    return `${hours}${t.h} ${minutes}${t.m}`;
  }
  if (hours > 0) {
    return `${hours}${t.h}`;
  }
  return `${minutes}${t.m}`;
}

/**
 * Format a phone number
 */
export function formatPhone(phone: string, locale: Locale = 'es'): string {
  // Remove all non-digits
  const digits = phone.replace(/\D/g, '');

  // Format based on locale
  switch (locale) {
    case 'en':
      // US format: (XXX) XXX-XXXX
      if (digits.length === 10) {
        return `(${digits.slice(0, 3)}) ${digits.slice(3, 6)}-${digits.slice(6)}`;
      }
      break;
    case 'es':
    case 'pt':
      // Format: XXX XXX XXXX
      if (digits.length === 10) {
        return `${digits.slice(0, 3)} ${digits.slice(3, 6)} ${digits.slice(6)}`;
      }
      break;
  }

  return phone;
}
