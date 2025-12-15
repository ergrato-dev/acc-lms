📁 STUDENT DASHBOARD
│
├─ 🔥 /dashboard (Dashboard Principal)
│  ├─ Header: Bienvenida personalizada
│  ├─ Sección "Continuar aprendiendo"
│  │  ├─ Últimos cursos vistos
│  │  └─ Progreso visual por curso
│  ├─ Sección "Mis cursos"
│  │  ├─ Grid de cursos matriculados
│  │  ├─ Filtros: En curso, Completados, Sin iniciar
│  │  ├─ Ordenamiento: Actividad reciente, Progreso, Fecha
│  │  └─ Barra de progreso por curso
│  ├─ Sección "Recomendados para ti" (IA)
│  ├─ Stats personales:
│  │  ├─ Total cursos
│  │  ├─ Cursos completados
│  │  ├─ Horas de aprendizaje
│  │  └─ Certificados obtenidos
│  ├─ Próximos vencimientos (tareas/quizzes)
│  └─ Actividad reciente
│
├─ 🔥 /mis-cursos
│  ├─ Lista completa de cursos matriculados
│  ├─ Filtros avanzados
│  ├─ Búsqueda
│  ├─ Vista: Grid / Lista
│  └─ Estado vacío con CTA al catálogo
│
├─ ⚡ /lista-de-deseos
│  ├─ Cursos guardados
│  ├─ Notificación de descuentos
│  ├─ Botón "Agregar al carrito"
│  └─ Opción de eliminar
│
└─ ⚡ /historial
   ├─ Actividad de aprendizaje
   ├─ Lecciones completadas
   ├─ Quizzes realizados
   └─ Filtros por fecha

---

Player de Aprendizaje

📁 LEARNING EXPERIENCE
│
├─ 🔥 /learn/:courseId (Player Principal)
│  ├─ Layout:
│  │  ├─ Sidebar izquierdo (colapsable):
│  │  │  ├─ Temario completo
│  │  │  ├─ Agrupación por secciones
│  │  │  ├─ Indicadores: ✓ completada, • actual, ○ pendiente
│  │  │  ├─ Duración por lección
│  │  │  └─ Búsqueda dentro del temario
│  │  │
│  │  ├─ Área central de contenido:
│  │  │  ├─ Video player (HLS adaptativo)
│  │  │  │  ├─ Controles: play/pause, seek, volumen, fullscreen
│  │  │  │  ├─ Velocidad: 0.5x - 2x
│  │  │  │  ├─ Calidad: Auto, 1080p, 720p, 360p
│  │  │  │  ├─ Subtítulos (si disponible)
│  │  │  │  ├─ Picture-in-picture
│  │  │  │  └─ Continuación desde último punto
│  │  │  │
│  │  │  ├─ Artículo/Texto (si aplica)
│  │  │  │  ├─ Renderizado Markdown
│  │  │  │  ├─ Code highlighting
│  │  │  │  └─ Imágenes responsive
│  │  │  │
│  │  │  └─ Quiz (si aplica)
│  │  │     └─ Ver detalle en sección QUIZZES
│  │  │
│  │  └─ Tabs inferiores:
│  │     ├─ 📝 Descripción
│  │     ├─ 📎 Recursos descargables
│  │     ├─ 💬 Preguntas y Respuestas
│  │     ├─ 📝 Notas personales
│  │     └─ 🤖 Tutor IA (si disponible)
│  │
│  ├─ Header superior:
│  │  ├─ Título del curso
│  │  ├─ Progreso general (barra + %)
│  │  ├─ Botones: ← Anterior | Siguiente →
│  │  ├─ Marcar como completada
│  │  └─ Salir del player
│  │
│  └─ Features:
│     ├─ Auto-advance al terminar lección
│     ├─ Keyboard shortcuts
│     ├─ Guardado automático de progreso
│     └─ Modo offline (PWA - futuro)
│
└─ ⚡ /learn/:courseId/notas
   ├─ Editor de notas por lección
   ├─ Timestamp linking (vincular a segundo del video)
   ├─ Markdown support
   ├─ Búsqueda en notas
   └─ Exportar notas (PDF, TXT)

---

Evaluaciones y Progreso

🔥 /learn/:courseId/quiz/:quizId
   ├─ Header: Título quiz, tiempo límite countdown, preguntas completadas (5/10)
   ├─ Área principal:
   │  ├─ Pregunta actual (numerada)
   │  ├─ Tipos: Opción múltiple, Verdadero/Falso, Respuesta corta, Rellenar espacios
   │  ├─ Navegación: Anterior/Siguiente, Saltar pregunta
   │  ├─ Estado: Respondida ✓, Sin responder ○
   │  └─ Botones: Guardar borrador, Enviar quiz
   ├─ Sidebar: Mapa preguntas (grid navegación rápida)
   └─ Post-envío: Resultados (puntaje, respuestas correctas/incorrectas, retroalimentación, intentos restantes)

🔥 /learn/:courseId/tarea/:assignmentId
   ├─ Header: Título, fecha límite, puntos máximos, estado (pendiente/enviada/calificada)
   ├─ Instrucciones: Descripción detallada, criterios evaluación, rúbrica
   ├─ Recursos: Archivos adjuntos instructor
   ├─ Área envío:
   │  ├─ Editor texto enriquecido
   │  ├─ Upload archivos (múltiples, drag-drop, límite 50MB/archivo)
   │  ├─ Vista previa archivos adjuntos
   │  └─ Botón: Guardar borrador / Enviar tarea
   ├─ Historial envíos: Lista intentos anteriores (si permitidos)
   └─ Retroalimentación: Calificación, comentarios instructor, archivos adjuntos retroalimentación

