# ACC LMS — Requisitos de Cumplimiento Normativo y Protección de Datos

**Versión:** 2025-12-14  
**Estado:** Definición completa para implementación  
**Derivado de:** site_map/8-CUMPLIMIENTO_NORMATIVO_Y_PROTECCIÓN_DE_DATOS.md  
**Alcance:** Multi-jurisdicción (Colombia, UE, US California, Brasil)

---

## Convenciones

**Formato de identificación:** `RF-COMPLIANCE-<n>` para requisitos de cumplimiento normativo

**Normativas cubiertas:**

- 🇨🇴 **Colombia:** Ley 1581/2012 (Habeas Data), Decreto 1377/2013, Ley 1266/2008
- 🇪🇺 **Unión Europea:** GDPR (Reglamento 2016/679), ePrivacy Directive 2002/58/EC
- 🇺🇸 **Estados Unidos:** CCPA/CPRA (California), COPPA (Menores)
- 🇧🇷 **Brasil:** LGPD (Lei 13.709/2018)

**Prioridad implementación:**

- 🔴 **Crítico:** Obligatorio legal, bloquea lanzamiento
- 🟡 **Alto:** Requerido para mercados específicos
- 🟢 **Medio:** Best practice, mejora confianza

---

## 1. Documentación Legal Pública

### RF-COMPLIANCE-001: Términos y Condiciones 🔴

**Propósito:** Documento legal que define la relación contractual usuario-plataforma

**Ruta:** `/terminos-y-condiciones`

**Especificación técnica:**

```typescript
// Estructura de página términos
interface TermsPage {
  title: string; // "Términos y Condiciones de Uso"
  lastUpdated: Date; // Fecha última actualización
  version: string; // Número de versión (ej: "2.1")
  effectiveDate: Date; // Fecha entrada en vigor
  language: 'es' | 'en' | 'pt'; // Idioma actual
  availableLanguages: string[]; // Idiomas disponibles

  tableOfContents: Section[]; // Índice navegable
  content: MarkdownContent; // Contenido completo

  previousVersions: {
    // Historial versiones
    version: string;
    date: Date;
    url: string;
  }[];
}
```

**Secciones obligatorias:**

1. Aceptación de los términos
2. Descripción del servicio
3. Registro y cuenta de usuario
4. Uso aceptable de la plataforma
5. Contenido del usuario (licencias, responsabilidad)
6. Propiedad intelectual
7. Pagos y facturación
8. Política de reembolsos
9. Suspensión y terminación de cuenta
10. Limitación de responsabilidad
11. Indemnización
12. Ley aplicable y jurisdicción
13. Modificaciones a los términos
14. Contacto

**Features UI:**

- Índice navegable (TOC) con anchor links
- Secciones colapsables (accordion)
- Definiciones destacadas con tooltips
- Botón descargar PDF
- Botón imprimir
- Selector de idioma
- Link a versiones anteriores

---

### RF-COMPLIANCE-002: Política de Privacidad Integral 🔴

**Propósito:** Documento maestro de protección de datos cumpliendo múltiples normativas

**Ruta:** `/politica-de-privacidad`

**Especificación técnica:**

```typescript
interface PrivacyPolicy {
  // Metadata
  lastUpdated: Date;
  version: string;
  effectiveDate: Date;
  language: string;

  // Cumplimiento declarado
  compliance: {
    colombia: {
      ley1581_2012: boolean; // Habeas Data
      decreto1377_2013: boolean;
    };
    eu: {
      gdpr: boolean;
      eprivacy: boolean;
    };
    us: {
      ccpa: boolean;
      cpra: boolean;
      coppa: boolean;
    };
    brazil: {
      lgpd: boolean;
    };
  };

  // Responsable del tratamiento
  dataController: {
    companyName: string; // Razón social
    taxId: string; // NIT/CIF/EIN
    address: string; // Domicilio registrado
    dpoEmail: string; // privacy@acc-lms.com
    dpoPhone?: string; // Línea gratuita
  };
}
```

**Secciones obligatorias (GDPR Art. 13-14 + Habeas Data):**

1. **Responsable del tratamiento**

   - Identificación completa empresa
   - Oficial de Protección de Datos (DPO)
   - Canales de contacto privacidad

2. **Datos personales que recopilamos**

   - Datos de identificación (nombre, email, teléfono, documento)
   - Datos de ubicación (país, ciudad, IP, timezone)
   - Datos académicos/profesionales
   - Datos financieros (tokenizados, nunca tarjetas completas)
   - Datos técnicos (logs, cookies, device info)
   - Datos de comunicaciones (emails, chat, tickets)
   - Datos sensibles: **Declarar que NO se recopilan sin consentimiento explícito**

3. **Base legal del tratamiento**

   - Consentimiento (marketing, cookies no esenciales)
   - Ejecución de contrato (servicio, pagos, soporte)
   - Obligación legal (fiscal, prevención fraude)
   - Interés legítimo (seguridad, mejora servicio)

4. **Finalidades del tratamiento**

   - Lista exhaustiva de usos de datos

5. **Compartir datos con terceros**

   - Proveedores/procesadores con DPA
   - Transferencias internacionales (SCC, adecuación)
   - Lista de proveedores con ubicación
   - **Declaración: NO vendemos datos personales**

6. **Tiempo de conservación (retención)**

   - Tabla por tipo de dato con base legal

7. **Derechos del titular de datos**

   - Derechos ARCO (Colombia)
   - Derechos GDPR (art. 15-22)
   - Derechos CCPA
   - Derechos LGPD
   - Cómo ejercerlos
   - Autoridades de control

8. **Seguridad de los datos**

   - Medidas técnicas
   - Medidas organizativas
   - Notificación de brechas

9. **Cookies y tecnologías similares**

   - Referencia a política específica

10. **Menores de edad**

    - Edad mínima por jurisdicción
    - Consentimiento parental

