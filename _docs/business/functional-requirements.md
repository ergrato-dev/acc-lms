# ACC LMS — Requisitos Funcionales (RF)

**Versión:** 2025-08-08  
**Estado:** MVP definitivo - Listo para implementación  
**Alcance:** Multi-stack microservices + AI-enhanced learning

---

## Convenciones del Sistema

**Nomenclatura técnica:** Todos los identificadores (endpoints, servicios, campos JSON, tablas, variables) en **inglés**. Documentación en **español**.

**Formato de identificación:** `RF-<DOMINIO>-<n>` donde:

- `DOMINIO`: AUTH, USERS, COURSES, ENROLL, CONTENT, ASSIGN, GRADES, PAYMENTS, NOTIF, ANALYTICS, SEARCH, AI, FE, FLOW
- `n`: número secuencial

**Arquitectura base:** Clean Architecture + Domain Events + CQRS selectivo + Observabilidad completa

---

## Actores del Sistema

| Rol            | Descripción                    | Permisos base                                  |
| -------------- | ------------------------------ | ---------------------------------------------- |
| **Anonymous**  | Visitante sin autenticación    | Ver catálogo público, registro, login          |
| **Student**    | Usuario autenticado estudiante | Acceso a cursos matriculados, progreso, perfil |
| **Instructor** | Creador de contenido           | Gestión de cursos propios, analytics, ventas   |
| **Admin**      | Administrador de plataforma    | Acceso completo, auditoría, configuración      |
| **System**     | Procesos automáticos           | Webhooks, jobs, eventos inter-servicios        |

---

## Glosario de Dominio

| Término           | Definición                                      | Contexto            |
| ----------------- | ----------------------------------------------- | ------------------- |
| **Course**        | Contenedor de lecciones con metadata comercial  | courses-service     |
| **Lesson**        | Unidad mínima de contenido (video/article/quiz) | courses-service     |
| **Enrollment**    | Matrícula activa estudiante↔curso con progreso  | enrollments-service |
| **Order**         | Transacción de compra con proveedor de pago     | payments-service    |
| **Quiz**          | Evaluación con preguntas tipificadas            | assignments-service |
| **Submission**    | Respuesta de estudiante a quiz con timestamp    | assignments-service |
| **Grade**         | Calificación final calculada con feedback       | grades-service      |
| **Content Asset** | Archivo multimedia en MinIO con metadata        | content-service     |
| **Presigned URL** | URL temporal firmada para upload/download       | content-service     |
| **Embedding**     | Vector semántico para búsqueda IA               | ai-service          |

---

## 1. Requisitos Transversales (Cross-Cutting Concerns)

### RF-GLOBAL-001: Autenticación JWT/PASETO

**Propósito:** Sistema unificado de autenticación stateless para todos los microservicios

**Especificación técnica:**

- **Token format:** JWT RS256 (production) / HS256 (development)
- **Access token:** TTL 15min, claims: `sub`, `email`, `role`, `exp`, `iat`
- **Refresh token:** TTL 7 días, rotación obligatoria, almacenado en httpOnly cookie
- **Endpoints centralizados:** auth-service como authority único

**Contratos de API:**

```typescript
// POST /api/v1/auth/login
interface LoginRequest {
  email: string;
  password: string;
}

interface AuthResponse {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
  user: {
    userId: string;
    email: string;
    role: 'student' | 'instructor' | 'admin';
    firstName: string;
    lastName: string;
  };
}
```

**Reglas de negocio:**

- Máximo 5 intentos fallidos → bloqueo temporal 15min
- Refresh rotation: invalidar token anterior al generar nuevo
- Logout: blacklist de access token hasta expiración natural
- Multi-device: permitir hasta 3 refresh tokens simultáneos por usuario

---

### RF-GLOBAL-002: Autorización RBAC (Role-Based Access Control)

**Propósito:** Control granular de acceso basado en roles y ownership

**Matriz de permisos:**

| Recurso            | Anonymous | Student | Instructor | Admin |
| ------------------ | --------- | ------- | ---------- | ----- |
| Courses (public)   | READ      | READ    | READ       | FULL  |
| Courses (own)      | -         | READ    | FULL       | FULL  |
| Enrollments (own)  | -         | FULL    | READ       | FULL  |
| Content (enrolled) | -         | READ    | READ       | FULL  |
| Analytics (own)    | -         | READ    | FULL       | FULL  |
| Users (own)        | -         | FULL    | FULL       | FULL  |
| Orders (own)       | -         | FULL    | READ       | FULL  |

**Implementación:**

- Middleware per-service validando JWT + role claim
- Ownership verification: `ownerId === token.sub` para recursos propios
- Admin override: acceso completo con audit trail obligatorio

---

### RF-GLOBAL-003: Observabilidad Distribuida

**Propósito:** Trazabilidad completa de requests cross-service

**Structured logging (JSON):**

```json
{
  "timestamp": "2025-08-08T10:30:00.123Z",
  "level": "info",
  "service": "courses-service",
  "version": "1.2.3",
  "environment": "production",
  "correlationId": "req-7f2a8b4c-1d9e-4f8a-b3c6-9e8d7c6b5a49",
  "traceId": "trace-550e8400-e29b-41d4-a716-446655440000",
  "spanId": "span-123e4567-e89b-12d3-a456-426614174000",
  "userId": "user-a1b2c3d4-e5f6-7g8h-9i0j-k1l2m3n4o5p6",
  "operation": "create_course",
  "method": "POST",
  "path": "/api/v1/courses",
  "statusCode": 201,
  "duration": 145,
  "metadata": {
    "courseId": "course-new-id",
    "instructorId": "instructor-id"
  }
}
```

**Métricas clave (Prometheus format):**

- `http_request_duration_seconds_bucket` (histogram)
- `http_requests_total` (counter con labels: method, status, service)
- `database_connections_active` (gauge)
- `business_metric_enrollments_total` (counter)

**Distributed tracing:** W3C Trace Context headers propagation

---

### RF-GLOBAL-004: Rate Limiting Inteligente

**Propósito:** Protección contra abuso con diferenciación por actor

**Estrategia por capas:**

```nginx
# Layer 1: Nginx (IP-based)
limit_req_zone $binary_remote_addr zone=rate_ip:10m rate=10r/s;

# Layer 2: Application (JWT-based)
map $http_authorization $jwt_sub {
    default "anonymous";
    ~^Bearer\s+(.+)$ $1;
}
limit_req_zone $jwt_sub zone=rate_user:10m rate=100r/s;
```

**Límites por rol:**

- Anonymous: 10 req/s, burst 20
- Student: 100 req/s, burst 200
- Instructor: 200 req/s, burst 400
- Admin: 500 req/s, burst 1000

---

### RF-GLOBAL-005: Auditoría de Eventos de Dominio

**Propósito:** Trazabilidad de operaciones críticas de negocio

**Event Store (MongoDB):**

```typescript
interface DomainEvent {
  eventId: string;
  eventType: string;
  aggregateId: string;
  aggregateType: string;
  eventVersion: number;
  timestamp: Date;
  correlationId: string;
  causationId?: string;
  userId?: string;
  payload: object;
  metadata: {
    service: string;
    version: string;
    environment: string;
  };
}
```

**Eventos auditables:**

- `course.published`, `course.unpublished`
- `order.created`, `order.paid`, `order.refunded`
- `enrollment.created`, `enrollment.completed`
- `quiz.submitted`, `grade.updated`
- `user.registered`, `user.role_changed`

---

### RF-GLOBAL-006: Internacionalización (i18n)

**Propósito:** Soporte multi-idioma sin acoplar textos en backend

**Arquitectura:**

- Backend: NO textos de UI, solo error codes y enums
- Frontend: React i18n con lazy loading de bundles
- API responses: incluir `Accept-Language` header para formateo de fechas/números

**Idiomas MVP:** ES (español), EN (inglés), PT (portugués)

**Estructura de mensajes:**

```typescript
// Frontend translations
interface Translations {
  'auth.login.title': string;
  'errors.INVALID_CREDENTIALS': string;
  'course.status.PUBLISHED': string;
}

// API error format
interface ApiError {
  code:
    | 'INVALID_CREDENTIALS'
    | 'INSUFFICIENT_PERMISSIONS'
    | 'RESOURCE_NOT_FOUND';
  message: string; // Solo para debugging, NO para UI
  details?: object;
}
```

## 2. auth-service (Autenticación y Autorización)

**Responsabilidad:** Authority central para AuthN/AuthZ, gestión de identidades y tokens

### RF-AUTH-001: Registro de Usuario

**Endpoint:** `POST /api/v1/auth/register`

**Contrato:**

```typescript
interface RegisterRequest {
  email: string; // Valid email format, max 255 chars
  password: string; // Min 10 chars, 1 upper, 1 lower, 1 digit, 1 symbol (!@#$%^&*)
  firstName: string; // Min 2, max 50 chars
  lastName: string; // Min 2, max 50 chars
  acceptTerms: boolean; // Must be true
}

interface RegisterResponse {
  userId: string;
  message: 'REGISTRATION_SUCCESS';
  verification?: {
    emailSent: boolean;
    expiresIn: number;
  };
}
```

**Reglas de negocio:**

- Email único en el sistema (constraint DB level)
- Password hashing: Argon2id con salt único
- Rol por defecto: `student`
- Email verification opcional en MVP (feature flag)
- Rate limit: 3 registros por IP/hora

**Casos edge:**

- Email duplicado → `409 CONFLICT`
- Password débil → `400 BAD_REQUEST` con detalles específicos
- Términos no aceptados → `400 BAD_REQUEST`

---

### RF-AUTH-002: Autenticación (Login)

**Endpoint:** `POST /api/v1/auth/login`

**Contrato:**

```typescript
interface LoginRequest {
  email: string;
  password: string;
  rememberMe?: boolean; // Affects refresh token TTL
}

interface LoginResponse {
  accessToken: string; // JWT, 15min TTL
  refreshToken: string; // Secure cookie, 7d TTL (30d if rememberMe)
  expiresIn: number; // Access token seconds until expiry
  user: UserProfile;
}

interface UserProfile {
  userId: string;
  email: string;
  firstName: string;
  lastName: string;
  role: 'student' | 'instructor' | 'admin';
  avatarUrl?: string;
  preferences: {
    language: string;
    timezone: string;
    emailNotifications: boolean;
  };
}
```

**Reglas de negocio:**

- Máximo 5 intentos fallidos → lockout 15min (exponential backoff)
- 2FA opcional para rol admin (future scope)
- Session tracking: registrar IP, User-Agent, timestamp
- Concurrent sessions: máximo 3 activas por usuario

