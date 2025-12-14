# ACC LMS — Infraestructura y API Gateway

**Versión:** 2025-08-08  
**Estado:** Diseño para monorepo inicial

---

## 🚀 Estrategia de Monorepo con Traefik

### Decisión Arquitectónica

El proyecto **ACC LMS** inicia como **monorepo** por las siguientes razones:

- **Tamaño actual:** "Little repo" con 78 microservicios en estructura inicial
- **Equipo pequeño:** 1-5 desarrolladores en fase inicial
- **Coordinación:** Benefits de unificación > costs de separación
- **Evolución:** Métrica-driven separation cuando sea necesario

### Separación Futura

Revisar mensualmente con [Scorecard de Separación](_docs/monorepo-separation-scorecard.md):

**Trigger de separación:** >60% métricas en rojo

- Equipo: +50 desarrolladores
- Servicios en prod: +40 activos
- Build time: +45 minutos
- Teams especializados: +8 equipos

---

## 🌐 Traefik como API Gateway

### Arquitectura de Red

```mermaid
graph TB
    Client[Client Apps] --> Traefik[Traefik Gateway]
    Traefik --> Auth[auth-service:8000]
    Traefik --> Users[users-service:8001]
    Traefik --> Courses[courses-service:8002]
    Traefik --> AI[ai-service:8003]
    Traefik --> Analytics[analytics-service:8004]
    Traefik --> Content[content-service:8005]
    Traefik --> Enrollments[enrollments-service:8006]
    Traefik --> Grades[grades-service:8007]
    Traefik --> Assignments[assignments-service:8008]
    Traefik --> Payments[payments-service:8009]
    Traefik --> Notifications[notifications-service:8010]
    Traefik --> Search[search-service:8011]
    Traefik --> BI[business-intelligence-service:8012]
```

### Service Discovery Automático

```yaml
# docker-compose.yml para desarrollo
version: '3.8'

services:
  traefik:
    image: traefik:v3.0
    command:
      - '--api.dashboard=true'
      - '--api.insecure=true'
      - '--providers.docker=true'
      - '--providers.docker.exposedbydefault=false'
      - '--entrypoints.web.address=:80'
      - '--entrypoints.websecure.address=:443'
      - '--log.level=INFO'
      - '--accesslog=true'
      - '--metrics.prometheus=true'
    ports:
      - '80:80'
      - '443:443'
      - '8080:8080' # Dashboard
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    networks:
      - acc-lms-network

  # FastAPI Services
  auth-service:
    build: ./be/fastapi/auth-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.auth.rule=PathPrefix(`/api/v1/auth`)'
      - 'traefik.http.services.auth.loadbalancer.server.port=8000'
      - 'traefik.http.routers.auth.middlewares=cors,rate-limit'
    environment:
      - DATABASE_URL=postgresql://user:pass@postgres:5432/acc_lms
    depends_on:
      - postgres
    networks:
      - acc-lms-network

  ai-service:
    build: ./be/fastapi/ai-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.ai.rule=PathPrefix(`/api/v1/ai`)'
      - 'traefik.http.services.ai.loadbalancer.server.port=8000'
      - 'traefik.http.routers.ai.middlewares=auth,cors,rate-limit'
    networks:
      - acc-lms-network

  # Go Services
  users-service:
    build: ./be/go/users-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.users.rule=PathPrefix(`/api/v1/users`)'
      - 'traefik.http.services.users.loadbalancer.server.port=8080'
      - 'traefik.http.routers.users.middlewares=auth,cors'
    networks:
      - acc-lms-network

  analytics-service:
    build: ./be/go/analytics-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.analytics.rule=PathPrefix(`/api/v1/analytics`)'
      - 'traefik.http.services.analytics.loadbalancer.server.port=8080'
      - 'traefik.http.routers.analytics.middlewares=auth,cors'
    networks:
      - acc-lms-network

  # Express Services
  courses-service:
    build: ./be/express/courses-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.courses.rule=PathPrefix(`/api/v1/courses`)'
      - 'traefik.http.services.courses.loadbalancer.server.port=3000'
      - 'traefik.http.routers.courses.middlewares=cors'
    networks:
      - acc-lms-network

  # Spring Boot Services
  payments-service:
    build: ./be/sb-java/payments-service
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.payments.rule=PathPrefix(`/api/v1/payments`)'
      - 'traefik.http.services.payments.loadbalancer.server.port=8080'
      - 'traefik.http.routers.payments.middlewares=auth,cors,rate-limit-strict'
    networks:
      - acc-lms-network

networks:
  acc-lms-network:
    external: false
```

