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

## Epic 9: Cumplimiento Legal y Privacidad

### US-050: Ejercer Derechos ARCO 🔥

**Como** usuario de la plataforma  
**Quiero** ejercer mis derechos de Acceso, Rectificación, Cancelación y Oposición (ARCO)  
**Para** controlar mis datos personales según la ley Habeas Data (Colombia)

**Criterios de Aceptación:**

- **Dado** que estoy autenticado y accedo al portal de privacidad
- **Cuando** solicito acceso a mis datos personales
- **Entonces** puedo ver y descargar toda la información que la plataforma tiene sobre mí en máximo 15 días hábiles

- **Dado** que identifico datos incorrectos en mi perfil
- **Cuando** solicito rectificación indicando los datos a corregir
- **Entonces** mis datos se actualizan y recibo confirmación de la corrección

- **Dado** que quiero eliminar mi cuenta y todos mis datos
- **Cuando** solicito cancelación/supresión
- **Entonces** mis datos se eliminan permanentemente en máximo 15 días hábiles (excepto datos con retención legal obligatoria)

- **Dado** que no deseo que mis datos se usen para marketing
- **Cuando** ejerzo mi derecho de oposición
- **Entonces** se detiene todo tratamiento de mis datos para fines no esenciales

**RF Relacionado:** RF-COMPLIANCE-005, RF-COMPLIANCE-006  
**Endpoints:** GET/POST /api/v1/compliance/data-requests  
**Estimación:** 8 story points

---

### US-051: Portabilidad de Datos (GDPR) ⚡

**Como** usuario residente en la UE  
**Quiero** exportar mis datos en formato legible por máquina  
**Para** llevarlos a otra plataforma según mi derecho de portabilidad GDPR

**Criterios de Aceptación:**

- **Dado** que solicito exportación de mis datos
- **Cuando** selecciono formato JSON o CSV
- **Entonces** recibo enlace de descarga con mis datos en máximo 30 días (estándar GDPR)

- **Dado** que mis datos incluyen historial de cursos, progreso y certificados
- **Cuando** descargo el archivo de exportación
- **Entonces** toda esta información está incluida en formato estructurado y documentado

- **Dado** que el archivo está listo
- **Cuando** recibo notificación
- **Entonces** tengo 7 días para descargarlo antes de que expire el enlace por seguridad

- **Dado** que quiero transferir mis datos a otra plataforma educativa
- **Cuando** proporciono endpoint de destino autorizado
- **Entonces** mis datos se transmiten directamente de forma segura (opcional, si el destino lo soporta)

**RF Relacionado:** RF-COMPLIANCE-010  
**Endpoint:** POST /api/v1/compliance/export  
**Estimación:** 8 story points

---

### US-052: Opt-out de Venta de Datos (CCPA) 🔥

**Como** usuario residente en California  
**Quiero** ejercer mi derecho de opt-out de venta de datos personales  
**Para** cumplir con mis derechos bajo CCPA/CPRA

**Criterios de Aceptación:**

- **Dado** que soy residente de California (detectado por IP o declarado)
- **Cuando** visito cualquier página de la plataforma
- **Entonces** veo enlace visible "Do Not Sell My Personal Information" en el footer

- **Dado** que hago clic en el enlace de opt-out
- **Cuando** confirmo mi elección
- **Entonces** mis datos se marcan como "no vendible" inmediatamente sin necesidad de autenticación

- **Dado** que he ejercido opt-out
- **Cuando** terceros solicitan mis datos
- **Entonces** la plataforma rechaza la solicitud y registra el intento

- **Dado** que la plataforma recibe solicitud de Authorized Agent
- **Cuando** el agente presenta autorización verificable
- **Entonces** puede ejercer derechos en mi nombre previo proceso de verificación

**RF Relacionado:** RF-COMPLIANCE-009  
**Endpoint:** POST /api/v1/compliance/ccpa/opt-out  
**Estimación:** 5 story points

---

### US-053: Gestión de Consentimientos 🔥

