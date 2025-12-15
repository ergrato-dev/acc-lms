¡Brutal elección! 🧠🔥
Hagamos **ACC — Aprender Con Criterio** como un **LMS open source** con microservicios en **Rust (Actix-web/Axum)** y un **frontend único (React 19 + Vite + Tailwind)**. Abajo va el **blueprint ejecutable** para arrancar en serio.

---

# 1) Visión de ACC (LMS) y alcance MVP

**Objetivo MVP (8–10 semanas):**

- Vender cursos propios (checkout) y gestionar catálogo.
- Matriculación + progreso básico + evaluaciones simples.
- Panel de instructor (subir contenido, crear evaluaciones).
- Panel de estudiante (ver cursos, lecciones, quizzes).
- Autenticación/roles (student, instructor, admin).
- Pagos (Stripe/MercadoPago, modo sandbox).
- Observabilidad, calidad (SonarQube), resiliencia (Nginx x3), backups DB.
- Internacionalización i18n

---

# 2) Dominios y microservicios (bounded contexts)

### Servicios Core (MVP)

- **auth-service** (AuthN/AuthZ, JWT o PASETO, RBAC)
- **users-service** (perfil, preferencias, billing profile)
- **courses-service** (cursos, lecciones, módulos, tags, prerequisitos)
- **enrollments-service** (matrículas, estado de acceso)
- **content-service** (multimedia en S3/MinIO, firmas presignadas)
- **assignments-service** (quizzes/tareas, intentos, envíos)
- **grades-service** (calificaciones, rúbricas simples, feedback)
- **payments-service** (ordenes, comprobantes, webhooks)
- **notifications-service** (email/push/WhatsApp; plantillas)
- **analytics-service** (KPIs: completion rate, funnel de compra, cohortes)
- **search-service** (catálogo/FAQ usando MongoDB + text index o Elastic)

### Servicios de Soporte y Compliance

- **chatbot-service** (asistente conversacional 24/7, NLU, escalamiento a humano)
- **kb-service** (knowledge base, artículos de ayuda, FAQ categorizado)
- **compliance-service** (GDPR/CCPA/LGPD/Habeas Data, consentimientos, ARCO, portabilidad)
- **subscription-service** (planes de suscripción, billing recurrente, lifecycle)

### Servicios de IA

- **ai-service** (búsqueda semántica, tutor RAG, generación quizzes, embeddings)

---

# 3) Stack tecnológico (Rust monolítico por servicio)

**Backend:** Rust (Actix-web / Axum) + SQLx + PostgreSQL + Redis (cache)

| Servicio              | Framework | DB Principal          | Cache |
| --------------------- | --------- | --------------------- | ----- |
| auth-service          | Actix-web | PostgreSQL            | Redis |
| users-service         | Actix-web | PostgreSQL            | Redis |
| courses-service       | Axum      | PostgreSQL            | Redis |
| content-service       | Actix-web | PostgreSQL + MinIO    | Redis |
| enrollments-service   | Actix-web | PostgreSQL            | Redis |
| assignments-service   | Actix-web | PostgreSQL            | Redis |
| grades-service        | Actix-web | PostgreSQL            | Redis |
| payments-service      | Actix-web | PostgreSQL            | Redis |
| notifications-service | Actix-web | MongoDB               | Redis |
| analytics-service     | Axum      | ClickHouse            | Redis |
| ai-service            | Actix-web | PostgreSQL (pgvector) | Redis |
| chatbot-service       | Actix-web | PostgreSQL + MongoDB  | Redis |
| kb-service            | Actix-web | PostgreSQL            | Redis |
| compliance-service    | Actix-web | PostgreSQL            | Redis |
| subscription-service  | Actix-web | PostgreSQL            | Redis |

**Convenciones (todas en inglés):**

- Rutas REST: `/api/v1/...` kebab-case.
- JSON: camelCase.
- Tablas/campos Postgres: snake_case, plural.
- Env vars: UPPER_SNAKE_CASE.
- Docker services: kebab-case.

---

# 4) Modelo de datos (núcleo MVP, PostgreSQL)

## Tablas principales (snake_case)

