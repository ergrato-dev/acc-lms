# Plan de Actualización de Documentación - ACC LMS

**Fecha:** 2025-12-14  
**Actualización:** 2025-12-14 (Fase 1 completada)  
**Basado en:** Análisis del site_map completo vs documentación actual  
**Estado:** ✅ Fase 1 COMPLETADA

---

## 📋 Resumen Ejecutivo

El análisis del `site_map/` revela **funcionalidades y rutas documentadas detalladamente** que actualmente no están reflejadas en:

- `functional-requirements.md` (RFs)
- `non-functional-requirements.md` (RNFs)
- `user-stories.md` (Historias de Usuario)
- `info-proyecto.md` (Blueprint del proyecto)

### Progreso de Actualización

| Área                                                | Estado Anterior                   | Estado Actual                          |
| --------------------------------------------------- | --------------------------------- | -------------------------------------- |
| Chatbot + Knowledge Base                            | ⚠️ Solo RF-SUPPORT-001/002 básico | ✅ RF-CHATBOT + RF-KB                  |
| Cumplimiento Normativo (GDPR/LGPD/CCPA/Habeas Data) | ❌ No existía                     | ✅ compliance-requirements.md (19 RFs) |
| Panel Admin completo                                | ⚠️ Parcial                        | ✅ RF-ADMIN-001..007                   |
| Instructor Analytics/Moderación                     | ⚠️ Parcial                        | ✅ RF-INSTRUCTOR-001..007              |
| Player/Learning Experience                          | ⚠️ Solo endpoints básicos         | ✅ RF-STUDENT-001..005                 |
| Seguridad y Auditoría Admin                         | ⚠️ Solo RNF-003 básico            | ✅ RF-ADMIN-007 + RNF-013              |
| Suscripciones/Planes                                | ❌ No en RFs                      | ✅ RF-SUB-001..003                     |
| Foro/Q&A/Comunidad                                  | ❌ No en RFs                      | ✅ RF-INSTRUCTOR-005, RF-STUDENT-004   |
| RNFs Legal/Accesibilidad                            | ⚠️ Básico                         | ✅ RNF-013, RNF-014, RNF-015           |
| User Stories nuevos                                 | 20 historias                      | ✅ 39 historias (US-050..US-096)       |
| info-proyecto.md servicios                          | 11 servicios                      | ✅ 16 servicios                        |

### Documentos Actualizados

1. ✅ `_docs/business/compliance-requirements.md` - CREADO (19 RF-COMPLIANCE)
2. ✅ `_docs/business/functional-requirements.md` - ACTUALIZADO (85+ RFs)
3. ✅ `_docs/business/non-functional-requirements.md` - ACTUALIZADO (RNF-013..015)
4. ✅ `_docs/business/user-stories.md` - ACTUALIZADO (39 US nuevas, 443 story points)
5. ✅ `_docs/business/info-proyecto.md` - ACTUALIZADO (16 servicios, nuevos endpoints)

---

## ✅ COMPLETADO - Fase 1: Compliance y Fundamentos

### 1. Nuevo Documento: `compliance-requirements.md`

El archivo `8-CUMPLIMIENTO_NORMATIVO_Y_PROTECCIÓN_DE_DATOS.md` documenta requisitos legales extensos que **deben** estar en la documentación técnica:

#### Contenido a crear:

