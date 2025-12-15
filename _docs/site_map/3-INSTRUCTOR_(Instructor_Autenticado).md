Dashboard instructor

🔥 /instructor/dashboard
   ├─ Bienvenida: "Bienvenido, [Nombre Instructor]"
   ├─ Stats principales (cards):
   │  ├─ Total estudiantes activos
   │  ├─ Total cursos publicados/borradores
   │  ├─ Ingresos mes actual/total (si monetizado)
   │  ├─ Rating promedio (estrellas)
   │  └─ Nuevas inscripciones última semana
   ├─ Gráficos:
   │  ├─ Inscripciones últimos 30 días (línea temporal)
   │  ├─ Distribución estudiantes por curso (barras)
   │  └─ Engagement: Tasa completitud, tiempo promedio
   ├─ Acciones rápidas:
   │  ├─ Crear nuevo curso
   │  ├─ Ver tareas pendientes calificación (badge contador)
   │  ├─ Responder preguntas foro
   │  └─ Ver reportes detallados
   ├─ Actividad reciente:
   │  ├─ Nuevas inscripciones
   │  ├─ Reviews recientes
   │  ├─ Preguntas sin responder
   │  └─ Tareas enviadas
   └─ Notificaciones importantes: Políticas actualizadas, pagos procesados

🔥 /instructor/cursos
   ├─ Tabs:
   │  ├─ Todos | Publicados | Borradores | Archivados
   ├─ Grid/Lista cursos:
   │  ├─ Card: Thumbnail, título, estado (publicado/borrador/revisión)
   │  ├─ Stats mini: Estudiantes, rating, ingresos
   │  ├─ Acciones rápidas: Editar, Ver como estudiante, Duplicar, Analytics
   │  └─ Menú contextual: Archivar, Eliminar, Configuración
   ├─ Filtros: Categoría, nivel, estado publicación, fecha creación
   ├─ Ordenar: Recientes, Nombre A-Z, Más estudiantes, Mejor rating
   ├─ Búsqueda
   └─ Botón flotante: + Crear Nuevo Curso

---

Creación y Edición de Cursos

