# ACC LMS — Historias de Usuario y Criterios de Aceptación

**Versión:** 2025-08-08  
**Estado:** Backlog definido para implementación  
**Derivado de:** functional-requirements.md

---

## Estructura de Historias de Usuario

**Formato estándar:**

```
Como [ROL]
Quiero [FUNCIONALIDAD]
Para [BENEFICIO/OBJETIVO]

Criterios de Aceptación:
- Dado [CONTEXTO]
- Cuando [ACCIÓN]
- Entonces [RESULTADO ESPERADO]
```

**Priorización:**

- 🔥 **Critical:** Bloquea MVP
- ⚡ **High:** Impacto alto en UX/revenue
- 🎯 **Medium:** Importante para completitud
- 💡 **Low:** Nice to have, backlog futuro

---

## Epic 1: Autenticación y Gestión de Usuarios

### US-001: Registro de Usuario Nuevo 🔥

**Como** visitante anónimo  
**Quiero** registrarme en la plataforma  
**Para** acceder al catálogo de cursos y crear mi perfil de aprendizaje

**Criterios de Aceptación:**

- **Dado** que soy un visitante sin cuenta
- **Cuando** completo el formulario de registro con email válido, contraseña segura, nombre y apellido
- **Entonces** recibo confirmación de cuenta creada y acceso automático a la plataforma

- **Dado** que intento registrarme con un email ya existente
- **Cuando** envío el formulario
- **Entonces** veo mensaje "Email ya registrado" sin revelar información de la cuenta existente

- **Dado** que ingreso una contraseña débil (menos de 10 caracteres, sin mayúscula/minúscula/número/símbolo)
- **Cuando** envío el formulario
- **Entonces** veo indicador visual de fortaleza y sugerencias específicas para mejorarla

**RF Relacionado:** RF-AUTH-001  
**Endpoint:** POST /api/v1/auth/register  
**Estimación:** 5 story points

---

### US-002: Login de Usuario Existente 🔥

**Como** usuario registrado  
**Quiero** iniciar sesión con mis credenciales  
**Para** acceder a mis cursos y progreso personal

**Criterios de Aceptación:**

- **Dado** que tengo credenciales válidas
- **Cuando** ingreso email y contraseña correctos
- **Entonces** accedo a mi dashboard personalizado en menos de 2 segundos

- **Dado** que ingreso credenciales incorrectas
- **Cuando** envío el formulario de login
- **Entonces** veo mensaje genérico "Credenciales inválidas" después de 3 intentos fallidos

- **Dado** que he fallado 5 veces consecutivas
- **Cuando** intento login nuevamente
- **Entonces** mi cuenta se bloquea temporalmente por 15 minutos con mensaje explicativo

- **Dado** que marco "Recordarme"
- **Cuando** cierro y reabro el navegador
- **Entonces** permanezco autenticado por hasta 7 días sin necesidad de re-login

**RF Relacionado:** RF-AUTH-002  
**Endpoint:** POST /api/v1/auth/login  
**Estimación:** 3 story points

---

### US-003: Gestión de Perfil Personal ⚡

**Como** usuario autenticado  
**Quiero** editar mi información personal y preferencias  
**Para** mantener mi perfil actualizado y personalizar mi experiencia

**Criterios de Aceptación:**

- **Dado** que estoy en mi página de perfil
- **Cuando** modifico mi nombre, avatar, bio o preferencias de idioma
- **Entonces** los cambios se guardan instantáneamente con feedback visual de confirmación

- **Dado** que subo una foto de perfil
- **Cuando** selecciono un archivo JPG/PNG menor a 2MB
- **Entonces** la imagen se procesa, redimensiona automáticamente y se muestra en tiempo real

- **Dado** que configuro mis preferencias de notificación
- **Cuando** desactivo emails promocionales
- **Entonces** no recibo más comunicaciones de marketing pero sí notificaciones de cursos

**RF Relacionado:** RF-USERS-002, RF-USERS-003  
**Endpoints:** PATCH /api/v1/users/:id, PATCH /api/v1/users/:id/preferences  
**Estimación:** 5 story points

---

## Epic 2: Catálogo y Gestión de Cursos

