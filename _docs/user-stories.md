# ACC LMS — Historias de Usuario Completas

**Versión:** 2025-12-14  
**Estado:** Backlog completo para implementación  
**Derivado de:** [functional-requirements.md](functional-requirements.md)

---

## Convenciones y Mejores Prácticas

### Actores del Sistema

| Actor          | Tipo    | Descripción                                   |
| -------------- | ------- | --------------------------------------------- |
| **Anonymous**  | Usuario | Visitante sin autenticación                   |
| **Student**    | Usuario | Estudiante autenticado                        |
| **Instructor** | Usuario | Creador de contenido educativo                |
| **Admin**      | Usuario | Administrador de plataforma                   |
| **Frontend**   | Sistema | Aplicación React que consume APIs             |
| **Backend**    | Sistema | Servicios Rust que procesan lógica de negocio |
| **System**     | Sistema | Procesos automáticos, jobs, webhooks          |

### Formato de Historia de Usuario

```
ID: US-[EPIC]-[NNN]
Prioridad: 🔥 Critical | ⚡ High | 🎯 Medium | 💡 Low

Como [ACTOR]
Quiero [ACCIÓN/FUNCIONALIDAD]
Para [BENEFICIO/VALOR DE NEGOCIO]

Criterios de Aceptación:
├─ Escenario 1: [Nombre descriptivo]
│  ├─ Dado [contexto/precondición]
│  ├─ Cuando [acción del usuario/sistema]
│  └─ Entonces [resultado esperado]
│
└─ Escenario N: ...

Notas Técnicas:
├─ RF Relacionado: RF-XXX-NNN
├─ Endpoint: METHOD /api/v1/...
├─ Servicio: xxx-service
└─ Estimación: N story points

Tareas de Implementación:
├─ [ ] Backend: ...
├─ [ ] Frontend: ...
└─ [ ] Tests: ...
```

### Priorización (MoSCoW + Valor)

| Prioridad       | Significado                 | Criterio                               |
| --------------- | --------------------------- | -------------------------------------- |
| 🔥 **Critical** | Bloquea MVP                 | Sin esto no hay producto viable        |
| ⚡ **High**     | Alto impacto en UX/revenue  | Diferenciador competitivo              |
| 🎯 **Medium**   | Importante para completitud | Mejora experiencia significativamente  |
| 💡 **Low**      | Nice to have                | Backlog futuro, no bloquea lanzamiento |

### Estimación (Story Points - Fibonacci)

| Points | Complejidad     | Tiempo aprox. (1 dev) |
| ------ | --------------- | --------------------- |
| 1      | Trivial         | < 2 horas             |
| 2      | Simple          | 2-4 horas             |
| 3      | Pequeña         | 0.5-1 día             |
| 5      | Media           | 1-2 días              |
| 8      | Grande          | 2-3 días              |
| 13     | Muy grande      | 3-5 días              |
| 21     | Épica (dividir) | 1+ semana             |

---

## Epic 1: Autenticación y Seguridad

### US-AUTH-001: Registro de Usuario Nuevo 🔥

**Como** visitante anónimo  
**Quiero** registrarme en la plataforma con email y contraseña  
**Para** acceder al catálogo de cursos y crear mi perfil de aprendizaje

**Criterios de Aceptación:**

├─ **Escenario 1: Registro exitoso**
│ ├─ Dado que soy un visitante sin cuenta
│ ├─ Cuando completo el formulario con email válido, contraseña segura (10+ chars, mayúscula, minúscula, número, símbolo), nombre y apellido
│ └─ Entonces recibo confirmación visual, se crea mi cuenta con rol Student, y accedo automáticamente al dashboard

├─ **Escenario 2: Email duplicado**
│ ├─ Dado que intento registrarme con un email ya existente
│ ├─ Cuando envío el formulario
│ └─ Entonces veo mensaje genérico "No se pudo completar el registro" (sin revelar que el email existe por seguridad)

├─ **Escenario 3: Contraseña débil**
│ ├─ Dado que ingreso una contraseña que no cumple requisitos
│ ├─ Cuando el campo pierde foco o intento enviar
│ └─ Entonces veo indicador visual de fortaleza con sugerencias específicas en tiempo real

├─ **Escenario 4: Validación de campos**
│ ├─ Dado que dejo campos requeridos vacíos o con formato inválido
│ ├─ Cuando intento enviar el formulario
│ └─ Entonces veo mensajes de error inline sin perder los datos ya ingresados

└─ **Escenario 5: Rate limiting**
├─ Dado que se detectan múltiples intentos de registro desde la misma IP
├─ Cuando se superan 3 registros por hora
└─ Entonces se bloquea temporalmente con mensaje explicativo y captcha

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-005
├─ Endpoint: POST /api/v1/auth/register
├─ Servicio: auth-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint registro con validación, hash Argon2id, rate limiting
├─ [ ] Backend: Evento domain user.registered
├─ [ ] Frontend: Formulario con validación client-side, indicador de fortaleza
├─ [ ] Frontend: Estados de loading, error, success
└─ [ ] Tests: Unit (validaciones), Integration (flujo completo), E2E (happy path)

---

### US-AUTH-002: Login de Usuario 🔥

**Como** usuario registrado  
**Quiero** iniciar sesión con mis credenciales  
**Para** acceder a mis cursos matriculados y progreso personal

**Criterios de Aceptación:**

├─ **Escenario 1: Login exitoso**
│ ├─ Dado que tengo credenciales válidas
│ ├─ Cuando ingreso email y contraseña correctos
│ └─ Entonces accedo a mi dashboard en <2s, recibo JWT access token (15min) y refresh token (7d en httpOnly cookie)

├─ **Escenario 2: Credenciales inválidas**
│ ├─ Dado que ingreso credenciales incorrectas
│ ├─ Cuando envío el formulario
│ └─ Entonces veo mensaje genérico "Credenciales inválidas" (sin especificar qué campo falló)

├─ **Escenario 3: Bloqueo por intentos fallidos**
│ ├─ Dado que he fallado 5 veces consecutivas
│ ├─ Cuando intento login nuevamente
│ └─ Entonces mi cuenta se bloquea 15 minutos con contador visible de tiempo restante

├─ **Escenario 4: Recordar sesión**
│ ├─ Dado que marco "Recordarme" al hacer login
│ ├─ Cuando cierro el navegador y vuelvo dentro de 30 días
│ └─ Entonces permanezco autenticado sin necesidad de re-login

└─ **Escenario 5: Redirección post-login**
├─ Dado que intenté acceder a /course/xyz sin autenticación
├─ Cuando completo el login exitosamente
└─ Entonces soy redirigido a /course/xyz (URL original) en lugar del dashboard

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-006
├─ Endpoint: POST /api/v1/auth/login
├─ Servicio: auth-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint login, verificación Argon2id, generación JWT RS256
├─ [ ] Backend: Tracking de intentos fallidos con Redis, exponential backoff
├─ [ ] Backend: Registro de sesión (IP, User-Agent, timestamp)
├─ [ ] Frontend: Formulario login, manejo de estados, redirect logic
├─ [ ] Frontend: Persistencia de token en memoria (no localStorage por XSS)
└─ [ ] Tests: Unit, Integration, E2E, Security (brute force)

---

### US-AUTH-003: Renovación Automática de Token 🔥

**Como** Frontend (aplicación React)  
**Quiero** renovar tokens de acceso automáticamente antes de que expiren  
**Para** mantener la sesión del usuario sin interrupciones ni re-login

**Criterios de Aceptación:**

├─ **Escenario 1: Renovación proactiva**
│ ├─ Dado que el access token expira en menos de 2 minutos
│ ├─ Cuando se detecta en cualquier request API
│ └─ Entonces se renueva automáticamente en background sin afectar la operación actual

├─ **Escenario 2: Token rotation**
│ ├─ Dado que se solicita un nuevo access token
│ ├─ Cuando el backend procesa el refresh
│ └─ Entonces se invalida el refresh token anterior y se emite uno nuevo (rotation)

├─ **Escenario 3: Refresh token expirado**
│ ├─ Dado que el refresh token ha expirado (>7 días sin actividad)
│ ├─ Cuando intento renovar
│ └─ Entonces soy redirigido al login con mensaje "Sesión expirada, por favor inicia sesión nuevamente"

├─ **Escenario 4: Detección de replay attack**
│ ├─ Dado que un refresh token ya fue usado
│ ├─ Cuando se intenta usar nuevamente (posible robo)
│ └─ Entonces se invalidan TODAS las sesiones del usuario y se requiere login completo

└─ **Escenario 5: Múltiples pestañas**
├─ Dado que tengo la app abierta en 3 pestañas
├─ Cuando una pestaña renueva el token
└─ Entonces las otras pestañas detectan el nuevo token via BroadcastChannel API

**Notas Técnicas:**
├─ RF Relacionado: RF-GLOBAL-001
├─ Endpoint: POST /api/v1/auth/refresh
├─ Servicio: auth-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint refresh con token rotation
├─ [ ] Backend: Blacklist de tokens usados en Redis (TTL = token lifetime)
├─ [ ] Backend: Detección de replay attacks con invalidación de sesiones
├─ [ ] Frontend: Interceptor Axios para refresh proactivo
├─ [ ] Frontend: BroadcastChannel para sincronizar tokens entre pestañas
├─ [ ] Frontend: Queue de requests pendientes durante refresh
└─ [ ] Tests: Concurrencia, race conditions, security

---

### US-AUTH-004: Logout y Cierre de Sesión 🔥

**Como** usuario autenticado  
**Quiero** cerrar mi sesión de forma segura  
**Para** proteger mi cuenta en dispositivos compartidos

**Criterios de Aceptación:**

├─ **Escenario 1: Logout de sesión actual**
│ ├─ Dado que estoy autenticado
│ ├─ Cuando hago clic en "Cerrar sesión"
│ └─ Entonces se invalida mi token, se limpia la cookie, y soy redirigido al home

├─ **Escenario 2: Logout de todas las sesiones**
│ ├─ Dado que estoy en configuración de seguridad
│ ├─ Cuando selecciono "Cerrar todas las sesiones"
│ └─ Entonces se invalidan todos mis refresh tokens en todos los dispositivos

├─ **Escenario 3: Logout forzado por admin**
│ ├─ Dado que un admin invalida mi sesión
│ ├─ Cuando intento cualquier operación autenticada
│ └─ Entonces recibo error 401 y soy redirigido al login con mensaje explicativo