11. **Decisiones automatizadas y perfilado**

    - Qué algoritmos usamos
    - Derecho a revisión humana

12. **Modificaciones a la política**

    - Proceso de notificación

13. **Marco legal aplicable**
    - Lista completa de normativas

---

### RF-COMPLIANCE-003: Política de Cookies 🔴

**Propósito:** Cumplimiento ePrivacy Directive y RGPD para cookies/tracking

**Ruta:** `/politica-cookies`

**Especificación técnica:**

```typescript
interface CookiePolicy {
  // Categorías de cookies
  categories: {
    essential: Cookie[]; // Necesarias, sin consentimiento
    functional: Cookie[]; // Preferencias, requiere consentimiento
    analytics: Cookie[]; // Google Analytics, Mixpanel, etc.
    marketing: Cookie[]; // Remarketing, ads
    socialMedia: Cookie[]; // Botones compartir
  };
}

interface Cookie {
  name: string; // Nombre técnico
  provider: string; // Quién la establece (1st/3rd party)
  purpose: string; // Para qué se usa
  duration: string; // Session / X días / X años
  type: 'http' | 'localStorage' | 'sessionStorage';
}
```

**Tabla de cookies documentadas:**

| Categoría | Nombre       | Proveedor | Propósito            | Duración |
| --------- | ------------ | --------- | -------------------- | -------- |
| Esencial  | `session_id` | ACC LMS   | Sesión autenticación | Session  |
| Esencial  | `csrf_token` | ACC LMS   | Protección CSRF      | Session  |
| Funcional | `locale`     | ACC LMS   | Idioma preferido     | 1 año    |
| Funcional | `theme`      | ACC LMS   | Tema claro/oscuro    | 1 año    |
| Analytics | `_ga`        | Google    | Analytics            | 2 años   |
| Analytics | `_gid`       | Google    | Analytics            | 24 horas |
| Marketing | `_fbp`       | Facebook  | Remarketing          | 90 días  |

---

### RF-COMPLIANCE-004: Banner de Consentimiento de Cookies 🔴

**Propósito:** Obtener consentimiento explícito antes de cookies no esenciales

**Componente:** `<CookieConsentBanner />`

**Especificación técnica:**

```typescript
interface CookieConsent {
  // Estado del consentimiento
  hasConsented: boolean;
  consentDate: Date;
  consentVersion: string;       // Versión política al consentir

  // Preferencias granulares
  preferences: {
    essential: true;            // Siempre true, no modificable
    functional: boolean;
    analytics: boolean;
    marketing: boolean;
    socialMedia: boolean;
  };

  // Tracking
  ipAddress?: string;           // Para audit trail
  userAgent?: string;
}

// Endpoint
POST /api/v1/consent/cookies
{
  "preferences": {
    "functional": true,
    "analytics": true,
    "marketing": false,
    "socialMedia": false
  }
}

// Response
{
  "consentId": "consent-uuid",
  "savedAt": "2025-12-14T10:30:00Z",
  "expiresAt": "2026-12-14T10:30:00Z"
}
```

**UI Requirements:**

- Banner visible en primera visita (no intrusivo)
- Botones: "Aceptar todo", "Rechazar no esenciales", "Configurar"
- Modal configuración con toggles por categoría
- Descripción clara de cada categoría
- Link a política completa
- Persistir preferencias 12 meses
- Re-mostrar si política actualizada

**Ruta configuración:** `/configuracion-cookies`

---

## 2. Derechos de los Titulares de Datos

### RF-COMPLIANCE-005: Portal de Privacidad del Usuario 🔴

**Propósito:** Permitir a usuarios ejercer derechos de privacidad de forma autoservicio

**Ruta:** `/mi-privacidad` (autenticado)

**Especificación técnica:**

```typescript
interface PrivacyPortal {
  // Secciones
  sections: {
    myData: DataSummary; // Resumen datos almacenados
    downloadData: DataExport; // Portabilidad
    deleteData: AccountDeletion; // Derecho al olvido
    consentManagement: Consents; // Gestionar consentimientos
    communicationPrefs: CommPrefs; // Preferencias comunicación
    requestHistory: Request[]; // Historial solicitudes
  };
}

interface DataSummary {
  // Mostrar qué datos tenemos
  profile: {
    name: string;
    email: string;
    phone?: string;
    country?: string;
    registeredAt: Date;
  };

  activity: {
    coursesEnrolled: number;
    coursesCompleted: number;
    lastLogin: Date;
    totalLogins: number;
  };

  financial: {
    totalPurchases: number;
    totalSpent: MoneyAmount;
  };

  communications: {
    emailsReceived: number;
    ticketsCreated: number;
    chatSessions: number;
  };
}
```

**Funcionalidades:**

1. **Ver mis datos** - Resumen de información almacenada
2. **Descargar mis datos** - Exportación completa (portabilidad)
3. **Eliminar mi cuenta** - Derecho al olvido con confirmación
4. **Gestionar consentimientos** - Cookies, marketing, etc.
5. **Preferencias de comunicación** - Email, push, SMS
6. **Historial de solicitudes** - Tracking de peticiones ARCO/GDPR

---

### RF-COMPLIANCE-006: Solicitudes ARCO (Colombia - Habeas Data) 🔴

**Propósito:** Cumplir Ley 1581/2012 para derechos ARCO en Colombia

**Rutas:**

- Portal: `/mi-privacidad/solicitud-arco`
- Formulario público: `/solicitud-datos`

**Especificación técnica:**