**Como** usuario de la plataforma  
**Quiero** gestionar mis consentimientos de forma granular  
**Para** controlar exactamente cómo se usan mis datos

**Criterios de Aceptación:**

- **Dado** que visito la plataforma por primera vez
- **Cuando** se muestra el banner de cookies
- **Entonces** puedo aceptar todas, rechazar no esenciales, o configurar individualmente cada categoría

- **Dado** que accedo a mi panel de privacidad
- **Cuando** reviso mis consentimientos
- **Entonces** veo lista completa de todos los consentimientos con fecha de otorgamiento y opción de revocar

- **Dado** que revoco un consentimiento de marketing
- **Cuando** confirmo la revocación
- **Entonces** se detiene inmediatamente todo procesamiento basado en ese consentimiento

- **Dado** que la plataforma añade nuevo uso de datos
- **Cuando** el uso requiere consentimiento
- **Entonces** se me solicita explícitamente antes de activarlo, sin casillas pre-marcadas

**RF Relacionado:** RF-COMPLIANCE-004, RF-COMPLIANCE-012, RF-COMPLIANCE-013  
**Endpoints:** GET/PATCH /api/v1/compliance/consents  
**Estimación:** 8 story points

---

## Epic 10: Chatbot y Soporte

### US-060: Obtener Ayuda del Chatbot 🔥

**Como** usuario (cualquier rol)  
**Quiero** obtener ayuda instantánea del chatbot  
**Para** resolver mis dudas sin esperar atención humana

**Criterios de Aceptación:**

- **Dado** que estoy en cualquier página de la plataforma
- **Cuando** hago clic en el ícono de chat
- **Entonces** se abre el widget con saludo personalizado y sugerencias contextuales

- **Dado** que pregunto "¿Cómo puedo ver mis certificados?"
- **Cuando** el chatbot procesa mi pregunta
- **Entonces** recibo respuesta útil en menos de 2 segundos con enlace directo a la sección

- **Dado** que mi pregunta no tiene respuesta clara
- **Cuando** el chatbot detecta baja confianza
- **Entonces** me sugiere artículos relacionados o escalamiento a soporte humano

- **Dado** que estoy en la página de checkout con problemas
- **Cuando** abro el chatbot
- **Entonces** las sugerencias iniciales incluyen FAQ de pagos y problemas comunes de checkout

**RF Relacionado:** RF-CHATBOT-001, RF-CHATBOT-002  
**Endpoints:** POST /api/v1/chatbot/sessions, POST /api/v1/chatbot/messages  
**Estimación:** 13 story points

---

### US-061: Escalar a Soporte Humano ⚡

**Como** usuario con problema complejo  
**Quiero** hablar con un agente humano  
**Para** resolver situaciones que el chatbot no puede manejar

**Criterios de Aceptación:**

- **Dado** que el chatbot no ha resuelto mi problema en 3 intentos
- **Cuando** elijo "Hablar con un humano"
- **Entonces** se crea ticket con resumen de la conversación y se me informa tiempo de espera estimado

- **Dado** que mi consulta es sobre disputa de pago
- **Cuando** el chatbot detecta el tema sensible
- **Entonces** ofrece proactivamente escalamiento a equipo de finanzas

- **Dado** que escalo fuera de horario de atención
- **Cuando** confirmo que quiero soporte humano
- **Entonces** puedo elegir recibir respuesta por email o esperar a horario de atención

- **Dado** que un agente toma mi ticket
- **Cuando** comienza la interacción
- **Entonces** el agente ve todo el historial de chat con el bot sin que yo tenga que repetir información

**RF Relacionado:** RF-CHATBOT-003  
**Endpoint:** POST /api/v1/chatbot/escalate  
**Estimación:** 8 story points

---

### US-062: Buscar en Knowledge Base 🎯

**Como** usuario autosuficiente  
**Quiero** buscar respuestas en la base de conocimiento  
**Para** resolver mis dudas sin necesidad de contactar soporte