### Middleware Pipeline

```yaml
# Middleware definitions
labels:
  # CORS para desarrollo
  - 'traefik.http.middlewares.cors.headers.accesscontrolalloworigin=http://localhost:3000'
  - 'traefik.http.middlewares.cors.headers.accesscontrolallowmethods=GET,POST,PUT,DELETE,OPTIONS'
  - 'traefik.http.middlewares.cors.headers.accesscontrolallowheaders=Content-Type,Authorization'

  # Rate limiting básico
  - 'traefik.http.middlewares.rate-limit.ratelimit.burst=100'
  - 'traefik.http.middlewares.rate-limit.ratelimit.average=50'

  # Rate limiting estricto para pagos
  - 'traefik.http.middlewares.rate-limit-strict.ratelimit.burst=10'
  - 'traefik.http.middlewares.rate-limit-strict.ratelimit.average=5'

  # Autenticación JWT
  - 'traefik.http.middlewares.auth.forwardauth.address=http://auth-service:8000/api/v1/auth/validate'
  - 'traefik.http.middlewares.auth.forwardauth.authResponseHeaders=X-User-Id,X-User-Role,X-User-Email'

  # Circuit breaker
  - 'traefik.http.middlewares.circuit-breaker.circuitbreaker.expression=NetworkErrorRatio() > 0.3'
  - 'traefik.http.middlewares.circuit-breaker.circuitbreaker.checkperiod=10s'
  - 'traefik.http.middlewares.circuit-breaker.circuitbreaker.fallbackduration=30s'
```

---

## 🔧 Configuración por Entorno

### Desarrollo Local

```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  traefik:
    extends:
      file: docker-compose.yml
      service: traefik
    command:
      - '--api.dashboard=true'
      - '--api.insecure=true' # Solo desarrollo
      - '--log.level=DEBUG'
      - '--providers.docker.watch=true'
    labels:
      - 'traefik.http.routers.dashboard.rule=Host(`traefik.localhost`)'

  # Override para desarrollo con hot reload
  auth-service:
    extends:
      file: docker-compose.yml
      service: auth-service
    volumes:
      - ./be/fastapi/auth-service:/app
    environment:
      - DEBUG=true
      - RELOAD=true
```

### Producción

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  traefik:
    extends:
      file: docker-compose.yml
      service: traefik
    command:
      - '--api.dashboard=false'
      - '--certificatesresolvers.letsencrypt.acme.email=admin@acc-lms.com'
      - '--certificatesresolvers.letsencrypt.acme.storage=/acme.json'
      - '--certificatesresolvers.letsencrypt.acme.httpchallenge.entrypoint=web'
    volumes:
      - ./acme.json:/acme.json
    labels:
      # Redirect HTTP to HTTPS
      - 'traefik.http.routers.web-redirect.rule=HostRegexp(`{host:.+}`)'
      - 'traefik.http.routers.web-redirect.entrypoints=web'
      - 'traefik.http.routers.web-redirect.middlewares=redirect-to-https'
      - 'traefik.http.middlewares.redirect-to-https.redirectscheme.scheme=https'

  auth-service:
    extends:
      file: docker-compose.yml
      service: auth-service
    labels:
      - 'traefik.http.routers.auth.tls.certresolver=letsencrypt'
      - 'traefik.http.routers.auth.rule=Host(`api.acc-lms.com`) && PathPrefix(`/api/v1/auth`)'
