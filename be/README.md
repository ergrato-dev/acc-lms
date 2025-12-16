# ACC LMS - Backend (Rust Microservices)

Backend del sistema LMS implementado como microservicios en Rust.

## 🔧 Requisitos

| Herramienta | Versión  | Notas                                |
| ----------- | -------- | ------------------------------------ |
| **Rust**    | >= 1.85  | Ver ADR-001 en development-standards |
| **Docker**  | >= 24.0  | Para builds containerizados          |
| **PostgreSQL** | 16.x  | Base de datos principal              |

## 📦 Servicios

| Servicio               | Puerto | Descripción                          |
| ---------------------- | ------ | ------------------------------------ |
| `auth-service`         | 8080   | Autenticación JWT, OAuth, MFA        |
| `users-service`        | 8081   | Gestión de usuarios y perfiles       |
| `courses-service`      | 8082   | Cursos, secciones y lecciones        |
| `enrollments-service`  | 8083   | Inscripciones y progreso             |
| `assessments-service`  | 8084   | Exámenes, quizzes y calificaciones   |
| `payments-service`     | 8085   | Pagos, suscripciones, facturas       |
| `notifications-service`| 8086   | Email, push, SMS, in-app             |
| `chatbot-service`      | 8087   | Tutor AI, RAG, embeddings            |
| `analytics-service`    | 8088   | Métricas, reportes, dashboards       |
| `content-service`      | 8089   | Almacenamiento multimedia            |

## 🚀 Comandos

### Desarrollo local

```bash
# Verificar compilación de todo el workspace
cargo check --workspace

# Compilar en modo desarrollo
cargo build --workspace

# Compilar en modo release
cargo build --release --workspace

# Ejecutar un servicio específico
cargo run -p auth-service

# Ejecutar tests
cargo test --workspace

# Formatear código
cargo fmt --all

# Linter
cargo clippy --workspace -- -D warnings

# Auditoría de seguridad
cargo audit
```

### Docker

```bash
# Construir imagen de un servicio
docker build --build-arg SERVICE_NAME=auth -t acc-lms-auth .

# Construir imagen de desarrollo
docker build --build-arg SERVICE_NAME=auth --target development -t acc-lms-auth-dev .

# Verificar compilación en Docker (sin instalar Rust localmente)
docker run --rm -v $(pwd):/app -w /app rust:1.85 cargo check --workspace
```

## 📁 Estructura

```
be/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lock file
├── Dockerfile              # Multi-stage production build
├── shared/                 # Código compartido entre servicios
│   ├── src/
│   │   ├── auth/           # JWT, middleware de auth
│   │   ├── config/         # Configuración por entorno
│   │   ├── database/       # Pool de conexiones
│   │   ├── error/          # Tipos de error comunes
│   │   └── telemetry/      # Logging, tracing, metrics
│   └── Cargo.toml
├── auth-service/           # Microservicio de autenticación
├── users-service/          # Microservicio de usuarios
├── courses-service/        # Microservicio de cursos
├── enrollments-service/    # Microservicio de inscripciones
├── assessments-service/    # Microservicio de evaluaciones
├── payments-service/       # Microservicio de pagos
├── notifications-service/  # Microservicio de notificaciones
├── chatbot-service/        # Microservicio de AI/chatbot
├── analytics-service/      # Microservicio de analytics
└── content-service/        # Microservicio de contenido multimedia
```

## ⚙️ Configuración

Cada servicio se configura mediante variables de entorno:

```bash
# Servidor
HOST=0.0.0.0
PORT=8080

# Base de datos
DATABASE_URL=postgres://user:pass@localhost:5432/acc_lms

# JWT
JWT_SECRET=your-secret-key
JWT_EXPIRATION=3600

# Logging
RUST_LOG=info,sqlx=warn
```

## 📝 Decisiones Arquitectónicas

Las decisiones importantes están documentadas en:
- [ADR-001: Versión de Rust 1.85](./../_docs/development/development-standards.md#adr-001-versión-de-rust-unificada)
- [ADR-002: Storage Híbrido](./../_docs/development/development-standards.md#adr-002-storage-híbrido-para-contenido-multimedia)

## 🔗 Documentación Relacionada

- [Estándares de Desarrollo](../_docs/development/development-standards.md)
- [Arquitectura de Base de Datos](../_docs/architecture/database-architecture.md)
- [Requerimientos Funcionales](../_docs/business/functional-requirements.md)