└─ **Escenario 4: Limpieza de estado local**
├─ Dado que hago logout
├─ Cuando se completa el proceso
└─ Entonces se limpia todo el estado en memoria, localStorage (si existe), y cache de React Query

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-006
├─ Endpoints: POST /api/v1/auth/logout, POST /api/v1/auth/logout-all
├─ Servicio: auth-service
└─ Estimación: 3 SP

**Tareas de Implementación:**
├─ [ ] Backend: Blacklist de access token hasta expiración
├─ [ ] Backend: Invalidación de refresh tokens en Redis
├─ [ ] Backend: Audit log de logout (voluntario vs forzado)
├─ [ ] Frontend: Limpieza completa de estado y redirección
└─ [ ] Tests: Verificar invalidación efectiva de tokens

---

### US-AUTH-005: Recuperación de Contraseña ⚡

**Como** usuario que olvidó su contraseña  
**Quiero** restablecerla mediante un enlace enviado a mi email  
**Para** recuperar acceso a mi cuenta sin contactar soporte

**Criterios de Aceptación:**

├─ **Escenario 1: Solicitud de reset**
│ ├─ Dado que estoy en la página de login
│ ├─ Cuando hago clic en "Olvidé mi contraseña" e ingreso mi email
│ └─ Entonces veo mensaje "Si el email existe, recibirás instrucciones" (sin confirmar existencia)

├─ **Escenario 2: Email de reset**
│ ├─ Dado que solicité reset para un email válido
│ ├─ Cuando el sistema procesa la solicitud
│ └─ Entonces recibo email con enlace único válido por 1 hora

├─ **Escenario 3: Cambio de contraseña**
│ ├─ Dado que accedo al enlace de reset válido
│ ├─ Cuando ingreso nueva contraseña que cumple requisitos
│ └─ Entonces mi contraseña se actualiza, se invalidan todas las sesiones, y puedo hacer login

├─ **Escenario 4: Enlace expirado o usado**
│ ├─ Dado que intento usar un enlace de reset expirado o ya utilizado
│ ├─ Cuando accedo al enlace
│ └─ Entonces veo mensaje de error con opción de solicitar nuevo enlace

└─ **Escenario 5: Rate limiting de solicitudes**
├─ Dado que se solicitan múltiples resets para el mismo email
├─ Cuando se superan 3 solicitudes por hora
└─ Entonces se bloquean nuevas solicitudes temporalmente

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-007
├─ Endpoints: POST /api/v1/auth/forgot-password, POST /api/v1/auth/reset-password
├─ Servicio: auth-service, notifications-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Generación de token seguro (crypto random, 32 bytes)
├─ [ ] Backend: Almacenamiento con TTL en Redis
├─ [ ] Backend: Endpoint de validación y cambio
├─ [ ] Backend: Invalidación de tokens anteriores al generar nuevo
├─ [ ] Frontend: Formularios de solicitud y cambio
├─ [ ] Notifications: Template de email con enlace
└─ [ ] Tests: Flujo completo, seguridad, expiración

---

## Epic 2: Gestión de Perfil de Usuario

### US-PROFILE-001: Ver y Editar Perfil Personal ⚡

**Como** usuario autenticado  
**Quiero** ver y editar mi información personal  
**Para** mantener mi perfil actualizado y personalizar mi experiencia

**Criterios de Aceptación:**

├─ **Escenario 1: Visualización de perfil**
│ ├─ Dado que estoy autenticado
│ ├─ Cuando accedo a mi perfil
│ └─ Entonces veo mi información actual: nombre, apellido, email, avatar, bio, fecha de registro

├─ **Escenario 2: Edición de campos básicos**
│ ├─ Dado que estoy en modo edición
│ ├─ Cuando modifico nombre, apellido o bio
│ └─ Entonces los cambios se guardan con feedback visual y persisten tras refrescar

├─ **Escenario 3: Subida de avatar**
│ ├─ Dado que subo una imagen JPG/PNG/WebP menor a 2MB
│ ├─ Cuando se procesa el archivo
│ └─ Entonces se redimensiona automáticamente, se convierte a WebP optimizado, y se muestra inmediatamente

├─ **Escenario 4: Validación de bio**
│ ├─ Dado que ingreso una bio mayor a 500 caracteres
│ ├─ Cuando intento guardar
│ └─ Entonces veo contador de caracteres y error de validación sin perder el texto

└─ **Escenario 5: Sanitización de HTML**
├─ Dado que intento inyectar HTML/scripts en campos de texto
├─ Cuando se guarda el contenido
└─ Entonces el backend sanitiza el input y almacena solo texto plano seguro

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-001
├─ Endpoints: GET /api/v1/users/:id, PATCH /api/v1/users/:id
├─ Servicio: users-service, content-service (avatar)
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint GET/PATCH con validación y sanitización
├─ [ ] Backend: Procesamiento de imagen (resize, WebP, MinIO)
├─ [ ] Frontend: Formulario con validación, preview de avatar
├─ [ ] Frontend: Optimistic updates con rollback en error
└─ [ ] Tests: Validaciones, upload de imagen, XSS prevention

---

### US-PROFILE-002: Configuración de Preferencias 🎯

**Como** usuario autenticado  
**Quiero** configurar mis preferencias de idioma, notificaciones y privacidad  
**Para** personalizar mi experiencia en la plataforma

**Criterios de Aceptación:**

├─ **Escenario 1: Cambio de idioma**
│ ├─ Dado que estoy en configuración
│ ├─ Cuando cambio el idioma de ES a EN
│ └─ Entonces la interfaz se actualiza inmediatamente sin recargar la página

├─ **Escenario 2: Preferencias de notificación**
│ ├─ Dado que configuro notificaciones por email
│ ├─ Cuando desactivo "Promociones y ofertas"
│ └─ Entonces dejo de recibir emails de marketing pero sigo recibiendo transaccionales

├─ **Escenario 3: Configuración de privacidad**
│ ├─ Dado que configuro mi perfil como privado
│ ├─ Cuando otros usuarios buscan mi perfil
│ └─ Entonces solo ven información básica (nombre y avatar)

└─ **Escenario 4: Zona horaria**
├─ Dado que configuro mi zona horaria
├─ Cuando veo fechas en la plataforma
└─ Entonces todas las fechas se muestran en mi zona horaria local

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-002
├─ Endpoint: PATCH /api/v1/users/:id/preferences
├─ Servicio: users-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de preferencias con estructura anidada
├─ [ ] Backend: Validación de valores permitidos (idiomas, timezones)
├─ [ ] Frontend: UI de configuración con toggles y selects
├─ [ ] Frontend: Integración con i18n para cambio de idioma
└─ [ ] Tests: Persistencia de preferencias, aplicación correcta

---

### US-PROFILE-003: Gestión de Sesiones Activas ⚡

**Como** usuario preocupado por la seguridad  
**Quiero** ver y gestionar mis sesiones activas en diferentes dispositivos  
**Para** detectar accesos no autorizados y proteger mi cuenta

**Criterios de Aceptación:**

├─ **Escenario 1: Listado de sesiones**
│ ├─ Dado que accedo a seguridad de cuenta
│ ├─ Cuando veo mis sesiones activas
│ └─ Entonces veo lista con: dispositivo, navegador, ubicación aproximada, última actividad

├─ **Escenario 2: Identificación de sesión actual**
│ ├─ Dado que veo el listado de sesiones
│ ├─ Cuando identifico cada sesión
│ └─ Entonces la sesión actual está marcada claramente y no puede cerrarse individualmente

├─ **Escenario 3: Cierre de sesión individual**
│ ├─ Dado que identifico una sesión sospechosa
│ ├─ Cuando hago clic en "Cerrar sesión"
│ └─ Entonces esa sesión se invalida inmediatamente y desaparece de la lista

└─ **Escenario 4: Cierre de todas las sesiones**
├─ Dado que sospecho acceso no autorizado
├─ Cuando hago clic en "Cerrar todas las demás sesiones"
└─ Entonces todas las sesiones excepto la actual se invalidan

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-003
├─ Endpoints: GET /api/v1/auth/sessions, DELETE /api/v1/auth/sessions/:id
├─ Servicio: auth-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Registro de sesiones con metadata (IP, UA, geo)
├─ [ ] Backend: Endpoints de listado y revocación
├─ [ ] Frontend: UI de gestión de sesiones
├─ [ ] Frontend: Confirmación antes de cerrar sesiones
└─ [ ] Tests: Invalidación efectiva, actualización de lista

---

_Continúa en siguiente parte..._

---

## Epic 3: Catálogo y Exploración de Cursos

### US-CAT-001: Explorar Catálogo Público 🔥

**Como** visitante o estudiante  
**Quiero** navegar el catálogo de cursos disponibles  
**Para** descubrir contenido relevante a mis intereses de aprendizaje

**Criterios de Aceptación:**

├─ **Escenario 1: Listado inicial**
│ ├─ Dado que accedo al catálogo
│ ├─ Cuando se carga la página
│ └─ Entonces veo hasta 20 cursos con: imagen, título, instructor, precio, rating, duración estimada

├─ **Escenario 2: Paginación**
│ ├─ Dado que hay más de 20 cursos
│ ├─ Cuando hago scroll o clic en "Cargar más"
│ └─ Entonces se cargan los siguientes 20 cursos sin perder los anteriores (infinite scroll)

├─ **Escenario 3: Estado vacío**
│ ├─ Dado que no hay cursos que coincidan con los filtros
│ ├─ Cuando veo el catálogo vacío
│ └─ Entonces veo mensaje amigable con sugerencia de ampliar búsqueda o limpiar filtros

└─ **Escenario 4: Responsive design**
├─ Dado que accedo desde móvil
├─ Cuando navego el catálogo
└─ Entonces veo grid adaptado (1-2 columnas), filtros en drawer, y touch-friendly

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-001
├─ Endpoint: GET /api/v1/courses?page=1&pageSize=20&isPublished=true
├─ Servicio: courses-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con paginación cursor-based para mejor performance
├─ [ ] Backend: Proyección de campos (no enviar contenido completo)
├─ [ ] Frontend: Grid responsivo con lazy loading de imágenes
├─ [ ] Frontend: Skeleton loading durante carga
├─ [ ] Frontend: React Query para cache y prefetch
└─ [ ] Tests: Paginación, responsive, performance

---

### US-CAT-002: Búsqueda y Filtrado de Cursos ⚡

**Como** usuario buscando curso específico  
**Quiero** buscar y filtrar cursos por múltiples criterios  
**Para** encontrar exactamente lo que necesito rápidamente

**Criterios de Aceptación:**