---

### RF-AUTH-003: Renovación de Token

**Endpoint:** `POST /api/v1/auth/refresh`

**Contrato:**

```typescript
interface RefreshRequest {
  // Refresh token from httpOnly cookie
}

interface RefreshResponse {
  accessToken: string;
  refreshToken: string; // New rotated token
  expiresIn: number;
}
```

**Reglas de negocio:**

- Token rotation: invalidar anterior, generar nuevo
- Refresh token único por dispositivo/browser
- Detección de replay attacks: si token ya usado → logout all sessions
- Grace period: 30s overlap para evitar race conditions

---

### RF-AUTH-004: Perfil Autenticado

**Endpoint:** `GET /api/v1/auth/me`

**Headers requeridos:** `Authorization: Bearer <access_token>`

**Respuesta:**

```typescript
interface AuthenticatedUser extends UserProfile {
  permissions: string[]; // Dynamic permissions based on role
  lastLoginAt: string;
  isEmailVerified: boolean;
  accountStatus: 'active' | 'suspended' | 'pending_verification';
}
```

---

### RF-AUTH-005: Logout y Revocación

**Endpoints:**

- `POST /api/v1/auth/logout` (current session)
- `POST /api/v1/auth/logout-all` (all sessions)

**Implementación:**

- Blacklist access token hasta expiración natural
- Invalidar refresh token(s) en Redis
- Clear httpOnly cookies
- Audit log: registro de logout voluntario vs. forzado

---

## 3. users-service (Gestión de Perfiles)

**Responsabilidad:** CRUD de perfiles de usuario, preferencias y configuración personal

### RF-USERS-001: Obtener Perfil por ID

**Endpoint:** `GET /api/v1/users/:userId`

**Autorización:**

- Usuario solo puede ver su propio perfil
- Instructor puede ver perfiles de estudiantes en sus cursos
- Admin puede ver cualquier perfil

**Contrato:**

```typescript
interface UserProfile {
  userId: string;
  email: string;
  firstName: string;
  lastName: string;
  role: UserRole;
  avatarUrl?: string;
  bio?: string;
  website?: string;
  socialLinks?: {
    linkedin?: string;
    twitter?: string;
    github?: string;
  };
  preferences: UserPreferences;
  stats: UserStats;
  createdAt: string;
  lastActiveAt: string;
}

interface UserStats {
  totalCourses: number; // Enrolled if student, created if instructor
  completedCourses: number;
  averageRating?: number; // For instructors
  totalStudents?: number; // For instructors
}
```

---

### RF-USERS-002: Actualizar Perfil

**Endpoint:** `PATCH /api/v1/users/:userId`

**Campos actualizables:**

```typescript
interface UpdateProfileRequest {
  firstName?: string; // Min 2, max 50 chars
  lastName?: string; // Min 2, max 50 chars
  bio?: string; // Max 500 chars
  website?: string; // Valid URL format
  socialLinks?: SocialLinks;
  preferences?: Partial<UserPreferences>;
}
```

**Validaciones:**

- Sanitización de HTML en campos de texto
- Validación de URLs
- Rate limit: 10 updates/hora por usuario
- Audit trail para cambios sensibles

---

### RF-USERS-003: Gestión de Preferencias

**Endpoint:** `PATCH /api/v1/users/:userId/preferences`

**Estructura de preferencias:**

```typescript
interface UserPreferences {
  language: 'es' | 'en' | 'pt';
  timezone: string; // IANA timezone format
  emailNotifications: {
    courseUpdates: boolean;
    newMessages: boolean;
    promotions: boolean;
    weeklyDigest: boolean;
  };
  privacy: {
    showProfile: boolean;
    showProgress: boolean;
    allowMessaging: boolean;
  };
  accessibility: {
    highContrast: boolean;
    fontSize: 'small' | 'medium' | 'large';
    reducedMotion: boolean;
  };
}
```

---

### RF-USERS-004: Avatar y Archivos de Perfil

**Endpoints:**

- `POST /api/v1/users/:userId/avatar` (upload)
- `DELETE /api/v1/users/:userId/avatar` (remove)

**Especificaciones:**

- Formatos: JPG, PNG, WebP
- Tamaño máximo: 2MB
- Dimensiones: mín 100x100px, máx 1024x1024px
- Procesamiento: auto-resize + formato WebP para optimización
- Storage: MinIO con CDN cache

## 4) courses-service (Catálogo, cursos y lecciones)

- RF-COURSES-01 Listado de cursos públicos

  - Endpoint: GET /api/v1/courses
  - Filtros: search, tags, ownerId, priceRange, isPublished.
  - Paginación: page, pageSize; orden: createdAt desc.

- RF-COURSES-02 Crear curso (instructor)

  - Endpoint: POST /api/v1/courses
  - Campos: title, description, priceCents, currency, tags[], prerequisites[], slug opcional (autogenerado si falta).
  - Reglas: ownerId = userId del token; isPublished=false por defecto.

- RF-COURSES-03 Ver detalle de curso

  - Endpoint: GET /api/v1/courses/:id
  - Reglas: cursos no publicados solo visibles para owner y Admin.

- RF-COURSES-04 Gestionar lecciones

  - Endpoints: POST /api/v1/courses/:id/lessons, PATCH /api/v1/courses/:id/lessons/:lessonId, DELETE /api/v1/courses/:id/lessons/:lessonId
  - Campos: title, contentType (video|article|quiz), contentRef, orderIndex.
  - Reglas: orderIndex sin huecos; reordenar debe recalcular índices.

- RF-COURSES-05 Publicar/Despublicar curso
  - Endpoint: POST /api/v1/courses/:id/publish, POST /api/v1/courses/:id/unpublish
  - Evento: course.published al publicar.

## 5) enrollments-service (Matrículas y progreso)

- RF-ENR-01 Crear matrícula al pagar

  - Trigger: evento order.paid con userId y courseId.
  - Reglas: si ya existe, mantener status active; idempotencia por orderId.

- RF-ENR-02 Listar mis matrículas

  - Endpoint: GET /api/v1/enrollments/my
  - Respuesta: cursos, status, progressPercent.

- RF-ENR-03 Actualizar progreso

  - Endpoint: PATCH /api/v1/enrollments/:id/progress
  - Reglas: 0–100; no decrementar salvo admin override; derivado también de lesson.viewed.

- RF-ENR-04 Verificación de acceso
  - Servicio debe exponer check para content-service: ¿userId tiene acceso activo a courseId?

## 6) content-service (Activos en MinIO/S3)

- RF-CONTENT-01 Presign para subida (instructor)

  - Endpoint: POST /api/v1/content/presign-upload
  - Entradas: contentType, contentLength, courseId opcional.
  - Reglas: tamaño máximo por tipo; key generado; solo owner/admin.
  - Salidas: url y fields si aplica (POST form) o URL PUT.

- RF-CONTENT-02 Presign para descarga (student)
  - Endpoint: GET /api/v1/content/:key/presign-download
  - Reglas: validar acceso vía enrollments-service; expira en N minutos.

## 7) assignments-service (Quizzes)

- RF-ASSIGN-01 Obtener quiz por id

  - Endpoint: GET /api/v1/quizzes/:id
  - Reglas: visible solo si el usuario está matriculado en el curso del quiz.

- RF-ASSIGN-02 Enviar respuestas de quiz
  - Endpoint: POST /api/v1/quizzes/:id/submit
  - Entradas: answers[] con questionId y value(s).
  - Reglas: validar tipos (single/multiple/boolean/code); límites de intentos opcional; emitir quiz.submitted.
  - Salidas: score preliminar si es autocorregible; submissionId.

## 8) grades-service (Calificaciones)

- RF-GRADES-01 Consultar mis calificaciones

  - Endpoint: GET /api/v1/grades/my

- RF-GRADES-02 Cálculo de score y feedback
  - Trigger: evento quiz.submitted.
  - Reglas: corrección automática para single/multiple/boolean; code/essay puede quedar pendiente para revisión o usar auto-feedback básico.
  - Evento: grade.updated al finalizar cálculo.

## 9) payments-service (Pedidos y webhooks)

- RF-PAY-01 Crear orden de compra

  - Endpoint: POST /api/v1/orders
  - Entradas: courseId, provider (stripe|mercadopago), currency.
  - Reglas: calcular amountCents desde courses-service; crear provider_ref (paymentIntent o preference).
  - Salidas: redirectInfo o clientSecret según proveedor.

- RF-PAY-02 Webhook de pago

  - Endpoints: POST /api/v1/payments/webhook/stripe, POST /api/v1/payments/webhook/mercadopago
  - Reglas: validar firma; idempotencia por provider_ref; actualizar status a paid/failed; emitir order.paid en caso de pago.

- RF-PAY-03 Reembolsos (opcional MVP+)
  - Endpoint admin para refund; actualizar status y emitir evento.

## 10) notifications-service (Correos y plantillas)

- RF-NOTIF-01 Envío de correo de bienvenida y recibos

  - Triggers: registro (welcome), order.paid (receipt), enrollment creada.
  - Reglas: plantillas versionadas; reintentos en fallo; registro en event_logs.

- RF-NOTIF-02 Preferencias de notificación
  - Fuente: users-service preferences.marketingOptIn.

## 11) analytics-service (KPIs y eventos)

- RF-AN-01 Registro de eventos de uso

  - Endpoint interno: POST /api/v1/analytics/events (batch)
  - Eventos mínimos: lesson.viewed, quiz.submitted, page.view, course.completed.

- RF-AN-02 KPIs de negocio y aprendizaje
  - Endpoints admin/instructor: GET /api/v1/analytics/kpis?scope=... (completionRate, purchaseFunnel, activeStudents).

## 12) search-service (Búsqueda)

- RF-SEARCH-01 Indexación de cursos

  - Triggers: course.created/updated/published.
  - Fuente: courses-service; campos: title, description, tags, ownerName opcional.
  - Extensión IA: mantener embeddings (pgvector) para title/description/transcripts mediante ai-service.

- RF-SEARCH-02 Búsqueda de cursos
  - Endpoint: GET /api/v1/search/courses?query=...&page=...&pageSize=...
  - Reglas: texto completo con Mongo text index o Elastic; ordenar por score.
  - Extensión IA: soportar búsqueda semántica (similaridad por embeddings) delegada en ai-service.

## 13) Frontend (React 19 + Vite + Tailwind)