### US-004: Explorar Catálogo de Cursos 🔥

**Como** visitante o estudiante  
**Quiero** navegar y filtrar el catálogo de cursos  
**Para** encontrar contenido relevante a mis intereses y objetivos

**Criterios de Aceptación:**

- **Dado** que estoy en la página principal
- **Cuando** veo el catálogo de cursos
- **Entonces** visualizo máximo 20 cursos por página con imagen, título, precio, rating y duración

- **Dado** que uso filtros de categoría, precio o nivel
- **Cuando** aplico múltiples filtros simultáneamente
- **Entonces** la lista se actualiza en tiempo real sin recargar la página completa

- **Dado** que busco "JavaScript avanzado"
- **Cuando** escribo en el campo de búsqueda
- **Entonces** veo resultados relevantes ordenados por relevancia con highlighting de términos

- **Dado** que estoy en móvil
- **Cuando** navego el catálogo
- **Entonces** la interfaz se adapta completamente con diseño touch-friendly

**RF Relacionado:** RF-COURSES-001  
**Endpoint:** GET /api/v1/courses  
**Estimación:** 8 story points

---

### US-005: Ver Detalle de Curso ⚡

**Como** estudiante potencial  
**Quiero** ver información completa de un curso  
**Para** decidir si vale la pena comprarlo

**Criterios de Aceptación:**

- **Dado** que hago clic en un curso del catálogo
- **Cuando** accedo a la página de detalle
- **Entonces** veo descripción completa, temario, instructor, reviews, duración y precio claramente organizados

- **Dado** que el curso tiene lecciones de vista previa
- **Cuando** reproduzco una lección gratuita
- **Entonces** el video se reproduce sin necesidad de registro con calidad adaptativa

- **Dado** que leo reviews de otros estudiantes
- **Cuando** reviso las calificaciones
- **Entonces** veo promedio, distribución de estrellas y comentarios más útiles destacados

- **Dado** que quiero comprar el curso
- **Cuando** hago clic en "Comprar"
- **Entonces** soy dirigido al checkout o login si no estoy autenticado

**RF Relacionado:** RF-COURSES-003  
**Endpoint:** GET /api/v1/courses/:id  
**Estimación:** 8 story points

---

### US-006: Crear Curso (Instructor) ⚡

**Como** instructor verificado  
**Quiero** crear un nuevo curso desde cero  
**Para** compartir mi conocimiento y generar ingresos

**Criterios de Aceptación:**

- **Dado** que tengo rol de instructor
- **Cuando** inicio la creación de curso
- **Entonces** soy guiado por un wizard paso a paso con preview en tiempo real

- **Dado** que estoy en el paso de información básica
- **Cuando** completo título, descripción, categoría y precio
- **Entonces** el sistema auto-genera un slug SEO-friendly editable

- **Dado** que subo una imagen de thumbnail
- **Cuando** selecciono un archivo de alta resolución
- **Entonces** se procesa automáticamente en múltiples tamaños para responsive design

- **Dado** que guardo un curso en draft
- **Cuando** cierro y vuelvo al editor
- **Entonces** recupero exactamente donde quedé con auto-save cada 30 segundos

**RF Relacionado:** RF-COURSES-002  
**Endpoint:** POST /api/v1/courses  
**Estimación:** 13 story points

---

### US-007: Gestionar Lecciones del Curso ⚡

**Como** instructor  
**Quiero** agregar, editar y organizar lecciones  
**Para** estructurar el contenido educativo de manera lógica

**Criterios de Aceptación:**

- **Dado** que estoy editando un curso
- **Cuando** agrego una nueva lección
- **Entonces** puedo elegir tipo (video, artículo, quiz), subir contenido y establecer duración

- **Dado** que quiero reordenar lecciones
- **Cuando** uso drag & drop para cambiar secuencia
- **Entonces** los índices se recalculan automáticamente y se preserva la numeración

- **Dado** que subo un video de 500MB
- **Cuando** el archivo se está procesando
- **Entonces** veo progreso en tiempo real y recibo notificación cuando termine la codificación