```sql
-- users-service
CREATE TABLE users (
  user_id UUID PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  hashed_password TEXT NOT NULL,
  first_name TEXT,
  last_name TEXT,
  role TEXT NOT NULL CHECK (role IN ('student','instructor','admin')),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- courses-service
CREATE TABLE courses (
  course_id UUID PRIMARY KEY,
  owner_id UUID NOT NULL, -- FK users.user_id
  title TEXT NOT NULL,
  slug TEXT UNIQUE NOT NULL,
  description TEXT,
  price_cents INT NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'USD',
  is_published BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE lessons (
  lesson_id UUID PRIMARY KEY,
  course_id UUID NOT NULL,
  title TEXT NOT NULL,
  content_type TEXT NOT NULL CHECK (content_type IN ('video','article','quiz')),
  content_ref TEXT, -- pointer a content-service (s3 key o url)
  order_index INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- enrollments-service
CREATE TABLE enrollments (
  enrollment_id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  course_id UUID NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('active','refunded','revoked')),
  progress_percent INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE(user_id, course_id)
);

-- assignments-service
CREATE TABLE quizzes (
  quiz_id UUID PRIMARY KEY,
  course_id UUID NOT NULL,
  title TEXT NOT NULL,
  total_points INT NOT NULL DEFAULT 100,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE quiz_questions (
  question_id UUID PRIMARY KEY,
  quiz_id UUID NOT NULL,
  question_text TEXT NOT NULL,
  question_type TEXT NOT NULL CHECK (question_type IN ('single','multiple','boolean','code')),
  points INT NOT NULL DEFAULT 5,
  order_index INT NOT NULL
);

-- grades-service
CREATE TABLE submissions (
  submission_id UUID PRIMARY KEY,
  quiz_id UUID NOT NULL,
  user_id UUID NOT NULL,
  score INT NOT NULL DEFAULT 0,
  feedback TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- payments-service
CREATE TABLE orders (
  order_id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  course_id UUID NOT NULL,
  amount_cents INT NOT NULL,
  currency TEXT NOT NULL DEFAULT 'USD',
  status TEXT NOT NULL CHECK (status IN ('pending','paid','failed','refunded')),
  provider TEXT NOT NULL, -- 'stripe' | 'mercadopago'
  provider_ref TEXT,      -- payment_intent / preference_id
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**MongoDB (documentos)**

- `content_assets` (metadatos de video/pdf/markdown).
- `event_logs` (auditoría y analítica cruda).
- `search_index` (si no usamos Elastic).

**Redis**

- `session:*` (si usamos session tokens para admin panel),
- cache de curso/landing y de disponibilidad de contenido,
- rate limiting.

---

# 5) Endpoints MVP (contratos base)

- **auth-service**

  - `POST /api/v1/auth/register`
  - `POST /api/v1/auth/login`
  - `POST /api/v1/auth/refresh`
  - `GET  /api/v1/auth/me`

- **users-service**

  - `GET  /api/v1/users/:id`
  - `PATCH /api/v1/users/:id`

- **courses-service**

  - `GET  /api/v1/courses`
  - `POST /api/v1/courses` (instructor)
  - `GET  /api/v1/courses/:id`
  - `POST /api/v1/courses/:id/lessons`

- **enrollments-service**

  - `POST /api/v1/enrollments` (al pagar)
  - `GET  /api/v1/enrollments/my`
  - `PATCH /api/v1/enrollments/:id/progress`

- **assignments-service**

  - `GET  /api/v1/quizzes/:id`
  - `POST /api/v1/quizzes/:id/submit`

- **grades-service**

  - `GET  /api/v1/grades/my`

- **payments-service**

  - `POST /api/v1/orders`
  - `POST /api/v1/payments/webhook/stripe`
  - `POST /api/v1/payments/webhook/mercadopago`

- **content-service**

  - `POST /api/v1/content/presign-upload`
  - `GET  /api/v1/content/:key/presign-download`

- **chatbot-service**

  - `POST /api/v1/chatbot/sessions` (iniciar sesión de chat)
  - `POST /api/v1/chatbot/messages` (enviar mensaje)
  - `GET  /api/v1/chatbot/suggestions` (sugerencias contextuales)
  - `POST /api/v1/chatbot/escalate` (escalar a humano)
  - `POST /api/v1/chatbot/feedback` (feedback de utilidad)

- **kb-service**

  - `GET  /api/v1/kb/categories` (categorías de KB)
  - `GET  /api/v1/kb/articles` (listar artículos)
  - `GET  /api/v1/kb/articles/:slug` (detalle de artículo)
  - `POST /api/v1/kb/articles/:id/feedback` (votación útil/no útil)
  - `POST /api/v1/kb/search/semantic` (búsqueda semántica)

- **compliance-service**

  - `GET  /api/v1/compliance/legal/:type` (términos, privacidad, cookies)
  - `GET  /api/v1/compliance/consents` (consentimientos del usuario)
  - `PATCH /api/v1/compliance/consents` (actualizar consentimientos)
  - `POST /api/v1/compliance/data-requests` (solicitud ARCO/GDPR)
  - `GET  /api/v1/compliance/data-requests/:id` (estado de solicitud)
  - `POST /api/v1/compliance/export` (portabilidad de datos)
  - `POST /api/v1/compliance/deletion` (derecho al olvido)
  - `POST /api/v1/compliance/ccpa/opt-out` (opt-out venta datos)

- **subscription-service**

  - `GET  /api/v1/subscriptions/plans` (planes disponibles)
  - `POST /api/v1/subscriptions/subscribe` (crear suscripción)
  - `GET  /api/v1/subscriptions/billing` (información de facturación)
  - `POST /api/v1/subscriptions/cancel` (cancelar suscripción)
  - `POST /api/v1/subscriptions/pause` (pausar suscripción)
  - `POST /api/v1/subscriptions/reactivate` (reactivar suscripción)

- **ai-service**

  - `GET  /api/v1/ai/semantic-search` (búsqueda semántica)
  - `POST /api/v1/ai/tutor/sessions` (sesión de tutoría)
  - `POST /api/v1/ai/tutor/messages` (mensaje a tutor)
  - `POST /api/v1/ai/quizzes/generate` (generar quiz con IA)
  - `POST /api/v1/ai/summaries` (resumen de contenido)

---

# 6) Eventos (asincronía y desacoplamiento)

- `course.published` → notificar suscriptores, refrescar search index.
- `order.paid` → crear `enrollment` y enviar recibo.
- `quiz.submitted` → calcular `score`, emitir `grade.updated`.
- `lesson.viewed` → analytics actualiza `progress`.
- `consent.updated` → compliance registra cambio.
- `subscription.created` → activar acceso premium.
- `subscription.canceled` → notificar retención, ajustar acceso.
- `data_request.submitted` → compliance inicia proceso.
- `chat.escalated` → crear ticket de soporte.

Formato (JSON camelCase) en un **topic** tipo `acc.events` (RabbitMQ/Redpanda/Kafka).

---

# 7) Monorepo (o polyrepo) y estructura

```
/acc-platform/
  ├─ /fe/ (React 19 + Vite + Tailwind)
  ├─ /be/
  │   ├─ /auth-service/
  │   ├─ /users-service/
  │   ├─ /courses-service/
  │   ├─ /enrollments-service/
  │   ├─ /assignments-service/
  │   ├─ /grades-service/
  │   ├─ /payments-service/
  │   ├─ /content-service/
  │   ├─ /notifications-service/
  │   ├─ /analytics-service/
  │   ├─ /search-service/
  │   ├─ /ai-service/
  │   ├─ /chatbot-service/
  │   ├─ /kb-service/
  │   ├─ /compliance-service/
  │   └─ /subscription-service/
  ├─ /infra/
  │   ├─ /nginx/
  │   │   └─ nginx.conf
  │   ├─ /docker/
  │   │   └─ docker-compose.yml
  │   └─ /sonarqube/ (sonar configs)
  ├─ /docs/
  └─ /scripts/