```typescript
// Tipos de solicitud ARCO
type ARCOType =
  | 'access'        // Acceso: Conocer qué datos tenemos
  | 'rectification' // Rectificación: Corregir datos inexactos
  | 'cancellation'  // Cancelación: Eliminar datos
  | 'opposition';   // Oposición: Oponerse a tratamiento

interface ARCORequest {
  // Identificación
  requestId: string;
  requestType: ARCOType;

  // Solicitante (verificación identidad)
  requester: {
    fullName: string;
    documentType: 'CC' | 'CE' | 'NIT' | 'passport';
    documentNumber: string;
    email: string;
    phone?: string;

    // Si actúa en representación
    isRepresentative: boolean;
    representedPerson?: {
      fullName: string;
      documentNumber: string;
      authorizationDocument?: string;  // URL documento poder
    };
  };

  // Detalles solicitud
  details: {
    dataCategories?: string[];         // Qué datos específicos
    reason?: string;                   // Motivo (obligatorio para oposición)
    specificRequest: string;           // Descripción detallada
    supportingDocuments?: string[];    // URLs documentos adjuntos
  };

  // Seguimiento
  status: ARCOStatus;
  createdAt: Date;
  acknowledgedAt?: Date;               // Acuse de recibo
  resolvedAt?: Date;
  responseDeadline: Date;              // 15 días hábiles

  // Respuesta
  response?: {
    decision: 'approved' | 'partial' | 'denied';
    explanation: string;
    actionsTaken?: string[];
    appealInfo?: string;               // Cómo apelar ante SIC
  };
}

type ARCOStatus =
  | 'received'           // Recibida
  | 'identity_pending'   // Pendiente verificación identidad
  | 'in_progress'        // En proceso
  | 'awaiting_info'      // Requiere información adicional
  | 'resolved'           // Resuelta
  | 'appealed';          // Apelada ante SIC

// Endpoint
POST /api/v1/privacy/arco-request
{
  "requestType": "access",
  "requester": {
    "fullName": "Juan Pérez",
    "documentType": "CC",
    "documentNumber": "123456789",
    "email": "juan@example.com"
  },
  "details": {
    "dataCategories": ["profile", "financial", "activity"],
    "specificRequest": "Solicito copia de todos mis datos personales..."
  }
}

// Response
{
  "requestId": "arco-2024-001234",
  "status": "received",
  "acknowledgedAt": "2025-12-14T10:30:00Z",
  "responseDeadline": "2026-01-06T23:59:59Z",  // 15 días hábiles
  "message": "Solicitud recibida. Recibirá respuesta antes del 6 de enero de 2026."
}
```

**Plazos legales (Decreto 1377/2013):**

- Acuse de recibo: Inmediato
- Respuesta: **15 días hábiles** máximo
- Prórroga: 8 días hábiles adicionales (notificando razón)

**Verificación de identidad:**

- Usuario autenticado: Automática
- Usuario no autenticado: Verificación documento + código email/SMS

---

### RF-COMPLIANCE-007: Derechos GDPR (Unión Europea) 🔴

**Propósito:** Cumplir artículos 15-22 del GDPR

**Ruta:** `/mi-privacidad/solicitud-gdpr`

**Especificación técnica:**

```typescript
type GDPRRight =
  | 'access'              // Art. 15 - Derecho de acceso
  | 'rectification'       // Art. 16 - Rectificación
  | 'erasure'             // Art. 17 - Supresión (olvido)
  | 'restriction'         // Art. 18 - Limitación tratamiento
  | 'portability'         // Art. 20 - Portabilidad
  | 'objection'           // Art. 21 - Oposición
  | 'automated_decision'; // Art. 22 - Decisiones automatizadas

interface GDPRRequest {
  requestId: string;
  rightType: GDPRRight;

  // Solicitante
  requester: {
    email: string;
    userId?: string;        // Si está autenticado
    euResident: boolean;    // Declaración residencia UE
    country?: string;       // País UE específico
  };

  // Detalles según tipo
  details: {
    // Para portabilidad
    exportFormat?: 'json' | 'csv' | 'xml';
    targetController?: string;  // Transferir a otro responsable

    // Para oposición
    grounds?: string;           // Motivo oposición

    // Para decisiones automatizadas
    specificDecision?: string;  // Qué decisión impugna
  };

  status: GDPRStatus;
  createdAt: Date;
  responseDeadline: Date;       // 30 días

  response?: {
    decision: 'fulfilled' | 'partial' | 'denied';
    explanation: string;
    exportUrl?: string;         // Para portabilidad
    validUntil?: Date;          // Expiración download link
  };
}

// Endpoint
POST /api/v1/privacy/gdpr-request
{
  "rightType": "portability",
  "requester": {
    "email": "user@example.com",
    "euResident": true,
    "country": "ES"
  },
  "details": {
    "exportFormat": "json"
  }
}

// Response
{
  "requestId": "gdpr-2024-005678",
  "status": "in_progress",
  "responseDeadline": "2026-01-13T23:59:59Z",  // 30 días
  "message": "Su solicitud está siendo procesada..."
}
```

**Plazos legales (GDPR):**

- Respuesta: **30 días** máximo (1 mes)
- Prórroga: 2 meses adicionales para solicitudes complejas (notificando en primeros 30 días)
- Notificación brecha: **72 horas** a autoridad control

**Formatos exportación portabilidad:**

- JSON estructurado (recomendado)
- CSV para datos tabulares
- XML como alternativa

---

### RF-COMPLIANCE-008: Derechos CCPA/CPRA (California) 🟡

**Propósito:** Cumplir California Consumer Privacy Act y Privacy Rights Act

**Rutas:**

- Opt-out: `/no-vender-mi-informacion`
- Solicitudes: `/mi-privacidad/solicitud-ccpa`

**Especificación técnica:**

