# ACC LMS — Arquitectura de Base de Datos

**Versión:** 2025-12-15
**Estado:** Implementado con Schema-per-Service

---

## 🏗️ Arquitectura Multi-Schema (Schema-per-Service)

### Decisión Arquitectónica

Se implementa **Schema Separation** en PostgreSQL en lugar de Database-per-Service:

| Opción | Pros | Contras |
|--------|------|---------|
| Database-per-Service | Aislamiento total, escalado independiente | Complejidad operacional alta, 5+ instancias |
| **Schema-per-Service** ✓ | Un punto de operación, aislamiento lógico, ACID cuando se necesita | Single point of failure (mitiga con réplicas) |
| Monolito | Simple | Sin aislamiento, acoplamiento fuerte |

### Schemas PostgreSQL

```
┌─────────────────────────────────────────────────────────────────┐
│                     PostgreSQL Instance                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   auth      │  │   users     │  │       courses           │  │
│  │  ─────────  │  │  ─────────  │  │  ───────────────────    │  │
│  │  • users    │  │  • profiles │  │  • courses              │  │
│  │  • refresh_ │  │  • prefs    │  │  • sections             │  │
│  │    tokens   │  │  • stats    │  │  • lessons              │  │
│  └─────────────┘  └─────────────┘  │  • categories           │  │
│                                     └─────────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ enrollments │  │ assessments │  │       payments          │  │
│  │  ─────────  │  │  ─────────  │  │  ───────────────────    │  │
│  │  • enroll   │  │  • quizzes  │  │  • orders               │  │
│  │    ments    │  │  • questions│  │  • transactions         │  │
│  │  • lesson_  │  │  • submiss  │  │  • discount_codes       │  │
│  │    progress │  │    ions     │  │  • reviews              │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│  ┌─────────────┐  ┌───────────────────────────────────────────┐ │
│  │     ai      │  │            notifications                   │ │
│  │  ─────────  │  │  ───────────────────────────────────────  │ │
│  │  • embedd   │  │  • templates                               │ │
│  │    ings     │  │  • queue                                   │ │
│  │  • convers  │  │  • user_settings                           │ │
│  │    ations   │  │                                            │ │
│  │  • messages │  │                                            │ │
│  └─────────────┘  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Usuarios por Servicio (Principio de Menor Privilegio)

| Usuario | Schema Principal | Permisos Adicionales |
|---------|-----------------|---------------------|
| `auth_svc` | `auth` (FULL) | - |
| `users_svc` | `users` (FULL) | - |
| `courses_svc` | `courses` (FULL) | - |
| `enrollments_svc` | `enrollments` (FULL) | `courses` (SELECT) |
| `assessments_svc` | `assessments` (FULL) | `courses` (SELECT), `enrollments` (SELECT) |
| `payments_svc` | `payments` (FULL) | `courses` (SELECT) |
| `ai_svc` | `ai` (FULL) | `courses` (SELECT) |
| `notifications_svc` | `notifications` (FULL) | - |

### Referencias Cross-Schema

```sql
-- SIN Foreign Keys entre schemas (aislamiento)
-- Referencias por UUID, validación a nivel de aplicación

-- Ejemplo: enrollments.enrollments
CREATE TABLE enrollments.enrollments (
    enrollment_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,      -- Referencias auth.users(user_id) - NO FK
    course_id UUID NOT NULL,    -- Referencias courses.courses(course_id) - NO FK
    -- ...
);

-- La integridad referencial se maneja en el servicio:
-- 1. API Gateway valida existencia antes de crear
-- 2. Eventos de dominio sincronizan estados
-- 3. Soft deletes previenen referencias huérfanas
```

---

## 🗄️ Estrategia Multi-Engine

### PostgreSQL (Principal)

- **Datos transaccionales:** Usuarios, cursos, inscripciones, pagos
- **ACID compliance:** Para operaciones críticas de negocio
- **pgvector:** Para embeddings de AI (semantic search)
- **JSON support:** Para metadatos flexibles

### MongoDB (Documentos)

- **Notificaciones:** Templates complejos con historial
- **Schemas flexibles:** Para contenido educativo variable

### ClickHouse (Analytics)

- **Event streaming:** User interactions, video views, quiz attempts
- **Real-time analytics:** Dashboards y reportes de BI
- **Time-series data:** Métricas de performance y uso

---

## 📊 Schema Details

### auth.*

```sql
auth.users (
    user_id UUID PK,
    email TEXT UNIQUE,
    hashed_password TEXT,
    role TEXT CHECK (student|instructor|admin),
    email_verified BOOLEAN,
    -- tokens de verificación
    -- timestamps
)