```

---

## 📊 Monitoreo y Observabilidad

### Métricas Traefik

```yaml
# Prometheus integration
services:
  prometheus:
    image: prom/prometheus
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - '9090:9090'

  grafana:
    image: grafana/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    ports:
      - '3001:3000'
    volumes:
      - ./monitoring/grafana/dashboards:/var/lib/grafana/dashboards
```

### Dashboard Traefik

```yaml
# Configuración del dashboard
labels:
  - 'traefik.http.routers.dashboard.rule=Host(`monitor.acc-lms.local`)'
  - 'traefik.http.routers.dashboard.service=api@internal'
  - 'traefik.http.routers.dashboard.middlewares=dashboard-auth'
  - 'traefik.http.middlewares.dashboard-auth.basicauth.users=admin:$$2y$$10$$...'
```

### Health Checks

```yaml
# Health checks para cada servicio
healthcheck:
  test: ['CMD', 'curl', '-f', 'http://localhost:8000/health']
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s

labels:
  - 'traefik.http.services.auth.loadbalancer.healthcheck.path=/health'
  - 'traefik.http.services.auth.loadbalancer.healthcheck.interval=30s'
  - 'traefik.http.services.auth.loadbalancer.healthcheck.timeout=10s'
```

---

## 🚀 Scripts de Gestión

### Startup Script

```bash
#!/bin/bash
# scripts/start-dev.sh

echo "🚀 Starting ACC LMS Development Environment"

# Crear red si no existe
docker network create acc-lms-network 2>/dev/null || true

# Iniciar bases de datos
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d postgres redis mongodb clickhouse

# Esperar a que las bases de datos estén listas
echo "⏳ Waiting for databases..."
sleep 10

# Ejecutar migraciones
./db/scripts/db-manager.sh migrate

# Iniciar Traefik
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d traefik

# Iniciar servicios core
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d \
  auth-service users-service courses-service

echo "✅ Development environment ready!"
echo "📊 Traefik Dashboard: http://traefik.localhost:8080"
echo "🔧 API Base URL: http://localhost/api/v1"
```

### Service Management

```bash
#!/bin/bash
# scripts/manage-services.sh

COMMAND=$1
STACK=$2
SERVICE=$3

case $COMMAND in
  "start")
    if [ -n "$STACK" ] && [ -n "$SERVICE" ]; then
      echo "Starting $STACK/$SERVICE"
      docker-compose up -d ${STACK}-${SERVICE}
    elif [ -n "$STACK" ]; then
      echo "Starting all $STACK services"
      docker-compose up -d $(docker-compose config --services | grep "^${STACK}-")
    else
      echo "Starting all services"
      docker-compose up -d
    fi
    ;;
  "stop")
    # Similar logic for stopping
    ;;
  "logs")
    if [ -n "$STACK" ] && [ -n "$SERVICE" ]; then
      docker-compose logs -f ${STACK}-${SERVICE}
    fi
    ;;
esac
```

---

## 📋 Checklist de Setup

### Desarrollo Local

- [ ] Docker y Docker Compose instalados
- [ ] Clonar repositorio: `git clone repo && cd acc-lms`
- [ ] Configurar variables: `cp .env.example .env`
- [ ] Crear red: `docker network create acc-lms-network`
- [ ] Iniciar stack: `./scripts/start-dev.sh`
- [ ] Verificar dashboard: `http://traefik.localhost:8080`
- [ ] Test API: `curl http://localhost/api/v1/health`

### Producción

- [ ] Servidor con Docker configurado
- [ ] Dominio configurado (api.acc-lms.com)
- [ ] SSL certificados (Let's Encrypt automático)
- [ ] Variables de entorno de producción
- [ ] Backup y monitoreo configurados
- [ ] CI/CD pipeline configurado

Este diseño nos permite empezar con **monorepo simple** y **escalar orgánicamente** según las métricas del scorecard.
