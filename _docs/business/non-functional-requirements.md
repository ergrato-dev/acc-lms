# ACC LMS — Requisitos No Funcionales (RNF)

**Versión:** 2025-08-08  
**Estado:** MVP definitivo para implementación

---

## RNF-001: Rendimiento y Escalabilidad

### Métricas objetivo

- **Latencia:** <200ms P95 para APIs críticas (auth, courses, enrollments)
- **Throughput:** 1000 RPS por servicio con 2 CPU cores
- **Concurrencia:** 500 usuarios simultáneos en player/checkout
- **Escalabilidad horizontal:** Auto-scaling basado en CPU >70%

### Implementación técnica

```nginx
# nginx.conf - Timeouts optimizados
proxy_connect_timeout 5s;
proxy_send_timeout 10s;
proxy_read_timeout 30s;
keepalive_timeout 65s;
```

---

## RNF-002: Disponibilidad y Resiliencia

### SLOs (Service Level Objectives)

- **Uptime:** 99.5% mensual (downtime máximo 3.6h/mes)
- **Recovery Time:** <5min para fallos de servicio individual
- **Data Recovery:** RPO 1h, RTO 15min para PostgreSQL

### Estrategias de HA

- **Nginx:** 3 instancias con health checks y failover automático
- **PostgreSQL:** Primary + 2 replicas (streaming replication)
- **Redis:** Sentinel con 3 nodos para cache crítico
- **Circuit breaker:** 5 fallos consecutivos → circuit abierto por 30s

```yaml
# docker-compose health check ejemplo
healthcheck:
  test: ['CMD', 'curl', '-f', 'http://localhost:8080/health']
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s
```

---

## RNF-003: Seguridad

### Autenticación/Autorización

- **JWT:** RS256, expiración 15min (access) / 7 días (refresh)
- **Password hashing:** Argon2id, memory=64MB, iterations=3
- **RBAC:** Validación en cada endpoint según claim `role`
- **Rate limiting:** 100 req/min por IP público, 1000 por JWT autenticado

### Protección de datos

- **HTTPS obligatorio** con TLS 1.3 mínimo
- **CORS:** whitelist de dominios específicos
- **Headers de seguridad:** HSTS, CSP, X-Frame-Options
- **Secrets:** rotación cada 90 días (JWT_SECRET, DB passwords)

```nginx
# Security headers
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
```

---

## RNF-004: Observabilidad y Monitoreo

### Logging estructurado

```json
{
  "timestamp": "2025-08-08T10:30:00Z",
  "level": "info",
  "service": "courses-service",
  "correlationId": "req-123e4567-e89b-12d3",
  "traceId": "trace-abc123",
  "userId": "user-456def",
  "method": "POST",
  "path": "/api/v1/courses",
  "statusCode": 201,
  "duration": 150
}
```

### Métricas críticas

- **RED:** Rate, Errors, Duration por endpoint
- **USE:** Utilization, Saturation, Errors por recurso
- **Negocio:** conversion_rate, course_completion_rate, churn_rate

### Alertas automáticas

- Error rate >5% en 5min → Slack/PagerDuty
- Latencia P95 >500ms → Email team
- Disk usage >85% → Auto-cleanup logs older than 7 days

---

## RNF-005: Calidad de Código

### Cobertura y testing

- **Unit tests:** >80% coverage por servicio
- **Integration:** endpoints críticos (auth, payments, enrollments)
- **E2E:** flujos completos (signup→purchase→learn)
- **Performance:** load testing con 500 usuarios simulados

### Linting y formateo

```yaml
# .github/workflows/ci.yml
- name: Quality Gates
  run: |
    # Backend (Rust)
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo audit
    cargo tarpaulin --out Xml
    # Frontend (React/TypeScript)
    pnpm run lint       # ESLint + Prettier
    pnpm run type-check # TypeScript
    # Analysis
    sonar-scanner      # SonarQube analysis
```

### SonarQube quality gates

- **Bugs:** 0 blocker/critical
- **Vulnerabilities:** 0 high/critical
- **Code smells:** <100 total
- **Duplicated lines:** <5%