auth.refresh_tokens (
    token_id UUID PK,
    user_id UUID FK → auth.users,
    token_hash TEXT,
    device_fingerprint TEXT,
    -- expiración
)
```

### users.*

```sql
users.profiles (
    user_id UUID PK,    -- ref auth.users
    first_name, last_name TEXT,
    avatar_url, bio, website TEXT,
    social_links JSONB,
    timezone, language TEXT
)

users.preferences (
    user_id UUID PK,
    email_notifications JSONB,
    privacy JSONB,
    accessibility JSONB
)

users.stats (
    user_id UUID PK,
    courses_enrolled, courses_completed INTEGER,
    total_learning_time_minutes BIGINT,
    current_streak_days, longest_streak_days INTEGER
)
```

### courses.*

```sql
courses.categories (category_id, name, slug, parent_category_id)

courses.courses (
    course_id UUID PK,
    instructor_id UUID,     -- ref auth.users
    category_id UUID FK,
    title, slug, description TEXT,
    price_cents, currency,
    difficulty_level, language,
    is_published, published_at,
    -- ratings, enrollments counters
    -- JSONB: requirements, learning_objectives, target_audience
)

courses.sections (section_id, course_id FK, title, sort_order)

courses.lessons (
    lesson_id UUID PK,
    section_id FK, course_id FK,
    title, content_type, content_ref,
    duration_seconds, is_preview, sort_order
)
```

### enrollments.*

```sql
enrollments.enrollments (
    enrollment_id UUID PK,
    user_id UUID,           -- ref auth.users
    course_id UUID,         -- ref courses.courses
    status CHECK (active|completed|paused|refunded|expired),
    progress_percentage DECIMAL,
    started_at, completed_at, last_accessed_at,
    certificate_issued_at, expires_at
)

enrollments.lesson_progress (
    progress_id UUID PK,
    enrollment_id FK,
    lesson_id UUID,         -- ref courses.lessons
    user_id UUID,
    status CHECK (not_started|in_progress|completed),
    completion_percentage, time_spent_seconds,
    last_position_seconds
)
```

### assessments.*

```sql
assessments.quizzes (
    quiz_id UUID PK,
    course_id, lesson_id UUID,
    title, description, instructions,
    total_points, passing_score_percentage,
    time_limit_minutes, max_attempts,
    shuffle_questions, show_correct_answers, is_published
)

assessments.quiz_questions (
    question_id UUID PK,
    quiz_id FK,
    question_text, question_type,
    points, sort_order, explanation,
    options JSONB, correct_answers JSONB
)

assessments.quiz_submissions (
    submission_id UUID PK,
    quiz_id FK, user_id, enrollment_id UUID,
    attempt_number, status, score, max_score, passed,
    time_spent_seconds
)

assessments.quiz_responses (
    response_id UUID PK,
    submission_id FK, question_id FK,
    answer_data JSONB, is_correct, points_earned
)
```

### payments.*

```sql
payments.orders (
    order_id UUID PK,
    user_id, course_id UUID,
    order_number TEXT UNIQUE,
    status CHECK (pending|processing|paid|failed|cancelled|refunded),
    subtotal_cents, tax_cents, discount_cents, total_cents,
    currency, payment_provider, payment_intent_id,
    discount_code, metadata JSONB
)

payments.transactions (
    transaction_id UUID PK,
    order_id FK,
    provider, provider_transaction_id,
    transaction_type CHECK (payment|refund|chargeback),
    amount_cents, currency, status, provider_fee_cents
)

payments.discount_codes (
    code_id UUID PK, code TEXT UNIQUE,
    discount_type CHECK (percentage|fixed_amount),
    discount_value, minimum_order_cents,
    max_uses, current_uses, valid_from, valid_until
)