🔥 /instructor/curso/crear
   ├─ Wizard multi-paso (guardar automático cada paso):
   │
   │  **PASO 1: Información Básica**
   │  ├─ Título curso (validación tiempo real duplicados)
   │  ├─ Subtítulo (descripción corta, 120 chars)
   │  ├─ Slug URL (auto-generado, editable)
   │  ├─ Descripción completa (editor WYSIWYG, Markdown opcional)
   │  ├─ Categoría (desplegable anidado: Programación > Frontend > React)
   │  ├─ Etiquetas (input tags, sugerencias, max 10)
   │  ├─ Nivel: Principiante | Intermedio | Avanzado | Todos
   │  ├─ Idioma principal
   │  ├─ Subtítulos disponibles (multi-select)
   │  └─ Imagen portada (upload 1280x720, crop tool, preview múltiples tamaños)
   │
   │  **PASO 2: Contenido Curricular**
   │  ├─ Constructor temario (drag & drop):
   │  │  ├─ Secciones (acordeón colapsable)
   │  │  │  ├─ Título sección
   │  │  │  ├─ Descripción opcional
   │  │  │  └─ Lecciones dentro:
   │  │  │     ├─ Tipos: Video | Artículo | Quiz | Tarea | Recurso descargable | Videoconferencia en vivo
   │  │  │     ├─ Título lección
   │  │  │     ├─ Duración estimada
   │  │  │     ├─ Preview gratuito (toggle)
   │  │  │     └─ Contenido (dependiendo tipo):
   │  │  │        ├─ **Video**: Upload (MP4, <2GB), URL externa (YouTube/Vimeo), procesamiento HLS
   │  │  │        ├─ **Artículo**: Editor Markdown/WYSIWYG, code blocks, imágenes inline
   │  │  │        ├─ **Quiz**: Ver sección Quiz Builder más abajo
   │  │  │        ├─ **Tarea**: Instrucciones, rúbrica, fecha límite sugerida, tipo entrega
   │  │  │        └─ **Recurso**: Upload archivos (PDF, ZIP, código fuente)
   │  │  ├─ Acciones: Agregar sección, Reordenar (drag handles), Duplicar, Eliminar
   │  │  └─ Vista previa temario (modo estudiante)
   │  ├─ Requisitos previos: Lista bullets, agregar/eliminar
   │  ├─ Objetivos aprendizaje: Lista bullets (qué aprenderá estudiante)
   │  └─ Público objetivo: Descripción texto libre
   │
   │  **PASO 3: Video Promocional y Presentación**
   │  ├─ Video promocional:
   │  │  ├─ Upload (30seg - 2min recomendado)
   │  │  ├─ URL YouTube/Vimeo
   │  │  └─ Preview player
   │  ├─ Mensaje bienvenida estudiantes (texto/video corto)
   │  └─ Mensaje felicitación completitud
   │
   │  **PASO 4: Pricing y Publicación**
   │  ├─ Modelo monetización:
   │  │  ├─ Gratis
   │  │  ├─ Pago único (input precio USD/COP/MXN, conversión automática)
   │  │  ├─ Suscripción mensual
   │  │  └─ Pago por capítulo
   │  ├─ Precio regular
   │  ├─ Precio descuento (opcional, fechas inicio/fin)
   │  ├─ Cupones descuento (crear, código, %, límite usos)
   │  ├─ Visibilidad:
   │  │  ├─ Público (catálogo)
   │  │  ├─ Privado (solo por enlace directo)
   │  │  └─ Por invitación (whitelist emails)
   │  ├─ Inscripción automática o requiere aprobación
   │  ├─ Límite estudiantes (opcional)
   │  ├─ Fechas programadas:
   │  │  ├─ Fecha inicio inscripciones
   │  │  ├─ Fecha cierre inscripciones
   │  │  ├─ Fecha inicio curso
   │  │  └─ Fecha finalización (si cohorte cerrada)
   │  └─ Certificado completitud (toggle habilitado, % mínimo aprobación)
   │
   │  **PASO 5: Configuración Avanzada**
   │  ├─ Foro curso (habilitar/deshabilitar)
   │  ├─ Q&A instructor (habilitar/deshabilitar)
   │  ├─ Mensajería directa estudiantes
   │  ├─ Descargar recursos (permitir/denegar)
   │  ├─ Límite dispositivos simultáneos
   │  ├─ DRM protección video (toggle)
   │  ├─ Co-instructores: Agregar (email, permisos: editar/solo ver)
   │  ├─ Asistentes enseñanza: Agregar (permisos calificar/moderar foro)
   │  └─ Plantilla comunicación: Emails automáticos (bienvenida, recordatorios)
   │
   └─ Botones finales:
      ├─ Guardar borrador
      ├─ Vista previa completa (abrir en nueva pestaña, vista estudiante)
      ├─ Enviar a revisión (si plataforma tiene proceso aprobación)
      └─ Publicar curso

🔥 /instructor/curso/:courseId/editar
   ├─ Mismo wizard pero con datos pre-cargados
   ├─ Alertas: "X estudiantes inscritos, cambios visibles inmediatamente"
   ├─ Sistema versionado:
   │  ├─ Historial versiones (lista cambios, rollback)
   │  ├─ Notificar estudiantes sobre actualización
   │  └─ Changelog público
   └─ Modo colaboración: Ver quién está editando en tiempo real

---

Quiz Builder (Sub-módulo)

⚡ /instructor/curso/:courseId/leccion/:lessonId/quiz-builder
   ├─ Header: Título quiz, configuración general
   ├─ Configuración:
   │  ├─ Tiempo límite (minutos, o sin límite)
   │  ├─ Intentos permitidos (ilimitados, 1, 2, 3)
   │  ├─ Mostrar respuestas correctas (inmediato/después envío/nunca)
   │  ├─ Puntaje mínimo aprobación (%)
   │  ├─ Orden preguntas aleatorio (toggle)
   │  ├─ Orden respuestas aleatorio (toggle)
   │  └─ Permitir retroceder preguntas
   ├─ Constructor preguntas:
   │  ├─ Agregar pregunta (tipos):
   │  │  ├─ **Opción múltiple**: 
   │  │  │  ├─ Enunciado (editor rico, imágenes, código)
   │  │  │  ├─ Opciones (mín 2, máx 6, marcar correcta(s))
   │  │  │  ├─ Explicación respuesta correcta (opcional)
   │  │  │  └─ Puntos pregunta
   │  │  ├─ **Verdadero/Falso**
   │  │  ├─ **Respuesta corta**: Validación texto (exacto/contiene/regex)
   │  │  ├─ **Rellenar espacios**: Texto con blanks [___]
   │  │  ├─ **Matching**: Emparejar columnas
   │  │  └─ **Ensayo**: Respuesta abierta (calificación manual)
   │  ├─ Reordenar preguntas (drag & drop)
   │  ├─ Duplicar pregunta
   │  ├─ Banco preguntas: Importar de otros quizzes
   │  └─ Importar CSV/Excel (template descargable)
   ├─ Preview quiz (modo estudiante)
   └─ Guardar quiz