---

## RNF-006: Datos y Backup

### PostgreSQL

- **Backup diario:** full dump + WAL archiving
- **Retención:** 30 días daily, 12 meses monthly
- **Replicación:** sync a 2 replicas geográficamente separadas
- **Encryption:** TDE (Transparent Data Encryption) en reposo

### MinIO/S3

- **Versioning:** habilitado para content crítico
- **Lifecycle:** videos >1 año → Glacier, >3 años → Deep Archive
- **Cross-region replication:** backup automático a región secundaria

### MongoDB (event_logs, analytics)

- **Sharding:** por date para escalabilidad temporal
- **TTL:** logs >90 días eliminados automáticamente
- **Agregaciones:** pre-cálculo nocturno de KPIs

---

## RNF-007: DevOps y Deploy

### CI/CD Pipeline

```mermaid
graph LR
    A[Git Push] --> B[Build]
    B --> C[Unit Tests]
    C --> D[Integration Tests]
    D --> E[SonarQube]
    E --> F[Security Scan]
    F --> G[Deploy Staging]
    G --> H[E2E Tests]
    H --> I[Deploy Production]
```

### Estrategia de releases

- **Blue-Green deployment** para servicios críticos
- **Feature flags** para A/B testing
- **Database migrations:** backwards compatible, rollback automático
- **Zero downtime:** rolling updates con health checks

### Ambientes

- **Development:** docker-compose local
- **Staging:** K8s/Docker Swarm con datos anonimizados
- **Production:** multi-AZ con load balancing

---

## RNF-008: Usabilidad y UX

### Performance frontend

- **First Contentful Paint:** <1.5s
- **Largest Contentful Paint:** <2.5s
- **Time to Interactive:** <3s
- **Cumulative Layout Shift:** <0.1

### Accesibilidad

- **WCAG 2.1 AA compliance**
- **Keyboard navigation** completa
- **Screen readers** compatibilidad
- **Color contrast ratio** >4.5:1

### Responsive design

- **Mobile-first:** optimizado para 320px+
- **Breakpoints:** 768px (tablet), 1024px (desktop)
- **Touch targets:** mínimo 44px

---

## RNF-009: Escalabilidad de Datos

### Particionamiento

```sql
-- Partición por fecha en event_logs
CREATE TABLE event_logs_2025_08 PARTITION OF event_logs
FOR VALUES FROM ('2025-08-01') TO ('2025-09-01');
```

### Indexación estratégica

- **PostgreSQL:** btree en FKs, gin en JSON fields, partial en boolean filters
- **MongoDB:** compound indexes para queries frecuentes
- **Redis:** apropiada configuración de eviction (allkeys-lru)

### Cache layers

- **L1:** Redis por servicio (session, course metadata)
- **L2:** CDN para static assets (Cloudflare/AWS CloudFront)
- **L3:** Database query result cache (15min TTL)

---

## RNF-010: Costos y Sostenibilidad

### Optimización de recursos

- **Auto-scaling:** scale down en horarios de baja demanda
- **Reserved instances:** 70% capacity con 1-year commitment
- **Spot instances:** para jobs no críticos (analytics, backups)

### Green computing

- **Efficient algorithms:** O(log n) en búsquedas, lazy loading
- **Resource monitoring:** alertas por usage ineficiente
- **Carbon footprint:** preferir DCs con energía renovable

---

## RNF-011: API Design y HATEOAS

### Principios de API Hipermedia

**HATEOAS (Hypermedia as the Engine of Application State)** es obligatorio para:

- Navegación dinámica entre recursos relacionados
- Descubrimiento automático de acciones disponibles
- Evolución de APIs sin breaking changes
- Reducción de acoplamiento frontend-backend

### Implementación HATEOAS