- **Dado** que marco una lección como "preview gratuita"
- **Cuando** publico el curso
- **Entonces** esa lección es visible para usuarios no matriculados

**RF Relacionado:** RF-COURSES-004  
**Endpoints:** POST/PATCH/DELETE /api/v1/courses/:id/lessons  
**Estimación:** 13 story points

---

## Epic 3: Comercio y Pagos

### US-008: Proceso de Compra de Curso 🔥

**Como** estudiante interesado  
**Quiero** comprar un curso de manera segura y rápida  
**Para** acceder inmediatamente al contenido

**Criterios de Aceptación:**

- **Dado** que decido comprar un curso
- **Cuando** hago clic en "Comprar"
- **Entonces** soy llevado a un checkout optimizado con resumen claro del pedido

- **Dado** que estoy en el checkout
- **Cuando** selecciono método de pago (tarjeta/MercadoPago)
- **Entonces** el formulario se adapta mostrando campos específicos con validación en tiempo real

- **Dado** que completo el pago exitosamente
- **Cuando** la transacción se confirma
- **Entonces** recibo acceso inmediato al curso y email de confirmación con recibo

- **Dado** que el pago falla por fondos insuficientes
- **Cuando** ocurre el error
- **Entonces** veo mensaje específico del problema y opciones para reintentar o cambiar método

**RF Relacionado:** RF-PAY-001, RF-PAY-002  
**Endpoint:** POST /api/v1/orders  
**Estimación:** 13 story points

---

### US-009: Webhook de Confirmación de Pago (Sistema) 🔥

**Como** sistema de pagos  
**Quiero** procesar webhooks de Stripe/MercadoPago de manera confiable  
**Para** activar automáticamente las matrículas tras pagos confirmados

**Criterios de Aceptación:**

- **Dado** que recibo un webhook de pago exitoso
- **Cuando** valido la firma y datos del proveedor
- **Entonces** creo la matrícula automáticamente en menos de 5 segundos

- **Dado** que recibo el mismo webhook duplicado
- **Cuando** proceso la solicitud
- **Entonces** respondo 200 OK sin crear matrícula duplicada (idempotencia)

- **Dado** que el webhook llega fuera de orden
- **Cuando** intento procesar un evento de pago anterior
- **Entonces** rechazo el evento obsoleto y mantengo el estado más reciente

- **Dado** que falla el procesamiento interno
- **Cuando** no puedo crear la matrícula por error de BD
- **Entonces** reintento automáticamente hasta 3 veces con backoff exponencial

**RF Relacionado:** RF-PAY-002  
**Endpoints:** POST /api/v1/payments/webhook/stripe, /mercadopago  
**Estimación:** 8 story points

---

## Epic 4: Experiencia de Aprendizaje

### US-010: Player de Video Interactivo ⚡

**Como** estudiante matriculado  
**Quiero** ver lecciones en video con funciones avanzadas  
**Para** optimizar mi experiencia de aprendizaje

**Criterios de Aceptación:**

- **Dado** que reproduzco una lección
- **Cuando** uso el player de video
- **Entonces** puedo ajustar velocidad (0.5x a 2x), activar subtítulos y cambiar calidad

- **Dado** que pauso el video y cambio de pestaña
- **Cuando** regreso días después
- **Entonces** el video resume exactamente donde lo dejé

- **Dado** que estoy viendo en móvil con conexión lenta
- **Cuando** el video se reproduce
- **Entonces** la calidad se ajusta automáticamente para evitar buffering

- **Dado** que completo una lección
- **Cuando** el video termina
- **Entonces** se marca automáticamente como completada y avanza al siguiente contenido

**RF Relacionado:** RF-FE-002  
**Frontend:** /learn/:courseId  
**Estimación:** 13 story points

---

### US-011: Seguimiento de Progreso Personal ⚡

**Como** estudiante activo  
**Quiero** ver mi progreso en cada curso  
**Para** mantener motivación y planificar mi aprendizaje

**Criterios de Aceptación:**

- **Dado** que estoy en mi dashboard
- **Cuando** reviso mis cursos activos
- **Entonces** veo porcentaje de completación, tiempo estimado restante y última actividad