---

Gestión Estudiantes y Calificaciones

🔥 /instructor/curso/:courseId/estudiantes
   ├─ Lista estudiantes inscritos:
   │  ├─ Tabla: Nombre, Email, Fecha inscripción, Progreso %, Última actividad
   │  ├─ Filtros: Estado (activo/inactivo/completado), Progreso (0-25%, 26-50%, etc)
   │  ├─ Búsqueda nombre/email
   │  ├─ Ordenar: Nombre, Progreso, Fecha
   │  └─ Exportar CSV/Excel
   ├─ Acciones masivas:
   │  ├─ Enviar email grupo
   │  ├─ Dar acceso especial
   │  └─ Remover estudiantes
   ├─ Vista individual estudiante:
   │  ├─ Perfil: Datos, foto, contacto
   │  ├─ Progreso detallado: Timeline lecciones completadas
   │  ├─ Evaluaciones: Puntajes quizzes, tareas enviadas
   │  ├─ Participación: Posts foro, preguntas
   │  ├─ Tiempo invertido total
   │  ├─ Acciones: Enviar mensaje, Extender acceso, Resetear progreso
   │  └─ Notas privadas instructor sobre estudiante
   └─ Analytics grupo:
      ├─ Tasa completitud
      ├─ Lecciones con más abandonos
      └─ Promedio tiempo por sección

🔥 /instructor/curso/:courseId/calificaciones
   ├─ Tabs:
   │  ├─ Tareas pendientes (badge contador)
   │  ├─ Quizzes enviados
   │  └─ Todas las calificaciones
   ├─ Vista tarea:
   │  ├─ Lista envíos estudiantes
   │  ├─ Filtros: Pendiente/Calificada, Fecha envío
   │  ├─ Card envío:
   │  │  ├─ Estudiante, fecha envío
   │  │  ├─ Archivos adjuntos (viewer inline PDFs, imágenes, código)
   │  │  ├─ Texto respuesta
   │  │  ├─ Rúbrica calificación (criterios, puntos por criterio)
   │  │  ├─ Input puntaje total
   │  │  ├─ Feedback texto enriquecido (menciones, adjuntar archivos)
   │  │  ├─ Feedback audio/video (opcional, grabación inline)
   │  │  └─ Botones: Guardar borrador, Publicar calificación
   │  └─ Navegación: Siguiente envío (sin salir página)
   ├─ Vista quiz:
   │  ├─ Auto-calificados (solo ver resultados)
   │  ├─ Preguntas abiertas: Calificar manualmente
   │  └─ Estadísticas pregunta: % acierto, respuestas comunes incorrectas
   └─ Exportar calificaciones: CSV con todos los estudiantes

⚡ /instructor/curso/:courseId/comunicacion
   ├─ Anuncios:
   │  ├─ Crear anuncio: Título, contenido, adjuntos
   │  ├─ Enviar email adicional (toggle)
   │  ├─ Programar envío
   │  ├─ Historial anuncios publicados
   │  └─ Analytics: Tasa apertura email
   ├─ Emails masivos:
   │  ├─ Plantillas predefinidas (bienvenida, recordatorio, felicitación)
   │  ├─ Destinatarios: Todos/Activos/Inactivos/Completados/Filtro personalizado
   │  ├─ Personalización: Variables {nombre}, {progreso}
   │  └─ Preview antes enviar
   └─ Mensajes directos: Ver bandeja mensajes estudiantes

---

Moderación Foro y Q&A