├─ **Escenario 1: Búsqueda por texto**
│ ├─ Dado que escribo "React hooks avanzado" en el buscador
│ ├─ Cuando presiono Enter o espero 300ms
│ └─ Entonces veo resultados ordenados por relevancia con términos destacados (highlighting)

├─ **Escenario 2: Filtros múltiples**
│ ├─ Dado que aplico filtros: categoría "Frontend", nivel "Avanzado", precio "< $50"
│ ├─ Cuando se actualizan los resultados
│ └─ Entonces veo solo cursos que cumplen TODOS los criterios (AND lógico)

├─ **Escenario 3: Ordenamiento**
│ ├─ Dado que tengo resultados de búsqueda
│ ├─ Cuando cambio ordenamiento a "Mejor valorados"
│ └─ Entonces los cursos se reordenan sin perder los filtros aplicados

├─ **Escenario 4: URL con estado de búsqueda**
│ ├─ Dado que tengo búsqueda y filtros activos
│ ├─ Cuando copio la URL y la comparto
│ └─ Entonces quien la abra verá exactamente los mismos resultados

└─ **Escenario 5: Limpieza de filtros**
├─ Dado que tengo múltiples filtros aplicados
├─ Cuando hago clic en "Limpiar filtros"
└─ Entonces todos los filtros se resetean y veo el catálogo completo

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-002
├─ Endpoint: GET /api/v1/courses?search=...&category=...&level=...&priceMax=...
├─ Servicio: courses-service, search-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Full-text search con índices
├─ [ ] Backend: Filtros combinables con query builder
├─ [ ] Frontend: Componentes de filtro con debounce en búsqueda
├─ [ ] Frontend: Sincronización URL ↔ estado (useSearchParams)
├─ [ ] Frontend: Tags de filtros activos con opción de remover individualmente
└─ [ ] Tests: Combinaciones de filtros, edge cases

---

### US-CAT-003: Ver Detalle de Curso ⚡

**Como** estudiante potencial  
**Quiero** ver toda la información de un curso antes de comprarlo  
**Para** tomar una decisión informada de compra

**Criterios de Aceptación:**

├─ **Escenario 1: Información completa**
│ ├─ Dado que accedo al detalle de un curso
│ ├─ Cuando se carga la página
│ └─ Entonces veo: descripción, temario completo, instructor, precio, duración, requisitos, objetivos, rating

├─ **Escenario 2: Temario visible sin acceso**
│ ├─ Dado que no estoy matriculado
│ ├─ Cuando veo el temario
│ └─ Entonces veo títulos de todas las lecciones pero no puedo acceder al contenido (excepto previews)

├─ **Escenario 3: Lecciones de preview**
│ ├─ Dado que hay lecciones marcadas como preview
│ ├─ Cuando hago clic en una lección de preview
│ └─ Entonces puedo ver el contenido completo sin necesidad de comprar

├─ **Escenario 4: Información del instructor**
│ ├─ Dado que veo el detalle del curso
│ ├─ Cuando hago clic en el nombre del instructor
│ └─ Entonces veo su perfil público con bio, otros cursos, y estadísticas

├─ **Escenario 5: Reviews y valoraciones**
│ ├─ Dado que el curso tiene reviews
│ ├─ Cuando veo la sección de valoraciones
│ └─ Entonces veo promedio, distribución de estrellas, y reviews destacadas con paginación

└─ **Escenario 6: CTA de compra**
├─ Dado que quiero comprar el curso
├─ Cuando hago clic en "Comprar" o "Inscribirme"
└─ Entonces soy llevado al checkout (o login si no estoy autenticado, con redirect post-login)

**Notas Técnicas:**
├─ RF Relacionado: RF-ANON-003
├─ Endpoint: GET /api/v1/courses/:idOrSlug
├─ Servicio: courses-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con joins para instructor y stats
├─ [ ] Backend: Soporte para ID y slug en URL
├─ [ ] Frontend: Layout de página de curso con secciones
├─ [ ] Frontend: Componente de temario expandible
├─ [ ] Frontend: Componente de reviews con paginación
├─ [ ] Frontend: SEO meta tags dinámicos
└─ [ ] Tests: Renderizado completo, preview de lecciones

---

## Epic 4: Gestión de Cursos (Instructor)

### US-COURSE-001: Crear Nuevo Curso ⚡

**Como** instructor verificado  
**Quiero** crear un nuevo curso desde cero  
**Para** compartir mi conocimiento y generar ingresos

**Criterios de Aceptación:**

├─ **Escenario 1: Inicio de creación**
│ ├─ Dado que tengo rol instructor
│ ├─ Cuando hago clic en "Crear curso"
│ └─ Entonces veo wizard paso a paso: Info básica → Contenido → Precio → Revisión

├─ **Escenario 2: Información básica**
│ ├─ Dado que estoy en el paso de info básica
│ ├─ Cuando completo título, descripción, categoría
│ └─ Entonces se auto-genera un slug SEO-friendly editable

├─ **Escenario 3: Guardado automático**
│ ├─ Dado que estoy editando el curso
│ ├─ Cuando hago cambios en cualquier campo
│ └─ Entonces se guarda automáticamente cada 30 segundos con indicador visual

├─ **Escenario 4: Curso en borrador**
│ ├─ Dado que guardo el curso sin publicar
│ ├─ Cuando vuelvo al panel de instructor
│ └─ Entonces veo el curso en "Borradores" y puedo continuar editándolo

└─ **Escenario 5: Validación de campos requeridos**
├─ Dado que intento avanzar en el wizard
├─ Cuando hay campos requeridos sin completar
└─ Entonces veo indicadores de error y no puedo avanzar hasta completarlos

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-003
├─ Endpoint: POST /api/v1/courses
├─ Servicio: courses-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de creación con validaciones
├─ [ ] Backend: Generación de slug único con colisión handling
├─ [ ] Backend: Verificación de rol instructor
├─ [ ] Frontend: Wizard multi-step con state management
├─ [ ] Frontend: Auto-save con debounce y merge de cambios
├─ [ ] Frontend: Preview en tiempo real
└─ [ ] Tests: Flujo completo de creación, validaciones

---

### US-COURSE-002: Gestionar Lecciones ⚡

**Como** instructor  
**Quiero** agregar, editar y organizar lecciones en mi curso  
**Para** estructurar el contenido educativo de manera lógica

**Criterios de Aceptación:**

├─ **Escenario 1: Agregar lección**
│ ├─ Dado que estoy editando un curso
│ ├─ Cuando hago clic en "Agregar lección"
│ └─ Entonces puedo seleccionar tipo (video, artículo, quiz) y completar contenido

├─ **Escenario 2: Tipos de contenido**
│ ├─ Dado que creo una lección
│ ├─ Cuando selecciono el tipo
│ └─ Entonces veo formulario específico: upload para video, editor rich-text para artículo, builder para quiz

├─ **Escenario 3: Reordenamiento**
│ ├─ Dado que tengo múltiples lecciones
│ ├─ Cuando arrastro una lección a nueva posición (drag & drop)
│ └─ Entonces los índices se recalculan automáticamente y se persiste el orden

├─ **Escenario 4: Lección de preview**
│ ├─ Dado que marco una lección como "Vista previa gratuita"
│ ├─ Cuando publico el curso
│ └─ Entonces esa lección es accesible para usuarios no matriculados

└─ **Escenario 5: Eliminación con confirmación**
├─ Dado que quiero eliminar una lección
├─ Cuando hago clic en eliminar
└─ Entonces veo modal de confirmación mencionando que la acción es irreversible

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-004
├─ Endpoints: POST/PATCH/DELETE /api/v1/courses/:id/lessons/:lessonId
├─ Servicio: courses-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: CRUD de lecciones con ordenamiento
├─ [ ] Backend: Recálculo de índices en reordenamiento
├─ [ ] Backend: Validación de ownership (solo owner puede editar)
├─ [ ] Frontend: Lista de lecciones con drag & drop (dnd-kit)
├─ [ ] Frontend: Formularios dinámicos por tipo de lección
├─ [ ] Frontend: Editor rich-text para artículos (Tiptap/Lexical)
└─ [ ] Tests: CRUD, reordenamiento, permisos

---

### US-COURSE-003: Subir y Procesar Videos 🔥

**Como** instructor  
**Quiero** subir videos de alta calidad para mis lecciones  
**Para** ofrecer contenido multimedia profesional a mis estudiantes

**Criterios de Aceptación:**

├─ **Escenario 1: Subida de video**
│ ├─ Dado que creo una lección tipo video
│ ├─ Cuando selecciono un archivo MP4/WebM de hasta 2GB
│ └─ Entonces veo progreso de upload en tiempo real con opción de cancelar

├─ **Escenario 2: Procesamiento en background**
│ ├─ Dado que el video se sube correctamente
│ ├─ Cuando inicia el procesamiento
│ └─ Entonces veo estado "Procesando..." y puedo continuar editando otras lecciones

├─ **Escenario 3: Múltiples calidades**
│ ├─ Dado que el procesamiento termina
│ ├─ Cuando el video está listo
│ └─ Entonces está disponible en 360p, 720p, 1080p con streaming adaptativo

├─ **Escenario 4: Generación de thumbnail**
│ ├─ Dado que el video se procesa
│ ├─ Cuando está listo
│ └─ Entonces se genera thumbnail automático que puedo cambiar si deseo

├─ **Escenario 5: Error en upload/procesamiento**
│ ├─ Dado que ocurre un error durante el proceso
│ ├─ Cuando el sistema detecta el fallo
│ └─ Entonces veo mensaje de error específico con opción de reintentar

└─ **Escenario 6: Límites de almacenamiento**
├─ Dado que tengo un límite de almacenamiento
├─ Cuando intento subir más allá del límite
└─ Entonces veo mensaje indicando espacio disponible y opciones de upgrade

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-005
├─ Endpoints: POST /api/v1/content/presign-upload, POST /api/v1/content/complete-upload
├─ Servicio: content-service, jobs (procesamiento)
└─ Estimación: 21 SP

**Tareas de Implementación:**
├─ [ ] Backend: Presigned URL para upload directo a MinIO
├─ [ ] Backend: Webhook de completado para iniciar procesamiento
├─ [ ] Backend: Job de transcodificación (FFmpeg) con múltiples calidades
├─ [ ] Backend: Generación de HLS manifest para streaming adaptativo
├─ [ ] Frontend: Componente de upload con progreso (resumable uploads)
├─ [ ] Frontend: Estado de procesamiento con polling/websocket
├─ [ ] Frontend: Selector de thumbnail
└─ [ ] Tests: Upload grande, procesamiento, error handling

