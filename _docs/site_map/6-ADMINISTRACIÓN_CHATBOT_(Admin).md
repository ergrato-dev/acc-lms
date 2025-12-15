🔥 /admin/chatbot
   ├─ **Dashboard chatbot**:
   │  ├─ Métricas principales (período seleccionable):
   │  │  ├─ Total conversaciones
   │  │  ├─ Usuarios únicos interactuaron
   │  │  ├─ Mensajes enviados (usuario + bot)
   │  │  ├─ Tasa resolución automática (sin escalación humano)
   │  │  ├─ Satisfaction score (CSAT - basado en 👍👎)
   │  │  ├─ Tiempo promedio respuesta bot
   │  │  ├─ Tiempo promedio conversación
   │  │  ├─ Escalaciones a humano (#, %)
   │  │  └─ Tasa abandono chat
   │  ├─ Gráficos:
   │  │  ├─ Conversaciones en el tiempo (línea)
   │  │  ├─ Distribución por rol (pie: Anónimo/Estudiante/Instructor/Admin)
   │  │  ├─ Horarios pico actividad (heatmap)
   │  │  ├─ Sentiment analysis (positivo/neutral/negativo)
   │  │  └─ Topics más consultados (word cloud)
   │  ├─ Top consultas:
   │  │  ├─ Preguntas más frecuentes (últimos 7/30 días)
   │  │  ├─ # Veces preguntada
   │  │  ├─ Tasa respuesta satisfactoria
   │  │  └─ Link a respuesta KB asociada
   │  ├─ Alertas:
   │  │  ├─ Preguntas sin respuesta adecuada (gaps KB)
   │  │  ├─ Caída satisfaction score
   │  │  ├─ Aumento escalaciones
   │  │  └─ Errores técnicos bot
   │  └─ Quick actions:
   │     ├─ Ver conversaciones en vivo
   │     ├─ Agregar nuevo artículo KB
   │     ├─ Entrenar bot (nuevo contenido)
   │     └─ Configuración rápida
   │
   ├─ **Conversaciones (historial y monitoreo)**:
   │  ├─ **Vista lista conversaciones**:
   │  │  ├─ Filtros:
   │  │  │  ├─ Estado: Activa/Resuelta/Escalada/Abandonada
   │  │  │  ├─ Rol usuario: Anónimo/Estudiante/Instructor/Admin
   │  │  │  ├─ Satisfaction: Positiva/Neutral/Negativa/Sin feedback
   │  │  │  ├─ Duración conversación (rangos)
   │  │  │  ├─ # Mensajes (rangos)
   │  │  │  ├─ Escalada a humano (sí/no)
   │  │  │  ├─ Fecha (rango)
   │  │  │  ├─ Idioma
   │  │  │  └─ Contiene palabra clave
   │  │  ├─ Tabla conversaciones:
   │  │  │  ├─ ID conversación
   │  │  │  ├─ Usuario (nombre/email si autenticado, "Anónimo" si no)
   │  │  │  ├─ Rol, Dispositivo
   │  │  │  ├─ Fecha/hora inicio, Duración
   │  │  │  ├─ # Mensajes (usuario/bot)
   │  │  │  ├─ Estado, Escalada
   │  │  │  ├─ Tema principal (auto-detectado/tagged)
   │  │  │  ├─ Sentiment (emoji/color)
   │  │  │  ├─ Satisfaction (👍👎 o N/A)
   │  │  │  └─ Acciones: Ver detalle, Exportar, Etiquetar
   │  │  ├─ Ordenar: Fecha, Duración, Mensajes, Satisfaction
   │  │  ├─ Búsqueda: Full-text en mensajes
   │  │  └─ Paginación
   │  │
   │  ├─ **Vista detalle conversación**:
   │  │  ├─ Header info:
   │  │  │  ├─ Usuario (link perfil si autenticado)
   │  │  │  ├─ Metadata: IP, User Agent, Ubicación
   │  │  │  ├─ Contexto: Página donde inició chat, Referrer
   │  │  │  ├─ Duración, # Mensajes
   │  │  │  └─ Etiquetas (agregar/editar)
   │  │  ├─ Timeline conversación completa:
   │  │  │  ├─ Todos los mensajes (usuario + bot)
   │  │  │  ├─ Timestamps precisos
   │  │  │  ├─ Acciones bot (búsqueda KB, llamada API)
   │  │  │  ├─ Confidence score respuestas bot (%)
   │  │  │  ├─ Artículos KB referenciados (links)
   │  │  │  ├─ Punto escalación (si aplica)
   │  │  │  └─ Feedback inline (👍👎 con comentario si dejó)
   │  │  ├─ Análisis IA:
   │  │  │  ├─ Intent detection (intención usuario cada mensaje)
   │  │  │  ├─ Entities extraídas (curso, usuario, fecha, monto)
   │  │  │  ├─ Sentiment por mensaje
   │  │  │  ├─ Topics discutidos
   │  │  │  └─ Issue resolved: Sí/No/Parcialmente
   │  │  ├─ Acciones admin:
   │  │  │  ├─ Marcar como training example (buena/mala respuesta)
   │  │  │  ├─ Crear artículo KB desde esta conversación
   │  │  │  ├─ Reportar error bot
   │  │  │  ├─ Agregar a FAQs
   │  │  │  ├─ Compartir con equipo (interno)
   │  │  │  └─ Exportar (JSON/PDF)
   │  │  └─ Sidebar:
   │  │     ├─ Conversaciones relacionadas (mismo usuario)
   │  │     ├─ Tickets asociados
   │  │     └─ Sugerencias mejora respuesta
   │  │
   │  └─ **Conversaciones en vivo** (real-time monitoring):
   │     ├─ Lista chats activos ahora:
   │     │  ├─ Usuario, tiempo transcurrido
   │     │  ├─ Último mensaje (preview)
   │     │  ├─ Sentiment actual
   │     │  └─ Alerta si detecta frustración
   │     ├─ Join conversación (takeover humano):
   │     │  ├─ Admin puede intervenir
   │     │  ├─ Bot se pausa
   │     │  └─ Usuario notificado: "Un agente se ha unido"
   │     └─ Monitor mode (observar sin intervenir)
   │
   ├─ **Base de Conocimiento (Knowledge Base)**:
   │  ├─ **Gestión artículos**:
   │  │  ├─ Lista artículos KB:
   │  │  │  ├─ Título, Categoría
   │  │  │  ├─ Estado: Publicado/Borrador/Archivado
   │  │  │  ├─ Idioma(s) disponibles
   │  │  │  ├─ Fecha creación/actualización
   │  │  │  ├─ Autor (admin)
   │  │  │  ├─ Estadísticas:
   │  │  │  │  ├─ Veces mostrado por bot
   │  │  │  │  ├─ Veces marcado útil 👍
   │  │  │  │  ├─ Veces marcado no útil 👎
   │  │  │  │  └─ Effectiveness score (%)
   │  │  │  ├─ Tags/Keywords
   │  │  │  └─ Acciones: Editar, Duplicar, Archivar, Eliminar
   │  │  ├─ Filtros: Categoría, Estado, Idioma, Effectiveness
   │  │  ├─ Búsqueda: Título, contenido, tags
   │  │  └─ Ordenar: Fecha, Uso, Effectiveness
   │  │
   │  ├─ **Editor artículo KB**:
   │  │  ├─ Información básica:
   │  │  │  ├─ Título (claro, específico)
   │  │  │  ├─ Slug URL (auto-generado, editable)
   │  │  │  ├─ Categoría (dropdown jerárquico)
   │  │  │  ├─ Subcategoría
   │  │  │  └─ Idioma principal
   │  │  ├─ Contenido principal:
   │  │  │  ├─ Editor WYSIWYG/Markdown (toggle)
   │  │  │  ├─ Formato rico:
   │  │  │  │  ├─ Headers, Bold, Italic, Links
   │  │  │  │  ├─ Lists (ordered/unordered)
   │  │  │  │  ├─ Code blocks (syntax highlighting)
   │  │  │  │  ├─ Imágenes (upload/URL)
   │  │  │  │  ├─ Videos (embed YouTube, Vimeo)
   │  │  │  │  ├─ Tablas
   │  │  │  │  ├─ Accordions (collapsible sections)
   │  │  │  │  ├─ Callouts (info, warning, success, error)
   │  │  │  │  └─ Buttons/CTAs
   │  │  │  ├─ Templates predefinidos:
   │  │  │  │  ├─ How-to guide
   │  │  │  │  ├─ Troubleshooting
   │  │  │  │  ├─ FAQ item
   │  │  │  │  └─ Policy/Terms
   │  │  │  └─ Preview (modo usuario)
   │  │  ├─ SEO y metadatos:
   │  │  │  ├─ Meta title
   │  │  │  ├─ Meta description
   │  │  │  ├─ Keywords (para búsqueda)
   │  │  │  └─ Canonical URL
   │  │  ├─ Configuración chatbot:
   │  │  │  ├─ Keywords/Triggers (palabras que activan este artículo):
   │  │  │  │  ├─ Lista keywords (ej: "reembolso", "devolver dinero", "cancelar compra")
   │  │  │  │  ├─ Sinónimos automáticos (IA genera sugerencias)
   │  │  │  │  └─ Regex patterns (avanzado)
   │  │  │  ├─ Intents asociados:
   │  │  │  │  ├─ Select intents entrenados (ej: "request_refund")
   │  │  │  │  └─ Crear nuevo intent
   │  │  │  ├─ Variaciones pregunta (cómo usuarios preguntan esto):
   │  │  │  │  ├─ "¿Cómo pido un reembolso?"
   │  │  │  │  ├─ "Quiero devolver mi compra"
   │  │  │  │  ├─ "¿Puedo cancelar y recuperar mi dinero?"
   │  │  │  │  └─ Auto-generate variations (IA)
   │  │  │  ├─ Respuesta resumida (snippet bot):
   │  │  │  │  ├─ Versión corta para chat (2-3 oraciones)
   │  │  │  │  ├─ Auto-extract del contenido completo
   │  │  │  │  └─ Editable manualmente
   │  │  │  ├─ Quick replies (botones opciones):
   │  │  │  │  ├─ "Ver artículo completo"
   │  │  │  │  ├─ "Iniciar proceso reembolso"
   │  │  │  │  ├─ "Hablar con agente"
   │  │  │  │  └─ Custom buttons
   │  │  │  └─ Contexto aplicable:
   │  │  │     ├─ Roles: Anónimo/Estudiante/Instructor/Admin
   │  │  │     ├─ Estado usuario: Registrado/Trial/Paid/Inactivo
   │  │  │     └─ Páginas específicas (opcional)
   │  │  ├─ Relacionados:
   │  │  │  ├─ Artículos relacionados (sugerir al final)
   │  │  │  ├─ Auto-suggest (basado contenido)
   │  │  │  └─ Manual selection
   │  │  ├─ Traducciones:
   │  │  │  ├─ Agregar versión idioma
   │  │  │  ├─ Status traducción (completa/parcial)
   │  │  │  └─ AI translation assist (sugerir traducción)
   │  │  ├─ Versioning:
   │  │  │  ├─ Historial versiones (cambios, quién, cuándo)
   │  │  │  ├─ Comparar versiones (diff)
   │  │  │  ├─ Restaurar versión anterior
   │  │  │  └─ Changelog público (opcional)
   │  │  └─ Acciones:
   │  │     ├─ Guardar borrador
   │  │     ├─ Preview (como usuario lo verá)
   │  │     ├─ Publicar
   │  │     ├─ Programar publicación
   │  │     └─ Enviar a revisión (workflow aprobación)
   │  │
   │  ├─ **Categorías KB**:
   │  │  ├─ Estructura jerárquica (árbol)
   │  │  ├─ Por categoría:
   │  │  │  ├─ Nombre, Descripción
   │  │  │  ├─ Icono/Emoji
   │  │  │  ├─ # Artículos
   │  │  │  ├─ Orden display
   │  │  │  └─ Visible en portal público (toggle)
   │  │  ├─ Acciones: Crear, Editar, Mover, Eliminar
   │  │  └─ Ejemplos categorías:
   │  │     ├─ Primeros pasos
   │  │     ├─ Cuenta y perfil
   │  │     ├─ Cursos y aprendizaje
   │  │     ├─ Pagos y facturación
   │  │     ├─ Certificados
   │  │     ├─ Problemas técnicos
   │  │     ├─ Para instructores
   │  │     └─ Políticas y términos
   │  │
   │  ├─ **Import/Export**:
   │  │  ├─ Importar artículos:
   │  │  │  ├─ Formatos: CSV, JSON, Markdown files, Notion export
   │  │  │  ├─ Mapping campos
   │  │  │  ├─ Preview antes importar
   │  │  │  └─ Bulk import validation
   │  │  ├─ Exportar KB:
   │  │  │  ├─ Formatos: JSON, CSV, PDF, HTML
   │  │  │  ├─ Filtros: Categoría, idioma
   │  │  │  └─ Backup completo KB
   │  │  └─ Migración desde:
   │  │     ├─ Zendesk
   │  │     ├─ Intercom
   │  │     ├─ Help Scout
   │  │     └─ Confluence
   │  │
   │  └─ **Analytics KB**:
   │     ├─ Artículos más vistos (bot + portal)
   │     ├─ Artículos menos útiles (bajo 👍 rate)
   │     ├─ Artículos nunca mostrados (candidatos eliminar)
   │     ├─ Gaps conocimiento (preguntas sin respuesta)
   │     ├─ Search analytics (qué buscan usuarios en KB)
   │     └─ Suggested improvements (IA)
   │
   ├─ **Entrenamiento Bot (NLP/AI)**:
   │  ├─ **Intents (Intenciones)**:
   │  │  ├─ Lista intents entrenados:
   │  │  │  ├─ Nombre intent (ej: "check_course_price", "request_refund")
   │  │  │  ├─ Descripción
   │  │  │  ├─ # Training phrases (ejemplos)
   │  │  │  ├─ Confidence threshold (mín para activar)
   │  │  │  ├─ Acción asociada (respuesta/artículo/API call)
   │  │  │  ├─ Usage stats (veces detectado, accuracy)
   │  │  │  └─ Estado: Activo/Entrenamiento/Deshabilitado
   │  │  ├─ Crear/Editar intent:
   │  │  │  ├─ Nombre único (snake_case)
   │  │  │  ├─ Display name
   │  │  │  ├─ Descripción clara
   │  │  │  ├─ Training phrases (mínimo 10-20):
   │  │  │  │  ├─ Ejemplos variados cómo usuario expresa intent
   │  │  │  │  ├─ Anotar entities (resaltar parámetros)
   │  │  │  │  ├─ Auto-generate variations (IA)
   │  │  │  │  └─ Import desde conversaciones reales
   │  │  │  ├─ Parameters/Entities:
   │  │  │  │  ├─ Definir entities a extraer (ej: @course_name, @price, @date)
   │  │  │  │  ├─ Tipo: String, Number, Date, Custom entity
   │  │  │  │  ├─ Required/Optional
   │  │  │  │  └─ Prompts si falta (follow-up questions)
   │  │  │  ├─ Respuestas:
   │  │  │  │  ├─ Text responses (múltiples variaciones para naturalidad)
   │  │  │  │  ├─ Rich responses (cards, buttons, images)
   │  │  │  │  ├─ Conditional responses (basado en contexto)
   │  │  │  │  └─ Variables/placeholders: {user_name}, {course_title}
   │  │  │  ├─ Actions (webhooks):
   │  │  │  │  ├─ Call external API
   │  │  │  │  ├─ Query database
   │  │  │  │  ├─ Trigger internal function
   │  │  │  │  └─ Create ticket, Send email
   │  │  │  ├─ Context management:
   │  │  │  │  ├─ Input contexts (requiere contexto previo)
   │  │  │  │  ├─ Output contexts (setea contexto para siguiente)
   │  │  │  │  └─ Lifespan (cuántos turnos persiste)
   │  │  │  └─ Test intent (probar con frases nuevas)
   │  │  └─ Intents predefinidos (templates):
   │  │     ├─ Greetings
   │  │     ├─ Goodbye
   │  │     ├─ Help
   │  │     ├─ Escalate to human
   │  │     ├─ Account questions
   │  │     ├─ Payment issues
   │  │     └─ Technical support
   │  │
   │  ├─ **Entities (Entidades)**:
   │  │  ├─ System entities (predefinidas):
   │  │  │  ├─ @sys.date, @sys.time, @sys.number
   │  │  │  ├─ @sys.email, @sys.phone, @sys.url
   │  │  │  ├─ @sys.currency, @sys.percentage
   │  │  │  └─ @sys.person, @sys.location
   │  │  ├─ Custom entities:
   │  │  │  ├─ Lista entities customizadas:
   │  │  │  │  ├─ Nombre (ej: @course_category, @subscription_plan)
   │  │  │  │  ├─ Tipo: List-based, Regex, AI-learned
   │  │  │  │  ├─ # Valores definidos
   │  │  │  │  └─ Usage en intents
   │  │  │  ├─ Crear entity:
   │  │  │  │  ├─ Nombre único
   │  │  │  │  ├─ Tipo:
   │  │  │  │  │  ├─ **List**: Valores predefinidos con sinónimos
   │  │  │  │  │  │  Ejemplo: @course_category
   │  │  │  │  │  │  - "programación" (synonyms: "coding", "desarrollo", "dev")
   │  │  │  │  │  │  - "diseño" (synonyms: "design", "gráfico")
   │  │  │  │  │  ├─ **Regex**: Patrón (ej: IDs, códigos)
   │  │  │  │  │  └─ **Composite**: Combinación entities
   │  │  │  │  ├─ Fuzzy matching (tolerance typos)
   │  │  │  │  └─ Auto-expand (IA sugiere nuevos valores)
   │  │  │  └─ Bulk import entities (CSV)
   │  │  └─ Validación entities:
   │  │     ├─ Test extraction
   │  │     └─ Conflict detection (ambigüedades)
   │  │
   │  ├─ **Training & Testing**:
   │  │  ├─ Training dashboard:
   │  │  │  ├─ Model version actual
   │  │  │  ├─ Última fecha entrenamiento
   │  │  │  ├─ # Training examples total
   │  │  │  ├─ Accuracy score (validación)
   │  │  │  └─ Status: Trained/Training/Needs retraining
   │  │  ├─ Train model:
   │  │  │  ├─ Botón "Re-entrenar modelo"
   │  │  │  ├─ Progress (puede tomar minutos)
   │  │  │  ├─ Logs entrenamiento
   │  │  │  └─ Notificación al completar
   │  │  ├─ Test console:
   │  │  │  ├─ Input: Escribir frase usuario
   │  │  │  ├─ Output:
   │  │  │  │  ├─ Intent detectado + confidence (%)
   │  │  │  │  ├─ Entities extraídas
   │  │  │  │  ├─ Respuesta generada
   │  │  │  │  ├─ Contexts
   │  │  │  │  └─ Debug info (scoring, alternativos)
   │  │  │  ├─ Batch testing:
   │  │  │  │  ├─ Upload CSV frases test
   │  │  │  │  ├─ Run batch
   │  │  │  │  └─ Report accuracy, confusión matrix
   │  │  │  └─ Guardar como test case (regression testing)
   │  │  ├─ Validation set:
   │  │  │  ├─ Conjunto frases validación (hold-out)
   │  │  │  ├─ Auto-evaluar modelo contra este set
   │  │  │  └─ Precision, Recall, F1-score
   │  │  └─ Continuous learning:
   │  │     ├─ Feedback loop (👍👎 conversaciones)
   │  │     ├─ Sugerencias auto-mejora
   │  │     └─ Periodic retraining (scheduled)
   │  │
   │  ├─ **Conversational flows (Dialogos complejos)**:
   │  │  ├─ Visual flow builder:
   │  │  │  ├─ Canvas drag & drop
   │  │  │  ├─ Nodes:
   │  │  │  │  ├─ Start trigger (intent)
   │  │  │  │  ├─ Message (bot response)
   │  │  │  │  ├─ Question (recoger input)
   │  │  │  │  ├─ Condition (if/else branches)
   │  │  │  │  ├─ Action (webhook, API call)
   │  │  │  │  ├─ Delay (wait X seconds)
   │  │  │  │  └─ End (completion)
   │  │  │  ├─ Connections (flujo lógica)
   │  │  │  └─ Variables (pasar datos entre steps)
   │  │  ├─ Ejemplo flows:
   │  │  │  ├─ Onboarding estudiante nuevo
   │  │  │  ├─ Proceso reembolso paso a paso
   │  │  │  ├─ Troubleshooting video no carga
   │  │  │  ├─ Captura lead (pre-registro)
   │  │  │  └─ Survey satisfacción
   │  │  ├─ Test flow (simulación)
   │  │  └─ Deploy flow (activar/desactivar)
   │  │
   │  └─ **Analytics entrenamiento**:
   │     ├─ Intent confidence distribution
   │     ├─ Unhandled intents (sin match)
   │     ├─ Entity extraction accuracy
   │     ├─ Model performance over time
   │     └─ Suggested training data (AI recommendations)
   │
   ├─ **Configuración chatbot**:
   │  ├─ **Apariencia y branding**:
   │  │  ├─ Tema:
   │  │  │  ├─ Color primario (widget, header)
   │  │  │  ├─ Color secundario (mensajes bot)
   │  │  │  ├─ Color texto
   │  │  │  ├─ Fuente (font family)
   │  │  │  └─ Border radius (redondeado)
   │  │  ├─ Avatar bot:
   │  │  │  ├─ Upload imagen custom
   │  │  │  ├─ Iniciales (ej: "AA" para ACC Assistant)
   │  │  │  ├─ Emoji (ej: 🤖)
   │  │  │  └─ Sin avatar
   │  │  ├─ Nombre bot:
   │  │  │  ├─ Display name (ej: "ACC Assistant", "Sofía")
   │  │  │  └─ Greeting message personalizado
   │  │  ├─ Widget position:
   │  │  │  ├─ Esquina: Inferior derecha/izquierda
   │  │  │  ├─ Offset (pixels desde borde)
   │  │  │  └─ Mobile: Full-screen o minimizable
   │  │  ├─ Launcher button:
   │  │  │  ├─ Estilo: Circular/Cuadrado/Custom
   │  │  │  ├─ Icono: 💬 Chat / ❓ Help / Custom
   │  │  │  ├─ Badge notificación (color)
   │  │  │  └─ Texto hover: "¿Necesitas ayuda?"
   │  │  └─ Preview en vivo (diferentes dispositivos)
   │  │
   │  ├─ **Comportamiento**:
   │  │  ├─ Auto-open:
   │  │  │  ├─ Habilitado/Deshabilitado
   │  │  │  ├─ Delay (segundos después carga página)
   │  │  │  ├─ Solo primera visita usuario
   │  │  │  ├─ Por página específica
   │  │  │  └─ Según comportamiento (tiempo en página, scroll %)
   │  │  ├─ Greeting automático:
   │  │  │  ├─ Mensaje inicial bot (al abrir)
   │  │  │  ├─ Personalizado por:
   │  │  │  │  ├─ Rol usuario
   │  │  │  │  ├─ Hora día ("Buenos días", "Buenas tardes")
   │  │  │  │  ├─ Nuevo vs returning user
   │  │  │  │  └─ Página actual
   │  │  │  └─ Variaciones aleatorias (naturalidad)
   │  │  ├─ Response timing:
   │  │  │  ├─ Typing indicator delay (simular escritura)
   │  │  │  ├─ Min/max delay (ms)
   │  │  │  └─ Variable según longitud respuesta
   │  │  ├─ Fallback behavior:
   │  │  │  ├─ Mensaje "No entendí" (customizable)
   │  │  │  ├─ Sugerir reformular
   │  │  │  ├─ Mostrar sugerencias relacionadas
   │  │  │  ├─ Ofrecer búsqueda KB
   │  │  │  └─ Escalar a humano automático (después X fallos)
   │  │  ├─ Contexto conversación:
   │  │  │  ├─ Persistir conversación (días)
   │  │  │  ├─ Cross-device (mismo usuario)
   │  │  │  └─ Clear history option para usuario
   │  │  └─ Proactive messages:
   │  │     ├─ Habilitar/Deshabilitar
   │  │     ├─ Rules (triggers):
   │  │     │  ├─ Tiempo en página error
   │  │     │  ├─ Abandono checkout
   │  │     │  ├─ Scroll profundidad
   │  │     │  ├─ Exit intent
   │  │     │  └─ Custom events
   │  │     └─ Mensaje por trigger
   │  │
   │  ├─ **Idiomas y localización**:
   │  │  ├─ Idiomas habilitados:
   │  │  │  ├─ Lista idiomas soportados
   │  │  │  ├─ Idioma default
   │  │  │  └─ Fallback language
   │  │  ├─ Detección automática:
   │  │  │  ├─ Browser language
   │  │  │  ├─ Usuario preferencia (perfil)
   │  │  │  └─ Manual selector (usuario elige)
   │  │  ├─ Traducción UI strings:
   │  │  │  ├─ Placeholder input
   │  │  │  ├─ Botones (Enviar, Archivo adjunto)
   │  │  │  ├─ Mensajes sistema
   │  │  │  └─ Import/Export translations (JSON)
   │  │  └─ Soporte multiidioma:
   │  │     ├─ Artículos KB por idioma
   │  │     ├─ Intents entrenar por idioma
   │  │     └─ Auto-translation assist (API)
   │  │
   │  ├─ **Integraciones**:
   │  │  ├─ Soporte/Tickets:
   │  │  │  ├─ Sistema tickets integrado (nativo)
   │  │  │  ├─ Zendesk (API key, subdomain)
   │  │  │  ├─ Intercom (App ID, API token)
   │  │  │  ├─ Freshdesk (domain, API key)
   │  │  │  └─ Auto-crear ticket al escalar
   │  │  ├─ CRM:
   │  │  │  ├─ HubSpot (sync contactos, deals)
   │  │  │  ├─ Salesforce (crear leads)
   │  │  │  └─ Pipedrive
   │  │  ├─ Analytics:
   │  │  │  ├─ Google Analytics (tracking events)
   │  │  │  ├─ Mixpanel (events bot)
   │  │  │  ├─ Segment (track conversaciones)
   │  │  │  └─ Custom events (webhooks)
   │  │  ├─ Notificaciones:
   │  │  │  ├─ Slack (alertas admin, conversaciones)
   │  │  │  ├─ Email (notif conversaciones)
   │  │  │  └─ Webhooks custom
   │  │  └─ APIs externas:
   │  │     ├─ Payment gateway status
   │  │     ├─ Course catalog API
   │  │     ├─ User profile API
   │  │     └─ Custom webhooks (actions)
   │  │
   │  ├─ **Horarios y disponibilidad**:
   │  │  ├─ Modo chatbot:
   │  │  │  ├─ 24/7 Solo bot
   │  │  │  ├─ Business hours: Bot + Humano
   │  │  │  ├─ Fuera horario: Solo bot
   │  │  │  └─ Custom schedule por día
   │  │  ├─ Horario laboral:
   │  │  │  ├─ Lunes-Viernes: 9 AM - 6 PM
   │  │  │  ├─ Timezone
   │  │  │  ├─ Días festivos (calendario)
   │  │  │  └─ Mensaje fuera horario
   │  │  ├─ SLA (Service Level Agreement):
   │  │  │  ├─ Tiempo respuesta bot (target)
   │  │  │  ├─ Tiempo respuesta humano (target)
   │  │  │  └─ Tracking compliance
   │  │  └─ Escalación:
   │  │     ├─ Auto-escalar si bot no resuelve
   │  │     ├─ Threshold intentos (default: 3)
   │  │     ├─ Queue routing (round-robin, skill-based)
   │  │     └─ Priorización (VIP, plan, urgencia)
   │  │
   │  ├─ **Seguridad y privacidad**:
   │  │  ├─ Data retention:
   │  │  │  ├─ Tiempo retención conversaciones (días)
   │  │  │  ├─ Anonimizar después X tiempo
   │  │  │  ├─ Eliminar datos bajo solicitud (GDPR)
   │  │  │  └─ Backup conversations (compliance)
   │  │  ├─ PII (Personally Identifiable Information):
   │  │  │  ├─ Auto-detect y mask PII (emails, teléfonos, tarjetas)
   │  │  │  ├─ Advertencia si usuario comparte info sensible
   │  │  │  └─ Log sanitization
   │  │  ├─ Encryption:
   │  │  │  ├─ Messages in transit (TLS)
   │  │  │  ├─ Messages at rest (database encryption)
   │  │  │  └─ Compliance: GDPR, CCPA, SOC 2
   │  │  ├─ Access control:
   │  │  │  ├─ Quién puede ver conversaciones (roles)
   │  │  │  ├─ Audit log (quién accedió qué)
   │  │  │  └─ Restrict by user attributes
   │  │  ├─ Rate limiting:
   │  │  │  ├─ Mensajes/minuto por usuario
   │  │  │  ├─ Prevenir spam/abuse
   │  │  │  └─ CAPTCHA si sospechoso
   │  │  └─ Content filtering:
   │  │     ├─ Bloquear lenguaje inapropiado
   │  │     ├─ Detect spam/phishing
   │  │     └─ Moderación automática
   │  │
   │  └─ **Avanzado**:
   │     ├─ Custom CSS/JS:
   │     │  ├─ Inyectar CSS custom (widget)
   │     │  ├─ Custom JavaScript (eventos)
   │     │  └─ Widget API (programático)
   │     ├─ A/B testing:
   │     │  ├─ Test variantes respuestas
   │     │  ├─ Test greeting messages
   │     │  └─ Analytics experimentos
   │     ├─ White-label:
   │     │  ├─ Remover "Powered by"
   │     │  ├─ Custom domain bot API
   │     │  └─ Full branding control
   │     └─ API access:
   │        ├─ REST API chatbot
   │        ├─ Webhooks (events)
   │        ├─ SDK/Libraries (JS, Python)
   │        └─ Documentation API
   │
   └─ **Equipo y agentes**:
      ├─ Lista agentes soporte:
      │  ├─ Nombre, Email, Rol
      │  ├─ Estado: Online/Offline/Away
      │  ├─ Skills/Departamentos
      │  ├─ # Conversaciones activas
      │  ├─ # Conversaciones hoy/semana
      │  ├─ Avg response time
      │  ├─ CSAT score individual
      │  └─ Acciones: Editar, Desactivar
      ├─ Crear/Editar agente:
      │  ├─ Info básica (nombre, email)
      │  ├─ Rol: Admin/Agent/Viewer
      │  ├─ Skills (tags expertise)
      │  ├─ Departamentos asignados
      │  ├─ Disponibilidad (horarios)
      │  ├─ Max conversaciones simultáneas
      │  └─ Notificaciones (email, Slack, push)
      ├─ Routing rules:
      │  ├─ Round-robin (equitativo)
      │  ├─ Skill-based (match expertise)
      │  ├─ Load balancing (menos ocupado)
      │  ├─ VIP routing (clientes premium)
      │  └─ Manual assignment
      ├─ Performance dashboard:
      │  ├─ Por agente:
      │  │  ├─ Chats atendidos
      │  │  ├─ Avg response time
      │  │  ├─ Avg resolution time
      │  │  ├─ CSAT score
      │  │  └─ Escalaciones recibidas
      │  └─ Leaderboard (gamificación)
      └─ Training:
         ├─ Onboarding nuevos agentes
         ├─ Documentación interna
         ├─ Best practices chatbot
         └─ Macros/Templates respuestas

---