```json
{
  "id": "course-123",
  "title": "Clean Architecture en Microservicios",
  "status": "published",
  "price": 99.99,
  "_links": {
    "self": {
      "href": "/api/v1/courses/course-123"
    },
    "enroll": {
      "href": "/api/v1/enrollments",
      "method": "POST",
      "templated": false
    },
    "modules": {
      "href": "/api/v1/courses/course-123/modules"
    },
    "instructor": {
      "href": "/api/v1/users/instructor-456"
    }
  },
  "_embedded": {
    "instructor": {
      "id": "instructor-456",
      "name": "Tech Expert",
      "_links": {
        "self": { "href": "/api/v1/users/instructor-456" }
      }
    }
  }
}
```

### Estándares de Hipermedia

- **HAL (Hypertext Application Language):** Para responses estructurados
- **JSON-LD:** Para contexto semántico en APIs públicas
- **RFC 5988:** Para rel types estándar (`self`, `next`, `prev`, `edit`)

```typescript
// Ejemplo de implementación TypeScript
interface HATEOASResource {
  _links: {
    [rel: string]: {
      href: string;
      method?: string;
      templated?: boolean;
      title?: string;
    };
  };
  _embedded?: {
    [rel: string]: HATEOASResource | HATEOASResource[];
  };
}
```

---

## RNF-012: API Gateway y Routing (Traefik)

### Traefik como API Gateway

**Traefik** es el reverse proxy y load balancer principal para:

- **Service discovery automático** via Docker labels
- **SSL/TLS termination** con Let's Encrypt automático
- **Load balancing** entre instancias de microservicios
- **Middleware pipeline** para auth, CORS, rate limiting

### Configuración Traefik

```yaml
# docker-compose.traefik.yml
version: '3.8'
services:
  traefik:
    image: traefik:v3.0
    command:
      - '--api.dashboard=true'
      - '--providers.docker=true'
      - '--providers.docker.exposedbydefault=false'
      - '--entrypoints.web.address=:80'
      - '--entrypoints.websecure.address=:443'
      - '--certificatesresolvers.letsencrypt.acme.email=admin@acc-lms.com'
      - '--certificatesresolvers.letsencrypt.acme.storage=/acme.json'
      - '--certificatesresolvers.letsencrypt.acme.httpchallenge.entrypoint=web'
    ports:
      - '80:80'
      - '443:443'
      - '8080:8080' # Dashboard
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./acme.json:/acme.json
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.dashboard.rule=Host(`traefik.localhost`)'
```

### Microservices Labels

```yaml
# Ejemplo: auth-service (Rust/Actix-web)
auth-service:
  build: ./be/auth-service
  labels:
    - 'traefik.enable=true'
    - 'traefik.http.routers.auth.rule=Host(`api.acc-lms.com`) && PathPrefix(`/api/v1/auth`)'
    - 'traefik.http.routers.auth.tls.certresolver=letsencrypt'
    - 'traefik.http.services.auth.loadbalancer.server.port=8080'
    - 'traefik.http.middlewares.auth-cors.headers.accesscontrolalloworigin=*'
    - 'traefik.http.routers.auth.middlewares=auth-cors'
```

### Middleware Pipeline

```yaml
# Middleware para rate limiting
traefik.http.middlewares.api-ratelimit.ratelimit.burst=100
traefik.http.middlewares.api-ratelimit.ratelimit.average=10

# Middleware para JWT validation
traefik.http.middlewares.jwt-auth.forwardauth.address=http://auth-service:8080/validate
traefik.http.middlewares.jwt-auth.forwardauth.authResponseHeaders=X-User-Id,X-User-Role

# Circuit breaker
traefik.http.middlewares.circuit-breaker.circuitbreaker.expression=NetworkErrorRatio() > 0.3
```

### Routing Strategy

```
https://api.acc-lms.com/
├── /api/v1/auth/*          → auth-service (Rust/Actix-web)
├── /api/v1/users/*         → users-service (Rust/Actix-web)
├── /api/v1/courses/*       → courses-service (Rust/Axum)
├── /api/v1/content/*       → content-service (Rust/Actix-web)
├── /api/v1/enrollments/*   → enrollments-service (Rust/Actix-web)
├── /api/v1/payments/*      → payments-service (Rust/Actix-web)
├── /api/v1/analytics/*     → analytics-service (Rust/Axum)
├── /api/v1/ai/*            → ai-service (Rust/Actix-web)
└── /api/v1/notifications/* → notifications-service (Rust/Actix-web)
```

