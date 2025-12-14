# Documentación del Proyecto ACC LMS

Esta carpeta contiene toda la documentación técnica y de negocio del proyecto, organizada por categorías.

## 🏗️ Stack Tecnológico

- **Frontend:** React 19 + Vite + TypeScript + Tailwind CSS
- **Backend:** Rust REST API (Actix-web/Axum) + Clean Architecture
- **Base de Datos:** PostgreSQL 16 + Redis 7
- **Infraestructura:** Docker + Kubernetes + Traefik

## 📁 Estructura Organizacional

```
_docs/
├── architecture/           # Arquitectura técnica y diseño
├── business/              # Requisitos y especificaciones de negocio
├── development/           # Estándares y procesos de desarrollo
├── operations/            # Gestión de operaciones y métricas
└── security/              # Políticas de seguridad (LOCAL ONLY)
```

## 📋 Índice de Documentación

### 🏗️ Architecture

- [`database-architecture.md`](architecture/database-architecture.md) - Diseño de base de datos
- [`infrastructure-traefik.md`](architecture/infrastructure-traefik.md) - Configuración de API Gateway
- [`uuid-security-analysis.md`](architecture/uuid-security-analysis.md) - Análisis de seguridad UUIDs

### 💼 Business

- [`functional-requirements.md`](business/functional-requirements.md) - Requisitos funcionales completos
- [`non-functional-requirements.md`](business/non-functional-requirements.md) - SLOs y métricas de calidad
- [`user-stories.md`](business/user-stories.md) - Historias de usuario y criterios
- [`info-proyecto.md`](business/info-proyecto.md) - Información general del proyecto

### 🔧 Development

- [`development-standards.md`](development/development-standards.md) - Estándares React + Rust

### ⚙️ Operations

- [`monorepo-separation-scorecard.md`](operations/monorepo-separation-scorecard.md) - Métricas para decisión de separación

### 🔐 Security _(Local Only)_

- `granular-permissions.md` - Políticas granulares de permisos
- `cybersecurity-policies.md` - Políticas completas de ciberseguridad

> ⚠️ **Nota de Seguridad**: Los archivos en `security/` no se sincronizan con GitHub por razones de seguridad. Se mantienen únicamente en el repositorio local.

## 📖 Guías de Navegación

### Para Desarrolladores

1. Empezar con [`development-standards.md`](development/development-standards.md)
2. Revisar arquitectura en [`database-architecture.md`](architecture/database-architecture.md)
3. Consultar requisitos no funcionales en [`non-functional-requirements.md`](business/non-functional-requirements.md)

### Para Product Managers

1. Revisar [`functional-requirements.md`](business/functional-requirements.md)
2. Consultar [`user-stories.md`](business/user-stories.md)
3. Verificar SLOs en [`non-functional-requirements.md`](business/non-functional-requirements.md)

### Para DevOps/SRE

1. Configuración de infraestructura: [`infrastructure-traefik.md`](architecture/infrastructure-traefik.md)
2. Métricas de operación: [`monorepo-separation-scorecard.md`](operations/monorepo-separation-scorecard.md)
3. Políticas de seguridad: `security/` (local only)

---

**Última actualización**: Diciembre 2025  
**Mantenedores**: Tech Lead Team