**Criterios de Aceptación:**

- **Dado** que accedo al centro de ayuda (/support)
- **Cuando** veo la página principal
- **Entonces** encuentro categorías organizadas, artículos populares y barra de búsqueda prominente

- **Dado** que busco "descargar factura"
- **Cuando** escribo en el buscador
- **Entonces** veo resultados relevantes con extractos que destacan mi término de búsqueda

- **Dado** que leo un artículo
- **Cuando** llego al final
- **Entonces** puedo indicar si fue útil y ver artículos relacionados

- **Dado** que el artículo no resuelve mi duda
- **Cuando** marco "No fue útil"
- **Entonces** se me ofrece abrir chat con soporte con contexto del artículo que vi

**RF Relacionado:** RF-KB-001, RF-KB-002, RF-KB-003  
**Endpoints:** GET /api/v1/kb/articles, POST /api/v1/kb/search/semantic  
**Estimación:** 8 story points

---

## Epic 11: Administración Avanzada

### US-070: Dashboard Ejecutivo ⚡

**Como** administrador de la plataforma  
**Quiero** ver métricas consolidadas en un dashboard  
**Para** tomar decisiones informadas sobre el negocio

**Criterios de Aceptación:**

- **Dado** que accedo al panel de administración
- **Cuando** veo el dashboard principal
- **Entonces** veo KPIs de ingresos, usuarios activos, matrículas y completion rate con comparativa al período anterior

- **Dado** que hay alertas pendientes
- **Cuando** reviso el dashboard
- **Entonces** veo badges con contadores de tareas pendientes (cursos por revisar, reembolsos, tickets)

- **Dado** que quiero analizar tendencias
- **Cuando** selecciono rango de fechas personalizado
- **Entonces** todos los gráficos se actualizan para mostrar datos del período seleccionado

- **Dado** que detecto anomalía en métricas
- **Cuando** hago clic en un KPI
- **Entonces** puedo drill-down para ver detalles y origen de la variación

**RF Relacionado:** RF-ADMIN-001  
**Endpoint:** GET /api/v1/admin/dashboard  
**Estimación:** 13 story points

---

### US-071: Gestionar Usuarios como Admin 🔥

**Como** administrador  
**Quiero** gestionar usuarios de la plataforma  
**Para** resolver problemas de cuentas y mantener la integridad del sistema

**Criterios de Aceptación:**

- **Dado** que busco un usuario por email
- **Cuando** encuentro su perfil
- **Entonces** veo información completa: datos, historial de compras, cursos, tickets y notas internas

- **Dado** que un usuario reporta que no puede acceder a curso comprado
- **Cuando** verifico su cuenta
- **Entonces** puedo otorgar acceso manual con registro de la acción y motivo

- **Dado** que detecto cuenta fraudulenta
- **Cuando** decido banear al usuario
- **Entonces** se bloquea el acceso, se cancela suscripción activa y se notifica al usuario con razón genérica

- **Dado** que un usuario olvidó su email de registro
- **Cuando** verifico su identidad por otros medios
- **Entonces** puedo ver/revelar el email y hacer reset de password manualmente

**RF Relacionado:** RF-ADMIN-002  
**Endpoint:** GET/POST /api/v1/admin/users/:id/actions  
**Estimación:** 13 story points

---

### US-072: Moderar Contenido 🔥

**Como** moderador/admin  
**Quiero** revisar y aprobar cursos nuevos  
**Para** asegurar calidad y cumplimiento de políticas

**Criterios de Aceptación:**

- **Dado** que hay cursos pendientes de revisión
- **Cuando** accedo a la cola de moderación
- **Entonces** veo lista priorizada con preview del curso, instructor y tiempo en cola

- **Dado** que reviso un curso
- **Cuando** verifico todos los criterios del checklist
- **Entonces** puedo aprobar, rechazar o solicitar cambios con feedback específico

- **Dado** que un curso viola términos de servicio
- **Cuando** lo rechazo
- **Entonces** el instructor recibe notificación con razones claras y políticas violadas

