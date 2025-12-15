Componente Global Chatbot

🔥 Widget chatbot (presente en todas las páginas):
   ├─ **Ubicación UI**:
   │  ├─ Botón flotante esquina inferior derecha
   │  ├─ Badge notificación (mensajes sin leer)
   │  ├─ Icono: 💬 (personalizable por rol/contexto)
   │  └─ Click/Tap para expandir
   │
   ├─ **Panel chat (expanded)**:
   │  ├─ Header:
   │  │  ├─ Avatar bot (personalizable)
   │  │  ├─ Nombre: "ACC Assistant" (o nombre custom)
   │  │  ├─ Status: 🟢 En línea | 🟡 Respuesta lenta | Agente humano disponible
   │  │  ├─ Controles:
   │  │  │  ├─ Minimizar
   │  │  │  ├─ Abrir en ventana completa
   │  │  │  └─ Cerrar chat
   │  │  └─ Indicador: "Responde típicamente en <1 min"
   │  │
   │  ├─ **Área conversación**:
   │  │  ├─ Mensajes thread (scroll):
   │  │  │  ├─ Mensaje bot (izquierda, fondo gris claro):
   │  │  │  │  ├─ Avatar bot
   │  │  │  │  ├─ Texto mensaje (Markdown support)
   │  │  │  │  ├─ Timestamp
   │  │  │  │  ├─ Botones acción rápida (si aplica):
   │  │  │  │  │  ├─ "Ver artículo completo"
   │  │  │  │  │  ├─ "Contactar soporte humano"
   │  │  │  │  │  ├─ "Más opciones"
   │  │  │  │  │  └─ Quick replies (chips)
   │  │  │  │  ├─ Rich content:
   │  │  │  │  │  ├─ Cards (curso, artículo KB)
   │  │  │  │  │  ├─ Carousels (múltiples opciones)
   │  │  │  │  │  ├─ Imágenes, GIFs
   │  │  │  │  │  ├─ Videos embebidos
   │  │  │  │  │  ├─ Forms inline (captura datos)
   │  │  │  │  │  └─ Code snippets (con syntax highlighting)
   │  │  │  │  └─ Feedback:
   │  │  │  │     ├─ 👍 👎 (útil/no útil)
   │  │  │  │     └─ Opcional: "¿Por qué no fue útil?"
   │  │  │  │
   │  │  │  └─ Mensaje usuario (derecha, fondo azul):
   │  │  │     ├─ Avatar usuario (si autenticado)
   │  │  │     ├─ Texto mensaje
   │  │  │     ├─ Timestamp
   │  │  │     └─ Estado: Enviado ✓ | Visto ✓✓
   │  │  │
   │  │  ├─ Typing indicator: "ACC Assistant está escribiendo..."
   │  │  ├─ Scroll to bottom (auto-scroll nuevos mensajes)
   │  │  └─ Divider temporal (Hoy, Ayer, fecha específica)
   │  │
   │  ├─ **Input área**:
   │  │  ├─ Textarea mensaje:
   │  │  │  ├─ Placeholder contextual:
   │  │  │  │  ├─ Anónimo: "¿Cómo puedo ayudarte hoy?"
   │  │  │  │  ├─ Estudiante: "Pregunta sobre tus cursos, progreso..."
   │  │  │  │  ├─ Instructor: "Ayuda con tus cursos, estudiantes..."
   │  │  │  │  └─ Admin: "Gestión plataforma, configuración..."
   │  │  │  ├─ Auto-resize (máx 5 líneas)
   │  │  │  ├─ Character counter (si límite)
   │  │  │  └─ Shortcuts: Enter = Enviar, Shift+Enter = Nueva línea
   │  │  ├─ Controles adicionales:
   │  │  │  ├─ 📎 Adjuntar archivo (screenshots, docs)
   │  │  │  ├─ 😊 Emojis (picker)
   │  │  │  ├─ 🎤 Mensaje voz (speech-to-text)
   │  │  │  └─ 📸 Captura pantalla (screenshot helper)
   │  │  └─ Botón enviar (icono ➤)
   │  │
   │  ├─ **Sugerencias inteligentes** (antes primer mensaje):
   │  │  ├─ Por rol/contexto:
   │  │  │  │
   │  │  │  ├─ **ANÓNIMO**:
   │  │  │  │  ├─ "¿Cómo funciona la plataforma?"
   │  │  │  │  ├─ "¿Cómo me registro?"
   │  │  │  │  ├─ "¿Qué cursos ofrecen?"
   │  │  │  │  ├─ "¿Cuánto cuestan los cursos?"
   │  │  │  │  └─ "¿Ofrecen certificados?"
   │  │  │  │
   │  │  │  ├─ **ESTUDIANTE** (contextual según página):
   │  │  │  │  ├─ General:
   │  │  │  │  │  ├─ "¿Cómo veo mi progreso?"
   │  │  │  │  │  ├─ "¿Cómo obtengo mi certificado?"
   │  │  │  │  │  ├─ "Problemas reproduciendo videos"
   │  │  │  │  │  ├─ "¿Cómo contacto al instructor?"
   │  │  │  │  │  └─ "Cancelar suscripción"
   │  │  │  │  ├─ En página curso específico:
   │  │  │  │  │  ├─ "Información sobre este curso"
   │  │  │  │  │  ├─ "¿Cómo me inscribo?"
   │  │  │  │  │  ├─ "Requisitos previos"
   │  │  │  │  │  └─ "Política de reembolso"
   │  │  │  │  └─ En player curso:
   │  │  │  │     ├─ "Video no carga"
   │  │  │  │     ├─ "¿Cómo descargo recursos?"
   │  │  │  │     ├─ "¿Cómo tomo notas?"
   │  │  │  │     └─ "Reportar problema contenido"
   │  │  │  │
   │  │  │  ├─ **INSTRUCTOR**:
   │  │  │  │  ├─ "¿Cómo creo mi primer curso?"
   │  │  │  │  ├─ "Requisitos técnicos videos"
   │  │  │  │  ├─ "¿Cómo subo contenido?"
   │  │  │  │  ├─ "Gestionar estudiantes"
   │  │  │  │  ├─ "Ver mis ganancias"
   │  │  │  │  ├─ "Calificar tareas pendientes"
   │  │  │  │  └─ "Políticas de la plataforma"
   │  │  │  │
   │  │  │  └─ **ADMINISTRADOR**:
   │  │  │     ├─ "Ver health check sistema"
   │  │  │     ├─ "Configurar gateway pago"
   │  │  │     ├─ "Gestionar usuarios"
   │  │  │     ├─ "Generar reporte financiero"
   │  │  │     ├─ "Revisar alertas seguridad"
   │  │  │     └─ "Configurar SMTP"
   │  │  │
   │  │  └─ Click sugerencia = auto-enviar pregunta
   │  │
   │  ├─ **Features conversacionales**:
   │  │  ├─ Contexto persistente (memoria conversación)
   │  │  ├─ Follow-up questions naturales
   │  │  ├─ Multi-turn conversations
   │  │  ├─ Clarificación: "¿Te refieres a X o Y?"
   │  │  ├─ Confirmaciones: "¿Esto resolvió tu duda?"
   │  │  └─ Despedida educada al cerrar
   │  │
   │  ├─ **Escalación a humano**:
   │  │  ├─ Triggers automáticos:
   │  │  │  ├─ Usuario dice: "hablar con humano", "agente", "persona"
   │  │  │  ├─ Bot no encuentra respuesta (confidence <60%)
   │  │  │  ├─ Usuario frustrado (sentiment analysis negativo)
   │  │  │  ├─ Loop conversacional (mismo tema >3 veces)
   │  │  │  └─ Temas sensibles (pagos, cuenta suspendida, legal)
   │  │  ├─ Mensaje transición:
   │  │  │  └─ "Entiendo que necesitas ayuda adicional. 
   │  │  │      Te estoy conectando con un agente de soporte.
   │  │  │      Tiempo estimado de espera: 2-5 minutos"
   │  │  ├─ Queue sistema tickets (crear ticket automático)
   │  │  ├─ Transferencia contexto (historial chat al agente)
   │  │  ├─ Notificación agente disponible
   │  │  └─ Chat continúa con humano (mismo thread)
   │  │
   │  ├─ **Modo offline/horario**:
   │  │  ├─ Fuera horario laboral:
   │  │  │  ├─ Mensaje: "Nuestro equipo está fuera de línea.
   │  │  │  │             Bot disponible 24/7 para preguntas frecuentes."
   │  │  │  ├─ Opción: Dejar mensaje (crea ticket)
   │  │  │  └─ Horario: "Volvemos Lunes 9 AM COT"
   │  │  └─ Sin conexión internet (usuario):
   │  │     ├─ Modo offline (respuestas cacheadas)
   │  │     └─ Sync al reconectar
   │  │
   │  └─ **Footer**:
   │     ├─ "Powered by ACC LMS AI" (branding)
   │     ├─ Links rápidos:
   │     │  ├─ Ver base conocimiento completa
   │     │  ├─ Contacto email soporte
   │     │  └─ Nueva conversación (clear history)
   │     └─ Privacy: "Tus conversaciones son privadas"
   │
   ├─ **Acceso rápido teclado**:
   │  ├─ Ctrl/Cmd + K = Abrir chatbot
   │  ├─ Esc = Cerrar chatbot
   │  └─ / = Focus en input
   │
   ├─ **Notificaciones proactivas** (triggers):
   │  ├─ Usuario lleva >30 seg en página error → "¿Necesitas ayuda?"
   │  ├─ Usuario abandona checkout → "¿Dudas sobre el pago?"
   │  ├─ Primer login estudiante → "¡Bienvenido! ¿Te ayudo a empezar?"
   │  ├─ Video pausado >5 min → "¿Tuviste algún problema?"
   │  └─ Configurable por admin (habilitar/deshabilitar)
   │
   ├─ **Mobile responsive**:
   │  ├─ Full-screen en móvil (mejor UX)
   │  ├─ Botón flotante esquina
   │  ├─ Touch-optimized
   │  └─ Minimizable a burbuja
   │
   └─ **Accesibilidad**:
      ├─ Screen reader compatible
      ├─ Keyboard navigation completa
      ├─ Alto contraste (WCAG AA)
      └─ Focus indicators claros

---