```

Cada servicio (Clean Architecture):

```
/<service-name>/
  /docs/
  /deploy/
    /docker/
  /config/
  /migrations/
  /scripts/
  /src/
    /domain/
    /application/
    /infrastructure/
    /interfaces/
  .env.example
  dockerfile
  sonar-project.properties
  README.md
```

---

# 8) Nginx (3 instancias + upstreams + rate limit)

`docker-compose` levantará `nginx-gateway` con 3 réplicas tras un `scale`:

```nginx
# /infra/nginx/nginx.conf
worker_processes auto;
events { worker_connections 1024; }

http {
  map $http_authorization $jwt_sub { default "-"; }

  limit_req_zone $binary_remote_addr zone=rate_ip:10m rate=10r/s;

  upstream auth_service    { server auth-service:8080; }
  upstream users_service   { server users-service:8080; }
  upstream courses_service { server courses-service:8080; }
  upstream payments_service{ server payments-service:8080; }

  server {
    listen 80;
    server_name _;

    location /api/v1/auth/     { proxy_pass http://auth_service;     }
    location /api/v1/users/    { proxy_pass http://users_service;    }
    location /api/v1/courses/  { proxy_pass http://courses_service;  }
    location /api/v1/orders/   { proxy_pass http://payments_service; }

    location / {
      root /usr/share/nginx/html; # build del frontend
      try_files $uri /index.html;
      limit_req zone=rate_ip burst=20 nodelay;
    }

    proxy_set_header X-Request-Id $request_id;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header traceparent $request_id;
    proxy_read_timeout 30s;
  }
}
```

---

# 9) docker compose (HA mínima + DB réplicas + MinIO + Redis + Sonar)

```yaml
# /infra/docker/docker-compose.yml
version: '3.9'
services:
  postgres-primary:
    image: bitnami/postgresql:16
    environment:
      - POSTGRES_PASSWORD=postgres
    ports: ['5432:5432']
    volumes: ['pg_primary:/bitnami/postgresql']

  postgres-replica_1:
    image: bitnami/postgresql:16
    environment:
      - POSTGRES_PASSWORD=postgres
      - POSTGRESQL_REPLICATION_MODE=slave
      - POSTGRESQL_MASTER_HOST=postgres-primary
      - POSTGRESQL_PASSWORD=postgres
    depends_on: [postgres-primary]

  redis:
    image: redis:7-alpine
    ports: ['6379:6379']

  minio:
    image: minio/minio
    command: server /data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=admin
      - MINIO_ROOT_PASSWORD=admin12345
    ports: ['9000:9000', '9001:9001']
    volumes: ['minio_data:/data']

  sonarqube:
    image: sonarqube:community
    ports: ['9002:9000']
    environment:
      - SONAR_ES_BOOTSTRAP_CHECKS_DISABLE=true

  # ejemplo de 2 servicios
  auth-service:
    build: ../../services/auth-service
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@postgres-primary:5432/acc_auth
      - REDIS_URL=redis://redis:6379/0
      - JWT_SECRET=change_me
    depends_on: [postgres-primary, redis]

  courses-service:
    build: ../../services/courses-service
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@postgres-primary:5432/acc_courses
      - REDIS_URL=redis://redis:6379/1
    depends_on: [postgres-primary, redis]

  nginx-gateway:
    image: nginx:1.27-alpine
    volumes:
      - ../nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ../../fe/dist:/usr/share/nginx/html:ro
    ports: ['80:80']
    depends_on: [auth-service, courses-service]

volumes:
  pg_primary:
  minio_data:
```

> Para “3 instancias de Nginx”: `docker compose up --scale nginx-gateway=3` + un **reverse proxy externo** o **Swarm/K8s** para balancear hacia esas réplicas.

---

# 10) Seguridad

- **AuthN**: Access Token (JWT/PASETO) + Refresh Token rotado; `auth-service`.
- **AuthZ**: RBAC (`student`, `instructor`, `admin`) vía middleware en cada servicio.
- **Hardening**: headers HTTP, CORS por origen, rate limit Nginx, CSRF en panel admin si hay cookies.
- **Cumplimiento**: logs de auditoría (`event_logs`), hash de contraseñas (Argon2/BCrypt), rotación de secretos.

---

# 11) Observabilidad

- **Logs** JSON con: `service`, `correlationId`, `traceId`, `spanId`, `userId`.
- **Métricas**: `http_request_duration_seconds`, `db_pool_usage`, `cache_hit_ratio`, KPIs de negocio (conversion, completion).
- **Tracing**: W3C `traceparent`; usar OpenTelemetry SDK en cada stack.

---

# 12) Calidad y CI/CD

- **SonarQube** por servicio (`sonar-project.properties`: `projectKey=acc-<service>-<stack>`).
- **Pipelines** (GitHub Actions):

  - `build` → `lint` → `test` → `sonar` → `migrate` → `deploy`.

- **Testing**

  - `unit`, `integration`, `contract` (Pact), `e2e` (Playwright para frontend).

---

# 13) Pagos y venta de cursos

- **payments-service**: crea `orders` y redirige a checkout.
- Webhooks (`/api/v1/payments/webhook/...`) → valida firma → `order.paid` → `enrollments-service` crea matrícula → `notifications-service` envía recibo.
- Soporte a cupones/descuentos después del MVP.

---

# 14) Frontend único (React 19 + Vite + Tailwind)

- Rutas principales:

  - `/` catálogo
  - `/course/:slug` landing
  - `/learn/:courseId` player (video + notas + progreso)
  - `/instructor/*` gestión
  - `/checkout` compra
  - `/account` perfil

- Estado: React Query para cache de API; feature flags para AB testing.

---

# 15) Multi‑tenancy (opcional futuro)

- **Single-tenant** MVP.
- Evolución a **multi-tenant** (columna `tenant_id` en Postgres, scope en queries, segregación en S3 por prefijo).

---

# 16) Roadmap de implementación (propuesto)

**Sprint 0 (Infra + esqueleto):**

- Monorepo, docker-compose base, Traefik, SonarQube, MinIO, Redis.
- `auth-service` + `users-service` (Rust/Actix-web).
- Frontend scaffold + login/registro.

**Sprint 1 (Catálogo y pagos):**

- `courses-service` (Rust/Axum), `payments-service` (Rust/Actix-web).
- Checkout sandbox + webhooks + `enrollments-service`.

**Sprint 2 (Contenido y aprendizaje):**

- `content-service` (presign URLs MinIO) + `assignments-service` (quizzes) + `grades-service`.
- Player con progreso.

**Sprint 3 (Observabilidad + analytics + hardening):**

- `analytics-service`, dashboards, rate limiting fino, backup jobs.

---

# 17) ¿Qué entrego en el siguiente mensaje?

Puedo generarte **el primer microservicio completo** (siguiendo tus reglas) para arrancar de inmediato:

**Opción A (recomendada):** `auth-service` en **Rust/Actix-web** con:

- Clean Architecture (árbol completo),
- SQLx migrations,
- Endpoints `/api/v1/auth`,
- Dockerfile, compose parcial, Traefik labels,
- Linters (clippy, rustfmt), tests (`unit`, `integration`),
- `sonar-project.properties`,
- Observabilidad básica (tracing + prometheus metrics).

**Opción B:** `users-service` (perfil, preferencias) con CRUD completo y tests.

Exacto 🚀, un **LMS moderno como ACC** sin IA quedaría incompleto.
Hoy la inteligencia artificial no es solo un “plus”, sino un **diferenciador de mercado** en plataformas educativas.
Te propongo un set de **funcionalidades AI-first** que podemos acoplar a ACC desde el MVP y que escalan a features premium, **aprovechando que ya manejas embeddings/vectores en PostgreSQL** (pgvector).

---

## **1. Búsqueda semántica y recomendación de contenido (con pgvector)**

**Cómo funciona:**

- Indexar títulos, descripciones y transcripciones de videos con embeddings (OpenAI, Cohere, HuggingFace).
- Guardar embeddings en columna `embedding VECTOR` en PostgreSQL.
- Consultar con `cosine_similarity` o `vector <->` para búsqueda semántica.
- Recomendaciones “por similitud” de cursos y lecciones.

**Casos de uso en ACC:**

- “Encuentra cursos que respondan a mi necesidad” sin depender de keywords exactos.
- “Recomendados para ti” según historial y progresos.
- Agrupar contenido por afinidad semántica.

---

## **2. Asistente de aprendizaje por curso (AI Tutor)**

**Cómo funciona:**

- Microservicio `tutor-service` que recibe el contexto del curso (syllabus, transcripciones, materiales PDF).
- Usa un LLM para responder dudas del estudiante en tiempo real, con grounding en el contenido del curso (RAG: Retrieval Augmented Generation).
- **Prevención de alucinaciones**: restringir a contenido embebido y curado.

**Casos de uso en ACC:**

- Chat contextual: “Explícame este concepto que no entendí en la lección 3.”
- Sugerencia de ejercicios prácticos adicionales.
- Explicaciones en diferentes niveles (básico, intermedio, avanzado).

---

## **3. Generador automático de evaluaciones y quizzes**

**Cómo funciona:**

- El instructor sube material (PDF, transcripción).
- AI analiza el texto y genera preguntas tipo test, verdadero/falso, código, o ensayo.
- Permite edición antes de publicar.

**Casos de uso en ACC:**

- Ahorra tiempo a instructores.
- Mantiene consistencia en el nivel de dificultad.
- Puede crear bancos de preguntas reutilizables.

---

## **4. Feedback automatizado en envíos de tareas y código**

**Cómo funciona:**

- Para tareas de programación: ejecuta pruebas automáticas + LLM para retroalimentación textual.
- Para ensayos o respuestas largas: AI evalúa claridad, gramática, relevancia y entrega observaciones.
- Puede integrarse con rúbricas personalizadas.

**Casos de uso en ACC:**

- Retroalimentación inmediata 24/7.
- Mejora el aprendizaje autónomo.
- Libera carga del instructor.

---

## **5. Resumen automático de clases y materiales**

**Cómo funciona:**

- Procesa audio/video y genera resúmenes, glosarios y puntos clave.
- Opcional: entregar versión “resumen para repaso” en bullet points.
- Puede exportar a PDF o enviar como email diario/semanal.

**Casos de uso en ACC:**

- Estudiantes que no pueden ver toda la clase.
- Revisión rápida antes de exámenes.
- Glosarios por curso.

---

## **6. Detección de riesgo de abandono (Early Dropout Detection)**

**Cómo funciona:**

- Modelo supervisado (puede entrenarse con histórico).
- Variables: actividad, progreso, entregas, interacción en foros.
- Alertas tempranas para instructores.

**Casos de uso en ACC:**

- Intervención proactiva: enviar mensajes motivacionales.
- Ofrecer tutorías personalizadas a estudiantes en riesgo.
- Reducir churn y aumentar tasa de finalización.

---

## **7. Evaluación automática de calidad de contenido**

**Cómo funciona:**

- AI analiza videos y textos subidos por instructores.
- Sugiere mejoras en claridad, tono, ejemplos y estructura.
- Detecta inconsistencias o temas repetidos.

**Casos de uso en ACC:**

- Mantener calidad alta en marketplace de cursos.
- Guía para nuevos instructores.

---

## **8. Traducción y adaptación de contenido (Multilenguaje)**

**Cómo funciona:**

- Traduce transcripciones y materiales a múltiples idiomas.
- Adapta expresiones culturales según el país del estudiante.
- Permite accesibilidad global.

**Casos de uso en ACC:**

- Expandir mercado fuera de LATAM.
- Inclusión de estudiantes con diferentes idiomas nativos.

---

## **Arquitectura sugerida para IA en ACC**

- **ai-service** como microservicio dedicado:

  - API para generación de embeddings y consultas pgvector.
  - API para interacción con LLM (con RAG sobre contenido del curso).
  - API para generación de quizzes y resúmenes.

- **Pipeline de preprocesamiento**:

  - Extracción de texto (videos → transcripción → embeddings).
  - Indexación en PostgreSQL + almacenamiento de archivos en MinIO.

- **Estrategia de costos**:

  - Llamadas a LLM en batch para materiales nuevos.
  - Cache de respuestas frecuentes en Redis.

---