- **Dado** que hay contenido reportado por usuarios
- **Cuando** reviso el reporte
- **Entonces** puedo ver el contenido original, quién lo reportó, y tomar acción (eliminar, advertir, ignorar)

**RF Relacionado:** RF-ADMIN-003  
**Endpoints:** GET /api/v1/admin/moderation/queue, POST /api/v1/admin/moderation/:id/decision  
**Estimación:** 13 story points

---

### US-073: Gestionar Finanzas ⚡

**Como** administrador de finanzas  
**Quiero** controlar ingresos y pagos a instructores  
**Para** mantener la salud financiera de la plataforma

**Criterios de Aceptación:**

- **Dado** que reviso el panorama financiero mensual
- **Cuando** accedo al módulo de finanzas
- **Entonces** veo ingresos brutos, fees de pago, comisiones, payouts pendientes y neto

- **Dado** que es fecha de pago a instructores
- **Cuando** reviso la cola de payouts
- **Entonces** veo lista de instructores con montos a pagar, método de pago y periodo cubierto

- **Dado** que proceso pagos masivos
- **Cuando** confirmo batch de payouts
- **Entonces** se ejecutan transferencias y se actualiza estado con confirmación o error por cada uno

- **Dado** que necesito reporte para contabilidad
- **Cuando** genero reporte mensual
- **Entonces** obtengo Excel/PDF con todas las transacciones, impuestos retenidos y documentación fiscal

**RF Relacionado:** RF-ADMIN-004  
**Endpoints:** GET /api/v1/admin/finance/overview, POST /api/v1/admin/finance/payouts/process  
**Estimación:** 13 story points

---

### US-074: Auditoría y Seguridad 🔥

**Como** oficial de seguridad/compliance  
**Quiero** auditar acciones administrativas  
**Para** detectar y prevenir mal uso de privilegios

**Criterios de Aceptación:**

- **Dado** que investigo un incidente
- **Cuando** busco en el log de auditoría por usuario y fecha
- **Entonces** veo todas las acciones realizadas con IP, timestamp y cambios específicos

- **Dado** que se detecta patrón sospechoso (múltiples logins fallidos)
- **Cuando** el sistema genera alerta de seguridad
- **Entonces** recibo notificación inmediata con detalles y acciones sugeridas

- **Dado** que un admin accede a datos sensibles
- **Cuando** revisa información de usuario
- **Entonces** la acción se registra automáticamente con motivo requerido para accesos sensitivos

- **Dado** que exporto log de auditoría para auditor externo
- **Cuando** selecciono período y tipo de acciones
- **Entonces** obtengo reporte firmado digitalmente con cadena de custodia verificable

**RF Relacionado:** RF-ADMIN-007  
**Endpoints:** GET /api/v1/admin/audit-log, GET /api/v1/admin/security/alerts  
**Estimación:** 8 story points

---

## Epic 12: Experiencia del Instructor

### US-080: Dashboard de Instructor ⚡

**Como** instructor  
**Quiero** ver mi dashboard personalizado  
**Para** monitorear mis cursos, estudiantes e ingresos

**Criterios de Aceptación:**

- **Dado** que accedo a mi panel de instructor
- **Cuando** veo el dashboard
- **Entonces** veo ganancias del mes, total de estudiantes, rating promedio y tareas pendientes

- **Dado** que tengo preguntas sin responder de estudiantes
- **Cuando** veo alertas en el dashboard
- **Entonces** puedo acceder directamente a las preguntas pendientes con un clic

- **Dado** que quiero ver tendencias de mis cursos
- **Cuando** reviso gráficos de performance
- **Entonces** veo evolución de ventas, matrículas y ratings por curso

- **Dado** que tengo ganancias disponibles
- **Cuando** verifico mi balance
- **Entonces** veo monto retirable, próxima fecha de pago y enlace para configurar método de pago

**RF Relacionado:** RF-INSTRUCTOR-001  
**Endpoint:** GET /api/v1/instructor/dashboard  
**Estimación:** 8 story points