- **Dado** que completo una lección
- **Cuando** marco como terminada
- **Entonces** el progreso se actualiza inmediatamente y veo celebración visual

- **Dado** que llevo 30 días sin actividad en un curso
- **Cuando** reviso mi progreso
- **Entonces** veo recordatorio motivacional y sugerencia de retomar

- **Dado** que termino completamente un curso
- **Cuando** veo la última lección
- **Entonces** recibo certificado de completación y sugerencias de cursos relacionados

**RF Relacionado:** RF-ENR-003  
**Endpoint:** PATCH /api/v1/enrollments/:id/progress  
**Estimación:** 8 story points

---

## Epic 5: Evaluaciones y Feedback

### US-012: Realizar Quiz de Evaluación 🎯

**Como** estudiante  
**Quiero** completar quizzes y evaluaciones  
**Para** validar mi comprensión del material

**Criterios de Aceptación:**

- **Dado** que llego a una lección tipo quiz
- **Cuando** inicio la evaluación
- **Entonces** veo preguntas una por vez con indicador de progreso y tiempo estimado

- **Dado** que respondo una pregunta de opción múltiple
- **Cuando** selecciono mi respuesta
- **Entonces** puedo cambiar la selección antes de enviar y veo confirmación visual

- **Dado** que envío el quiz completo
- **Cuando** procesamiento termina
- **Entonces** veo mi puntuación, respuestas correctas/incorrectas y explicaciones detalladas

- **Dado** que fallo el quiz con menos de 70%
- **Cuando** veo los resultados
- **Entonces** puedo reintentarlo después de 1 hora con preguntas aleatorias diferentes

**RF Relacionado:** RF-ASSIGN-001, RF-ASSIGN-002  
**Endpoints:** GET /api/v1/quizzes/:id, POST /api/v1/quizzes/:id/submit  
**Estimación:** 13 story points

---

### US-013: Auto-feedback de Código (IA) 💡

**Como** estudiante de programación  
**Quiero** recibir feedback automático de mis ejercicios de código  
**Para** mejorar mis habilidades sin esperar revisión manual

**Criterios de Aceptación:**

- **Dado** que envío código Python/JavaScript
- **Cuando** el sistema IA procesa mi submission
- **Entonces** recibo feedback sobre funcionalidad, estilo y eficiencia en menos de 30 segundos

- **Dado** que mi código tiene errores de sintaxis
- **Cuando** ejecuta el auto-feedback
- **Entonces** veo ubicación exacta del error y sugerencias específicas para corregir

- **Dado** que mi código funciona pero es ineficiente
- **Cuando** recibo el feedback
- **Entonces** veo alternativas optimizadas y explicación de complejidad temporal

- **Dado** que quiero entender mejor el feedback
- **Cuando** hago clic en "Explicar más"
- **Entonces** accedo a recursos adicionales y ejemplos relacionados

**RF Relacionado:** RF-AI-005  
**Endpoint:** POST /api/v1/ai/feedback/code  
**Estimación:** 21 story points

---

## Epic 6: Inteligencia Artificial

### US-014: Tutor Virtual por Curso (RAG) ⚡

**Como** estudiante confundido  
**Quiero** hacer preguntas sobre el contenido del curso  
**Para** resolver dudas inmediatamente sin esperar al instructor

**Criterios de Aceptación:**

- **Dado** que estoy viendo una lección
- **Cuando** abro el chat del tutor IA
- **Entonces** puedo hacer preguntas en lenguaje natural sobre el contenido específico

- **Dado** que pregunto "¿Cómo funciona React hooks?"
- **Cuando** el tutor procesa mi pregunta
- **Entonces** recibo respuesta contextualizada basada en las lecciones del curso que he tomado

- **Dado** que hago una pregunta fuera del alcance del curso
- **Cuando** el tutor intenta responder
- **Entonces** me indica cortésmente que solo puede ayudar con contenido del curso matriculado

- **Dado** que la respuesta incluye referencias a lecciones
- **Cuando** hago clic en los enlaces
- **Entonces** soy llevado directamente al timestamp exacto del video relacionado