payments.reviews (
    review_id UUID PK,
    course_id, user_id, enrollment_id UUID,
    rating INTEGER 1-5,
    review_title, review_text,
    is_public, is_verified_purchase, helpful_votes
)
```

### ai.*

```sql
ai.content_embeddings (
    embedding_id UUID PK,
    content_type CHECK (course|lesson|quiz|user_query),
    content_id UUID,
    text_content TEXT,
    embedding vector(1536),     -- OpenAI ada-002
    metadata JSONB
)

ai.conversations (
    conversation_id UUID PK,
    user_id, course_id UUID,
    title, status CHECK (active|archived)
)

ai.messages (
    message_id UUID PK,
    conversation_id FK,
    role CHECK (user|assistant|system),
    content TEXT,
    tokens_used INTEGER,
    metadata JSONB
)
```

### notifications.*

```sql
notifications.templates (
    template_id UUID PK,
    name TEXT UNIQUE,
    type CHECK (email|push|in_app|sms),
    subject_template, body_template TEXT,
    variables JSONB, is_active
)

notifications.queue (
    notification_id UUID PK,
    user_id UUID, template_id FK,
    type, subject, content,
    status CHECK (pending|sent|failed|read),
    priority 1-5, scheduled_for,
    sent_at, read_at, error_message, retry_count
)

notifications.user_settings (
    user_id UUID PK,
    email_enabled, push_enabled, in_app_enabled, sms_enabled BOOLEAN,
    quiet_hours_start, quiet_hours_end TIME,
    timezone TEXT
)
```

---

## 🔄 Migration Strategy

### Estructura de Migraciones

```bash
db/migrations/
├── postgresql/
│   ├── 000_schema_setup.sql        # Schemas + usuarios + permisos
│   ├── 001_initial_schema.sql      # auth, users, courses, enrollments
│   ├── 002_assignments_and_grades.sql   # assessments
│   ├── 003_payments_and_orders.sql      # payments
│   └── 004_ai_and_notifications.sql     # ai, notifications
├── mongodb/
│   └── 001_notifications_schema.js
└── clickhouse/
    └── 001_analytics_schema.sql
```

### Orden de Ejecución

```bash
# 1. Crear schemas y usuarios
psql -f 000_schema_setup.sql

# 2. Tablas core (auth, users, courses, enrollments)
psql -f 001_initial_schema.sql

# 3. Assessments
psql -f 002_assignments_and_grades.sql

# 4. Payments
psql -f 003_payments_and_orders.sql

# 5. AI y Notifications
psql -f 004_ai_and_notifications.sql
```

### Connection Strings por Servicio

```env
# auth-service
DATABASE_URL=postgres://auth_svc:password@localhost:5432/acc_lms?options=-c%20search_path=auth

# users-service
DATABASE_URL=postgres://users_svc:password@localhost:5432/acc_lms?options=-c%20search_path=users

# courses-service
DATABASE_URL=postgres://courses_svc:password@localhost:5432/acc_lms?options=-c%20search_path=courses

# enrollments-service (necesita acceso a courses también)
DATABASE_URL=postgres://enrollments_svc:password@localhost:5432/acc_lms?options=-c%20search_path=enrollments,courses

# assessments-service
DATABASE_URL=postgres://assessments_svc:password@localhost:5432/acc_lms?options=-c%20search_path=assessments,courses,enrollments

# payments-service
DATABASE_URL=postgres://payments_svc:password@localhost:5432/acc_lms?options=-c%20search_path=payments,courses

# ai-service
DATABASE_URL=postgres://ai_svc:password@localhost:5432/acc_lms?options=-c%20search_path=ai,courses