- RF-FE-01 Catálogo y detalle

  - Páginas: /, /course/:slug. Ver cursos, filtros, CTA de compra.

- RF-FE-02 Player de aprendizaje

  - Página: /learn/:courseId. Lista de lecciones, reproducción de video/artículo/quiz. Marca progreso al completar lección.

- RF-FE-03 Panel de instructor

  - Páginas: /instructor/\* para crear/editar cursos y lecciones, publicar y ver ventas básicas.

- RF-FE-04 Checkout

  - Página: /checkout. Flujo de creación de order y redirección a proveedor.

- RF-FE-05 Cuenta de usuario
  - Página: /account. Ver/editar perfil, preferencias, facturación.

## 14) Flujos cross-service (happy path)

- RF-FLOW-01 Compra y matrícula

  - Student crea order (payments-service) → redirección/confirmación → webhook valida pago → payments-service emite order.paid → enrollments-service crea enrollment active → notifications-service envía recibo → student ve curso en /learn.

- RF-FLOW-02 Consumo de contenido

  - Student abre lección → frontend reporta lesson.viewed → enrollments-service incrementa progressPercent → analytics registra evento.

- RF-FLOW-03 Quiz y calificación
  - Student envía submit → assignments-service valida y calcula score preliminar → emite quiz.submitted → grades-service consolida score y emite grade.updated → frontend muestra resultado.

## 15) Reglas de negocio clave

- Un usuario solo puede tener una Enrollment por courseId (UNIQUE userId+courseId).
- Un curso publicado no puede cambiar slug si tiene ventas activas (bloqueo o redirección 301 gestionada por frontend/NGINX).
- Progreso no decrece salvo acción de Admin.
- Idempotencia obligatoria en webhooks y creación de Enrollment/Order.

## 16) Validaciones y errores estándar

- Respuestas de error JSON con code, message, details[].
- Paginación con límites: pageSize máximo 100.
- Payloads JSON en camelCase; tablas en snake_case; enums validados.

## 17) Eventos de dominio (JSON camelCase)

- course.published { courseId, publishedAt }
- order.paid { orderId, userId, courseId, amountCents, provider, paidAt }
- quiz.submitted { submissionId, quizId, userId, scorePreliminary, submittedAt }
- grade.updated { submissionId, userId, quizId, finalScore, updatedAt }
- lesson.viewed { userId, courseId, lessonId, viewedAt }

## 18) Soporte y Comunicaciones

### RF-SUPPORT-001: Chatbot como Primer Canal de Ayuda

**Propósito:** Proporcionar soporte automatizado 24/7 como primera línea de atención para todos los roles del sistema.

**Especificación técnica:**

El chatbot será el **canal primario de soporte** antes de escalar a atención humana. Debe ser capaz de:

- Responder preguntas frecuentes (FAQ) sobre la plataforma
- Guiar a usuarios en procesos comunes (registro, compra, acceso a cursos)
- Resolver problemas técnicos básicos (reset de contraseña, problemas de video)
- Escalar a soporte humano cuando no pueda resolver

**Tecnologías recomendadas para implementación:**

| Opción                       | Descripción                                          | Ventajas                                                               | Desventajas                                     |
| ---------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------- |
| **Rasa Open Source**         | Framework open-source para chatbots conversacionales | Control total, self-hosted, NLU personalizable, sin costos de licencia | Requiere más desarrollo, infraestructura propia |
| **Dialogflow CX** (Google)   | Plataforma enterprise de Google Cloud                | Integraciones nativas, NLU potente, fácil de mantener                  | Costos por uso, dependencia de vendor           |
| **Amazon Lex**               | Servicio de AWS para chatbots                        | Integración con AWS ecosystem, escalable                               | Costos por uso, curva de aprendizaje            |
| **Botpress**                 | Plataforma open-source con UI visual                 | Visual flow builder, self-hosted option, plugins                       | Comunidad más pequeña                           |
| **Custom con OpenAI/Claude** | Implementación propia con LLM                        | Máxima flexibilidad, respuestas naturales                              | Costos por token, requiere fine-tuning          |

**Recomendación para ACC-LMS:**

1. **MVP:** Implementar con **Rasa Open Source** para mantener control total y alinearse con filosofía open-source del proyecto
2. **Alternativa comercial:** **Dialogflow CX** si se prioriza time-to-market
3. **Evolución futura:** Integrar capacidades de LLM (OpenAI/Claude) para respuestas más naturales via ai-service

**Arquitectura propuesta:**

```
┌─────────────┐     ┌─────────────────┐     ┌─────────────┐
│   Frontend  │────▶│  chatbot-service │────▶│  ai-service │
│   Widget    │     │   (Rasa/Custom)  │     │  (OpenAI)   │
└─────────────┘     └────────┬────────┘     └─────────────┘
                             │
                    ┌────────▼────────┐
                    │ knowledge-base  │
                    │   (FAQ/Docs)    │
                    └─────────────────┘
```

**Flujos de escalamiento:**

1. **Tier 1 - Chatbot:** Resuelve 70-80% de consultas automáticamente
2. **Tier 2 - Formulario de contacto:** Casos no resueltos → ticket categorizado
3. **Tier 3 - Soporte humano:** Casos complejos vía email interno (nunca público)

**Categorías de soporte por rol:**

| Rol            | Categorías principales                                        |
| -------------- | ------------------------------------------------------------- |
| **Student**    | Acceso a cursos, problemas de pago, certificados, contenido   |
| **Instructor** | Publicación de cursos, analytics, pagos/comisiones, contenido |
| **Admin**      | Reportes de sistema, usuarios, configuración                  |

---

### RF-SUPPORT-002: Política de Privacidad de Emails

**Propósito:** Proteger la privacidad de usuarios y staff evitando la exposición pública de direcciones de correo electrónico.

**Regla fundamental:**

> ⚠️ **NUNCA se publicarán direcciones de email reales en ninguna página pública de la plataforma.**

**Especificación técnica:**

- **Prohibido:** Mostrar emails de instructores, administradores o soporte en páginas públicas
- **Prohibido:** Exponer emails en perfiles públicos de usuarios
- **Prohibido:** Incluir emails en metadata, schema.org, o cualquier markup público

**Comunicación gestionada por formularios:**

Toda comunicación debe canalizarse a través de formularios categorizados:

```typescript
// POST /api/v1/contact
interface ContactFormRequest {
  // Categoría obligatoria para routing
  category:
    | 'technical_support' // Problemas técnicos
    | 'billing_inquiry' // Consultas de facturación
    | 'course_question' // Preguntas sobre cursos (a instructor)
    | 'refund_request' // Solicitud de reembolso
    | 'partnership' // Propuestas de colaboración
    | 'bug_report' // Reporte de errores
    | 'feature_request' // Sugerencias de mejora
    | 'account_issue' // Problemas de cuenta
    | 'content_report' // Reporte de contenido inapropiado
    | 'other'; // Otros (requiere descripción)

  // Datos del remitente (autenticado o público)
  senderEmail: string; // Email del remitente para respuesta
  senderName: string; // Nombre para personalización

  // Contexto opcional
  courseId?: string; // Si aplica a curso específico
  orderId?: string; // Si aplica a orden específica

  // Mensaje
  subject: string; // Asunto (max 200 chars)
  message: string; // Mensaje (max 5000 chars)

  // Anti-spam
  captchaToken: string; // reCAPTCHA/hCaptcha token
}

interface ContactFormResponse {
  ticketId: string; // ID para seguimiento
  estimatedResponseTime: string; // Ej: "24-48 horas"
  category: string;
  message: string; // Confirmación
}
```

**Routing automático por categoría:**

| Categoría           | Destinatario         | SLA Respuesta |
| ------------------- | -------------------- | ------------- |
| `technical_support` | Equipo técnico       | 24h           |
| `billing_inquiry`   | Equipo financiero    | 24h           |
| `course_question`   | Instructor del curso | 48h           |
| `refund_request`    | Equipo financiero    | 48h           |
| `partnership`       | Equipo comercial     | 72h           |
| `bug_report`        | Equipo desarrollo    | 48h           |
| `feature_request`   | Product backlog      | N/A           |
| `account_issue`     | Equipo soporte       | 24h           |
| `content_report`    | Moderación           | 24h           |
| `other`             | Soporte general      | 48h           |

**Páginas de contacto:**

- `/contact` - Formulario público general
- `/course/:slug/contact` - Formulario para contactar instructor (sin exponer email)
- `/support` - Centro de ayuda con FAQ + chatbot + formulario

**Reglas de negocio:**

1. Usuarios autenticados: pre-llenar email y nombre del perfil
2. Rate limiting: máximo 5 mensajes por hora por IP/usuario
3. Notificación al destinatario via email interno (nunca reply-to público)
4. Historial de tickets accesible en `/account/support-tickets`
5. Los instructores reciben mensajes en su panel, nunca su email real

**Protección adicional:**

- Emails en base de datos siempre encriptados at-rest
- Logs de acceso a datos de email auditados
- Exportación de datos (GDPR) no incluye emails de otros usuarios

---

## 19) Chatbot Service y Knowledge Base

### RF-CHATBOT-001: Widget de Chatbot Embebido

**Propósito:** Proporcionar asistencia 24/7 mediante chatbot conversacional integrado en todas las páginas.

**Especificación técnica:**

```typescript
// Configuración del widget
interface ChatbotWidgetConfig {
  position: 'bottom-right' | 'bottom-left';
  theme: 'light' | 'dark' | 'auto';
  autoOpen: boolean; // Auto-abrir tras X segundos
  autoOpenDelay: number; // Segundos (default: 30)
  greetingMessage: string; // Mensaje inicial personalizable
  offlineMessage: string; // Mensaje fuera de horario (si humano requerido)
  avatar: string; // URL del avatar del bot
  locale: 'es' | 'en' | 'pt';
}

// POST /api/v1/chatbot/sessions
interface ChatSessionRequest {
  userId?: string; // Null para anónimos
  context: {
    currentPage: string; // URL actual
    userRole?: string;
    enrolledCourses?: string[];
    recentOrders?: string[];
  };
}

interface ChatSessionResponse {
  sessionId: string;
  greeting: string;
  suggestedQuestions: string[];
}
```

**Contratos de API:**

```typescript
// POST /api/v1/chatbot/messages
interface ChatMessageRequest {
  sessionId: string;
  message: string;
  attachments?: string[]; // URLs de archivos adjuntos
}

interface ChatMessageResponse {
  messageId: string;
  response: string;
  confidence: number; // 0-1, umbral para escalamiento
  suggestedActions?: ChatAction[];
  escalationRequired: boolean;
  relatedArticles?: ArticleReference[];
}

interface ChatAction {
  type: 'link' | 'form' | 'escalate' | 'article';
  label: string;
  payload: string; // URL o ID según tipo
}
```