```typescript
type CCPARight =
  | 'know'              // Derecho a saber qué datos recopilamos
  | 'delete'            // Derecho a eliminar
  | 'opt_out_sale'      // Opt-out de "venta" de datos
  | 'opt_out_sharing'   // Opt-out de compartir (CPRA)
  | 'correct'           // Derecho a corregir (CPRA)
  | 'limit_sensitive';  // Limitar uso datos sensibles (CPRA)

interface CCPARequest {
  requestId: string;
  rightType: CCPARight;

  requester: {
    email: string;
    californiaResident: boolean;  // Declaración residencia CA
    verificationMethod: 'email' | 'phone' | 'document';
  };

  // Para opt-out, no requiere verificación completa
  isOptOut: boolean;

  status: CCPAStatus;
  createdAt: Date;
  responseDeadline: Date;   // 45 días
}

// Endpoint opt-out (simplificado, sin verificación completa)
POST /api/v1/privacy/ccpa-opt-out
{
  "email": "user@example.com",
  "optOutType": "sale"  // o "sharing"
}

// Response
{
  "success": true,
  "optOutId": "ccpa-opt-2024-001",
  "effectiveDate": "2025-12-14T00:00:00Z",
  "message": "Your opt-out request has been processed."
}
```

**Requisitos específicos CCPA:**

- Link "Do Not Sell My Personal Information" en footer
- No discriminación por ejercer derechos
- Plazo respuesta: **45 días** (extensible 45 días más)
- Verificación identidad razonable

**Página /no-vender-mi-informacion:**

- Explicación de qué significa "venta" bajo CCPA
- Formulario simple para opt-out
- No requiere crear cuenta
- Confirmación inmediata

---

### RF-COMPLIANCE-009: Derechos LGPD (Brasil) 🟡

**Propósito:** Cumplir Lei Geral de Proteção de Dados brasileña

**Ruta:** `/mi-privacidad/solicitud-lgpd`

**Especificación técnica:**

```typescript
type LGPDRight =
  | 'confirmation' // Confirmación existencia tratamiento
  | 'access' // Acceso a los datos
  | 'correction' // Corrección datos incompletos
  | 'anonymization' // Anonimización/bloqueo/eliminación
  | 'portability' // Portabilidad
  | 'deletion' // Eliminación
  | 'sharing_info' // Info sobre compartir datos
  | 'consent_info' // Info sobre posibilidad negar consentimiento
  | 'revoke_consent'; // Revocación consentimiento

interface LGPDRequest {
  requestId: string;
  rightType: LGPDRight;

  requester: {
    cpf?: string; // CPF brasileño (opcional)
    email: string;
    brazilResident: boolean;
  };

  status: string;
  createdAt: Date;
  responseDeadline: Date; // 15 días
}
```

**Plazos LGPD:**

- Respuesta simplificada: **Inmediata**
- Respuesta completa: **15 días**

---

## 3. Exportación y Eliminación de Datos

### RF-COMPLIANCE-010: Exportación de Datos (Portabilidad) 🔴

**Propósito:** Permitir descarga de datos personales en formato estructurado

**Endpoint:** `GET /api/v1/privacy/export-data`

**Especificación técnica:**

```typescript
interface DataExportRequest {
  userId: string;
  format: 'json' | 'csv' | 'zip'; // ZIP contiene ambos
  scope: 'full' | 'partial';
  categories?: DataCategory[];

  // Seguridad
  verificationCode: string; // 2FA o código email
  requestIp: string;
  requestUserAgent: string;
}

type DataCategory =
  | 'profile' // Datos perfil
  | 'preferences' // Preferencias y configuración
  | 'enrollments' // Matrículas y progreso
  | 'certificates' // Certificados obtenidos
  | 'purchases' // Historial compras
  | 'communications' // Emails, mensajes, tickets
  | 'activity_logs' // Historial actividad
  | 'content_created'; // Contenido creado (instructores)

interface DataExport {
  exportId: string;
  userId: string;
  requestedAt: Date;
  status: 'processing' | 'ready' | 'expired' | 'failed';

  // Cuando esté listo
  downloadUrl?: string;
  expiresAt?: Date; // 24-48 horas típico
  fileSizeBytes?: number;
  checksum?: string; // SHA-256 para verificar integridad

  // Contenido
  includedCategories: DataCategory[];
  recordCounts: Record<DataCategory, number>;
}

// Estructura del archivo exportado
interface ExportedUserData {
  exportMetadata: {
    exportedAt: string;
    userId: string;
    requestedBy: string;
    format: string;
    version: string;
  };

  profile: {
    userId: string;
    email: string;
    firstName: string;
    lastName: string;
    phone?: string;
    bio?: string;
    avatarUrl?: string;
    createdAt: string;
    lastLoginAt: string;
    // ... todos los campos perfil
  };

  preferences: {
    language: string;
    timezone: string;
    theme: string;
    emailNotifications: boolean;
    marketingOptIn: boolean;
    // ... todas las preferencias
  };

  enrollments: Array<{
    courseId: string;
    courseName: string;
    enrolledAt: string;
    progressPercent: number;
    completedAt?: string;
    lastAccessedAt: string;
  }>;

  certificates: Array<{
    certificateId: string;
    courseName: string;
    issuedAt: string;
    verificationCode: string;
  }>;

  purchases: Array<{
    orderId: string;
    date: string;
    items: string[];
    amount: string;
    currency: string;
    paymentMethod: string;
  }>;

  // ... otras categorías
}
```

**Proceso:**

1. Usuario solicita exportación (requiere 2FA/verificación)
2. Sistema genera archivo en background (puede tomar minutos)
3. Notificación email cuando listo
4. Link descarga con token firmado, expira en 24-48h
5. Log de auditoría de la exportación

---

### RF-COMPLIANCE-011: Eliminación de Cuenta (Derecho al Olvido) 🔴

**Propósito:** Permitir eliminación completa de cuenta cumpliendo GDPR Art. 17

**Ruta:** `/mi-privacidad/eliminar-cuenta`

**Endpoint:** `DELETE /api/v1/privacy/delete-account`

**Especificación técnica:**