# notifications-service
DATABASE_URL=postgres://notifications_svc:password@localhost:5432/acc_lms?options=-c%20search_path=notifications
```

---

## 🚀 Service Ownership

| Servicio | Schema | Responsabilidad |
|----------|--------|-----------------|
| `auth-service` | `auth` | Identidad, autenticación, tokens |
| `users-service` | `users` | Perfiles, preferencias, estadísticas |
| `courses-service` | `courses` | Cursos, secciones, lecciones |
| `enrollments-service` | `enrollments` | Inscripciones, progreso |
| `assessments-service` | `assessments` | Quizzes, calificaciones |
| `payments-service` | `payments` | Órdenes, transacciones, reviews |
| `ai-service` | `ai` | Embeddings, conversaciones AI |
| `notifications-service` | `notifications` | Templates, cola de envío |
| `analytics-service` | ClickHouse | Eventos, métricas, reportes |
| `content-service` | MongoDB | Videos, documentos, multimedia |

### Cross-Service Communication

```sql
-- Event sourcing para sync entre servicios
CREATE TABLE domain_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL,
    occurred_at TIMESTAMP DEFAULT now(),
    published_at TIMESTAMP,

    -- Para HATEOAS en eventos
    related_resources JSONB DEFAULT '[]'
);

-- Index para event replay
CREATE INDEX idx_domain_events_aggregate
ON domain_events(aggregate_type, aggregate_id, event_version);
```

---

## 🔒 Seguridad de Base de Datos

### Principio de Menor Privilegio

**CRÍTICO:** Ningún servicio de aplicación usa credenciales de superusuario (`postgres`).

Cada microservicio tiene un usuario PostgreSQL dedicado con acceso **exclusivamente** a su schema:

```sql
-- ❌ PROHIBIDO: Servicios usando superusuario
-- DATABASE_URL=postgres://postgres:password@localhost/acc_lms

-- ✅ CORRECTO: Usuario dedicado por servicio
-- DATABASE_URL=postgres://courses_svc:password@localhost/acc_lms
```

### Matriz de Permisos Granulares

| Usuario | auth | users | courses | enrollments | assessments | payments | ai | notifications |
|---------|:----:|:-----:|:-------:|:-----------:|:-----------:|:--------:|:--:|:-------------:|
| `auth_svc` | **CRUD** | - | - | - | - | - | - | - |
| `users_svc` | - | **CRUD** | - | - | - | - | - | - |
| `courses_svc` | - | - | **CRUD** | - | - | - | - | - |
| `enrollments_svc` | - | - | R | **CRUD** | - | - | - | - |
| `assessments_svc` | - | - | R | R | **CRUD** | - | - | - |
| `payments_svc` | - | - | R | - | - | **CRUD** | - | - |
| `ai_svc` | - | - | R | - | - | - | **CRUD** | - |
| `notifications_svc` | - | - | - | - | - | - | - | **CRUD** |

**Leyenda:**
- **CRUD** = SELECT, INSERT, UPDATE, DELETE (propietario del schema)
- **R** = SELECT only (lectura para validación de referencias)
- **-** = Sin acceso

### Detalle de Permisos por Servicio

#### auth_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA auth TO auth_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA auth TO auth_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA auth TO auth_svc;
-- Tablas: auth.users, auth.refresh_tokens
```

#### users_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA users TO users_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA users TO users_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA users TO users_svc;
-- Tablas: users.profiles, users.preferences, users.stats
```

#### courses_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA courses TO courses_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA courses TO courses_svc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA courses TO courses_svc;
-- Tablas: courses.courses, courses.sections, courses.lessons, courses.categories
```

#### enrollments_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA enrollments TO enrollments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA enrollments TO enrollments_svc;
-- Tablas: enrollments.enrollments, enrollments.lesson_progress

-- Cross-schema: READ ONLY (validar que curso existe)
GRANT USAGE ON SCHEMA courses TO enrollments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO enrollments_svc;
```

#### assessments_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA assessments TO assessments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA assessments TO assessments_svc;
-- Tablas: assessments.quizzes, assessments.quiz_questions,
--         assessments.quiz_submissions, assessments.quiz_responses

-- Cross-schema: READ ONLY
GRANT USAGE ON SCHEMA courses TO assessments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO assessments_svc;
GRANT USAGE ON SCHEMA enrollments TO assessments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA enrollments TO assessments_svc;
```

#### payments_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA payments TO payments_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA payments TO payments_svc;
-- Tablas: payments.orders, payments.transactions,
--         payments.discount_codes, payments.reviews

