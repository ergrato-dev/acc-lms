# Estándares de Desarrollo - ACC LMS

**Versión:** 2025-12-14  
**Stack:** React 19 (Frontend) + Rust REST API (Backend)

---

## 🏗️ Arquitectura del Proyecto

### Stack Tecnológico

| Capa              | Tecnología                                  | Versión              |
| ----------------- | ------------------------------------------- | -------------------- |
| **Frontend**      | React 19 + Vite + TypeScript + Tailwind CSS | React 19.x, Vite 6.x |
| **Backend**       | Rust + Actix-web/Axum                       | Rust 1.75+           |
| **Base de Datos** | PostgreSQL                                  | 16.x                 |
| **Cache**         | Redis                                       | 7.x                  |
| **API Gateway**   | Traefik                                     | 3.x                  |
| **Contenedores**  | Docker + Kubernetes                         | Latest               |

---

## 📦 Gestión de Dependencias

### Frontend: PNPM como Estándar Obligatorio

**Decisión:** ACC LMS utiliza **PNPM** exclusivamente para el frontend React.

#### ✅ **Por qué PNPM sobre NPM**

##### 1. **Seguridad Superior**

- **Aislamiento estricto**: Previene dependency confusion attacks
- **Verificación de integridad**: SHA + content verification
- **Auditoría robusta**: Detecta más vulnerabilidades que npm audit

##### 2. **Eficiencia**

- **70% menos espacio en disco**: Hard links + store global
- **2x más rápido**: Instalación y resolución de dependencias
- **Determinismo**: pnpm-lock.yaml más confiable que package-lock.json

#### 📋 **Comandos Obligatorios (Frontend)**

```bash
# ✅ USAR SIEMPRE
pnpm install                    # Instalar dependencias
pnpm install --frozen-lockfile  # En CI/CD
pnpm add <package>             # Agregar dependencia
pnpm remove <package>          # Remover dependencia
pnpm update                    # Actualizar dependencias
pnpm audit                     # Auditoría de seguridad
pnpm run <script>              # Ejecutar scripts

# ❌ PROHIBIDO
npm install    # Usar pnpm install
npm ci         # Usar pnpm install --frozen-lockfile
npm run        # Usar pnpm run
yarn install   # Solo PNPM permitido
```

#### ⚙️ **Configuración Frontend**

##### .pnpmrc (fe/)

```ini
strict-peer-dependencies=true
auto-install-peers=false
enable-pre-post-scripts=false
registry=https://registry.npmjs.org/
verify-store-integrity=true
frozen-lockfile=true
```

##### package.json engines

```json
{
  "engines": {
    "node": ">=22.0.0",
    "pnpm": ">=9.0.0"
  },
  "packageManager": "pnpm@9.0.0"
}
```

### Backend: Cargo como Gestor de Dependencias Rust

#### 📋 **Comandos Obligatorios (Backend)**

```bash
# ✅ DESARROLLO
cargo build                    # Compilar
cargo build --release          # Compilar para producción
cargo run                      # Ejecutar
cargo test                     # Ejecutar tests
cargo clippy                   # Linter
cargo fmt                      # Formatear código
cargo audit                    # Auditoría de seguridad

# ✅ DEPENDENCIAS
cargo add <crate>              # Agregar dependencia
cargo remove <crate>           # Remover dependencia
cargo update                   # Actualizar dependencias
```

#### ⚙️ **Configuración Cargo.toml**

```toml
[package]
name = "acc-lms-api"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
jsonwebtoken = "9"
bcrypt = "0.15"
validator = { version = "0.16", features = ["derive"] }
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
mockall = "0.12"
```

---

## 🏗️ Stack-Specific Standards

### Frontend (React 19 + Vite)

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "pnpm lint --fix",
    "format": "prettier --write src/",
    "test": "vitest",
    "test:coverage": "vitest --coverage"
  }
}
```

### Backend (Rust)

```toml
# Justfile o Makefile
[tasks]
dev = "cargo watch -x run"
build = "cargo build --release"
test = "cargo test"
lint = "cargo clippy -- -D warnings"
format = "cargo fmt"
audit = "cargo audit"
migrate = "sqlx migrate run"
```

---

## 🚀 CI/CD Integration

### GitHub Actions Workflow

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  # Frontend
  frontend:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: frontend
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Setup PNPM
        uses: pnpm/action-setup@v2
        with:
          version: 9

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Lint
        run: pnpm lint

      - name: Test
        run: pnpm test

      - name: Build
        run: pnpm build

  # Backend
  backend:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: be
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-action@stable
        with:
          components: clippy, rustfmt

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Format check
        run: cargo fmt --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Test
        run: cargo test

      - name: Build
        run: cargo build --release

      - name: Security audit
        run: cargo audit
```