⚡ /instructor/curso/:courseId/foro
   ├─ Vista general:
   │  ├─ Tabs: Todas | Sin responder | Marcadas | Archivadas
   │  ├─ Filtros: Fecha, Etiquetas, Sección curso
   │  ├─ Búsqueda preguntas
   │  └─ Ordenar: Recientes, Más votadas, Sin respuesta
   ├─ Lista hilos:
   │  ├─ Card pregunta:
   │  │  ├─ Título, extracto
   │  │  ├─ Autor estudiante, fecha
   │  │  ├─ Badges: Sin responder, Instructor respondió ✓, Resuelto
   │  │  ├─ Stats: Votos, # respuestas, vistas
   │  │  └─ Acciones rápidas: Responder, Marcar, Archivar, Eliminar
   │  └─ Acceso rápido hilo (modal o navegación)
   ├─ Vista detalle hilo:
   │  ├─ Pregunta completa (código, imágenes, adjuntos)
   │  ├─ Respuestas ordenadas (votos/fecha)
   │  ├─ Editor respuesta instructor:
   │  │  ├─ Texto enriquecido, code highlighting
   │  │  ├─ Menciones @estudiante
   │  │  ├─ Adjuntar recursos
   │  │  └─ Marcar como "Respuesta oficial instructor" (destacada)
   │  ├─ Moderar respuestas estudiantes:
   │  │  ├─ Aprobar/Rechazar (si moderación activada)
   │  │  ├─ Editar contenido inapropiado
   │  │  ├─ Eliminar spam
   │  │  └─ Anclar respuesta útil
   │  ├─ Marcar pregunta como resuelta
   │  ├─ Convertir en FAQ
   │  └─ Cerrar hilo (no más respuestas)
   └─ Configuración foro:
      ├─ Moderación previa (aprobar posts antes publicar)
      ├─ Permitir estudiantes responder entre sí
      ├─ Habilitar votos
      ├─ Asignar moderadores (asistentes)
      └─ Palabras prohibidas/filtro spam

⚡ /instructor/curso/:courseId/preguntas
   ├─ Q&A por lección (alternativa a foro completo)
   ├─ Preguntas agrupadas por lección
   ├─ Responder inline
   ├─ Marcar como respondida
   └─ Exportar FAQ (generar documento)

---

Analytics y Reportes

🔥 /instructor/curso/:courseId/analytics
   ├─ Dashboard analytics:
   │  ├─ Selector fecha: Última semana/mes/trimestre/año/todo/personalizado
   │  ├─ **Métricas principales** (cards):
   │  │  ├─ Total estudiantes inscritos (gráfico tendencia)
   │  │  ├─ Estudiantes activos (últimos 30 días)
   │  │  ├─ Tasa completitud curso (%)
   │  │  ├─ Rating promedio (estrellas + distribución)
   │  │  ├─ Ingresos generados (si monetizado)
   │  │  └─ Tiempo promedio completitud
   │  │
   │  ├─ **Gráficos engagement**:
   │  │  ├─ Inscripciones en el tiempo (línea)
   │  │  ├─ Actividad diaria estudiantes (barras)
   │  │  ├─ Horas de video consumidas
   │  │  ├─ Engagement por sección (funnel abandonos)
   │  │  └─ Dispositivos utilizados (desktop/mobile/tablet)
   │  │
   │  ├─ **Performance contenido**:
   │  │  ├─ Top 10 lecciones más vistas
   │  │  ├─ Lecciones con mayor % abandono (alertas rojas)
   │  │  ├─ Promedio tiempo por lección vs duración
   │  │  ├─ Lecciones con más re-visualizaciones
   │  │  └─ Mapa calor: Partes video más repetidas/saltadas
   │  │
   │  ├─ **Evaluaciones**:
   │  │  ├─ Promedio puntajes quizzes
   │  │  ├─ Preguntas con menor % acierto (revisar dificultad)
   │  │  ├─ Tiempo promedio completar evaluaciones
   │  │  ├─ Tasa envío tareas (% estudiantes que envían)
   │  │  └─ Distribución calificaciones (histograma)
   │  │
   │  ├─ **Participación social**:
   │  │  ├─ # Preguntas foro (tendencia)
   │  │  ├─ Tiempo promedio respuesta instructor
   │  │  ├─ Estudiantes más activos foro
   │  │  ├─ Tasa respuesta estudiante-a-estudiante
   │  │  └─ Sentiment analysis reviews (positivo/neutral/negativo)
   │  │
   │  └─ **Ingresos** (si aplica):
   │     ├─ Revenue mensual (gráfico)
   │     ├─ Conversión visitantes → inscripciones (funnel)
   │     ├─ Efectividad cupones descuento
   │     ├─ Refunds/cancelaciones
   │     └─ Proyección ingresos
   │
   ├─ Comparativas:
   │  ├─ Este curso vs otros cursos propios
   │  ├─ Benchmark industria (si disponible)
   │  └─ Ranking plataforma
   │
   ├─ Exportar reportes:
   │  ├─ PDF ejecutivo
   │  ├─ Excel datos raw
   │  └─ Programar envío email automático (semanal/mensual)
   │
   └─ Insights IA (futuro):
      ├─ "Tu sección 3 tiene 40% abandono, considera dividirla"
      ├─ "Estudiantes tardan 2x más en lección 5, agrega recursos"
      └─ "Patrón: estudiantes que completan quiz 1 tienen 80% más probabilidad terminar curso"