**Reglas de negocio:**

1. Widget disponible en todas las páginas excepto checkout payment step
2. Si `confidence < 0.6`, sugerir artículos relacionados o escalar
3. Historial de sesión persistido 24h para usuarios anónimos, 30 días para autenticados
4. Rate limiting: 30 mensajes/hora por sesión
5. Máximo 3 archivos adjuntos de 5MB cada uno

---

### RF-CHATBOT-002: Sugerencias Contextuales Inteligentes

**Propósito:** Mostrar preguntas y acciones relevantes según contexto del usuario.

**Especificación técnica:**

```typescript
// GET /api/v1/chatbot/suggestions
interface SuggestionsRequest {
  sessionId: string;
  context: UserContext;
}

interface UserContext {
  currentPage: string;
  userRole?: 'anonymous' | 'student' | 'instructor' | 'admin';
  recentActivity?: {
    lastViewedCourse?: string;
    lastPurchase?: string;
    openTickets?: number;
  };
  locale: string;
}

interface SuggestionsResponse {
  quickActions: QuickAction[];
  suggestedQuestions: string[];
  contextualHelp: string; // Tip específico para la página
}

interface QuickAction {
  id: string;
  icon: string;
  label: string;
  type: 'navigate' | 'action' | 'dialog';
  payload: string;
}
```

**Sugerencias por página:**

| Página                | Sugerencias                                                            |
| --------------------- | ---------------------------------------------------------------------- |
| `/courses/:slug`      | "¿Tiene prerequisitos?", "¿Hay certificado?", "Ver opiniones"          |
| `/checkout`           | "¿Métodos de pago?", "¿Garantía de devolución?", "Código de descuento" |
| `/player/:id`         | "No carga el video", "¿Puedo descargar?", "Reportar problema"          |
| `/account/orders`     | "Ver factura", "Solicitar reembolso", "Problema con pago"              |
| `/instructor/courses` | "Crear nuevo curso", "Ver analytics", "Retirar ganancias"              |

---

### RF-CHATBOT-003: Escalamiento a Soporte Humano

**Propósito:** Transición fluida del chatbot a agente humano cuando sea necesario.

**Especificación técnica:**

```typescript
// POST /api/v1/chatbot/escalate
interface EscalationRequest {
  sessionId: string;
  reason: EscalationReason;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  summary: string; // Resumen generado por IA del chat
  transcript: ChatTranscript[];
}

type EscalationReason =
  | 'user_requested' // Usuario pidió hablar con humano
  | 'low_confidence' // Bot no puede responder
  | 'complex_issue' // Tema detectado como complejo
  | 'frustrated_user' // Análisis de sentimiento negativo
  | 'billing_dispute' // Temas de pago sensibles
  | 'technical_failure'; // Error técnico detectado

interface EscalationResponse {
  ticketId: string;
  estimatedWaitTime: string; // Ej: "15-30 minutos"
  queuePosition?: number;
  handoffMessage: string; // Mensaje de transición
  followUpOptions: {
    email: boolean; // Recibir respuesta por email
    callback: boolean; // Solicitar llamada
    livechat: boolean; // Esperar en chat (si disponible)
  };
}
```

**Reglas de escalamiento automático:**

1. 3 intentos fallidos consecutivos → ofrecer escalamiento
2. Palabras clave: "hablar con humano", "supervisor", "queja"
3. Sentimiento negativo detectado en 3+ mensajes
4. Temas sensibles: reembolsos >$100, disputas de cobro
5. Horario de atención humana: Lun-Vie 9:00-18:00 COT

---

### RF-CHATBOT-004: Feedback y Mejora Continua

**Propósito:** Recopilar feedback para mejorar respuestas del chatbot.

**Especificación técnica:**

```typescript
// POST /api/v1/chatbot/feedback
interface ChatFeedbackRequest {
  messageId: string;
  rating: 'helpful' | 'not_helpful';
  reason?: string; // Razón opcional
  suggestedAnswer?: string; // Respuesta correcta sugerida por usuario
}

interface ChatFeedbackResponse {
  feedbackId: string;
  thankYouMessage: string;
}

// GET /api/v1/chatbot/analytics (Admin only)
interface ChatbotAnalytics {
  period: DateRange;
  metrics: {
    totalSessions: number;
    totalMessages: number;
    averageSessionDuration: number;
    resolutionRate: number; // % resuelto sin escalar
    escalationRate: number;
    avgConfidenceScore: number;
    satisfactionScore: number; // De feedback
  };
  topQuestions: QuestionStat[];
  topUnresolvedQueries: string[]; // Para entrenar modelo
  conversionFromChat: number; // % que compraron tras chat
}
```

---

### RF-KB-001: Portal de Knowledge Base

**Propósito:** Centro de ayuda con artículos organizados por categorías y buscables.

**Especificación técnica:**

```typescript
// GET /api/v1/kb/categories
interface KBCategoriesResponse {
  categories: KBCategory[];
}

interface KBCategory {
  id: string;
  slug: string;
  name: string;
  description: string;
  icon: string;
  articleCount: number;
  subcategories?: KBCategory[];
}

// GET /api/v1/kb/articles?category=:slug&search=:query&page=:n
interface KBArticlesResponse {
  articles: KBArticle[];
  pagination: Pagination;
  filters: {
    category?: string;
    tag?: string;
    search?: string;
  };
}

interface KBArticle {
  id: string;
  slug: string;
  title: string;
  excerpt: string;
  category: KBCategory;
  tags: string[];
  viewCount: number;
  helpfulCount: number;
  updatedAt: string;
  readTime: number; // Minutos estimados
}
```

**Categorías base:**

| Categoría        | Subcategorías                                           |
| ---------------- | ------------------------------------------------------- |
| **Comenzar**     | Crear cuenta, Verificar email, Completar perfil         |
| **Cursos**       | Buscar cursos, Comprar, Acceder contenido, Certificados |
| **Pagos**        | Métodos de pago, Facturas, Reembolsos, Problemas        |
| **Cuenta**       | Seguridad, Privacidad, Configuración, Eliminar cuenta   |
| **Instructores** | Crear curso, Publicar, Ganancias, Promoción             |
| **Técnico**      | Video no carga, Problemas acceso, Compatibilidad        |

---

### RF-KB-002: Artículos con Versionado y Localización

**Propósito:** Gestión de artículos multiidioma con historial de cambios.

**Especificación técnica:**

```typescript
// GET /api/v1/kb/articles/:slug
interface KBArticleDetail {
  id: string;
  slug: string;
  title: string;
  content: string; // Markdown renderizado
  contentRaw: string; // Markdown original (admin)
  category: KBCategory;
  tags: string[];

  // Metadata
  author: {
    id: string;
    name: string;
  };
  createdAt: string;
  updatedAt: string;
  version: number;

  // Localización
  locale: string;
  availableLocales: string[];

  // Navegación
  relatedArticles: ArticleReference[];
  previousArticle?: ArticleReference;
  nextArticle?: ArticleReference;

  // Stats
  viewCount: number;
  helpfulVotes: number;
  notHelpfulVotes: number;
}

// POST /api/v1/kb/articles/:id/feedback
interface ArticleFeedbackRequest {
  helpful: boolean;
  comment?: string;
  missingInfo?: string; // ¿Qué información faltó?
}
```

**Reglas de negocio:**

1. Artículos siempre disponibles en español (idioma base)
2. Fallback a español si traducción no disponible
3. Historial de 10 versiones anteriores por artículo
4. Artículos revisados cada 90 días o tras cambios en producto

---

### RF-KB-003: Integración Chatbot-KB

**Propósito:** El chatbot sugiere artículos relevantes de la KB.

**Especificación técnica:**

```typescript
// POST /api/v1/kb/search/semantic
interface SemanticSearchRequest {
  query: string;
  maxResults: number;
  filters?: {
    categories?: string[];
    locale?: string;
  };
}

interface SemanticSearchResponse {
  results: {
    article: KBArticle;
    relevanceScore: number;
    matchedSection?: string; // Sección específica del artículo
  }[];
  suggestedQuery?: string; // Reformulación de búsqueda
}
```

**Flujo de integración:**

1. Usuario pregunta en chatbot
2. Chatbot busca en KB vía semantic search
3. Si `relevanceScore > 0.8`, responde con contenido del artículo
4. Si `0.6 < relevanceScore < 0.8`, sugiere artículo como "podría ayudarte"
5. Si `relevanceScore < 0.6`, intenta NLU interno o escala

---

## 20) Panel de Administración Avanzado

### RF-ADMIN-001: Dashboard Ejecutivo

**Propósito:** Vista consolidada de métricas clave para administradores.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/dashboard
interface AdminDashboardResponse {
  period: DateRange;

  // KPIs principales
  kpis: {
    totalRevenue: Money;
    revenueChange: number; // % vs período anterior
    activeUsers: number;
    activeUsersChange: number;
    newEnrollments: number;
    enrollmentsChange: number;
    completionRate: number;
    completionRateChange: number;
  };

  // Gráficos
  revenueChart: TimeSeriesData;
  enrollmentsChart: TimeSeriesData;
  userActivityChart: TimeSeriesData;

  // Alertas y tareas pendientes
  alerts: AdminAlert[];
  pendingTasks: {
    coursesForReview: number;
    refundRequests: number;
    reportedContent: number;
    supportTickets: number;
  };

  // Top performers
  topCourses: CourseStats[];
  topInstructors: InstructorStats[];
}

