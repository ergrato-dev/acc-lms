# Frontend Internationalization (i18n) Implementation Guide

**Version:** 2025-01-01  
**Status:** Implementation Guide  
**Scope:** ACC-LMS Frontend (React + TypeScript)

---

## Table of Contents

1. [Overview](#1-overview)
2. [Architecture](#2-architecture)
3. [Project Structure](#3-project-structure)
4. [Setup & Configuration](#4-setup--configuration)
5. [Translation Files](#5-translation-files)
6. [Usage Patterns](#6-usage-patterns)
7. [Best Practices](#7-best-practices)
8. [Testing](#8-testing)
9. [Adding New Languages](#9-adding-new-languages)

---

## 1. Overview

ACC-LMS implements internationalization from day one, following the principle:

> **Backend**: Error codes only (no UI text)  
> **Frontend**: All user-facing text with lazy-loaded translations

### Supported Languages (MVP)

| Code | Language  | Status  | Direction |
| ---- | --------- | ------- | --------- |
| `es` | Español   | Primary | LTR       |
| `en` | English   | Primary | LTR       |
| `pt` | Português | MVP     | LTR       |

### Key Principles

1. **No hardcoded strings** - All text through translation functions
2. **Type-safe keys** - TypeScript ensures valid translation keys
3. **Lazy loading** - Language bundles loaded on demand
4. **Fallback chain** - `user_lang → browser_lang → es`
5. **Context-aware** - Plurals, gender, formatting handled properly

---

## 2. Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         i18n ARCHITECTURE                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐         │
│  │   User Prefs    │    │  Browser Lang   │    │  Default (es)   │         │
│  │   (localStorage │ →  │  navigator      │ →  │                 │         │
│  │    + backend)   │    │  .language      │    │                 │         │
│  └────────┬────────┘    └────────┬────────┘    └────────┬────────┘         │
│           │                      │                      │                   │
│           └──────────────────────┼──────────────────────┘                   │
│                                  ▼                                          │
│                     ┌────────────────────────┐                              │
│                     │   i18n Instance        │                              │
│                     │   (react-i18next)      │                              │
│                     └───────────┬────────────┘                              │
│                                 │                                           │
│           ┌─────────────────────┼─────────────────────┐                     │
│           ▼                     ▼                     ▼                     │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐           │
│  │  Namespace:     │   │  Namespace:     │   │  Namespace:     │           │
│  │  common         │   │  auth           │   │  courses        │           │
│  │  (always load)  │   │  (lazy)         │   │  (lazy)         │           │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Technology Stack

| Library                            | Purpose                | Version |
| ---------------------------------- | ---------------------- | ------- |
| `i18next`                          | Core i18n framework    | ^23.x   |
| `react-i18next`                    | React bindings         | ^14.x   |
| `i18next-http-backend`             | Lazy load translations | ^2.x    |
| `i18next-browser-languagedetector` | Auto-detect language   | ^7.x    |

---

## 3. Project Structure

```
fe/
├── public/
│   └── locales/                    # Translation files (loaded at runtime)
│       ├── es/
│       │   ├── common.json         # Shared translations (buttons, labels)
│       │   ├── auth.json           # Authentication module
│       │   ├── courses.json        # Courses module
│       │   ├── dashboard.json      # Dashboard module
│       │   ├── errors.json         # Error messages (from API codes)
│       │   └── validation.json     # Form validation messages
│       ├── en/
│       │   ├── common.json
│       │   ├── auth.json
│       │   └── ...
│       └── pt/
│           └── ...
│
├── src/
│   ├── i18n/
│   │   ├── index.ts               # i18n initialization
│   │   ├── config.ts              # Configuration constants
│   │   ├── types.ts               # TypeScript types for keys
│   │   └── utils.ts               # Helper functions
│   │
│   ├── hooks/
│   │   └── useTranslation.ts      # Custom hook with namespace preloading
│   │
│   └── components/
│       └── common/
│           ├── LanguageSwitcher.tsx
│           └── TranslatedText.tsx
```

---

## 4. Setup & Configuration

### 4.1 Installation

```bash
cd fe
pnpm add i18next react-i18next i18next-http-backend i18next-browser-languagedetector
pnpm add -D @types/i18next
```

### 4.2 Core Configuration

```typescript
// fe/src/i18n/config.ts

/**
 * i18n Configuration Constants
 *
 * Defines supported languages, default settings, and namespace configuration
 * for the internationalization system.
 */

/** Supported language codes */
export const SUPPORTED_LANGUAGES = ['es', 'en', 'pt'] as const;
export type SupportedLanguage = (typeof SUPPORTED_LANGUAGES)[number];

/** Default/fallback language */
export const DEFAULT_LANGUAGE: SupportedLanguage = 'es';

/**
 * Translation namespaces
 *
 * - 'common': Always loaded (buttons, labels, navigation)
 * - Others: Lazy loaded when needed
 */
export const NAMESPACES = [
  'common',
  'auth',
  'courses',
  'dashboard',
  'errors',
  'validation',
  'payments',
  'profile',
  'notifications',
] as const;

export type TranslationNamespace = (typeof NAMESPACES)[number];

/** Namespaces to preload on app start */
export const PRELOAD_NAMESPACES: TranslationNamespace[] = ['common', 'errors'];

/** LocalStorage key for user language preference */
export const LANGUAGE_STORAGE_KEY = 'acc-lms-language';

/** Language display names for UI */
export const LANGUAGE_NAMES: Record<SupportedLanguage, string> = {
  es: 'Español',
  en: 'English',
  pt: 'Português',
};

/** Language flags for UI (emoji or icon key) */
export const LANGUAGE_FLAGS: Record<SupportedLanguage, string> = {
  es: '🇪🇸',
  en: '🇺🇸',
  pt: '🇧🇷',
};
```

### 4.3 i18n Initialization

```typescript
// fe/src/i18n/index.ts

/**
 * i18n Initialization Module
 *
 * Sets up react-i18next with:
 * - Lazy loading of translation files
 * - Browser language detection
 * - Fallback chain: user preference → browser → default
 * - Type-safe translation keys
 */

import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import HttpBackend from 'i18next-http-backend';
import LanguageDetector from 'i18next-browser-languagedetector';

import {
  SUPPORTED_LANGUAGES,
  DEFAULT_LANGUAGE,
  PRELOAD_NAMESPACES,
  LANGUAGE_STORAGE_KEY,
} from './config';

/**
 * Initialize i18next instance
 *
 * Call this once at app startup, before rendering the React tree.
 */
i18n
  // Load translations from /public/locales
  .use(HttpBackend)
  // Detect user language
  .use(LanguageDetector)
  // Pass to react-i18next
  .use(initReactI18next)
  // Initialize with config
  .init({
    // ─────────────────────────────────────────────────────────
    // Language Settings
    // ─────────────────────────────────────────────────────────
    supportedLngs: SUPPORTED_LANGUAGES,
    fallbackLng: DEFAULT_LANGUAGE,

    // ─────────────────────────────────────────────────────────
    // Namespace Settings
    // ─────────────────────────────────────────────────────────
    ns: PRELOAD_NAMESPACES,
    defaultNS: 'common',

    // ─────────────────────────────────────────────────────────
    // Backend Settings (loading translation files)
    // ─────────────────────────────────────────────────────────
    backend: {
      // Path to translation files
      loadPath: '/locales/{{lng}}/{{ns}}.json',

      // Add cache busting in production
      queryStringParams: import.meta.env.PROD
        ? { v: import.meta.env.VITE_BUILD_VERSION || '1' }
        : {},
    },

    // ─────────────────────────────────────────────────────────
    // Detection Settings
    // ─────────────────────────────────────────────────────────
    detection: {
      // Order of detection methods
      order: ['localStorage', 'navigator', 'htmlTag'],

      // Where to cache the detected language
      caches: ['localStorage'],

      // LocalStorage key
      lookupLocalStorage: LANGUAGE_STORAGE_KEY,
    },

    // ─────────────────────────────────────────────────────────
    // Interpolation Settings
    // ─────────────────────────────────────────────────────────
    interpolation: {
      // React already escapes values
      escapeValue: false,

      // Format functions for dates, numbers, etc.
      format: (value, format, lng) => {
        if (format === 'uppercase') return value.toUpperCase();
        if (format === 'lowercase') return value.toLowerCase();

        if (value instanceof Date) {
          return new Intl.DateTimeFormat(lng, {
            dateStyle: format as 'full' | 'long' | 'medium' | 'short',
          }).format(value);
        }

        if (typeof value === 'number' && format === 'currency') {
          return new Intl.NumberFormat(lng, {
            style: 'currency',
            currency: lng === 'en' ? 'USD' : lng === 'pt' ? 'BRL' : 'EUR',
          }).format(value);
        }

        return value;
      },
    },

    // ─────────────────────────────────────────────────────────
    // React Settings
    // ─────────────────────────────────────────────────────────
    react: {
      // Suspend while loading translations
      useSuspense: true,

      // Bind i18n to React context
      bindI18n: 'languageChanged',
      bindI18nStore: '',

      // Trans component settings
      transEmptyNodeValue: '',
      transSupportBasicHtmlNodes: true,
      transKeepBasicHtmlNodesFor: ['br', 'strong', 'i', 'em', 'b', 'u'],
    },

    // ─────────────────────────────────────────────────────────
    // Debug (development only)
    // ─────────────────────────────────────────────────────────
    debug: import.meta.env.DEV,

    // Save missing keys to console in dev
    saveMissing: import.meta.env.DEV,
    missingKeyHandler: (lngs, ns, key) => {
      console.warn(
        `[i18n] Missing key: ${ns}:${key} for languages: ${lngs.join(', ')}`
      );
    },
  });

export default i18n;
```

### 4.4 App Integration

```typescript
// fe/src/main.tsx

import React, { Suspense } from 'react';
import ReactDOM from 'react-dom/client';
import { I18nextProvider } from 'react-i18next';

// Initialize i18n BEFORE app renders
import i18n from './i18n';

import App from './App';
import { LoadingSpinner } from './components/common/LoadingSpinner';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <I18nextProvider i18n={i18n}>
      {/* Suspense for lazy-loaded translations */}
      <Suspense fallback={<LoadingSpinner fullScreen />}>
        <App />
      </Suspense>
    </I18nextProvider>
  </React.StrictMode>
);
```

---

## 5. Translation Files

### 5.1 File Structure Convention

Each namespace has a JSON file per language with nested keys:

```
<namespace>.json
├── <section>
│   ├── <element>
│   │   ├── <state>: "translation"
│   │   └── ...
│   └── ...
└── ...
```

### 5.2 Common Namespace (Always Loaded)

```json
// fe/public/locales/es/common.json
{
  "app": {
    "name": "ACC LMS",
    "tagline": "Aprende sin límites"
  },

  "navigation": {
    "home": "Inicio",
    "courses": "Cursos",
    "dashboard": "Mi Panel",
    "profile": "Perfil",
    "settings": "Configuración",
    "logout": "Cerrar Sesión"
  },

  "actions": {
    "save": "Guardar",
    "cancel": "Cancelar",
    "delete": "Eliminar",
    "edit": "Editar",
    "create": "Crear",
    "submit": "Enviar",
    "confirm": "Confirmar",
    "back": "Volver",
    "next": "Siguiente",
    "previous": "Anterior",
    "search": "Buscar",
    "filter": "Filtrar",
    "sort": "Ordenar",
    "refresh": "Actualizar",
    "download": "Descargar",
    "upload": "Subir",
    "share": "Compartir",
    "copy": "Copiar",
    "close": "Cerrar",
    "loading": "Cargando...",
    "processing": "Procesando..."
  },

  "status": {
    "active": "Activo",
    "inactive": "Inactivo",
    "pending": "Pendiente",
    "completed": "Completado",
    "error": "Error",
    "success": "Éxito"
  },

  "time": {
    "today": "Hoy",
    "yesterday": "Ayer",
    "tomorrow": "Mañana",
    "now": "Ahora",
    "ago": "hace {{time}}",
    "in": "en {{time}}",
    "seconds": "{{count}} segundo",
    "seconds_plural": "{{count}} segundos",
    "minutes": "{{count}} minuto",
    "minutes_plural": "{{count}} minutos",
    "hours": "{{count}} hora",
    "hours_plural": "{{count}} horas",
    "days": "{{count}} día",
    "days_plural": "{{count}} días"
  },

  "pagination": {
    "page": "Página {{current}} de {{total}}",
    "showing": "Mostrando {{from}}-{{to}} de {{total}}",
    "first": "Primera",
    "last": "Última",
    "perPage": "Por página"
  },

  "empty": {
    "noResults": "No se encontraron resultados",
    "noData": "No hay datos disponibles"
  },

  "confirm": {
    "title": "¿Estás seguro?",
    "deleteMessage": "Esta acción no se puede deshacer.",
    "yes": "Sí, continuar",
    "no": "No, cancelar"
  }
}
```

```json
// fe/public/locales/en/common.json
{
  "app": {
    "name": "ACC LMS",
    "tagline": "Learn without limits"
  },

  "navigation": {
    "home": "Home",
    "courses": "Courses",
    "dashboard": "Dashboard",
    "profile": "Profile",
    "settings": "Settings",
    "logout": "Log Out"
  },

  "actions": {
    "save": "Save",
    "cancel": "Cancel",
    "delete": "Delete",
    "edit": "Edit",
    "create": "Create",
    "submit": "Submit",
    "confirm": "Confirm",
    "back": "Back",
    "next": "Next",
    "previous": "Previous",
    "search": "Search",
    "filter": "Filter",
    "sort": "Sort",
    "refresh": "Refresh",
    "download": "Download",
    "upload": "Upload",
    "share": "Share",
    "copy": "Copy",
    "close": "Close",
    "loading": "Loading...",
    "processing": "Processing..."
  },

  "status": {
    "active": "Active",
    "inactive": "Inactive",
    "pending": "Pending",
    "completed": "Completed",
    "error": "Error",
    "success": "Success"
  },

  "time": {
    "today": "Today",
    "yesterday": "Yesterday",
    "tomorrow": "Tomorrow",
    "now": "Now",
    "ago": "{{time}} ago",
    "in": "in {{time}}",
    "seconds": "{{count}} second",
    "seconds_plural": "{{count}} seconds",
    "minutes": "{{count}} minute",
    "minutes_plural": "{{count}} minutes",
    "hours": "{{count}} hour",
    "hours_plural": "{{count}} hours",
    "days": "{{count}} day",
    "days_plural": "{{count}} days"
  },

  "pagination": {
    "page": "Page {{current}} of {{total}}",
    "showing": "Showing {{from}}-{{to}} of {{total}}",
    "first": "First",
    "last": "Last",
    "perPage": "Per page"
  },

  "empty": {
    "noResults": "No results found",
    "noData": "No data available"
  },

  "confirm": {
    "title": "Are you sure?",
    "deleteMessage": "This action cannot be undone.",
    "yes": "Yes, continue",
    "no": "No, cancel"
  }
}
```

### 5.3 Auth Namespace

```json
// fe/public/locales/es/auth.json
{
  "login": {
    "title": "Iniciar Sesión",
    "subtitle": "Bienvenido de nuevo",
    "email": "Correo electrónico",
    "emailPlaceholder": "tu@email.com",
    "password": "Contraseña",
    "passwordPlaceholder": "••••••••",
    "rememberMe": "Recordarme",
    "forgotPassword": "¿Olvidaste tu contraseña?",
    "submit": "Iniciar Sesión",
    "submitting": "Iniciando sesión...",
    "noAccount": "¿No tienes cuenta?",
    "signUp": "Regístrate",
    "or": "o continúa con",
    "google": "Google",
    "github": "GitHub"
  },

  "register": {
    "title": "Crear Cuenta",
    "subtitle": "Únete a nuestra comunidad",
    "firstName": "Nombre",
    "firstNamePlaceholder": "Juan",
    "lastName": "Apellido",
    "lastNamePlaceholder": "Pérez",
    "email": "Correo electrónico",
    "emailPlaceholder": "tu@email.com",
    "password": "Contraseña",
    "passwordPlaceholder": "Mínimo 8 caracteres",
    "confirmPassword": "Confirmar contraseña",
    "confirmPasswordPlaceholder": "Repite tu contraseña",
    "terms": "Acepto los <termsLink>Términos de Servicio</termsLink> y la <privacyLink>Política de Privacidad</privacyLink>",
    "submit": "Crear Cuenta",
    "submitting": "Creando cuenta...",
    "hasAccount": "¿Ya tienes cuenta?",
    "signIn": "Inicia sesión"
  },

  "forgotPassword": {
    "title": "Recuperar Contraseña",
    "subtitle": "Te enviaremos un enlace para restablecer tu contraseña",
    "email": "Correo electrónico",
    "emailPlaceholder": "tu@email.com",
    "submit": "Enviar Enlace",
    "submitting": "Enviando...",
    "success": "Si existe una cuenta con este email, recibirás un enlace de recuperación.",
    "backToLogin": "Volver a iniciar sesión"
  },

  "resetPassword": {
    "title": "Nueva Contraseña",
    "subtitle": "Ingresa tu nueva contraseña",
    "password": "Nueva contraseña",
    "passwordPlaceholder": "Mínimo 8 caracteres",
    "confirmPassword": "Confirmar contraseña",
    "confirmPasswordPlaceholder": "Repite tu nueva contraseña",
    "submit": "Cambiar Contraseña",
    "submitting": "Cambiando...",
    "success": "Tu contraseña ha sido actualizada. Ya puedes iniciar sesión.",
    "invalidToken": "El enlace ha expirado o no es válido."
  },

  "verifyEmail": {
    "title": "Verificar Email",
    "checking": "Verificando tu email...",
    "success": "¡Email verificado! Ya puedes acceder a tu cuenta.",
    "error": "No pudimos verificar tu email. El enlace puede haber expirado.",
    "resend": "Reenviar email de verificación"
  },

  "logout": {
    "title": "Cerrar Sesión",
    "message": "¿Estás seguro de que deseas cerrar sesión?",
    "confirm": "Sí, cerrar sesión",
    "cancel": "Cancelar"
  }
}
```

### 5.4 Errors Namespace (API Error Codes)

```json
// fe/public/locales/es/errors.json
{
  "api": {
    "INVALID_CREDENTIALS": "Email o contraseña incorrectos",
    "TOKEN_EXPIRED": "Tu sesión ha expirado. Por favor, inicia sesión nuevamente.",
    "INVALID_TOKEN": "Token de acceso inválido",
    "MISSING_AUTH": "Debes iniciar sesión para acceder",
    "FORBIDDEN": "No tienes permisos para realizar esta acción",
    "NOT_FOUND": "El recurso solicitado no existe",
    "CONFLICT": "Este recurso ya existe",
    "VALIDATION_ERROR": "Por favor, revisa los datos ingresados",
    "RATE_LIMITED": "Demasiadas solicitudes. Intenta de nuevo en unos minutos.",
    "SERVER_ERROR": "Error del servidor. Intenta de nuevo más tarde.",
    "NETWORK_ERROR": "Error de conexión. Verifica tu internet.",
    "TIMEOUT": "La solicitud tardó demasiado. Intenta de nuevo."
  },

  "resource": {
    "user": "usuario",
    "course": "curso",
    "lesson": "lección",
    "enrollment": "inscripción",
    "payment": "pago",
    "order": "orden"
  },

  "messages": {
    "notFound": "{{resource}} no encontrado",
    "alreadyExists": "Este {{resource}} ya existe",
    "cannotDelete": "No se puede eliminar este {{resource}}",
    "unauthorized": "No tienes acceso a este {{resource}}"
  },

  "generic": {
    "title": "¡Oops! Algo salió mal",
    "description": "Ha ocurrido un error inesperado. Por favor, intenta de nuevo.",
    "retry": "Intentar de nuevo",
    "goHome": "Ir al inicio",
    "contactSupport": "Contactar soporte"
  },

  "http": {
    "400": "Solicitud inválida",
    "401": "No autorizado",
    "403": "Acceso denegado",
    "404": "No encontrado",
    "409": "Conflicto",
    "422": "Datos inválidos",
    "429": "Demasiadas solicitudes",
    "500": "Error del servidor",
    "502": "Servicio no disponible",
    "503": "Servicio en mantenimiento"
  }
}
```

```json
// fe/public/locales/en/errors.json
{
  "api": {
    "INVALID_CREDENTIALS": "Invalid email or password",
    "TOKEN_EXPIRED": "Your session has expired. Please log in again.",
    "INVALID_TOKEN": "Invalid access token",
    "MISSING_AUTH": "You must log in to access this",
    "FORBIDDEN": "You don't have permission to perform this action",
    "NOT_FOUND": "The requested resource doesn't exist",
    "CONFLICT": "This resource already exists",
    "VALIDATION_ERROR": "Please check your input",
    "RATE_LIMITED": "Too many requests. Please try again in a few minutes.",
    "SERVER_ERROR": "Server error. Please try again later.",
    "NETWORK_ERROR": "Connection error. Please check your internet.",
    "TIMEOUT": "Request timed out. Please try again."
  },

  "resource": {
    "user": "user",
    "course": "course",
    "lesson": "lesson",
    "enrollment": "enrollment",
    "payment": "payment",
    "order": "order"
  },

  "messages": {
    "notFound": "{{resource}} not found",
    "alreadyExists": "This {{resource}} already exists",
    "cannotDelete": "Cannot delete this {{resource}}",
    "unauthorized": "You don't have access to this {{resource}}"
  },

  "generic": {
    "title": "Oops! Something went wrong",
    "description": "An unexpected error occurred. Please try again.",
    "retry": "Try again",
    "goHome": "Go to home",
    "contactSupport": "Contact support"
  },

  "http": {
    "400": "Bad request",
    "401": "Unauthorized",
    "403": "Access denied",
    "404": "Not found",
    "409": "Conflict",
    "422": "Invalid data",
    "429": "Too many requests",
    "500": "Server error",
    "502": "Service unavailable",
    "503": "Service under maintenance"
  }
}
```

### 5.5 Validation Namespace

```json
// fe/public/locales/es/validation.json
{
  "required": "Este campo es requerido",
  "email": "Ingresa un email válido",
  "minLength": "Debe tener al menos {{min}} caracteres",
  "maxLength": "Debe tener máximo {{max}} caracteres",
  "min": "El valor mínimo es {{min}}",
  "max": "El valor máximo es {{max}}",
  "pattern": "Formato inválido",
  "match": "Los campos no coinciden",
  "unique": "Este valor ya está en uso",

  "password": {
    "tooShort": "La contraseña debe tener al menos 8 caracteres",
    "tooWeak": "La contraseña es muy débil",
    "needsUppercase": "Debe incluir al menos una mayúscula",
    "needsLowercase": "Debe incluir al menos una minúscula",
    "needsNumber": "Debe incluir al menos un número",
    "needsSpecial": "Debe incluir al menos un carácter especial",
    "noMatch": "Las contraseñas no coinciden"
  },

  "file": {
    "tooLarge": "El archivo es muy grande (máximo {{max}})",
    "invalidType": "Tipo de archivo no permitido",
    "required": "Debes seleccionar un archivo"
  },

  "date": {
    "invalid": "Fecha inválida",
    "past": "La fecha debe ser en el pasado",
    "future": "La fecha debe ser en el futuro",
    "after": "Debe ser posterior a {{date}}",
    "before": "Debe ser anterior a {{date}}"
  }
}
```

---

## 6. Usage Patterns

### 6.1 Basic Hook Usage

```tsx
// Using useTranslation hook
import { useTranslation } from 'react-i18next';

function LoginForm() {
  // Load 'auth' namespace
  const { t } = useTranslation('auth');

  return (
    <form>
      <h1>{t('login.title')}</h1>
      <p>{t('login.subtitle')}</p>

      <label>{t('login.email')}</label>
      <input placeholder={t('login.emailPlaceholder')} />

      <button type="submit">{t('login.submit')}</button>
    </form>
  );
}
```

### 6.2 Multiple Namespaces

```tsx
import { useTranslation } from 'react-i18next';

function CourseCard({ course }) {
  // Load multiple namespaces
  const { t } = useTranslation(['courses', 'common']);

  return (
    <div>
      <h3>{course.title}</h3>
      <span>{t('courses:card.lessons', { count: course.lessonCount })}</span>
      <button>{t('common:actions.enroll')}</button>
    </div>
  );
}
```

### 6.3 Interpolation (Variables)

```tsx
function WelcomeMessage({ user }) {
  const { t } = useTranslation();

  return (
    <p>
      {/* "Hola, {{name}}! Tienes {{count}} notificaciones." */}
      {t('dashboard.welcome', {
        name: user.firstName,
        count: user.notifications,
      })}
    </p>
  );
}
```

### 6.4 Pluralization

```json
// In translation file
{
  "courses": {
    "enrolled": "Estás inscrito en {{count}} curso",
    "enrolled_plural": "Estás inscrito en {{count}} cursos",
    "enrolled_zero": "No estás inscrito en ningún curso"
  }
}
```

```tsx
function EnrollmentStatus({ count }) {
  const { t } = useTranslation('courses');

  // Automatically selects correct plural form
  return <p>{t('enrolled', { count })}</p>;
}
```

### 6.5 Trans Component (HTML in translations)

```json
{
  "register": {
    "terms": "Acepto los <termsLink>Términos</termsLink> y la <privacyLink>Privacidad</privacyLink>"
  }
}
```

```tsx
import { Trans } from 'react-i18next';
import { Link } from 'react-router-dom';

function TermsCheckbox() {
  return (
    <Trans
      i18nKey="auth:register.terms"
      components={{
        termsLink: <Link to="/terms" />,
        privacyLink: <Link to="/privacy" />,
      }}
    />
  );
}
```

### 6.6 Date and Number Formatting

```tsx
function CourseInfo({ course }) {
  const { t, i18n } = useTranslation();

  // Format date according to current language
  const formattedDate = new Intl.DateTimeFormat(i18n.language, {
    dateStyle: 'long',
  }).format(new Date(course.createdAt));

  // Format price with currency
  const formattedPrice = new Intl.NumberFormat(i18n.language, {
    style: 'currency',
    currency: course.currency,
  }).format(course.price);

  return (
    <div>
      <p>{t('courses:createdOn', { date: formattedDate })}</p>
      <p>
        {t('courses:price')}: {formattedPrice}
      </p>
    </div>
  );
}
```

### 6.7 Language Switcher Component

```tsx
// fe/src/components/common/LanguageSwitcher.tsx

import { useTranslation } from 'react-i18next';
import {
  SUPPORTED_LANGUAGES,
  LANGUAGE_NAMES,
  LANGUAGE_FLAGS,
  type SupportedLanguage,
} from '@/i18n/config';

/**
 * Language Switcher Component
 *
 * Displays current language and allows switching between supported languages.
 * Persists selection to localStorage and syncs with user profile if authenticated.
 */
export function LanguageSwitcher() {
  const { i18n } = useTranslation();

  const currentLanguage = i18n.language as SupportedLanguage;

  const handleChange = async (newLang: SupportedLanguage) => {
    // Change language (automatically persisted to localStorage)
    await i18n.changeLanguage(newLang);

    // Update HTML lang attribute for accessibility
    document.documentElement.lang = newLang;

    // If user is authenticated, also update their profile preference
    // This would be handled by a user context/hook
  };

  return (
    <div className="language-switcher">
      <button className="current-language">
        {LANGUAGE_FLAGS[currentLanguage]} {LANGUAGE_NAMES[currentLanguage]}
      </button>

      <ul className="language-options">
        {SUPPORTED_LANGUAGES.map((lang) => (
          <li key={lang}>
            <button
              onClick={() => handleChange(lang)}
              className={lang === currentLanguage ? 'active' : ''}
              aria-current={lang === currentLanguage ? 'true' : undefined}>
              {LANGUAGE_FLAGS[lang]} {LANGUAGE_NAMES[lang]}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### 6.8 Error Handling with i18n

```tsx
// fe/src/utils/errorHandler.ts

import i18n from '@/i18n';

interface ApiError {
  code: string;
  message?: string;
  details?: Record<string, unknown>;
}

/**
 * Translates an API error code to a user-friendly message
 */
export function getErrorMessage(error: ApiError): string {
  const { code, details } = error;

  // Try to find translation for this error code
  const translationKey = `errors:api.${code}`;

  if (i18n.exists(translationKey)) {
    return i18n.t(translationKey, details);
  }

  // Fallback to HTTP status if available
  if (details?.status) {
    const httpKey = `errors:http.${details.status}`;
    if (i18n.exists(httpKey)) {
      return i18n.t(httpKey);
    }
  }

  // Generic fallback
  return i18n.t('errors:generic.description');
}

/**
 * Hook for handling API errors with translations
 */
export function useErrorHandler() {
  const { t } = useTranslation('errors');

  const handleError = (error: ApiError) => {
    const message = getErrorMessage(error);

    // Show toast notification
    toast.error(message);

    // Log for debugging
    console.error('[API Error]', error);
  };

  return { handleError };
}
```

---

## 7. Best Practices

### 7.1 Key Naming Conventions

```
✅ Good:
- "auth.login.title"
- "courses.card.enrollButton"
- "errors.api.INVALID_CREDENTIALS"

❌ Avoid:
- "loginTitle" (no namespace structure)
- "auth.login.btn1" (meaningless identifier)
- "The login was successful!" (full sentences as keys)
```

### 7.2 Translation Guidelines

| Do ✅                                | Don't ❌                       |
| ------------------------------------ | ------------------------------ |
| Use context-specific keys            | Use generic keys everywhere    |
| Include pluralization                | Hardcode "1 item" vs "X items" |
| Use interpolation for variables      | Concatenate strings            |
| Keep translations close to component | Centralize all in one file     |
| Use `Trans` for HTML                 | Use `dangerouslySetInnerHTML`  |

### 7.3 TypeScript Integration

```typescript
// fe/src/i18n/types.ts

import 'i18next';
import common from '../../public/locales/es/common.json';
import auth from '../../public/locales/es/auth.json';
import errors from '../../public/locales/es/errors.json';

/**
 * Declare custom type for i18next resources
 *
 * This enables TypeScript autocompletion for translation keys
 */
declare module 'i18next' {
  interface CustomTypeOptions {
    defaultNS: 'common';
    resources: {
      common: typeof common;
      auth: typeof auth;
      errors: typeof errors;
    };
  }
}
```

### 7.4 Performance Tips

1. **Lazy load namespaces** - Only load what's needed for current route
2. **Preload critical namespaces** - `common` and `errors` on app start
3. **Use Suspense boundaries** - Show loading state while fetching translations
4. **Cache translations** - Browser caches JSON files by default
5. **Split by feature** - One namespace per major feature/route

```tsx
// Lazy loading namespace on route
function CoursesPage() {
  const { t, ready } = useTranslation('courses', { useSuspense: false });

  if (!ready) return <LoadingSpinner />;

  return <div>{t('title')}</div>;
}
```

---

## 8. Testing

### 8.1 Unit Testing with i18n

```tsx
// fe/src/test/i18n-test-utils.tsx

import { ReactNode } from 'react';
import { I18nextProvider } from 'react-i18next';
import i18n from 'i18next';

// Initialize i18n for tests with inline translations
i18n.init({
  lng: 'es',
  fallbackLng: 'es',
  ns: ['common', 'auth', 'errors'],
  defaultNS: 'common',
  resources: {
    es: {
      common: { actions: { save: 'Guardar' } },
      auth: { login: { title: 'Iniciar Sesión' } },
    },
  },
  interpolation: { escapeValue: false },
});

export function I18nTestProvider({ children }: { children: ReactNode }) {
  return <I18nextProvider i18n={i18n}>{children}</I18nextProvider>;
}

// Usage in tests
import { render, screen } from '@testing-library/react';
import { I18nTestProvider } from './i18n-test-utils';

test('shows translated login title', () => {
  render(
    <I18nTestProvider>
      <LoginForm />
    </I18nTestProvider>
  );

  expect(screen.getByText('Iniciar Sesión')).toBeInTheDocument();
});
```

### 8.2 Missing Translation Detection

```typescript
// In i18n config (development only)
saveMissing: true,
missingKeyHandler: (lngs, ns, key) => {
  // In CI, fail the build on missing translations
  if (process.env.CI) {
    throw new Error(`Missing translation: ${ns}:${key}`);
  }
  console.warn(`[i18n] Missing: ${ns}:${key}`);
};
```

---

## 9. Adding New Languages

### Step-by-step Guide

1. **Add language code to config**

```typescript
// fe/src/i18n/config.ts
export const SUPPORTED_LANGUAGES = ['es', 'en', 'pt', 'fr'] as const; // Added 'fr'

export const LANGUAGE_NAMES = {
  es: 'Español',
  en: 'English',
  pt: 'Português',
  fr: 'Français', // Added
};

export const LANGUAGE_FLAGS = {
  es: '🇪🇸',
  en: '🇺🇸',
  pt: '🇧🇷',
  fr: '🇫🇷', // Added
};
```

2. **Create translation files**

```bash
mkdir -p fe/public/locales/fr
cp fe/public/locales/en/*.json fe/public/locales/fr/
```

3. **Translate all keys**

4. **Test thoroughly**

```bash
# Run with new language
VITE_DEFAULT_LANG=fr pnpm dev
```

5. **Update TypeScript types**

```typescript
// Add French to type definitions
interface CustomTypeOptions {
  resources: {
    common: typeof commonFr;
    // ...
  };
}
```

---

## Related Documentation

- [Functional Requirements: RF-GLOBAL-006](../business/functional-requirements.md#rf-global-006-internacionalización-i18n)
- [Development Standards](./development-standards.md)
- [react-i18next Documentation](https://react.i18next.com/)
- [i18next Documentation](https://www.i18next.com/)

---

_Last updated: 2025-01-01_