**RF Relacionado:** RF-AI-003  
**Endpoints:** POST /api/v1/ai/tutor/sessions, POST /api/v1/ai/tutor/sessions/:id/messages  
**Estimación:** 21 story points

---

### US-015: Búsqueda Semántica de Contenido 🎯

**Como** estudiante o instructor  
**Quiero** buscar contenido usando lenguaje natural  
**Para** encontrar información específica sin recordar palabras clave exactas

**Criterios de Aceptación:**

- **Dado** que busco "cómo manejar errores en programación asíncrona"
- **Cuando** uso la búsqueda semántica
- **Entonces** encuentro lecciones relevantes aunque no contengan esas palabras exactas

- **Dado** que busco en múltiples cursos
- **Cuando** ejecuto la búsqueda
- **Entonces** veo resultados ordenados por relevancia con snippets destacados

- **Dado** que refino mi búsqueda con filtros
- **Cuando** combino búsqueda semántica con filtros tradicionales
- **Entonces** obtengo resultados híbridos más precisos

- **Dado** que busco contenido premium
- **Cuando** no tengo acceso a ciertos cursos
- **Entonces** veo teaser de resultados con opción de comprar para acceso completo

**RF Relacionado:** RF-AI-001  
**Endpoint:** GET /api/v1/ai/semantic-search  
**Estimación:** 13 story points

---

## Epic 7: Business Intelligence (Dueño/Admin)

### US-016: Dashboard Ejecutivo en Tiempo Real 🔥

**Como** dueño de la plataforma  
**Quiero** ver métricas de negocio en tiempo real  
**Para** tomar decisiones estratégicas basadas en datos

**Criterios de Aceptación:**

- **Dado** que accedo al dashboard ejecutivo
- **Cuando** veo las métricas principales
- **Entonces** observo MRR, ARR, usuarios activos, conversión y churn actualizados cada 5 minutos

- **Dado** que reviso el performance mensual
- **Cuando** comparo con meses anteriores
- **Entonces** veo tendencias visuales claras con indicadores de crecimiento o declive

- **Dado** que identifico una métrica preocupante
- **Cuando** hago clic para drill-down
- **Entonces** accedo a análisis detallado con segmentaciones y posibles causas

- **Dado** que configuro alertas personalizadas
- **Cuando** una métrica supera umbrales definidos
- **Entonces** recibo notificación inmediata por email y push notification

**RF Relacionado:** RF-BI-001  
**Endpoint:** GET /api/v1/bi/executive-dashboard  
**Estimación:** 13 story points

---

### US-017: Alertas Inteligentes de Anomalías 🔥

**Como** dueño del negocio  
**Quiero** ser notificado automáticamente de situaciones críticas  
**Para** actuar rápidamente ante problemas u oportunidades

**Criterios de Aceptación:**

- **Dado** que los ingresos caen 15% vs mes anterior
- **Cuando** se detecta la anomalía
- **Entonces** recibo alerta inmediata con análisis de posibles causas y acciones sugeridas

- **Dado** que un competidor lanza producto similar
- **Cuando** el sistema detecta amenaza competitiva
- **Entonces** soy notificado con análisis de impacto y estrategias de respuesta

- **Dado** que se detecta oportunidad de crecimiento
- **Cuando** aumenta demanda en categoría específica
- **Entonces** recibo sugerencia proactiva de contenido a desarrollar con ROI estimado

- **Dado** que quiero configurar sensibilidad de alertas
- **Cuando** ajusto parámetros de detección
- **Entonces** puedo balancear entre sensibilidad alta y ruido excesivo

**RF Relacionado:** RF-BI-005  
**Endpoint:** GET /api/v1/bi/alerts, WebSocket /bi/alerts  
**Estimación:** 13 story points

---

### US-018: Análisis Predictivo de Churn ⚡

**Como** dueño enfocado en retención  
**Quiero** identificar usuarios en riesgo de abandono  
**Para** implementar estrategias de retención proactivas

**Criterios de Aceptación:**

- **Dado** que reviso predicciones de churn
- **Cuando** accedo al análisis predictivo
- **Entonces** veo lista de usuarios con probabilidad de abandono y razones específicas