---

### US-COURSE-004: Crear y Configurar Quizzes ⚡

**Como** instructor  
**Quiero** crear evaluaciones con diferentes tipos de preguntas  
**Para** medir el aprendizaje de mis estudiantes

**Criterios de Aceptación:**

├─ **Escenario 1: Builder de quiz**
│ ├─ Dado que creo una lección tipo quiz
│ ├─ Cuando accedo al builder
│ └─ Entonces veo interfaz para agregar preguntas con drag & drop

├─ **Escenario 2: Tipos de pregunta**
│ ├─ Dado que agrego una pregunta
│ ├─ Cuando selecciono el tipo
│ └─ Entonces puedo crear: opción única, opción múltiple, V/F, respuesta corta, código

├─ **Escenario 3: Configuración de respuestas**
│ ├─ Dado que creo pregunta de opción múltiple
│ ├─ Cuando configuro las opciones
│ └─ Entonces marco las respuestas correctas y asigno puntuación

├─ **Escenario 4: Retroalimentación**
│ ├─ Dado que configuro una pregunta
│ ├─ Cuando activo feedback
│ └─ Entonces puedo escribir explicación para respuesta correcta e incorrecta

├─ **Escenario 5: Configuración de intentos**
│ ├─ Dado que configuro el quiz
│ ├─ Cuando establezco límites
│ └─ Entonces puedo definir: intentos máximos, tiempo límite, puntuación mínima para aprobar

└─ **Escenario 6: Preview de quiz**
├─ Dado que termino de crear el quiz
├─ Cuando hago clic en "Vista previa"
└─ Entonces puedo ver y responder el quiz como lo vería un estudiante

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-006
├─ Endpoints: CRUD /api/v1/courses/:id/quizzes
├─ Servicio: assignments-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Schema flexible para tipos de pregunta (JSONB)
├─ [ ] Backend: Validación de estructura de quiz
├─ [ ] Frontend: Quiz builder con componentes dinámicos
├─ [ ] Frontend: Drag & drop para reordenar preguntas
├─ [ ] Frontend: Preview mode
└─ [ ] Tests: Creación de cada tipo, validaciones

---

### US-COURSE-005: Publicar y Despublicar Curso ⚡

**Como** instructor  
**Quiero** publicar mi curso cuando esté listo  
**Para** hacerlo visible en el catálogo y comenzar a vender

**Criterios de Aceptación:**

├─ **Escenario 1: Validación pre-publicación**
│ ├─ Dado que intento publicar un curso
│ ├─ Cuando no cumple requisitos mínimos (título, descripción, 1+ lección)
│ └─ Entonces veo checklist de requisitos faltantes sin publicar

├─ **Escenario 2: Publicación exitosa**
│ ├─ Dado que el curso cumple todos los requisitos
│ ├─ Cuando hago clic en "Publicar"
│ └─ Entonces el curso aparece en el catálogo y recibo confirmación

├─ **Escenario 3: Despublicación**
│ ├─ Dado que tengo un curso publicado
│ ├─ Cuando lo despublico
│ └─ Entonces desaparece del catálogo pero estudiantes existentes mantienen acceso

├─ **Escenario 4: Edición post-publicación**
│ ├─ Dado que tengo un curso publicado
│ ├─ Cuando edito contenido
│ └─ Entonces los cambios se reflejan inmediatamente para estudiantes

└─ **Escenario 5: Restricción de slug**
├─ Dado que el curso tiene ventas
├─ Cuando intento cambiar el slug
└─ Entonces veo advertencia de que el slug no puede cambiar (SEO y enlaces existentes)

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-008
├─ Endpoints: POST /api/v1/courses/:id/publish, POST /api/v1/courses/:id/unpublish
├─ Servicio: courses-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Validaciones de publicación
├─ [ ] Backend: Evento course.published para indexación
├─ [ ] Backend: Lógica de protección de slug
├─ [ ] Frontend: Checklist visual de requisitos
├─ [ ] Frontend: Toggle de publicación con confirmación
└─ [ ] Tests: Validaciones, eventos, permisos

---

## Epic 5: Comercio y Pagos

### US-PAY-001: Proceso de Checkout 🔥

**Como** estudiante  
**Quiero** comprar un curso de forma segura y rápida  
**Para** acceder inmediatamente al contenido

**Criterios de Aceptación:**

├─ **Escenario 1: Inicio de checkout**
│ ├─ Dado que hago clic en "Comprar" en un curso
│ ├─ Cuando accedo al checkout
│ └─ Entonces veo resumen: nombre del curso, instructor, precio, y métodos de pago

├─ **Escenario 2: Selección de método de pago**
│ ├─ Dado que estoy en checkout
│ ├─ Cuando selecciono Stripe (tarjeta) o MercadoPago
│ └─ Entonces veo formulario específico del proveedor con validación en tiempo real

├─ **Escenario 3: Pago exitoso con Stripe**
│ ├─ Dado que ingreso datos de tarjeta válidos
│ ├─ Cuando confirmo el pago
│ └─ Entonces se crea la orden, se procesa el pago, y recibo confirmación en <5 segundos

├─ **Escenario 4: Pago con MercadoPago**
│ ├─ Dado que selecciono MercadoPago
│ ├─ Cuando hago clic en pagar
│ └─ Entonces soy redirigido a MercadoPago y tras pagar exitoso, vuelvo a la plataforma

├─ **Escenario 5: Error en pago**
│ ├─ Dado que el pago falla (fondos, tarjeta rechazada)
│ ├─ Cuando ocurre el error
│ └─ Entonces veo mensaje específico del problema con opciones: reintentar, cambiar método

└─ **Escenario 6: Confirmación y recibo**
├─ Dado que el pago fue exitoso
├─ Cuando veo la confirmación
└─ Entonces recibo email con recibo/factura y acceso inmediato al curso

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-006
├─ Endpoint: POST /api/v1/orders
├─ Servicio: payments-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Creación de orden con estado pending
├─ [ ] Backend: Integración Stripe Payment Intents
├─ [ ] Backend: Integración MercadoPago Preferences
├─ [ ] Frontend: Página de checkout responsive
├─ [ ] Frontend: Stripe Elements para formulario seguro
├─ [ ] Frontend: Manejo de redirects MercadoPago
├─ [ ] Frontend: Estados de loading, error, success
└─ [ ] Tests: Flujos de pago, error handling

---

### US-PAY-002: Procesamiento de Webhooks de Pago 🔥

**Como** Backend (payments-service)  
**Quiero** procesar webhooks de Stripe y MercadoPago de forma confiable  
**Para** confirmar pagos y activar matrículas automáticamente

**Criterios de Aceptación:**

├─ **Escenario 1: Webhook de pago exitoso**
│ ├─ Dado que recibo webhook de payment_intent.succeeded (Stripe)
│ ├─ Cuando valido la firma y proceso
│ └─ Entonces actualizo orden a "paid", emito evento order.paid, respondo 200 OK

├─ **Escenario 2: Validación de firma**
│ ├─ Dado que recibo un webhook
│ ├─ Cuando la firma no es válida
│ └─ Entonces rechazo con 401 y registro intento sospechoso

├─ **Escenario 3: Idempotencia**
│ ├─ Dado que recibo el mismo webhook duplicado
│ ├─ Cuando proceso la segunda vez
│ └─ Entonces respondo 200 OK sin crear duplicados ni re-emitir eventos

├─ **Escenario 4: Eventos fuera de orden**
│ ├─ Dado que recibo eventos en orden incorrecto
│ ├─ Cuando un evento más reciente ya fue procesado
│ └─ Entonces ignoro eventos obsoletos basándome en timestamp

├─ **Escenario 5: Error de procesamiento**
│ ├─ Dado que falla el procesamiento interno (BD, etc.)
│ ├─ Cuando ocurre el error
│ └─ Entonces respondo 500 para que el proveedor reintente, con límite de 3 reintentos

└─ **Escenario 6: Webhook de pago fallido**
├─ Dado que recibo webhook de payment_intent.payment_failed
├─ Cuando proceso el evento
└─ Entonces actualizo orden a "failed" y envío notificación al usuario

**Notas Técnicas:**
├─ RF Relacionado: RF-SYS-001
├─ Endpoints: POST /api/v1/payments/webhook/stripe, POST /api/v1/payments/webhook/mercadopago
├─ Servicio: payments-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con validación de firma (Stripe signature, MP signature)
├─ [ ] Backend: Procesamiento idempotente con dedup key
├─ [ ] Backend: Actualización de orden con optimistic locking
├─ [ ] Backend: Emisión de domain events
├─ [ ] Backend: Logging detallado para debugging
└─ [ ] Tests: Idempotencia, firmas, casos edge

---

### US-PAY-003: Creación Automática de Matrícula 🔥

**Como** System (enrollments-service)  
**Quiero** crear matrículas automáticamente cuando se confirma un pago  
**Para** dar acceso inmediato al contenido sin intervención manual

**Criterios de Aceptación:**

├─ **Escenario 1: Creación de matrícula**
│ ├─ Dado que recibo evento order.paid
│ ├─ Cuando proceso el evento
│ └─ Entonces creo enrollment con userId, courseId, status=active, progress=0

├─ **Escenario 2: Idempotencia**
│ ├─ Dado que recibo el mismo evento duplicado
│ ├─ Cuando ya existe enrollment para userId+courseId
│ └─ Entonces no creo duplicado, mantengo enrollment existente

├─ **Escenario 3: Notificación al usuario**
│ ├─ Dado que creo enrollment exitosamente
│ ├─ Cuando se persiste
│ └─ Entonces emito evento enrollment.created para que notifications-service envíe email

├─ **Escenario 4: Acceso inmediato**
│ ├─ Dado que se crea enrollment
│ ├─ Cuando el usuario accede al curso
│ └─ Entonces tiene acceso completo a todo el contenido sin delay

└─ **Escenario 5: Error de creación**
├─ Dado que falla la creación de enrollment
├─ Cuando ocurre el error
└─ Entonces reintento automático y alerta a admin si falla 3 veces

**Notas Técnicas:**
├─ RF Relacionado: RF-SYS-002
├─ Event: order.paid → enrollment.created
├─ Servicio: enrollments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Consumer de evento order.paid
├─ [ ] Backend: Creación de enrollment con upsert
├─ [ ] Backend: Emisión de enrollment.created
├─ [ ] Backend: Dead letter queue para eventos fallidos
└─ [ ] Tests: Idempotencia, flujo completo