```markdown
# RF-COMPLIANCE: Requisitos de Cumplimiento Normativo

## RF-COMPLIANCE-001: Política de Privacidad Dinámica

- /politica-de-privacidad con cumplimiento:
  - Ley 1581/2012 Habeas Data (Colombia)
  - GDPR (UE)
  - CCPA/CPRA (California)
  - LGPD (Brasil)
- Versionado de políticas
- Selector de idioma

## RF-COMPLIANCE-002: Derechos ARCO (Colombia)

- Acceso, Rectificación, Cancelación, Oposición
- Endpoint: POST /api/v1/privacy/arco-request
- Portal autoservicio: /mi-privacidad

## RF-COMPLIANCE-003: Derechos GDPR

- Portabilidad de datos (art. 20)
- Derecho al olvido (art. 17)
- Limitación tratamiento (art. 18)
- Endpoint: POST /api/v1/privacy/gdpr-request

## RF-COMPLIANCE-004: CCPA Do Not Sell

- Opt-out venta datos personales
- /no-vender-mi-informacion
- Verificación identidad

## RF-COMPLIANCE-005: Gestión de Cookies

- Banner de consentimiento (ePrivacy)
- Categorías: Esenciales, Funcionales, Analíticas, Marketing
- /configuracion-cookies
- Endpoint: POST /api/v1/consent/cookies

## RF-COMPLIANCE-006: Menores de Edad

- Verificación edad en registro
- COPPA (13 años US), Ley 1098 (18 años Colombia)
- Consentimiento parental

## RF-COMPLIANCE-007: Data Breach Notification

- 72 horas a autoridad control (GDPR)
- SIC Colombia (Habeas Data)
- Notificación a afectados
```

---

### 2. Actualizar `functional-requirements.md` - Nuevos RFs

#### RF-CHATBOT (Expandir RF-SUPPORT-001)

```markdown
## RF-CHATBOT-001: Widget Global

- Botón flotante en todas las páginas
- Panel expandible con threading
- Soporte multimedia (cards, carousels, code snippets)

## RF-CHATBOT-002: Sugerencias Contextuales por Rol

- Anonymous: FAQ generales
- Student: Progreso, certificados, videos
- Instructor: Creación cursos, analytics, pagos
- Admin: Health check, configuración, reportes

## RF-CHATBOT-003: Escalación Inteligente

- Triggers automáticos (confidence <60%, sentiment negativo)
- Transferencia contexto completo a agente
- Creación automática de tickets

## RF-CHATBOT-004: Feedback y Mejora Continua

- Thumbs up/down por respuesta
- Marcado para entrenamiento
- Analytics de gaps en KB
```

#### RF-KB: Knowledge Base Pública

```markdown
## RF-KB-001: Portal Autoservicio

- /ayuda con búsqueda prominente
- Categorías navegables
- Artículos populares

## RF-KB-002: Artículos KB

- Editor WYSIWYG/Markdown
- Templates (How-to, Troubleshooting, FAQ)
- Versiones multiidioma
- SEO metadata

## RF-KB-003: Integración Chatbot

- Keywords/triggers por artículo
- Intent mapping
- Respuesta resumida automática
```

#### RF-ADMIN: Panel Administrativo Completo

```markdown
## RF-ADMIN-001: Dashboard Ejecutivo

- KPIs principales con comparativa período anterior
- Alertas críticas (seguridad, pagos, contenido)
- Gráficos ejecutivos (crecimiento, revenue)

## RF-ADMIN-002: Gestión Usuarios Avanzada

- Filtros granulares (rol, estado, país, 2FA, plan)
- Acciones masivas (email, suspender, tags)
- Vista individual con timeline actividad
- Impersonación (login como usuario para debug)

## RF-ADMIN-003: Moderación de Cursos

- Workflow revisión/aprobación
- Checklist calidad
- Badges (Bestseller, Trending, Editor's Choice)

## RF-ADMIN-004: Gestión Financiera

- Dashboard financiero (MRR, ARR, ARPU)
- Pagos a instructores (ciclos, umbral mínimo)
- Gestión reembolsos con políticas
- Cupones plataforma
- Reportes fiscales

## RF-ADMIN-005: Analytics Plataforma

- Análisis usuarios (demografía, cohortes, churn)
- Análisis cursos (engagement, rankings)
- Análisis instructores (performance, earnings)
- Reportes customizados con query builder

## RF-ADMIN-006: Configuración Sistema

- Localización (idiomas, formatos regionales)
- Email (SMTP, templates, logs)
- Autenticación (OAuth, SSO, políticas password)
- Roles y permisos granulares
- Pagos (gateways, comisiones, impuestos)
- Multimedia (storage, CDN, procesamiento video)

## RF-ADMIN-007: Seguridad y Auditoría

- Dashboard seguridad (score, alertas)
- Logs auditoría con filtros granulares
- Gestión sesiones y accesos
- IPs bloqueadas/whitelist
- Escaneo vulnerabilidades
- Gestión API keys
```