- **Dado** que identifico un usuario con 85% probabilidad de churn
- **Cuando** reviso sus detalles
- **Entonces** veo intervenciones sugeridas (descuento, contacto personal, contenido adicional)

- **Dado** que implemento estrategia de retención
- **Cuando** ejecuto campaña dirigida
- **Entonces** puedo medir efectividad comparando churn real vs. predicho

- **Dado** que analizo patrones de churn
- **Cuando** segmento por características del usuario
- **Entonces** identifico factores de riesgo comunes para prevención sistemática

**RF Relacionado:** RF-BI-007  
**Endpoint:** GET /api/v1/bi/predictive-insights  
**Estimación:** 21 story points

---

## Epic 8: Infraestructura y DevOps

### US-019: Monitoreo de Salud del Sistema 🔥

**Como** administrador de sistemas  
**Quiero** monitorear salud de todos los servicios  
**Para** mantener disponibilidad y performance óptimos

**Criterios de Aceptación:**

- **Dado** que accedo al dashboard de infraestructura
- **Cuando** reviso el status de servicios
- **Entonces** veo estado en tiempo real de cada microservicio con latencia y error rate

- **Dado** que un servicio falla health check
- **Cuando** se detecta la indisponibilidad
- **Entonces** se activa automáticamente failover y se envían alertas al equipo técnico

- **Dado** que la latencia P95 supera 500ms
- **Cuando** se detecta degradación de performance
- **Entonces** se escalan automáticamente recursos y se investigan bottlenecks

- **Dado** que reviso métricas históricas
- **Cuando** analizo tendencias de carga
- **Entonces** puedo planificar capacidad y identificar patrones de uso

**RF Relacionado:** RF-GLOBAL-004, RNF-002  
**Endpoints:** /health, /metrics (por servicio)  
**Estimación:** 8 story points

---

### US-020: Deployment Automático y Rollback 🎯

**Como** desarrollador  
**Quiero** desplegar código de manera segura y confiable  
**Para** entregar features rápidamente sin riesgo de downtime

**Criterios de Aceptación:**

- **Dado** que hago push a rama main
- **Cuando** se ejecuta el pipeline CI/CD
- **Entonces** el código pasa tests, security scan y se despliega automáticamente en staging

- **Dado** que los tests E2E pasan en staging
- **Cuando** aprobo deployment a producción
- **Entonces** se ejecuta blue-green deployment con zero downtime

- **Dado** que se detectan errores post-deployment
- **Cuando** error rate supera 5% por más de 2 minutos
- **Entonces** se ejecuta rollback automático a versión anterior estable

- **Dado** que quiero deployar feature específica
- **Cuando** uso feature flags
- **Entonces** puedo activar/desactivar funcionalidad sin re-deployment

**RF Relacionado:** RNF-007  
**Tools:** GitHub Actions, Docker, K8s  
**Estimación:** 13 story points

---

## Resumen de Backlog

### Por Prioridad:

- **🔥 Critical (MVP Blockers):** 8 historias - 65 story points
- **⚡ High Priority:** 8 historias - 89 story points
- **🎯 Medium Priority:** 3 historias - 34 story points
- **💡 Low Priority:** 1 historia - 21 story points

### Por Epic:

1. **Autenticación:** 3 historias - 13 points
2. **Catálogo/Cursos:** 4 historias - 42 points
3. **Comercio:** 2 historias - 21 points
4. **Aprendizaje:** 2 historias - 21 points
5. **Evaluaciones:** 2 historias - 34 points
6. **IA:** 2 historias - 34 points
7. **Business Intelligence:** 3 historias - 47 points
8. **Infraestructura:** 2 historias - 21 points

### Roadmap Sugerido:

**Sprint 1-2:** US-001, US-002, US-016, US-019 (Fundación + Monitoreo)
**Sprint 3-4:** US-004, US-005, US-008, US-009 (Comercio básico)
**Sprint 5-6:** US-006, US-007, US-010, US-011 (Contenido + UX)
**Sprint 7-8:** US-012, US-014, US-017, US-018 (IA + Analytics)

**Total estimado:** 233 story points (≈ 8-10 sprints para equipo de 5 developers)