### Health Checks y Service Discovery

```yaml
# Health check obligatorio por servicio (Rust services en puerto 8080)
healthcheck:
  test: ['CMD', 'curl', '-f', 'http://localhost:8080/health']
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s

# Labels para health check
labels:
  - 'traefik.http.services.auth.loadbalancer.healthcheck.path=/health'
  - 'traefik.http.services.auth.loadbalancer.healthcheck.interval=30s'
```

### Métricas Traefik

- **Request rate:** Requests por segundo por servicio
- **Response time:** P50, P90, P95 por endpoint
- **Error rate:** 4xx/5xx ratio
- **Service availability:** Health check success rate

```prometheus
# Métricas exportadas a Prometheus
traefik_service_requests_total
traefik_service_request_duration_seconds
traefik_service_server_up
traefik_config_reloads_total
```

---

## Validación y Testing de RNFs

### Load testing (Artillery/K6)

```javascript
export const options = {
  stages: [
    { duration: '2m', target: 100 }, // Ramp up
    { duration: '5m', target: 500 }, // Stay at 500 users
    { duration: '2m', target: 0 }, // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<200'],
    http_req_failed: ['rate<0.05'],
  },
};
```

### Security testing

- **OWASP ZAP:** scan automático en staging
- **Dependency check:** vulnerabilidades en librerías
- **Penetration testing:** quarterly por terceros

### Compliance audit

- **GDPR readiness:** data portability, right to be forgotten
- **Financial audit:** para payments-service (PCI DSS basic)
- **Code review:** peer review obligatorio, 2 aprobaciones mínimo

---

## RNF-013: Cumplimiento Legal y Protección de Datos

**Referencia completa:** [compliance-requirements.md](./compliance-requirements.md)

### Normativas soportadas

| Jurisdicción     | Normativa                   | Alcance                        |
| ---------------- | --------------------------- | ------------------------------ |
| 🇨🇴 Colombia      | Ley 1581/2012 (Habeas Data) | Todos los usuarios colombianos |
| 🇪🇺 Unión Europea | GDPR (2016/679)             | Residentes UE/EEE              |
| 🇺🇸 California    | CCPA/CPRA                   | Residentes California          |
| 🇧🇷 Brasil        | LGPD (Lei 13.709)           | Residentes Brasil              |
| 🌍 Global        | COPPA                       | Menores de edad                |

### Implementación técnica

```typescript
// Encriptación datos sensibles
interface DataProtection {
  // At rest
  encryption: {
    algorithm: 'AES-256-GCM';
    keyManagement: 'AWS KMS' | 'HashiCorp Vault';
    encryptedFields: [
      'email',
      'phone',
      'address',
      'document_id',
      'payment_tokens'
    ];
  };

  // Passwords
  hashing: {
    algorithm: 'Argon2id';
    memory: 65536; // 64 MB
    iterations: 3;
    parallelism: 4;
  };

  // Tokenización pagos
  paymentData: {
    provider: 'Stripe';
    storedLocally: false;
    pciCompliant: true;
  };
}
```

### Retención de datos

| Tipo de dato                    | Retención                        | Base legal              |
| ------------------------------- | -------------------------------- | ----------------------- |
| Cuenta usuario                  | Mientras activa + 30 días gracia | Contrato                |
| Datos fiscales/facturas         | 10 años                          | Ley tributaria Colombia |
| Transacciones comerciales       | 5 años mínimo                    | Código de comercio      |
| Logs de seguridad               | 6 meses - 2 años                 | Interés legítimo        |
| Logs auditoría datos personales | 2 años                           | Accountability GDPR     |
| Cookies analytics               | 24 meses máximo                  | ePrivacy                |
| Datos marketing (sin actividad) | 2 años                           | Consentimiento          |

### Medidas de seguridad organizativas

