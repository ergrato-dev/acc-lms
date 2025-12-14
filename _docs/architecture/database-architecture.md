# ACC LMS — Arquitectura de Base de Datos

**Versión:** 2025-08-08  
**Estado:** Diseño completo para implementación

---

## 🏗️ Estrategia Multi-Engine

### Base de Datos Principal: PostgreSQL

- **Datos transaccionales:** Usuarios, cursos, inscripciones, pagos
- **ACID compliance:** Para operaciones críticas de negocio
- **JSON support:** Para metadatos flexibles y HATEOAS links

### MongoDB: Datos de Contenido

- **Content storage:** Videos, documentos, metadatos de archivos
- **Notificaciones:** Templates y historial de envíos
- **Schemas flexibles:** Para contenido educativo variable

### ClickHouse: Analytics

- **Event streaming:** User interactions, video views, quiz attempts
- **Real-time analytics:** Dashboards y reportes de BI
- **Time-series data:** Métricas de performance y uso

---

## 🔗 Diseño HATEOAS-Aware

### Resource Identification Strategy

```sql
-- PostgreSQL: URLs consistentes para HATEOAS
CREATE TABLE resource_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resource_type VARCHAR(50) NOT NULL, -- 'course', 'user', 'enrollment'
    resource_id UUID NOT NULL,
    rel_type VARCHAR(50) NOT NULL,      -- 'self', 'edit', 'enroll'
    href_template VARCHAR(500) NOT NULL,-- '/api/v1/courses/{id}'
    method VARCHAR(10) DEFAULT 'GET',
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT now()
);

-- Index para lookups rápidos
CREATE INDEX idx_resource_links_lookup
ON resource_links(resource_type, resource_id)
WHERE enabled = true;
```

### HATEOAS Link Generation

```sql
-- Función para generar links dinámicos
CREATE OR REPLACE FUNCTION generate_hateoas_links(
    p_resource_type VARCHAR,
    p_resource_id UUID,
    p_user_role VARCHAR DEFAULT 'student'
) RETURNS JSONB AS $$
DECLARE
    links JSONB := '{}';
    link_record RECORD;
BEGIN
    FOR link_record IN
        SELECT rel_type, href_template, method
        FROM resource_links rl
        JOIN resource_permissions rp ON rl.rel_type = rp.rel_type
        WHERE rl.resource_type = p_resource_type
        AND rl.enabled = true
        AND rp.role = p_user_role
    LOOP
        links := jsonb_set(
            links,
            ARRAY[link_record.rel_type],
            jsonb_build_object(
                'href', replace(link_record.href_template, '{id}', p_resource_id::text),
                'method', link_record.method
            )
        );
    END LOOP;

    RETURN links;
END;
$$ LANGUAGE plpgsql;
```

---

## 📊 Schema Design

### Core Entities con HATEOAS Support

```sql
-- Courses con embedded links
CREATE TABLE courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    instructor_id UUID REFERENCES users(id),
    price DECIMAL(10,2),
    status course_status DEFAULT 'draft',

    -- HATEOAS metadata
    resource_version INTEGER DEFAULT 1,
    last_modified TIMESTAMP DEFAULT now(),

    -- JSON para metadatos flexibles
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

-- View con HATEOAS links embebidos
CREATE VIEW courses_with_links AS
SELECT
    c.*,
    generate_hateoas_links('course', c.id, 'student') as _links
FROM courses c
WHERE c.status = 'published';
```

### API Response Structure

```json
{
  "id": "course-123",
  "title": "Clean Architecture",
  "slug": "clean-architecture-microservices",
  "instructor_id": "instructor-456",
  "price": 99.99,
  "status": "published",
  "metadata": {
    "duration_hours": 8,
    "difficulty": "intermediate",
    "tags": ["architecture", "microservices"]
  },
  "_links": {
    "self": {
      "href": "/api/v1/courses/course-123",
      "method": "GET"
    },
    "enroll": {
      "href": "/api/v1/enrollments",
      "method": "POST"
    },
    "modules": {
      "href": "/api/v1/courses/course-123/modules",
      "method": "GET"
    },
    "reviews": {
      "href": "/api/v1/courses/course-123/reviews",
      "method": "GET"
    }
  },
  "resource_version": 1,
  "last_modified": "2025-08-08T10:30:00Z"
}
```

---

## 🚀 Microservices Data Strategy

### Service Ownership

| Servicio                | Base Datos          | Responsabilidad              |
| ----------------------- | ------------------- | ---------------------------- |
| `auth-service`          | PostgreSQL          | Users, sessions, permissions |
| `courses-service`       | PostgreSQL          | Courses, modules, lessons    |
| `content-service`       | MongoDB             | Videos, docs, multimedia     |
| `enrollments-service`   | PostgreSQL          | Enrollments, progress        |
| `analytics-service`     | ClickHouse          | Events, metrics, reports     |
| `ai-service`            | PostgreSQL + Vector | Recommendations, ML models   |
| `notifications-service` | MongoDB             | Templates, delivery status   |

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

## 🔄 Migration Strategy

### Database Versioning

```bash
# Estructura de migraciones
db/migrations/
├── postgresql/
│   ├── 001_initial_schema.sql
│   ├── 002_hateoas_support.sql
│   └── 003_analytics_events.sql
├── mongodb/
│   ├── 001_content_collections.js
│   └── 002_notification_templates.js
└── clickhouse/
    ├── 001_analytics_tables.sql
    └── 002_user_behavior_views.sql
```

### HATEOAS Migration

```sql
-- Migración para soporte HATEOAS
-- 002_hateoas_support.sql

-- Tabla de configuración de links
INSERT INTO resource_links (resource_type, rel_type, href_template, method) VALUES
('course', 'self', '/api/v1/courses/{id}', 'GET'),
('course', 'enroll', '/api/v1/enrollments', 'POST'),
('course', 'modules', '/api/v1/courses/{id}/modules', 'GET'),
('course', 'edit', '/api/v1/courses/{id}', 'PUT'),
('user', 'self', '/api/v1/users/{id}', 'GET'),
('user', 'courses', '/api/v1/users/{id}/courses', 'GET'),
('enrollment', 'self', '/api/v1/enrollments/{id}', 'GET'),
('enrollment', 'progress', '/api/v1/enrollments/{id}/progress', 'GET');

-- Permisos por rol
CREATE TABLE resource_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rel_type VARCHAR(50) NOT NULL,
    role VARCHAR(50) NOT NULL,
    allowed BOOLEAN DEFAULT true
);

INSERT INTO resource_permissions (rel_type, role, allowed) VALUES
('enroll', 'student', true),
('enroll', 'instructor', false),
('edit', 'instructor', true),
('edit', 'student', false),
('self', 'student', true),
('self', 'instructor', true);
```
