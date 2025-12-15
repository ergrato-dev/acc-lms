1️⃣ ANONYMOUS (Visitante No Autenticado)
Página Principal y Marketing

📁 PUBLIC AREA
│
├─ 🔥 / (Home/Landing)
│  ├─ Hero con propuesta de valor
│  ├─ Cursos destacados (carousel)
│  ├─ Testimonios
│  ├─ Stats de la plataforma
│  ├─ CTA: "Explorar cursos" / "Registrarse"
│  └─ Footer: links institucionales
│
├─ 🔥 /catalogo (Catálogo de Cursos)
│  ├─ Grid de cursos publicados
│  ├─ Filtros:
│  │  ├─ Búsqueda por texto
│  │  ├─ Categorías
│  │  ├─ Precio (gratis, de pago, rango)
│  │  ├─ Nivel (principiante, intermedio, avanzado)
│  │  ├─ Idioma
│  │  └─ Rating
│  ├─ Ordenamiento:
│  │  ├─ Más recientes
│  │  ├─ Mejor valorados
│  │  ├─ Más vendidos
│  │  └─ Precio (menor a mayor)
│  ├─ Paginación (20 por página)
│  └─ Estado vacío con sugerencias
│
├─ 🔥 /curso/:slug (Detalle de Curso)
│  ├─ Header: Título, instructor, rating, precio
│  ├─ Video preview/trailer
│  ├─ Descripción completa
│  ├─ Temario completo (solo títulos)
│  │  └─ Lecciones marcadas como "Preview" accesibles
│  ├─ Requisitos previos
│  ├─ Objetivos de aprendizaje
│  ├─ Información del instructor
│  │  ├─ Bio
│  │  ├─ Otros cursos
│  │  └─ Estadísticas (estudiantes, rating)
│  ├─ Reviews y valoraciones
│  │  ├─ Distribución de estrellas
│  │  ├─ Paginación de reviews
│  │  └─ Filtro por rating
│  ├─ FAQ del curso
│  ├─ CTA sticky: "Comprar ahora" / "Matricularme"
│  └─ Cursos relacionados
│
├─ ⚡ /instructor/:id (Perfil Público de Instructor)
│  ├─ Header: foto, nombre, bio
│  ├─ Estadísticas:
│  │  ├─ Total estudiantes
│  │  ├─ Total cursos
│  │  └─ Rating promedio
│  ├─ Cursos publicados
│  ├─ Redes sociales
│  └─ Testimonios de estudiantes
│
└─ ⚡ /buscar (Búsqueda Global)
   ├─ Barra de búsqueda principal
   ├─ Resultados de cursos
   ├─ Filtros avanzados
   ├─ Sugerencias mientras escribes
   └─ Búsqueda semántica (IA - si disponible)

---

Authentication_Registration

📁 AUTH FLOWS
│
├─ 🔥 /registro
│  ├─ Formulario:
│  │  ├─ Email (validación en tiempo real)
│  │  ├─ Contraseña (indicador de fortaleza)
│  │  ├─ Nombre
│  │  ├─ Apellido
│  │  └─ Aceptar términos (obligatorio)
│  ├─ Opciones de registro social (futuro)
│  ├─ Validaciones client-side
│  ├─ Link a "¿Ya tienes cuenta? Inicia sesión"
│  └─ Redirect post-registro: /dashboard o URL original
│
├─ 🔥 /login
│  ├─ Formulario:
│  │  ├─ Email
│  │  ├─ Contraseña
│  │  └─ Checkbox "Recordarme" (30 días)
│  ├─ Link: "¿Olvidaste tu contraseña?"
│  ├─ Link: "¿No tienes cuenta? Regístrate"
│  ├─ Manejo de errores genéricos (seguridad)
│  └─ Redirect post-login: dashboard o URL original
│
├─ ⚡ /recuperar-contrasena
│  ├─ Paso 1: Ingresar email
│  ├─ Paso 2: Mensaje confirmación (genérico)
│  └─ Email con enlace temporal (1h)
│
└─ ⚡ /restablecer-contrasena/:token
   ├─ Validación de token
   ├─ Formulario nueva contraseña
   ├─ Indicador de fortaleza
   └─ Confirmación y redirect a /login

---

Institutional_Pages

📁 INSTITUTIONAL
│
├─ 🎯 /acerca-de
│  ├─ Historia de ACC LMS
│  ├─ Misión y visión
│  ├─ Equipo
│  └─ Valores
│
├─ 🔥 /terminos-y-condiciones
│  ├─ Términos de uso
│  ├─ Última actualización
│  └─ Versiones anteriores
│
├─ 🔥 /politica-de-privacidad
│  ├─ Cumplimiento GDPR/CCPA/LGPD/Habeas Data
│  ├─ Uso de datos
│  ├─ Cookies
│  ├─ Derechos del usuario
│  └─ Contacto DPO
│
├─ ⚡ /preguntas-frecuentes
│  ├─ Categorías:
│  │  ├─ Sobre la plataforma
│  │  ├─ Pagos y reembolsos
│  │  ├─ Cursos y certificados
│  │  └─ Soporte técnico
│  ├─ Búsqueda de preguntas
│  └─ Accordion para respuestas
│
├─ ⚡ /contacto
│  ├─ Formulario de contacto
│  ├─ Email de soporte
│  ├─ Horarios de atención
│  └─ Redes sociales
│
├─ 🎯 /ayuda
│  ├─ Centro de ayuda
│  ├─ Tutoriales
│  ├─ Guías
│  └─ Videos explicativos
│
└─ 💡 /blog (opcional)
   ├─ Artículos educativos
   ├─ Noticias de la plataforma
   ├─ Tips de aprendizaje
   └─ Casos de éxito