---

### US-PAY-004: Historial de Compras y Facturas ⚡

**Como** estudiante  
**Quiero** ver mi historial de compras y descargar facturas  
**Para** llevar control de mis gastos y para efectos fiscales

**Criterios de Aceptación:**

├─ **Escenario 1: Listado de compras**
│ ├─ Dado que accedo a mi historial de compras
│ ├─ Cuando veo la lista
│ └─ Entonces veo todas mis órdenes: curso, fecha, monto, estado, método de pago

├─ **Escenario 2: Detalle de orden**
│ ├─ Dado que hago clic en una orden
│ ├─ Cuando veo el detalle
│ └─ Entonces veo información completa incluyendo ID de transacción

├─ **Escenario 3: Descarga de factura**
│ ├─ Dado que tengo una orden pagada
│ ├─ Cuando hago clic en "Descargar factura"
│ └─ Entonces descargo PDF con datos fiscales, desglose, y número de factura

└─ **Escenario 4: Filtros y búsqueda**
├─ Dado que tengo muchas compras
├─ Cuando filtro por fecha o busco por nombre de curso
└─ Entonces veo resultados filtrados con paginación

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-007
├─ Endpoints: GET /api/v1/orders/my, GET /api/v1/orders/:id/invoice
├─ Servicio: payments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de listado con filtros
├─ [ ] Backend: Generación de PDF de factura
├─ [ ] Frontend: Página de historial con tabla
├─ [ ] Frontend: Descarga de PDF
└─ [ ] Tests: Listado, generación PDF

---

_Continúa en siguiente parte (Epic 6-10)..._

---

## Epic 6: Experiencia de Aprendizaje

### US-LEARN-001: Acceso a Cursos Matriculados 🔥

**Como** estudiante matriculado  
**Quiero** acceder a mis cursos desde un dashboard personal  
**Para** continuar mi aprendizaje de forma organizada

**Criterios de Aceptación:**

├─ **Escenario 1: Dashboard de cursos**
│ ├─ Dado que estoy autenticado
│ ├─ Cuando accedo a mi dashboard
│ └─ Entonces veo mis cursos matriculados con: imagen, título, progreso %, última actividad

├─ **Escenario 2: Ordenamiento**
│ ├─ Dado que tengo múltiples cursos
│ ├─ Cuando ordeno por "Actividad reciente"
│ └─ Entonces los cursos se ordenan por última lección vista

├─ **Escenario 3: Acceso rápido**
│ ├─ Dado que hago clic en un curso
│ ├─ Cuando accedo al player
│ └─ Entonces me lleva directamente a la última lección donde quedé

├─ **Escenario 4: Cursos completados**
│ ├─ Dado que completé un curso al 100%
│ ├─ Cuando veo el dashboard
│ └─ Entonces está marcado como "Completado" con opción de ver certificado

└─ **Escenario 5: Estado vacío**
├─ Dado que no tengo cursos matriculados
├─ Cuando veo el dashboard
└─ Entonces veo mensaje motivacional con enlace al catálogo

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-008
├─ Endpoint: GET /api/v1/enrollments/my
├─ Servicio: enrollments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con join a courses para metadata
├─ [ ] Backend: Cálculo de progreso agregado
├─ [ ] Frontend: Dashboard con grid de cursos
├─ [ ] Frontend: Componente de tarjeta de curso con progreso
└─ [ ] Tests: Listado, ordenamiento, estados

---

### US-LEARN-002: Reproductor de Video ⚡

**Como** estudiante  
**Quiero** ver videos con controles avanzados  
**Para** optimizar mi experiencia de aprendizaje

**Criterios de Aceptación:**

├─ **Escenario 1: Controles básicos**
│ ├─ Dado que reproduzco un video
│ ├─ Cuando uso el player
│ └─ Entonces puedo: play/pause, seek, volumen, pantalla completa

├─ **Escenario 2: Velocidad de reproducción**
│ ├─ Dado que quiero ver más rápido/lento
│ ├─ Cuando cambio la velocidad
│ └─ Entonces puedo elegir 0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x

├─ **Escenario 3: Calidad adaptativa**
│ ├─ Dado que estoy en conexión lenta
│ ├─ Cuando el video detecta buffering
│ └─ Entonces cambia automáticamente a menor calidad, o puedo elegir manualmente

├─ **Escenario 4: Subtítulos**
│ ├─ Dado que el video tiene transcripción
│ ├─ Cuando activo subtítulos
│ └─ Entonces veo texto sincronizado con el audio

├─ **Escenario 5: Continuación automática**
│ ├─ Dado que pausé el video y vuelvo después
│ ├─ Cuando abro la lección nuevamente
│ └─ Entonces el video comienza donde lo dejé con opción de "empezar desde inicio"

└─ **Escenario 6: Marcado automático de completado**
├─ Dado que veo el 90%+ del video
├─ Cuando se alcanza ese punto
└─ Entonces la lección se marca automáticamente como completada

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-009
├─ Frontend: /learn/:courseId/lesson/:lessonId
├─ Player: Video.js o HLS.js para streaming adaptativo
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Frontend: Componente de video player con HLS
├─ [ ] Frontend: UI de controles custom
├─ [ ] Frontend: Persistencia de posición en localStorage + backend
├─ [ ] Frontend: Detector de progreso para auto-complete
├─ [ ] Backend: Endpoint para guardar posición de video
└─ [ ] Tests: Player, controles, persistencia

---

### US-LEARN-003: Navegación de Contenido ⚡

**Como** estudiante  
**Quiero** navegar fácilmente entre lecciones  
**Para** seguir el temario de forma estructurada

**Criterios de Aceptación:**

├─ **Escenario 1: Sidebar de temario**
│ ├─ Dado que estoy viendo una lección
│ ├─ Cuando veo el sidebar lateral
│ └─ Entonces veo lista de todas las lecciones con indicador de completadas/pendientes

├─ **Escenario 2: Navegación secuencial**
│ ├─ Dado que completo una lección
│ ├─ Cuando hago clic en "Siguiente"
│ └─ Entonces avanzo a la siguiente lección automáticamente

├─ **Escenario 3: Navegación libre**
│ ├─ Dado que quiero ir a una lección específica
│ ├─ Cuando hago clic en el sidebar
│ └─ Entonces puedo acceder a cualquier lección (sin bloqueo secuencial)

├─ **Escenario 4: Indicadores visuales**
│ ├─ Dado que veo el temario
│ ├─ Cuando reviso las lecciones
│ └─ Entonces veo: check verde (completada), punto azul (actual), gris (pendiente)

└─ **Escenario 5: Colapso del sidebar**
├─ Dado que quiero más espacio para el contenido
├─ Cuando colapso el sidebar
└─ Entonces el contenido se expande y puedo restaurar el sidebar

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-011
├─ Frontend: /learn/:courseId
├─ Servicio: courses-service, enrollments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Frontend: Layout con sidebar colapsable
├─ [ ] Frontend: Componente de temario con estados
├─ [ ] Frontend: Navegación con keyboard (flechas)
├─ [ ] Frontend: Mobile: sidebar como drawer
└─ [ ] Tests: Navegación, estados, responsive

---

### US-LEARN-004: Seguimiento de Progreso 🔥

**Como** estudiante  
**Quiero** ver mi progreso de aprendizaje  
**Para** mantener motivación y saber cuánto me falta

**Criterios de Aceptación:**

├─ **Escenario 1: Barra de progreso**
│ ├─ Dado que estoy en un curso
│ ├─ Cuando veo el header
│ └─ Entonces veo barra de progreso con porcentaje exacto (ej: "45% completado")

├─ **Escenario 2: Actualización en tiempo real**
│ ├─ Dado que completo una lección
│ ├─ Cuando se marca como completada
│ └─ Entonces el progreso se actualiza inmediatamente sin recargar

├─ **Escenario 3: Tiempo restante estimado**
│ ├─ Dado que el curso tiene duración estimada
│ ├─ Cuando veo mi progreso
│ └─ Entonces veo "Tiempo restante: ~2h 30min" basado en lecciones pendientes

├─ **Escenario 4: Celebración de milestones**
│ ├─ Dado que alcanzo 25%, 50%, 75%, 100%
│ ├─ Cuando se detecta el milestone
│ └─ Entonces veo animación de celebración breve

└─ **Escenario 5: Persistencia de progreso**
├─ Dado que completo lecciones
├─ Cuando vuelvo al curso días después
└─ Entonces mi progreso está exactamente donde lo dejé

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-010
├─ Endpoint: PATCH /api/v1/enrollments/:id/progress
├─ Servicio: enrollments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Cálculo de progreso basado en lecciones completadas
├─ [ ] Backend: Evento lesson.completed para tracking
├─ [ ] Frontend: Componentes de progreso (barra, %)
├─ [ ] Frontend: Animaciones de milestone
└─ [ ] Tests: Cálculo correcto, persistencia

---

## Epic 7: Evaluaciones y Calificaciones

### US-QUIZ-001: Realizar Quiz de Evaluación 🎯

**Como** estudiante  
**Quiero** completar quizzes para evaluar mi conocimiento  
**Para** validar que estoy aprendiendo correctamente

**Criterios de Aceptación:**

├─ **Escenario 1: Inicio de quiz**
│ ├─ Dado que llego a una lección tipo quiz
│ ├─ Cuando inicio el quiz
│ └─ Entonces veo: número de preguntas, tiempo límite (si aplica), intentos disponibles

├─ **Escenario 2: Navegación de preguntas**
│ ├─ Dado que estoy respondiendo
│ ├─ Cuando navego entre preguntas
│ └─ Entonces puedo ir adelante/atrás, ver indicador de respondidas/pendientes

├─ **Escenario 3: Tipos de pregunta**
│ ├─ Dado que encuentro diferentes tipos
│ ├─ Cuando respondo cada tipo
│ └─ Entonces funciona: radio (única), checkbox (múltiple), toggle (V/F), input (corta)

├─ **Escenario 4: Guardado automático**
│ ├─ Dado que respondo preguntas
│ ├─ Cuando cambio de pregunta o se pierde conexión
│ └─ Entonces mis respuestas se guardan y puedo continuar después

├─ **Escenario 5: Envío de quiz**
│ ├─ Dado que completé todas las preguntas
│ ├─ Cuando hago clic en "Enviar"
│ └─ Entonces veo confirmación y mis respuestas se procesan