- **DPO (Data Protection Officer):** Designado y registrado
- **Capacitación:** Anual obligatoria para empleados con acceso a datos
- **NDAs:** Obligatorios para empleados y contractors
- **Acceso mínimo:** Principio de menor privilegio implementado
- **Evaluaciones de impacto (DPIA):** Para tratamientos de alto riesgo
- **Registro de actividades:** Mantenido según GDPR Art. 30

### Plazos de respuesta legal

| Solicitud           | GDPR     | Habeas Data     | CCPA           | LGPD      |
| ------------------- | -------- | --------------- | -------------- | --------- |
| Respuesta estándar  | 30 días  | 15 días hábiles | 45 días        | 15 días   |
| Extensión máxima    | +60 días | +8 días         | +45 días       | Razonable |
| Notificación brecha | 72 horas | ASAP            | Pronto posible | Razonable |

### Transferencias internacionales

```yaml
# Proveedores con DPA firmado
processors:
  - name: AWS
    location: US (us-east-1)
    mechanism: Standard Contractual Clauses (SCCs)
    dpa_signed: true

  - name: Stripe
    location: US
    mechanism: SCCs
    dpa_signed: true
    pci_compliant: true

  - name: SendGrid
    location: US
    mechanism: SCCs
    dpa_signed: true

  - name: Cloudflare
    location: Global
    mechanism: SCCs
    dpa_signed: true
```

---

## RNF-014: Accesibilidad Web (WCAG 2.1 AA)

### Estándar objetivo

- **Nivel de conformidad:** WCAG 2.1 Nivel AA
- **Plazo cumplimiento:** MVP debe cumplir AA en flujos críticos
- **Auditoría:** Antes de lanzamiento público

### Principios POUR

#### 1. Perceptible

```css
/* Contraste mínimo 4.5:1 para texto normal, 3:1 para texto grande */
:root {
  --text-primary: #1a1a1a; /* Sobre blanco: 16.1:1 */
  --text-secondary: #4a4a4a; /* Sobre blanco: 9.0:1 */
  --text-on-primary: #ffffff; /* Sobre primary: verificar */
  --background: #ffffff;
}

/* Tamaño mínimo de texto */
body {
  font-size: 16px; /* Nunca menor a 16px */
  line-height: 1.5; /* Interlineado adecuado */
}

/* No usar solo color para transmitir información */
.error {
  color: var(--error-red);
  border-left: 4px solid var(--error-red); /* Indicador adicional */
}
.error::before {
  content: '⚠ '; /* Icono adicional */
}
```

```html
<!-- Textos alternativos obligatorios -->
<img
  src="course-thumbnail.jpg"
  alt="Miniatura del curso: Introducción a React" />

<!-- Para imágenes decorativas -->
<img
  src="decoration.svg"
  alt=""
  role="presentation" />

<!-- Subtítulos en videos -->
<video>
  <source
    src="lesson.mp4"
    type="video/mp4" />
  <track
    kind="captions"
    src="lesson-es.vtt"
    srclang="es"
    label="Español" />
  <track
    kind="captions"
    src="lesson-en.vtt"
    srclang="en"
    label="English" />
</video>
```

#### 2. Operable

```typescript
// Navegación por teclado completa
interface KeyboardAccessibility {
  // Todos los interactivos alcanzables con Tab
  tabIndex: number;

  // Atajos de teclado documentados
  shortcuts: {
    Escape: 'Cerrar modal/dropdown';
    'Enter/Space': 'Activar botón/link';
    'Arrow keys': 'Navegar menús/listas';
    'Ctrl+K': 'Abrir búsqueda global';
  };

  // Focus visible obligatorio
  focusIndicator: {
    outline: '2px solid var(--primary)';
    outlineOffset: '2px';
  };
}

// Skip links para navegación rápida
<a
  href="#main-content"
  class="skip-link">
  Saltar al contenido principal
</a>;
```

```css
/* Focus visible - NUNCA ocultar */
*:focus {
  outline: 2px solid var(--focus-color);
  outline-offset: 2px;
}

/* Solo ocultar outline si hay focus-visible */
*:focus:not(:focus-visible) {
  outline: none;
}
*:focus-visible {
  outline: 2px solid var(--focus-color);
  outline-offset: 2px;
}

/* Touch targets mínimo 44x44px */
button,
a,
input[type='checkbox'],
input[type='radio'] {
  min-height: 44px;
  min-width: 44px;
}
```