#### RF-INSTRUCTOR: Panel Instructor Completo

```markdown
## RF-INSTRUCTOR-001: Dashboard Instructor

- Stats principales (estudiantes, cursos, ingresos, rating)
- Acciones rápidas (tareas pendientes, preguntas)
- Actividad reciente

## RF-INSTRUCTOR-002: Quiz Builder

- Tipos: Opción múltiple, V/F, Respuesta corta, Matching, Ensayo
- Configuración (tiempo, intentos, aleatorio)
- Banco de preguntas
- Import CSV/Excel

## RF-INSTRUCTOR-003: Gestión Estudiantes

- Lista con progreso, última actividad
- Acciones masivas (email, acceso especial)
- Vista individual con timeline

## RF-INSTRUCTOR-004: Calificaciones

- Workflow calificación tareas
- Rúbricas
- Feedback multimedia
- Exportación

## RF-INSTRUCTOR-005: Moderación Foro

- Respuestas oficiales destacadas
- Moderación (aprobar, rechazar, eliminar)
- Convertir a FAQ

## RF-INSTRUCTOR-006: Analytics Curso

- Engagement por sección/lección
- Heatmap partes video más vistas/saltadas
- Performance evaluaciones
- Insights IA

## RF-INSTRUCTOR-007: Biblioteca Multimedia

- Gestión centralizada archivos
- Quotas almacenamiento
- Reemplazo manteniendo referencias
```

#### RF-STUDENT: Experiencia Estudiante

```markdown
## RF-STUDENT-001: Player Avanzado

- Layout: Sidebar temario + área contenido + tabs inferiores
- Video: HLS adaptativo, velocidad 0.5x-2x, calidad auto
- Picture-in-picture
- Keyboard shortcuts
- Auto-advance

## RF-STUDENT-002: Notas Personales

- Por lección con timestamp linking
- Markdown support
- Búsqueda y exportación

## RF-STUDENT-003: Wishlist

- Cursos guardados
- Notificación descuentos

## RF-STUDENT-004: Foro Curso

- Crear hilos con tags
- Votos arriba/abajo
- Respuesta aceptada
- Seguir hilo

## RF-STUDENT-005: Mensajería

- Chat con instructores
- Tiempo real (WebSocket)
```

#### RF-SUBSCRIPTION: Sistema de Suscripciones

```markdown
## RF-SUB-001: Gestión Planes

- Planes: Free, Pro, Premium
- Features por plan
- Trials
- Comparativa

## RF-SUB-002: Billing

- Ciclos (mensual, anual)
- Métodos pago guardados
- Historial facturas (PDF descargable)

## RF-SUB-003: Lifecycle

- Upgrade/Downgrade
- Cancelación con encuesta
- Reactivación
```

---

### 3. Actualizar `non-functional-requirements.md`

#### RNF-008: Cumplimiento Legal (Nuevo)

```markdown
## RNF-008: Cumplimiento Legal y Privacidad

### Normativas soportadas

- **Colombia:** Ley 1581/2012, Decreto 1377/2013
- **UE:** GDPR, ePrivacy Directive
- **US:** CCPA/CPRA, COPPA
- **Brasil:** LGPD

### Implementación técnica

- Encriptación datos sensibles (AES-256 at rest)
- Tokenización tarjetas (nunca almacenamos completas)
- Logs acceso a datos personales auditados
- Data Processing Agreements con proveedores
- Transferencias internacionales vía SCC

### Retención de datos

| Tipo dato      | Retención          | Base legal       |
| -------------- | ------------------ | ---------------- |
| Cuenta usuario | Mientras activa    | Contrato         |
| Datos fiscales | 10 años            | Ley tributaria   |
| Transacciones  | 5 años mínimo      | Comercial        |
| Logs seguridad | 6m-2 años          | Interés legítimo |
| Marketing      | 2 años inactividad | Consentimiento   |

### Brechas de seguridad

- Notificación 72h a autoridad (GDPR)
- Plan respuesta incidentes documentado
- Comunicación transparente a afectados
```