```typescript
interface AccountDeletionRequest {
  userId: string;

  // Verificación múltiple
  verification: {
    password: string; // Confirmar contraseña
    twoFactorCode?: string; // Si tiene 2FA
    confirmationPhrase: string; // "ELIMINAR MI CUENTA"
  };

  // Opciones
  options: {
    reason?: DeletionReason;
    feedback?: string; // Feedback opcional
    downloadDataFirst: boolean; // Ofrecemos exportar antes
  };

  // Consentimientos
  acknowledgements: {
    understandPermanent: boolean; // Entiende que es permanente
    understandContentRemoved: boolean; // Contenido creado se elimina
    understandNoRefund: boolean; // Sin reembolso por cursos
  };
}

type DeletionReason =
  | 'privacy_concerns'
  | 'no_longer_needed'
  | 'found_alternative'
  | 'too_expensive'
  | 'poor_experience'
  | 'other';

interface AccountDeletionResponse {
  deletionId: string;
  status: 'scheduled' | 'processing' | 'completed' | 'cancelled';

  // Período de gracia (30 días para cancelar)
  gracePeriod: {
    startDate: Date;
    endDate: Date; // Fecha eliminación real
    canCancel: boolean;
    cancelUrl: string;
  };

  // Qué se eliminará
  dataToDelete: {
    profile: boolean;
    enrollments: boolean;
    certificates: boolean;
    purchases: boolean; // Registros, no facturas (obligación legal)
    communications: boolean;
    activityLogs: boolean;
  };

  // Qué se retiene (obligación legal)
  dataRetained: {
    invoices: {
      reason: string; // "Obligación fiscal - 10 años"
      retentionPeriod: string;
    };
    fraudPrevention: {
      reason: string; // "Prevención fraude"
      retentionPeriod: string;
      dataType: string; // "Hash email, IP registro"
    };
  };
}
```

**Proceso de eliminación:**

1. **Solicitud:** Usuario completa formulario con verificaciones
2. **Confirmación:** Email de confirmación con link cancelar
3. **Período de gracia:** 30 días para reconsiderar
4. **Recordatorios:** Email a los 7 y 1 día antes
5. **Ejecución:** Eliminación automática tras período gracia
6. **Notificación:** Email confirmando eliminación completada
7. **Auditoría:** Log de eliminación (sin datos personales)

**Datos que NO se eliminan (excepciones legales):**

- Facturas/registros fiscales (10 años - ley tributaria)
- Hash de email para prevención fraude
- Logs anonimizados agregados
- Contenido público ya compartido por otros (reviews anonimizadas)

**Para instructores:**

- Cursos transferidos a cuenta plataforma o eliminados
- Estudiantes notificados
- Pagos pendientes procesados antes de eliminar

---

## 4. Gestión de Consentimientos

### RF-COMPLIANCE-012: Registro de Consentimientos 🔴

**Propósito:** Mantener audit trail de todos los consentimientos (GDPR Art. 7)

**Especificación técnica:**

```typescript
interface ConsentRecord {
  consentId: string;
  userId: string;

  // Tipo de consentimiento
  consentType: ConsentType;

  // Estado
  granted: boolean;
  grantedAt?: Date;
  revokedAt?: Date;

  // Contexto del consentimiento
  context: {
    ipAddress: string;
    userAgent: string;
    pageUrl: string;            // Dónde se otorgó
    policyVersion: string;      // Versión de política aceptada
    method: 'checkbox' | 'banner' | 'form' | 'api';
  };

  // Para auditoría
  proofOfConsent?: {
    timestamp: Date;
    checkboxText: string;       // Texto exacto mostrado
    screenshot?: string;        // Opcional, URL screenshot
  };
}

type ConsentType =
  | 'terms_of_service'          // Términos uso (obligatorio)
  | 'privacy_policy'            // Política privacidad (obligatorio)
  | 'cookies_essential'         // Cookies esenciales (obligatorio)
  | 'cookies_functional'        // Cookies funcionales
  | 'cookies_analytics'         // Cookies analíticas
  | 'cookies_marketing'         // Cookies marketing
  | 'email_marketing'           // Email promocional
  | 'email_newsletter'          // Newsletter
  | 'sms_notifications'         // SMS
  | 'push_notifications'        // Push browser/app
  | 'data_sharing_analytics'    // Compartir datos analytics
  | 'profiling'                 // Perfilado para recomendaciones
  | 'third_party_marketing';    // Marketing terceros

// Endpoint para consultar/modificar consentimientos
GET /api/v1/consent/status
{
  "userId": "user-123",
  "consents": {
    "terms_of_service": {
      "granted": true,
      "grantedAt": "2025-01-15T10:00:00Z",
      "policyVersion": "2.1"
    },
    "email_marketing": {
      "granted": false,
      "revokedAt": "2025-06-20T15:30:00Z"
    },
    // ...
  }
}

PATCH /api/v1/consent/update
{
  "consentType": "email_marketing",
  "granted": false
}
```

**Requisitos:**

- Consentimiento debe ser: libre, específico, informado, inequívoco
- Tan fácil revocar como otorgar
- Separar consentimientos (no bundle obligatorio)
- Registrar CUÁNDO, CÓMO y QUÉ se consintió

---

### RF-COMPLIANCE-013: Gestión de Preferencias de Comunicación 🔴

**Propósito:** Control granular sobre comunicaciones recibidas

**Ruta:** `/mi-privacidad/preferencias-comunicacion`

**Especificación técnica:**

```typescript
interface CommunicationPreferences {
  userId: string;

  // Por canal
  email: {
    transactional: boolean;     // Siempre true (compras, seguridad)
    courseUpdates: boolean;     // Actualizaciones cursos inscritos
    instructorMessages: boolean;// Mensajes de instructores
    marketing: boolean;         // Promociones plataforma
    newsletter: boolean;        // Newsletter semanal/mensual
    partnerOffers: boolean;     // Ofertas de partners
  };

  push: {
    enabled: boolean;
    courseReminders: boolean;
    newContent: boolean;
    promotions: boolean;
  };

  sms: {
    enabled: boolean;
    securityAlerts: boolean;    // 2FA, login sospechoso
    orderConfirmations: boolean;
  };

  inApp: {
    courseProgress: boolean;
    achievements: boolean;
    recommendations: boolean;
    systemNotices: boolean;
  };

  // Frecuencia
  digestFrequency: 'realtime' | 'daily' | 'weekly' | 'never';

  // Horarios (no molestar)
  quietHours?: {
    enabled: boolean;
    startTime: string;  // "22:00"
    endTime: string;    // "08:00"
    timezone: string;
  };
}

// Endpoint
PATCH /api/v1/users/:id/communication-preferences
```