interface AdminAlert {
  id: string;
  type: 'warning' | 'error' | 'info';
  title: string;
  description: string;
  action?: {
    label: string;
    url: string;
  };
  createdAt: string;
}
```

---

### RF-ADMIN-002: Gestión de Usuarios Avanzada

**Propósito:** CRUD completo de usuarios con historial y acciones administrativas.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/users
interface AdminUsersRequest {
  page: number;
  pageSize: number;
  search?: string;
  filters: {
    role?: string[];
    status?: UserStatus[];
    createdAfter?: string;
    createdBefore?: string;
    hasEnrollments?: boolean;
    hasPurchases?: boolean;
  };
  sort: {
    field: 'createdAt' | 'lastLoginAt' | 'totalSpent' | 'enrollments';
    order: 'asc' | 'desc';
  };
}

interface AdminUserDetail {
  user: User;

  // Estadísticas
  stats: {
    totalSpent: Money;
    enrollmentCount: number;
    completedCourses: number;
    loginCount: number;
    lastLoginAt: string;
    lastActivityAt: string;
  };

  // Historial
  enrollments: EnrollmentSummary[];
  orders: OrderSummary[];
  supportTickets: TicketSummary[];

  // Auditoría
  auditLog: AuditEntry[];
  notes: AdminNote[]; // Notas internas de admin

  // Flags
  flags: {
    emailVerified: boolean;
    isRestricted: boolean;
    restrictionReason?: string;
    isBanned: boolean;
    banReason?: string;
    banExpiresAt?: string;
  };
}

// POST /api/v1/admin/users/:id/actions
interface UserAdminAction {
  action:
    | 'verify_email'
    | 'reset_password'
    | 'restrict'
    | 'unrestrict'
    | 'ban'
    | 'unban'
    | 'impersonate'
    | 'add_note'
    | 'grant_course'
    | 'revoke_course'
    | 'export_data';
  reason?: string;
  payload?: Record<string, any>;
}
```

**Acciones administrativas:**

| Acción           | Descripción                             | Requiere          |
| ---------------- | --------------------------------------- | ----------------- |
| `verify_email`   | Marca email como verificado manualmente | Admin             |
| `reset_password` | Fuerza reset y envía email              | Admin             |
| `restrict`       | Limita acciones (comprar, comentar)     | Admin + razón     |
| `ban`            | Bloquea acceso completamente            | Admin + razón     |
| `impersonate`    | Login temporal como el usuario          | Super Admin + log |
| `grant_course`   | Da acceso gratuito a curso              | Admin + razón     |
| `export_data`    | Genera export GDPR                      | Sistema           |

---

### RF-ADMIN-003: Moderación de Contenido

**Propósito:** Revisión y aprobación de cursos e instructores.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/moderation/queue
interface ModerationQueueResponse {
  items: ModerationItem[];
  pagination: Pagination;
  summary: {
    pendingCourses: number;
    reportedContent: number;
    pendingInstructors: number;
  };
}

interface ModerationItem {
  id: string;
  type: 'course_review' | 'content_report' | 'instructor_application';
  priority: 'low' | 'medium' | 'high';
  status: 'pending' | 'in_review' | 'approved' | 'rejected';

  // Contexto
  subject: {
    type: string;
    id: string;
    title: string;
    createdBy: UserSummary;
    createdAt: string;
  };

  // Para reportes
  report?: {
    reportedBy: UserSummary;
    reason: string;
    description: string;
  };

  // Asignación
  assignedTo?: UserSummary;
  assignedAt?: string;
}

// POST /api/v1/admin/moderation/:id/decision
interface ModerationDecision {
  decision: 'approve' | 'reject' | 'request_changes';
  feedback: string;
  internalNotes?: string;

  // Para rechazo
  violations?: string[]; // Políticas violadas

  // Para cambios
  requiredChanges?: {
    field: string;
    issue: string;
    suggestion?: string;
  }[];
}
```

**Checklist de revisión de cursos:**

- [ ] Título claro y descriptivo
- [ ] Descripción completa con objetivos
- [ ] Miniatura apropiada (sin texto engañoso)
- [ ] Precio razonable para el contenido
- [ ] Videos de calidad aceptable
- [ ] Sin contenido plagiado
- [ ] Sin información de contacto externa
- [ ] Cumple términos de servicio

---

### RF-ADMIN-004: Gestión Financiera

**Propósito:** Control de ingresos, pagos a instructores y reportes fiscales.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/finance/overview
interface FinanceOverviewResponse {
  period: DateRange;

  revenue: {
    gross: Money;
    netAfterFees: Money; // Después de fees de pago
    platformCommission: Money;
    instructorPayouts: Money;
    refunds: Money;
    chargebacks: Money;
  };

  breakdown: {
    byPaymentMethod: {
      method: string;
      amount: Money;
      transactions: number;
      fees: Money;
    }[];
    byCourse: {
      courseId: string;
      title: string;
      revenue: Money;
      sales: number;
    }[];
    byInstructor: {
      instructorId: string;
      name: string;
      revenue: Money;
      commission: Money;
      pending: Money;
    }[];
  };

  pending: {
    instructorPayouts: Money;
    refundRequests: number;
    disputes: number;
  };
}

// GET /api/v1/admin/finance/payouts
interface PayoutQueueResponse {
  payouts: PendingPayout[];
  summary: {
    totalPending: Money;
    instructorsToPaycount: number;
    nextPayoutDate: string;
  };
}

interface PendingPayout {
  instructorId: string;
  instructorName: string;
  amount: Money;
  paymentMethod: string; // bank_transfer, paypal, etc.
  periodStart: string;
  periodEnd: string;
  sales: number;
  commissionRate: number;
  status: 'pending' | 'processing' | 'completed' | 'failed';
}

// POST /api/v1/admin/finance/payouts/process
interface ProcessPayoutsRequest {
  payoutIds: string[]; // Específicos o todos
  processAll: boolean;
}
```

**Reportes fiscales:**

- Reporte mensual de ventas por país
- Reporte de comisiones por instructor
- Reporte de IVA/impuestos retenidos
- Facturas electrónicas (DIAN Colombia)

---

### RF-ADMIN-005: Analytics y Reportes

**Propósito:** Reportes personalizables y exportables.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/reports/generate
interface ReportRequest {
  type: ReportType;
  period: DateRange;
  filters?: Record<string, any>;
  format: 'json' | 'csv' | 'xlsx' | 'pdf';
  delivery: 'download' | 'email';
}

type ReportType =
  | 'sales_summary'
  | 'instructor_performance'
  | 'course_analytics'
  | 'user_growth'
  | 'completion_rates'
  | 'revenue_by_category'
  | 'refund_analysis'
  | 'support_metrics'
  | 'audit_log';

interface ReportResponse {
  reportId: string;
  status: 'generating' | 'ready' | 'failed';
  downloadUrl?: string;
  expiresAt?: string;
}

// GET /api/v1/admin/analytics/realtime
interface RealtimeAnalyticsResponse {
  activeUsers: number;
  activeSessions: number;
  currentlyWatching: {
    courseId: string;
    title: string;
    viewers: number;
  }[];
  recentOrders: OrderSummary[];
  recentSignups: number; // Últimos 15 min
}
```

---

### RF-ADMIN-006: Configuración del Sistema

**Propósito:** Ajustes globales de la plataforma.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/settings
interface SystemSettingsResponse {
  general: {
    siteName: string;
    siteDescription: string;
    supportEmail: string;
    defaultLocale: string;
    defaultCurrency: string;
    maintenanceMode: boolean;
    maintenanceMessage?: string;
  };

  commerce: {
    platformCommissionRate: number; // % de comisión
    minimumPayout: Money;
    payoutSchedule: 'weekly' | 'biweekly' | 'monthly';
    allowedPaymentMethods: string[];
    refundWindowDays: number;
  };

  content: {
    maxVideoSizeMB: number;
    allowedVideoFormats: string[];
    requireCourseReview: boolean;
    autoPublishAfterReview: boolean;
    minimumCoursePriceUSD: number;
    maximumCoursePriceUSD: number;
  };

  security: {
    maxLoginAttempts: number;
    lockoutDurationMinutes: number;
    sessionTimeoutMinutes: number;
    require2FA: boolean;
    passwordMinLength: number;
    allowedEmailDomains?: string[]; // Null = todos permitidos
  };

  emails: {
    smtpConfigured: boolean;
    fromName: string;
    fromEmail: string;
    templates: {
      welcome: boolean;
      purchase: boolean;
      enrollment: boolean;
      // ...
    };
  };

  integrations: {
    stripeEnabled: boolean;
    mercadoPagoEnabled: boolean;
    googleAnalyticsId?: string;
    facebookPixelId?: string;
    intercomAppId?: string;
  };
}

// PATCH /api/v1/admin/settings/:section
interface UpdateSettingsRequest {
  settings: Partial<SystemSettings>;
  reason: string; // Para audit log
}
```

---

### RF-ADMIN-007: Auditoría y Seguridad

**Propósito:** Registro completo de acciones administrativas y detección de anomalías.

**Especificación técnica:**

```typescript
// GET /api/v1/admin/audit-log
interface AuditLogRequest {
  page: number;
  pageSize: number;
  filters: {
    actorId?: string;
    action?: string[];
    resourceType?: string[];
    dateFrom?: string;
    dateTo?: string;
    severity?: ('info' | 'warning' | 'critical')[];
  };
}

interface AuditEntry {
  id: string;
  timestamp: string;
  actor: {
    id: string;
    name: string;
    role: string;
    ip: string;
    userAgent: string;
  };
  action: string;
  resource: {
    type: string;
    id: string;
    name?: string;
  };
  changes?: {
    field: string;
    oldValue: any;
    newValue: any;
  }[];
  severity: 'info' | 'warning' | 'critical';
  metadata?: Record<string, any>;
}

// GET /api/v1/admin/security/alerts
interface SecurityAlertsResponse {
  alerts: SecurityAlert[];
  summary: {
    critical: number;
    high: number;
    medium: number;
    low: number;
  };
}

interface SecurityAlert {
  id: string;
  type:
    | 'brute_force_attempt'
    | 'unusual_login_location'
    | 'privilege_escalation'
    | 'mass_data_access'
    | 'failed_payment_spike'
    | 'suspicious_refund_pattern';
  severity: 'critical' | 'high' | 'medium' | 'low';
  description: string;
  affectedUser?: UserSummary;
  metadata: Record<string, any>;
  status: 'open' | 'investigating' | 'resolved' | 'false_positive';
  createdAt: string;
}
```

---

## 21) Panel de Instructor Avanzado

### RF-INSTRUCTOR-001: Dashboard de Instructor

**Propósito:** Vista personalizada de métricas y tareas para instructores.

**Especificación técnica:**

```typescript
// GET /api/v1/instructor/dashboard
interface InstructorDashboardResponse {
  instructor: InstructorProfile;

  // KPIs del período
  kpis: {
    totalEarnings: Money;
    earningsChange: number;
    totalStudents: number;
    studentsChange: number;
    avgRating: number;
    ratingChange: number;
    coursesPublished: number;
  };

  // Gráficos
  earningsChart: TimeSeriesData;
  enrollmentsChart: TimeSeriesData;

  // Actividad reciente
  recentEnrollments: EnrollmentSummary[];
  recentReviews: ReviewSummary[];
  recentQuestions: QuestionSummary[];

  // Tareas pendientes
  pendingTasks: {
    questionsToAnswer: number;
    assignmentsToGrade: number;
    reviewsToRespond: number;
    coursesInDraft: number;
  };

  // Earnings disponibles
  availableForWithdrawal: Money;
  nextPayoutDate: string;
}
```