#### RNF-009: Accesibilidad (Nuevo)

```markdown
## RNF-009: Accesibilidad Web

### Estándar objetivo

- **WCAG 2.1 Nivel AA**

### Requisitos

- Navegación completa por teclado
- Screen reader compatible
- Alto contraste disponible
- Focus indicators claros
- Alt text en imágenes
- Transcripciones/subtítulos video
```

---

### 4. Actualizar `user-stories.md`

#### Nuevas Historias de Usuario

```markdown
## Epic: Cumplimiento y Privacidad

### US-050: Ejercer Derechos ARCO 🔥

**Como** usuario registrado en Colombia
**Quiero** solicitar acceso, rectificación o eliminación de mis datos
**Para** ejercer mis derechos bajo Ley Habeas Data

**Criterios:**

- Dado que accedo a /mi-privacidad
- Cuando selecciono tipo de solicitud (Acceso/Rectificación/Cancelación/Oposición)
- Entonces puedo enviar solicitud verificando mi identidad
- Y recibo respuesta en máximo 15 días hábiles

### US-051: Exportar Mis Datos (Portabilidad) ⚡

**Como** usuario en la UE
**Quiero** descargar todos mis datos personales
**Para** ejercer mi derecho de portabilidad GDPR

**Criterios:**

- Dado que solicito exportación
- Cuando proceso se completa
- Entonces recibo archivo JSON/CSV estructurado con todos mis datos
- Y el download link expira en 24 horas

### US-052: Opt-Out Venta Datos 🔥

**Como** residente de California
**Quiero** que no vendan mi información personal
**Para** ejercer derechos CCPA

**Criterios:**

- Dado que accedo a /no-vender-mi-informacion
- Cuando confirmo opt-out
- Entonces mi preferencia se registra inmediatamente
- Y recibo confirmación email

---

## Epic: Chatbot y Soporte

### US-060: Obtener Ayuda Vía Chatbot 🔥

**Como** cualquier usuario
**Quiero** hacer preguntas al chatbot integrado
**Para** resolver dudas sin esperar soporte humano

**Criterios:**

- Dado que abro el widget de chat
- Cuando escribo una pregunta sobre reembolsos
- Entonces recibo respuesta relevante de la KB en <3 segundos
- Y puedo indicar si fue útil (👍👎)

### US-061: Escalar a Agente Humano ⚡

**Como** usuario con problema complejo
**Quiero** hablar con un humano cuando el bot no resuelve
**Para** obtener asistencia personalizada

**Criterios:**

- Dado que el bot no puede resolver (o digo "hablar con persona")
- Cuando se inicia escalación
- Entonces veo tiempo estimado de espera
- Y el agente recibe contexto completo de mi conversación

### US-062: Buscar en Knowledge Base 🔥

**Como** usuario
**Quiero** buscar artículos de ayuda en el portal
**Para** encontrar soluciones autoservicio

**Criterios:**

- Dado que accedo a /ayuda
- Cuando busco "certificado"
- Entonces veo resultados relevantes con términos resaltados
- Y puedo filtrar por categoría

---

## Epic: Panel Administrador

### US-070: Monitorear Dashboard Seguridad 🔥

**Como** administrador
**Quiero** ver score de seguridad y alertas críticas
**Para** mantener la plataforma protegida

**Criterios:**

- Dado que accedo a /admin/seguridad
- Cuando veo el dashboard
- Entonces veo score global (0-100) con tendencia
- Y alertas priorizadas con acciones recomendadas

### US-071: Auditar Actividad Usuario ⚡

**Como** administrador
**Quiero** ver historial completo de acciones de un usuario
**Para** investigar incidentes o soportar compliance

**Criterios:**

- Dado que busco un usuario sospechoso
- Cuando accedo a su perfil admin
- Entonces veo timeline de todas sus acciones
- Y puedo filtrar por tipo de evento
- Y cada evento tiene detalles expandibles (IP, dispositivo, cambios)

### US-072: Gestionar Reembolsos 🔥

**Como** administrador financiero
**Quiero** procesar solicitudes de reembolso
**Para** mantener satisfacción cliente y compliance

**Criterios:**

- Dado que hay reembolsos pendientes
- Cuando reviso una solicitud
- Entonces veo: usuario, curso, días desde compra, % progreso
- Y puedo aprobar total/parcial o rechazar con mensaje

---

## Epic: Panel Instructor

### US-080: Construir Quiz 🔥

**Como** instructor
**Quiero** crear evaluaciones con diferentes tipos de pregunta
**Para** evaluar comprensión de mis estudiantes

**Criterios:**

- Dado que estoy en quiz builder
- Cuando agrego preguntas de opción múltiple y verdadero/falso
- Entonces puedo configurar puntaje, explicación y orden
- Y ver preview como estudiante

### US-081: Calificar Tarea con Rúbrica ⚡

**Como** instructor
**Quiero** calificar entregas usando criterios predefinidos
**Para** ser consistente y dar feedback estructurado

**Criterios:**

- Dado que tengo entregas pendientes
- Cuando abro una entrega
- Entonces veo rúbrica con criterios y puedo asignar puntos
- Y agregar feedback texto/audio/video
- Y navegar a siguiente entrega sin salir

### US-082: Ver Analytics de Lección ⚡

**Como** instructor
**Quiero** saber qué partes del video repiten/saltan mis estudiantes
**Para** mejorar el contenido problemático

**Criterios:**

- Dado que accedo a analytics de una lección
- Cuando veo el heatmap de video
- Entonces identifico segundos más reproducidos (alto engagement)
- Y segundos más saltados (bajo engagement)
```