└─ **Escenario 6: Tiempo agotado**
├─ Dado que el quiz tiene tiempo límite
├─ Cuando se agota el tiempo
└─ Entonces se envía automáticamente con las respuestas actuales

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-012
├─ Endpoints: GET /api/v1/quizzes/:id, POST /api/v1/quizzes/:id/submit
├─ Servicio: assignments-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de quiz con preguntas
├─ [ ] Backend: Endpoint de submit con validación
├─ [ ] Backend: Cálculo de score para preguntas auto-corregibles
├─ [ ] Frontend: Componente de quiz con múltiples tipos
├─ [ ] Frontend: Timer con warnings
├─ [ ] Frontend: Guardado automático
└─ [ ] Tests: Tipos de pregunta, timer, submit

---

### US-QUIZ-002: Ver Resultados de Quiz ⚡

**Como** estudiante  
**Quiero** ver mis resultados detallados  
**Para** entender qué acerté y qué debo repasar

**Criterios de Aceptación:**

├─ **Escenario 1: Puntuación general**
│ ├─ Dado que envié el quiz
│ ├─ Cuando veo los resultados
│ └─ Entonces veo: puntuación obtenida, puntuación máxima, porcentaje, aprobado/reprobado

├─ **Escenario 2: Detalle por pregunta**
│ ├─ Dado que reviso mis respuestas
│ ├─ Cuando veo cada pregunta
│ └─ Entonces veo: mi respuesta, respuesta correcta, si acerté o no

├─ **Escenario 3: Retroalimentación**
│ ├─ Dado que el instructor configuró feedback
│ ├─ Cuando veo una respuesta incorrecta
│ └─ Entonces veo explicación de por qué era incorrecta y cuál era la correcta

├─ **Escenario 4: Intentos adicionales**
│ ├─ Dado que no aprobé y quedan intentos
│ ├─ Cuando veo los resultados
│ └─ Entonces veo botón "Reintentar" con intentos restantes

└─ **Escenario 5: Sin más intentos**
├─ Dado que agoté mis intentos
├─ Cuando veo los resultados
└─ Entonces veo mensaje y sugerencia de repasar material

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-013
├─ Endpoint: GET /api/v1/quizzes/:id/submissions/:submissionId
├─ Servicio: assignments-service, grades-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de resultados con detalle
├─ [ ] Backend: Lógica de intentos restantes
├─ [ ] Frontend: Página de resultados con revisión
├─ [ ] Frontend: Indicadores visuales (verde/rojo)
└─ [ ] Tests: Cálculo de score, display correcto

---

### US-QUIZ-003: Calificaciones del Estudiante ⚡

**Como** estudiante  
**Quiero** ver todas mis calificaciones en un solo lugar  
**Para** tener visión general de mi rendimiento

**Criterios de Aceptación:**

├─ **Escenario 1: Listado de calificaciones**
│ ├─ Dado que accedo a mis calificaciones
│ ├─ Cuando veo la lista
│ └─ Entonces veo todas mis evaluaciones agrupadas por curso

├─ **Escenario 2: Promedio por curso**
│ ├─ Dado que tengo múltiples quizzes en un curso
│ ├─ Cuando veo el resumen
│ └─ Entonces veo promedio ponderado del curso

├─ **Escenario 3: Detalle de evaluación**
│ ├─ Dado que hago clic en una calificación
│ ├─ Cuando veo el detalle
│ └─ Entonces veo la revisión completa del quiz

└─ **Escenario 4: Calificaciones pendientes**
├─ Dado que tengo quizzes de código/ensayo pendientes de revisión
├─ Cuando veo la lista
└─ Entonces aparecen marcados como "Pendiente de revisión"

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-014
├─ Endpoint: GET /api/v1/grades/my
├─ Servicio: grades-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con agregación por curso
├─ [ ] Frontend: Página de calificaciones con agrupación
├─ [ ] Frontend: Links a revisión de cada quiz
└─ [ ] Tests: Cálculo de promedios, estados

---

## Epic 8: Inteligencia Artificial

### US-AI-001: Tutor IA Conversacional ⚡

**Como** estudiante con dudas  
**Quiero** chatear con un tutor IA contextualizado al curso  
**Para** resolver dudas inmediatamente sin esperar al instructor

**Criterios de Aceptación:**

├─ **Escenario 1: Inicio de conversación**
│ ├─ Dado que estoy en una lección
│ ├─ Cuando abro el chat del tutor
│ └─ Entonces veo historial de conversación anterior (si existe) y puedo escribir

├─ **Escenario 2: Respuesta contextualizada**
│ ├─ Dado que pregunto sobre un tema del curso
│ ├─ Cuando el tutor responde
│ └─ Entonces la respuesta está basada en el contenido del curso (RAG) con referencias

├─ **Escenario 3: Referencias a lecciones**
│ ├─ Dado que la respuesta menciona contenido específico
│ ├─ Cuando veo las referencias
│ └─ Entonces puedo hacer clic para ir directamente a esa lección/timestamp

├─ **Escenario 4: Límites de alcance**
│ ├─ Dado que pregunto algo fuera del tema del curso
│ ├─ Cuando el tutor detecta esto
│ └─ Entonces responde cortésmente que solo puede ayudar con contenido del curso

├─ **Escenario 5: Límite de uso**
│ ├─ Dado que tengo cuota de mensajes
│ ├─ Cuando me acerco al límite
│ └─ Entonces veo indicador de mensajes restantes y fecha de reset

└─ **Escenario 6: Historial persistente**
├─ Dado que tuve una conversación
├─ Cuando vuelvo al curso días después
└─ Entonces puedo ver y continuar conversaciones anteriores

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-018
├─ Endpoints: POST /api/v1/ai/tutor/sessions, POST /api/v1/ai/tutor/messages
├─ Servicio: ai-service
└─ Estimación: 21 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoints de chat con streaming
├─ [ ] Backend: RAG pipeline con embeddings del curso
├─ [ ] Backend: Límites de uso por usuario
├─ [ ] Frontend: Componente de chat con streaming
├─ [ ] Frontend: Referencias clickeables
├─ [ ] Frontend: Persistencia de historial
└─ [ ] Tests: Respuestas, límites, streaming

---

### US-AI-002: Búsqueda Semántica 🎯

**Como** usuario  
**Quiero** buscar contenido usando lenguaje natural  
**Para** encontrar información sin recordar palabras exactas

**Criterios de Aceptación:**

├─ **Escenario 1: Búsqueda por significado**
│ ├─ Dado que busco "cómo manejar errores en async/await"
│ ├─ Cuando ejecuto la búsqueda
│ └─ Entonces encuentro lecciones relevantes aunque no contengan esas palabras exactas

├─ **Escenario 2: Resultados rankeados**
│ ├─ Dado que obtengo resultados
│ ├─ Cuando veo la lista
│ └─ Entonces están ordenados por relevancia semántica con score visible

├─ **Escenario 3: Snippets contextuales**
│ ├─ Dado que veo un resultado
│ ├─ Cuando reviso el preview
│ └─ Entonces veo fragmento de texto más relevante a mi búsqueda

├─ **Escenario 4: Filtro por acceso**
│ ├─ Dado que busco en cursos no matriculados
│ ├─ Cuando veo resultados
│ └─ Entonces veo preview limitado con CTA de compra

└─ **Escenario 5: Combinación con filtros**
├─ Dado que combino búsqueda semántica con filtros tradicionales
├─ Cuando aplico ambos
└─ Entonces obtengo resultados que cumplen ambos criterios

**Notas Técnicas:**
├─ RF Relacionado: RF-STU-019
├─ Endpoint: GET /api/v1/ai/semantic-search
├─ Servicio: ai-service, search-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de búsqueda con pgvector
├─ [ ] Backend: Generación de embeddings para contenido
├─ [ ] Backend: Índice de búsqueda híbrido (texto + semántico)
├─ [ ] Frontend: Componente de búsqueda con resultados
├─ [ ] Frontend: Snippets con highlighting
└─ [ ] Tests: Relevancia, performance

---

### US-AI-003: Generación de Quizzes con IA 💡

**Como** instructor  
**Quiero** generar preguntas de quiz automáticamente  
**Para** ahorrar tiempo en la creación de evaluaciones

**Criterios de Aceptación:**

├─ **Escenario 1: Generación a partir de lección**
│ ├─ Dado que estoy editando una lección
│ ├─ Cuando hago clic en "Generar quiz con IA"
│ └─ Entonces el sistema genera 5-10 preguntas basadas en el contenido

├─ **Escenario 2: Configuración de generación**
│ ├─ Dado que inicio la generación
│ ├─ Cuando configuro parámetros
│ └─ Entonces puedo elegir: cantidad, dificultad, tipos de pregunta

├─ **Escenario 3: Edición de preguntas generadas**
│ ├─ Dado que se generan las preguntas
│ ├─ Cuando las reviso
│ └─ Entonces puedo editar, eliminar, o agregar más antes de guardar

├─ **Escenario 4: No publicación automática**
│ ├─ Dado que genero preguntas
│ ├─ Cuando las guardo
│ └─ Entonces quedan en borrador hasta que explícitamente las publique

└─ **Escenario 5: Feedback de calidad**
├─ Dado que uso las preguntas generadas
├─ Cuando los estudiantes las responden
└─ Entonces puedo ver estadísticas de calidad para mejorar futuras generaciones

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-018
├─ Endpoint: POST /api/v1/ai/quizzes/generate
├─ Servicio: ai-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint de generación con LLM
├─ [ ] Backend: Prompt engineering para diferentes tipos
├─ [ ] Backend: Validación de output
├─ [ ] Frontend: Wizard de generación
├─ [ ] Frontend: Editor de preguntas generadas
└─ [ ] Tests: Calidad de generación, validación

---

_Continúa en siguiente parte (Epic 9-12)..._

---

## Epic 9: Gestión de Estudiantes (Instructor)

### US-INS-001: Ver Estudiantes Matriculados ⚡

**Como** instructor  
**Quiero** ver la lista de estudiantes de mis cursos  
**Para** conocer a mi audiencia y dar seguimiento

**Criterios de Aceptación:**

├─ **Escenario 1: Listado de estudiantes**
│ ├─ Dado que accedo a un curso que creé
│ ├─ Cuando veo la sección de estudiantes
│ └─ Entonces veo lista con: nombre, email, fecha de matrícula, progreso %

├─ **Escenario 2: Filtros**
│ ├─ Dado que tengo muchos estudiantes
│ ├─ Cuando aplico filtros
│ └─ Entonces puedo filtrar por: progreso (sin iniciar, en curso, completado), fecha