#### 3. Comprensible

```typescript
// Idioma de página declarado
<html lang="es">

// Idioma de fragmentos específicos
<p>El término <span lang="en">responsive design</span> significa...</p>

// Mensajes de error claros y específicos
interface FormError {
  field: string;
  message: string;          // Mensaje legible, no código
  suggestion?: string;      // Cómo corregir
}

// Ejemplo
{
  field: "email",
  message: "El correo electrónico no tiene un formato válido",
  suggestion: "Asegúrate de incluir @ y un dominio (ej: usuario@ejemplo.com)"
}
```

```html
<!-- Labels asociados a inputs -->
<label for="email">Correo electrónico</label>
<input
  type="email"
  id="email"
  name="email"
  aria-describedby="email-hint email-error" />
<span
  id="email-hint"
  class="hint"
  >Usaremos este email para notificaciones</span
>
<span
  id="email-error"
  class="error"
  role="alert"
  aria-live="polite"></span>

<!-- Campos obligatorios indicados claramente -->
<label for="name">
  Nombre
  <span
    aria-label="requerido"
    class="required"
    >*</span
  >
</label>
```

#### 4. Robusto

```html
<!-- HTML semántico -->
<header role="banner">
  <nav
    role="navigation"
    aria-label="Principal">
    ...
  </nav>
</header>

<main
  role="main"
  id="main-content">
  <article>
    <h1>Título del curso</h1>
    <section aria-labelledby="section-overview">
      <h2 id="section-overview">Descripción general</h2>
      ...
    </section>
  </article>
</main>

<aside
  role="complementary"
  aria-label="Cursos relacionados">
  ...
</aside>

<footer role="contentinfo">...</footer>

<!-- ARIA solo cuando HTML nativo no es suficiente -->
<div
  role="tablist"
  aria-label="Secciones del curso">
  <button
    role="tab"
    aria-selected="true"
    aria-controls="panel-1">
    Contenido
  </button>
  <button
    role="tab"
    aria-selected="false"
    aria-controls="panel-2">
    Recursos
  </button>
</div>
<div
  role="tabpanel"
  id="panel-1"
  aria-labelledby="tab-1">
  ...
</div>
```

### Componentes específicos a auditar

| Componente          | Requisitos                                              |
| ------------------- | ------------------------------------------------------- |
| **Video player**    | Controles por teclado, subtítulos, descripción audio    |
| **Formularios**     | Labels asociados, errores en aria-live, grupos fieldset |
| **Modales**         | Focus trap, Escape cierra, aria-modal                   |
| **Navegación**      | Skip links, aria-current, breadcrumbs                   |
| **Carruseles**      | Pause, controles visibles, anuncio cambio               |
| **Tablas de datos** | Headers th, scope, caption                              |
| **Alertas**         | role="alert", aria-live="polite/assertive"              |
| **Loading states**  | aria-busy, mensajes de progreso                         |

### Testing de accesibilidad

```yaml
# Herramientas obligatorias en CI/CD
tools:
  automated:
    - axe-core # Integrado en tests E2E
    - lighthouse # Auditoría Performance + A11y
    - pa11y # CLI para CI

  manual:
    - NVDA / VoiceOver # Screen readers
    - Keyboard-only testing # Navegación sin mouse
    - Color blindness sim # Simuladores daltonismo

# Criterio de aceptación
thresholds:
  axe_violations: 0 # Cero violaciones críticas
  lighthouse_a11y: 90 # Score mínimo 90/100
```

```javascript
// Test E2E con axe-core
describe('Accessibility', () => {
  it('should have no accessibility violations on homepage', async () => {
    await page.goto('/');
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations).toHaveLength(0);
  });

  it('should be navigable by keyboard', async () => {
    await page.goto('/login');
    await page.keyboard.press('Tab');
    const focused = await page.evaluate(() => document.activeElement.id);
    expect(focused).toBe('email');
  });
});
```