---

### 5. Actualizar `.vscode/copilot-instructions.md`

#### Nuevas secciones a agregar:

```markdown
## Compliance Service

### Endpoints requeridos

POST /api/v1/privacy/arco-request # Solicitudes ARCO (Colombia)
POST /api/v1/privacy/gdpr-request # Solicitudes GDPR
GET /api/v1/privacy/export-data # Exportar datos (portabilidad)
POST /api/v1/privacy/delete-account # Derecho al olvido
POST /api/v1/consent/cookies # Preferencias cookies
GET /api/v1/consent/status # Estado consentimientos

### Reglas de negocio

- Verificación identidad antes de procesar solicitudes
- Respuesta máxima 15 días hábiles (Colombia), 30 días (GDPR)
- Logs de todas las solicitudes para auditoría
- No eliminar datos requeridos por ley (fiscal, fraude)

## Chatbot Service

### Arquitectura

Frontend Widget → chatbot-service (Rasa/Custom) → ai-service (OpenAI fallback)
↓
knowledge-base (PostgreSQL)

### Endpoints requeridos

POST /api/v1/chatbot/message # Enviar mensaje
GET /api/v1/chatbot/history/:sessionId
POST /api/v1/chatbot/feedback # Thumbs up/down
POST /api/v1/chatbot/escalate # Escalar a humano
GET /api/v1/kb/search # Buscar knowledge base
GET /api/v1/kb/article/:slug # Artículo individual

### Integraciones

- WebSocket para conversaciones en tiempo real
- Redis para sesiones de chat
- PostgreSQL para KB y historial

## Admin Service (Extender)

### Nuevos endpoints críticos

GET /admin/security/dashboard # Score y alertas
GET /admin/audit/logs # Logs auditoría
GET /admin/audit/logs/:eventId # Detalle evento
POST /admin/users/:id/impersonate # Login como usuario
GET /admin/finance/dashboard # KPIs financieros
GET /admin/finance/instructor-payouts
POST /admin/finance/refunds/:id/process
GET /admin/analytics/custom-report # Query builder

## Restricciones de Seguridad

### Datos sensibles - NUNCA loguear

- Passwords (ni hashes completos)
- Tokens de acceso/refresh
- Números tarjeta completos
- Documentos identidadvamos con courses-service

### Auditoría obligatoria

- Todos los cambios en /admin/\*
- Accesos a datos personales
- Transacciones financieras
- Cambios de rol/permisos
- Login/logout/password changes

### Encriptación requerida

- Emails en reposo (AES-256)
- Documentos sensibles
- Backups
- Comunicaciones inter-servicio (mTLS)
```

