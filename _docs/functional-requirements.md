# ACC LMS — Requisitos Funcionales Completos por Rol

**Versión:** 2025-12-14  
**Estado:** Documento maestro de requisitos funcionales  
**Alcance:** Todos los roles del sistema  
**Nota:** Los criterios de aceptación se definen en [user-stories.md](user-stories.md)

---

## Índice

1. [Actores del Sistema](#actores-del-sistema)
2. [Requisitos por Rol: Anonymous](#1-anonymous-visitante-no-autenticado)
3. [Requisitos por Rol: Student](#2-student-estudiante)
4. [Requisitos por Rol: Instructor](#3-instructor-creador-de-contenido)
5. [Requisitos por Rol: Admin](#4-admin-administrador)
6. [Requisitos por Rol: System](#5-system-procesos-automáticos)
7. [Requisitos Transversales](#6-requisitos-transversales)
8. [Matriz de Permisos](#7-matriz-de-permisos)

---

## Actores del Sistema

| Rol            | Descripción                             | Contexto de uso                                    |
| -------------- | --------------------------------------- | -------------------------------------------------- |
| **Anonymous**  | Visitante sin autenticación             | Exploración, registro, información pública         |
| **Student**    | Usuario autenticado como estudiante     | Consumo de cursos, evaluaciones, progreso personal |
| **Instructor** | Creador y gestor de contenido educativo | Creación de cursos, gestión de estudiantes, ventas |
| **Admin**      | Administrador de la plataforma          | Gestión global, auditoría, configuración           |
| **System**     | Procesos automáticos y jobs programados | Webhooks, eventos, tareas en background            |

---

## 1. Anonymous (Visitante No Autenticado)

### 1.1 Navegación y Exploración

#### RF-ANON-001: Acceso al catálogo público de cursos

El sistema debe permitir a los visitantes anónimos explorar el catálogo completo de cursos publicados.

- Visualización de listado de cursos con paginación
- Información visible: título, descripción corta, imagen de portada, instructor, precio, rating promedio
- Sin acceso a contenido de lecciones ni materiales

#### RF-ANON-002: Búsqueda y filtrado de cursos

El sistema debe proporcionar capacidades de búsqueda y filtrado sin requerir autenticación.

- Búsqueda por texto en título y descripción
- Filtros por categoría/tags
- Filtros por rango de precio (gratis, de pago, rango específico)
- Filtros por idioma del curso
- Ordenamiento por: más recientes, mejor valorados, más vendidos, precio

#### RF-ANON-003: Vista de detalle de curso público

El sistema debe mostrar información detallada de cualquier curso publicado.

- Descripción completa del curso
- Temario/índice de lecciones (sin acceso al contenido)
- Información del instructor (nombre, bio, foto, cursos)
- Reseñas y valoraciones de estudiantes
- Requisitos previos y objetivos de aprendizaje
- Precio y opciones de compra
- Duración estimada y número de lecciones

#### RF-ANON-004: Vista de perfil público de instructor

El sistema debe permitir ver perfiles públicos de instructores.

- Nombre, foto, biografía
- Cursos publicados por el instructor
- Estadísticas públicas: número de estudiantes, valoración promedio
- Enlaces a redes sociales (si el instructor los configuró)

### 1.2 Registro y Autenticación

#### RF-ANON-005: Registro de nueva cuenta

El sistema debe permitir la creación de cuentas nuevas.

- Campos requeridos: email, contraseña, nombre, apellido
- Aceptación obligatoria de términos y condiciones
- Validación de formato de email
- Validación de fortaleza de contraseña (mínimo 10 caracteres, mayúscula, minúscula, número, símbolo)
- Verificación de email único en el sistema
- Rol por defecto: Student

#### RF-ANON-006: Inicio de sesión

El sistema debe autenticar usuarios registrados.

- Login con email y contraseña
- Opción "Recordarme" para sesión extendida
- Bloqueo temporal tras 5 intentos fallidos consecutivos (15 minutos)
- Mensaje de error genérico para evitar enumeración de usuarios

#### RF-ANON-007: Recuperación de contraseña

El sistema debe permitir restablecer contraseñas olvidadas.

- Solicitud de reset mediante email
- Envío de enlace temporal de restablecimiento (válido por 1 hora)
- Obligación de crear nueva contraseña que cumpla requisitos de seguridad
- Invalidación de tokens anteriores al generar uno nuevo

### 1.3 Información Institucional

#### RF-ANON-008: Acceso a páginas institucionales

El sistema debe proporcionar páginas de información pública.

- Página "Acerca de" con información de la plataforma
- Términos y condiciones
- Política de privacidad
- Política de reembolsos
- Página de contacto/soporte
- FAQ (preguntas frecuentes)

---

## 2. Student (Estudiante)

### 2.1 Gestión de Cuenta y Perfil

#### RF-STU-001: Visualización y edición de perfil personal

El sistema debe permitir a los estudiantes gestionar su información personal.

- Visualización de datos actuales del perfil
- Edición de: nombre, apellido, foto de perfil, biografía
- Edición de enlaces a redes sociales
- Cambio de contraseña (requiere contraseña actual)
- Cambio de email (requiere verificación del nuevo email)

#### RF-STU-002: Gestión de preferencias

El sistema debe permitir configurar preferencias personales.

- Idioma de la interfaz (ES, EN, PT)
- Zona horaria
- Preferencias de notificaciones por email:
  - Actualizaciones de cursos matriculados
  - Mensajes de instructores
  - Promociones y ofertas
  - Resumen semanal de progreso
- Preferencias de privacidad:
  - Visibilidad del perfil público
  - Permitir mensajes de otros usuarios

#### RF-STU-003: Gestión de sesiones activas

El sistema debe permitir controlar las sesiones de acceso.

- Visualización de dispositivos/sesiones activas
- Cierre de sesiones individuales
- Cierre de todas las sesiones excepto la actual

### 2.2 Descubrimiento y Compra de Cursos

#### RF-STU-004: Búsqueda avanzada de cursos

El sistema debe proporcionar búsqueda mejorada para usuarios autenticados.

- Todas las capacidades de búsqueda de anonymous
- Recomendaciones personalizadas basadas en historial
- Filtro "Excluir cursos ya adquiridos"
- Guardado de búsquedas frecuentes

#### RF-STU-005: Lista de deseos (Wishlist)

El sistema debe permitir guardar cursos de interés.

- Agregar/quitar cursos a lista de deseos
- Visualización de lista de deseos en perfil
- Notificaciones opcionales de descuentos en cursos guardados

#### RF-STU-006: Proceso de compra de curso

El sistema debe facilitar la adquisición de cursos.

- Visualización de precio y detalles antes de comprar
- Selección de método de pago (Stripe, MercadoPago)
- Aplicación de cupones de descuento (si existen)
- Redirección segura a pasarela de pago
- Confirmación de compra exitosa
- Generación automática de factura/recibo

#### RF-STU-007: Historial de compras y facturación

El sistema debe mantener registro de transacciones.

- Listado de todas las compras realizadas
- Descarga de facturas/recibos en PDF
- Estado de cada transacción (pagado, pendiente, reembolsado)
- Detalle de método de pago utilizado (últimos 4 dígitos)

### 2.3 Consumo de Contenido Educativo

#### RF-STU-008: Acceso a cursos matriculados

El sistema debe proporcionar acceso al contenido de cursos adquiridos.

- Dashboard con listado de cursos matriculados
- Indicador de progreso por curso
- Acceso rápido a última lección vista
- Ordenamiento por: recientes, progreso, fecha de compra

#### RF-STU-009: Reproducción de contenido de lecciones

El sistema debe permitir consumir diferentes tipos de contenido.

- **Video:** Reproductor con controles (play, pause, velocidad, calidad, subtítulos, pantalla completa)
- **Artículo/Texto:** Visualización formateada con soporte Markdown
- **Quiz:** Interfaz interactiva para responder evaluaciones
- **Recursos descargables:** Descarga de materiales adjuntos (PDFs, código, etc.)
- Marcado de posición en videos para continuar después

#### RF-STU-010: Registro y visualización de progreso

El sistema debe trackear el avance del estudiante.

- Marcado automático de lección como completada
- Barra de progreso visual por curso
- Porcentaje de completitud del curso
- Historial de lecciones vistas con fechas
- Estimación de tiempo restante para completar

#### RF-STU-011: Navegación entre lecciones

El sistema debe facilitar la navegación del contenido.

- Listado lateral de todas las lecciones del curso
- Indicador visual de lecciones completadas vs pendientes
- Navegación "Siguiente" / "Anterior" entre lecciones
- Acceso directo a cualquier lección del temario

### 2.4 Evaluaciones y Calificaciones

#### RF-STU-012: Realización de quizzes

El sistema debe permitir completar evaluaciones.

- Visualización de preguntas según tipo (opción única, opción múltiple, verdadero/falso, respuesta corta)
- Navegación entre preguntas
- Guardado automático de respuestas parciales
- Envío de quiz para calificación
- Límite de intentos configurable por quiz

#### RF-STU-013: Visualización de resultados

El sistema debe mostrar resultados de evaluaciones.

- Puntuación obtenida vs puntuación máxima
- Porcentaje de acierto
- Retroalimentación por pregunta (si está habilitada)
- Respuestas correctas (si está configurado mostrarlas)
- Historial de intentos anteriores

#### RF-STU-014: Consulta de calificaciones globales

El sistema debe mostrar el rendimiento general.

- Listado de todas las calificaciones por curso
- Promedio general de calificaciones
- Calificaciones pendientes de revisión (para ensayos/código)
- Exportación de transcript/historial académico

### 2.5 Interacción y Comunicación

#### RF-STU-015: Sistema de valoraciones y reseñas

El sistema debe permitir evaluar cursos completados.

- Calificación con estrellas (1-5)
- Comentario/reseña de texto
- Edición de reseña propia
- Visualización de reseñas de otros estudiantes

#### RF-STU-016: Preguntas y respuestas por lección

El sistema debe facilitar la comunicación con instructores.

- Realizar preguntas en contexto de lección específica
- Visualización de preguntas y respuestas de otros estudiantes
- Notificación cuando el instructor responde
- Marcado de respuesta como "útil"

#### RF-STU-017: Mensajería con instructores

El sistema debe permitir comunicación directa.

- Envío de mensajes privados a instructores de cursos matriculados
- Historial de conversaciones
- Notificaciones de nuevos mensajes

### 2.6 Funcionalidades de IA

#### RF-STU-018: Tutor IA conversacional

El sistema debe proporcionar asistencia inteligente por curso.

- Chat con tutor IA contextualizado al contenido del curso
- Respuestas basadas en el material del curso (RAG)
- Historial de conversaciones con el tutor
- Límite de uso según plan/configuración

#### RF-STU-019: Búsqueda semántica de contenido

El sistema debe permitir búsqueda inteligente.

- Búsqueda por significado (no solo palabras exactas)
- Búsqueda dentro del contenido de cursos matriculados
- Resultados relevantes de lecciones, transcripciones, materiales

#### RF-STU-020: Resúmenes generados por IA

El sistema debe generar resúmenes automáticos.

- Resumen de lección/video disponible bajo demanda
- Puntos clave extraídos del contenido
- Glosario de términos importantes

### 2.7 Certificaciones

#### RF-STU-021: Obtención de certificados

El sistema debe emitir certificados de completitud.

- Generación automática al completar 100% del curso
- Certificado con: nombre del estudiante, curso, fecha, instructor
- Descarga en formato PDF
- Enlace verificable público para validación

---

## 3. Instructor (Creador de Contenido)

### 3.1 Gestión de Perfil de Instructor

#### RF-INS-001: Perfil profesional de instructor

El sistema debe permitir configurar un perfil profesional público.

- Biografía extendida (formación, experiencia)
- Foto profesional
- Enlaces a redes sociales y sitio web
- Áreas de especialización/experticia
- Video de presentación opcional

#### RF-INS-002: Configuración de pagos

El sistema debe permitir configurar la recepción de ingresos.

- Conexión con cuenta de Stripe/MercadoPago para recibir pagos
- Configuración de datos fiscales/facturación
- Visualización de porcentaje de comisión de la plataforma

### 3.2 Creación y Gestión de Cursos

#### RF-INS-003: Creación de nuevo curso

El sistema debe permitir crear cursos desde cero.

- Título del curso (único por instructor)
- Descripción corta (para listados) y descripción larga (detalle)
- Imagen de portada
- Video promocional opcional
- Categoría principal y tags
- Idioma del curso
- Nivel de dificultad (principiante, intermedio, avanzado)
- Requisitos previos
- Objetivos de aprendizaje
- Precio (o gratuito)
- Moneda

#### RF-INS-004: Gestión de lecciones

El sistema debe permitir estructurar el contenido del curso.

- Crear, editar, eliminar lecciones
- Tipos de lección: video, artículo, quiz
- Ordenamiento de lecciones mediante drag & drop
- Agrupación en secciones/módulos
- Lecciones de vista previa (gratis para todos)
- Duración estimada por lección

#### RF-INS-005: Subida y gestión de contenido multimedia

El sistema debe facilitar la gestión de archivos.

- Subida de videos (formatos MP4, WebM)
- Subida de recursos descargables (PDF, ZIP, código)
- Procesamiento automático de video (transcodificación, calidades)
- Generación automática de transcripciones (si está habilitado)
- Límites de almacenamiento según plan
- Visualización de espacio utilizado

#### RF-INS-006: Creación de quizzes

El sistema debe permitir crear evaluaciones.

- Tipos de pregunta: opción única, opción múltiple, verdadero/falso, respuesta corta, código
- Configuración de respuestas correctas
- Puntuación por pregunta
- Número de intentos permitidos
- Tiempo límite opcional
- Retroalimentación por respuesta (correcta/incorrecta)
- Mostrar/ocultar respuestas correctas después del intento

#### RF-INS-007: Vista previa de curso

El sistema debe permitir previsualizar el curso como estudiante.

- Modo "vista de estudiante" para revisar contenido
- Verificación de funcionamiento de videos y quizzes
- Validación de navegación y progreso

#### RF-INS-008: Publicación y despublicación de cursos

El sistema debe controlar la visibilidad de cursos.

- Validación de requisitos mínimos antes de publicar (título, descripción, al menos 1 lección)
- Publicación hace el curso visible en catálogo
- Despublicación oculta el curso (estudiantes existentes mantienen acceso)
- Cursos en borrador solo visibles para el instructor

#### RF-INS-009: Edición de cursos publicados

El sistema debe permitir actualizar cursos existentes.

- Edición de información general (descripción, precio, imagen)
- Agregar nuevas lecciones
- Editar contenido de lecciones existentes
- Restricciones: no cambiar slug si hay ventas activas

### 3.3 Gestión de Estudiantes

#### RF-INS-010: Listado de estudiantes matriculados

El sistema debe mostrar los estudiantes de cada curso.

- Lista de estudiantes por curso
- Información: nombre, email, fecha de matrícula, progreso
- Filtros por progreso (completados, en curso, sin iniciar)
- Exportación de lista en CSV

#### RF-INS-011: Comunicación con estudiantes

El sistema debe facilitar la comunicación.

- Envío de anuncios a todos los estudiantes de un curso
- Respuesta a preguntas en secciones de Q&A
- Mensajería directa con estudiantes individuales

#### RF-INS-012: Gestión de accesos manuales

El sistema debe permitir otorgar acceso sin compra.

- Agregar estudiante manualmente por email (acceso gratuito)
- Revocar acceso de estudiante específico
- Registro de motivo de acceso manual (cortesía, promoción, etc.)

### 3.4 Evaluación y Calificaciones

#### RF-INS-013: Revisión de envíos de estudiantes

El sistema debe permitir evaluar trabajos.

- Cola de envíos pendientes de revisión (ensayos, código)
- Visualización de respuesta del estudiante
- Asignación de puntuación manual
- Redacción de feedback/comentarios
- Aprobación/rechazo de envíos

#### RF-INS-014: Estadísticas de evaluaciones

El sistema debe mostrar métricas de quizzes.

- Porcentaje de aprobación por quiz
- Preguntas con mayor índice de error
- Distribución de calificaciones
- Tiempo promedio de completado

### 3.5 Analytics y Reportes

#### RF-INS-015: Dashboard de instructor

El sistema debe proporcionar una vista general de métricas.

- Ingresos totales y por período
- Número total de estudiantes
- Cursos activos y en borrador
- Ventas recientes
- Rating promedio global

#### RF-INS-016: Analytics por curso

El sistema debe mostrar métricas detalladas por curso.

- Ventas totales e ingresos del curso
- Número de estudiantes activos
- Tasa de completitud promedio
- Rating y número de reseñas
- Engagement: lecciones más/menos vistas
- Puntos de abandono (lecciones donde estudiantes dejan el curso)

#### RF-INS-017: Reportes de ingresos

El sistema debe generar reportes financieros.

- Ingresos por período (diario, semanal, mensual)
- Desglose por curso
- Comisiones de plataforma
- Reembolsos realizados
- Exportación en CSV/PDF
- Proyección de pagos pendientes

### 3.6 Herramientas de IA para Instructores

#### RF-INS-018: Generación de quizzes asistida por IA

El sistema debe facilitar la creación de evaluaciones.

- Generación de preguntas basadas en contenido de lección
- Sugerencia de opciones incorrectas (distractores)
- Edición de preguntas generadas antes de publicar
- Configuración de dificultad y cantidad

#### RF-INS-019: Generación de resúmenes y materiales

El sistema debe asistir en la creación de contenido.

- Generación de resúmenes de lecciones
- Creación de glosarios automáticos
- Sugerencia de objetivos de aprendizaje
- Generación de descripción de curso

#### RF-INS-020: Transcripción automática de videos

El sistema debe generar transcripciones.

- Transcripción automática al subir video
- Edición manual de transcripciones
- Uso de transcripciones para búsqueda y IA

---

## 4. Admin (Administrador)

### 4.1 Gestión de Usuarios

#### RF-ADM-001: Listado y búsqueda de usuarios

El sistema debe permitir gestionar todos los usuarios.

- Listado completo de usuarios con paginación
- Búsqueda por email, nombre, ID
- Filtros por rol, estado de cuenta, fecha de registro
- Exportación de listados

#### RF-ADM-002: Gestión de cuentas de usuario

El sistema debe permitir modificar cuentas.

- Visualización de perfil completo de cualquier usuario
- Edición de información de usuario
- Cambio de rol (student, instructor, admin)
- Activación/desactivación de cuentas
- Reseteo de contraseña
- Verificación manual de email

#### RF-ADM-003: Gestión de sesiones y seguridad

El sistema debe controlar accesos.

- Visualización de sesiones activas de cualquier usuario
- Forzar cierre de sesiones
- Desbloqueo de cuentas bloqueadas
- Historial de intentos de login fallidos

### 4.2 Gestión de Cursos

#### RF-ADM-004: Supervisión del catálogo

El sistema debe permitir supervisar todos los cursos.

- Listado de todos los cursos (publicados y borradores)
- Filtros por instructor, estado, categoría, fecha
- Búsqueda por título, descripción

#### RF-ADM-005: Moderación de contenido

El sistema debe permitir moderar cursos.

- Revisión de cursos antes de publicación (si está habilitado)
- Despublicación de cursos que violen políticas
- Notificación al instructor con motivo
- Eliminación de cursos (soft delete)

#### RF-ADM-006: Gestión de categorías y tags

El sistema debe permitir organizar el catálogo.

- CRUD de categorías de cursos
- CRUD de tags/etiquetas
- Asignación de categorías a cursos
- Ordenamiento de categorías

### 4.3 Gestión de Transacciones

#### RF-ADM-007: Supervisión de pagos

El sistema debe permitir revisar transacciones.

- Listado de todas las órdenes de compra
- Filtros por estado, fecha, usuario, curso, monto
- Detalle completo de cada transacción
- Historial de estados de la orden

#### RF-ADM-008: Procesamiento de reembolsos

El sistema debe permitir gestionar reembolsos.

- Visualización de solicitudes de reembolso
- Aprobación/rechazo de reembolsos
- Procesamiento de reembolso con pasarela de pago
- Notificación automática al usuario
- Registro de motivo de reembolso

#### RF-ADM-009: Reportes financieros globales

El sistema debe generar reportes de toda la plataforma.

- Ingresos totales por período
- Desglose por instructor y curso
- Comisiones generadas
- Reembolsos procesados
- Métodos de pago utilizados
- Exportación en múltiples formatos

### 4.4 Moderación de Contenido

#### RF-ADM-010: Gestión de reseñas

El sistema debe permitir moderar reseñas.

- Listado de reseñas con filtros
- Reportes de reseñas inapropiadas
- Eliminación de reseñas que violen políticas
- Notificación al autor

#### RF-ADM-011: Gestión de mensajes y Q&A

El sistema debe permitir moderar comunicaciones.

- Visualización de mensajes reportados
- Eliminación de contenido inapropiado
- Suspensión de usuarios por comportamiento
- Configuración de palabras/frases bloqueadas

### 4.5 Configuración de Plataforma

#### RF-ADM-012: Configuración general

El sistema debe permitir configurar parámetros globales.

- Nombre y branding de la plataforma
- Idiomas habilitados
- Monedas aceptadas
- Porcentaje de comisión por venta
- Límites de almacenamiento por rol

#### RF-ADM-013: Gestión de pasarelas de pago

El sistema debe permitir configurar integraciones de pago.

- Configuración de credenciales de Stripe
- Configuración de credenciales de MercadoPago
- Habilitación/deshabilitación de métodos de pago
- Modo sandbox/producción

#### RF-ADM-014: Configuración de email

El sistema debe permitir configurar notificaciones.

- Configuración de proveedor SMTP
- Edición de plantillas de email
- Configuración de remitente
- Prueba de envío de emails

#### RF-ADM-015: Feature flags

El sistema debe permitir habilitar/deshabilitar funcionalidades.

- Toggle de funcionalidades específicas
- Configuración de funcionalidades por rol
- Habilitación gradual (porcentaje de usuarios)

### 4.6 Analytics y Monitoreo

#### RF-ADM-016: Dashboard ejecutivo

El sistema debe proporcionar vista de alto nivel.

- KPIs principales: usuarios, cursos, ingresos, matrículas
- Gráficos de tendencias
- Comparativas con períodos anteriores
- Alertas de métricas fuera de rango

#### RF-ADM-017: Analytics de negocio

El sistema debe proporcionar métricas detalladas.

- Funnel de conversión (visita → registro → compra)
- Tasa de retención de usuarios
- Cursos más vendidos
- Instructores top
- Análisis de cohortes

#### RF-ADM-018: Logs de auditoría

El sistema debe registrar acciones administrativas.

- Registro de todas las acciones de admins
- Quién, qué, cuándo, desde dónde
- Filtros por tipo de acción, usuario, fecha
- Exportación de logs
- Retención configurable

#### RF-ADM-019: Monitoreo de sistema

El sistema debe mostrar estado de infraestructura.

- Estado de servicios (health checks)
- Métricas de rendimiento (latencia, errores)
- Uso de recursos (CPU, memoria, almacenamiento)
- Alertas de incidentes

### 4.7 Soporte y Atención

#### RF-ADM-020: Gestión de tickets de soporte

El sistema debe permitir atender consultas.

- Listado de tickets/consultas de usuarios
- Asignación a agentes de soporte
- Cambio de estado (abierto, en progreso, resuelto)
- Historial de comunicaciones
- Métricas de tiempo de respuesta

#### RF-ADM-021: Acciones en nombre de usuario

El sistema debe permitir actuar como usuario.

- Impersonación de usuario para debugging
- Registro de cada impersonación en audit log
- Restricciones de acciones durante impersonación

---

## 5. System (Procesos Automáticos)

### 5.1 Procesamiento de Eventos

#### RF-SYS-001: Procesamiento de webhooks de pago

El sistema debe procesar notificaciones de pasarelas de pago.

- Recepción de webhooks de Stripe
- Recepción de webhooks de MercadoPago
- Validación de firma/autenticidad
- Procesamiento idempotente (evitar duplicados)
- Actualización de estado de órdenes
- Emisión de eventos de dominio (order.paid, order.failed)

#### RF-SYS-002: Creación automática de matrículas

El sistema debe crear matrículas tras pagos exitosos.

- Trigger: evento order.paid
- Creación de enrollment con estado active
- Idempotencia por orderId
- Notificación al estudiante

#### RF-SYS-003: Procesamiento de eventos de dominio

El sistema debe propagar eventos entre servicios.

- Publicación de eventos en cola de mensajes
- Consumo por servicios interesados
- Reintentos en caso de fallo
- Dead letter queue para eventos no procesables

### 5.2 Notificaciones Automáticas

#### RF-SYS-004: Envío de emails transaccionales

El sistema debe enviar notificaciones por email.

- Email de bienvenida al registrarse
- Confirmación de compra/recibo
- Confirmación de matrícula
- Notificación de nueva lección disponible
- Recordatorios de curso sin completar
- Resumen semanal de progreso

#### RF-SYS-005: Gestión de cola de notificaciones

El sistema debe gestionar envíos de forma eficiente.

- Cola de envíos pendientes
- Reintentos con backoff exponencial
- Tracking de entregas (enviado, abierto, clickeado)
- Manejo de bounces y unsubscribes

### 5.3 Procesamiento de Contenido

#### RF-SYS-006: Transcodificación de video

El sistema debe procesar videos subidos.

- Conversión a múltiples calidades (360p, 720p, 1080p)
- Generación de thumbnails
- Formato de salida optimizado (HLS/DASH)
- Notificación al instructor cuando está listo

#### RF-SYS-007: Generación de transcripciones

El sistema debe generar transcripciones automáticas.

- Procesamiento de audio de videos
- Generación de texto con timestamps
- Disponibilidad para edición por instructor
- Indexación para búsqueda

#### RF-SYS-008: Indexación de contenido

El sistema debe mantener índices de búsqueda actualizados.

- Indexación de cursos nuevos/actualizados
- Indexación de lecciones y transcripciones
- Generación de embeddings para búsqueda semántica
- Actualización incremental

### 5.4 Mantenimiento y Limpieza

#### RF-SYS-009: Limpieza de tokens expirados

El sistema debe mantener la base de datos limpia.

- Eliminación de refresh tokens expirados
- Limpieza de tokens de reset de password
- Limpieza de sesiones inactivas

#### RF-SYS-010: Limpieza de archivos temporales

El sistema debe gestionar almacenamiento.

- Eliminación de uploads abandonados
- Limpieza de archivos de procesamiento temporal
- Reportes de uso de almacenamiento

#### RF-SYS-011: Generación de reportes programados

El sistema debe generar reportes automáticos.

- Reportes diarios de ventas
- Reportes semanales de actividad
- Reportes mensuales para instructores
- Envío automático por email

### 5.5 Monitoreo y Alertas

#### RF-SYS-012: Health checks

El sistema debe verificar su propio estado.

- Verificación periódica de cada servicio
- Verificación de conexiones a bases de datos
- Verificación de servicios externos
- Reporte de estado a sistema de monitoreo

#### RF-SYS-013: Alertas automáticas

El sistema debe notificar problemas.

- Alertas por errores en webhooks de pago
- Alertas por tasa de errores elevada
- Alertas por servicios caídos
- Alertas por uso anormal de recursos
- Notificación a canales configurados (email, Slack)

---

## 6. Requisitos Transversales

### 6.1 Seguridad

#### RF-GLOBAL-001: Autenticación JWT

- Tokens de acceso con TTL de 15 minutos
- Tokens de refresco con TTL de 7 días (30 días con "recordarme")
- Rotación obligatoria de refresh tokens
- Blacklist de tokens revocados

#### RF-GLOBAL-002: Autorización RBAC

- Verificación de rol en cada request
- Verificación de ownership para recursos propios
- Permisos granulares por endpoint
- Audit trail de accesos denegados

#### RF-GLOBAL-003: Protección de datos

- Hashing de contraseñas con Argon2id
- Encriptación de datos sensibles en reposo
- HTTPS obligatorio en producción
- Sanitización de inputs

#### RF-GLOBAL-004: Rate limiting

- Límites por IP para endpoints públicos
- Límites por usuario para endpoints autenticados
- Límites diferenciados por rol
- Headers de respuesta con límites restantes

### 6.2 Observabilidad

#### RF-GLOBAL-005: Logging estructurado

- Formato JSON con campos estándar
- Correlation ID en todas las operaciones
- Niveles: debug, info, warn, error
- Contexto de usuario y request

#### RF-GLOBAL-006: Métricas

- Latencia de requests (histograma)
- Tasa de errores (counter)
- Conexiones activas (gauge)
- Métricas de negocio (matrículas, ventas)

#### RF-GLOBAL-007: Distributed tracing

- Propagación de trace context W3C
- Spans por operación significativa
- Integración con Jaeger/Zipkin

### 6.3 API Standards

#### RF-GLOBAL-008: Formato de respuestas

- JSON con camelCase para propiedades
- Estructura consistente de errores
- Paginación estándar (page, pageSize, total)
- Versionado en URL (/api/v1/)

#### RF-GLOBAL-009: Documentación de API

- OpenAPI/Swagger para cada servicio
- Ejemplos de request/response
- Descripción de errores posibles
- Playground interactivo

### 6.4 Internacionalización

#### RF-GLOBAL-010: Soporte multi-idioma

- Idiomas: ES, EN, PT
- Textos en frontend, no en backend
- Formateo de fechas/números según locale
- Header Accept-Language respetado

---

## 7. Matriz de Permisos

### Recursos y Operaciones por Rol

| Recurso / Operación                | Anonymous | Student | Instructor  | Admin |
| ---------------------------------- | --------- | ------- | ----------- | ----- |
| **Cursos**                         |           |         |             |       |
| Ver catálogo público               | ✅        | ✅      | ✅          | ✅    |
| Ver detalle de curso público       | ✅        | ✅      | ✅          | ✅    |
| Crear curso                        | ❌        | ❌      | ✅          | ✅    |
| Editar curso propio                | ❌        | ❌      | ✅          | ✅    |
| Editar cualquier curso             | ❌        | ❌      | ❌          | ✅    |
| Publicar/despublicar curso propio  | ❌        | ❌      | ✅          | ✅    |
| Eliminar curso                     | ❌        | ❌      | ✅ (propio) | ✅    |
| **Lecciones**                      |           |         |             |       |
| Ver lección de curso matriculado   | ❌        | ✅      | ✅          | ✅    |
| Crear/editar lecciones propias     | ❌        | ❌      | ✅          | ✅    |
| **Matrículas**                     |           |         |             |       |
| Ver matrículas propias             | ❌        | ✅      | ✅          | ✅    |
| Ver matrículas de curso propio     | ❌        | ❌      | ✅          | ✅    |
| Ver todas las matrículas           | ❌        | ❌      | ❌          | ✅    |
| Crear matrícula manual             | ❌        | ❌      | ✅          | ✅    |
| **Usuarios**                       |           |         |             |       |
| Registrarse                        | ✅        | ❌      | ❌          | ❌    |
| Ver perfil propio                  | ❌        | ✅      | ✅          | ✅    |
| Editar perfil propio               | ❌        | ✅      | ✅          | ✅    |
| Ver cualquier perfil               | ❌        | ❌      | ❌          | ✅    |
| Editar cualquier perfil            | ❌        | ❌      | ❌          | ✅    |
| Cambiar roles                      | ❌        | ❌      | ❌          | ✅    |
| **Pagos**                          |           |         |             |       |
| Realizar compra                    | ❌        | ✅      | ✅          | ✅    |
| Ver compras propias                | ❌        | ✅      | ✅          | ✅    |
| Ver ventas de cursos propios       | ❌        | ❌      | ✅          | ✅    |
| Ver todas las transacciones        | ❌        | ❌      | ❌          | ✅    |
| Procesar reembolsos                | ❌        | ❌      | ❌          | ✅    |
| **Evaluaciones**                   |           |         |             |       |
| Realizar quiz de curso matriculado | ❌        | ✅      | ✅          | ✅    |
| Ver calificaciones propias         | ❌        | ✅      | ✅          | ✅    |
| Crear quizzes en curso propio      | ❌        | ❌      | ✅          | ✅    |
| Calificar envíos de curso propio   | ❌        | ❌      | ✅          | ✅    |
| **Analytics**                      |           |         |             |       |
| Ver analytics de cursos propios    | ❌        | ❌      | ✅          | ✅    |
| Ver analytics globales             | ❌        | ❌      | ❌          | ✅    |
| Ver audit logs                     | ❌        | ❌      | ❌          | ✅    |
| **IA**                             |           |         |             |       |
| Usar tutor IA (curso matriculado)  | ❌        | ✅      | ✅          | ✅    |
| Búsqueda semántica                 | ❌        | ✅      | ✅          | ✅    |
| Generar quizzes con IA             | ❌        | ❌      | ✅          | ✅    |
| **Configuración**                  |           |         |             |       |
| Configuración de plataforma        | ❌        | ❌      | ❌          | ✅    |
| Gestión de categorías              | ❌        | ❌      | ❌          | ✅    |
| Feature flags                      | ❌        | ❌      | ❌          | ✅    |

---

## Trazabilidad

### Mapeo RF → Servicios

| Prefijo RF | Servicio             | Descripción                     |
| ---------- | -------------------- | ------------------------------- |
| RF-ANON    | auth-service, fe     | Funcionalidades públicas        |
| RF-STU     | Múltiples            | Funcionalidades de estudiante   |
| RF-INS     | courses, content, fe | Funcionalidades de instructor   |
| RF-ADM     | Todos                | Funcionalidades administrativas |
| RF-SYS     | Todos                | Procesos automáticos            |
| RF-GLOBAL  | Transversal          | Aplica a todos los servicios    |

### Mapeo RF → User Stories

Los criterios de aceptación detallados para cada requisito funcional se encuentran documentados en [user-stories.md](user-stories.md), siguiendo el formato:

```
Como [ROL]
Quiero [FUNCIONALIDAD del RF]
Para [BENEFICIO]

Criterios de Aceptación:
- Dado [CONTEXTO]
- Cuando [ACCIÓN]
- Entonces [RESULTADO]
```

---

**Total de requisitos funcionales:** 85+  
**Roles cubiertos:** 5 (Anonymous, Student, Instructor, Admin, System)  
**Fecha de última actualización:** 2025-12-14
