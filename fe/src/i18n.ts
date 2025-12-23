import i18n from 'i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import HttpBackend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

// Namespaces for lazy loading
export const defaultNS = 'common';
export const namespaces = [
  'common',
  'auth',
  'courses',
  'checkout',
  'player',
  'account',
  'instructor',
  'errors',
] as const;

export type Namespace = (typeof namespaces)[number];

i18n
  // Load translations via HTTP
  .use(HttpBackend)
  // Detect user language
  .use(LanguageDetector)
  // Pass i18n to react-i18next
  .use(initReactI18next)
  // Initialize i18next
  .init({
    // Supported languages (RNF-015)
    supportedLngs: ['es', 'en', 'pt'],
    fallbackLng: 'es',
    defaultNS,
    ns: namespaces,

    // Debug in development
    debug: import.meta.env.DEV,

    // Interpolation settings
    interpolation: {
      escapeValue: false, // React already escapes
    },

    // Backend configuration for loading translations
    backend: {
      loadPath: '/locales/{{lng}}/{{ns}}.json',
    },

    // Language detection settings
    detection: {
      // Order of detection methods
      order: ['querystring', 'cookie', 'localStorage', 'navigator', 'htmlTag'],
      // Keys to look for
      lookupQuerystring: 'lang',
      lookupCookie: 'i18next',
      lookupLocalStorage: 'i18nextLng',
      // Cache user language
      caches: ['localStorage', 'cookie'],
      // Cookie settings
      cookieMinutes: 10080, // 7 days
    },

    // React specific settings
    react: {
      useSuspense: true,
    },
  });

export default i18n;