**Unsubscribe links:**

- Cada email marketing incluye link unsubscribe con 1 click
- Link lleva a página para gestionar todas las preferencias
- No requiere login para unsubscribe básico

---

## 5. Menores de Edad

### RF-COMPLIANCE-014: Verificación de Edad y Consentimiento Parental 🔴

**Propósito:** Cumplir COPPA (US), GDPR Art. 8, Ley 1098 (Colombia)

**Especificación técnica:**

```typescript
interface AgeVerification {
  // En registro
  dateOfBirth: Date;
  ageAtRegistration: number;

  // Verificación por jurisdicción
  jurisdiction: {
    country: string;
    minimumAge: number; // Calculado según país
    requiresParentalConsent: boolean;
  };

  // Si es menor
  parentalConsent?: {
    required: boolean;
    obtained: boolean;
    obtainedAt?: Date;
    parentEmail?: string;
    parentName?: string;
    verificationMethod: 'email' | 'document' | 'credit_card';
    expiresAt?: Date; // Renovar al cumplir edad
  };
}

// Edades mínimas por jurisdicción
const MINIMUM_AGES = {
  // COPPA (US)
  US: 13,

  // GDPR (varía por país UE)
  ES: 14, // España
  DE: 16, // Alemania
  FR: 15, // Francia
  NL: 16, // Países Bajos
  IE: 16, // Irlanda
  default_EU: 16,

  // Colombia
  CO: 18, // Ley 1098, menores requieren autorización tutor

  // Brasil
  BR: 18, // LGPD, menores requieren consentimiento parental

  // Default
  default: 13,
};
```

**Flujo para menores:**

1. **Detección:** Fecha nacimiento en registro indica menor de edad
2. **Bloqueo parcial:** Cuenta creada pero limitada
3. **Solicitud:** Email automático a padre/tutor
4. **Verificación:** Padre confirma vía link seguro + verificación identidad
5. **Activación:** Cuenta completa una vez verificado
6. **Monitoreo:** Notificaciones a padre sobre actividad

**Restricciones para menores:**

- No pueden realizar compras directamente
- No pueden participar en foros públicos sin supervisión
- Datos adicionales NO recopilados (ubicación precisa, etc.)
- Marketing deshabilitado por defecto

**Contacto padres:** `parents@acc-lms.com`

---

## 6. Seguridad de Datos

### RF-COMPLIANCE-015: Notificación de Brechas de Seguridad 🔴

**Propósito:** Cumplir obligaciones de notificación en caso de brecha

**Especificación técnica:**

```typescript
interface DataBreach {
  breachId: string;

  // Detección
  detectedAt: Date;
  detectedBy: 'automated' | 'employee' | 'external' | 'user_report';

  // Clasificación
  severity: 'critical' | 'high' | 'medium' | 'low';
  affectedDataTypes: DataType[];
  estimatedAffectedUsers: number;

  // Evaluación de riesgo
  riskAssessment: {
    likelihoodOfHarm: 'high' | 'medium' | 'low';
    potentialImpact: string;
    mitigatingFactors: string[];
  };

  // Notificaciones
  notifications: {
    // Autoridades
    authorityNotified: boolean;
    authorityNotifiedAt?: Date;
    authority: string; // "SIC Colombia", "AEPD España", etc.
    referenceNumber?: string;

    // Usuarios afectados
    usersNotified: boolean;
    usersNotifiedAt?: Date;
    notificationMethod: string;

    // Interno
    ceoNotified: boolean;
    legalNotified: boolean;
    dpoNotified: boolean;
  };

  // Acciones tomadas
  remediation: {
    immediatActions: string[];
    longTermActions: string[];
    preventiveMeasures: string[];
  };

  // Timeline
  timeline: {
    detected: Date;
    contained: Date;
    investigated: Date;
    authorityNotified?: Date; // Dentro de 72h (GDPR)
    usersNotified?: Date;
    resolved?: Date;
  };
}

type DataType =
  | 'email'
  | 'password_hash'
  | 'name'
  | 'phone'
  | 'address'
  | 'payment_info'
  | 'document_id'
  | 'health_data'
  | 'biometric';
```

**Plazos de notificación:**

| Jurisdicción           | Autoridad                   | Usuarios                    |
| ---------------------- | --------------------------- | --------------------------- |
| GDPR (UE)              | 72 horas                    | Sin demora indebida         |
| Colombia (Habeas Data) | Tan pronto como sea posible | Tan pronto como sea posible |
| CCPA (California)      | -                           | Más pronto posible          |
| LGPD (Brasil)          | Plazo razonable             | Plazo razonable             |

**Contenido notificación a usuarios:**

- Descripción de la brecha
- Tipos de datos afectados
- Posibles consecuencias
- Medidas tomadas
- Recomendaciones al usuario
- Contacto para más información

---

### RF-COMPLIANCE-016: Medidas Técnicas de Seguridad 🔴

**Propósito:** Documentar medidas de seguridad para cumplimiento y auditoría

**Medidas implementadas:**