├─ **Escenario 3: Exportación**
│ ├─ Dado que necesito los datos
│ ├─ Cuando hago clic en "Exportar"
│ └─ Entonces descargo CSV con todos los datos de estudiantes

└─ **Escenario 4: Detalle de estudiante**
├─ Dado que hago clic en un estudiante
├─ Cuando veo su perfil
└─ Entonces veo: progreso detallado por lección, calificaciones, última actividad

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-010
├─ Endpoint: GET /api/v1/courses/:id/enrollments
├─ Servicio: enrollments-service
└─ Estimación: 5 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoint con filtros y paginación
├─ [ ] Backend: Exportación CSV
├─ [ ] Frontend: Tabla de estudiantes con filtros
├─ [ ] Frontend: Vista de detalle de estudiante
└─ [ ] Tests: Filtros, exportación, permisos

---

### US-INS-002: Analytics de Curso ⚡

**Como** instructor  
**Quiero** ver métricas de rendimiento de mis cursos  
**Para** entender qué funciona y mejorar

**Criterios de Aceptación:**

├─ **Escenario 1: Dashboard de curso**
│ ├─ Dado que accedo a analytics de un curso
│ ├─ Cuando veo el dashboard
│ └─ Entonces veo: ventas totales, estudiantes activos, rating, tasa de completitud

├─ **Escenario 2: Engagement por lección**
│ ├─ Dado que analizo el contenido
│ ├─ Cuando veo la tabla de lecciones
│ └─ Entonces veo: vistas, tiempo promedio, drop-off rate por lección

├─ **Escenario 3: Tendencias temporales**
│ ├─ Dado que quiero ver evolución
│ ├─ Cuando selecciono rango de fechas
│ └─ Entonces veo gráficos de ventas, matrículas, actividad por semana/mes

├─ **Escenario 4: Identificación de problemas**
│ ├─ Dado que una lección tiene alto drop-off
│ ├─ Cuando veo el análisis
│ └─ Entonces veo sugerencias de mejora (muy larga, baja calidad, etc.)

└─ **Escenario 5: Comparación con promedios**
├─ Dado que veo mis métricas
├─ Cuando comparo con benchmarks
└─ Entonces veo cómo me comparo con promedios de la plataforma

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-016
├─ Endpoint: GET /api/v1/analytics/courses/:id
├─ Servicio: analytics-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Agregación de eventos por curso
├─ [ ] Backend: Cálculo de métricas y benchmarks
├─ [ ] Frontend: Dashboard con gráficos (Recharts/D3)
├─ [ ] Frontend: Tabla de lecciones con métricas
└─ [ ] Tests: Cálculos correctos, visualización

---

### US-INS-003: Reportes de Ingresos 🔥

**Como** instructor  
**Quiero** ver mis ingresos y proyecciones  
**Para** gestionar mi negocio de cursos online

**Criterios de Aceptación:**

├─ **Escenario 1: Resumen de ingresos**
│ ├─ Dado que accedo a mis reportes financieros
│ ├─ Cuando veo el resumen
│ └─ Entonces veo: ingresos brutos, comisión plataforma, ingresos netos, por período

├─ **Escenario 2: Desglose por curso**
│ ├─ Dado que tengo múltiples cursos
│ ├─ Cuando veo el desglose
│ └─ Entonces veo ingresos por cada curso con tendencia

├─ **Escenario 3: Historial de transacciones**
│ ├─ Dado que quiero ver cada venta
│ ├─ Cuando accedo al historial
│ └─ Entonces veo lista con: fecha, curso, monto, estado, método de pago

├─ **Escenario 4: Reembolsos**
│ ├─ Dado que hubo reembolsos
│ ├─ Cuando veo el reporte
│ └─ Entonces los reembolsos están claramente indicados y descontados

└─ **Escenario 5: Exportación**
├─ Dado que necesito datos para contabilidad
├─ Cuando exporto el reporte
└─ Entonces descargo PDF o CSV con todos los datos

**Notas Técnicas:**
├─ RF Relacionado: RF-INS-017
├─ Endpoint: GET /api/v1/instructors/me/revenue
├─ Servicio: payments-service, analytics-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Agregación de ventas por instructor
├─ [ ] Backend: Cálculo de comisiones
├─ [ ] Backend: Exportación PDF/CSV
├─ [ ] Frontend: Dashboard de ingresos
├─ [ ] Frontend: Gráficos de tendencias
└─ [ ] Tests: Cálculos, exportación

---

## Epic 10: Administración de Plataforma

### US-ADM-001: Gestión de Usuarios 🔥

**Como** administrador  
**Quiero** gestionar cuentas de usuarios  
**Para** mantener la seguridad y resolver problemas

**Criterios de Aceptación:**

├─ **Escenario 1: Listado de usuarios**
│ ├─ Dado que accedo a gestión de usuarios
│ ├─ Cuando veo la lista
│ └─ Entonces veo todos los usuarios con: nombre, email, rol, estado, fecha registro

├─ **Escenario 2: Búsqueda y filtros**
│ ├─ Dado que busco un usuario específico
│ ├─ Cuando uso búsqueda o filtros
│ └─ Entonces puedo encontrar por: email, nombre, rol, estado

├─ **Escenario 3: Cambio de rol**
│ ├─ Dado que necesito cambiar el rol de un usuario
│ ├─ Cuando selecciono nuevo rol
│ └─ Entonces se actualiza con audit log del cambio

├─ **Escenario 4: Suspensión de cuenta**
│ ├─ Dado que un usuario viola términos
│ ├─ Cuando suspendo la cuenta
│ └─ Entonces el usuario no puede acceder y recibe notificación

├─ **Escenario 5: Reset de contraseña forzado**
│ ├─ Dado que un usuario reporta problema
│ ├─ Cuando forzo reset de contraseña
│ └─ Entonces el usuario recibe email para crear nueva contraseña

└─ **Escenario 6: Impersonación (debugging)**
├─ Dado que necesito ver la app como un usuario
├─ Cuando activo impersonación
└─ Entonces veo la app como el usuario con banner visible y audit log

**Notas Técnicas:**
├─ RF Relacionado: RF-ADM-001, RF-ADM-002, RF-ADM-021
├─ Endpoints: GET/PATCH /api/v1/admin/users
├─ Servicio: users-service, auth-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Endpoints admin con autorización
├─ [ ] Backend: Audit logging de todas las acciones
├─ [ ] Backend: Impersonación con token especial
├─ [ ] Frontend: Panel de gestión de usuarios
├─ [ ] Frontend: Banner de impersonación
└─ [ ] Tests: Permisos, audit, impersonación

---

### US-ADM-002: Moderación de Contenido ⚡

**Como** administrador  
**Quiero** moderar cursos y reseñas  
**Para** mantener la calidad y evitar contenido inapropiado

**Criterios de Aceptación:**

├─ **Escenario 1: Cola de moderación**
│ ├─ Dado que hay contenido reportado
│ ├─ Cuando accedo a moderación
│ └─ Entonces veo lista de items pendientes de revisión

├─ **Escenario 2: Revisión de curso**
│ ├─ Dado que reviso un curso reportado
│ ├─ Cuando evalúo el contenido
│ └─ Entonces puedo: aprobar, rechazar, despublicar, solicitar cambios

├─ **Escenario 3: Moderación de reseñas**
│ ├─ Dado que una reseña es inapropiada
│ ├─ Cuando la elimino
│ └─ Entonces desaparece y el autor recibe notificación

├─ **Escenario 4: Notificación al autor**
│ ├─ Dado que tomo acción de moderación
│ ├─ Cuando completo la acción
│ └─ Entonces el autor recibe email con motivo y posibles acciones

└─ **Escenario 5: Historial de moderación**
├─ Dado que quiero ver acciones anteriores
├─ Cuando accedo al historial
└─ Entonces veo log de todas las acciones de moderación con contexto

**Notas Técnicas:**
├─ RF Relacionado: RF-ADM-005, RF-ADM-010
├─ Endpoints: GET/PATCH /api/v1/admin/moderation
├─ Servicio: courses-service, notifications-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Sistema de reportes y cola
├─ [ ] Backend: Acciones de moderación con audit
├─ [ ] Frontend: Panel de moderación
├─ [ ] Frontend: Visor de contenido reportado
└─ [ ] Tests: Flujo de moderación, notificaciones

---

### US-ADM-003: Dashboard Ejecutivo 🔥

**Como** administrador/dueño  
**Quiero** ver métricas clave de la plataforma  
**Para** tomar decisiones de negocio informadas

**Criterios de Aceptación:**

├─ **Escenario 1: KPIs principales**
│ ├─ Dado que accedo al dashboard
│ ├─ Cuando veo el resumen
│ └─ Entonces veo: usuarios totales, cursos activos, ingresos, matrículas, rating promedio

├─ **Escenario 2: Tendencias**
│ ├─ Dado que quiero ver evolución
│ ├─ Cuando selecciono período
│ └─ Entonces veo gráficos con tendencias y comparación vs período anterior

├─ **Escenario 3: Funnel de conversión**
│ ├─ Dado que analizo conversión
│ ├─ Cuando veo el funnel
│ └─ Entonces veo: visitantes → registros → compras con porcentajes

├─ **Escenario 4: Top performers**
│ ├─ Dado que quiero ver los mejores
│ ├─ Cuando veo rankings
│ └─ Entonces veo: top cursos por ventas, top instructores, cursos trending

└─ **Escenario 5: Alertas**
├─ Dado que hay situaciones que requieren atención
├─ Cuando accedo al dashboard
└─ Entonces veo alertas destacadas (caída de ventas, errores, etc.)

**Notas Técnicas:**
├─ RF Relacionado: RF-ADM-016, RF-ADM-017
├─ Endpoint: GET /api/v1/admin/dashboard
├─ Servicio: analytics-service
└─ Estimación: 13 SP

**Tareas de Implementación:**
├─ [ ] Backend: Agregación de métricas globales
├─ [ ] Backend: Cálculo de funnels y tendencias
├─ [ ] Frontend: Dashboard con widgets
├─ [ ] Frontend: Gráficos interactivos
└─ [ ] Tests: Cálculos, performance

---

## Epic 11: Notificaciones

### US-NOTIF-001: Emails Transaccionales 🔥

**Como** System (notifications-service)  
**Quiero** enviar emails automáticos en eventos clave  
**Para** mantener informados a los usuarios

**Criterios de Aceptación:**

├─ **Escenario 1: Email de bienvenida**
│ ├─ Dado que un usuario se registra
│ ├─ Cuando se emite evento user.registered
│ └─ Entonces se envía email de bienvenida en <30 segundos