⚡ /instructor/analytics-global
   ├─ Vista consolidada todos los cursos
   ├─ Comparativa performance entre cursos
   ├─ Total estudiantes impactados
   ├─ Ingresos totales
   └─ Tendencias crecimiento

---

Recursos Multimedia y Biblioteca

🎯 /instructor/biblioteca
   ├─ Gestión archivos centralizada:
   │  ├─ Videos: Lista todos videos subidos (cualquier curso)
   │  ├─ Imágenes: Galería
   │  ├─ Documentos: PDFs, slides, código
   │  ├─ Audios: Podcasts, grabaciones
   │  └─ Otros: ZIP, archivos diversos
   ├─ Metadata archivos:
   │  ├─ Nombre, tamaño, formato
   │  ├─ Fecha subida, cursos que lo usan
   │  ├─ Etiquetas, descripción
   │  └─ Procesamiento estado (videos HLS)
   ├─ Acciones:
   │  ├─ Upload masivo (drag & drop múltiples)
   │  ├─ Organizar en carpetas
   │  ├─ Renombrar, mover, eliminar
   │  ├─ Reemplazar archivo (mantiene referencias)
   │  └─ Ver uso (¿en qué lecciones está?)
   ├─ Búsqueda y filtros
   ├─ Almacenamiento: Usado/Límite (barra progreso)
   └─ Papelera: Archivos eliminados (recuperar 30 días)

⚡ /instructor/videoconferencias (si integración)
   ├─ Programar sesión en vivo:
   │  ├─ Título, descripción
   │  ├─ Fecha/hora, duración
   │  ├─ Asociar a curso (opcional)
   │  ├─ Configuración:
   │  │  ├─ Límite asistentes
   │  │  ├─ Requiere registro previo
   │  │  ├─ Grabar sesión (auto-upload como lección)
   │  │  ├─ Chat habilitado
   │  │  └─ Q&A habilitado
   │  └─ Generar enlace/enviar invitaciones
   ├─ Mis sesiones: Próximas, Pasadas, Grabaciones
   ├─ Durante sesión: Panel control (screen share, mute all, etc.)
   └─ Estadísticas: Asistencia, engagement, grabación analytics

---

Marketing y Promoción

🎯 /instructor/curso/:courseId/marketing
   ├─ **Landing page personalizada**:
   │  ├─ Editar elementos: Hero, testimonios, FAQ personalizado
   │  ├─ Galería screenshots curso
   │  ├─ Video destacado diferente a preview
   │  ├─ SEO: Meta title, description, keywords
   │  └─ Preview cambios
   │
   ├─ **Cupones descuento**:
   │  ├─ Crear cupón: Código, % o monto fijo, fecha validez
   │  ├─ Límite usos: Ilimitado/X veces/1 por usuario
   │  ├─ Restricciones: Solo nuevos/todos
   │  ├─ Tracking: Usos, conversión
   │  └─ Compartir enlace con cupón pre-aplicado
   │
   ├─ **Promociones especiales**:
   │  ├─ Flash sales (fechas específicas)
   │  ├─ Descuento por volumen (3x2, pack cursos)
   │  ├─ Early bird (pre-lanzamiento)
   │  └─ Descuento afiliados
   │
   ├─ **Email marketing**:
   │  ├─ Captura leads (landing pre-lanzamiento)
   │  ├─ Drip campaigns (secuencias automáticas)
   │  └─ Re-engagement inactivos
   │
   ├─ **Afiliados** (si habilitado):
   │  ├─ Programa afiliados: Comisión %, cookies duración
   │  ├─ Enlaces de afiliado personalizados
   │  ├─ Dashboard afiliados: Clicks, conversiones, comisiones
   │  └─ Materiales marketing: Banners, copy sugerido
   │
   └─ **Social media**:
      ├─ Compartir curso: Generar posts optimizados (Twitter, LinkedIn, FB)
      ├─ Badges: "Instructor en PCC-LMS", "Curso destacado"
      └─ Stats compartidos: Alcance, clicks

---

Configuración Perfil Instructor