```typescript
interface SecurityMeasures {
  // Encriptación
  encryption: {
    inTransit: {
      protocol: 'TLS 1.3';
      certificateAuthority: string;
      hsts: boolean;
    };
    atRest: {
      algorithm: 'AES-256-GCM';
      keyManagement: 'AWS KMS' | 'HashiCorp Vault';
      encryptedFields: string[]; // emails, documents, etc.
    };
    passwords: {
      algorithm: 'Argon2id';
      parameters: {
        memory: 65536; // 64 MB
        iterations: 3;
        parallelism: 4;
      };
    };
    paymentData: {
      tokenization: true;
      provider: 'Stripe' | 'other';
      pciCompliance: boolean;
    };
  };

  // Control de acceso
  accessControl: {
    authentication: {
      mfa: boolean;
      mfaMethods: ('totp' | 'sms' | 'email')[];
      passwordPolicy: PasswordPolicy;
      sessionTimeout: number; // minutos
      maxConcurrentSessions: number;
    };
    authorization: {
      model: 'RBAC';
      principleOfLeastPrivilege: boolean;
      regularAccessReview: boolean;
    };
  };

  // Infraestructura
  infrastructure: {
    firewall: boolean;
    waf: boolean; // Web Application Firewall
    ddosProtection: boolean;
    intrusionDetection: boolean;
    vulnerabilityScanning: {
      frequency: 'weekly';
      automated: boolean;
    };
  };

  // Organizativas
  organizational: {
    employeeTraining: boolean;
    backgroundChecks: boolean;
    ndaRequired: boolean;
    accessLogging: boolean;
    incidentResponsePlan: boolean;
    businessContinuityPlan: boolean;
  };

  // Certificaciones
  certifications: {
    iso27001: boolean | 'in_progress';
    soc2: boolean | 'planned';
    pciDss: boolean; // Si procesamos tarjetas
  };
}
```

---

## 7. Transferencias Internacionales

### RF-COMPLIANCE-017: Transferencias de Datos Fuera de Jurisdicción 🟡

**Propósito:** Documentar y legitimar transferencias internacionales de datos

**Especificación técnica:**

```typescript
interface InternationalTransfer {
  // Proveedor/Destinatario
  recipient: {
    name: string; // Ej: "Amazon Web Services"
    purpose: string; // Ej: "Cloud hosting"
    dataTypes: DataType[]; // Qué datos recibe
    country: string; // País destino
    region?: string; // Región específica
  };

  // Base legal de transferencia
  legalBasis: {
    // UE
    gdpr: {
      mechanism:
        | 'adequacy_decision' // Decisión adecuación (ej: Japón, UK)
        | 'standard_contractual_clauses' // SCCs
        | 'binding_corporate_rules' // BCRs
        | 'explicit_consent' // Consentimiento explícito
        | 'contract_performance'; // Necesario para contrato

      sccVersion?: string; // Versión SCCs si aplica
      sccSignedDate?: Date;
    };

    // Colombia
    habeasData: {
      mechanism:
        | 'country_adequate_protection' // País con protección adecuada
        | 'contract' // Contrato con cláusulas
        | 'authorization'; // Autorización titular
    };
  };

  // DPA (Data Processing Agreement)
  dpa: {
    signed: boolean;
    signedDate?: Date;
    expirationDate?: Date;
    documentUrl?: string;
  };
}

// Lista de proveedores con transferencias
const DATA_PROCESSORS: InternationalTransfer[] = [
  {
    recipient: {
      name: 'Amazon Web Services (AWS)',
      purpose: 'Cloud infrastructure hosting',
      dataTypes: ['all'],
      country: 'US',
      region: 'us-east-1',
    },
    legalBasis: {
      gdpr: {
        mechanism: 'standard_contractual_clauses',
        sccVersion: '2021/914',
        sccSignedDate: new Date('2024-01-15'),
      },
    },
    dpa: { signed: true },
  },
  {
    recipient: {
      name: 'Stripe, Inc.',
      purpose: 'Payment processing',
      dataTypes: ['email', 'name', 'payment_info'],
      country: 'US',
    },
    legalBasis: {
      gdpr: {
        mechanism: 'standard_contractual_clauses',
      },
    },
    dpa: { signed: true },
  },
  // ... otros proveedores
];
```

**Proveedores típicos a documentar:**

- Cloud: AWS, GCP, Azure
- Pagos: Stripe, PayPal, MercadoPago
- Email: SendGrid, Mailgun
- Analytics: Google Analytics, Mixpanel
- CDN: Cloudflare
- Soporte: Zendesk, Intercom
- Video: Mux, AWS MediaConvert

---

## 8. Auditoría y Cumplimiento

### RF-COMPLIANCE-018: Registros de Auditoría de Datos Personales 🔴

**Propósito:** Mantener logs de acceso a datos personales para accountability

**Especificación técnica:**

```typescript
interface PersonalDataAccessLog {
  logId: string;
  timestamp: Date;

  // Quién accedió
  accessor: {
    userId: string;
    role: string;
    department?: string;
    ipAddress: string;
    userAgent: string;
  };

  // Qué datos
  dataAccessed: {
    dataSubjectId: string; // Usuario cuyos datos se accedieron
    dataCategories: DataCategory[];
    specificFields?: string[]; // Campos específicos
    recordCount: number;
  };

  // Contexto
  context: {
    operation: 'view' | 'export' | 'modify' | 'delete';
    purpose: string; // Razón del acceso
    legalBasis: string; // Base legal
    ticketId?: string; // Si es por ticket soporte
    approved?: boolean; // Si requirió aprobación
    approvedBy?: string;
  };

  // Para auditoría
  retentionPeriod: string; // Cuánto tiempo guardar este log
}

// Configuración de qué loguear
const AUDIT_CONFIG = {
  // Siempre loguear acceso a:
  sensitiveFields: [
    'email',
    'phone',
    'address',
    'document_id',
    'payment_info',
    'password_hash',
  ],

  // Roles que generan logs al acceder
  auditedRoles: ['admin', 'support', 'finance'],

  // Operaciones que siempre se loguean
  auditedOperations: ['export', 'delete', 'bulk_access'],

  // Retención logs
  retentionDays: 365 * 2, // 2 años
};
```