-- Cross-schema: READ ONLY (obtener precio del curso)
GRANT USAGE ON SCHEMA courses TO payments_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO payments_svc;
```

#### ai_svc
```sql
-- Schema propio: FULL ACCESS
GRANT USAGE ON SCHEMA ai TO ai_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA ai TO ai_svc;
-- Tablas: ai.content_embeddings, ai.conversations, ai.messages

-- Cross-schema: READ ONLY (generar embeddings de contenido)
GRANT USAGE ON SCHEMA courses TO ai_svc;
GRANT SELECT ON ALL TABLES IN SCHEMA courses TO ai_svc;
```

#### notifications_svc
```sql
-- Schema propio: FULL ACCESS (sin acceso cross-schema)
GRANT USAGE ON SCHEMA notifications TO notifications_svc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA notifications TO notifications_svc;
-- Tablas: notifications.templates, notifications.queue, notifications.user_settings
-- Recibe datos de usuarios vía eventos, no consulta directamente
```

### Restricciones de Seguridad

```sql
-- ❌ NUNCA otorgar a servicios de aplicación:
-- SUPERUSER, CREATEDB, CREATEROLE, REPLICATION

-- ❌ NUNCA permitir acceso al schema public
REVOKE ALL ON SCHEMA public FROM PUBLIC;

-- ❌ NUNCA permitir conexiones sin SSL en producción
-- postgresql.conf: ssl = on
-- pg_hba.conf: hostssl all all 0.0.0.0/0 scram-sha-256

-- ✅ Contraseñas desde secrets manager (no hardcoded)
-- En producción usar: AWS Secrets Manager, Vault, etc.
```

### Auditoría de Permisos

```sql
-- Query para verificar permisos actuales
SELECT
    grantee,
    table_schema,
    table_name,
    privilege_type
FROM information_schema.role_table_grants
WHERE grantee LIKE '%_svc'
ORDER BY grantee, table_schema, table_name;

-- Query para verificar acceso a schemas
SELECT
    nspname AS schema,
    pg_catalog.pg_get_userbyid(nspowner) AS owner,
    array_agg(DISTINCT usename) AS users_with_access
FROM pg_namespace n
LEFT JOIN pg_user u ON has_schema_privilege(u.usename, n.nspname, 'USAGE')
WHERE nspname NOT IN ('pg_catalog', 'information_schema', 'pg_toast')
GROUP BY nspname, nspowner;
```

### Sin Foreign Keys Cross-Schema

Las referencias entre schemas usan UUIDs sin constraints de FK:
- ✅ Aislamiento de servicios
- ✅ Cada servicio puede migrar independientemente
- ⚠️ Integridad referencial manejada en aplicación

---

## 📈 Escalabilidad

### Réplicas de Lectura

```
┌─────────────┐     ┌─────────────┐
│   Primary   │────▶│   Replica   │
│  (writes)   │     │  (reads)    │
└─────────────┘     └─────────────┘
       │
       ▼
┌─────────────┐
│   Replica   │
│  (reports)  │
└─────────────┘
```

### Particionamiento Futuro

```sql
-- Cuando sea necesario, particionar por fecha
CREATE TABLE analytics.events (
    -- ...
) PARTITION BY RANGE (created_at);

-- Particionar enrollments por año
CREATE TABLE enrollments.enrollments_2025
    PARTITION OF enrollments.enrollments
    FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
```

---

## 🛠️ Operaciones

### Backup Strategy

```bash
# Backup completo
pg_dump -Fc acc_lms > backup_$(date +%Y%m%d).dump

# Backup por schema (para restauración parcial)
pg_dump -Fc -n auth acc_lms > auth_$(date +%Y%m%d).dump
pg_dump -Fc -n courses acc_lms > courses_$(date +%Y%m%d).dump
```

### Monitoreo

```sql
-- Queries lentas por schema
SELECT schemaname, relname, seq_scan, idx_scan
FROM pg_stat_user_tables
ORDER BY seq_scan DESC
LIMIT 20;

-- Conexiones por usuario de servicio
SELECT usename, count(*)
FROM pg_stat_activity
WHERE usename LIKE '%_svc'
GROUP BY usename;
```