---

### US-081: Crear Quizzes Interactivos ⚡

**Como** instructor  
**Quiero** crear evaluaciones con múltiples tipos de pregunta  
**Para** evaluar el aprendizaje de mis estudiantes de forma efectiva

**Criterios de Aceptación:**

- **Dado** que creo un nuevo quiz
- **Cuando** uso el builder visual
- **Entonces** puedo agregar preguntas multiple choice, verdadero/falso, respuesta corta, essay y código

- **Dado** que configuro una pregunta de código
- **Cuando** defino test cases
- **Entonces** el sistema ejecutará automáticamente el código del estudiante contra mis tests

- **Dado** que quiero aleatorizar el examen
- **Cuando** activo shuffle de preguntas y opciones
- **Entonces** cada estudiante ve un orden diferente para prevenir copia

- **Dado** que publico el quiz
- **Cuando** establezco fecha límite y número de intentos
- **Entonces** los estudiantes pueden acceder según las reglas configuradas

**RF Relacionado:** RF-INSTRUCTOR-002  
**Endpoint:** POST /api/v1/instructor/quizzes  
**Estimación:** 21 story points

---

### US-082: Calificar Trabajos Manuales 🎯

**Como** instructor  
**Quiero** revisar y calificar essays y tareas complejas  
**Para** proporcionar feedback personalizado a mis estudiantes

**Criterios de Aceptación:**

- **Dado** que tengo submissions pendientes
- **Cuando** accedo a mi cola de calificación
- **Entonces** veo lista ordenada por fecha con preview del trabajo y días esperando

- **Dado** que califico un essay
- **Cuando** uso la rúbrica definida
- **Entonces** asigno puntos por criterio con comentarios específicos

- **Dado** que quiero usar asistencia de IA
- **Cuando** solicito sugerencia de calificación
- **Entonces** veo score sugerido y feedback generado que puedo editar antes de publicar

- **Dado** que publico la calificación
- **Cuando** el estudiante recibe notificación
- **Entonces** puede ver su nota, feedback y tiene opción de pedir reconsideración si está habilitado

**RF Relacionado:** RF-INSTRUCTOR-004  
**Endpoints:** GET /api/v1/instructor/grading/pending, POST /api/v1/instructor/grading/:submissionId  
**Estimación:** 13 story points

---

### US-083: Responder Preguntas de Estudiantes 🎯

**Como** instructor  
**Quiero** gestionar preguntas y discusiones de mis cursos  
**Para** mantener engagement y resolver dudas de mis estudiantes

**Criterios de Aceptación:**

- **Dado** que un estudiante publica pregunta en mi curso
- **Cuando** recibo notificación
- **Entonces** puedo responder directamente desde el email o acceder al foro del curso

- **Dado** que respondo una pregunta
- **Cuando** marco mi respuesta como "respuesta del instructor"
- **Entonces** la pregunta se marca como resuelta y otros estudiantes ven la respuesta destacada

- **Dado** que una pregunta es duplicada
- **Cuando** la identifico
- **Entonces** puedo fusionarla con pregunta existente y redirigir al estudiante

- **Dado** que una discusión se vuelve irrelevante o spam
- **Cuando** tomo acción de moderación
- **Entonces** puedo cerrar, eliminar o mover el thread con notificación al autor

**RF Relacionado:** RF-INSTRUCTOR-005  
**Endpoints:** GET /api/v1/courses/:courseId/discussions, POST /api/v1/courses/:courseId/discussions/:threadId/replies  
**Estimación:** 8 story points

---

## Epic 13: Experiencia del Estudiante Avanzada

### US-090: Tomar Notas Durante Videos ⚡

**Como** estudiante  
**Quiero** tomar notas sincronizadas con el video  
**Para** recordar puntos importantes y estudiar después

**Criterios de Aceptación:**

- **Dado** que estoy viendo una lección en video
- **Cuando** presiono tecla de atajo o botón de nota
- **Entonces** se abre panel de notas con timestamp actual pre-llenado