---

### RF-COMPLIANCE-019: Evaluación de Impacto en Privacidad (DPIA) 🟡

**Propósito:** Documentar DPIAs para tratamientos de alto riesgo (GDPR Art. 35)

**Tratamientos que requieren DPIA:**

1. **Perfilado para recomendaciones de cursos**
   - Uso de ML para recomendar cursos
   - Análisis de comportamiento de aprendizaje
2. **Detección de fraude automatizada**

   - Análisis de patrones de pago
   - Bloqueo automático de cuentas sospechosas

3. **Analytics de comportamiento**
   - Tracking de navegación
   - Heatmaps de video

**Template DPIA:**

```typescript
interface DPIA {
  dpiaId: string;
  processingActivity: string;
  createdDate: Date;
  lastReviewDate: Date;
  dpoApproval: boolean;

  // Descripción
  description: {
    purpose: string;
    dataTypes: string[];
    dataSubjects: string[];
    recipients: string[];
    retentionPeriod: string;
  };

  // Necesidad y proporcionalidad
  necessity: {
    legalBasis: string;
    isNecessary: boolean;
    isProportionate: boolean;
    alternatives: string[];
  };

  // Riesgos identificados
  risks: Array<{
    description: string;
    likelihood: 'high' | 'medium' | 'low';
    severity: 'high' | 'medium' | 'low';
    riskScore: number;
    mitigationMeasures: string[];
    residualRisk: 'high' | 'medium' | 'low';
  }>;

  // Conclusión
  conclusion: {
    approved: boolean;
    conditions?: string[];
    reviewSchedule: string;
  };
}
```

---

## Endpoints Consolidados - Compliance Service

```yaml
# Privacy Portal
GET  /api/v1/privacy/my-data           # Resumen datos del usuario
GET  /api/v1/privacy/export-data       # Iniciar exportación datos
GET  /api/v1/privacy/export-data/:id   # Estado/descarga exportación
DELETE /api/v1/privacy/delete-account  # Solicitar eliminación cuenta

# Solicitudes de derechos
POST /api/v1/privacy/arco-request      # Solicitud ARCO (Colombia)
POST /api/v1/privacy/gdpr-request      # Solicitud GDPR (UE)
POST /api/v1/privacy/ccpa-opt-out      # Opt-out CCPA (California)
POST /api/v1/privacy/lgpd-request      # Solicitud LGPD (Brasil)
GET  /api/v1/privacy/requests          # Historial solicitudes del usuario
GET  /api/v1/privacy/requests/:id      # Detalle solicitud

# Consentimientos
GET  /api/v1/consent/status            # Estado todos los consentimientos
PATCH /api/v1/consent/update           # Actualizar consentimiento específico
POST /api/v1/consent/cookies           # Guardar preferencias cookies
GET  /api/v1/consent/history           # Historial cambios consentimientos

# Comunicaciones
GET  /api/v1/users/:id/communication-preferences
PATCH /api/v1/users/:id/communication-preferences
POST /api/v1/unsubscribe/:token        # Unsubscribe sin login

# Admin - Gestión solicitudes
GET  /api/v1/admin/privacy/requests    # Lista solicitudes pendientes
GET  /api/v1/admin/privacy/requests/:id
PATCH /api/v1/admin/privacy/requests/:id  # Actualizar estado/respuesta
GET  /api/v1/admin/privacy/audit-logs  # Logs acceso datos personales
GET  /api/v1/admin/privacy/breaches    # Registro brechas
POST /api/v1/admin/privacy/breaches    # Reportar brecha
```

---

## Trazabilidad RF ↔ Normativa

| RF                | GDPR       | Habeas Data      | CCPA          | LGPD          |
| ----------------- | ---------- | ---------------- | ------------- | ------------- |
| RF-COMPLIANCE-001 | ✓          | ✓                | ✓             | ✓             |
| RF-COMPLIANCE-002 | Art. 13-14 | Art. 12 Ley 1581 | §1798.100     | Art. 9        |
| RF-COMPLIANCE-003 | ePrivacy   | -                | -             | -             |
| RF-COMPLIANCE-004 | Art. 7     | -                | -             | Art. 8        |
| RF-COMPLIANCE-005 | Art. 15-22 | Art. 14-16       | §1798.100-125 | Art. 18       |
| RF-COMPLIANCE-006 | -          | Art. 14-16       | -             | -             |
| RF-COMPLIANCE-007 | Art. 15-22 | -                | -             | -             |
| RF-COMPLIANCE-008 | -          | -                | §1798.120     | -             |
| RF-COMPLIANCE-009 | -          | -                | -             | Art. 18       |
| RF-COMPLIANCE-010 | Art. 20    | -                | §1798.100     | Art. 18(V)    |
| RF-COMPLIANCE-011 | Art. 17    | Art. 15          | §1798.105     | Art. 18(VI)   |
| RF-COMPLIANCE-012 | Art. 7     | -                | -             | Art. 8        |
| RF-COMPLIANCE-013 | Art. 21    | -                | -             | Art. 18(VIII) |
| RF-COMPLIANCE-014 | Art. 8     | Ley 1098         | COPPA         | Art. 14       |
| RF-COMPLIANCE-015 | Art. 33-34 | SIC              | §1798.150     | Art. 48       |
| RF-COMPLIANCE-016 | Art. 32    | Art. 17 Ley 1581 | -             | Art. 46       |
| RF-COMPLIANCE-017 | Cap. V     | -                | -             | Art. 33       |
| RF-COMPLIANCE-018 | Art. 30    | -                | -             | Art. 37       |
| RF-COMPLIANCE-019 | Art. 35    | -                | -             | -             |

---

**Total RFs Compliance:** 19  
**Endpoints:** 18  
**Normativas cubiertas:** 4 jurisdicciones principales

**Estado:** ✅ **LISTO PARA IMPLEMENTACIÓN**