├─ **Escenario 2: Confirmación de compra**
│ ├─ Dado que un pago es exitoso
│ ├─ Cuando se emite evento order.paid
│ └─ Entonces se envía email con recibo y enlace al curso

├─ **Escenario 3: Recordatorio de curso**
│ ├─ Dado que un estudiante lleva 7 días sin actividad
│ ├─ Cuando se detecta inactividad
│ └─ Entonces se envía email recordatorio (respetando preferencias)

├─ **Escenario 4: Nueva lección disponible**
│ ├─ Dado que el instructor publica nueva lección
│ ├─ Cuando se emite evento lesson.published
│ └─ Entonces se notifica a estudiantes matriculados

└─ **Escenario 5: Reintentos y manejo de errores**
├─ Dado que falla el envío de email
├─ Cuando se detecta el error
└─ Entonces se reintenta hasta 3 veces con backoff exponencial

**Notas Técnicas:**
├─ RF Relacionado: RF-SYS-004
├─ Events: user.registered, order.paid, enrollment.created, lesson.published
├─ Servicio: notifications-service
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Consumers de eventos
├─ [ ] Backend: Templates de email (MJML/React Email)
├─ [ ] Backend: Integración con SendGrid/SES
├─ [ ] Backend: Cola de reintentos
├─ [ ] Backend: Tracking de envíos
└─ [ ] Tests: Envío, templates, reintentos

---

## Epic 12: Infraestructura y Sistema

### US-SYS-001: Health Checks y Monitoreo 🔥

**Como** Backend (todos los servicios)  
**Quiero** exponer endpoints de health check  
**Para** que la infraestructura pueda monitorear y reiniciar servicios fallidos

**Criterios de Aceptación:**

├─ **Escenario 1: Health check básico**
│ ├─ Dado que el servicio está funcionando
│ ├─ Cuando se llama a /health
│ └─ Entonces responde 200 OK con status: "healthy"

├─ **Escenario 2: Verificación de dependencias**
│ ├─ Dado que se llama a /health/detailed
│ ├─ Cuando se verifican dependencias
│ └─ Entonces responde con estado de: DB, Redis, servicios externos

├─ **Escenario 3: Readiness check**
│ ├─ Dado que el servicio está iniciando
│ ├─ Cuando aún no está listo para recibir tráfico
│ └─ Entonces /ready responde 503 hasta que esté listo

├─ **Escenario 4: Liveness check**
│ ├─ Dado que el servicio está colgado
│ ├─ Cuando /live no responde en timeout
│ └─ Entonces Kubernetes reinicia el pod

└─ **Escenario 5: Métricas Prometheus**
├─ Dado que se necesitan métricas
├─ Cuando se llama a /metrics
└─ Entonces responde con métricas en formato Prometheus

**Notas Técnicas:**
├─ RF Relacionado: RF-SYS-012
├─ Endpoints: /health, /health/detailed, /ready, /live, /metrics
├─ Servicio: Todos
└─ Estimación: 3 SP por servicio

**Tareas de Implementación:**
├─ [ ] Backend: Middleware de health checks
├─ [ ] Backend: Verificación de conexiones a DB/Redis
├─ [ ] Backend: Exportación de métricas Prometheus
├─ [ ] Infra: Configuración de probes en K8s
└─ [ ] Tests: Health checks, métricas

---

### US-SYS-002: Logging Estructurado 🔥

**Como** Backend (todos los servicios)  
**Quiero** generar logs estructurados en JSON  
**Para** facilitar debugging y análisis en producción

**Criterios de Aceptación:**

├─ **Escenario 1: Formato JSON**
│ ├─ Dado que ocurre cualquier evento logueable
│ ├─ Cuando se genera el log
│ └─ Entonces es JSON con: timestamp, level, service, message, context

├─ **Escenario 2: Correlation ID**
│ ├─ Dado que llega un request
│ ├─ Cuando se procesa a través del sistema
│ └─ Entonces todos los logs relacionados tienen el mismo correlationId

├─ **Escenario 3: Niveles de log**
│ ├─ Dado que hay diferentes eventos
│ ├─ Cuando se loguean
│ └─ Entonces usan nivel apropiado: debug, info, warn, error

├─ **Escenario 4: Contexto de usuario**
│ ├─ Dado que el request está autenticado
│ ├─ Cuando se genera log
│ └─ Entonces incluye userId (no datos sensibles como password)

└─ **Escenario 5: Sanitización**
├─ Dado que el log podría contener datos sensibles
├─ Cuando se genera
└─ Entonces passwords, tokens, y PII son redactados

**Notas Técnicas:**
├─ RF Relacionado: RF-GLOBAL-005
├─ Servicio: Todos
└─ Estimación: 3 SP por servicio

**Tareas de Implementación:**
├─ [ ] Backend: Logger estructurado (tracing crate en Rust)
├─ [ ] Backend: Middleware para correlation ID
├─ [ ] Backend: Sanitización de campos sensibles
├─ [ ] Infra: Configuración de log shipping
└─ [ ] Tests: Formato de logs, sanitización

---

### US-SYS-003: Procesamiento de Eventos 🔥

**Como** Backend (event bus)  
**Quiero** procesar eventos de dominio de forma confiable  
**Para** mantener los servicios desacoplados y consistentes

**Criterios de Aceptación:**

├─ **Escenario 1: Publicación de eventos**
│ ├─ Dado que ocurre un evento de dominio
│ ├─ Cuando el servicio lo emite
│ └─ Entonces se publica a la cola de mensajes con garantía at-least-once

├─ **Escenario 2: Consumo de eventos**
│ ├─ Dado que un servicio está suscrito a un evento
│ ├─ Cuando llega el mensaje
│ └─ Entonces lo procesa y confirma (ack) solo si fue exitoso

├─ **Escenario 3: Reintentos**
│ ├─ Dado que falla el procesamiento
│ ├─ Cuando el servicio no hace ack
│ └─ Entonces el mensaje se reintenta con backoff exponencial

├─ **Escenario 4: Dead letter queue**
│ ├─ Dado que un mensaje falla repetidamente
│ ├─ Cuando se superan los reintentos
│ └─ Entonces va a DLQ para revisión manual

└─ **Escenario 5: Idempotencia**
├─ Dado que un evento se recibe duplicado
├─ Cuando se procesa
└─ Entonces se detecta duplicado y se ignora sin efectos secundarios

**Notas Técnicas:**
├─ RF Relacionado: RF-SYS-003
├─ Tecnología: Redis Streams o RabbitMQ
├─ Servicio: Todos
└─ Estimación: 8 SP

**Tareas de Implementación:**
├─ [ ] Backend: Abstracción de event bus
├─ [ ] Backend: Publisher con outbox pattern
├─ [ ] Backend: Consumer con idempotencia
├─ [ ] Backend: Dead letter queue
├─ [ ] Infra: Setup de cola de mensajes
└─ [ ] Tests: Publicación, consumo, reintentos, idempotencia

---

## Resumen del Backlog

### Por Prioridad

| Prioridad   | Historias | Story Points |
| ----------- | --------- | ------------ |
| 🔥 Critical | 18        | ~120 SP      |
| ⚡ High     | 16        | ~130 SP      |
| 🎯 Medium   | 6         | ~45 SP       |
| 💡 Low      | 2         | ~26 SP       |
| **Total**   | **42**    | **~321 SP**  |

### Por Epic

| Epic                       | Historias | Story Points |
| -------------------------- | --------- | ------------ |
| 1. Autenticación           | 5         | 26 SP        |
| 2. Perfil de Usuario       | 3         | 15 SP        |
| 3. Catálogo                | 3         | 24 SP        |
| 4. Gestión de Cursos       | 5         | 65 SP        |
| 5. Pagos                   | 4         | 31 SP        |
| 6. Aprendizaje             | 4         | 28 SP        |
| 7. Evaluaciones            | 3         | 26 SP        |
| 8. Inteligencia Artificial | 3         | 47 SP        |
| 9. Instructor              | 3         | 26 SP        |
| 10. Administración         | 3         | 29 SP        |
| 11. Notificaciones         | 1         | 8 SP         |
| 12. Infraestructura        | 3         | 14 SP        |

### Roadmap Sugerido (10 Sprints de 2 semanas)

**Sprint 1-2: Fundación**

- US-AUTH-001 a US-AUTH-005 (Autenticación completa)
- US-SYS-001 a US-SYS-003 (Infraestructura base)
- US-PROFILE-001 (Perfil básico)

**Sprint 3-4: Catálogo y Contenido**

- US-CAT-001 a US-CAT-003 (Catálogo completo)
- US-COURSE-001, US-COURSE-002 (Creación de cursos)
- US-NOTIF-001 (Emails base)

**Sprint 5-6: Comercio**

- US-PAY-001 a US-PAY-004 (Pagos completo)
- US-COURSE-003 (Upload de videos)
- US-LEARN-001 (Dashboard estudiante)

**Sprint 7-8: Aprendizaje**

- US-LEARN-002 a US-LEARN-004 (Player y progreso)
- US-QUIZ-001 a US-QUIZ-003 (Evaluaciones)
- US-COURSE-004, US-COURSE-005 (Quizzes y publicación)

**Sprint 9-10: IA y Analytics**

- US-AI-001 a US-AI-003 (Funcionalidades IA)
- US-INS-001 a US-INS-003 (Instructor analytics)
- US-ADM-001 a US-ADM-003 (Administración)

### Definition of Done (DoD)

Cada historia se considera **DONE** cuando:

- [ ] Código implementado y funcionando
- [ ] Tests unitarios con >80% cobertura
- [ ] Tests de integración para flujos críticos
- [ ] Code review aprobado
- [ ] Documentación API actualizada (OpenAPI)
- [ ] Sin errores de linting ni security warnings
- [ ] Desplegado en staging y verificado
- [ ] Logs y métricas funcionando
- [ ] Cumple todos los criterios de aceptación

### Definition of Ready (DoR)

Una historia está **READY** para desarrollo cuando:

- [ ] Criterios de aceptación definidos y claros
- [ ] Dependencias identificadas
- [ ] Diseño UI/UX disponible (si aplica)
- [ ] Estimación acordada por el equipo
- [ ] Sin bloqueadores conocidos
- [ ] APIs de dependencias documentadas

---

**Total historias:** 42  
**Total estimado:** ~321 story points  
**Equipo sugerido:** 4-5 developers  
**Duración estimada:** 10 sprints (20 semanas)  
**Fecha de actualización:** 2025-12-14