---

## 📅 Plan de Ejecución

### Fase 1: Crítico (Semana 1-2)

| Tarea                              | Prioridad  | Estimación |
| ---------------------------------- | ---------- | ---------- |
| Crear `compliance-requirements.md` | 🔴 Crítico | 4h         |
| Actualizar RNF-008 (Legal)         | 🔴 Crítico | 2h         |
| RF-COMPLIANCE completos            | 🔴 Crítico | 4h         |
| US Compliance (50-52)              | 🔴 Crítico | 2h         |

### Fase 2: Alto (Semana 3-4)

| Tarea                 | Prioridad | Estimación |
| --------------------- | --------- | ---------- |
| RF-CHATBOT expandidos | 🟡 Alto   | 3h         |
| RF-KB knowledge base  | 🟡 Alto   | 2h         |
| RF-ADMIN completos    | 🟡 Alto   | 4h         |
| US Chatbot (60-62)    | 🟡 Alto   | 2h         |
| US Admin (70-72)      | 🟡 Alto   | 2h         |

### Fase 3: Medio (Semana 5-6)

| Tarea                           | Prioridad | Estimación |
| ------------------------------- | --------- | ---------- |
| RF-INSTRUCTOR expandidos        | 🟢 Medio  | 3h         |
| RF-STUDENT experiencia          | 🟢 Medio  | 2h         |
| RF-SUBSCRIPTION                 | 🟢 Medio  | 2h         |
| US Instructor (80-82)           | 🟢 Medio  | 2h         |
| Actualizar copilot-instructions | 🟢 Medio  | 3h         |

### Fase 4: Validación (Semana 7)

| Tarea                             | Prioridad | Estimación |
| --------------------------------- | --------- | ---------- |
| Review consistencia RF ↔ site_map | ⚪        | 4h         |
| Actualizar trazabilidad           | ⚪        | 2h         |
| Generar changelog                 | ⚪        | 1h         |

---

## ✅ Checklist de Validación Final

- [ ] Todos los endpoints del site_map tienen RF correspondiente
- [ ] Todas las páginas del site_map tienen US correspondiente
- [ ] RNFs cubren aspectos de compliance, seguridad, accesibilidad
- [ ] copilot-instructions refleja arquitectura completa
- [ ] Trazabilidad RF ↔ endpoint ↔ tabla actualizada
- [ ] Numeración de RFs es consistente y sin gaps
- [ ] Prioridades alineadas con roadmap implementación

---

## 📊 Métricas de Cobertura Esperada

| Documento                  | Antes | Después |
| -------------------------- | ----- | ------- |
| RFs totales                | ~50   | ~80     |
| User Stories               | ~30   | ~60     |
| Páginas site_map cubiertas | 60%   | 95%     |
| Endpoints documentados     | 70%   | 95%     |
| Compliance requirements    | 0%    | 100%    |

---

**Autor:** GitHub Copilot  
**Próxima revisión:** Al completar Fase 1
