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

## 19) Fuera de alcance MVP (para backlog)

- Marketplace multi-tenant, cupones/descuentos avanzados, foros/discusiones, certificaciones, app móvil nativa.

## 20) Trazabilidad (mapa RF ↔ endpoints/tablas)

- auth-service: RF-AUTH-01..04 ↔ /api/v1/auth/\*, tables users (users-service), sessions opcional.
- users-service: RF-USERS-01..03 ↔ /api/v1/users/\*, tables users, user_preferences.
- courses-service: RF-COURSES-01..05 ↔ /api/v1/courses/\*, tables courses, lessons.
- enrollments-service: RF-ENR-01..04 ↔ /api/v1/enrollments/\*, tables enrollments.
- content-service: RF-CONTENT-01..02 ↔ /api/v1/content/\*, MinIO buckets.
- assignments-service: RF-ASSIGN-01..02 ↔ /api/v1/quizzes/\*, tables quizzes, quiz_questions.
- grades-service: RF-GRADES-01..02 ↔ /api/v1/grades/\*, tables submissions.
- payments-service: RF-PAY-01..03 ↔ /api/v1/orders, /api/v1/payments/webhook/\*, table orders.
- notifications-service: RF-NOTIF-01..02 ↔ jobs/queues, event_logs.
- analytics-service: RF-AN-01..02 ↔ /api/v1/analytics/\*, event_logs.
- search-service: RF-SEARCH-01..02 ↔ /api/v1/search/\*, index.

Anexos (referencias)

- Ver blueprint y nomenclatura en .vscode/copilot-instructions.md e .vscode/\_docs/info-proyecto.md.

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

**Total de RFs definidos:** 50+ funcionalidades
**Servicios principales:** 11 microservicios + frontend
**Cobertura IA:** 6 funcionalidades específicas  
**Stack diversity:** 4 tecnologías backend diferentes

**Estado:** ✅ **LISTO PARA IMPLEMENTACIÓN**
