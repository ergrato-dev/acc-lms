# ACC LMS — Requisitos No Funcionales (RNF)

> **Versión:** 1.2.0  
> **Última actualización:** 2024-12-14  
> **Estado:** Definitivo para implementación MVP  
> **Stack:** React 19+ + Vite (FE) | Rust stable (BE) | PostgreSQL 17+ | Redis stable | Nginx stable  
> **Containerización:** Docker + Docker Compose (dev & prod)  
> **Infraestructura:** Nginx (LB) + Traefik (API GW)

---

## Convenciones de Documentación

### Diagramas

> **📐 Preferencia: SVG sobre ASCII**
>
> Para diagramas de arquitectura, flujos y topologías, preferimos **SVG** sobre ASCII art:
>
> - **SVG:** Escalable, profesional, editable, accesible (alt text), versionable en Git
> - **ASCII:** Solo para contexto rápido en comentarios de código o READMEs simples
>
> **Herramientas recomendadas:**
>
> - [Excalidraw](https://excalidraw.com/) → Export SVG (hand-drawn style)
> - [draw.io/diagrams.net](https://draw.io/) → Export SVG (formal diagrams)
> - [Mermaid](https://mermaid.js.org/) → Render como SVG (code-first)
>
> **Ubicación:** `/_assets/diagrams/{categoria}/{nombre}.svg`
>
> Los diagramas ASCII en este documento son **temporales** hasta que se generen los SVG correspondientes.

---

## Tabla de Contenidos

1. [Rendimiento y Escalabilidad](#rnf-01-rendimiento-y-escalabilidad)
2. [Disponibilidad y Resiliencia](#rnf-02-disponibilidad-y-resiliencia)
   - [Alta Disponibilidad y Fault Tolerance](#rnf-024-arquitectura-de-alta-disponibilidad)
3. [Seguridad](#rnf-03-seguridad)
4. [Observabilidad y Monitoreo](#rnf-04-observabilidad-y-monitoreo)
5. [Calidad de Código](#rnf-05-calidad-de-código)
6. [Datos y Backup](#rnf-06-datos-y-backup)
7. [DevOps y Deploy](#rnf-07-devops-y-deploy)
8. [Usabilidad y Accesibilidad](#rnf-08-usabilidad-y-accesibilidad)
9. [Escalabilidad de Datos](#rnf-09-escalabilidad-de-datos)
10. [API Design y HATEOAS](#rnf-10-api-design-y-hateoas)
11. [API Gateway (Traefik)](#rnf-11-api-gateway-traefik)
12. [Internacionalización](#rnf-12-internacionalización)
13. [Costos y Sostenibilidad](#rnf-13-costos-y-sostenibilidad)
14. [Compliance y Legal](#rnf-14-compliance-y-legal)
15. [**Seguridad — Modelo "Assume Breach"**](#rnf-15-seguridad--modelo-assume-breach) ⚠️
    - [Zero Trust](#rnf-151-principios-de-zero-trust)
    - [Protección de Datos](#rnf-152-protección-de-datos-de-usuarios)
    - [Detección y Respuesta](#rnf-153-detección-y-respuesta-a-incidentes)
    - [Segmentación](#rnf-154-segmentación-y-contención)
    - [Incident Response Playbook](#rnf-156-incident-response-playbook-resumen)
16. [Matriz de Trazabilidad](#matriz-de-trazabilidad-rnf--rf)

---

## Arquitectura de Infraestructura

```
┌────────────────────────────────────────────────────────────────┐
│                         INTERNET                               │
└────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│              CDN (Cloudflare) - Static Assets                  │
└────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│          NGINX x3 (Load Balancer + Frontend Server)            │
│     - SSL Termination, Rate Limiting, Static Files (SPA)       │
│     - Keepalived VIP para failover automático                  │
└────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│              TRAEFIK x2 (API Gateway + Service Discovery)      │
│     - Routing dinámico, Circuit Breaker, JWT Validation        │
└────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│                   MICROSERVICES (Rust x2 each)                 │
│     - Stateless, Auto-scaling, Health Checks                   │
└────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│                        DATA LAYER                              │
│   PostgreSQL (1+2) │ Redis Sentinel │ MongoDB RS │ MinIO x4   │
└────────────────────────────────────────────────────────────────┘
```

---

## Containerización: Docker + Docker Compose

> **🐳 Estrategia de Containerización**
>
> Todo el stack de ACC LMS se ejecuta en contenedores Docker, orquestados con Docker Compose.
> Esto garantiza:
>
> - Entornos reproducibles (dev = staging ≈ prod)
> - Despliegue simplificado
> - Aislamiento de servicios
> - Escalabilidad horizontal

### Política de Versiones de Imágenes

| Componente         | Imagen                                | Política de Versión | Justificación                |
| ------------------ | ------------------------------------- | ------------------- | ---------------------------- |
| **Frontend**       | `node:22-alpine`                      | LTS (pares)         | Soporte largo, estabilidad   |
| **Backend (Rust)** | `rust:1-slim-bookworm`                | **Latest Stable**   | Nunca beta/nightly           |
| **PostgreSQL**     | `postgres:17-alpine`                  | **17+**             | Features JSON, performance   |
| **Redis**          | `redis:alpine`                        | **Latest Stable**   | Nunca beta                   |
| **MongoDB**        | `mongo:7`                             | Latest stable minor | Document TTL, Change Streams |
| **ClickHouse**     | `clickhouse/clickhouse-server:latest` | Latest stable       | Analytics optimizations      |
| **Nginx**          | `nginx:stable-alpine`                 | **Latest Stable**   | Nunca mainline               |
| **Traefik**        | `traefik:v3.2`                        | Latest v3.x stable  | API Gateway features         |
| **MinIO**          | `minio/minio:latest`                  | Latest stable       | S3 compatibility             |

> ⚠️ **IMPORTANTE:** Nunca usar versiones `beta`, `rc`, `nightly`, o `mainline` en producción.
> Siempre `stable`, `alpine` cuando disponible (menor tamaño), y versiones LTS para Node.js.

### Archivos Docker del Proyecto

```
acc-lms/
├── docker-compose.yml          # Desarrollo local
├── docker-compose.prod.yml     # Producción (con réplicas y limits)
├── .env.example                # Variables de entorno template
├── be/
│   └── Dockerfile              # Multi-stage: chef → planner → builder → dev → prod
├── fe/
│   ├── Dockerfile              # Multi-stage: base → dev → build → prod (nginx)
│   └── nginx.conf              # Config nginx para SPA
└── infra/
    ├── nginx/
    │   └── nginx.conf          # Load balancer principal
    ├── traefik/
    │   ├── traefik.yml         # Config estática
    │   └── dynamic/
    │       └── middlewares.yml # Rate limiting, CORS, security headers
    ├── postgres/
    │   └── postgresql.conf     # Tuning para LMS workload
    └── clickhouse/
        └── config.xml          # Configuración analytics
```

### Comandos de Desarrollo

```bash
# Iniciar todo el stack de desarrollo
docker compose up -d

# Ver logs de todos los servicios
docker compose logs -f

# Ver logs de un servicio específico
docker compose logs -f svc-auth

# Reconstruir después de cambios en Dockerfile
docker compose build --no-cache svc-auth

# Detener todo
docker compose down

# Detener y eliminar volúmenes (CUIDADO: borra datos)
docker compose down -v

# Ejecutar migraciones de base de datos
docker compose exec postgres psql -U acc -d acc_lms -f /docker-entrypoint-initdb.d/001_initial_schema.sql
```

### Comandos de Producción

```bash
# Desplegar en producción
docker compose -f docker-compose.prod.yml up -d

# Escalar un servicio
docker compose -f docker-compose.prod.yml up -d --scale svc-courses=3

# Actualizar un servicio sin downtime
docker compose -f docker-compose.prod.yml up -d --no-deps --build svc-auth

# Backup de base de datos
docker compose exec postgres pg_dump -U acc acc_lms > backup_$(date +%Y%m%d).sql
```

### Multi-Stage Build (Rust Backend)

```dockerfile
# Stages del Dockerfile de backend:
# 1. chef     - Instala cargo-chef para caching de dependencias
# 2. planner  - Genera recipe.json
# 3. builder  - Compila dependencias (cached) + aplicación
# 4. development - Imagen completa con cargo-watch para hot reload
# 5. production  - Imagen mínima (~50MB) solo con el binario
```

**Tamaños de imagen resultantes:**
| Stage | Tamaño | Uso |
|-------|--------|-----|
| development | ~1.5GB | Dev local con hot reload |
| production | ~50-80MB | Deploy a producción |

### Health Checks

Todos los servicios incluyen health checks para Docker y orquestadores:

```yaml
healthcheck:
  test: ['CMD', 'curl', '-f', 'http://localhost:8080/health']
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 60s # Tiempo para que Rust compile en dev
```

### Networks

| Network        | Tipo                      | Servicios                | Propósito       |
| -------------- | ------------------------- | ------------------------ | --------------- |
| `acc-frontend` | bridge                    | nginx, traefik, frontend | Tráfico público |
| `acc-backend`  | bridge (internal en prod) | microservices, databases | Tráfico interno |

### Volumes

| Volume            | Servicio   | Datos                   |
| ----------------- | ---------- | ----------------------- |
| `postgres_data`   | PostgreSQL | Base de datos principal |
| `redis_data`      | Redis      | Cache y sesiones        |
| `mongodb_data`    | MongoDB    | Notificaciones          |
| `clickhouse_data` | ClickHouse | Analytics               |
| `minio_data`      | MinIO      | Archivos y videos       |

---

## Convenciones

| Prioridad | Descripción                   |
| --------- | ----------------------------- |
| **P0**    | Crítico - Bloquea lanzamiento |
| **P1**    | Alto - Requerido para MVP     |
| **P2**    | Medio - Post-MVP              |
| **P3**    | Bajo - Nice to have           |

---

## RNF-01: Rendimiento y Escalabilidad

### RNF-01.1: Latencia de APIs

| ID         | Requisito                                                           | Prioridad | Métrica              |
| ---------- | ------------------------------------------------------------------- | --------- | -------------------- |
| RNF-01.1.1 | APIs críticas (auth, courses, enrollments) responden en <200ms P95  | P0        | Prometheus histogram |
| RNF-01.1.2 | APIs secundarias (analytics, notifications) responden en <500ms P95 | P1        | Prometheus histogram |
| RNF-01.1.3 | Streaming de video: buffer inicial <3s                              | P1        | Player metrics       |
| RNF-01.1.4 | Búsqueda full-text: <300ms para queries típicas                     | P1        | Query timing         |

### RNF-01.2: Throughput

| ID         | Requisito                              | Prioridad | Métrica         |
| ---------- | -------------------------------------- | --------- | --------------- |
| RNF-01.2.1 | 1,000 RPS por servicio con 2 CPU cores | P0        | Load test K6    |
| RNF-01.2.2 | 5,000 RPS agregado en API Gateway      | P1        | Traefik metrics |
| RNF-01.2.3 | 100 uploads concurrentes de video      | P2        | MinIO metrics   |

### RNF-01.3: Concurrencia

| ID         | Requisito                                | Prioridad | Métrica               |
| ---------- | ---------------------------------------- | --------- | --------------------- |
| RNF-01.3.1 | 500 usuarios simultáneos en video player | P0        | WebSocket connections |
| RNF-01.3.2 | 200 usuarios simultáneos en checkout     | P0        | Session tracking      |
| RNF-01.3.3 | 1,000 conexiones WebSocket activas       | P1        | Connection pool       |

### RNF-01.4: Escalabilidad

| ID         | Requisito                                        | Prioridad | Métrica          |
| ---------- | ------------------------------------------------ | --------- | ---------------- |
| RNF-01.4.1 | Auto-scaling horizontal cuando CPU >70%          | P0        | K8s HPA          |
| RNF-01.4.2 | Scale-down automático en baja demanda (<20% CPU) | P1        | K8s HPA          |
| RNF-01.4.3 | Escalado de réplicas de lectura PostgreSQL       | P2        | Read replica lag |

### Implementación Rust

```rust
// Configuración de pool de conexiones (sqlx)
let pool = PgPoolOptions::new()
    .max_connections(100)
    .min_connections(10)
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(600))
    .connect(&database_url)
    .await?;

// Timeouts en Actix-web
HttpServer::new(|| {
    App::new()
        .wrap(middleware::Timeout::new(Duration::from_secs(30)))
})
.keep_alive(Duration::from_secs(75))
.client_request_timeout(Duration::from_secs(60))
```

---

## RNF-02: Disponibilidad y Resiliencia

### RNF-02.1: SLOs (Service Level Objectives)

| ID         | Requisito                                           | Prioridad | Métrica       |
| ---------- | --------------------------------------------------- | --------- | ------------- |
| RNF-02.1.1 | Uptime 99.5% mensual (máx 3.6h downtime/mes)        | P0        | Uptime robot  |
| RNF-02.1.2 | Uptime 99.9% para auth-service                      | P0        | Health checks |
| RNF-02.1.3 | Uptime 99.0% para analytics (degradación permitida) | P2        | Health checks |

### RNF-02.2: Recovery

| ID         | Requisito                                           | Prioridad | Métrica               |
| ---------- | --------------------------------------------------- | --------- | --------------------- |
| RNF-02.2.1 | RTO (Recovery Time Objective): <15min               | P0        | Disaster drill        |
| RNF-02.2.2 | RPO (Recovery Point Objective): <1h para PostgreSQL | P0        | WAL archiving         |
| RNF-02.2.3 | RPO <24h para MongoDB analytics                     | P2        | Backup frequency      |
| RNF-02.2.4 | Failover automático de servicios en <30s            | P1        | Health check interval |

### RNF-02.3: Resiliencia

| ID         | Requisito                                              | Prioridad | Métrica       |
| ---------- | ------------------------------------------------------ | --------- | ------------- |
| RNF-02.3.1 | Circuit breaker: apertura tras 5 fallos consecutivos   | P0        | Circuit state |
| RNF-02.3.2 | Retry con backoff exponencial (max 3 intentos)         | P0        | Retry count   |
| RNF-02.3.3 | Graceful degradation: funcionalidad core sin analytics | P1        | Feature flags |
| RNF-02.3.4 | Bulkhead: aislamiento de fallos entre servicios        | P1        | Thread pools  |

### Implementación Rust

```rust
// Circuit breaker pattern
use governor::{Quota, RateLimiter};

pub struct CircuitBreaker {
    failure_count: AtomicU32,
    state: AtomicU8, // 0=Closed, 1=Open, 2=HalfOpen
    last_failure: AtomicU64,
    threshold: u32,
    timeout_ms: u64,
}

impl CircuitBreaker {
    pub fn can_execute(&self) -> bool {
        match self.state.load(Ordering::Relaxed) {
            0 => true, // Closed
            1 => { // Open - check timeout
                let elapsed = now_ms() - self.last_failure.load(Ordering::Relaxed);
                if elapsed > self.timeout_ms {
                    self.state.store(2, Ordering::Relaxed); // HalfOpen
                    true
                } else {
                    false
                }
            }
            2 => true, // HalfOpen - allow one request
            _ => false,
        }
    }
}
```

### Health Check Endpoint

```rust
// Health check con dependencias
#[get("/health")]
async fn health_check(
    db: web::Data<PgPool>,
    redis: web::Data<RedisPool>,
) -> impl Responder {
    let db_ok = sqlx::query("SELECT 1").fetch_one(db.get_ref()).await.is_ok();
    let redis_ok = redis.get_ref().ping().await.is_ok();

    let status = if db_ok && redis_ok { "healthy" } else { "degraded" };
    let code = if db_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };

    HttpResponse::build(code).json(json!({
        "status": status,
        "checks": {
            "database": db_ok,
            "cache": redis_ok
        },
        "timestamp": Utc::now().to_rfc3339()
    }))
}
```

### RNF-02.4: Arquitectura de Alta Disponibilidad

#### Topología de Red (Fault Tolerance)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           INTERNET / CDN (Cloudflare)                       │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    LOAD BALANCER (Nginx x3 - Keepalived VIP)                │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                      │
│  │  nginx-01   │    │  nginx-02   │    │  nginx-03   │  ← Health monitored  │
│  │  (active)   │◄──►│  (standby)  │◄──►│  (standby)  │                      │
│  └─────────────┘    └─────────────┘    └─────────────┘                      │
│         │ Virtual IP: 10.0.0.100 (failover automático)                      │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         API GATEWAY (Traefik x2)                            │
│  ┌─────────────────────┐         ┌─────────────────────┐                    │
│  │    traefik-01       │◄───────►│    traefik-02       │                    │
│  │  (leader/active)    │         │  (follower/standby) │                    │
│  └─────────────────────┘         └─────────────────────┘                    │
│         │ Service Discovery: Docker Labels / Consul                         │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
          ┌───────────────────────────┼───────────────────────────┐
          ▼                           ▼                           ▼
┌─────────────────────┐   ┌─────────────────────┐   ┌─────────────────────┐
│  MICROSERVICES      │   │  MICROSERVICES      │   │  MICROSERVICES      │
│  (Rust/Actix-web)   │   │  (Rust/Axum)        │   │  (Rust/Actix-web)   │
│  ┌───────────────┐  │   │  ┌───────────────┐  │   │  ┌───────────────┐  │
│  │ auth-svc x2   │  │   │  │ courses-svc x2│  │   │  │ payments-svc x2│ │
│  │ users-svc x2  │  │   │  │ analytics x2  │  │   │  │ content-svc x2 │ │
│  │ enroll-svc x2 │  │   │  │ search-svc x2 │  │   │  │ ai-svc x2      │ │
│  └───────────────┘  │   │  └───────────────┘  │   │  └───────────────┘  │
└─────────────────────┘   └─────────────────────┘   └─────────────────────┘
          │                           │                           │
          └───────────────────────────┼───────────────────────────┘
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              DATA LAYER                                     │
│  ┌─────────────────────────┐  ┌─────────────────────────┐                   │
│  │     PostgreSQL HA       │  │      Redis Sentinel     │                   │
│  │  ┌─────────┐            │  │  ┌─────────┐            │                   │
│  │  │ Primary │◄──sync────►│  │  │ Master  │◄──────────►│                   │
│  │  └────┬────┘            │  │  └────┬────┘            │                   │
│  │       │                 │  │       │                 │                   │
│  │  ┌────┴────┐ ┌────────┐ │  │  ┌────┴────┐ ┌────────┐ │                   │
│  │  │Replica 1│ │Replica 2│ │  │  │ Slave 1 │ │ Slave 2│ │                   │
│  │  └─────────┘ └─────────┘ │  │  └─────────┘ └────────┘ │                   │
│  └─────────────────────────┘  └─────────────────────────┘                   │
│                                                                             │
│  ┌─────────────────────────┐  ┌─────────────────────────┐                   │
│  │   MongoDB Replica Set   │  │   ClickHouse Cluster    │                   │
│  │  ┌───────┐ ┌───────┐    │  │  ┌───────┐ ┌───────┐    │                   │
│  │  │Primary│ │Second.│    │  │  │Shard 1│ │Shard 2│    │                   │
│  │  └───────┘ └───────┘    │  │  └───────┘ └───────┘    │                   │
│  │            ┌───────┐    │  │            ┌───────┐    │                   │
│  │            │Arbiter│    │  │            │Replica│    │                   │
│  │            └───────┘    │  │            └───────┘    │                   │
│  └─────────────────────────┘  └─────────────────────────┘                   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    MinIO (S3-compatible) Cluster                     │    │
│  │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐                             │    │
│  │  │Node 1 │ │Node 2 │ │Node 3 │ │Node 4 │  ← Erasure coding (4 nodes) │    │
│  │  └───────┘ └───────┘ └───────┘ └───────┘                             │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Matriz de Replicación por Componente

| Componente                | Mínimo Réplicas | Producción | Estrategia                   | Failover |
| ------------------------- | --------------- | ---------- | ---------------------------- | -------- |
| **Nginx**                 | 2               | 3          | Active-Standby (Keepalived)  | <5s      |
| **Traefik**               | 2               | 2          | Active-Standby               | <10s     |
| **Auth Service**          | 2               | 3          | Active-Active (stateless)    | Instant  |
| **Users Service**         | 2               | 2          | Active-Active (stateless)    | Instant  |
| **Courses Service**       | 2               | 3          | Active-Active (stateless)    | Instant  |
| **Content Service**       | 2               | 2          | Active-Active (stateless)    | Instant  |
| **Payments Service**      | 2               | 2          | Active-Active (stateless)    | Instant  |
| **Enrollments Service**   | 2               | 2          | Active-Active (stateless)    | Instant  |
| **Analytics Service**     | 1               | 2          | Active-Active (degradable)   | 30s      |
| **AI Service**            | 1               | 2          | Active-Active (degradable)   | 30s      |
| **Notifications Service** | 2               | 2          | Active-Active                | Instant  |
| **PostgreSQL**            | 1+1             | 1+2        | Primary + Sync Replicas      | <30s     |
| **Redis**                 | 3               | 3          | Sentinel (Master + 2 Slaves) | <15s     |
| **MongoDB**               | 3               | 3          | Replica Set (PSA)            | <10s     |
| **ClickHouse**            | 1               | 2          | Replicated Tables            | <60s     |
| **MinIO**                 | 4               | 4          | Erasure Coding               | Instant  |

#### Nginx como Frontend Server y Load Balancer

```nginx
# /infra/nginx/nginx.conf - Configuración HA
worker_processes auto;
worker_rlimit_nofile 65535;
pid /var/run/nginx.pid;

events {
    worker_connections 4096;
    use epoll;
    multi_accept on;
}

http {
    # Logging
    log_format json_combined escape=json '{'
        '"time":"$time_iso8601",'
        '"remote_addr":"$remote_addr",'
        '"request":"$request",'
        '"status":$status,'
        '"body_bytes_sent":$body_bytes_sent,'
        '"request_time":$request_time,'
        '"upstream_response_time":"$upstream_response_time",'
        '"request_id":"$request_id"'
    '}';
    access_log /var/log/nginx/access.log json_combined;

    # Performance
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    keepalive_requests 1000;

    # Gzip
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml application/json application/javascript;

    # Rate limiting zones
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=100r/s;
    limit_req_zone $binary_remote_addr zone=auth_limit:10m rate=10r/s;
    limit_conn_zone $binary_remote_addr zone=conn_limit:10m;

    # Upstream to Traefik (API Gateway)
    upstream traefik_api {
        least_conn;
        server traefik-01:80 weight=5 max_fails=3 fail_timeout=30s;
        server traefik-02:80 weight=5 max_fails=3 fail_timeout=30s backup;
        keepalive 32;
    }

    # Frontend server (React SPA)
    server {
        listen 80;
        listen [::]:80;
        server_name acc-lms.com www.acc-lms.com;

        # Redirect to HTTPS
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl http2;
        listen [::]:443 ssl http2;
        server_name acc-lms.com www.acc-lms.com;

        # SSL
        ssl_certificate /etc/nginx/ssl/acc-lms.com.crt;
        ssl_certificate_key /etc/nginx/ssl/acc-lms.com.key;
        ssl_session_timeout 1d;
        ssl_session_cache shared:SSL:50m;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
        ssl_prefer_server_ciphers off;

        # Security headers
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;

        # Connection limits
        limit_conn conn_limit 50;

        # API routes → Traefik
        location /api/ {
            limit_req zone=api_limit burst=50 nodelay;

            proxy_pass http://traefik_api;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_set_header X-Request-ID $request_id;
            proxy_set_header Connection "";

            proxy_connect_timeout 5s;
            proxy_send_timeout 60s;
            proxy_read_timeout 60s;

            proxy_next_upstream error timeout http_502 http_503 http_504;
            proxy_next_upstream_tries 2;
        }

        # Auth endpoints (stricter rate limit)
        location /api/v1/auth/ {
            limit_req zone=auth_limit burst=5 nodelay;

            proxy_pass http://traefik_api;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_set_header X-Request-ID $request_id;
        }

        # WebSocket support (real-time notifications)
        location /ws/ {
            proxy_pass http://traefik_api;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_read_timeout 3600s;
        }

        # Frontend SPA (React build)
        location / {
            root /usr/share/nginx/html;
            index index.html;
            try_files $uri $uri/ /index.html;

            # Cache static assets
            location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
                expires 1y;
                add_header Cache-Control "public, immutable";
            }
        }

        # Health check endpoint
        location /health {
            access_log off;
            return 200 '{"status":"healthy","service":"nginx"}';
            add_header Content-Type application/json;
        }
    }
}
```

#### Keepalived para Nginx HA (Virtual IP Failover)

```conf
# /etc/keepalived/keepalived.conf (nginx-01 - MASTER)
vrrp_script check_nginx {
    script "/usr/bin/curl -sf http://localhost/health"
    interval 2
    weight -20
    fall 3
    rise 2
}

vrrp_instance VI_1 {
    state MASTER
    interface eth0
    virtual_router_id 51
    priority 100
    advert_int 1

    authentication {
        auth_type PASS
        auth_pass acc_lms_ha
    }

    virtual_ipaddress {
        10.0.0.100/24
    }

    track_script {
        check_nginx
    }

    notify_master "/etc/keepalived/notify.sh master"
    notify_backup "/etc/keepalived/notify.sh backup"
    notify_fault  "/etc/keepalived/notify.sh fault"
}
```

```conf
# /etc/keepalived/keepalived.conf (nginx-02 - BACKUP)
vrrp_instance VI_1 {
    state BACKUP
    interface eth0
    virtual_router_id 51
    priority 90
    advert_int 1

    authentication {
        auth_type PASS
        auth_pass acc_lms_ha
    }

    virtual_ipaddress {
        10.0.0.100/24
    }

    track_script {
        check_nginx
    }
}
```

#### Docker Compose HA

```yaml
# docker-compose.ha.yml
version: '3.9'

services:
  # ============= NGINX LOAD BALANCERS =============
  nginx-01:
    image: nginx:alpine
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
      - ./fe/dist:/usr/share/nginx/html:ro
    networks:
      - frontend
      - backend
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
    healthcheck:
      test: ['CMD', 'curl', '-f', 'http://localhost/health']
      interval: 10s
      timeout: 5s
      retries: 3
    restart: always

  nginx-02:
    image: nginx:alpine
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
      - ./fe/dist:/usr/share/nginx/html:ro
    networks:
      - frontend
      - backend
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
    healthcheck:
      test: ['CMD', 'curl', '-f', 'http://localhost/health']
      interval: 10s
      timeout: 5s
      retries: 3
    restart: always

  nginx-03:
    image: nginx:alpine
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
      - ./fe/dist:/usr/share/nginx/html:ro
    networks:
      - frontend
      - backend
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
    healthcheck:
      test: ['CMD', 'curl', '-f', 'http://localhost/health']
      interval: 10s
      timeout: 5s
      retries: 3
    restart: always

  # ============= TRAEFIK API GATEWAYS =============
  traefik-01:
    image: traefik:v3.0
    command:
      - '--api.dashboard=true'
      - '--providers.docker=true'
      - '--providers.docker.exposedbydefault=false'
      - '--entrypoints.web.address=:80'
      - '--metrics.prometheus=true'
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    networks:
      - backend
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
    restart: always

  traefik-02:
    image: traefik:v3.0
    command:
      - '--api.dashboard=true'
      - '--providers.docker=true'
      - '--providers.docker.exposedbydefault=false'
      - '--entrypoints.web.address=:80'
      - '--metrics.prometheus=true'
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    networks:
      - backend
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
    restart: always

  # ============= MICROSERVICES (x2 each) =============
  auth-service-01:
    build: ./be/auth-service
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@postgres-primary:5432/acc_lms
      - REDIS_URL=redis://redis-master:6379
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.auth.rule=PathPrefix(`/api/v1/auth`)'
      - 'traefik.http.services.auth.loadbalancer.server.port=8080'
    networks:
      - backend
    deploy:
      replicas: 2
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
    restart: always
    depends_on:
      - postgres-primary
      - redis-master

  # ... (similar pattern for all services)

  # ============= POSTGRESQL HA =============
  postgres-primary:
    image: bitnami/postgresql:16
    environment:
      - POSTGRESQL_REPLICATION_MODE=master
      - POSTGRESQL_REPLICATION_USER=repl_user
      - POSTGRESQL_REPLICATION_PASSWORD=repl_password
      - POSTGRESQL_USERNAME=postgres
      - POSTGRESQL_PASSWORD=postgres
      - POSTGRESQL_DATABASE=acc_lms
    volumes:
      - pg_primary_data:/bitnami/postgresql
    networks:
      - backend
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
    restart: always

  postgres-replica-01:
    image: bitnami/postgresql:16
    environment:
      - POSTGRESQL_REPLICATION_MODE=slave
      - POSTGRESQL_REPLICATION_USER=repl_user
      - POSTGRESQL_REPLICATION_PASSWORD=repl_password
      - POSTGRESQL_MASTER_HOST=postgres-primary
      - POSTGRESQL_PASSWORD=postgres
    volumes:
      - pg_replica_01_data:/bitnami/postgresql
    networks:
      - backend
    depends_on:
      - postgres-primary
    restart: always

  postgres-replica-02:
    image: bitnami/postgresql:16
    environment:
      - POSTGRESQL_REPLICATION_MODE=slave
      - POSTGRESQL_REPLICATION_USER=repl_user
      - POSTGRESQL_REPLICATION_PASSWORD=repl_password
      - POSTGRESQL_MASTER_HOST=postgres-primary
      - POSTGRESQL_PASSWORD=postgres
    volumes:
      - pg_replica_02_data:/bitnami/postgresql
    networks:
      - backend
    depends_on:
      - postgres-primary
    restart: always

  # ============= REDIS SENTINEL =============
  redis-master:
    image: bitnami/redis:7.2
    environment:
      - REDIS_REPLICATION_MODE=master
      - REDIS_PASSWORD=redis_password
    volumes:
      - redis_master_data:/bitnami/redis/data
    networks:
      - backend
    restart: always

  redis-slave-01:
    image: bitnami/redis:7.2
    environment:
      - REDIS_REPLICATION_MODE=slave
      - REDIS_MASTER_HOST=redis-master
      - REDIS_MASTER_PASSWORD=redis_password
      - REDIS_PASSWORD=redis_password
    networks:
      - backend
    depends_on:
      - redis-master
    restart: always

  redis-slave-02:
    image: bitnami/redis:7.2
    environment:
      - REDIS_REPLICATION_MODE=slave
      - REDIS_MASTER_HOST=redis-master
      - REDIS_MASTER_PASSWORD=redis_password
      - REDIS_PASSWORD=redis_password
    networks:
      - backend
    depends_on:
      - redis-master
    restart: always

  redis-sentinel-01:
    image: bitnami/redis-sentinel:7.2
    environment:
      - REDIS_MASTER_HOST=redis-master
      - REDIS_MASTER_PASSWORD=redis_password
      - REDIS_SENTINEL_QUORUM=2
    networks:
      - backend
    depends_on:
      - redis-master
      - redis-slave-01
      - redis-slave-02
    restart: always

  redis-sentinel-02:
    image: bitnami/redis-sentinel:7.2
    environment:
      - REDIS_MASTER_HOST=redis-master
      - REDIS_MASTER_PASSWORD=redis_password
      - REDIS_SENTINEL_QUORUM=2
    networks:
      - backend
    restart: always

  redis-sentinel-03:
    image: bitnami/redis-sentinel:7.2
    environment:
      - REDIS_MASTER_HOST=redis-master
      - REDIS_MASTER_PASSWORD=redis_password
      - REDIS_SENTINEL_QUORUM=2
    networks:
      - backend
    restart: always

  # ============= MONGODB REPLICA SET =============
  mongo-primary:
    image: bitnami/mongodb:7.0
    environment:
      - MONGODB_REPLICA_SET_MODE=primary
      - MONGODB_REPLICA_SET_NAME=rs0
      - MONGODB_REPLICA_SET_KEY=replicasetkey123
      - MONGODB_ROOT_PASSWORD=mongodb_password
    volumes:
      - mongo_primary_data:/bitnami/mongodb
    networks:
      - backend
    restart: always

  mongo-secondary:
    image: bitnami/mongodb:7.0
    environment:
      - MONGODB_REPLICA_SET_MODE=secondary
      - MONGODB_REPLICA_SET_NAME=rs0
      - MONGODB_REPLICA_SET_KEY=replicasetkey123
      - MONGODB_INITIAL_PRIMARY_HOST=mongo-primary
      - MONGODB_INITIAL_PRIMARY_ROOT_PASSWORD=mongodb_password
    volumes:
      - mongo_secondary_data:/bitnami/mongodb
    networks:
      - backend
    depends_on:
      - mongo-primary
    restart: always

  mongo-arbiter:
    image: bitnami/mongodb:7.0
    environment:
      - MONGODB_REPLICA_SET_MODE=arbiter
      - MONGODB_REPLICA_SET_NAME=rs0
      - MONGODB_REPLICA_SET_KEY=replicasetkey123
      - MONGODB_INITIAL_PRIMARY_HOST=mongo-primary
      - MONGODB_INITIAL_PRIMARY_ROOT_PASSWORD=mongodb_password
    networks:
      - backend
    depends_on:
      - mongo-primary
    restart: always

  # ============= MINIO CLUSTER =============
  minio-01:
    image: minio/minio
    command: server http://minio-{01...04}/data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=minio_admin
      - MINIO_ROOT_PASSWORD=minio_password
    volumes:
      - minio_01_data:/data
    networks:
      - backend
    restart: always

  minio-02:
    image: minio/minio
    command: server http://minio-{01...04}/data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=minio_admin
      - MINIO_ROOT_PASSWORD=minio_password
    volumes:
      - minio_02_data:/data
    networks:
      - backend
    restart: always

  minio-03:
    image: minio/minio
    command: server http://minio-{01...04}/data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=minio_admin
      - MINIO_ROOT_PASSWORD=minio_password
    volumes:
      - minio_03_data:/data
    networks:
      - backend
    restart: always

  minio-04:
    image: minio/minio
    command: server http://minio-{01...04}/data --console-address ":9001"
    environment:
      - MINIO_ROOT_USER=minio_admin
      - MINIO_ROOT_PASSWORD=minio_password
    volumes:
      - minio_04_data:/data
    networks:
      - backend
    restart: always

networks:
  frontend:
    driver: bridge
  backend:
    driver: bridge

volumes:
  pg_primary_data:
  pg_replica_01_data:
  pg_replica_02_data:
  redis_master_data:
  mongo_primary_data:
  mongo_secondary_data:
  minio_01_data:
  minio_02_data:
  minio_03_data:
  minio_04_data:
```

---

## RNF-03: Seguridad

### RNF-03.1: Autenticación

| ID         | Requisito                              | Prioridad | Métrica          |
| ---------- | -------------------------------------- | --------- | ---------------- |
| RNF-03.1.1 | JWT con algoritmo RS256 (asymmetric)   | P0        | Token validation |
| RNF-03.1.2 | Access token expira en 15 minutos      | P0        | Token claims     |
| RNF-03.1.3 | Refresh token expira en 7 días         | P0        | Token claims     |
| RNF-03.1.4 | Rotación de refresh token en cada uso  | P0        | Token rotation   |
| RNF-03.1.5 | Blacklist de tokens revocados en Redis | P0        | Redis TTL        |

### RNF-03.2: Contraseñas

| ID         | Requisito                                                                       | Prioridad | Métrica       |
| ---------- | ------------------------------------------------------------------------------- | --------- | ------------- |
| RNF-03.2.1 | Hash con Argon2id (memory=64MB, iterations=3, parallelism=4)                    | P0        | Hash timing   |
| RNF-03.2.2 | Mínimo 10 caracteres, 1 mayúscula, 1 minúscula, 1 número, 1 símbolo (!@#$%^&\*) | P0        | Validation    |
| RNF-03.2.3 | Verificación contra lista de contraseñas comprometidas (HaveIBeenPwned)         | P1        | API check     |
| RNF-03.2.4 | Bloqueo tras 5 intentos fallidos (lockout 15 min)                               | P0        | Rate limiting |

### RNF-03.3: Autorización

| ID         | Requisito                                             | Prioridad | Métrica       |
| ---------- | ----------------------------------------------------- | --------- | ------------- |
| RNF-03.3.1 | RBAC con roles: anonymous, student, instructor, admin | P0        | Role claims   |
| RNF-03.3.2 | Validación de permisos en cada endpoint               | P0        | Middleware    |
| RNF-03.3.3 | Ownership validation para recursos propios            | P0        | Query filters |
| RNF-03.3.4 | Audit log de acciones administrativas                 | P0        | Event logging |

### RNF-03.4: Protección de Datos

| ID         | Requisito                                 | Prioridad | Métrica          |
| ---------- | ----------------------------------------- | --------- | ---------------- |
| RNF-03.4.1 | HTTPS obligatorio con TLS 1.3             | P0        | SSL Labs A+      |
| RNF-03.4.2 | Encryption at rest para datos sensibles   | P0        | DB encryption    |
| RNF-03.4.3 | PII hasheado/cifrado (email, phone)       | P1        | Field encryption |
| RNF-03.4.4 | Secrets en vault (no en código/env files) | P0        | Secret scanning  |

### RNF-03.5: Rate Limiting

| ID         | Requisito                                         | Prioridad | Métrica         |
| ---------- | ------------------------------------------------- | --------- | --------------- |
| RNF-03.5.1 | 100 req/min por IP (endpoints públicos)           | P0        | Traefik metrics |
| RNF-03.5.2 | 1,000 req/min por JWT autenticado                 | P0        | Traefik metrics |
| RNF-03.5.3 | 10 req/min para login/register (anti-brute force) | P0        | Auth service    |
| RNF-03.5.4 | 5 req/min para password reset                     | P0        | Auth service    |

### RNF-03.6: Headers de Seguridad

| ID         | Requisito                                      | Prioridad | Métrica         |
| ---------- | ---------------------------------------------- | --------- | --------------- |
| RNF-03.6.1 | Strict-Transport-Security (HSTS) max-age 1 año | P0        | Header check    |
| RNF-03.6.2 | Content-Security-Policy restrictiva            | P0        | CSP report      |
| RNF-03.6.3 | X-Frame-Options: SAMEORIGIN                    | P0        | Header check    |
| RNF-03.6.4 | X-Content-Type-Options: nosniff                | P0        | Header check    |
| RNF-03.6.5 | CORS whitelist de dominios específicos         | P0        | Preflight check |

### Implementación Rust

```rust
// Argon2id password hashing
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(65536, 3, 4, None)? // 64MB, 3 iterations, 4 parallelism
    );
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

// JWT validation middleware
pub async fn jwt_auth(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(PUBLIC_KEY)?,
        &Validation::new(Algorithm::RS256)
    )?;

    // Check blacklist
    if redis.exists(format!("blacklist:{}", claims.jti)).await? {
        return Err(AuthError::TokenRevoked);
    }

    req.extensions_mut().insert(claims.claims);
    Ok(req)
}
```

---

## RNF-04: Observabilidad y Monitoreo

### RNF-04.1: Logging

| ID         | Requisito                                              | Prioridad | Métrica          |
| ---------- | ------------------------------------------------------ | --------- | ---------------- |
| RNF-04.1.1 | Logs estructurados en JSON                             | P0        | Log format       |
| RNF-04.1.2 | Correlation ID en todas las requests                   | P0        | Trace sampling   |
| RNF-04.1.3 | Niveles: ERROR, WARN, INFO, DEBUG, TRACE               | P0        | Log levels       |
| RNF-04.1.4 | Retención: 7 días hot, 30 días cold, 1 año archive     | P1        | Log lifecycle    |
| RNF-04.1.5 | No logging de datos sensibles (passwords, tokens, PII) | P0        | Log sanitization |

### RNF-04.2: Métricas

| ID         | Requisito                                                | Prioridad | Métrica        |
| ---------- | -------------------------------------------------------- | --------- | -------------- |
| RNF-04.2.1 | RED metrics: Rate, Errors, Duration por endpoint         | P0        | Prometheus     |
| RNF-04.2.2 | USE metrics: Utilization, Saturation, Errors por recurso | P0        | Prometheus     |
| RNF-04.2.3 | Business metrics: enrollments, completions, revenue      | P1        | Custom metrics |
| RNF-04.2.4 | SLI/SLO dashboards en Grafana                            | P1        | Grafana        |

### RNF-04.3: Tracing

| ID         | Requisito                                 | Prioridad | Métrica           |
| ---------- | ----------------------------------------- | --------- | ----------------- |
| RNF-04.3.1 | Distributed tracing con OpenTelemetry     | P1        | Jaeger/Tempo      |
| RNF-04.3.2 | Trace context propagation entre servicios | P1        | W3C Trace Context |
| RNF-04.3.3 | Sampling rate 10% en producción           | P1        | Sample config     |

### RNF-04.4: Alertas

| ID         | Requisito                            | Prioridad | Umbral   |
| ---------- | ------------------------------------ | --------- | -------- |
| RNF-04.4.1 | Error rate >5% en 5 min → PagerDuty  | P0        | Critical |
| RNF-04.4.2 | Latencia P95 >500ms en 5 min → Slack | P0        | Warning  |
| RNF-04.4.3 | CPU >85% sostenido 10 min → Slack    | P1        | Warning  |
| RNF-04.4.4 | Disk >85% → Auto-cleanup + Alert     | P1        | Warning  |
| RNF-04.4.5 | Service down >1 min → PagerDuty      | P0        | Critical |

### Implementación Rust

```rust
// Structured logging con tracing
use tracing::{info, instrument, Span};
use tracing_subscriber::fmt::format::JsonFields;

#[instrument(
    skip(db, payload),
    fields(
        correlation_id = %correlation_id,
        user_id = %user_id,
        endpoint = "POST /api/v1/enrollments"
    )
)]
pub async fn create_enrollment(
    db: web::Data<PgPool>,
    payload: web::Json<CreateEnrollmentRequest>,
    correlation_id: CorrelationId,
    user_id: UserId,
) -> Result<HttpResponse, Error> {
    info!(
        course_id = %payload.course_id,
        "Creating enrollment"
    );

    let enrollment = enrollment_service::create(&db, &payload, user_id).await?;

    info!(
        enrollment_id = %enrollment.id,
        "Enrollment created successfully"
    );

    Ok(HttpResponse::Created().json(enrollment))
}

// Log output format
{
    "timestamp": "2024-12-14T10:30:00.000Z",
    "level": "INFO",
    "target": "acc_lms::enrollments",
    "correlation_id": "req-123e4567-e89b-12d3",
    "user_id": "usr-456def",
    "endpoint": "POST /api/v1/enrollments",
    "course_id": "crs-789abc",
    "message": "Creating enrollment"
}
```

### Prometheus Metrics

```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "http_requests_total",
        "Total HTTP requests"
    ).unwrap();

    static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds",
        vec![0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    ).unwrap();
}
```

---

## RNF-05: Calidad de Código

### RNF-05.1: Testing

| ID         | Requisito                                 | Prioridad | Métrica         |
| ---------- | ----------------------------------------- | --------- | --------------- |
| RNF-05.1.1 | Cobertura de unit tests ≥80%              | P0        | cargo tarpaulin |
| RNF-05.1.2 | Integration tests para endpoints críticos | P0        | Test count      |
| RNF-05.1.3 | E2E tests para flujos principales         | P1        | Playwright      |
| RNF-05.1.4 | Load testing: 500 usuarios simulados      | P1        | K6 reports      |
| RNF-05.1.5 | Contract testing entre servicios          | P2        | Pact            |

### RNF-05.2: Code Quality

| ID         | Requisito                                 | Prioridad | Métrica   |
| ---------- | ----------------------------------------- | --------- | --------- |
| RNF-05.2.1 | Rust: clippy sin warnings (deny warnings) | P0        | CI gate   |
| RNF-05.2.2 | Rust: rustfmt formatting obligatorio      | P0        | CI gate   |
| RNF-05.2.3 | TypeScript: ESLint + Prettier sin errores | P0        | CI gate   |
| RNF-05.2.4 | Código duplicado <5%                      | P1        | SonarQube |
| RNF-05.2.5 | Complejidad ciclomática <15 por función   | P1        | SonarQube |

### RNF-05.3: Security Scanning

| ID         | Requisito                                      | Prioridad | Métrica      |
| ---------- | ---------------------------------------------- | --------- | ------------ |
| RNF-05.3.1 | cargo audit: 0 vulnerabilidades críticas/altas | P0        | CI gate      |
| RNF-05.3.2 | npm audit: 0 vulnerabilidades críticas/altas   | P0        | CI gate      |
| RNF-05.3.3 | SAST con SonarQube en cada PR                  | P0        | Quality gate |
| RNF-05.3.4 | DAST con OWASP ZAP en staging                  | P1        | Scan report  |
| RNF-05.3.5 | Secret scanning en commits                     | P0        | CI gate      |

### RNF-05.4: Code Review

| ID         | Requisito                                | Prioridad | Métrica           |
| ---------- | ---------------------------------------- | --------- | ----------------- |
| RNF-05.4.1 | Mínimo 1 aprobación para merge a develop | P0        | Branch protection |
| RNF-05.4.2 | Mínimo 2 aprobaciones para merge a main  | P0        | Branch protection |
| RNF-05.4.3 | CI debe pasar antes de merge             | P0        | Branch protection |
| RNF-05.4.4 | PR size máximo: 400 líneas de código     | P1        | PR template       |

### Quality Gates CI

```yaml
# .github/workflows/ci.yml (ejemplo)
quality-gates:
  runs-on: ubuntu-latest
  steps:
    - name: Rust Quality
      run: |
        cargo fmt --check
        cargo clippy -- -D warnings
        cargo audit
        cargo tarpaulin --out Xml --fail-under 80

    - name: Frontend Quality
      working-directory: ./fe
      run: |
        pnpm run lint
        pnpm run type-check
        pnpm run test:coverage
        pnpm audit --audit-level=high

    - name: SonarQube Scan
      uses: sonarqube/sonarcloud-github-action@master
      env:
        SONAR_TOKEN: ${{ secrets.SONAR_TOKEN }}
```

---

## RNF-06: Datos y Backup

### RNF-06.1: PostgreSQL

| ID         | Requisito                                  | Prioridad | Métrica         |
| ---------- | ------------------------------------------ | --------- | --------------- |
| RNF-06.1.1 | Backup full diario con pg_dump             | P0        | Backup job      |
| RNF-06.1.2 | WAL archiving continuo para PITR           | P0        | WAL lag         |
| RNF-06.1.3 | Retención: 30 días daily, 12 meses monthly | P0        | Backup count    |
| RNF-06.1.4 | Streaming replication a 1 réplica          | P1        | Replication lag |
| RNF-06.1.5 | Encryption at rest (TDE)                   | P0        | DB config       |
| RNF-06.1.6 | Backup testing mensual (restore drill)     | P1        | Drill report    |

### RNF-06.2: MongoDB

| ID         | Requisito                           | Prioridad | Métrica         |
| ---------- | ----------------------------------- | --------- | --------------- |
| RNF-06.2.1 | Backup diario con mongodump         | P1        | Backup job      |
| RNF-06.2.2 | TTL index: logs >90 días eliminados | P0        | Collection size |
| RNF-06.2.3 | Retención: 30 días                  | P1        | Backup count    |

### RNF-06.3: ClickHouse

| ID         | Requisito                                     | Prioridad | Métrica         |
| ---------- | --------------------------------------------- | --------- | --------------- |
| RNF-06.3.1 | Backup semanal de tablas analytics            | P2        | Backup job      |
| RNF-06.3.2 | Particionamiento por mes                      | P1        | Partition count |
| RNF-06.3.3 | Retención: datos >2 años → aggregate + delete | P2        | Data lifecycle  |

### RNF-06.4: Redis

| ID         | Requisito                        | Prioridad | Métrica         |
| ---------- | -------------------------------- | --------- | --------------- |
| RNF-06.4.1 | RDB snapshots cada 15 minutos    | P1        | Snapshot timing |
| RNF-06.4.2 | AOF persistence para durabilidad | P1        | AOF config      |
| RNF-06.4.3 | Eviction policy: allkeys-lru     | P0        | Memory config   |
| RNF-06.4.4 | Max memory: 70% del disponible   | P0        | Memory limit    |

### RNF-06.5: MinIO/S3

| ID         | Requisito                          | Prioridad | Métrica         |
| ---------- | ---------------------------------- | --------- | --------------- |
| RNF-06.5.1 | Versioning habilitado para content | P1        | Bucket config   |
| RNF-06.5.2 | Lifecycle: videos >1 año → Glacier | P2        | Lifecycle rules |
| RNF-06.5.3 | Replicación cross-region           | P2        | Replication lag |

### Backup Script

```bash
#!/bin/bash
# db/scripts/backup-postgresql.sh

set -euo pipefail

BACKUP_DIR="/backups/postgresql"
DATE=$(date +%Y%m%d_%H%M%S)
RETENTION_DAYS=30

# Full backup
pg_dump -Fc -h $PGHOST -U $PGUSER $PGDATABASE > "$BACKUP_DIR/full_$DATE.dump"

# Verify backup
pg_restore --list "$BACKUP_DIR/full_$DATE.dump" > /dev/null

# Upload to S3
aws s3 cp "$BACKUP_DIR/full_$DATE.dump" "s3://acc-lms-backups/postgresql/full_$DATE.dump"

# Cleanup old backups
find $BACKUP_DIR -name "full_*.dump" -mtime +$RETENTION_DAYS -delete

echo "Backup completed: full_$DATE.dump"
```

---

## RNF-07: DevOps y Deploy

### RNF-07.1: CI/CD Pipeline

| ID         | Requisito                                       | Prioridad | Métrica           |
| ---------- | ----------------------------------------------- | --------- | ----------------- |
| RNF-07.1.1 | Build time <10 minutos                          | P1        | Pipeline duration |
| RNF-07.1.2 | Deploy to staging automático en merge a develop | P0        | Pipeline trigger  |
| RNF-07.1.3 | Deploy to production manual con approval        | P0        | Deployment gate   |
| RNF-07.1.4 | Rollback automático si health check falla       | P0        | Deployment status |

### RNF-07.2: Estrategia de Deploy

| ID         | Requisito                                      | Prioridad | Métrica          |
| ---------- | ---------------------------------------------- | --------- | ---------------- |
| RNF-07.2.1 | Blue-Green deployment para servicios stateless | P1        | Deployment type  |
| RNF-07.2.2 | Rolling updates con maxUnavailable=1           | P0        | K8s config       |
| RNF-07.2.3 | Zero downtime deployments                      | P0        | Downtime metrics |
| RNF-07.2.4 | Feature flags para releases graduales          | P1        | Flag config      |

### RNF-07.3: Database Migrations

| ID         | Requisito                                 | Prioridad | Métrica          |
| ---------- | ----------------------------------------- | --------- | ---------------- |
| RNF-07.3.1 | Migrations backwards compatible           | P0        | Migration script |
| RNF-07.3.2 | Rollback script para cada migration       | P0        | Rollback tested  |
| RNF-07.3.3 | Migrations en transacción donde posible   | P0        | Transaction wrap |
| RNF-07.3.4 | Separar schema changes de data migrations | P1        | Migration type   |

### RNF-07.4: Environments

| ID         | Requisito  | Prioridad | Descripción                   |
| ---------- | ---------- | --------- | ----------------------------- |
| RNF-07.4.1 | Local      | P0        | Docker Compose, hot reload    |
| RNF-07.4.2 | Staging    | P0        | K8s, datos anonimizados       |
| RNF-07.4.3 | Production | P0        | K8s multi-AZ, full monitoring |

### CI/CD Pipeline Diagram

```
┌─────────┐    ┌─────────┐    ┌──────────┐    ┌──────────┐
│  Push   │───▶│  Build  │───▶│  Test    │───▶│  Scan    │
│  to PR  │    │  & Lint │    │  (Unit)  │    │ (SAST)   │
└─────────┘    └─────────┘    └──────────┘    └──────────┘
                                                    │
                    ┌───────────────────────────────┘
                    ▼
              ┌──────────┐    ┌──────────┐    ┌──────────┐
              │  Deploy  │───▶│  Test    │───▶│  Approve │
              │ Staging  │    │  (E2E)   │    │  (Gate)  │
              └──────────┘    └──────────┘    └──────────┘
                                                    │
                    ┌───────────────────────────────┘
                    ▼
              ┌──────────┐    ┌──────────┐    ┌──────────┐
              │  Deploy  │───▶│  Health  │───▶│  Monitor │
              │   Prod   │    │  Check   │    │   SLOs   │
              └──────────┘    └──────────┘    └──────────┘
```

---

## RNF-08: Usabilidad y Accesibilidad

### RNF-08.1: Performance Frontend

| ID         | Requisito                            | Prioridad | Métrica    |
| ---------- | ------------------------------------ | --------- | ---------- |
| RNF-08.1.1 | First Contentful Paint (FCP) <1.5s   | P0        | Lighthouse |
| RNF-08.1.2 | Largest Contentful Paint (LCP) <2.5s | P0        | Lighthouse |
| RNF-08.1.3 | Time to Interactive (TTI) <3.5s      | P0        | Lighthouse |
| RNF-08.1.4 | Cumulative Layout Shift (CLS) <0.1   | P0        | Lighthouse |
| RNF-08.1.5 | First Input Delay (FID) <100ms       | P0        | Lighthouse |
| RNF-08.1.6 | Lighthouse Performance Score ≥90     | P1        | Lighthouse |

### RNF-08.2: Accesibilidad

| ID         | Requisito                                           | Prioridad | Métrica          |
| ---------- | --------------------------------------------------- | --------- | ---------------- |
| RNF-08.2.1 | WCAG 2.1 AA compliance                              | P0        | Axe audit        |
| RNF-08.2.2 | Navegación completa por teclado                     | P0        | Manual test      |
| RNF-08.2.3 | Compatibilidad con screen readers (NVDA, VoiceOver) | P0        | Manual test      |
| RNF-08.2.4 | Color contrast ratio ≥4.5:1 (texto normal)          | P0        | Contrast checker |
| RNF-08.2.5 | Color contrast ratio ≥3:1 (texto grande)            | P0        | Contrast checker |
| RNF-08.2.6 | Focus visible en todos los elementos interactivos   | P0        | Visual test      |
| RNF-08.2.7 | Alt text en todas las imágenes                      | P0        | Automated scan   |
| RNF-08.2.8 | ARIA labels en componentes custom                   | P0        | Axe audit        |

### RNF-08.3: Responsive Design

| ID         | Requisito                                                     | Prioridad | Métrica     |
| ---------- | ------------------------------------------------------------- | --------- | ----------- |
| RNF-08.3.1 | Mobile-first: optimizado para 320px+                          | P0        | Visual test |
| RNF-08.3.2 | Breakpoints: 640px (sm), 768px (md), 1024px (lg), 1280px (xl) | P0        | CSS         |
| RNF-08.3.3 | Touch targets mínimo 44x44px                                  | P0        | Size audit  |
| RNF-08.3.4 | No horizontal scroll en mobile                                | P0        | Visual test |

### RNF-08.4: UX General

| ID         | Requisito                                    | Prioridad | Métrica          |
| ---------- | -------------------------------------------- | --------- | ---------------- |
| RNF-08.4.1 | Feedback visual en acciones (<100ms)         | P0        | Interaction      |
| RNF-08.4.2 | Loading states para operaciones >300ms       | P0        | UI patterns      |
| RNF-08.4.3 | Error messages claros y accionables          | P0        | Copy review      |
| RNF-08.4.4 | Undo/confirmación para acciones destructivas | P0        | UI patterns      |
| RNF-08.4.5 | Persistencia de estado en navegación         | P1        | State management |

### Implementación React

```tsx
// Componente accesible con Tailwind
const Button: React.FC<ButtonProps> = ({
  children,
  onClick,
  disabled,
  loading,
  ariaLabel,
}) => (
  <button
    onClick={onClick}
    disabled={disabled || loading}
    aria-label={ariaLabel}
    aria-busy={loading}
    className={cn(
      'min-h-[44px] min-w-[44px] px-4 py-2',
      'rounded-lg font-medium',
      'focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2',
      'transition-colors duration-150',
      disabled && 'opacity-50 cursor-not-allowed',
      loading && 'cursor-wait'
    )}>
    {loading ? (
      <span className="flex items-center gap-2">
        <Spinner aria-hidden="true" />
        <span>Cargando...</span>
      </span>
    ) : (
      children
    )}
  </button>
);
```

---

## RNF-09: Escalabilidad de Datos

### RNF-09.1: Particionamiento

| ID         | Requisito                             | Prioridad | Métrica         |
| ---------- | ------------------------------------- | --------- | --------------- |
| RNF-09.1.1 | event_logs particionado por mes       | P0        | Partition count |
| RNF-09.1.2 | analytics_events particionado por mes | P0        | Partition count |
| RNF-09.1.3 | Partition pruning en queries          | P0        | Query plan      |
| RNF-09.1.4 | Automated partition creation          | P1        | Cron job        |

### RNF-09.2: Indexación

| ID         | Requisito                                | Prioridad | Métrica              |
| ---------- | ---------------------------------------- | --------- | -------------------- |
| RNF-09.2.1 | B-tree indexes en foreign keys           | P0        | Index coverage       |
| RNF-09.2.2 | GIN indexes en campos JSONB              | P1        | Query performance    |
| RNF-09.2.3 | Partial indexes para boolean filters     | P1        | Index size           |
| RNF-09.2.4 | Compound indexes para queries frecuentes | P0        | Query analysis       |
| RNF-09.2.5 | Index usage monitoring                   | P1        | pg_stat_user_indexes |

### RNF-09.3: Cache Strategy

| ID         | Requisito                             | Prioridad | Nivel       |
| ---------- | ------------------------------------- | --------- | ----------- |
| RNF-09.3.1 | L1: Redis - sessions, course metadata | P0        | Application |
| RNF-09.3.2 | L2: CDN - static assets, videos       | P0        | Edge        |
| RNF-09.3.3 | L3: Database query cache (15min TTL)  | P1        | Database    |
| RNF-09.3.4 | Cache invalidation on write           | P0        | Application |

### RNF-09.4: Query Optimization

| ID         | Requisito                            | Prioridad | Métrica         |
| ---------- | ------------------------------------ | --------- | --------------- |
| RNF-09.4.1 | Queries <100ms en P95                | P0        | Query timing    |
| RNF-09.4.2 | No N+1 queries                       | P0        | Query count     |
| RNF-09.4.3 | Connection pooling (min 10, max 100) | P0        | Pool stats      |
| RNF-09.4.4 | Prepared statements                  | P0        | Statement cache |

### Implementación SQL

```sql
-- Particionamiento por fecha
CREATE TABLE event_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(50) NOT NULL,
    user_id UUID REFERENCES users(id),
    payload JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Partición automática mensual
CREATE TABLE event_logs_2024_12 PARTITION OF event_logs
    FOR VALUES FROM ('2024-12-01') TO ('2025-01-01');

-- Índices optimizados
CREATE INDEX CONCURRENTLY idx_event_logs_user_type
    ON event_logs (user_id, event_type);
CREATE INDEX CONCURRENTLY idx_event_logs_payload
    ON event_logs USING GIN (payload);

-- Partial index para queries frecuentes
CREATE INDEX CONCURRENTLY idx_courses_published
    ON courses (instructor_id) WHERE status = 'published';
```

---

## RNF-10: API Design y HATEOAS

### RNF-10.1: RESTful Design

| ID         | Requisito                                                       | Prioridad | Métrica        |
| ---------- | --------------------------------------------------------------- | --------- | -------------- |
| RNF-10.1.1 | Versionado: /api/v1/ prefix                                     | P0        | URL structure  |
| RNF-10.1.2 | HTTP methods semánticos (GET, POST, PUT, PATCH, DELETE)         | P0        | API spec       |
| RNF-10.1.3 | Status codes correctos (200, 201, 204, 400, 401, 403, 404, 500) | P0        | Response codes |
| RNF-10.1.4 | Recursos en plural (/courses, /users)                           | P0        | URL structure  |
| RNF-10.1.5 | Filtering, sorting, pagination consistentes                     | P0        | Query params   |

### RNF-10.2: HATEOAS

| ID         | Requisito                      | Prioridad | Métrica            |
| ---------- | ------------------------------ | --------- | ------------------ |
| RNF-10.2.1 | \_links en todas las responses | P1        | Response structure |
| RNF-10.2.2 | Self link obligatorio          | P1        | Link presence      |
| RNF-10.2.3 | Related resources como links   | P1        | Link coverage      |
| RNF-10.2.4 | Action links con method hint   | P2        | Link metadata      |

### RNF-10.3: Error Responses

| ID         | Requisito                           | Prioridad | Métrica         |
| ---------- | ----------------------------------- | --------- | --------------- |
| RNF-10.3.1 | Formato RFC 7807 (Problem Details)  | P0        | Response format |
| RNF-10.3.2 | Error codes únicos y documentados   | P0        | Error catalog   |
| RNF-10.3.3 | Mensajes user-friendly (i18n ready) | P0        | Copy review     |
| RNF-10.3.4 | Stack traces solo en desarrollo     | P0        | Env config      |

### Implementación HATEOAS

```rust
#[derive(Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub status: CourseStatus,
    pub price: Decimal,
    pub instructor_id: Uuid,
    #[serde(rename = "_links")]
    pub links: CourseLinks,
}

#[derive(Serialize)]
pub struct CourseLinks {
    #[serde(rename = "self")]
    pub self_link: Link,
    pub modules: Link,
    pub instructor: Link,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enroll: Option<ActionLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<ActionLink>,
}

#[derive(Serialize)]
pub struct Link {
    pub href: String,
}

#[derive(Serialize)]
pub struct ActionLink {
    pub href: String,
    pub method: String,
}

impl CourseResponse {
    pub fn from_course(course: Course, user_role: Role) -> Self {
        let base_url = "/api/v1";

        let mut links = CourseLinks {
            self_link: Link { href: format!("{}/courses/{}", base_url, course.id) },
            modules: Link { href: format!("{}/courses/{}/modules", base_url, course.id) },
            instructor: Link { href: format!("{}/users/{}", base_url, course.instructor_id) },
            enroll: None,
            edit: None,
        };

        // Acciones disponibles según rol
        if matches!(user_role, Role::Student | Role::Anonymous) && course.status == CourseStatus::Published {
            links.enroll = Some(ActionLink {
                href: format!("{}/enrollments", base_url),
                method: "POST".into(),
            });
        }

        if user_role == Role::Admin || course.instructor_id == user_id {
            links.edit = Some(ActionLink {
                href: format!("{}/courses/{}", base_url, course.id),
                method: "PUT".into(),
            });
        }

        Self {
            id: course.id,
            title: course.title,
            slug: course.slug,
            status: course.status,
            price: course.price,
            instructor_id: course.instructor_id,
            links,
        }
    }
}
```

### Error Response (RFC 7807)

```rust
#[derive(Serialize)]
pub struct ProblemDetails {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

// Ejemplo de response
{
    "type": "https://api.acc-lms.com/errors/validation",
    "title": "Validation Error",
    "status": 400,
    "detail": "The request body contains invalid data",
    "instance": "/api/v1/courses",
    "errors": [
        {
            "field": "price",
            "code": "INVALID_RANGE",
            "message": "Price must be between 0 and 10000"
        }
    ]
}
```

---

## RNF-11: API Gateway (Traefik)

### RNF-11.1: Routing

| ID         | Requisito                             | Prioridad | Métrica      |
| ---------- | ------------------------------------- | --------- | ------------ |
| RNF-11.1.1 | Path-based routing a microservicios   | P0        | Route config |
| RNF-11.1.2 | Host-based routing (api.acc-lms.com)  | P0        | Route config |
| RNF-11.1.3 | SSL/TLS termination con Let's Encrypt | P0        | Cert status  |
| RNF-11.1.4 | Load balancing round-robin            | P0        | LB config    |

### RNF-11.2: Middleware

| ID         | Requisito                     | Prioridad | Métrica    |
| ---------- | ----------------------------- | --------- | ---------- |
| RNF-11.2.1 | Rate limiting por IP y JWT    | P0        | Middleware |
| RNF-11.2.2 | CORS headers                  | P0        | Middleware |
| RNF-11.2.3 | JWT validation (forward auth) | P0        | Middleware |
| RNF-11.2.4 | Request/Response compression  | P1        | Middleware |
| RNF-11.2.5 | Circuit breaker               | P1        | Middleware |

### RNF-11.3: Observability

| ID         | Requisito                 | Prioridad | Métrica            |
| ---------- | ------------------------- | --------- | ------------------ |
| RNF-11.3.1 | Access logs estructurados | P0        | Log format         |
| RNF-11.3.2 | Prometheus metrics export | P0        | Metrics endpoint   |
| RNF-11.3.3 | Request tracing headers   | P1        | Header propagation |

### Configuración Traefik

```yaml
# traefik/traefik.yml
api:
  dashboard: true
  insecure: false

entryPoints:
  web:
    address: ':80'
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
  websecure:
    address: ':443'

providers:
  docker:
    exposedByDefault: false
    network: acc-lms-network

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@acc-lms.com
      storage: /acme.json
      httpChallenge:
        entryPoint: web

metrics:
  prometheus:
    addEntryPointsLabels: true
    addServicesLabels: true
    buckets:
      - 0.1
      - 0.3
      - 1.2
      - 5.0
```

### Service Labels

```yaml
# docker-compose.yml - auth-service
services:
  auth-service:
    build: ./be/auth-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.auth.rule=Host(`api.acc-lms.com`) && PathPrefix(`/api/v1/auth`)'
      - 'traefik.http.routers.auth.entrypoints=websecure'
      - 'traefik.http.routers.auth.tls.certresolver=letsencrypt'
      - 'traefik.http.services.auth.loadbalancer.server.port=8080'
      - 'traefik.http.services.auth.loadbalancer.healthcheck.path=/health'
      - 'traefik.http.services.auth.loadbalancer.healthcheck.interval=30s'
      # Middlewares
      - 'traefik.http.routers.auth.middlewares=auth-ratelimit,security-headers'
      # Rate limit: 10 req/min para auth
      - 'traefik.http.middlewares.auth-ratelimit.ratelimit.average=10'
      - 'traefik.http.middlewares.auth-ratelimit.ratelimit.period=1m'
      - 'traefik.http.middlewares.auth-ratelimit.ratelimit.burst=5'
```

### Routing Map

```
https://api.acc-lms.com/
├── /api/v1/auth/*       → auth-service:8080     (Rust/Actix)
├── /api/v1/users/*      → users-service:8080    (Rust/Actix)
├── /api/v1/courses/*    → courses-service:8080  (Rust/Axum)
├── /api/v1/content/*    → content-service:8080  (Rust/Actix)
├── /api/v1/enrollments/*→ enrollments-service:8080 (Rust/Actix)
├── /api/v1/payments/*   → payments-service:8080 (Rust/Actix)
├── /api/v1/analytics/*  → analytics-service:8080 (Rust/Axum)
├── /api/v1/ai/*         → ai-service:8080       (Rust/Actix)
└── /api/v1/notifications/* → notifications-service:8080 (Rust/Actix)
```

---

## RNF-12: Internacionalización

### RNF-12.1: Frontend i18n

| ID         | Requisito                                        | Prioridad | Métrica           |
| ---------- | ------------------------------------------------ | --------- | ----------------- |
| RNF-12.1.1 | Soporte inicial: es-ES, en-US                    | P0        | Language files    |
| RNF-12.1.2 | Detección automática de idioma (Accept-Language) | P1        | Browser detection |
| RNF-12.1.3 | Persistencia de preferencia de idioma            | P1        | User settings     |
| RNF-12.1.4 | Formateo de fechas según locale                  | P0        | Intl API          |
| RNF-12.1.5 | Formateo de moneda según locale                  | P0        | Intl API          |
| RNF-12.1.6 | Pluralización correcta                           | P1        | i18n library      |

### RNF-12.2: Backend i18n

| ID         | Requisito                  | Prioridad | Métrica                |
| ---------- | -------------------------- | --------- | ---------------------- |
| RNF-12.2.1 | Error messages localizados | P1        | Message catalog        |
| RNF-12.2.2 | Email templates por idioma | P1        | Template files         |
| RNF-12.2.3 | Notifications localizadas  | P1        | Notification templates |

### RNF-12.3: Contenido

| ID         | Requisito                             | Prioridad | Métrica         |
| ---------- | ------------------------------------- | --------- | --------------- |
| RNF-12.3.1 | Cursos pueden tener múltiples idiomas | P2        | Course metadata |
| RNF-12.3.2 | Subtítulos en múltiples idiomas       | P2        | Video tracks    |
| RNF-12.3.3 | Búsqueda respeta idioma del contenido | P2        | Search index    |

### Implementación React

```tsx
// i18n setup con react-i18next
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    fallbackLng: 'es',
    supportedLngs: ['es', 'en'],
    interpolation: {
      escapeValue: false,
    },
    resources: {
      es: { translation: require('./locales/es.json') },
      en: { translation: require('./locales/en.json') },
    },
  });

// Uso en componentes
const CourseCard: React.FC<{ course: Course }> = ({ course }) => {
  const { t, i18n } = useTranslation();

  return (
    <div>
      <h3>{course.title}</h3>
      <p>{t('course.price', { price: course.price })}</p>
      <p>{new Intl.DateTimeFormat(i18n.language).format(course.createdAt)}</p>
      <p>
        {new Intl.NumberFormat(i18n.language, {
          style: 'currency',
          currency: course.currency,
        }).format(course.price)}
      </p>
    </div>
  );
};
```

---

## RNF-13: Costos y Sostenibilidad

### RNF-13.1: Optimización de Recursos

| ID         | Requisito                                     | Prioridad | Métrica         |
| ---------- | --------------------------------------------- | --------- | --------------- |
| RNF-13.1.1 | Auto-scaling down en horarios de baja demanda | P1        | Cost savings    |
| RNF-13.1.2 | Resource limits en contenedores               | P0        | Memory/CPU caps |
| RNF-13.1.3 | Cleanup automático de recursos huérfanos      | P1        | Resource count  |
| RNF-13.1.4 | Log rotation y cleanup                        | P0        | Disk usage      |

### RNF-13.2: Eficiencia de Código

| ID         | Requisito                            | Prioridad | Métrica             |
| ---------- | ------------------------------------ | --------- | ------------------- |
| RNF-13.2.1 | Algoritmos O(log n) para búsquedas   | P1        | Complexity analysis |
| RNF-13.2.2 | Lazy loading de recursos             | P0        | Load time           |
| RNF-13.2.3 | Image optimization (WebP, lazy load) | P0        | Image size          |
| RNF-13.2.4 | Code splitting por ruta              | P0        | Bundle size         |
| RNF-13.2.5 | Tree shaking efectivo                | P0        | Bundle analysis     |

### RNF-13.3: Green Computing

| ID         | Requisito                                    | Prioridad | Métrica        |
| ---------- | -------------------------------------------- | --------- | -------------- |
| RNF-13.3.1 | Preferir datacenters con energía renovable   | P3        | DC selection   |
| RNF-13.3.2 | Batch processing en horarios de baja demanda | P2        | Job scheduling |
| RNF-13.3.3 | Compresión de datos en tránsito              | P1        | Transfer size  |

### Resource Limits

```yaml
# kubernetes deployment
resources:
  requests:
    memory: '256Mi'
    cpu: '250m'
  limits:
    memory: '512Mi'
    cpu: '500m'
```

---

## RNF-14: Compliance y Legal — Protección de Datos Personales

> **🌍 Jurisdicciones Aplicables**
>
> ACC LMS opera bajo múltiples marcos regulatorios de protección de datos:
>
> - **Colombia (origen):** Ley 1581 de 2012 (Habeas Data) + decretos reglamentarios
> - **Unión Europea:** GDPR (Reglamento General de Protección de Datos)
> - **Estados Unidos:** CCPA (California Consumer Privacy Act)
> - **Brasil:** LGPD (Lei Geral de Proteção de Dados)
>
> **Principio:** Aplicamos el estándar más restrictivo cuando hay conflicto entre jurisdicciones.

---

### RNF-14.1: Habeas Data Colombia (Ley 1581 de 2012)

> **📜 Marco Legal Colombiano**
>
> | Normativa                        | Descripción                                                            |
> | -------------------------------- | ---------------------------------------------------------------------- |
> | **Ley 1581 de 2012**             | Régimen general de protección de datos personales                      |
> | **Decreto 1377 de 2013**         | Reglamentación parcial de la Ley 1581                                  |
> | **Decreto 1074 de 2015**         | Decreto Único Reglamentario del Sector Comercio (Título 2, Cap. 25-26) |
> | **Circular Externa 002 de 2015** | Instrucciones SIC sobre Registro Nacional de Bases de Datos            |
> | **Ley 1266 de 2008**             | Habeas Data financiero (datos crediticios)                             |
> | **Ley 2300 de 2023**             | Actualización de derechos digitales                                    |
>
> **Autoridad de control:** Superintendencia de Industria y Comercio (SIC)

#### RNF-14.1.1: Principios de Tratamiento (Art. 4 Ley 1581)

| ID           | Principio                            | Requisito                                  | Implementación                               |
| ------------ | ------------------------------------ | ------------------------------------------ | -------------------------------------------- |
| RNF-14.1.1.1 | **Legalidad**                        | Tratamiento sujeto a ley colombiana        | Política de privacidad conforme a Ley 1581   |
| RNF-14.1.1.2 | **Finalidad**                        | Propósito legítimo informado al titular    | Consentimiento con finalidades específicas   |
| RNF-14.1.1.3 | **Libertad**                         | Consentimiento previo, expreso e informado | Opt-in explícito, no casillas pre-marcadas   |
| RNF-14.1.1.4 | **Veracidad**                        | Datos veraces, completos, actualizados     | Validación de datos, actualización periódica |
| RNF-14.1.1.5 | **Transparencia**                    | Información clara sobre tratamiento        | Avisos de privacidad accesibles              |
| RNF-14.1.1.6 | **Acceso y circulación restringida** | Acceso solo a autorizados                  | RBAC, logging de acceso a PII                |
| RNF-14.1.1.7 | **Seguridad**                        | Medidas técnicas y organizativas           | Encryption, access control, auditoría        |
| RNF-14.1.1.8 | **Confidencialidad**                 | Reserva de la información                  | NDAs, training de empleados                  |

#### RNF-14.1.2: Derechos del Titular (Art. 8 Ley 1581)

| ID           | Derecho        | Requisito Legal                     | Implementación Técnica        | Endpoint                               |
| ------------ | -------------- | ----------------------------------- | ----------------------------- | -------------------------------------- |
| RNF-14.1.2.1 | **Conocer**    | Acceder a sus datos personales      | Export JSON/PDF de perfil     | `GET /api/v1/users/me/data`            |
| RNF-14.1.2.2 | **Actualizar** | Rectificar datos inexactos          | Edición de perfil             | `PATCH /api/v1/users/me`               |
| RNF-14.1.2.3 | **Rectificar** | Corregir información errónea        | Histórico de cambios          | `PUT /api/v1/users/me/rectify`         |
| RNF-14.1.2.4 | **Suprimir**   | Eliminar datos (derecho al olvido)  | Soft delete → Hard delete 30d | `DELETE /api/v1/users/me`              |
| RNF-14.1.2.5 | **Revocar**    | Retirar autorización de tratamiento | Gestión de consentimientos    | `POST /api/v1/users/me/consent/revoke` |
| RNF-14.1.2.6 | **Consultar**  | Conocer uso dado a sus datos        | Log de accesos a PII          | `GET /api/v1/users/me/access-log`      |
| RNF-14.1.2.7 | **Reclamar**   | Presentar quejas ante SIC           | Formulario de PQR             | `POST /api/v1/pqr`                     |

#### RNF-14.1.3: Autorización para Tratamiento de Datos

```typescript
// Modelo de consentimiento conforme a Ley 1581
interface ConsentimientoHabeasData {
  // Identificación del titular
  titular: {
    nombre: string;
    documento: string;
    tipoDocumento: 'CC' | 'CE' | 'NIT' | 'Pasaporte';
    email: string;
  };

  // Autorización
  autorizacion: {
    fechaOtorgamiento: Date;
    medioObtencion: 'web' | 'app' | 'presencial' | 'telefono';
    ipAddress?: string;

    // Finalidades autorizadas (granular)
    finalidades: {
      prestacionServicio: boolean; // Requerido
      comunicacionesComerciales: boolean; // Opcional
      perfilamiento: boolean; // Opcional
      transferenciaTerceros: boolean; // Opcional
      transferenciasInternacionales: boolean; // Opcional
    };

    // Categorías de datos autorizadas
    categoriasDatos: {
      identificacion: boolean; // Nombre, documento, email
      contacto: boolean; // Teléfono, dirección
      financieros: boolean; // Solo si aplica
      sensibles: boolean; // Salud, biométricos (requiere autorización especial)
    };
  };

  // Información del responsable
  responsable: {
    razonSocial: string;
    nit: string;
    direccion: string;
    email: string;
    telefono: string;
  };

  // Registro
  prueba: {
    hash: string; // SHA-256 del documento
    timestamp: Date;
    versionPolitica: string;
  };
}
```

#### RNF-14.1.4: Registro Nacional de Bases de Datos (RNBD)

| ID           | Requisito             | Descripción                                               | Estado                   |
| ------------ | --------------------- | --------------------------------------------------------- | ------------------------ |
| RNF-14.1.4.1 | Inscripción en RNBD   | Registro ante SIC de todas las bases con datos personales | Pendiente pre-launch     |
| RNF-14.1.4.2 | Actualización anual   | Renovar registro cada año                                 | Automatizar recordatorio |
| RNF-14.1.4.3 | Reporte de incidentes | Notificar brechas de seguridad a SIC                      | Playbook de incidentes   |

#### RNF-14.1.5: Transferencias Internacionales (Art. 26 Ley 1581)

| Destino                       | Requisito                                | Implementación                |
| ----------------------------- | ---------------------------------------- | ----------------------------- |
| **Países con nivel adecuado** | Permitido sin autorización adicional     | Lista SIC actualizada         |
| **Países sin nivel adecuado** | Requiere contrato con cláusulas tipo SIC | AWS/GCP tienen cláusulas tipo |
| **Excepciones**               | Consentimiento expreso del titular       | Checkbox específico en signup |

```yaml
# Países con nivel adecuado de protección (Circular 005 de 2017 SIC)
paises_nivel_adecuado:
  - España
  - Alemania
  - Francia
  - Italia
  - Reino Unido
  - Canadá
  - Argentina
  - Uruguay
  - Costa Rica
  - México
  - Perú
  # ... y demás países UE/EEE

# AWS Regions permitidas sin autorización adicional
aws_regions_permitidas:
  - eu-west-1 # Irlanda
  - eu-central-1 # Frankfurt
  - sa-east-1 # São Paulo
```

---

### RNF-14.2: GDPR (Reglamento UE 2016/679)

> **🇪🇺 Aplicabilidad**
>
> GDPR aplica si:
>
> - Procesamos datos de residentes de la UE/EEE
> - Ofrecemos bienes/servicios a personas en la UE
> - Monitoreamos comportamiento de personas en la UE
>
> **Autoridad de control:** Autoridad de Protección de Datos del país donde tengamos más usuarios UE

#### RNF-14.2.1: Bases Legales para el Tratamiento (Art. 6 GDPR)

| Base Legal                | Uso en ACC LMS                   | Datos Aplicables               |
| ------------------------- | -------------------------------- | ------------------------------ |
| **Consentimiento**        | Marketing, cookies no esenciales | Email marketing, analytics     |
| **Ejecución de contrato** | Prestación del servicio LMS      | Perfil, progreso, certificados |
| **Obligación legal**      | Facturación, retención fiscal    | Datos de facturación           |
| **Interés legítimo**      | Seguridad, prevención de fraude  | Logs de acceso, IP             |

#### RNF-14.2.2: Derechos del Interesado (Arts. 15-22 GDPR)

| ID           | Derecho GDPR                              | Plazo     | Endpoint                         | Automatizado |
| ------------ | ----------------------------------------- | --------- | -------------------------------- | ------------ |
| RNF-14.2.2.1 | **Acceso** (Art. 15)                      | 30 días   | `GET /api/v1/users/me/data`      | ✅           |
| RNF-14.2.2.2 | **Rectificación** (Art. 16)               | 30 días   | `PATCH /api/v1/users/me`         | ✅           |
| RNF-14.2.2.3 | **Supresión** (Art. 17)                   | 30 días   | `DELETE /api/v1/users/me`        | ✅           |
| RNF-14.2.2.4 | **Limitación** (Art. 18)                  | 30 días   | `POST /api/v1/users/me/restrict` | ✅           |
| RNF-14.2.2.5 | **Portabilidad** (Art. 20)                | 30 días   | `GET /api/v1/users/me/export`    | ✅ JSON/CSV  |
| RNF-14.2.2.6 | **Oposición** (Art. 21)                   | Inmediato | `POST /api/v1/users/me/object`   | ✅           |
| RNF-14.2.2.7 | **No decisiones automatizadas** (Art. 22) | N/A       | Revisión humana de AI            | ⚠️ Manual    |

#### RNF-14.2.3: Privacy by Design y Privacy by Default (Art. 25 GDPR)

| Principio                                 | Implementación                                         |
| ----------------------------------------- | ------------------------------------------------------ |
| **Minimización de datos**                 | Solo recolectar datos estrictamente necesarios         |
| **Limitación de almacenamiento**          | TTL automático en datos temporales                     |
| **Anonimización**                         | Analytics con datos agregados, no individuales         |
| **Pseudonimización**                      | UUIDs en lugar de datos identificables en logs         |
| **Configuración restrictiva por defecto** | Opt-in para marketing, cookies opcionales desactivadas |

#### RNF-14.2.4: Registro de Actividades de Tratamiento (Art. 30 GDPR)

```yaml
# Registro de actividades de tratamiento
registro_actividades:
  - nombre: 'Gestión de usuarios'
    responsable: 'ACC LMS S.A.S.'
    finalidad: 'Prestación de servicios educativos'
    categorias_interesados: ['estudiantes', 'instructores']
    categorias_datos: ['identificación', 'contacto', 'progreso académico']
    destinatarios: ['procesadores de pago', 'proveedores cloud']
    transferencias_internacionales: 'AWS EU (cláusulas tipo)'
    plazos_supresion: '5 años post-última actividad'
    medidas_seguridad: 'Encryption, RBAC, auditoría'

  - nombre: 'Procesamiento de pagos'
    responsable: 'ACC LMS S.A.S.'
    encargado: 'Stripe Inc.'
    finalidad: 'Procesamiento de transacciones'
    categorias_datos: ['identificación', 'datos de facturación']
    transferencias_internacionales: 'Stripe EU-US (DPF)'
    plazos_supresion: '7 años (obligación fiscal)'
```

#### RNF-14.2.5: Data Protection Impact Assessment (DPIA) (Art. 35 GDPR)

| Tratamiento                    | Requiere DPIA | Razón                                         |
| ------------------------------ | ------------- | --------------------------------------------- |
| Profiling para recomendaciones | ✅ Sí         | Evaluación sistemática de aspectos personales |
| Analytics de comportamiento    | ✅ Sí         | Monitoreo a gran escala                       |
| Procesamiento de pagos         | ❌ No         | Via Stripe (ellos hacen DPIA)                 |
| Registro básico de usuarios    | ❌ No         | Tratamiento estándar                          |

---

### RNF-14.3: CCPA (California Consumer Privacy Act)

> **🇺🇸 Aplicabilidad**
>
> CCPA aplica si tenemos usuarios de California Y:
>
> - Ingresos brutos >$25M anuales, O
> - Datos de >50,000 consumidores/hogares/dispositivos, O
> - > 50% ingresos de venta de datos personales

#### RNF-14.3.1: Derechos del Consumidor (CCPA)

| ID           | Derecho                         | Implementación                        |
| ------------ | ------------------------------- | ------------------------------------- |
| RNF-14.3.1.1 | **Right to Know**               | Qué datos recolectamos y por qué      |
| RNF-14.3.1.2 | **Right to Delete**             | Mismo endpoint que GDPR               |
| RNF-14.3.1.3 | **Right to Opt-Out**            | "Do Not Sell My Personal Information" |
| RNF-14.3.1.4 | **Right to Non-Discrimination** | No penalizar por ejercer derechos     |

#### RNF-14.3.2: "Do Not Sell" (CCPA §1798.120)

```tsx
// Componente de opt-out para CCPA
const DoNotSellBanner: React.FC = () => {
  const [optedOut, setOptedOut] = useState(false);

  const handleOptOut = async () => {
    await api.post('/api/v1/privacy/do-not-sell');
    setOptedOut(true);
    // Deshabilitar tracking de terceros
    disableThirdPartyTracking();
  };

  return (
    <footer>
      <a
        href="/privacy/do-not-sell"
        onClick={handleOptOut}>
        Do Not Sell or Share My Personal Information
      </a>
    </footer>
  );
};
```

---

### RNF-14.4: LGPD Brasil (Lei 13.709/2018)

> **🇧🇷 Aplicabilidad**
>
> LGPD aplica si procesamos datos de personas en Brasil, independiente de dónde estemos ubicados.
>
> **Autoridad de control:** ANPD (Autoridade Nacional de Proteção de Dados)

#### RNF-14.4.1: Derechos del Titular (Art. 18 LGPD)

| Derecho LGPD               | Equivalente GDPR       | Implementación    |
| -------------------------- | ---------------------- | ----------------- |
| Confirmação de tratamento  | Acceso                 | ✅ Mismo endpoint |
| Acesso aos dados           | Acceso                 | ✅ Mismo endpoint |
| Correção                   | Rectificación          | ✅ Mismo endpoint |
| Anonimização               | Limitación             | ✅ Nuevo          |
| Portabilidade              | Portabilidad           | ✅ Mismo endpoint |
| Eliminação                 | Supresión              | ✅ Mismo endpoint |
| Revogação do consentimento | Retirar consentimiento | ✅ Mismo endpoint |

---

### RNF-14.5: Matriz Comparativa de Cumplimiento

| Requisito                          | 🇨🇴 Habeas Data  | 🇪🇺 GDPR          | 🇺🇸 CCPA          | 🇧🇷 LGPD            | ACC LMS        |
| ---------------------------------- | --------------- | ---------------- | ---------------- | ------------------ | -------------- |
| **Consentimiento explícito**       | ✅ Art. 9       | ✅ Art. 7        | ⚠️ Opt-out       | ✅ Art. 8          | ✅ Opt-in      |
| **Derecho de acceso**              | ✅ Art. 8       | ✅ Art. 15       | ✅ §1798.100     | ✅ Art. 18         | ✅ API         |
| **Derecho de rectificación**       | ✅ Art. 8       | ✅ Art. 16       | ❌               | ✅ Art. 18         | ✅ API         |
| **Derecho de supresión**           | ✅ Art. 8       | ✅ Art. 17       | ✅ §1798.105     | ✅ Art. 18         | ✅ API         |
| **Portabilidad**                   | ⚠️ No explícito | ✅ Art. 20       | ❌               | ✅ Art. 18         | ✅ JSON/CSV    |
| **Notificación de brecha**         | ✅ 15 días      | ✅ 72 horas      | ❌               | ✅ Plazo razonable | ✅ 72h         |
| **DPO/Oficial de datos**           | ✅ Recomendado  | ✅ Obligatorio\* | ❌               | ✅ Obligatorio     | ✅ Designado   |
| **Registro de tratamientos**       | ✅ RNBD         | ✅ Art. 30       | ❌               | ✅ Art. 37         | ✅ Documentado |
| **Transferencias internacionales** | ✅ Art. 26      | ✅ Cap. V        | ❌               | ✅ Art. 33         | ✅ SCCs        |
| **Multas máximas**                 | ~$400K USD      | €20M / 4%        | $7,500/violación | 2% facturación     | N/A            |

---

### RNF-14.6: Implementación Técnica Unificada

#### Endpoints de Privacidad

```yaml
# OpenAPI spec para endpoints de privacidad
paths:
  /api/v1/privacy/consent:
    post:
      summary: Otorgar consentimiento
      description: Registra consentimiento granular del usuario
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ConsentRequest'
    get:
      summary: Consultar consentimientos
      description: Obtiene estado actual de consentimientos

  /api/v1/privacy/consent/revoke:
    post:
      summary: Revocar consentimiento
      description: Revoca uno o más consentimientos

  /api/v1/privacy/data-export:
    post:
      summary: Solicitar exportación de datos
      description: Inicia proceso de exportación (GDPR Art. 20, LGPD Art. 18)
      responses:
        202:
          description: Solicitud aceptada, se enviará por email

  /api/v1/privacy/data-deletion:
    post:
      summary: Solicitar eliminación de datos
      description: Derecho al olvido (Ley 1581 Art. 8, GDPR Art. 17)

  /api/v1/privacy/do-not-sell:
    post:
      summary: Opt-out de venta de datos (CCPA)
      description: California Consumer Privacy Act §1798.120

  /api/v1/privacy/access-log:
    get:
      summary: Consultar log de accesos a datos
      description: Quién ha accedido a mis datos personales

components:
  schemas:
    ConsentRequest:
      type: object
      required:
        - purposes
        - jurisdiction
      properties:
        purposes:
          type: object
          properties:
            service_provision:
              type: boolean
              description: Necesario para el servicio
            marketing_email:
              type: boolean
            marketing_push:
              type: boolean
            analytics:
              type: boolean
            profiling:
              type: boolean
            third_party_sharing:
              type: boolean
        jurisdiction:
          type: string
          enum: [CO, EU, US-CA, BR, OTHER]
        version:
          type: string
          description: Versión de la política aceptada
```

#### Modelo de Base de Datos para Consentimientos

```sql
-- Tabla de consentimientos (auditoría completa)
CREATE TABLE user_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),

    -- Tipo de consentimiento
    purpose VARCHAR(50) NOT NULL, -- 'marketing_email', 'analytics', 'profiling', etc.

    -- Estado
    granted BOOLEAN NOT NULL,
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,

    -- Contexto legal
    jurisdiction VARCHAR(10) NOT NULL, -- 'CO', 'EU', 'US-CA', 'BR'
    legal_basis VARCHAR(50) NOT NULL, -- 'consent', 'contract', 'legal_obligation', 'legitimate_interest'
    policy_version VARCHAR(20) NOT NULL,

    -- Evidencia
    ip_address INET,
    user_agent TEXT,
    consent_text_hash VARCHAR(64), -- SHA-256 del texto mostrado

    -- Metadatos
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_user_purpose UNIQUE (user_id, purpose)
);

-- Índices para consultas de cumplimiento
CREATE INDEX idx_consents_user ON user_consents(user_id);
CREATE INDEX idx_consents_purpose ON user_consents(purpose);
CREATE INDEX idx_consents_jurisdiction ON user_consents(jurisdiction);

-- Tabla de solicitudes de derechos (ARCO, GDPR, CCPA)
CREATE TABLE privacy_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),

    -- Tipo de solicitud
    request_type VARCHAR(30) NOT NULL, -- 'access', 'rectification', 'deletion', 'portability', 'restriction', 'objection'

    -- Jurisdicción aplicable
    jurisdiction VARCHAR(10) NOT NULL,
    legal_reference VARCHAR(100), -- 'GDPR Art. 17', 'Ley 1581 Art. 8', etc.

    -- Estado del proceso
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'rejected'

    -- Plazos
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deadline_at TIMESTAMPTZ NOT NULL, -- 30 días para GDPR, 15 para Colombia
    completed_at TIMESTAMPTZ,

    -- Detalles
    request_details JSONB,
    response_details JSONB,
    processed_by UUID REFERENCES users(id), -- Admin que procesó

    -- Auditoría
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger para calcular deadline según jurisdicción
CREATE OR REPLACE FUNCTION set_privacy_request_deadline()
RETURNS TRIGGER AS $$
BEGIN
    NEW.deadline_at := CASE NEW.jurisdiction
        WHEN 'CO' THEN NEW.requested_at + INTERVAL '15 days'  -- Ley 1581
        WHEN 'EU' THEN NEW.requested_at + INTERVAL '30 days'  -- GDPR
        WHEN 'BR' THEN NEW.requested_at + INTERVAL '15 days'  -- LGPD
        ELSE NEW.requested_at + INTERVAL '30 days'            -- Default
    END;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER privacy_request_deadline
    BEFORE INSERT ON privacy_requests
    FOR EACH ROW EXECUTE FUNCTION set_privacy_request_deadline();
```

#### Automatización de Cumplimiento

```rust
// Servicio de cumplimiento de privacidad
use chrono::{Duration, Utc};

pub struct PrivacyComplianceService {
    db: PgPool,
    email_service: EmailService,
    storage_service: StorageService,
}

impl PrivacyComplianceService {
    /// Procesa solicitud de exportación de datos (GDPR Art. 20, Ley 1581 Art. 8)
    pub async fn process_data_export(&self, user_id: Uuid) -> Result<(), Error> {
        // 1. Recopilar todos los datos del usuario
        let user_data = self.collect_all_user_data(user_id).await?;

        // 2. Generar archivo exportable
        let export = DataExport {
            generated_at: Utc::now(),
            user_id,
            format: "JSON",
            data: UserDataExport {
                profile: user_data.profile,
                consents: user_data.consents,
                enrollments: user_data.enrollments,
                progress: user_data.progress,
                certificates: user_data.certificates,
                payments: user_data.payments,
                access_logs: user_data.access_logs,
            },
        };

        // 3. Cifrar y almacenar temporalmente
        let encrypted_file = self.encrypt_export(&export)?;
        let download_url = self.storage_service
            .upload_temp(&encrypted_file, Duration::days(7))
            .await?;

        // 4. Notificar al usuario
        self.email_service.send_data_export_ready(
            &user_data.profile.email,
            &download_url,
        ).await?;

        // 5. Registrar en auditoría
        self.log_privacy_action(user_id, "data_export", "completed").await?;

        Ok(())
    }

    /// Procesa solicitud de eliminación (Derecho al olvido)
    pub async fn process_data_deletion(&self, user_id: Uuid) -> Result<(), Error> {
        // 1. Verificar que no hay obligaciones legales de retención
        if self.has_retention_obligations(user_id).await? {
            return Err(Error::RetentionObligationExists);
        }

        // 2. Soft delete inmediato (anonimización)
        self.anonymize_user_data(user_id).await?;

        // 3. Programar hard delete (30 días para posible recuperación)
        self.schedule_hard_delete(user_id, Duration::days(30)).await?;

        // 4. Revocar todos los tokens
        self.revoke_all_tokens(user_id).await?;

        // 5. Notificar procesadores de datos
        self.notify_data_processors_deletion(user_id).await?;

        // 6. Confirmar al usuario
        self.email_service.send_deletion_confirmation(&user_email).await?;

        Ok(())
    }

    /// Job diario para alertar solicitudes próximas a vencer
    pub async fn check_pending_requests_deadlines(&self) -> Result<(), Error> {
        let approaching_deadline = sqlx::query_as!(
            PrivacyRequest,
            r#"
            SELECT * FROM privacy_requests
            WHERE status IN ('pending', 'processing')
            AND deadline_at <= NOW() + INTERVAL '3 days'
            ORDER BY deadline_at ASC
            "#
        )
        .fetch_all(&self.db)
        .await?;

        for request in approaching_deadline {
            // Alertar al equipo
            self.alert_team_deadline_approaching(&request).await?;
        }

        Ok(())
    }
}
```

### RNF-14.7: Documentos Legales Requeridos

| Documento                        | Jurisdicción | Ubicación                 | Actualización   |
| -------------------------------- | ------------ | ------------------------- | --------------- |
| **Política de Privacidad**       | Todas        | `/privacy`                | Anual + cambios |
| **Términos y Condiciones**       | Todas        | `/terms`                  | Anual + cambios |
| **Política de Cookies**          | UE, CO       | `/cookies`                | Anual           |
| **Aviso de Privacidad**          | CO           | Formulario signup         | Con cada cambio |
| **Autorización de Tratamiento**  | CO           | Formulario signup         | Con cada cambio |
| **Data Processing Agreement**    | UE           | Bajo solicitud            | Anual           |
| **Standard Contractual Clauses** | UE           | Contratos con proveedores | Según UE        |
| **CCPA Privacy Notice**          | US-CA        | `/privacy-ca`             | Anual           |

---

### RNF-14.8: Pagos (PCI DSS)

| ID         | Requisito                          | Prioridad | Métrica            |
| ---------- | ---------------------------------- | --------- | ------------------ |
| RNF-14.8.1 | PCI DSS compliance via Stripe      | P0        | Stripe integration |
| RNF-14.8.2 | No almacenar datos de tarjeta      | P0        | Tokenization       |
| RNF-14.8.3 | Facturación electrónica según país | P1        | Invoice format     |
| RNF-14.8.4 | Términos y condiciones de compra   | P0        | Legal docs         |

### RNF-14.9: Contenido y Propiedad Intelectual

| ID         | Requisito                                     | Prioridad | Métrica          |
| ---------- | --------------------------------------------- | --------- | ---------------- |
| RNF-14.9.1 | DMCA takedown process                         | P1        | Policy doc       |
| RNF-14.9.2 | Copyright notice en contenido                 | P0        | Footer/header    |
| RNF-14.9.3 | Licencias de contenido definidas              | P1        | License metadata |
| RNF-14.9.4 | Moderación de contenido generado por usuarios | P1        | Moderation queue |

### RNF-14.10: Accesibilidad Legal

| ID          | Requisito                           | Prioridad | Métrica      |
| ----------- | ----------------------------------- | --------- | ------------ |
| RNF-14.10.1 | Declaración de accesibilidad        | P1        | Legal page   |
| RNF-14.10.2 | WCAG 2.1 AA para cumplir normativas | P0        | Audit report |

---

## RNF-15: Seguridad — Modelo "Assume Breach"

> **🛡️ Filosofía de Seguridad: Ya nos atacaron**
>
> No diseñamos la seguridad pensando "qué pasa si nos atacan", sino asumiendo que **ya fuimos comprometidos** y debemos:
>
> 1. **Detectar** la intrusión lo antes posible
> 2. **Contener** el daño y limitar el movimiento lateral
> 3. **Recuperar** los sistemas y datos afectados
> 4. **Aprender** y fortalecer las defensas
>
> Este modelo se conoce como **"Assume Breach"** o **"Zero Trust"** y es el estándar de la industria (Microsoft, Google, NIST).

### RNF-15.1: Principios de Zero Trust

| ID         | Principio                      | Implementación                                     |
| ---------- | ------------------------------ | -------------------------------------------------- |
| RNF-15.1.1 | **Never trust, always verify** | JWT validado en cada request, no solo en login     |
| RNF-15.1.2 | **Least privilege**            | Permisos mínimos por rol, deny by default          |
| RNF-15.1.3 | **Assume breach**              | Logging exhaustivo, alertas de anomalías           |
| RNF-15.1.4 | **Verify explicitly**          | MFA para acciones críticas (pagos, delete account) |
| RNF-15.1.5 | **Limit blast radius**         | Microservicios aislados, network segmentation      |

### RNF-15.2: Protección de Datos de Usuarios

#### Datos en Reposo (At Rest)

| ID         | Requisito                                            | Prioridad | Implementación                    |
| ---------- | ---------------------------------------------------- | --------- | --------------------------------- |
| RNF-15.2.1 | Encryption at rest para toda la base de datos        | P0        | PostgreSQL TDE, LUKS en volúmenes |
| RNF-15.2.2 | PII cifrado a nivel de campo (email, phone, address) | P0        | AES-256-GCM con key rotation      |
| RNF-15.2.3 | Passwords hasheados (nunca cifrados)                 | P0        | Argon2id (irreversible)           |
| RNF-15.2.4 | Backups cifrados con clave separada                  | P0        | GPG/age encryption                |
| RNF-15.2.5 | Keys de cifrado en HSM/Vault (no en código)          | P0        | HashiCorp Vault / AWS KMS         |

#### Datos en Tránsito (In Transit)

| ID         | Requisito                                         | Prioridad | Implementación                |
| ---------- | ------------------------------------------------- | --------- | ----------------------------- |
| RNF-15.2.6 | TLS 1.3 obligatorio para todas las comunicaciones | P0        | Nginx/Traefik SSL termination |
| RNF-15.2.7 | mTLS entre microservicios internos                | P1        | Service mesh / Traefik mTLS   |
| RNF-15.2.8 | Certificate pinning en apps móviles               | P2        | SHA-256 fingerprint           |
| RNF-15.2.9 | No transmitir PII en URLs (query params)          | P0        | POST body / headers           |

#### Datos en Uso (In Use)

| ID          | Requisito                                    | Prioridad | Implementación               |
| ----------- | -------------------------------------------- | --------- | ---------------------------- |
| RNF-15.2.10 | No logging de PII en plain text              | P0        | Redacción automática en logs |
| RNF-15.2.11 | Memory scrubbing después de procesar secrets | P0        | Zeroize crate en Rust        |
| RNF-15.2.12 | No PII en error messages/stack traces        | P0        | Custom error handlers        |

### RNF-15.3: Detección y Respuesta a Incidentes

#### Detección (Assume we're already compromised)

| ID         | Requisito                                           | Prioridad | Métrica                      |
| ---------- | --------------------------------------------------- | --------- | ---------------------------- |
| RNF-15.3.1 | Logging de TODAS las operaciones de auth            | P0        | login, logout, token refresh |
| RNF-15.3.2 | Logging de acceso a datos sensibles                 | P0        | PII read/write audit         |
| RNF-15.3.3 | Alertas por login desde nueva ubicación/dispositivo | P0        | GeoIP + device fingerprint   |
| RNF-15.3.4 | Alertas por múltiples intentos fallidos             | P0        | >5 failures in 5 min         |
| RNF-15.3.5 | Alertas por acceso fuera de horario habitual        | P1        | Behavioral analysis          |
| RNF-15.3.6 | Alertas por descarga masiva de datos                | P0        | >100 records in 1 min        |
| RNF-15.3.7 | Alertas por escalación de privilegios               | P0        | Role change detection        |
| RNF-15.3.8 | Monitoreo de integridad de archivos críticos        | P1        | AIDE/Tripwire                |

#### Respuesta a Incidentes

| ID          | Requisito                                           | Prioridad | SLA                      |
| ----------- | --------------------------------------------------- | --------- | ------------------------ |
| RNF-15.3.9  | Playbook documentado para breach de datos           | P0        | Actualizado cada 6 meses |
| RNF-15.3.10 | Capacidad de revocar TODOS los tokens de un usuario | P0        | <1 min                   |
| RNF-15.3.11 | Capacidad de revocar TODOS los tokens del sistema   | P0        | <5 min (key rotation)    |
| RNF-15.3.12 | Capacidad de bloquear IP/rango sospechoso           | P0        | <1 min                   |
| RNF-15.3.13 | Capacidad de forzar re-autenticación global         | P0        | <5 min                   |
| RNF-15.3.14 | Notificación a usuarios afectados en breach         | P0        | <72h (GDPR requirement)  |
| RNF-15.3.15 | Retención de logs de seguridad                      | P0        | 2 años mínimo            |

### RNF-15.4: Segmentación y Contención

#### Network Segmentation

```
┌─────────────────────────────────────────────────────────────────┐
│                        DMZ (Public)                             │
│  ┌─────────────┐                                                │
│  │   Nginx     │ ← Solo puerto 443 abierto                      │
│  └─────────────┘                                                │
└─────────────────────────────────────────────────────────────────┘
         │ Firewall L7 (WAF)
         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Application Zone                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │  Traefik    │  │  Services   │  │   Workers   │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│  Sin acceso directo a Internet                                  │
└─────────────────────────────────────────────────────────────────┘
         │ Firewall interno
         ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Data Zone                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │ PostgreSQL  │  │   Redis     │  │   MinIO     │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
│  Solo accesible desde Application Zone                          │
└─────────────────────────────────────────────────────────────────┘
         │ Backup network (aislada)
         ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Backup Zone                                 │
│  ┌─────────────┐                                                │
│  │ Backup Srv  │ ← Read-only desde Data Zone                    │
│  └─────────────┘                                                │
└─────────────────────────────────────────────────────────────────┘
```

#### Principio de Mínimo Privilegio por Servicio

| Servicio              | DB Access          | Redis Access  | MinIO Access  | External APIs    |
| --------------------- | ------------------ | ------------- | ------------- | ---------------- |
| auth-service          | users (RW)         | sessions (RW) | ❌            | HaveIBeenPwned   |
| users-service         | users (RW)         | cache (RW)    | avatars (RW)  | ❌               |
| courses-service       | courses (RW)       | cache (R)     | ❌            | ❌               |
| content-service       | content (RW)       | cache (R)     | media (RW)    | ❌               |
| payments-service      | orders (RW)        | ❌            | invoices (RW) | Stripe           |
| analytics-service     | ClickHouse (R)     | ❌            | ❌            | ❌               |
| notifications-service | notifications (RW) | queue (RW)    | ❌            | SendGrid, Twilio |

### RNF-15.5: Políticas de Retención y Eliminación

| Tipo de Dato              | Retención Activa       | Retención Archivo    | Eliminación           |
| ------------------------- | ---------------------- | -------------------- | --------------------- |
| **PII de usuario**        | Mientras cuenta activa | 30 días post-delete  | GDPR compliant        |
| **Logs de acceso**        | 90 días                | 2 años               | Automática            |
| **Logs de seguridad**     | 1 año                  | 5 años               | Manual con aprobación |
| **Transacciones de pago** | 7 años                 | 10 años              | Requerimiento fiscal  |
| **Contenido de cursos**   | Mientras publicado     | 1 año post-unpublish | Manual                |
| **Backups**               | 30 días                | 1 año                | Rotación automática   |

### RNF-15.6: Incident Response Playbook (Resumen)

```yaml
# Playbook: Data Breach Response
name: data-breach-response
version: 1.0
last_updated: 2024-12-14

phases:
  1_detection:
    triggers:
      - Alert: 'Mass data download detected'
      - Alert: 'Unauthorized PII access'
      - Alert: 'Privilege escalation'
      - External: 'Bug bounty report'
      - External: 'User complaint'
    actions:
      - Notify security team (PagerDuty)
      - Create incident ticket (severity: critical)
      - Start incident timeline documentation

  2_containment:
    immediate:
      - Block suspicious IPs/users
      - Revoke compromised tokens
      - Isolate affected services
    short_term:
      - Enable enhanced logging
      - Snapshot affected systems
      - Preserve evidence

  3_eradication:
    actions:
      - Identify attack vector
      - Patch vulnerability
      - Rotate all credentials
      - Verify no persistence mechanisms

  4_recovery:
    actions:
      - Restore from clean backup if needed
      - Gradual service restoration
      - Enhanced monitoring period (30 days)

  5_post_incident:
    actions:
      - Root cause analysis (5 whys)
      - Update security controls
      - Notify affected users (if required)
      - Regulatory notification (GDPR: 72h)
      - Public disclosure (if required)
      - Lessons learned meeting
      - Update this playbook

contacts:
  security_team: security@acc-lms.com
  legal: legal@acc-lms.com
  dpo: dpo@acc-lms.com # Data Protection Officer

escalation:
  - L1: On-call engineer (15 min response)
  - L2: Security lead (30 min response)
  - L3: CTO + Legal (1 hour response)
```

### RNF-15.7: Seguridad de Desarrollo (DevSecOps)

| ID         | Requisito                         | Prioridad | Herramienta             |
| ---------- | --------------------------------- | --------- | ----------------------- |
| RNF-15.7.1 | SAST en cada PR                   | P0        | SonarQube, Semgrep      |
| RNF-15.7.2 | Dependency scanning               | P0        | cargo audit, npm audit  |
| RNF-15.7.3 | Secret scanning en commits        | P0        | GitLeaks, TruffleHog    |
| RNF-15.7.4 | Container image scanning          | P0        | Trivy, Snyk             |
| RNF-15.7.5 | DAST en staging                   | P1        | OWASP ZAP               |
| RNF-15.7.6 | Penetration testing trimestral    | P1        | Third party             |
| RNF-15.7.7 | Security training anual para devs | P1        | OWASP Top 10            |
| RNF-15.7.8 | Bug bounty program                | P2        | HackerOne (post-launch) |

### Implementación Rust: Logging Seguro

```rust
use tracing::{info, warn, instrument};
use serde::Serialize;

/// Trait para redactar PII en logs
pub trait Redactable {
    fn redact(&self) -> String;
}

impl Redactable for String {
    fn redact(&self) -> String {
        if self.len() <= 4 {
            "****".to_string()
        } else {
            format!("{}****{}", &self[..2], &self[self.len()-2..])
        }
    }
}

/// Evento de seguridad para audit log
#[derive(Serialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub user_id: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
    pub resource: String,
    pub action: String,
    pub outcome: Outcome,
    pub details: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub enum SecurityEventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    PrivilegeChange,
    Anomaly,
}

#[derive(Serialize)]
pub enum Outcome {
    Success,
    Failure,
    Blocked,
}

/// Middleware para logging de seguridad
#[instrument(skip(req, srv), fields(
    ip = %req.connection_info().realip_remote_addr().unwrap_or("unknown"),
    user_agent = %req.headers().get("User-Agent").map(|h| h.to_str().unwrap_or("")).unwrap_or(""),
))]
pub async fn security_logging_middleware(
    req: ServiceRequest,
    srv: &impl Service<ServiceRequest, Response = ServiceResponse>,
) -> Result<ServiceResponse, Error> {
    let user_id = req.extensions().get::<Claims>().map(|c| c.sub.clone());
    let path = req.path().to_string();
    let method = req.method().to_string();

    // Log antes de procesar
    info!(
        event = "request_start",
        user_id = ?user_id,
        path = %path,
        method = %method,
    );

    let response = srv.call(req).await?;

    // Detectar anomalías
    if response.status().is_client_error() || response.status().is_server_error() {
        warn!(
            event = "request_error",
            user_id = ?user_id,
            path = %path,
            status = %response.status().as_u16(),
        );
    }

    // Log acceso a datos sensibles
    if is_sensitive_endpoint(&path) {
        log_security_event(SecurityEvent {
            event_type: SecurityEventType::DataAccess,
            user_id,
            ip_address: "redacted".to_string(), // Se obtiene del request
            user_agent: "redacted".to_string(),
            resource: path,
            action: method,
            outcome: if response.status().is_success() { Outcome::Success } else { Outcome::Failure },
            details: None,
            timestamp: chrono::Utc::now(),
        }).await;
    }

    Ok(response)
}

fn is_sensitive_endpoint(path: &str) -> bool {
    path.contains("/users/")
        || path.contains("/payments/")
        || path.contains("/profile")
        || path.contains("/export")
}

/// Revocación masiva de tokens
pub async fn revoke_all_user_tokens(user_id: &str, redis: &RedisPool) -> Result<(), Error> {
    // Incrementar el token version del usuario
    // Todos los tokens con version anterior serán inválidos
    let key = format!("user:{}:token_version", user_id);
    redis.incr(&key).await?;

    warn!(
        event = "tokens_revoked",
        user_id = %user_id,
        reason = "security_action"
    );

    Ok(())
}

/// Revocación de todos los tokens del sistema (key rotation)
pub async fn rotate_jwt_signing_key() -> Result<(), Error> {
    // 1. Generar nueva key pair
    // 2. Publicar nueva public key
    // 3. Mantener old key para validación por 15 min
    // 4. Remover old key

    warn!(
        event = "jwt_key_rotation",
        reason = "security_incident"
    );

    Ok(())
}
```

---

## Matriz de Trazabilidad RNF → RF

| RNF                          | Requisitos Funcionales Relacionados                                |
| ---------------------------- | ------------------------------------------------------------------ |
| RNF-01 (Performance)         | RF-AUTH-_, RF-CAT-_, RF-ENR-\* (todos los endpoints críticos)      |
| RNF-02 (Disponibilidad)      | RF-AUTH-001 (login siempre disponible), RF-CAT-001 (catálogo)      |
| RNF-03 (Seguridad)           | RF-AUTH-001 a RF-AUTH-008 (autenticación completa)                 |
| RNF-04 (Observabilidad)      | RF-SYS-001 a RF-SYS-013 (operaciones del sistema)                  |
| RNF-05 (Calidad)             | Aplica a todos los RFs (quality gates)                             |
| RNF-06 (Datos)               | RF-USR-_, RF-CRS-_, RF-ORD-\* (datos persistentes)                 |
| RNF-07 (DevOps)              | RF-SYS-\* (deploy, migrations)                                     |
| RNF-08 (Usabilidad)          | RF-CAT-_, RF-LRN-_, RF-CRS-\* (interfaces de usuario)              |
| RNF-09 (Escalabilidad datos) | RF-REP-\*, RF-SYS-008 (analytics)                                  |
| RNF-10 (API Design)          | Todos los endpoints de API                                         |
| RNF-11 (Gateway)             | Routing a todos los servicios                                      |
| RNF-12 (i18n)                | RF-CAT-003, RF-NOT-\*, RF-USR-003 (preferencias)                   |
| RNF-13 (Costos)              | RF-SYS-007 (escalado), RF-SYS-009 (cleanup)                        |
| RNF-14 (Compliance)          | RF-AUTH-007 (privacy), RF-USR-007 (data export), RF-ORD-\* (pagos) |
| **RNF-15 (Assume Breach)**   | **TODOS los RFs** (seguridad transversal a todo el sistema)        |

---

## Validación de RNFs

### Checklist Pre-Release

```markdown
## Performance

- [ ] Load test passed (500 users, <200ms P95)
- [ ] Lighthouse score ≥90
- [ ] No memory leaks detected

## Security (Assume Breach)

- [ ] OWASP ZAP scan passed
- [ ] cargo audit: 0 critical vulnerabilities
- [ ] npm audit: 0 critical vulnerabilities
- [ ] Incident response playbook updated
- [ ] Token revocation tested
- [ ] PII encryption verified
- [ ] Security event logging active
- [ ] Penetration test completed (quarterly)

## Quality

- [ ] Test coverage ≥80%
- [ ] SonarQube quality gate passed
- [ ] No clippy warnings

## Observability

- [ ] All services have health endpoints
- [ ] Prometheus metrics configured
- [ ] Alerting rules active

## Compliance

- [ ] Privacy policy updated
- [ ] WCAG 2.1 AA audit passed
- [ ] Cookie consent implemented
```

### Monitoring Dashboard

```yaml
# Grafana dashboard panels
panels:
  - title: 'Request Rate'
    query: sum(rate(http_requests_total[5m])) by (service)

  - title: 'Error Rate'
    query: sum(rate(http_requests_total{status=~"5.."}[5m])) / sum(rate(http_requests_total[5m]))

  - title: 'Latency P95'
    query: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))

  - title: 'Database Connections'
    query: pg_stat_activity_count

  - title: 'Redis Memory'
    query: redis_memory_used_bytes / redis_memory_max_bytes
```

---

## Resumen de Métricas Clave

| Categoría          | Métrica         | Target     | Crítico      |
| ------------------ | --------------- | ---------- | ------------ |
| **Latencia**       | API P95         | <200ms     | >500ms       |
| **Throughput**     | RPS             | 1000       | <500         |
| **Disponibilidad** | Uptime          | 99.5%      | <99%         |
| **Errores**        | Error rate      | <1%        | >5%          |
| **Seguridad**      | Vulnerabilities | 0 critical | Any critical |
| **Calidad**        | Test coverage   | ≥80%       | <70%         |
| **Performance FE** | Lighthouse      | ≥90        | <80          |
| **Accesibilidad**  | WCAG            | AA         | Below AA     |

---

**Documento generado:** 2024-12-14  
**Próxima revisión:** Cada sprint o ante cambios de arquitectura

```

```
