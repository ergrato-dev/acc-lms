# ACC LMS - Infraestructura Docker

> Configuración de Docker y Docker Compose para desarrollo y producción.

## Requisitos

- Docker Engine 24+
- Docker Compose v2.20+
- 8GB RAM mínimo (16GB recomendado para desarrollo)
- 20GB espacio en disco

## Estructura

```
acc-lms/
├── docker-compose.yml          # Desarrollo local
├── docker-compose.prod.yml     # Producción
├── .env.example                # Template de variables
├── be/
│   └── Dockerfile              # Backend Rust (multi-stage)
├── fe/
│   ├── Dockerfile              # Frontend React (multi-stage)
│   └── nginx.conf              # Config SPA
└── infra/
    ├── nginx/
    │   └── nginx.conf          # Load balancer
    ├── traefik/
    │   ├── traefik.yml         # API Gateway config
    │   └── dynamic/
    │       └── middlewares.yml # Middlewares
    ├── postgres/
    │   └── postgresql.conf     # DB tuning
    └── clickhouse/
        └── config.xml          # Analytics config
```

## Quick Start (Desarrollo)

```bash
# 1. Copiar variables de entorno
cp .env.example .env

# 2. Iniciar todos los servicios
docker compose up -d

# 3. Verificar que todo está corriendo
docker compose ps

# 4. Ver logs
docker compose logs -f
```

## Servicios

### URLs de Desarrollo

| Servicio          | URL                   | Descripción             |
| ----------------- | --------------------- | ----------------------- |
| Frontend          | http://localhost:5173 | React + Vite dev server |
| API Gateway       | http://localhost:8080 | Traefik (routing)       |
| Traefik Dashboard | http://localhost:8081 | Solo en desarrollo      |
| PostgreSQL        | localhost:5432        | Base de datos principal |
| Redis             | localhost:6379        | Cache                   |
| MongoDB           | localhost:27017       | Notificaciones          |
| ClickHouse HTTP   | localhost:8123        | Analytics               |
| MinIO Console     | http://localhost:9001 | Object storage          |

### Microservicios (Rust)

| Servicio      | Endpoint                  | Puerto Interno |
| ------------- | ------------------------- | -------------- |
| Auth          | `/api/v1/auth/*`          | 8080           |
| Users         | `/api/v1/users/*`         | 8080           |
| Courses       | `/api/v1/courses/*`       | 8080           |
| Enrollments   | `/api/v1/enrollments/*`   | 8080           |
| Payments      | `/api/v1/payments/*`      | 8080           |
| Notifications | `/api/v1/notifications/*` | 8080           |
| Analytics     | `/api/v1/analytics/*`     | 8080           |

## Comandos Útiles

### Desarrollo

```bash
# Iniciar en modo detached
docker compose up -d

# Iniciar con logs visibles
docker compose up

# Reconstruir un servicio específico
docker compose build --no-cache svc-auth

# Reiniciar un servicio
docker compose restart svc-auth

# Ver logs de un servicio
docker compose logs -f svc-auth

# Ejecutar comando en un contenedor
docker compose exec postgres psql -U acc -d acc_lms

# Shell en un contenedor
docker compose exec svc-auth /bin/sh
```

### Base de Datos

```bash
# Conectar a PostgreSQL
docker compose exec postgres psql -U acc -d acc_lms

# Backup
docker compose exec postgres pg_dump -U acc acc_lms > backup.sql

# Restore
docker compose exec -T postgres psql -U acc acc_lms < backup.sql

# Ejecutar migraciones
docker compose exec postgres psql -U acc -d acc_lms -f /docker-entrypoint-initdb.d/001_initial_schema.sql
```

### Limpieza

```bash
# Detener servicios
docker compose down

# Detener y eliminar volúmenes (⚠️ BORRA DATOS)
docker compose down -v

# Eliminar imágenes huérfanas
docker image prune -f

# Limpieza completa
docker system prune -af --volumes
```

## Producción

### Despliegue

```bash
# 1. Configurar variables de producción
cp .env.example .env.production
# Editar .env.production con valores reales

# 2. Desplegar
docker compose -f docker-compose.prod.yml --env-file .env.production up -d

# 3. Verificar
docker compose -f docker-compose.prod.yml ps
```

### Escalado

```bash
# Escalar servicio horizontalmente
docker compose -f docker-compose.prod.yml up -d --scale svc-courses=3

# Ver recursos
docker stats
```

### Actualización sin Downtime

```bash
# Actualizar un servicio
docker compose -f docker-compose.prod.yml up -d --no-deps --build svc-auth

# Rolling update (con réplicas)
docker compose -f docker-compose.prod.yml up -d --scale svc-auth=3
# Esperar a que estén healthy
docker compose -f docker-compose.prod.yml up -d --scale svc-auth=2
```

## Política de Versiones

| Componente | Versión           | Política                               |
| ---------- | ----------------- | -------------------------------------- |
| Rust       | `1-slim-bookworm` | Latest **stable** (nunca beta/nightly) |
| PostgreSQL | `17-alpine`       | **17+** (latest stable)                |
| Redis      | `alpine`          | Latest **stable**                      |
| Nginx      | `stable-alpine`   | Latest **stable** (nunca mainline)     |
| Node.js    | `22-alpine`       | LTS (versiones pares)                  |

## Troubleshooting

### Servicio no inicia

```bash
# Ver logs detallados
docker compose logs svc-auth

# Verificar health check
docker inspect acc-lms-auth --format='{{.State.Health}}'

# Reiniciar
docker compose restart svc-auth
```

### Base de datos no conecta

```bash
# Verificar que postgres está healthy
docker compose ps postgres

# Ver logs de postgres
docker compose logs postgres

# Probar conexión
docker compose exec postgres pg_isready -U acc
```

### Puerto en uso

```bash
# Encontrar proceso usando el puerto
lsof -i :5432

# O cambiar el puerto en docker-compose.yml
ports:
  - '5433:5432'  # Mapear a puerto diferente
```

### Hot reload no funciona (Rust)

El hot reload usa `cargo-watch`. Si no funciona:

```bash
# Verificar que el volumen está montado
docker compose exec svc-auth ls -la /app

# Reiniciar con rebuild
docker compose up --build svc-auth
```

## Recursos de Hardware (Recomendados)

### Desarrollo

| Recurso | Mínimo  | Recomendado |
| ------- | ------- | ----------- |
| CPU     | 4 cores | 8 cores     |
| RAM     | 8GB     | 16GB        |
| Disco   | 20GB    | 50GB SSD    |

### Producción (por nodo)

| Recurso | Mínimo    | Recomendado |
| ------- | --------- | ----------- |
| CPU     | 4 cores   | 8+ cores    |
| RAM     | 16GB      | 32GB        |
| Disco   | 100GB SSD | 500GB NVMe  |