---

### RF-INSTRUCTOR-002: Quiz Builder Avanzado

**Propósito:** Herramienta visual para crear evaluaciones con múltiples tipos de preguntas.

**Especificación técnica:**

```typescript
// POST /api/v1/instructor/quizzes
interface CreateQuizRequest {
  courseId: string;
  lessonId?: string; // Si pertenece a lección específica
  title: string;
  description?: string;

  settings: QuizSettings;
  questions: QuizQuestion[];
}

interface QuizSettings {
  type: 'practice' | 'graded' | 'survey';
  timeLimit?: number; // Minutos, null = sin límite
  attempts: number | 'unlimited';
  passingScore?: number; // Porcentaje (solo graded)
  shuffleQuestions: boolean;
  shuffleAnswers: boolean;
  showCorrectAnswers:
    | 'immediately'
    | 'after_submit'
    | 'after_deadline'
    | 'never';
  showFeedback: boolean;
  releaseDate?: string;
  deadline?: string;
  lateSubmissionPolicy?: {
    allowed: boolean;
    penaltyPerDay: number; // % de penalización
    maxDaysLate: number;
  };
}

interface QuizQuestion {
  id: string;
  order: number;
  type: QuestionType;
  question: string; // Soporta Markdown
  mediaUrl?: string; // Imagen/video opcional
  points: number;
  required: boolean;

  // Según tipo
  options?: QuestionOption[]; // Para multiple choice
  correctAnswer?: string; // Para short answer
  rubric?: GradingRubric; // Para essay
  codeConfig?: CodeQuestionConfig;
  matchingPairs?: MatchingPair[];
}

type QuestionType =
  | 'multiple_choice' // Una respuesta correcta
  | 'multiple_answer' // Múltiples correctas
  | 'true_false'
  | 'short_answer' // Texto corto
  | 'essay' // Texto largo
  | 'code' // Código con evaluación
  | 'matching' // Emparejar columnas
  | 'ordering' // Ordenar elementos
  | 'fill_blank'; // Completar espacios

interface QuestionOption {
  id: string;
  text: string;
  isCorrect: boolean;
  feedback?: string; // Feedback específico de esta opción
}

interface GradingRubric {
  criteria: {
    name: string;
    description: string;
    maxPoints: number;
    levels: {
      score: number;
      description: string;
    }[];
  }[];
}

interface CodeQuestionConfig {
  language: string;
  starterCode?: string;
  testCases: {
    input: string;
    expectedOutput: string;
    isHidden: boolean; // Tests ocultos para anti-trampa
    points: number;
  }[];
  timeoutMs: number;
  memoryLimitMB: number;
}
```

---

### RF-INSTRUCTOR-003: Gestión de Estudiantes

**Propósito:** Ver y gestionar estudiantes matriculados en cursos propios.

**Especificación técnica:**

```typescript
// GET /api/v1/instructor/courses/:courseId/students
interface CourseStudentsResponse {
  students: StudentProgress[];
  pagination: Pagination;
  summary: {
    totalEnrolled: number;
    activeLastWeek: number;
    completionRate: number;
    avgProgress: number;
  };
}

interface StudentProgress {
  userId: string;
  name: string;
  email: string; // Solo visible para instructor
  enrolledAt: string;
  lastAccessAt: string;

  progress: {
    percentage: number;
    completedLessons: number;
    totalLessons: number;
    timeSpent: number; // Minutos
  };

  grades: {
    quizzes: {
      completed: number;
      total: number;
      avgScore: number;
    };
    assignments: {
      submitted: number;
      graded: number;
      pending: number;
      avgScore: number;
    };
  };

  engagement: {
    questionsAsked: number;
    discussionPosts: number;
    notesCreated: number;
  };
}

// POST /api/v1/instructor/courses/:courseId/students/:userId/message
interface MessageStudentRequest {
  subject: string;
  message: string;
  type: 'general' | 'reminder' | 'feedback' | 'congratulation';
}
```

---

### RF-INSTRUCTOR-004: Calificación Manual

**Propósito:** Revisar y calificar submissions de essays y tareas complejas.

**Especificación técnica:**

```typescript
// GET /api/v1/instructor/grading/pending
interface PendingGradingResponse {
  submissions: PendingSubmission[];
  pagination: Pagination;
  summary: {
    totalPending: number;
    overdueGrading: number; // Más de 7 días sin calificar
  };
}

interface PendingSubmission {
  submissionId: string;
  student: UserSummary;
  course: CourseSummary;
  assignment: AssignmentSummary;
  submittedAt: string;
  daysWaiting: number;

  // Preview
  answerPreview: string; // Primeros 500 chars
  attachments: string[]; // URLs

  // AI assist
  aiSuggestedScore?: number;
  aiFeedback?: string;
}

// POST /api/v1/instructor/grading/:submissionId
interface GradeSubmissionRequest {
  scores: {
    criterionId: string;
    score: number;
    comment?: string;
  }[];
  overallFeedback: string;
  finalScore: number;

  // Opciones
  publishImmediately: boolean;
  allowResubmission: boolean;
  resubmissionDeadline?: string;
}
```

---

### RF-INSTRUCTOR-005: Foros de Discusión

**Propósito:** Gestionar discusiones y Q&A de cursos.

**Especificación técnica:**

```typescript
// GET /api/v1/courses/:courseId/discussions
interface DiscussionsResponse {
  threads: DiscussionThread[];
  pagination: Pagination;
  stats: {
    totalThreads: number;
    unanswered: number;
    answeredByInstructor: number;
  };
}

interface DiscussionThread {
  id: string;
  title: string;
  content: string;
  author: UserSummary;
  createdAt: string;

  // Contexto
  lessonId?: string;
  lessonTitle?: string;

  // Estado
  isAnswered: boolean;
  isPinned: boolean;
  isLocked: boolean;

  // Stats
  repliesCount: number;
  likesCount: number;
  viewsCount: number;

  // Última actividad
  lastReply?: {
    author: UserSummary;
    createdAt: string;
    isInstructor: boolean;
  };
}

// POST /api/v1/courses/:courseId/discussions/:threadId/replies
interface CreateReplyRequest {
  content: string; // Markdown
  markAsAnswer: boolean; // Solo instructor puede marcar
  attachments?: string[];
}

// Acciones de instructor
// POST /api/v1/instructor/discussions/:threadId/actions
interface DiscussionActionRequest {
  action:
    | 'pin'
    | 'unpin'
    | 'lock'
    | 'unlock'
    | 'mark_answered'
    | 'delete'
    | 'move';
  reason?: string;
  targetLessonId?: string; // Para move
}
```

---

### RF-INSTRUCTOR-006: Analytics de Cursos

**Propósito:** Métricas detalladas de rendimiento de cursos.

**Especificación técnica:**

```typescript
// GET /api/v1/instructor/courses/:courseId/analytics
interface CourseAnalyticsResponse {
  course: CourseSummary;
  period: DateRange;

  // Métricas de ventas
  sales: {
    totalRevenue: Money;
    instructorEarnings: Money;
    totalEnrollments: number;
    refunds: number;
    refundRate: number;
  };

  // Métricas de engagement
  engagement: {
    avgTimePerSession: number;
    avgLessonsPerSession: number;
    completionRate: number;
    dropOffRate: number;
  };

  // Análisis de contenido
  contentAnalysis: {
    lessonId: string;
    title: string;
    viewCount: number;
    avgWatchTime: number; // % del video visto
    dropOffPoint?: number; // Segundo donde más abandonan
    questions: number;
    rating?: number;
  }[];

  // Análisis de quizzes
  quizAnalysis: {
    quizId: string;
    title: string;
    attempts: number;
    avgScore: number;
    passRate: number;
    hardestQuestions: {
      questionId: string;
      question: string;
      correctRate: number;
    }[];
  }[];

  // Reviews
  reviews: {
    avgRating: number;
    ratingDistribution: number[]; // [1star, 2star, 3star, 4star, 5star]
    recentReviews: ReviewSummary[];
    sentiment: {
      positive: string[]; // Temas mencionados positivamente
      negative: string[]; // Temas mencionados negativamente
    };
  };

  // Comparativa
  benchmark: {
    categoryAvgRating: number;
    categoryAvgCompletion: number;
    categoryAvgPrice: Money;
    yourRanking: number; // Posición en categoría
  };
}
```

---

### RF-INSTRUCTOR-007: Media Library

**Propósito:** Gestión centralizada de archivos multimedia del instructor.

**Especificación técnica:**

```typescript
// GET /api/v1/instructor/media
interface MediaLibraryResponse {
  items: MediaItem[];
  pagination: Pagination;
  usage: {
    totalSizeMB: number;
    limitMB: number;
    videoCount: number;
    imageCount: number;
    documentCount: number;
  };
}

interface MediaItem {
  id: string;
  filename: string;
  type: 'video' | 'image' | 'document' | 'audio';
  mimeType: string;
  sizeMB: number;
  uploadedAt: string;

  // URLs
  url: string; // Presigned, temporal
  thumbnailUrl?: string;

  // Metadata
  metadata: {
    duration?: number; // Para video/audio
    dimensions?: {
      width: number;
      height: number;
    };
    processingStatus: 'pending' | 'processing' | 'ready' | 'failed';
    transcriptionStatus?: 'pending' | 'processing' | 'ready' | 'failed';
  };

  // Uso
  usedIn: {
    type: 'course' | 'lesson' | 'quiz';
    id: string;
    title: string;
  }[];
}

// POST /api/v1/instructor/media/upload
interface UploadMediaRequest {
  filename: string;
  mimeType: string;
  sizeBytes: number;
}

interface UploadMediaResponse {
  mediaId: string;
  uploadUrl: string; // Presigned PUT URL
  expiresAt: string;
}

// POST /api/v1/instructor/media/:id/process
interface ProcessMediaRequest {
  operations: (
    | { type: 'transcode'; quality: 'low' | 'medium' | 'high' | 'all' }
    | { type: 'thumbnail'; timestamp?: number }
    | { type: 'transcribe'; language?: string }
    | { type: 'compress' }
  )[];
}
```

---

## 22) Experiencia del Estudiante Avanzada

### RF-STUDENT-001: Video Player Enriquecido