### Declaración de accesibilidad

- **Ubicación:** `/accesibilidad`
- **Contenido:** Estado de conformidad, limitaciones conocidas, contacto
- **Actualización:** Cada release mayor

---

## RNF-015: Internacionalización y Localización (i18n/L10n)

### Idiomas soportados

| Idioma    | Código | Estado         | Cobertura |
| --------- | ------ | -------------- | --------- |
| Español   | es     | ✅ Principal   | 100%      |
| English   | en     | ✅ Secundario  | 100%      |
| Português | pt     | 🟡 Planificado | 80%       |

### Arquitectura i18n

```typescript
// Frontend - React i18next
interface I18nConfig {
  fallbackLng: 'es';
  supportedLngs: ['es', 'en', 'pt'];

  // Lazy loading de traducciones
  backend: {
    loadPath: '/locales/{{lng}}/{{ns}}.json';
  };

  // Namespaces por feature
  ns: ['common', 'auth', 'courses', 'checkout', 'errors'];

  // Detección automática
  detection: {
    order: ['querystring', 'cookie', 'localStorage', 'navigator'];
  };
}

// Estructura de archivos
/locales
  /es
    common.json
    auth.json
    courses.json
  /en
    common.json
    auth.json
    courses.json
```

```json
// /locales/es/auth.json
{
  "login": {
    "title": "Iniciar sesión",
    "email_label": "Correo electrónico",
    "password_label": "Contraseña",
    "submit": "Entrar",
    "forgot_password": "¿Olvidaste tu contraseña?",
    "no_account": "¿No tienes cuenta? {{link}}",
    "register_link": "Regístrate"
  },
  "errors": {
    "invalid_credentials": "Correo o contraseña incorrectos",
    "account_locked": "Cuenta bloqueada temporalmente. Intenta en {{minutes}} minutos.",
    "email_not_verified": "Por favor verifica tu correo electrónico"
  }
}
```

### Backend - Códigos de error

```rust
// Backend NO envía textos de UI, solo códigos
#[derive(Serialize)]
pub struct ApiError {
    pub code: ErrorCode,      // "INVALID_CREDENTIALS"
    pub message: String,      // Mensaje técnico (inglés)
    pub details: Option<Value>,
}

// Frontend traduce según código
const errorMessages = {
  INVALID_CREDENTIALS: t('auth.errors.invalid_credentials'),
  ACCOUNT_LOCKED: t('auth.errors.account_locked', { minutes: data.minutes }),
};
```

### Formatos regionales (L10n)

```typescript
// Configuración por locale
const localeFormats = {
  es: {
    dateFormat: 'DD/MM/YYYY',
    timeFormat: 'HH:mm',
    currency: 'COP',
    numberFormat: {
      decimal: ',',
      thousands: '.',
    },
  },
  en: {
    dateFormat: 'MM/DD/YYYY',
    timeFormat: 'h:mm A',
    currency: 'USD',
    numberFormat: {
      decimal: '.',
      thousands: ',',
    },
  },
};

// Uso con Intl API
const formatDate = (date: Date, locale: string) =>
  new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
  }).format(date);

const formatCurrency = (amount: number, currency: string, locale: string) =>
  new Intl.NumberFormat(locale, {
    style: 'currency',
    currency,
  }).format(amount);
```

### Contenido multiidioma (cursos)

```typescript
// Cursos pueden tener contenido en múltiples idiomas
interface CourseLocalization {
  courseId: string;
  defaultLanguage: string;

  localizations: {
    [locale: string]: {
      title: string;
      description: string;
      subtitles?: string[]; // VTT files
      transcripts?: string[]; // Transcripciones
    };
  };
}
```

---

**Próximos pasos:**

1. Implementar health checks y métricas base
2. Configurar CI/CD con quality gates
3. Setup monitoring stack (Prometheus/Grafana)
4. Definir runbooks para incidents comunes
5. Auditoría de accesibilidad pre-lanzamiento
6. Completar traducciones PT-BR