- **Dado** que escribo una nota
- **Cuando** la guardo
- **Entonces** queda asociada al curso, lección y segundo del video

- **Dado** que repaso mis notas después
- **Cuando** hago clic en una nota con timestamp
- **Entonces** el video salta automáticamente a ese momento

- **Dado** que quiero exportar mis notas de un curso
- **Cuando** selecciono exportar en PDF o Markdown
- **Entonces** obtengo documento organizado por lecciones con timestamps

**RF Relacionado:** RF-STUDENT-002  
**Endpoints:** POST /api/v1/students/notes, GET /api/v1/students/notes/export  
**Estimación:** 8 story points

---

### US-091: Guardar Cursos en Wishlist 🎯

**Como** estudiante  
**Quiero** guardar cursos que me interesan para después  
**Para** comprarlos cuando tenga presupuesto o estén en oferta

**Criterios de Aceptación:**

- **Dado** que veo un curso interesante
- **Cuando** hago clic en el ícono de corazón/guardar
- **Entonces** el curso se añade a mi wishlist con confirmación visual

- **Dado** que un curso en mi wishlist baja de precio
- **Cuando** entro a mi wishlist
- **Entonces** veo indicador destacado del descuento con porcentaje de ahorro

- **Dado** que quiero recibir alertas de precio
- **Cuando** activo alertas para un curso en mi wishlist
- **Entonces** recibo email cuando el precio baje o haya promoción

- **Dado** que el curso tiene oferta por tiempo limitado
- **Cuando** veo mi wishlist
- **Entonces** veo contador de tiempo restante para la oferta

**RF Relacionado:** RF-STUDENT-003  
**Endpoints:** GET /api/v1/students/wishlist, POST /api/v1/students/wishlist  
**Estimación:** 5 story points

---

### US-092: Participar en Foros del Curso 🎯

**Como** estudiante  
**Quiero** participar en discusiones con otros estudiantes e instructor  
**Para** resolver dudas y profundizar mi aprendizaje

**Criterios de Aceptación:**

- **Dado** que tengo una duda sobre una lección
- **Cuando** abro el foro del curso
- **Entonces** puedo buscar si la pregunta ya fue respondida antes de publicar

- **Dado** que mi duda no está respondida
- **Cuando** creo nuevo thread
- **Entonces** puedo asociarlo a la lección específica y agregar código/imágenes

- **Dado** que otro estudiante responde mi pregunta
- **Cuando** recibo notificación
- **Entonces** puedo marcar la respuesta como útil o seguir la discusión

- **Dado** que el instructor responde
- **Cuando** veo su respuesta
- **Entonces** está claramente destacada y puedo marcarla como "respuesta aceptada"

**RF Relacionado:** RF-STUDENT-004  
**Endpoints:** POST /api/v1/courses/:courseId/discussions, POST /api/v1/discussions/:threadId/like  
**Estimación:** 8 story points

---

## Epic 14: Suscripciones

### US-095: Suscribirme a Plan Premium 💡

**Como** estudiante frecuente  
**Quiero** suscribirme a un plan de acceso ilimitado  
**Para** acceder a todos los cursos por un precio fijo mensual

**Criterios de Aceptación:**

- **Dado** que exploro opciones de suscripción
- **Cuando** veo la página de planes
- **Entonces** veo comparativa clara de beneficios, precios mensuales y anuales con ahorro destacado

- **Dado** que elijo plan anual
- **Cuando** completo el checkout
- **Entonces** mi tarjeta se cobra y obtengo acceso inmediato a todos los cursos incluidos

- **Dado** que tengo suscripción activa
- **Cuando** accedo a un curso de la biblioteca
- **Entonces** puedo empezar sin proceso de compra adicional

- **Dado** que mi período de prueba está por terminar
- **Cuando** faltan 3 días
- **Entonces** recibo recordatorio con opción de continuar o cancelar antes del primer cobro