**Propósito:** Reproductor de video con funcionalidades avanzadas de aprendizaje.

**Especificación técnica:**

```typescript
// Configuración del player
interface VideoPlayerConfig {
  videoUrl: string;
  subtitles: SubtitleTrack[];
  playbackRates: number[]; // [0.5, 0.75, 1, 1.25, 1.5, 2]
  quality: VideoQuality[];

  features: {
    autoplay: boolean;
    pictureInPicture: boolean;
    fullscreen: boolean;
    theater: boolean;
    keyboard: boolean; // Atajos de teclado
    chromecast: boolean;
    airplay: boolean;
  };

  resumeAt?: number; // Segundo donde continuar
  bookmarks: Bookmark[];
  notes: VideoNote[];
}

interface SubtitleTrack {
  language: string;
  label: string;
  url: string; // VTT file
  isDefault: boolean;
}

interface VideoQuality {
  label: string; // "1080p", "720p", etc.
  url: string;
  width: number;
  height: number;
}

interface Bookmark {
  id: string;
  timestamp: number;
  label: string;
  isSystemGenerated: boolean; // Chapters del instructor
}

// POST /api/v1/lessons/:lessonId/progress
interface LessonProgressEvent {
  eventType: 'play' | 'pause' | 'seek' | 'complete' | 'heartbeat';
  timestamp: number; // Posición en video
  playbackRate: number;
  totalWatchedSeconds: number;
}
```

**Funcionalidades del player:**

- **Atajos de teclado:** Space=play/pause, ←/→=±10s, ↑/↓=volumen, F=fullscreen
- **Picture-in-Picture:** Seguir viendo mientras navega
- **Velocidad variable:** 0.5x a 2x con pitch correction
- **Chapters:** Navegación por secciones del video
- **Autoplay next:** Siguiente lección automáticamente
- **Remember position:** Recordar donde quedó

---

### RF-STUDENT-002: Sistema de Notas Personal

**Propósito:** Tomar y organizar notas durante el aprendizaje.

**Especificación técnica:**

```typescript
// GET /api/v1/students/notes
interface StudentNotesResponse {
  notes: Note[];
  pagination: Pagination;
  stats: {
    totalNotes: number;
    notesByCourse: {
      courseId: string;
      title: string;
      count: number;
    }[];
  };
}

interface Note {
  id: string;
  content: string; // Markdown

  // Contexto
  context: {
    courseId: string;
    courseTitle: string;
    lessonId?: string;
    lessonTitle?: string;
    videoTimestamp?: number; // Si tomada durante video
  };

  // Organización
  tags: string[];
  color?: string; // Highlight color
  isPinned: boolean;

  // Metadata
  createdAt: string;
  updatedAt: string;
}

// POST /api/v1/students/notes
interface CreateNoteRequest {
  content: string;
  courseId: string;
  lessonId?: string;
  videoTimestamp?: number;
  tags?: string[];
}

// GET /api/v1/students/notes/export
interface ExportNotesRequest {
  format: 'markdown' | 'pdf' | 'docx';
  courseId?: string; // Todas o de un curso
  includeTimestamps: boolean;
}
```

---

### RF-STUDENT-003: Wishlist y Favoritos

**Propósito:** Guardar cursos para ver/comprar después.

**Especificación técnica:**

```typescript
// GET /api/v1/students/wishlist
interface WishlistResponse {
  items: WishlistItem[];
  pagination: Pagination;
}

interface WishlistItem {
  id: string;
  addedAt: string;
  course: CourseSummary;

  // Alertas
  priceAlert: {
    enabled: boolean;
    targetPrice?: Money;
    notifyOnAnyDiscount: boolean;
  };

  // Estado
  currentPrice: Money;
  priceDropped: boolean;
  priceDropAmount?: Money;
  isOnSale: boolean;
  saleEndsAt?: string;
}

// POST /api/v1/students/wishlist
interface AddToWishlistRequest {
  courseId: string;
  priceAlertEnabled?: boolean;
  targetPrice?: number;
}

// POST /api/v1/students/wishlist/:id/alert
interface UpdatePriceAlertRequest {
  enabled: boolean;
  targetPrice?: number;
  notifyOnAnyDiscount: boolean;
}
```

---

### RF-STUDENT-004: Foros y Comunidad

**Propósito:** Participación en discusiones de cursos.

**Especificación técnica:**

```typescript
// GET /api/v1/students/discussions
interface MyDiscussionsResponse {
  threads: {
    started: DiscussionThread[];
    participated: DiscussionThread[];
    bookmarked: DiscussionThread[];
  };
  stats: {
    threadsStarted: number;
    repliesPosted: number;
    likesReceived: number;
    markedAsAnswer: number;
  };
}

// POST /api/v1/courses/:courseId/discussions
interface CreateDiscussionRequest {
  title: string;
  content: string; // Markdown
  lessonId?: string; // Asociar a lección
  tags?: string[];
  attachments?: string[];
}

// Interacciones
// POST /api/v1/discussions/:threadId/like
// POST /api/v1/discussions/:threadId/bookmark
// POST /api/v1/discussions/:threadId/report
interface ReportDiscussionRequest {
  reason: 'spam' | 'inappropriate' | 'off_topic' | 'harassment' | 'other';
  description?: string;
}
```

---

### RF-STUDENT-005: Mensajería con Instructor

**Propósito:** Comunicación directa con instructores de cursos matriculados.

**Especificación técnica:**

```typescript
// GET /api/v1/students/messages
interface StudentMessagesResponse {
  conversations: Conversation[];
  unreadCount: number;
}

interface Conversation {
  id: string;
  instructor: UserSummary;
  course: CourseSummary;

  lastMessage: {
    content: string;
    sentAt: string;
    sentBy: 'student' | 'instructor';
    isRead: boolean;
  };

  unreadCount: number;
  createdAt: string;
}

// GET /api/v1/students/messages/:conversationId
interface ConversationDetailResponse {
  conversation: Conversation;
  messages: Message[];
  pagination: Pagination;
}

interface Message {
  id: string;
  content: string;
  attachments?: {
    filename: string;
    url: string;
    type: string;
  }[];
  sentBy: 'student' | 'instructor';
  sentAt: string;
  readAt?: string;
}

// POST /api/v1/students/messages/:conversationId
interface SendMessageRequest {
  content: string;
  attachments?: string[]; // IDs de archivos subidos
}

// Restricciones:
// - Solo puede mensajear instructores de cursos matriculados
// - Rate limit: 10 mensajes/hora por conversación
// - Instructor puede bloquear mensajes (reportar abuso)
```

---

## 23) Sistema de Suscripciones

### RF-SUB-001: Planes de Suscripción

**Propósito:** Ofrecer acceso ilimitado mediante planes mensuales/anuales.

**Especificación técnica:**

```typescript
// GET /api/v1/subscriptions/plans
interface SubscriptionPlansResponse {
  plans: SubscriptionPlan[];
  currentPlan?: ActiveSubscription;
}

interface SubscriptionPlan {
  id: string;
  name: string;
  description: string;

  pricing: {
    monthly: Money;
    yearly: Money;
    yearlyDiscount: number; // % de ahorro
  };

  features: {
    unlimitedCourses: boolean;
    downloadVideos: boolean;
    certificates: boolean;
    prioritySupport: boolean;
    exclusiveContent: boolean;
    aiTutor: boolean;
    offlineAccess: boolean;
  };

  restrictions?: {
    coursesPerMonth?: number; // Si no es ilimitado
    downloadLimit?: number;
  };

  isPopular: boolean; // Highlight en UI
  trialDays: number; // 0 si no hay trial
}

interface ActiveSubscription {
  id: string;
  plan: SubscriptionPlan;
  status: 'active' | 'trialing' | 'past_due' | 'canceled' | 'expired';

  billing: {
    interval: 'monthly' | 'yearly';
    currentPeriodStart: string;
    currentPeriodEnd: string;
    nextBillingDate: string;
    amount: Money;
  };

  canceledAt?: string;
  cancelReason?: string;
  willRenew: boolean;
}
```

---

### RF-SUB-002: Gestión de Facturación

**Propósito:** Control de método de pago y historial de cobros.

**Especificación técnica:**

```typescript
// GET /api/v1/subscriptions/billing
interface BillingInfoResponse {
  paymentMethods: PaymentMethod[];
  defaultPaymentMethod?: PaymentMethod;

  invoices: Invoice[];

  upcomingInvoice?: {
    amount: Money;
    dueDate: string;
    items: {
      description: string;
      amount: Money;
    }[];
  };
}

interface PaymentMethod {
  id: string;
  type: 'card' | 'bank_transfer' | 'paypal';
  isDefault: boolean;

  // Para card
  card?: {
    brand: string; // visa, mastercard, etc.
    last4: string;
    expiryMonth: number;
    expiryYear: number;
  };

  // Para bank
  bank?: {
    bankName: string;
    last4: string;
  };
}

interface Invoice {
  id: string;
  number: string;
  status: 'paid' | 'open' | 'void' | 'uncollectible';
  amount: Money;
  paidAt?: string;
  dueDate: string;
  pdfUrl: string;

  items: {
    description: string;
    quantity: number;
    unitPrice: Money;
    amount: Money;
  }[];
}

// POST /api/v1/subscriptions/billing/payment-method
interface AddPaymentMethodRequest {
  type: 'card';
  token: string; // Token de Stripe/MP
  setAsDefault: boolean;
}

// POST /api/v1/subscriptions/billing/update-plan
interface ChangePlanRequest {
  newPlanId: string;
  billingInterval: 'monthly' | 'yearly';
  prorationBehavior: 'create_prorations' | 'none';
}
```

---

### RF-SUB-003: Lifecycle de Suscripción

**Propósito:** Gestión del ciclo de vida completo de la suscripción.

**Especificación técnica:**

```typescript
// POST /api/v1/subscriptions/subscribe
interface SubscribeRequest {
  planId: string;
  billingInterval: 'monthly' | 'yearly';
  paymentMethodId: string;
  couponCode?: string;
}

interface SubscribeResponse {
  subscription: ActiveSubscription;
  invoice: Invoice;
  requiresAction: boolean; // Si necesita 3DS
  clientSecret?: string; // Para confirmar con Stripe.js
}

// POST /api/v1/subscriptions/cancel
interface CancelSubscriptionRequest {
  reason: CancelReason;
  feedback?: string;
  cancelImmediately: boolean; // O al final del período
}

type CancelReason =
  | 'too_expensive'
  | 'not_using'
  | 'found_alternative'
  | 'missing_features'
  | 'technical_issues'
  | 'temporary'
  | 'other';

interface CancelSubscriptionResponse {
  subscription: ActiveSubscription;
  accessUntil: string;
  canReactivate: boolean;

  // Retención
  offer?: RetentionOffer;
}

interface RetentionOffer {
  type: 'discount' | 'pause' | 'downgrade';
  description: string;

  // Para discount
  discountPercent?: number;
  discountMonths?: number;

  // Para pause
  pauseMonths?: number;

  // Para downgrade
  alternativePlan?: SubscriptionPlan;
}

// POST /api/v1/subscriptions/reactivate
// POST /api/v1/subscriptions/pause
interface PauseSubscriptionRequest {
  months: number; // 1-3 meses
  reason?: string;
}
```