🔥 /instructor/perfil
   ├─ **Perfil público**:
   │  ├─ Foto profesional (grande, circular)
   │  ├─ Banner perfil (cover)
   │  ├─ Nombre completo
   │  ├─ Título profesional (ej: "Senior Developer", "PhD Computer Science")
   │  ├─ Biografía (500 palabras, Markdown)
   │  ├─ Video introducción (opcional, 1-2 min)
   │  ├─ Expertise: Etiquetas habilidades
   │  ├─ Redes sociales: LinkedIn, Twitter, GitHub, website
   │  ├─ Estadísticas públicas:
   │  │  ├─ Total estudiantes enseñados
   │  │  ├─ Total cursos
   │  │  ├─ Rating promedio
   │  │  └─ Desde (año inicio como instructor)
   │  └─ Testimonios destacados
   │
   ├─ **Información privada**:
   │  ├─ Email contacto (diferente a login)
   │  ├─ Teléfono
   │  ├─ Datos facturación (para pagos)
   │  └─ Datos fiscales: NIT/RFC/Tax ID
   │
   ├─ **Configuración cuenta**:
   │  ├─ Cambiar contraseña
   │  ├─ 2FA
   │  ├─ Sesiones activas
   │  └─ Preferencias notificaciones:
   │     ├─ Nuevas inscripciones
   │     ├─ Nuevas preguntas foro
   │     ├─ Reviews/ratings
   │     ├─ Tareas enviadas
   │     ├─ Pagos recibidos
   │     └─ Actualizaciones plataforma
   │
   └─ **Pagos y facturación**:
      ├─ Método pago preferido (cuenta bancaria, PayPal)
      ├─ Historial pagos recibidos
      ├─ Facturas emitidas (generar)
      ├─ Retenciones fiscales
      └─ Reporte ingresos anual (para impuestos)

🎯 /instructor/certificaciones (opcional)
   ├─ Subir certificados profesionales propios
   ├─ Validación identidad (KYC instructor verificado badge)
   └─ Logros plataforma: "Top instructor 2024", "100K+ estudiantes"

---

Plantillas y Recursos Instructor

💡 /instructor/plantillas
   ├─ Plantillas cursos: Importar estructura pre-armada
   ├─ Banco preguntas: Quiz questions reutilizables por tema
   ├─ Emails templates: Personalizar comunicaciones
   └─ Rúbricas evaluación: Guardar rúbricas favoritas

💡 /instructor/recursos-ayuda
   ├─ Centro ayuda instructor:
   │  ├─ Guía creación primer curso
   │  ├─ Mejores prácticas pedagógicas
   │  ├─ Tips engagement estudiantes
   │  ├─ Optimización SEO curso
   │  └─ Guía monetización
   ├─ Videos tutoriales
   ├─ Webinars instructores
   └─ Comunidad instructores (foro privado)

---

Configuración Avanzada y Herramientas

⚡ /instructor/curso/:courseId/configuracion
   ├─ **General**:
   │  ├─ Cambiar estado: Publicado/Borrador/Archivado
   │  ├─ Transferir propiedad curso
   │  ├─ Eliminar curso (confirmación, afecta estudiantes)
   │  └─ Duplicar curso completo
   │
   ├─ **Acceso**:
   │  ├─ Período inscripción
   │  ├─ Drip content (liberar lecciones gradualmente):
   │  │  ├─ Por calendario (X días después inscripción)
   │  │  └─ Por completitud (desbloquear al terminar anterior)
   │  ├─ Prerequisitos: Requiere otro curso completado
   │  └─ Whitelist/Blacklist emails
   │
   ├─ **Certificados**:
   │  ├─ Plantilla diseño certificado (editor visual)
   │  ├─ Variables: {nombre}, {curso}, {fecha}, {firma}
   │  ├─ Requisitos emisión: % completitud, puntaje mínimo
   │  └─ Preview certificado
   │
   ├─ **Integraciones**:
   │  ├─ Zapier/Webhooks (eventos: nueva inscripción, completitud)
   │  ├─ Zoom/Google Meet para videoconferencias
   │  ├─ GitHub para ejercicios código
   │  └─ API keys (si instructor tiene acceso API)
   │
   └─ **Avanzado**:
      ├─ Custom CSS (personalización avanzada UI curso)
      ├─ JavaScript tracking (analytics externo)
      ├─ Embed código (badges, widgets)
      └─ Backup curso: Exportar todo (JSON/ZIP)

---