**RF Relacionado:** RF-SUB-001  
**Endpoint:** POST /api/v1/subscriptions/subscribe  
**Estimación:** 13 story points

---

### US-096: Gestionar Mi Suscripción 💡

**Como** suscriptor  
**Quiero** gestionar mi plan y facturación  
**Para** mantener control de mi inversión educativa

**Criterios de Aceptación:**

- **Dado** que quiero cambiar de plan mensual a anual
- **Cuando** selecciono el cambio en mi cuenta
- **Entonces** veo el prorrateo del cambio y confirmo el nuevo cargo

- **Dado** que mi tarjeta está por vencer
- **Cuando** actualizo método de pago
- **Entonces** puedo agregar nueva tarjeta y establecerla como default

- **Dado** que quiero ver mis facturas
- **Cuando** accedo al historial de facturación
- **Entonces** puedo descargar PDF de cada factura para mis registros

- **Dado** que decido cancelar mi suscripción
- **Cuando** inicio el proceso de cancelación
- **Entonces** se me ofrece descuento de retención o pausa temporal antes de confirmar

**RF Relacionado:** RF-SUB-002, RF-SUB-003  
**Endpoints:** GET /api/v1/subscriptions/billing, POST /api/v1/subscriptions/cancel  
**Estimación:** 8 story points

---

## Resumen de Backlog Actualizado

### Por Prioridad:

- **🔥 Critical (MVP Blockers):** 14 historias - 107 story points
- **⚡ High Priority:** 14 historias - 154 story points
- **🎯 Medium Priority:** 8 historias - 72 story points
- **💡 Low Priority:** 3 historias - 42 story points

### Por Epic:

1. **Autenticación:** 3 historias - 13 points
2. **Catálogo/Cursos:** 4 historias - 42 points
3. **Comercio:** 2 historias - 21 points
4. **Aprendizaje:** 2 historias - 21 points
5. **Evaluaciones:** 2 historias - 34 points
6. **IA:** 2 historias - 34 points
7. **Business Intelligence:** 3 historias - 47 points
8. **Infraestructura:** 2 historias - 21 points
9. **Compliance/Privacidad:** 4 historias - 29 points
10. **Chatbot/Soporte:** 3 historias - 29 points
11. **Admin Avanzado:** 5 historias - 60 points
12. **Instructor Avanzado:** 4 historias - 50 points
13. **Estudiante Avanzado:** 3 historias - 21 points
14. **Suscripciones:** 2 historias - 21 points

### Roadmap Actualizado:

**Sprint 1-2:** US-001, US-002, US-016, US-019, US-050, US-053 (Fundación + Compliance base)
**Sprint 3-4:** US-004, US-005, US-008, US-009, US-060 (Comercio + Chatbot)
**Sprint 5-6:** US-006, US-007, US-010, US-011, US-071, US-072 (Contenido + Admin)
**Sprint 7-8:** US-012, US-014, US-017, US-018, US-080, US-081 (IA + Instructor)
**Sprint 9-10:** US-051, US-052, US-061, US-062, US-082, US-083 (Compliance avanzado + Features)
**Sprint 11-12:** US-070, US-073, US-074, US-090, US-091, US-092, US-095, US-096 (Admin/Student/Subs)

**Total estimado:** 443 story points (≈ 12-15 sprints para equipo de 5 developers)

**Cobertura de nuevos requisitos:**

| Área          | RFs Cubiertos                       | User Stories |
| ------------- | ----------------------------------- | ------------ |
| Compliance    | RF-COMPLIANCE-001..019              | US-050..053  |
| Chatbot/KB    | RF-CHATBOT-001..004, RF-KB-001..003 | US-060..062  |
| Admin         | RF-ADMIN-001..007                   | US-070..074  |
| Instructor    | RF-INSTRUCTOR-001..007              | US-080..083  |
| Estudiante    | RF-STUDENT-001..005                 | US-090..092  |
| Suscripciones | RF-SUB-001..003                     | US-095..096  |