⚡ /progreso/:courseId
   ├─ Visión general: Progreso general (%), tiempo invertido, fecha inicio/última actividad
   ├─ Desglose por secciones: Progress bars, lecciones completadas/totales
   ├─ Evaluaciones: Lista quizzes/tareas (puntajes, intentos, fechas)
   ├─ Certificado: Estado (disponible/en progreso), requisitos cumplimiento, botón descargar
   └─ Timeline actividad: Gráfico actividad por día/semana

🎯 /certificados
   ├─ Grid certificados obtenidos
   ├─ Card: Curso, fecha obtención, código verificación, instructor
   ├─ Acciones: Ver certificado, Descargar PDF, Compartir LinkedIn
   └─ Verificación pública: /certificado/verificar/:codigo (página pública)

   ---

   Interacción Social

   ⚡ /learn/:courseId/foro
   ├─ Lista hilos (ordenar: recientes/populares/sin respuesta)
   ├─ Filtros: Mis preguntas, Instructor respondió, Resuelto
   ├─ Nuevo hilo: Título, contenido (editor rico), etiquetas, adjuntar código
   ├─ Vista hilo:
   │  ├─ Pregunta original (autor, fecha, votos, marcar resuelto)
   │  ├─ Respuestas (ordenar: votos/fecha, respuesta aceptada destacada)
   │  ├─ Votar arriba/abajo
   │  ├─ Responder (editor, menciones @usuario, code snippets)
   │  └─ Seguir hilo (notificaciones)
   └─ Búsqueda foro

💡 /comunidad (futuro)
   ├─ Foro general plataforma
   ├─ Grupos estudio
   └─ Eventos/Webinars

⚡ /mensajes
   ├─ Lista conversaciones (instructores, estudiantes si habilitado)
   ├─ Chat view: Mensajes tiempo real, adjuntar archivos, emojis
   ├─ Filtros: No leídos, Archivados
   └─ Notificaciones: Badge contador no leídos

---

Configuración y perfil

🔥 /perfil
   ├─ Tabs:
   │  ├─ **Información Personal**
   │  │  ├─ Foto perfil (upload, crop circular)
   │  │  ├─ Datos: Nombre, Apellido, Email (verificado badge), Teléfono
   │  │  ├─ Biografía (textarea, 500 chars)
   │  │  ├─ Ubicación: País, Ciudad
   │  │  └─ Redes sociales: LinkedIn, GitHub, Twitter
   │  │
   │  ├─ **Privacidad**
   │  │  ├─ Perfil público (toggle)
   │  │  ├─ Mostrar progreso cursos
   │  │  ├─ Mostrar certificados
   │  │  └─ Permitir mensajes estudiantes
   │  │
   │  ├─ **Seguridad**
   │  │  ├─ Cambiar contraseña (actual, nueva, confirmar)
   │  │  ├─ Autenticación 2 factores (QR setup, códigos backup)
   │  │  ├─ Sesiones activas (lista dispositivos, cerrar sesión remota)
   │  │  └─ Actividad cuenta: IPs, fechas acceso
   │  │
   │  ├─ **Notificaciones**
   │  │  ├─ Email: Nuevos cursos, Actualizaciones curso, Mensajes, Tareas vencimiento
   │  │  ├─ Push: Habilitadas/Deshabilitadas, permisos browser
   │  │  └─ In-app: Notificaciones plataforma
   │  │
   │  └─ **Preferencias**
   │     ├─ Idioma interfaz (ES/EN/PT)
   │     ├─ Zona horaria
   │     ├─ Tema: Claro/Oscuro/Auto
   │     ├─ Velocidad reproducción predeterminada
   │     └─ Auto-avance lecciones

🎯 /planes-y-suscripcion
   ├─ Plan actual: Nombre, precio, siguiente factura, beneficios
   ├─ Comparativa planes: Free/Pro/Premium (tabla características)
   ├─ Upgrade/Downgrade
   ├─ Historial facturación: Lista facturas (descargar PDF)
   ├─ Métodos pago: Tarjetas guardadas, agregar/eliminar
   └─ Cancelar suscripción: Flow confirmación, encuesta feedback

💡 /objetivos (futuro)
   ├─ Planificador objetivos aprendizaje
   ├─ Metas semanales/mensuales
   ├─ Tracking racha días
   └─ Gamificación badges

---

Compras y Transacciones

⚡ /carrito
   ├─ Lista cursos agregados
   ├─ Subtotal, descuentos, total
   ├─ Cupón descuento (input, aplicar)
   ├─ Eliminar items
   └─ Botón: Proceder al pago

🔥 /checkout
   ├─ Paso 1 - Resumen orden: Cursos, precios
   ├─ Paso 2 - Método pago:
   │  ├─ Tarjeta crédito/débito (Stripe/MercadoPago)
   │  ├─ PayPal
   │  ├─ Transferencia bancaria
   │  └─ Pago local (PSE Colombia, Oxxo México, etc.)
   ├─ Paso 3 - Datos facturación: Nombre, dirección, NIT/RFC
   ├─ Paso 4 - Confirmación: Revisar y confirmar
   └─ Post-pago: Página éxito, acceso inmediato cursos, email confirmación

🎯 /mis-compras
   ├─ Historial transacciones
   ├─ Facturas (descargar PDF/XML)
   ├─ Reembolsos: Estado, políticas
   └─ Filtros: Fecha, monto, estado

---