---

## 24) Fuera de alcance MVP (para backlog)

- Marketplace multi-tenant, cupones/descuentos avanzados, certificaciones formales, app móvil nativa, gamification/badges, learning paths.

## 25) Trazabilidad (mapa RF ↔ endpoints/tablas)

- **auth-service:** RF-AUTH-001..004 ↔ `/api/v1/auth/*`, tables: sessions
- **users-service:** RF-USERS-001..003 ↔ `/api/v1/users/*`, tables: users, user_preferences
- **courses-service:** RF-COURSES-001..005 ↔ `/api/v1/courses/*`, tables: courses, lessons
- **enrollments-service:** RF-ENR-001..004 ↔ `/api/v1/enrollments/*`, tables: enrollments
- **content-service:** RF-CONTENT-001..002 ↔ `/api/v1/content/*`, MinIO buckets
- **assignments-service:** RF-ASSIGN-001..002 ↔ `/api/v1/quizzes/*`, tables: quizzes, quiz_questions
- **grades-service:** RF-GRADES-001..002 ↔ `/api/v1/grades/*`, tables: submissions
- **payments-service:** RF-PAY-001..003 ↔ `/api/v1/orders`, `/api/v1/payments/webhook/*`, tables: orders
- **notifications-service:** RF-NOTIF-001..002 ↔ jobs/queues, tables: event_logs
- **analytics-service:** RF-AN-001..002 ↔ `/api/v1/analytics/*`, tables: event_logs
- **search-service:** RF-SEARCH-001..002 ↔ `/api/v1/search/*`, indices: courses_index
- **ai-service:** RF-AI-001..007 ↔ `/api/v1/ai/*`, tables: embeddings
- **chatbot-service:** RF-CHATBOT-001..004 ↔ `/api/v1/chatbot/*`, tables: chat_sessions, chat_messages
- **kb-service:** RF-KB-001..003 ↔ `/api/v1/kb/*`, tables: kb_articles, kb_categories
- **compliance-service:** RF-COMPLIANCE-001..019 ↔ `/api/v1/compliance/*`, tables: consent_records, data_requests
- **subscription-service:** RF-SUB-001..003 ↔ `/api/v1/subscriptions/*`, tables: subscriptions, invoices

### Mapa Frontend ↔ Backend

| Ruta Frontend    | Servicios Backend                                       |
| ---------------- | ------------------------------------------------------- |
| `/`              | courses-service (featured)                              |
| `/courses`       | courses-service, search-service                         |
| `/courses/:slug` | courses-service, enrollments-service                    |
| `/checkout`      | payments-service, enrollments-service                   |
| `/player/:id`    | content-service, enrollments-service, analytics-service |
| `/account/*`     | users-service, enrollments-service, payments-service    |
| `/instructor/*`  | courses-service, analytics-service, grades-service      |
| `/admin/*`       | Todos los servicios                                     |
| `/support`       | chatbot-service, kb-service                             |
| `/legal/*`       | compliance-service                                      |

Anexos (referencias)

- Ver blueprint y nomenclatura en `.vscode/copilot-instructions.md` e `_docs/business/info-proyecto.md`.

## 21) ai-service (Inteligencia artificial)

- RF-AI-01 Búsqueda semántica

  - Endpoint: GET /api/v1/ai/semantic-search?query=...&scope=(courses|lessons)&page=...&pageSize=...
  - Reglas: usar embeddings (pgvector) indexados desde contenido de courses-service (título, descripción) y transcripts/metadata de content-service; ordenar por similaridad.

- RF-AI-02 Indexación de embeddings

  - Endpoint interno: POST /api/v1/ai/embeddings/index
  - Triggers: course.created/updated/published, content actualizado.
  - Reglas: generar/actualizar embeddings; idempotencia por content hash; soportar batch.

- RF-AI-03 Tutor conversacional por curso (RAG)

  - Endpoints: POST /api/v1/ai/tutor/sessions, POST /api/v1/ai/tutor/messages
  - Reglas: respuestas basadas en contenido del curso (retrieval de documentación y transcripts); limitar contexto por enrollment activo; logs con correlationId.

- RF-AI-04 Generación de quizzes asistida

  - Endpoint: POST /api/v1/ai/quizzes/generate
  - Entradas: courseId o contenido fuente; parámetros de dificultad y cantidad.
  - Reglas: produce banco de preguntas compatible con assignments-service para edición previa a publicar.

- RF-AI-05 Resumen de lecciones y materiales

  - Endpoint: POST /api/v1/ai/summaries
  - Reglas: generar resúmenes y glosarios a partir de transcripts/documentos; guardar como content_assets derivados opcionales.

- RF-AI-06 Auto-feedback básico

  - Endpoint: POST /api/v1/ai/feedback/code | /essay
  - Reglas: generar feedback no vinculante para code/essay; no modifica score final salvo configuración explícita en grades-service.

- RF-AI-07 Privacidad y límites de uso
  - Reglas: no persistir prompts/respuestas con datos personales sin consentimiento; anonimizar userId en logs; cache en Redis con TTL; cuotas por usuario/rol.

---

## Matriz de Validación y Completitud

### Cobertura funcional por stack tecnológico

| Servicio              | Framework      | Puerto | Estado         |
| --------------------- | -------------- | ------ | -------------- |
| auth-service          | Rust/Actix-web | 8080   | ✅ Implementar |
| users-service         | Rust/Actix-web | 8080   | ✅ Implementar |
| courses-service       | Rust/Axum      | 8080   | ✅ Implementar |
| content-service       | Rust/Actix-web | 8080   | ✅ Implementar |
| enrollments-service   | Rust/Actix-web | 8080   | ✅ Implementar |
| payments-service      | Rust/Actix-web | 8080   | ✅ Implementar |
| analytics-service     | Rust/Axum      | 8080   | ✅ Implementar |
| ai-service            | Rust/Actix-web | 8080   | ✅ Implementar |
| notifications-service | Rust/Actix-web | 8080   | ✅ Implementar |

### Criterios de definición de terminado (DoD)

Para cada RF implementado:

1. **Código completado:**

   - Endpoint funcional con validaciones
   - Tests unitarios >80% cobertura
   - Tests de integración para flujos críticos
   - Documentación OpenAPI/Swagger

2. **Calidad asegurada:**

   - SonarQube sin blocker/critical issues
   - Security scan sin vulnerabilidades high/critical
   - Performance test: <200ms P95 en endpoints críticos
   - Load test: 100 RPS sostenidos sin degradación

3. **Observabilidad:**

   - Logs estructurados con correlationId
   - Métricas RED (Rate, Errors, Duration)
   - Health checks funcionales
   - Alerts configurados para errores >5%

4. **Deployment:**
   - Docker build exitoso
   - Deploy automático en staging
   - E2E tests pass en staging environment
   - Rollback plan documentado

---

## Roadmap de Implementación Sugerido

### Sprint 1: Fundación (Semanas 1-2)

**Objetivo:** Base sólida de autenticación y usuarios

- RF-GLOBAL-001 a RF-GLOBAL-006 (Transversales)
- RF-AUTH-001 a RF-AUTH-005 (auth-service completo)
- RF-USERS-001 a RF-USERS-004 (users-service completo)
- Infraestructura: Docker compose, Nginx, PostgreSQL, Redis
- CI/CD pipeline básico

### Sprint 2: Contenido y Catálogo (Semanas 3-4)

**Objetivo:** Gestión de cursos y contenido

- RF-COURSES-001 a RF-COURSES-004 (courses-service MVP)
- content-service básico (presigned URLs)
- Frontend: Catálogo público y panel instructor
- search-service con indexación básica

### Sprint 3: Comercio y Matrículas (Semanas 5-6)

**Objetivo:** Flujo completo de compra

- payments-service con Stripe/MercadoPago
- enrollments-service completo
- notifications-service básico
- Frontend: Checkout y player básico

### Sprint 4: Evaluación y IA (Semanas 7-8)

**Objetivo:** Diferenciadores competitivos

- assignments-service y grades-service
- RF-AI-001 a RF-AI-003 (IA básica: búsqueda semántica + tutor)
- analytics-service con KPIs básicos
- Frontend: Quizzes y chat de tutoría

### Sprint 5: Optimización y Producción (Semanas 9-10)

**Objetivo:** Production-ready

- RF-AI-004 y RF-AI-005 (generación de contenido)
- Observabilidad completa (Prometheus, Grafana)
- Security hardening y penetration testing
- Performance optimization y caching
- Documentation y runbooks

---

## Anexos Técnicos

### Referencias de Arquitectura

- [Clean Architecture Patterns](/.vscode/copilot-instructions.md)
- [Multi-stack Implementation Guide](/.vscode/_docs/info-proyecto.md)
- [Non-Functional Requirements](/.vscode/_docs/non-functional-requirements.md)

### APIs de Terceros Requeridas

- **OpenAI API:** text-embedding-3-small, gpt-4o-mini
- **Stripe API:** Payment Intents, Webhooks, Customer Portal
- **MercadoPago API:** Preferences, Webhooks, Payments
- **MinIO/S3 API:** Presigned URLs, Bucket policies
- **SMTP Provider:** SendGrid, AWS SES, o Mailgun

### Schemas de Base de Datos

```sql
-- Ver implementación detallada en:
-- ./migrations/postgresql/
-- ./migrations/mongodb/
```

---

**Total de RFs definidos:** 85+ funcionalidades completas
**Servicios principales:** 16 microservicios + frontend
**Cobertura IA:** 6 funcionalidades específicas + chatbot con NLU
**Compliance:** 19 RFs de cumplimiento legal (GDPR, CCPA, LGPD, Habeas Data)
**Stack diversity:** Rust backend monolítico con servicios especializados

**Estado:** ✅ **LISTO PARA IMPLEMENTACIÓN**