### Docker Integration

#### Frontend Dockerfile

```dockerfile
# Build stage
FROM node:22-alpine AS builder

RUN npm install -g pnpm@9

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# Production stage
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

#### Backend Dockerfile

```dockerfile
# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN touch src/main.rs
RUN cargo build --release

# Production stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/acc-lms-api /usr/local/bin/

EXPOSE 8080

CMD ["acc-lms-api"]
```

---

## 📁 Estructura del Proyecto

### Frontend (React 19)

```
fe/
├── src/
│   ├── components/           # Componentes reutilizables
│   │   ├── ui/              # Componentes UI básicos
│   │   └── features/        # Componentes específicos de features
│   ├── pages/               # Páginas/vistas
│   ├── hooks/               # Custom hooks
│   ├── context/             # React Context providers
│   ├── services/            # API clients
│   ├── utils/               # Utilidades
│   ├── types/               # TypeScript types
│   └── styles/              # Estilos globales
├── public/
├── tests/
├── package.json
├── pnpm-lock.yaml
├── vite.config.ts
├── tsconfig.json
└── tailwind.config.js
```

### Backend (Rust Clean Architecture)

```
be/
├── src/
│   ├── domain/              # Entidades y reglas de negocio
│   │   ├── entities/
│   │   ├── value_objects/
│   │   └── services/
│   ├── application/         # Casos de uso
│   │   ├── use_cases/
│   │   ├── ports/           # Interfaces/traits
│   │   └── dtos/
│   ├── infrastructure/      # Implementaciones externas
│   │   ├── repositories/
│   │   ├── database/
│   │   ├── cache/
│   │   └── external/
│   └── interfaces/          # HTTP layer
│       ├── http/
│       │   ├── routes/
│       │   ├── handlers/
│       │   └── middleware/
│       └── dto/
├── migrations/
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── Cargo.toml
├── Cargo.lock
└── .env.example
```

---

## 🔒 Security Policies

### Dependency Management

- **Audit frequency**: Semanal obligatorio
- **Update strategy**: Patch automático, minor/major manual
- **Vulnerability response**: <24h para critical, <7d para high

### Rust Security

```bash
# Instalar cargo-audit
cargo install cargo-audit

# Ejecutar auditoría
cargo audit

# Verificar dependencias desactualizadas
cargo outdated
```

---

## 📊 Monitoring & Metrics

### Performance Tracking

- **Frontend bundle size**: <500KB gzipped
- **API latency P95**: <100ms
- **Test coverage**: >80%

### Code Quality

- **Rust**: clippy sin warnings, fmt aplicado
- **TypeScript**: ESLint sin errores, Prettier aplicado

---

## 🎯 Enforcement

### Pre-commit Hooks

```bash
# Frontend: .husky/pre-commit
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

cd frontend
pnpm lint
pnpm run type-check
```

```bash
# Backend: pre-push hook
#!/usr/bin/env sh
cd be
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

### Validation Rules

```bash
# Verificar que se usa PNPM en frontend
if [ -f "fe/package-lock.json" ] || [ -f "fe/yarn.lock" ]; then
  echo "❌ Solo pnpm-lock.yaml permitido en fe/"
  exit 1
fi

# Verificar formato Rust
if ! cargo fmt --check; then
  echo "❌ Código Rust no formateado. Ejecutar: cargo fmt"
  exit 1
fi
```

---

## 🔧 Herramientas de Desarrollo

### Requeridas

| Herramienta   | Propósito        | Instalación                      |
| ------------- | ---------------- | -------------------------------- |
| Node.js 22+   | Runtime frontend | `nvm install 22`                 |
| PNPM 9+       | Package manager  | `npm install -g pnpm`            |
| Rust 1.75+    | Backend          | `rustup update stable`           |
| Docker        | Contenedores     | [docker.com](https://docker.com) |
| PostgreSQL 16 | Base de datos    | Docker o local                   |
| Redis 7       | Cache            | Docker o local                   |

### Recomendadas (VS Code Extensions)

- rust-analyzer
- ESLint
- Prettier
- Tailwind CSS IntelliSense
- GitLens
- Docker
- Thunder Client / REST Client

---

**Última actualización**: Diciembre 2025  
**Mantenedor**: Tech Lead Team
