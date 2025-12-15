Dashboard Administrativo

🔥 /admin/dashboard
   ├─ Bienvenida: "Panel Administrativo - [Nombre Admin]"
   ├─ **Métricas principales** (cards con comparativa período anterior):
   │  ├─ Total usuarios registrados (estudiantes + instructores)
   │  ├─ Usuarios activos (últimos 30 días)
   │  ├─ Total cursos publicados
   │  ├─ Nuevas inscripciones (hoy/semana/mes)
   │  ├─ Ingresos plataforma (total/mes actual)
   │  ├─ Tasa conversión (visitantes → registro → compra)
   │  ├─ Tickets soporte (abiertos/resueltos)
   │  └─ Uptime sistema (%)
   │
   ├─ **Gráficos ejecutivos**:
   │  ├─ Crecimiento usuarios (línea temporal 12 meses)
   │  ├─ Ingresos mensuales (barras, desglose por tipo)
   │  ├─ Top 10 cursos más vendidos
   │  ├─ Distribución geográfica usuarios (mapa)
   │  ├─ Engagement plataforma (DAU/MAU ratio)
   │  └─ Dispositivos/navegadores (pie chart)
   │
   ├─ **Alertas y notificaciones**:
   │  ├─ Sistema: Errores críticos, servidor issues
   │  ├─ Seguridad: Intentos login sospechosos, brechas
   │  ├─ Contenido: Cursos pendiente revisión
   │  ├─ Pagos: Transacciones fallidas, chargebacks
   │  ├─ Usuarios: Reportes abuso, solicitudes soporte urgentes
   │  └─ Infraestructura: Almacenamiento >80%, bandwidth alto
   │
   ├─ **Acciones rápidas**:
   │  ├─ Crear usuario manualmente
   │  ├─ Revisar contenido pendiente
   │  ├─ Ver logs sistema
   │  ├─ Configuración general
   │  └─ Generar reporte ejecutivo
   │
   └─ **Actividad reciente**:
      ├─ Últimas transacciones
      ├─ Nuevos usuarios registrados
      ├─ Cursos publicados/actualizados
      └─ Tickets soporte cerrados

---

Gestión de Usuarios

🔥 /admin/usuarios
   ├─ Tabs:
   │  ├─ Todos | Estudiantes | Instructores | Administradores | Suspendidos | Eliminados
   ├─ Tabla usuarios:
   │  ├─ Columnas:
   │  │  ├─ ID, Avatar, Nombre completo, Email
   │  │  ├─ Rol(es) (badges: EST/INS/ADM)
   │  │  ├─ Estado: Activo/Suspendido/Eliminado/Pendiente verificación
   │  │  ├─ Fecha registro
   │  │  ├─ Última actividad
   │  │  ├─ Cursos (inscritos/creados según rol)
   │  │  ├─ Plan suscripción (si aplica)
   │  │  └─ Acciones rápidas
   │  ├─ Filtros avanzados:
   │  │  ├─ Rol, Estado, Plan
   │  │  ├─ Fecha registro (rango)
   │  │  ├─ País, ciudad
   │  │  ├─ Verificado email (sí/no)
   │  │  ├─ 2FA habilitado (sí/no)
   │  │  └─ Tags personalizados
   │  ├─ Búsqueda: Nombre, email, ID
   │  ├─ Ordenar: Fecha registro, Actividad, Nombre A-Z
   │  └─ Paginación: 25/50/100 por página
   │
   ├─ Acciones masivas (checkbox select):
   │  ├─ Enviar email masivo
   │  ├─ Cambiar rol
   │  ├─ Suspender/Activar cuentas
   │  ├─ Aplicar/Remover tags
   │  ├─ Exportar selección (CSV/Excel)
   │  └─ Eliminar permanente (confirmación doble)
   │
   ├─ Botones toolbar:
   │  ├─ + Crear usuario manualmente
   │  ├─ Importar usuarios (CSV template)
   │  ├─ Exportar todos
   │  └─ Configuración campos personalizados
   │
   └─ Estado vacío: Mensajes apropiados si sin resultados

🔥 /admin/usuario/:userId
   ├─ **Información general**:
   │  ├─ Header: Avatar grande, nombre, email, ID usuario
   │  ├─ Badges: Roles, estado cuenta, verificaciones
   │  ├─ Datos personales:
   │  │  ├─ Nombre, apellido, email (editable)
   │  │  ├─ Teléfono, fecha nacimiento
   │  │  ├─ País, ciudad, zona horaria
   │  │  ├─ Idioma preferido
   │  │  └─ Biografía
   │  ├─ Información cuenta:
   │  │  ├─ Fecha registro, última actividad
   │  │  ├─ IP registro, IPs recientes
   │  │  ├─ Email verificado (fecha, reenviar verificación)
   │  │  ├─ 2FA status
   │  │  └─ Sesiones activas (lista, cerrar remotamente)
   │  ├─ Tags administrativos:
   │  │  ├─ Agregar/remover tags (VIP, Beta tester, Problem user, etc.)
   │  │  └─ Tags visibles solo para admins
   │  └─ Notas internas:
   │     ├─ Timeline notas admin (fecha, admin que escribió)
   │     └─ Agregar nota nueva (texto libre)
   │
   ├─ **Actividad y estadísticas**:
   │  ├─ Timeline actividad completa:
   │  │  ├─ Registro, logins, cursos inscritos/completados
   │  │  ├─ Compras, reembolsos
   │  │  ├─ Posts foro, preguntas
   │  │  ├─ Reportes realizados/recibidos
   │  │  └─ Filtrar por tipo evento
   │  ├─ Si es Estudiante:
   │  │  ├─ Cursos inscritos (lista, progreso, última actividad)
   │  │  ├─ Certificados obtenidos
   │  │  ├─ Tiempo total aprendizaje
   │  │  ├─ Compras realizadas (historial transacciones)
   │  │  └─ Suscripción activa (plan, próximo pago, cancelar)
   │  ├─ Si es Instructor:
   │  │  ├─ Cursos creados (lista, estado, estudiantes)
   │  │  ├─ Total estudiantes impactados
   │  │  ├─ Rating promedio
   │  │  ├─ Ingresos generados (total/mensual)
   │  │  ├─ Pagos recibidos (historial)
   │  │  ├─ Perfil instructor (link ver público)
   │  │  └─ Cursos pendiente aprobación
   │  └─ Engagement:
   │     ├─ Días activo plataforma
   │     ├─ Frecuencia acceso (gráfico)
   │     └─ Features más usadas
   │
   ├─ **Seguridad**:
   │  ├─ Historial logins:
   │  │  ├─ Fecha/hora, IP, ubicación, dispositivo
   │  │  ├─ Exitosos/Fallidos
   │  │  └─ Marcar IPs sospechosas
   │  ├─ Intentos fallidos recientes
   │  ├─ Cambios contraseña (historial fechas)
   │  ├─ Recuperaciones contraseña solicitadas
   │  └─ Dispositivos autorizados
   │
   ├─ **Moderación**:
   │  ├─ Reportes sobre este usuario:
   │  │  ├─ Lista reportes (tipo, reportado por, fecha, estado)
   │  │  ├─ Contenido reportado (comentario, post, review)
   │  │  └─ Historial acciones tomadas
   │  ├─ Reportes realizados por este usuario
   │  ├─ Historial suspensiones/warnings
   │  └─ Contenido eliminado por moderación
   │
   ├─ **Acciones administrativas**:
   │  ├─ Editar información (override cualquier campo)
   │  ├─ Cambiar email (envía confirmación)
   │  ├─ Resetear contraseña (genera link temporal)
   │  ├─ Verificar email manualmente
   │  ├─ Deshabilitar 2FA
   │  ├─ Gestión roles:
   │  │  ├─ Agregar rol (EST/INS/ADM)
   │  │  ├─ Remover rol
   │  │  └─ Permisos granulares (si sistema RBAC avanzado)
   │  ├─ Estado cuenta:
   │  │  ├─ Suspender temporalmente (razón, duración)
   │  │  ├─ Activar cuenta
   │  │  ├─ Eliminar cuenta (soft delete, recuperable 30 días)
   │  │  └─ Eliminar permanente (GDPR compliance)
   │  ├─ Dar acceso especial curso (bypass pago)
   │  ├─ Extender suscripción manualmente
   │  ├─ Reembolsar transacción
   │  ├─ Enviar email personalizado
   │  ├─ Impersonate usuario (login como, para debug)
   │  └─ Exportar datos usuario (GDPR data portability)
   │
   └─ **Comunicaciones**:
      ├─ Emails enviados al usuario (historial)
      ├─ Notificaciones push enviadas
      ├─ Mensajes soporte (tickets asociados)
      └─ Enviar mensaje directo

---

Gestión de Cursos

🔥 /admin/cursos
   ├─ Tabs:
   │  ├─ Todos | Publicados | Borradores | Pendiente revisión | Archivados | Rechazados
   ├─ Tabla cursos:
   │  ├─ Columnas:
   │  │  ├─ ID, Thumbnail, Título
   │  │  ├─ Instructor (nombre, link perfil)
   │  │  ├─ Categoría
   │  │  ├─ Estado: Publicado/Borrador/Revisión/Archivado
   │  │  ├─ Fecha creación/publicación
   │  │  ├─ Total estudiantes inscritos
   │  │  ├─ Rating (estrellas + # reviews)
   │  │  ├─ Precio
   │  │  ├─ Ingresos generados
   │  │  └─ Última actualización
   │  ├─ Filtros:
   │  │  ├─ Estado, Categoría, Nivel
   │  │  ├─ Instructor (dropdown/búsqueda)
   │  │  ├─ Precio: Gratis/Pago
   │  │  ├─ Fecha publicación (rango)
   │  │  ├─ # Estudiantes (rangos)
   │  │  ├─ Rating (≥ 4.5, ≥ 4.0, etc.)
   │  │  └─ Flags: Featured, Destacado, Problema calidad
   │  ├─ Búsqueda: Título, descripción, ID
   │  ├─ Ordenar: Fecha, Estudiantes, Rating, Ingresos, Alfabético
   │  └─ Vista: Grid / Lista
   │
   ├─ Acciones masivas:
   │  ├─ Cambiar estado (publicar/archivar)
   │  ├─ Marcar como destacado
   │  ├─ Aplicar descuento masivo
   │  ├─ Re-categorizar
   │  ├─ Exportar selección
   │  └─ Eliminar (soft delete)
   │
   └─ Botones:
      ├─ Crear curso (como admin en nombre de instructor)
      ├─ Importar curso (migración)
      ├─ Exportar todos (backup)
      └─ Configuración categorías/etiquetas

🔥 /admin/curso/:courseId
   ├─ **Vista general**:
   │  ├─ Header: Thumbnail, título, instructor, estado
   │  ├─ Info básica (ver y editar cualquier campo)
   │  ├─ Temario completo (ver estructura, acceso lecciones)
   │  ├─ Estadísticas:
   │  │  ├─ Total estudiantes (activos/completados)
   │  │  ├─ Tasa completitud
   │  │  ├─ Rating y distribución reviews
   │  │  ├─ Ingresos totales (desglose período)
   │  │  ├─ Tráfico página curso
   │  │  └─ Conversión visitantes → inscripción
   │  └─ Timeline historial (creación, publicación, actualizaciones)
   │
   ├─ **Revisión contenido** (si estado = Pendiente revisión):
   │  ├─ Checklist calidad:
   │  │  ├─ Título/descripción apropiados
   │  │  ├─ Contenido completo (no placeholder)
   │  │  ├─ Videos procesados correctamente
   │  │  ├─ Sin plagio detectado
   │  │  ├─ Cumple políticas plataforma
   │  │  └─ Precio razonable
   │  ├─ Preview curso completo (como estudiante)
   │  ├─ Comentarios internos (entre admins/revisores)
   │  ├─ Notas para instructor
   │  └─ Acciones:
   │     ├─ Aprobar y publicar
   │     ├─ Solicitar cambios (mensaje a instructor, razones específicas)
   │     ├─ Rechazar (razón detallada, no publicable)
   │     └─ Marcar para revisión adicional
   │
   ├─ **Moderación**:
   │  ├─ Reportes sobre curso:
   │  │  ├─ Lista reportes (plagio, contenido inapropiado, spam)
   │  │  ├─ Usuario reportante, fecha, evidencia
   │  │  └─ Estado: Pendiente/Revisado/Acción tomada
   │  ├─ Reviews reportadas (spam, abusivas)
   │  ├─ Foro curso (posts reportados)
   │  └─ Acciones:
   │     ├─ Despublicar temporalmente
   │     ├─ Marcar contenido específico
   │     ├─ Contactar instructor
   │     └─ Banear curso permanente
   │
   ├─ **Acciones administrativas**:
   │  ├─ Editar cualquier campo curso
   │  ├─ Cambiar instructor (transferir propiedad)
   │  ├─ Modificar temario (emergencias)
   │  ├─ Cambiar estado: Publicar/Despublicar/Archivar
   │  ├─ Marcar como destacado (featured)
   │  ├─ Ajustar precio (override instructor)
   │  ├─ Aplicar descuento plataforma
   │  ├─ Fijar en homepage
   │  ├─ Agregar badge (Bestseller, Trending, Editor's Choice)
   │  ├─ Modificar visibilidad (público/privado)
   │  ├─ Acceso SEO avanzado (meta tags custom)
   │  ├─ Ver como estudiante (preview)
   │  ├─ Duplicar curso
   │  ├─ Exportar curso (backup completo)
   │  └─ Eliminar curso (confirmación, afecta estudiantes)
   │
   ├─ **Estudiantes inscritos**:
   │  ├─ Lista completa (igual que vista instructor pero read-only)
   │  ├─ Dar acceso manual estudiante
   │  ├─ Remover estudiante (reembolsar opcional)
   │  └─ Exportar lista
   │
   └─ **Analytics**:
      ├─ Dashboard completo (igual que instructor)
      ├─ Comparativa vs promedio plataforma
      └─ Detección anomalías (fraude, bots)

---

Categorías y Taxonomía

⚡ /admin/categorias
   ├─ Gestión jerárquica categorías:
   │  ├─ Árbol categorías (drag & drop reordenar)
   │  ├─ Categorías principales → Subcategorías → Sub-subcategorías
   │  ├─ Por categoría:
   │  │  ├─ Nombre (multiidioma si aplica)
   │  │  ├─ Slug URL
   │  │  ├─ Descripción
   │  │  ├─ Icono/Imagen
   │  │  ├─ # Cursos asociados
   │  │  ├─ Visible/Oculta
   │  │  ├─ Orden display
   │  │  └─ Meta SEO
   │  ├─ Acciones:
   │  │  ├─ Crear categoría
   │  │  ├─ Editar inline
   │  │  ├─ Mover (cambiar padre)
   │  │  ├─ Fusionar categorías (mover cursos)
   │  │  └─ Eliminar (requiere re-asignar cursos)
   │  └─ Preview: Ver página categoría pública
   │
   ├─ Gestión etiquetas (tags):
   │  ├─ Lista todas las etiquetas
   │  ├─ # Cursos por etiqueta
   │  ├─ Sugerir etiquetas (IA, análisis contenido)
   │  ├─ Fusionar etiquetas similares
   │  ├─ Eliminar tags sin uso
   │  └─ Tags prohibidos/moderados
   │
   └─ Configuración:
      ├─ Máximo niveles anidación
      ├─ Requerir categoría en curso
      └─ Auto-sugerir basado en contenido

---

Gestión Financiera y Pagos

🔥 /admin/finanzas
   ├─ **Dashboard financiero**:
   │  ├─ Período selector: Hoy/Semana/Mes/Trimestre/Año/Personalizado
   │  ├─ Métricas principales:
   │  │  ├─ Ingresos brutos (total transacciones)
   │  │  ├─ Comisión plataforma (% configurado)
   │  │  ├─ Pago a instructores
   │  │  ├─ Reembolsos procesados
   │  │  ├─ Ingresos netos
   │  │  ├─ MRR (Monthly Recurring Revenue - suscripciones)
   │  │  └─ ARR (Annual Recurring Revenue)
   │  ├─ Gráficos:
   │  │  ├─ Ingresos diarios/mensuales (línea)
   │  │  ├─ Desglose por tipo: Cursos/Suscripciones/Otros (pie)
   │  │  ├─ Top 10 cursos ingresos
   │  │  ├─ Top 10 instructores ingresos
   │  │  ├─ Métodos pago utilizados (barras)
   │  │  └─ Geolocalización ingresos (mapa)
   │  └─ Proyecciones:
   │     ├─ Forecast ingresos próximo mes (IA/ML)
   │     └─ Tendencia crecimiento
   │
   ├─ **Transacciones**:
   │  ├─ Tabla todas transacciones:
   │  │  ├─ ID transacción, Fecha/hora
   │  │  ├─ Usuario (comprador)
   │  │  ├─ Producto (curso/suscripción)
   │  │  ├─ Monto, Moneda
   │  │  ├─ Método pago
   │  │  ├─ Estado: Exitosa/Pendiente/Fallida/Reembolsada
   │  │  ├─ Gateway (Stripe/PayPal/etc.)
   │  │  ├─ Comisión plataforma
   │  │  ├─ Pago instructor
   │  │  └─ Acciones: Ver detalle, Reembolsar
   │  ├─ Filtros:
   │  │  ├─ Estado, Método pago, Gateway
   │  │  ├─ Rango fechas, monto
   │  │  ├─ Usuario, Curso, Instructor
   │  │  └─ Moneda
   │  ├─ Búsqueda: ID, email usuario, curso
   │  └─ Exportar: CSV/Excel (declaraciones impuestos)
   │
   ├─ **Reembolsos**:
   │  ├─ Lista solicitudes reembolso:
   │  │  ├─ Usuario, curso, fecha compra
   │  │  ├─ Monto, razón solicitada
   │  │  ├─ Estado: Pendiente/Aprobado/Rechazado
   │  │  └─ Días desde compra
   │  ├─ Acciones:
   │  │  ├─ Aprobar reembolso (total/parcial)
   │  │  ├─ Rechazar (con mensaje)
   │  │  └─ Contactar usuario
   │  ├─ Configuración políticas:
   │  │  ├─ Período elegible reembolso (días)
   │  │  ├─ % progreso máximo para reembolso
   │  │  ├─ Aprobación automática (criterios)
   │  │  └─ Reembolsos parciales permitidos
   │  └─ Estadísticas:
   │     ├─ Tasa reembolso (%)
   │     ├─ Razones principales
   │     └─ Cursos con más reembolsos (alerta calidad)
   │
   ├─ **Pagos a instructores**:
   │  ├─ Ciclo pagos (mensual típicamente):
   │  │  ├─ Ingresos instructor período
   │  │  ├─ Comisión plataforma descontada
   │  │  ├─ Retenciones fiscales (si aplica)
   │  │  ├─ Monto neto a pagar
   │  │  └─ Estado: Pendiente/Procesado/Pagado
   │  ├─ Lista instructores pendiente pago:
   │  │  ├─ Nombre, email
   │  │  ├─ Método pago preferido
   │  │  ├─ Monto adeudado
   │  │  ├─ Umbral mínimo alcanzado (ej. $50 USD)
   │  │  └─ Acciones: Marcar pagado, Retener, Contactar
   │  ├─ Historial pagos realizados
   │  ├─ Generación facturas instructor
   │  ├─ Exportar para procesamiento batch (ACH, SEPA)
   │  └─ Configuración:
   │     ├─ Comisión plataforma (%, o tiers por volumen)
   │     ├─ Umbral mínimo pago
   │     ├─ Frecuencia pagos (semanal/mensual)
   │     └─ Métodos pago soportados
   │
   ├─ **Suscripciones**:
   │  ├─ Lista suscripciones activas:
   │  │  ├─ Usuario, plan, precio
   │  │  ├─ Fecha inicio, próximo cobro
   │  │  ├─ Estado: Activa/Cancelada/Vencida/Trial
   │  │  ├─ Método pago (tarjeta últimos 4 dígitos)
   │  │  └─ Acciones: Cancelar, Extender, Cambiar plan
   │  ├─ Filtros: Plan, estado, método pago
   │  ├─ Métricas suscripciones:
   │  │  ├─ Total activas
   │  │  ├─ Churn rate (cancelaciones)
   │  │  ├─ LTV (Lifetime Value) promedio
   │  │  ├─ MRR por plan
   │  │  └─ Nuevas vs canceladas (gráfico)
   │  └─ Gestión planes:
   │     ├─ Crear/editar planes suscripción
   │     ├─ Precios, features, límites
   │     └─ Trials, descuentos
   │
   ├─ **Cupones y descuentos**:
   │  ├─ Lista cupones plataforma:
   │  │  ├─ Código, descripción
   │  │  ├─ Tipo: % o monto fijo
   │  │  ├─ Validez (fechas)
   │  │  ├─ Usos: Actual/Límite
   │  │  ├─ Restricciones: Cursos, usuarios, primer compra
   │  │  └─ Estado: Activo/Expirado/Pausado
   │  ├─ Analytics cupones:
   │  │  ├─ Conversión por cupón
   │  │  ├─ Revenue generado con cupón
   │  │  └─ ROI campañas
   │  └─ Crear cupón promocional plataforma
   │
   └─ **Reportes fiscales**:
      ├─ Generar reporte ingresos (fecha range)
      ├─ Desglose impuestos por región
      ├─ Certificados retención
      ├─ Exportar para contador (múltiples formatos)
      └─ Cumplimiento: IVA/Sales Tax por jurisdicción

---

Reportes y Analytics Plataforma

🔥 /admin/analytics
   ├─ **Selector período global**: Hoy/7días/30días/90días/Año/Todo/Personalizado
   ├─ **Dashboard ejecutivo**:
   │  ├─ KPIs principales (comparativa período anterior):
   │  │  ├─ Total usuarios (crecimiento %)
   │  │  ├─ DAU/MAU/WAU (Daily/Monthly/Weekly Active Users)
   │  │  ├─ Tasa retención (cohortes)
   │  │  ├─ Tiempo promedio sesión
   │  │  ├─ Pages per session
   │  │  ├─ Bounce rate
   │  │  └─ Conversión registro → compra (funnel)
   │  ├─ Gráficos clave:
   │  │  ├─ Tráfico sitio (sesiones, pageviews, usuarios únicos)
   │  │  ├─ Crecimiento usuarios (línea acumulativa)
   │  │  ├─ Engagement: Distribución sesiones/usuario (histograma)
   │  │  ├─ Heatmap actividad (hora/día semana)
   │  │  └─ Device breakdown (desktop/mobile/tablet)
   │  └─ Exportar dashboard (PDF ejecutivo)
   │
   ├─ **Análisis usuarios**:
   │  ├─ Demografía:
   │  │  ├─ Distribución geográfica (mapa interactivo)
   │  │  ├─ Top 20 países/ciudades
   │  │  ├─ Idiomas navegador
   │  │  ├─ Zonas horarias
   │  │  └─ Edad promedio (si dato disponible)
   │  ├─ Comportamiento:
   │  │  ├─ Análisis cohortes (retención por semana registro)
   │  │  ├─ Funnel conversión completo:
   │  │  │  Visitante → Registro → Exploración → Compra → Activación → Retención
   │  │  ├─ User journey más común (flow diagram)
   │  │  ├─ Tiempo hasta primera compra
   │  │  ├─ Frecuencia retorno
   │  │  └─ Features más/menos usadas
   │  ├─ Segmentación:
   │  │  ├─ Por tipo: Free vs Paid users
   │  │  ├─ Por engagement: Power users vs Casual vs Churned
   │  │  ├─ Por valor: Segmentos LTV (alto/medio/bajo)
   │  │  └─ Custom segments (crear filtros complejos)
   │  └─ Churn analysis:
   │     ├─ Tasa churn mensual
   │     ├─ Predicción churn (ML model)
   │     ├─ Razones churn (encuestas salida)
   │     └─ Segmentos alto riesgo
   │
   ├─ **Análisis cursos**:
   │  ├─ Performance global:
   │  │  ├─ Total cursos publicados (tendencia)
   │  │  ├─ Tasa aprobación (revisión)
   │  │  ├─ Tiempo promedio creación curso
   │  │  ├─ Distribución por categoría
   │  │  └─ Nivel cursos (principiante/intermedio/avanzado)
   │  ├─ Engagement cursos:
   │  │  ├─ Tasa inscripción promedio
   │  │  ├─ Tasa completitud global
   │  │  ├─ Promedio estudiantes/curso
   │  │  ├─ Tiempo promedio completar curso
   │  │  └─ Cursos con mejores/peores métricas
   │  ├─ Rankings:
   │  │  ├─ Top 50 cursos (estudiantes, ingresos, rating, completitud)
   │  │  ├─ Cursos tendencia (crecimiento rápido)
   │  │  ├─ Cursos problema (bajo engagement, alta tasa abandono)
   │  │  └─ Nuevos cursos prometedores
   │  └─ Contenido:
   │     ├─ Tipo lecciones (video/texto/quiz distribución)
   │     ├─ Duración promedio cursos
   │     ├─ # Lecciones promedio
   │     └─ Uso multimedia (videos, PDFs, etc.)
   │
   ├─ **Análisis instructores**:
   │  ├─ Estadísticas generales:
   │  │  ├─ Total instructores activos
   │  │  ├─ Nuevos instructores/mes
   │  │  ├─ Promedio cursos/instructor
   │  │  ├─ Distribución: 1 curso, 2-5, 6-10, 10+
   │  │  └─ Ratio estudiantes/instructor
   │  ├─ Performance:
   │  │  ├─ Top instructores (estudiantes, ingresos, rating)
   │  │  ├─ Instructores inactivos (sin publicar X meses)
   │  │  ├─ Tasa respuesta foro promedio
   │  │  ├─ Engagement con estudiantes
   │  │  └─ Calificación promedio contenido
   │  └─ Ingresos:
   │     ├─ Revenue por instructor (distribución)
   │     ├─ Top earners
   │     ├─ Comisiones pagadas total
   │     └─ Power law analysis (80/20 rule)
   │
   ├─ **Análisis financiero** (expandido):
   │  ├─ Revenue dashboard:
   │  │  ├─ Ingresos totales (gráfico temporal múltiples métricas)
   │  │  ├─ Desglose: Cursos individuales/Suscripciones/Otros
   │  │  ├─ ARPU (Average Revenue Per User)
   │  │  ├─ ARPPU (Average Revenue Per Paying User)
   │  │  └─ Conversión free → paid (%)
   │  ├─ Forecasting:
   │  │  ├─ Predicción ingresos próximos 3/6/12 meses
   │  │  ├─ Estacionalidad ventas
   │  │  └─ Tendencias crecimiento
   │  ├─ Análisis precios:
   │  │  ├─ Distribución precios cursos
   │  │  ├─ Elasticidad precio (experimentos)
   │  │  ├─ Descuentos efectividad
   │  │  └─ Optimal pricing recomendaciones
   │  └─ Unit economics:
   │     ├─ CAC (Customer Acquisition Cost)
   │     ├─ LTV (Lifetime Value)
   │     ├─ LTV:CAC ratio
   │     ├─ Payback period
   │     └─ Gross margin
   │
   ├─ **Análisis marketing**:
   │  ├─ Acquisition:
   │  │  ├─ Fuentes tráfico (orgánico, directo, referral, social, paid)
   │  │  ├─ Canales conversión (cual convierte mejor)
   │  │  ├─ Campañas activas performance
   │  │  ├─ SEO: Rankings keywords, tráfico orgánico
   │  │  └─ UTM tracking (campañas específicas)
   │  ├─ Engagement campaigns:
   │  │  ├─ Email marketing: Open rate, click rate, conversión
   │  │  ├─ Push notifications: Delivery, open, action
   │  │  └─ In-app messages: Views, clicks
   │  └─ Virality:
   │     ├─ Referral program stats
   │     ├─ Social shares (cursos compartidos)
   │     ├─ Viral coefficient (K-factor)
   │     └─ Organic growth rate
   │
   ├─ **Análisis técnico**:
   │  ├─ Performance:
   │  │  ├─ Page load times (promedio, p50, p95, p99)
   │  │  ├─ API response times
   │  │  ├─ Database query performance
   │  │  ├─ CDN hit rate
   │  │  └─ Video buffering rate
   │  ├─ Errores:
   │  │  ├─ Tasa error global (%)
   │  │  ├─ Errores por endpoint (top)
   │  │  ├─ Errores frontend (JavaScript)
   │  │  ├─ Failed transactions
   │  │  └─ Crash reports (mobile apps)
   │  ├─ Infraestructura:
   │  │  ├─ Uptime (SLA compliance)
   │  │  ├─ CPU/Memory/Disk usage
   │  │  ├─ Bandwidth consumido
   │  │  ├─ Storage usado (videos, archivos)
   │  │  └─ Database size/growth
   │  └─ Browser/Device:
   │     ├─ Navegadores (Chrome, Safari, Firefox, etc.)
   │     ├─ OS (Windows, macOS, iOS, Android)
   │     ├─ Resoluciones pantalla
   │     └─ Versiones mobile apps
   │
   ├─ **Reportes customizados**:
   │  ├─ Query builder visual:
   │  │  ├─ Seleccionar métricas (multi-select)
   │  │  ├─ Filtros complejos (AND/OR)
   │  │  ├─ Agrupación (por fecha, país, curso, etc.)
   │  │  ├─ Ordenamiento
   │  │  └─ Preview resultados
   │  ├─ Guardar reportes favoritos
   │  ├─ Programar envío automático (email, Slack)
   │  └─ Compartir reportes (link público, dashboard embed)
   │
   └─ **Exportaciones**:
      ├─ Formatos: CSV, Excel, JSON, PDF
      ├─ Data warehouse export (BigQuery, Snowflake)
      ├─ Scheduled exports
      └─ API access datos analytics

---

Configuración del Sistema

🔥 /admin/configuracion
   ├─ **General**:
   │  ├─ Información plataforma:
   │  │  ├─ Nombre plataforma
   │  │  ├─ Logo (múltiples tamaños: header, favicon, email)
   │  │  ├─ Tagline/Slogan
   │  │  ├─ Descripción breve
   │  │  ├─ Email contacto general
   │  │  ├─ Teléfono soporte
   │  │  └─ Redes sociales oficiales (URLs)
   │  ├─ URLs y dominios:
   │  │  ├─ URL principal (ej: https://acc-lms.com)
   │  │  ├─ Subdominios (api, cdn, admin)
   │  │  ├─ SSL/HTTPS enforcement
   │  │  └─ Redirecciones (www → no-www)
   │  ├─ Zona horaria servidor (default)
   │  ├─ Idioma default plataforma
   │  ├─ Moneda default (USD/COP/MXN/etc.)
   │  └─ Modo mantenimiento:
   │     ├─ Activar/Desactivar
   │     ├─ Mensaje personalizado
   │     ├─ Whitelist IPs (admin access durante mantenimiento)
   │     └─ Programar mantenimiento futuro
   │
   ├─ **Localización e internacionalización**:
   │  ├─ Idiomas habilitados:
   │  │  ├─ Lista idiomas activos (ES, EN, PT, etc.)
   │  │  ├─ Agregar nuevo idioma
   │  │  ├─ Idioma default
   │  │  └─ Fallback language
   │  ├─ Traducciones:
   │  │  ├─ Interface strings (editor inline)
   │  │  ├─ Emails templates por idioma
   │  │  ├─ Páginas estáticas (términos, privacidad)
   │  │  ├─ Importar/Exportar traducciones (JSON)
   │  │  └─ Progreso traducción por idioma (%)
   │  ├─ Formatos regionales:
   │  │  ├─ Fecha (DD/MM/YYYY vs MM/DD/YYYY)
   │  │  ├─ Hora (12h vs 24h)
   │  │  ├─ Números (separadores decimales/miles)
   │  │  └─ Moneda (símbolo posición)
   │  └─ Detección automática idioma (browser/IP)
   │
   ├─ **Email**:
   │  ├─ Configuración SMTP:
   │  │  ├─ Host, Puerto, Usuario, Contraseña
   │  │  ├─ Encriptación (TLS/SSL)
   │  │  ├─ Email "From" (nombre y dirección)
   │  │  ├─ Reply-to address
   │  │  └─ Test connection (enviar email prueba)
   │  ├─ Proveedores alternativos:
   │  │  ├─ SendGrid (API key)
   │  │  ├─ Mailgun (domain, API key)
   │  │  ├─ AWS SES (credentials)
   │  │  └─ Seleccionar proveedor activo
   │  ├─ Templates emails transaccionales:
   │  │  ├─ Bienvenida, Verificación email
   │  │  ├─ Recuperación contraseña
   │  │  ├─ Confirmación compra, Factura
   │  │  ├─ Inscripción curso
   │  │  ├─ Recordatorios, Notificaciones
   │  │  └─ Editor visual (HTML + variables dinámicas)
   │  ├─ Configuración envío:
   │  │  ├─ Rate limiting (emails/hora)
   │  │  ├─ Retry policy (fallos)
   │  │  ├─ Bounce handling
   │  │  └─ Unsubscribe link (automático)
   │  └─ Logs emails:
   │     ├─ Historial emails enviados
   │     ├─ Estado: Enviado/Abierto/Clicked/Bounced/Spam
   │     └─ Filtros: Destinatario, tipo, fecha
   │
   ├─ **Autenticación y seguridad**:
   │  ├─ Métodos autenticación:
   │  │  ├─ Email/Password (habilitado siempre)
   │  │  ├─ OAuth providers:
   │  │  │  ├─ Google (Client ID, Secret)
   │  │  │  ├─ Facebook
   │  │  │  ├─ GitHub
   │  │  │  ├─ LinkedIn
   │  │  │  └─ Apple Sign In
   │  │  ├─ SSO Enterprise:
   │  │  │  ├─ SAML 2.0 (metadata XML)
   │  │  │  ├─ LDAP/Active Directory
   │  │  │  └─ Okta integration
   │  │  └─ Magic Links (passwordless)
   │  ├─ Políticas contraseña:
   │  │  ├─ Longitud mínima (default 8)
   │  │  ├─ Requerir: Mayúsculas, minúsculas, números, símbolos
   │  │  ├─ Prevenir contraseñas comunes (diccionario)
   │  │  ├─ Historial contraseñas (no reutilizar últimas X)
   │  │  └─ Expiración contraseña (días, opcional)
   │  ├─ 2FA (Two-Factor Authentication):
   │  │  ├─ Habilitar/Deshabilitar globalmente
   │  │  ├─ Forzar 2FA para admins
   │  │  ├─ Métodos: TOTP (Google Authenticator), SMS, Email
   │  │  └─ Códigos backup
   │  ├─ Sesiones:
   │  │  ├─ Duración sesión (minutos inactividad)
   │  │  ├─ "Remember me" duración (días)
   │  │  ├─ Sesiones concurrentes permitidas
   │  │  ├─ Invalidar sesiones remotas (logout todos dispositivos)
   │  │  └─ Session storage (Redis, database)
   │  ├─ Rate limiting:
   │  │  ├─ Login attempts (máx intentos/tiempo, bloqueo temporal)
   │  │  ├─ API calls (por endpoint, por usuario)
   │  │  ├─ Password reset requests
   │  │  └─ Whitelist IPs (sin límite)
   │  ├─ CAPTCHA:
   │  │  ├─ Habilitar en: Registro, Login (después X fallos), Contacto
   │  │  ├─ Proveedor: reCAPTCHA v2/v3, hCaptcha
   │  │  └─ API keys configuración
   │  └─ Security headers:
   │     ├─ CORS policy (allowed origins)
   │     ├─ CSP (Content Security Policy)
   │     ├─ HSTS (HTTP Strict Transport Security)
   │     └─ X-Frame-Options, X-Content-Type-Options
   │
   ├─ **Roles y permisos**:
   │  ├─ Roles predefinidos: Admin, Instructor, Estudiante
   │  ├─ Crear roles personalizados:
   │  │  ├─ Nombre rol
   │  │  ├─ Descripción
   │  │  └─ Permisos granulares (checkboxes):
   │  │     ├─ Usuarios: Ver, Crear, Editar, Eliminar, Suspender
   │  │     ├─ Cursos: Ver, Crear, Editar, Publicar, Eliminar, Moderar
   │  │     ├─ Contenido: Agregar, Modificar, Eliminar
   │  │     ├─ Finanzas: Ver reportes, Procesar pagos, Reembolsos
   │  │     ├─ Configuración: Ver, Modificar
   │  │     ├─ Analytics: Ver, Exportar
   │  │     └─ Soporte: Ver tickets, Responder, Cerrar
   │  ├─ Asignar roles a usuarios
   │  ├─ Roles múltiples por usuario (herencia permisos)
   │  └─ Audit trail: Log cambios permisos
   │
   ├─ **Pagos y monetización**:
   │  ├─ Gateways pago habilitados:
   │  │  ├─ Stripe:
   │  │  │  ├─ Publishable key, Secret key
   │  │  │  ├─ Webhook URL, Secret
   │  │  │  ├─ Modos: Test/Live
   │  │  │  └─ Métodos: Cards, ACH, wallets
   │  │  ├─ PayPal:
   │  │  │  ├─ Client ID, Secret
   │  │  │  ├─ Sandbox/Production
   │  │  │  └─ IPN listener
   │  │  ├─ MercadoPago (LATAM):
   │  │  │  ├─ Access token
   │  │  │  ├─ Países habilitados
   │  │  │  └─ Métodos locales (PSE, Oxxo, etc.)
   │  │  └─ Otros: Razorpay, Paddle, etc.
   │  ├─ Configuración monetización:
   │  │  ├─ Comisión plataforma:
   │  │  │  ├─ Porcentaje fijo (ej: 20%)
   │  │  │  ├─ Tiers por volumen (>$1000 → 15%)
   │  │  │  └─ Mínimo por transacción
   │  │  ├─ Umbral pago instructor (mín. para pagar)
   │  │  ├─ Frecuencia pagos (semanal/mensual)
   │  │  ├─ Período retención (días antes pagar instructor)
   │  │  └─ Métodos pago a instructores (PayPal, transferencia)
   │  ├─ Impuestos:
   │  │  ├─ Habilitar cálculo automático impuestos
   │  │  ├─ Tax IDs por región (VAT, Sales Tax)
   │  │  ├─ Tasas impuesto por país/estado
   │  │  ├─ Precio incluye impuesto (toggle)
   │  │  └─ Integración TaxJar/Avalara
   │  ├─ Monedas soportadas:
   │  │  ├─ Lista monedas activas
   │  │  ├─ Tasas cambio (auto-actualización API)
   │  │  └─ Display multi-moneda
   │  └─ Políticas reembolso:
   │     ├─ Período elegible (días)
   │     ├─ Progreso máximo (%)
   │     ├─ Aprobación automática criterios
   │     └─ Texto política pública
   │
   ├─ **Multimedia y almacenamiento**:
   │  ├─ Proveedores storage:
   │  │  ├─ Local (servidor)
   │  │  ├─ AWS S3 (bucket, region, keys)
   │  │  ├─ Google Cloud Storage
   │  │  ├─ Azure Blob Storage
   │  │  ├─ DigitalOcean Spaces
   │  │  └─ Cloudflare R2
   │  ├─ CDN:
   │  │  ├─ Cloudflare (zone, API token)
   │  │  ├─ AWS CloudFront
   │  │  ├─ Fastly
   │  │  └─ Purge cache (manual/automático)
   │  ├─ Videos:
   │  │  ├─ Procesamiento:
   │  │  │  ├─ Transcoding automático (resolutions: 1080p, 720p, 480p, 360p)
   │  │  │  ├─ HLS streaming habilitado
   │  │  │  ├─ Thumbnails generación (cada X segundos)
   │  │  │  └─ Proveedor: Local (FFmpeg), AWS MediaConvert, Mux, Vimeo
   │  │  ├─ Seguridad:
   │  │  │  ├─ DRM (Digital Rights Management)
   │  │  │  ├─ Signed URLs (expiran)
   │  │  │  ├─ Hotlink protection
   │  │  │  └─ Watermark overlay
   │  │  └─ Límites:
   │  │     ├─ Tamaño máximo archivo (GB)
   │  │     ├─ Duración máxima (horas)
   │  │     └─ Formatos aceptados
   │  ├─ Archivos:
   │  │  ├─ Formatos permitidos upload:
   │  │  │  ├─ Imágenes: JPG, PNG, GIF, WebP, SVG
   │  │  │  ├─ Documentos: PDF, DOCX, PPTX, XLSX
   │  │  │  ├─ Código: ZIP, RAR
   │  │  │  └─ Blacklist extensiones (executable, scripts)
   │  │  ├─ Tamaño máximo por tipo
   │  │  ├─ Virus scan (ClamAV, VirusTotal)
   │  │  └─ Image optimization (compresión automática)
   │  └─ Quotas almacenamiento:
   │     ├─ Por usuario (estudiante/instructor)
   │     ├─ Por curso
   │     ├─ Total plataforma (alertas >80%)
   │     └─ Políticas retención (eliminar archivos huérfanos)
   │
   ├─ **Notificaciones push**:
   │  ├─ Proveedores:
   │  │  ├─ Firebase Cloud Messaging (FCM):
   │  │  │  ├─ Server key, Sender ID
   │  │  │  └─ Service account JSON
   │  │  ├─ OneSignal (App ID, API key)
   │  │  ├─ Pusher (credentials)
   │  │  └─ Web Push (VAPID keys)
   │  ├─ Configuración notificaciones:
   │  │  ├─ Tipos habilitados (por categoría)
   │  │  ├─ Quiet hours (no enviar entre X-Y hora)
   │  │  ├─ Frecuencia límite (no spam)
   │  │  └─ Templates notificaciones
   │  └─ Test envío (a dispositivo específico)
   │
   └─ **Integraciones terceros**:
      ├─ Analytics:
      │  ├─ Google Analytics (Tracking ID)
      │  ├─ Mixpanel (API token)
      │  ├─ Amplitude
      │  └─ Hotjar (Site ID)
      ├─ Marketing:
      │  ├─ Mailchimp (API key, audience)
      │  ├─ HubSpot (API key)
      │  └─ Facebook Pixel
      ├─ Comunicación:
      │  ├─ Slack (webhook URL, notificaciones admin)
      │  ├─ Discord (webhook)
      │  └─ Telegram bot
      ├─ Soporte:
      │  ├─ Intercom (App ID, Secret)
      │  ├─ Zendesk (subdomain, API token)
      │  └─ Crisp
      ├─ Videoconferencias:
      │  ├─ Zoom (API Key, Secret, JWT)
      │  ├─ Google Meet (OAuth)
      │  └─ Microsoft Teams
      ├─ Desarrollo:
      │  ├─ GitHub (OAuth App, webhooks)
      │  ├─ GitLab
      │  └─ Bitbucket
      └─ Otros:
         ├─ Zapier (webhook endpoints)
         ├─ Make (Integromat)
         └─ Custom webhooks

---

Seguridad y Auditoría

🔥 /admin/seguridad
   ├─ **Dashboard seguridad**:
   │  ├─ Score seguridad global (0-100):
   │  │  ├─ Basado en: 2FA adoption, password strength, vulnerabilities, compliance
   │  │  ├─ Recomendaciones mejora (accionables)
   │  │  └─ Tendencia score (últimos 30 días)
   │  ├─ Alertas críticas (tiempo real):
   │  │  ├─ Intentos acceso no autorizados
   │  │  ├─ Múltiples fallos login mismo usuario/IP
   │  │  ├─ Actividad inusual (ej: admin login desde país nuevo)
   │  │  ├─ Cambios configuración crítica
   │  │  ├─ Vulnerabilidades detectadas
   │  │  └─ Certificados SSL próximos expirar
   │  ├─ Métricas período (7/30/90 días):
   │  │  ├─ Intentos login fallidos
   │  │  ├─ Cuentas bloqueadas
   │  │  ├─ IPs bloqueadas
   │  │  ├─ 2FA adoption rate (%)
   │  │  ├─ Sesiones sospechosas detectadas
   │  │  └─ Incidentes seguridad reportados
   │  └─ Último scan seguridad (fecha, resultado)
   │
   ├─ **Logs de auditoría**:
   │  ├─ Vista principal logs:
   │  │  ├─ Tabla eventos:
   │  │  │  ├─ Timestamp, Usuario (actor), IP, Ubicación
   │  │  │  ├─ Acción/Evento (ej: "Login exitoso", "Usuario eliminado", "Config modificada")
   │  │  │  ├─ Recurso afectado (usuario, curso, configuración)
   │  │  │  ├─ Detalles (JSON expandible)
   │  │  │  ├─ Nivel: Info/Warning/Error/Critical
   │  │  │  ├─ User Agent (navegador/dispositivo)
   │  │  │  └─ Request ID (trazabilidad)
   │  │  ├─ Filtros avanzados:
   │  │  │  ├─ Rango fechas/horas (precisión minuto)
   │  │  │  ├─ Usuario/Rol (dropdown multi-select)
   │  │  │  ├─ Tipo evento:
   │  │  │  │  ├─ Autenticación (login, logout, 2FA, password change)
   │  │  │  │  ├─ Usuarios (CRUD, suspensión, cambio rol)
   │  │  │  │  ├─ Cursos (creación, edición, publicación, eliminación)
   │  │  │  │  ├─ Contenido (upload, modificación, eliminación)
   │  │  │  │  ├─ Financiero (transacciones, reembolsos, pagos)
   │  │  │  │  ├─ Configuración (cambios sistema)
   │  │  │  │  ├─ Permisos (cambio roles, permisos)
   │  │  │  │  ├─ API (llamadas, errores)
   │  │  │  │  └─ Seguridad (intentos fallidos, bloqueos, vulnerabilidades)
   │  │  │  ├─ Nivel gravedad (Info/Warning/Error/Critical)
   │  │  │  ├─ IP address/rango
   │  │  │  ├─ País/Ciudad
   │  │  │  ├─ Recurso afectado (ID, tipo)
   │  │  │  └─ Búsqueda texto libre (contenido evento)
   │  │  ├─ Ordenar: Fecha (asc/desc), Gravedad
   │  │  ├─ Paginación inteligente (virtual scroll grandes volúmenes)
   │  │  └─ Live updates (WebSocket, eventos tiempo real)
   │  │
   │  ├─ Vista detalle evento (modal/drawer):
   │  │  ├─ Información completa:
   │  │  │  ├─ ID evento único
   │  │  │  ├─ Timestamp exacto (ms)
   │  │  │  ├─ Actor: Usuario completo (nombre, email, rol)
   │  │  │  ├─ Acción descriptiva
   │  │  │  ├─ Recurso: Tipo, ID, nombre/título
   │  │  │  ├─ Cambios (diff):
   │  │  │  │  ├─ Valores anteriores vs nuevos (JSON diff visual)
   │  │  │  │  └─ Campos modificados highlighted
   │  │  │  ├─ Metadata:
   │  │  │  │  ├─ IP address (+ geolocalización en mapa)
   │  │  │  │  ├─ User Agent completo
   │  │  │  │  ├─ Dispositivo (desktop/mobile/tablet)
   │  │  │  │  ├─ OS y versión
   │  │  │  │  ├─ Navegador y versión
   │  │  │  │  ├─ Request ID (correlación logs)
   │  │  │  │  └─ Session ID
   │  │  │  ├─ Contexto adicional (variables ambiente, headers)
   │  │  │  └─ Stack trace (si error)
   │  │  ├─ Eventos relacionados:
   │  │  │  ├─ Timeline eventos mismo usuario (antes/después)
   │  │  │  ├─ Eventos mismo recurso
   │  │  │  └─ Eventos misma sesión
   │  │  ├─ Acciones:
   │  │  │  ├─ Copiar detalles (JSON)
   │  │  │  ├─ Marcar como revisado
   │  │  │  ├─ Agregar nota investigación
   │  │  │  ├─ Crear alerta similar
   │  │  │  ├─ Exportar evento (PDF/JSON)
   │  │  │  └─ Reportar incidente (escalar)
   │  │  └─ AI insights (futuro):
   │  │     ├─ "Patrón inusual detectado"
   │  │     ├─ "Usuario accedió desde nueva ubicación"
   │  │     └─ "Acción fuera horario habitual"
   │  │
   │  ├─ Análisis y reportes:
   │  │  ├─ Eventos por usuario (top activos)
   │  │  ├─ Eventos por tipo (distribución)
   │  │  ├─ Actividad por hora/día (heatmap)
   │  │  ├─ Ubicaciones acceso (mapa geográfico)
   │  │  ├─ Dispositivos más usados
   │  │  ├─ Acciones más frecuentes
   │  │  └─ Anomalías detectadas
   │  │
   │  ├─ Exportaciones:
   │  │  ├─ Formato: CSV, JSON, PDF (reporte)
   │  │  ├─ Rango: Filtros aplicados
   │  │  ├─ Programar exportación automática (compliance)
   │  │  └─ Enviar a SIEM (Splunk, ELK, etc.)
   │  │
   │  └─ Configuración auditoría:
   │     ├─ Eventos a registrar (selectivo para performance):
   │     │  ├─ Habilitar/deshabilitar por categoría
   │     │  ├─ Nivel mínimo gravedad
   │     │  └─ Excluir usuarios/IPs (ej: bots monitoreo)
   │     ├─ Retención logs:
   │     │  ├─ Tiempo retención (días/meses/años)
   │     │  ├─ Archivar logs antiguos (S3 Glacier)
   │     │  └─ Cumplimiento legal (GDPR: mín 6 meses, máx según región)
   │     ├─ Almacenamiento:
   │     │  ├─ Base datos principal
   │     │  ├─ ElasticSearch (búsquedas rápidas)
   │     │  ├─ Archivo externo (compliance)
   │     │  └─ Uso almacenamiento actual
   │     └─ Notificaciones:
   │        ├─ Email admins eventos críticos
   │        ├─ Slack/webhooks integración
   │        └─ Umbral alertas (X eventos/minuto)
   │
   ├─ **Gestión de accesos**:
   │  ├─ Sesiones activas (todas plataforma):
   │  │  ├─ Lista global sesiones:
   │  │  │  ├─ Usuario, Rol
   │  │  │  ├─ Inicio sesión (timestamp)
   │  │  │  ├─ Última actividad
   │  │  │  ├─ IP, Ubicación, Dispositivo
   │  │  │  ├─ Navegador
   │  │  │  └─ Inactividad (minutos)
   │  │  ├─ Filtros: Rol, Inactivas >X min, País, Dispositivo
   │  │  ├─ Ordenar: Inicio, Última actividad, Usuario
   │  │  ├─ Acciones:
   │  │  │  ├─ Terminar sesión individual
   │  │  │  ├─ Terminar todas sesiones usuario
   │  │  │  ├─ Bloquear IP
   │  │  │  └─ Terminar sesiones masivas (filtradas)
   │  │  └─ Métricas:
   │  │     ├─ Total sesiones activas
   │  │     ├─ Distribución por rol
   │  │     └─ Promedio duración sesión
   │  │
   │  ├─ Intentos de login:
   │  │  ├─ Tabla intentos:
   │  │  │  ├─ Timestamp, Email/Usuario, IP, País
   │  │  │  ├─ Estado: Exitoso/Fallido
   │  │  │  ├─ Razón fallo (password incorrecto, usuario no existe, 2FA, etc.)
   │  │  │  ├─ User Agent
   │  │  │  └─ Bloqueado (si alcanzó límite intentos)
   │  │  ├─ Filtros: Estado, IP, Usuario, Rango fechas
   │  │  ├─ Vista fallos consecutivos:
   │  │  │  ├─ Usuarios con múltiples fallos (posible ataque)
   │  │  │  ├─ IPs con múltiples usuarios (credential stuffing)
   │  │  │  └─ Patrones bot (User Agent, timing)
   │  │  ├─ Acciones:
   │  │  │  ├─ Bloquear IP manualmente
   │  │  │  ├─ Resetear contador fallos usuario
   │  │  │  ├─ Whitelist IP
   │  │  │  └─ Agregar a firewall (automático)
   │  │  └─ Analytics:
   │  │     ├─ Tasa éxito login (%)
   │  │     ├─ Picos intentos (gráfico temporal)
   │  │     └─ Top IPs atacantes
   │  │
   │  ├─ IPs bloqueadas:
   │  │  ├─ Lista negra (blacklist):
   │  │  │  ├─ IP/CIDR, Razón bloqueo, Fecha, Bloqueado por (admin/auto)
   │  │  │  ├─ Tipo: Temporal/Permanente
   │  │  │  ├─ Expiración (si temporal)
   │  │  │  └─ # Intentos que causaron bloqueo
   │  │  ├─ Acciones:
   │  │  │  ├─ Agregar IP manualmente
   │  │  │  ├─ Importar lista IPs (CSV)
   │  │  │  ├─ Desbloquear IP
   │  │  │  ├─ Editar duración bloqueo
   │  │  │  └─ Bloquear rango (CIDR)
   │  │  ├─ Listas públicas integración:
   │  │  │  ├─ Spamhaus
   │  │  │  ├─ AbuseIPDB
   │  │  │  ├─ Tor exit nodes
   │  │  │  └─ Auto-sync (diario)
   │  │  └─ Whitelist IPs (nunca bloquear):
   │  │     ├─ IPs oficina/admin
   │  │     ├─ APIs confiables
   │  │     └─ Partners/integraciones
   │  │
   │  └─ API Keys y tokens:
   │     ├─ Lista API keys activas:
   │     │  ├─ Key (truncada, mostrar/ocultar)
   │     │  ├─ Usuario propietario
   │     │  ├─ Nombre/Descripción
   │     │  ├─ Permisos (scopes)
   │     │  ├─ Fecha creación, Última uso
   │     │  ├─ Expiración (si aplica)
   │     │  ├─ # Llamadas (total, últimas 24h)
   │     │  └─ Rate limit aplicado
   │     ├─ Acciones:
   │     │  ├─ Crear nueva API key
   │     │  ├─ Regenerar key
   │     │  ├─ Revocar key
   │     │  ├─ Modificar permisos
   │     │  └─ Ver uso detallado (logs llamadas)
   │     ├─ Tokens OAuth:
   │     │  ├─ Lista tokens terceros (Google, GitHub, etc.)
   │     │  ├─ Revocar acceso
   │     │  └─ Auditar permisos concedidos
   │     └─ Configuración:
   │        ├─ Rate limits por key
   │        ├─ Expiración default tokens
   │        └─ Requerir IP whitelist (opcional)
   │
   ├─ **Escaneo vulnerabilidades**:
   │  ├─ Último scan:
   │  │  ├─ Fecha/hora ejecución
   │  │  ├─ Duración scan
   │  │  ├─ Cobertura (URLs/endpoints escaneados)
   │  │  ├─ Resultado general: Crítico/Alto/Medio/Bajo/Info
   │  │  └─ Score seguridad (A-F grade)
   │  ├─ Vulnerabilidades detectadas:
   │  │  ├─ Lista vulnerabilidades:
   │  │  │  ├─ ID, Nombre, Categoría (OWASP Top 10)
   │  │  │  ├─ Severidad: Critical/High/Medium/Low
   │  │  │  ├─ Descripción, Impacto potencial
   │  │  │  ├─ Ubicación (URL, endpoint, código)
   │  │  │  ├─ CVE ID (si aplica)
   │  │  │  ├─ CVSS score
   │  │  │  ├─ Estado: Nuevo/Confirmado/En progreso/Resuelto/Falso positivo
   │  │  │  ├─ Fecha detección
   │  │  │  ├─ Asignado a (dev/admin)
   │  │  │  └─ Prioridad
   │  │  ├─ Categorías comunes:
   │  │  │  ├─ SQL Injection
   │  │  │  ├─ XSS (Cross-Site Scripting)
   │  │  │  ├─ CSRF (Cross-Site Request Forgery)
   │  │  │  ├─ Authentication bypass
   │  │  │  ├─ Sensitive data exposure
   │  │  │  ├─ Broken access control
   │  │  │  ├─ Security misconfiguration
   │  │  │  ├─ Insecure dependencies (outdated libraries)
   │  │  │  └─ Server-side request forgery (SSRF)
   │  │  ├─ Acciones por vulnerabilidad:
   │  │  │  ├─ Ver detalles técnicos completos
   │  │  │  ├─ Prueba concepto (PoC)
   │  │  │  ├─ Recomendación remediación
   │  │  │  ├─ Crear ticket (Jira, GitHub Issues)
   │  │  │  ├─ Cambiar estado/prioridad
   │  │  │  ├─ Asignar responsable
   │  │  │  ├─ Agregar notas
   │  │  │  ├─ Marcar falso positivo
   │  │  │  └─ Re-escanear específico
   │  │  └─ Filtros: Severidad, Estado, Categoría, Fecha
   │  ├─ Dependencias inseguras:
   │  │  ├─ Frontend (npm packages):
   │  │  │  ├─ Paquete, Versión actual, Vulnerabilidad
   │  │  │  ├─ Versión segura recomendada
   │  │  │  ├─ Severidad
   │  │  │  └─ CVE/Advisory link
   │  │  ├─ Backend (Go modules, Python packages):
   │  │  │  ├─ Similar estructura
   │  │  │  └─ Comando actualización sugerido
   │  │  └─ Acción: Generar PR automático actualización (GitHub integration)
   │  ├─ Configuración scans:
   │  │  ├─ Herramientas:
   │  │  │  ├─ OWASP ZAP (API)
   │  │  │  ├─ Snyk (dependency scanning)
   │  │  │  ├─ SonarQube (code quality + security)
   │  │  │  ├─ npm audit / Go vulnerability DB
   │  │  │  └─ Custom scripts
   │  │  ├─ Frecuencia:
   │  │  │  ├─ Manual (on-demand)
   │  │  │  ├─ Programado (diario/semanal)
   │  │  │  ├─ CI/CD pipeline (cada deploy)
   │  │  │  └─ Trigger: Al detectar cambios críticos
   │  │  ├─ Alcance scan:
   │  │  │  ├─ URLs incluidas/excluidas
   │  │  │  ├─ Profundidad crawling
   │  │  │  ├─ Autenticado (con credenciales test)
   │  │  │  └─ Pruebas activas (invasivas) o pasivas
   │  │  └─ Notificaciones:
   │  │     ├─ Email al completar scan
   │  │     ├─ Alerta críticos inmediata
   │  │     └─ Slack/webhook integración
   │  └─ Historial scans:
   │     ├─ Lista scans anteriores
   │     ├─ Comparativa (más/menos vulnerabilidades)
   │     ├─ Tendencia seguridad (gráfico temporal)
   │     └─ Exportar reportes
   │
   ├─ **Compliance y certificaciones**:
   │  ├─ **GDPR (General Data Protection Regulation)**:
   │  │  ├─ Checklist cumplimiento:
   │  │  │  ├─ ✓ Consent management (cookies, marketing)
   │  │  │  ├─ ✓ Right to access (usuarios pueden descargar datos)
   │  │  │  ├─ ✓ Right to erasure ("derecho al olvido")
   │  │  │  ├─ ✓ Data portability (exportar JSON/CSV)
   │  │  │  ├─ ✓ Privacy by design
   │  │  │  ├─ ✓ Data breach notification (<72h)
   │  │  │  ├─ ✓ DPO designado (Data Protection Officer)
   │  │  │  └─ ✓ Registro procesamiento datos
   │  │  ├─ Gestión solicitudes:
   │  │  │  ├─ Data access requests (SAR - Subject Access Request)
   │  │  │  ├─ Deletion requests
   │  │  │  ├─ Rectification requests
   │  │  │  └─ Tracking estado (pendiente/procesado, <30 días)
   │  │  ├─ Inventario datos personales:
   │  │  │  ├─ Qué datos recolectamos
   │  │  │  ├─ Base legal procesamiento
   │  │  │  ├─ Tiempo retención
   │  │  │  ├─ Terceros que acceden
   │  │  │  └─ Medidas seguridad
   │  │  └─ Registro incidentes (data breaches)
   │  │
   │  ├─ **CCPA/CPRA (California Consumer Privacy Act)**:
   │  │  ├─ "Do Not Sell My Info" (opt-out)
   │  │  ├─ Disclosure categorías datos recolectados
   │  │  ├─ Deletion requests (similar GDPR)
   │  │  └─ Non-discrimination policy
   │  │
   │  ├─ **LGPD (Brasil - Lei Geral de Proteção de Dados)**:
   │  │  ├─ Similar GDPR
   │  │  ├─ Consentimiento específico
   │  │  └─ Agente tratamiento datos designado
   │  │
   │  ├─ **COPPA (Children's Online Privacy Protection)**:
   │  │  ├─ Verificación edad (no <13 años sin consentimiento parental)
   │  │  ├─ Parental consent management
   │  │  └─ Restricciones datos menores
   │  │
   │  ├─ **SOC 2 / ISO 27001** (si aplica):
   │  │  ├─ Controles seguridad documentados
   │  │  ├─ Políticas acceso
   │  │  ├─ Gestión incidentes
   │  │  ├─ Business continuity plan
   │  │  └─ Auditorías periódicas
   │  │
   │  ├─ **PCI-DSS** (si procesan tarjetas):
   │  │  ├─ No almacenar CVV
   │  │  ├─ Tokenización datos pago
   │  │  ├─ Encriptación tránsito/reposo
   │  │  ├─ Logs acceso cardholder data
   │  │  └─ Quarterly vulnerability scans
   │  │
   │  └─ Reportes compliance:
   │     ├─ Generar reporte cumplimiento (PDF)
   │     ├─ Evidencias auditoría
   │     ├─ Políticas actualizadas
   │     └─ Certificados vigentes
   │
   ├─ **Gestión incidentes seguridad**:
   │  ├─ Registro incidentes:
   │  │  ├─ Lista incidentes:
   │  │  │  ├─ ID, Título, Severidad (P0/P1/P2/P3)
   │  │  │  ├─ Tipo: Data breach, DDoS, Malware, Acceso no autorizado, etc.
   │  │  │  ├─ Estado: Detectado/Investigando/Contenido/Resuelto/Post-mortem
   │  │  │  ├─ Fecha detección, Reportado por
   │  │  │  ├─ Asignado a (incident commander)
   │  │  │  ├─ Impacto: # usuarios afectados, datos expuestos
   │  │  │  └─ SLA respuesta (tiempo transcurrido)
   │  │  ├─ Vista detalle incidente:
   │  │  │  ├─ Descripción completa
   │  │  │  ├─ Timeline eventos (cronología)
   │  │  │  ├─ Afectación (alcance, usuarios, datos)
   │  │  │  ├─ Vectores ataque / Causa raíz
   │  │  │  ├─ Acciones tomadas (contención, remediación)
   │  │  │  ├─ Comunicaciones (interna/externa/legal)
   │  │  │  ├─ Evidencias (logs, screenshots, dumps)
   │  │  │  ├─ Equipo respuesta (roles)
   │  │  │  └─ Post-mortem document
   │  │  ├─ Acciones workflow:
   │  │  │  ├─ Escalar severidad
   │  │  │  ├─ Asignar equipo respuesta
   │  │  │  ├─ Notificar stakeholders
   │  │  │  ├─ Actualizar estado
   │  │  │  ├─ Documentar acciones
   │  │  │  └─ Cerrar con lessons learned
   │  │  └─ Templates respuesta:
   │  │     ├─ Playbook DDoS
   │  │     ├─ Playbook data breach
   │  │     ├─ Playbook ransomware
   │  │     └─ Comunicado usuarios (draft)
   │  ├─ Notificaciones obligatorias:
   │  │  ├─ Usuarios afectados (GDPR: <72h)
   │  │  ├─ Autoridades regulatorias
   │  │  ├─ DPO / Legal
   │  │  └─ Tracking entrega notificaciones
   │  ├─ Métricas incidentes:
   │  │  ├─ MTTD (Mean Time To Detect)
   │  │  ├─ MTTR (Mean Time To Respond)
   │  │  ├─ MTTR (Mean Time To Resolve)
   │  │  ├─ # Incidentes por mes
   │  │  └─ Severidad distribución
   │  └─ Simulacros (tabletop exercises):
   │     ├─ Programar simulacros
   │     ├─ Escenarios (data breach, DDoS, insider threat)
   │     └─ Evaluar respuesta equipo
   │
   └─ **Configuración avanzada seguridad**:
      ├─ WAF (Web Application Firewall):
      │  ├─ Provider: Cloudflare, AWS WAF, ModSecurity
      │  ├─ Reglas habilitadas:
      │  │  ├─ OWASP Core Rule Set
      │  │  ├─ SQL Injection protection
      │  │  ├─ XSS protection
      │  │  ├─ Rate limiting
      │  │  ├─ Geo-blocking
      │  │  └─ Custom rules
      │  ├─ Modo: Monitor/Block
      │  └─ Logs WAF (eventos bloqueados)
      ├─ DDoS protection:
      │  ├─ Provider configuración
      │  ├─ Umbral detección
      │  ├─ Challenge mode (CAPTCHA)
      │  └─ Estadísticas ataques mitigados
      ├─ SSL/TLS:
      │  ├─ Certificados instalados
      │  ├─ Renovación automática (Let's Encrypt)
      │  ├─ Forzar HTTPS (redirect)
      │  ├─ TLS versión mínima (1.2+)
      │  ├─ Cipher suites (secure only)
      │  └─ HSTS preload
      ├─ Backups seguridad:
      │  ├─ Encriptación backups (AES-256)
      │  ├─ Almacenamiento offsite
      │  ├─ Acceso restringido
      │  └─ Test restauración regular
      └─ Políticas seguridad:
         ├─ Documento políticas (versión, última actualización)
         ├─ Aceptación políticas (empleados/admins)
         ├─ Revisión periódica (anual)
         └─ Training seguridad obligatorio

---

Soporte y Tickets

⚡ /admin/soporte
   ├─ **Dashboard soporte**:
   │  ├─ Métricas principales (período):
   │  │  ├─ Tickets abiertos/cerrados/pendientes
   │  │  ├─ Tiempo promedio primera respuesta
   │  │  ├─ Tiempo promedio resolución
   │  │  ├─ Satisfacción cliente (CSAT score)
   │  │  ├─ Tickets por categoría (distribución)
   │  │  └─ Agentes más activos
   │  ├─ Gráficos:
   │  │  ├─ Volumen tickets (línea temporal)
   │  │  ├─ Estado distribución (pie)
   │  │  ├─ SLA compliance (%)
   │  │  └─ Backlog growth/reduction
   │  └─ Alertas:
   │     ├─ Tickets sin responder >X horas
   │     ├─ SLA breach inminente
   │     └─ Picos volumen inusuales
   │
   ├─ **Gestión tickets**:
   │  ├─ Vista principal:
   │  │  ├─ Tabs: Mis tickets | Todos | Sin asignar | Urgentes | Cerrados
   │  │  ├─ Tabla tickets:
   │  │  │  ├─ ID, Asunto, Usuario
   │  │  │  ├─ Categoría: Técnico, Billing, Cuenta, Curso, Otro
   │  │  │  ├─ Prioridad: Baja/Media/Alta/Crítica (color-coded)
   │  │  │  ├─ Estado: Nuevo/Abierto/Pendiente usuario/Resuelto/Cerrado
   │  │  │  ├─ Asignado a (agente)
   │  │  │  ├─ Fecha creación, Última actualización
   │  │  │  ├─ SLA: Tiempo restante (countdown)
   │  │  │  └─ # Respuestas
   │  │  ├─ Filtros: Estado, Prioridad, Categoría, Asignado, Fecha
   │  │  ├─ Búsqueda: ID, email usuario, asunto, contenido
   │  │  └─ Acciones masivas: Asignar, Cambiar estado, Cerrar
   │  │
   │  ├─ Vista detalle ticket:
   │  │  ├─ Header:
   │  │  │  ├─ ID ticket, Asunto (editable)
   │  │  │  ├─ Usuario: Nombre, email, link perfil
   │  │  │  ├─ Estado (dropdown cambio rápido)
   │  │  │  ├─ Prioridad (dropdown)
   │  │  │  ├─ Categoría (dropdown)
   │  │  │  └─ Tags (agregar/remover)
   │  │  ├─ Timeline conversación:
   │  │  │  ├─ Mensajes ordenados cronológicamente
   │  │  │  ├─ Usuario vs Agente diferenciados
   │  │  │  ├─ Timestamp cada mensaje
   │  │  │  ├─ Adjuntos inline (ver/descargar)
   │  │  │  └─ Notas internas (solo visibles agentes)
   │  │  ├─ Editor respuesta:
   │  │  │  ├─ WYSIWYG editor
   │  │  │  ├─ Templates respuestas rápidas (macros)
   │  │  │  ├─ Menciones @agente (notificación)
   │  │  │  ├─ Adjuntar archivos, screenshots
   │  │  │  ├─ Insertar links, imágenes
   │  │  │  ├─ Code blocks (si técnico)
   │  │  │  └─ Botones: Responder, Responder y cerrar, Nota interna
   │  │  ├─ Sidebar info:
   │  │  │  ├─ Asignado a (cambiar agente)
   │  │  │  ├─ Seguidores (CCs, notificaciones)
   │  │  │  ├─ Tiempo primera respuesta
   │  │  │  ├─ Tiempo total abierto
   │  │  │  ├─ SLA: Vence en... (visual)
   │  │  │  ├─ Historial cambios estado
   │  │  │  └─ Tickets relacionados (mismo usuario)
   │  │  ├─ Contexto usuario:
   │  │  │  ├─ Cursos inscritos
   │  │  │  ├─ Última actividad plataforma
   │  │  │  ├─ Plan suscripción
   │  │  │  ├─ Tickets previos
   │  │  │  └─ Valor LTV
   │  │  ├─ Acciones:
   │  │  │  ├─ Escalar ticket
   │  │  │  ├─ Fusionar tickets duplicados
   │  │  │  ├─ Convertir en bug/feature request
   │  │  │  ├─ Programar seguimiento
   │  │  │  ├─ Solicitar info adicional usuario
   │  │  │  └─ Cerrar ticket (con razón)
   │  │  └─ AI assist (futuro):
   │  │     ├─ Sugerir respuesta (basado en KB)
   │  │     ├─ Auto-categorización
   │  │     ├─ Detectar sentiment (frustrado/neutral/satisfecho)
   │  │     └─ Artículos KB relacionados
   │  │
   │  └─ Configuración tickets:
   │     ├─ Categorías customizables
   │     ├─ Prioridades y SLA por categoría
   │     ├─ Auto-asignación reglas
   │     ├─ Email templates
   │     └─ Satisfacción encuesta (post-cierre)
   │
   ├─ **Knowledge Base (Base de Conocimiento)**:
   │  ├─ Artículos ayuda:
   │  │  ├─ Lista artículos (categorías jerárquicas)
   │  │  ├─ Título, categoría, autor, fecha
   │  │  ├─ Publicado/Borrador
   │  │  ├─ # Vistas, # Útil/No útil
   │  │  └─ Idioma
   │  ├─ Editor artículo:
   │  │  ├─ Título, slug URL
   │  │  ├─ Contenido (Markdown/WYSIWYG)
   │  │  ├─ Categoría, tags
   │  │  ├─ SEO: Meta description
   │  │  ├─ Artículos relacionados
   │  │  ├─ Video embed (opcional)
   │  │  ├─ Archivos adjuntos
   │  │  └─ Preview público
   │  ├─ Gestión categorías KB
   │  ├─ Analytics:
   │  │  ├─ Artículos más vistos
   │  │  ├─ Búsquedas sin resultados (oportunidades)
   │  │  ├─ Artículos que resuelven tickets (conversión)
   │  │  └─ Feedback usuarios
   │  └─ Configuración:
   │     ├─ Portal público KB (activar/desactivar)
   │     ├─ Multiidioma
   │     └─ Búsqueda semántica
   │
   └─ **Reportes soporte**:
      ├─ Performance agentes:
      │  ├─ # Tickets resueltos
      │  ├─ Tiempo promedio respuesta/resolución
      │  ├─ CSAT score
      │  └─ Ranking agentes
      ├─ Análisis tendencias:
      │  ├─ Categorías más reportadas
      │  ├─ Problemas recurrentes
      │  └─ Estacionalidad volumen
      └─ Exportar reportes (PDF/CSV)

---

Contenido y Moderación

🔥 /admin/moderacion
   ├─ **Dashboard moderación**:
   │  ├─ Métricas principales (período):
   │  │  ├─ Contenido pendiente revisión (total)
   │  │  ├─ Reportes abiertos (usuarios)
   │  │  ├─ Acciones moderación tomadas (aprobados/rechazados/removidos)
   │  │  ├─ Tiempo promedio revisión
   │  │  ├─ Precisión decisiones (appeals exitosos vs total)
   │  │  └─ Contenido flagged automáticamente (IA)
   │  ├─ Queues (colas trabajo):
   │  │  ├─ Cursos pendiente aprobación (badge contador)
   │  │  ├─ Lecciones reportadas
   │  │  ├─ Comentarios/Posts foro flagged
   │  │  ├─ Reviews reportadas
   │  │  ├─ Perfiles usuarios sospechosos
   │  │  └─ Mensajes privados reportados
   │  ├─ Alertas prioritarias:
   │  │  ├─ Contenido ilegal detectado (CSAM, violencia extrema)
   │  │  ├─ Spam masivo detectado
   │  │  ├─ Cuentas bot sospechosas
   │  │  ├─ Plagio detectado (similaridad >80%)
   │  │  └─ Reportes múltiples mismo contenido
   │  ├─ Actividad moderadores:
   │  │  ├─ Lista moderadores activos
   │  │  ├─ # Revisiones por moderador (hoy/semana)
   │  │  ├─ Precisión decisiones
   │  │  └─ Tiempo promedio por revisión
   │  └─ Gráficos:
   │     ├─ Volumen reportes (línea temporal)
   │     ├─ Tipos violaciones (pie chart)
   │     ├─ Acciones tomadas distribución
   │     └─ Tasa apelaciones (%)
   │
   ├─ **Queue revisión cursos**:
   │  ├─ Filtros y vistas:
   │  │  ├─ Estado: Pendiente primera revisión | Re-revisión | Apelación
   │  │  ├─ Prioridad (auto-calculada):
   │  │  │  ├─ Instructor premium/verificado (alta)
   │  │  │  ├─ Tiempo en queue (urgente si >7 días)
   │  │  │  ├─ Curso pre-venta (alta prioridad)
   │  │  │  └─ Reportes previos instructor (baja prioridad)
   │  │  ├─ Categoría curso
   │  │  ├─ Asignado a (moderador)
   │  │  └─ Ordenar: Fecha envío, Prioridad, Instructor
   │  ├─ Lista cursos revisión:
   │  │  ├─ Card curso:
   │  │  │  ├─ Thumbnail, Título
   │  │  │  ├─ Instructor (nombre, rating histórico, # cursos previos)
   │  │  │  ├─ Categoría, Nivel
   │  │  │  ├─ Duración total, # Lecciones
   │  │  │  ├─ Precio propuesto
   │  │  │  ├─ Fecha envío revisión
   │  │  │  ├─ Tiempo en queue (días/horas)
   │  │  │  ├─ Flags automáticos (si aplica):
   │  │  │  │  ├─ 🚩 Plagio detectado (%)
   │  │  │  │  ├─ 🚩 Contenido inapropiado (IA)
   │  │  │  │  ├─ 🚩 Calidad video baja
   │  │  │  │  └─ 🚩 Audio problemas detectados
   │  │  │  └─ Acciones: Revisar, Asignarme, Ver preview
   │  │  └─ Asignación automática (round-robin entre moderadores)
   │  │
   │  ├─ Panel revisión curso (detalle):
   │  │  ├─ **Sección 1: Información básica**
   │  │  │  ├─ Título curso (verificar apropiado, no spam)
   │  │  │  ├─ Descripción (completa, coherente, no misleading)
   │  │  │  ├─ Categoría correcta
   │  │  │  ├─ Objetivos aprendizaje (claros, realistas)
   │  │  │  ├─ Requisitos previos (apropiados)
   │  │  │  └─ Thumbnail (calidad, apropiado, no clickbait)
   │  │  │
   │  │  ├─ **Sección 2: Contenido curricular**
   │  │  │  ├─ Temario completo (vista árbol):
   │  │  │  │  ├─ Navegación rápida secciones/lecciones
   │  │  │  │  ├─ Player video inline (cada lección)
   │  │  │  │  ├─ Controles: 0.5x-2x speed, skip 10s
   │  │  │  │  ├─ Visualizador artículos (Markdown rendered)
   │  │  │  │  └─ Preview evaluaciones (quiz/tareas)
   │  │  │  ├─ Checklist calidad por lección:
   │  │  │  │  ├─ ✓ Audio claro (no ruido, volumen adecuado)
   │  │  │  │  ├─ ✓ Video calidad (mín 720p, enfocado)
   │  │  │  │  ├─ ✓ Iluminación adecuada
   │  │  │  │  ├─ ✓ Sin errores técnicos (cortes, glitches)
   │  │  │  │  ├─ ✓ Contenido original (no plagiado)
   │  │  │  │  ├─ ✓ Información precisa (fact-checking)
   │  │  │  │  ├─ ✓ Ritmo apropiado (no muy rápido/lento)
   │  │  │  │  ├─ ✓ Transiciones claras
   │  │  │  │  └─ ✓ Pantalla compartida legible (si aplica)
   │  │  │  ├─ Duración total vs prometido
   │  │  │  ├─ Completitud (no placeholder content)
   │  │  │  └─ Coherencia estructura (flujo lógico)
   │  │  │
   │  │  ├─ **Sección 3: Verificaciones automáticas**
   │  │  │  ├─ Análisis plagio:
   │  │  │  │  ├─ Comparación con cursos existentes (texto)
   │  │  │  │  ├─ Búsqueda web (contenido público)
   │  │  │  │  ├─ Fingerprint video (comparación visual)
   │  │  │  │  └─ Resultado: % similaridad + fuente
   │  │  │  ├─ Content filtering (IA):
   │  │  │  │  ├─ Lenguaje inapropiado detectado
   │  │  │  │  ├─ Imágenes violentas/sexuales/sensibles
   │  │  │  │  ├─ Discurso odio
   │  │  │  │  ├─ Desinformación (fact-check APIs)
   │  │  │  │  └─ Confidence score (0-100%)
   │  │  │  ├─ Quality checks técnicos:
   │  │  │  │  ├─ Resolución videos (min/max/promedio)
   │  │  │  │  ├─ Bitrate audio/video
   │  │  │  │  ├─ Duración archivos cargados vs declarados
   │  │  │  │  └─ Formatos válidos
   │  │  │  └─ SEO spam detection:
   │  │  │     ├─ Keyword stuffing
   │  │  │     ├─ Links spam (descripción)
   │  │  │     └─ Texto duplicado
   │  │  │
   │  │  ├─ **Sección 4: Historial instructor**
   │  │  │  ├─ Cursos previos publicados
   │  │  │  ├─ Rating promedio histórico
   │  │  │  ├─ Rechazos anteriores (razones)
   │  │  │  ├─ Warnings/strikes
   │  │  │  ├─ Reportes usuarios (si hay)
   │  │  │  └─ Comportamiento patrón (confiable/sospechoso)
   │  │  │
   │  │  ├─ **Sección 5: Pricing y monetización**
   │  │  │  ├─ Precio propuesto (comparar similar categoría)
   │  │  │  ├─ Relación precio/valor (duración, calidad)
   │  │  │  ├─ Descuentos programados
   │  │  │  └─ Pricing razonable (no excesivo/sospechoso)
   │  │  │
   │  │  ├─ **Panel decisión**:
   │  │  │  ├─ Checklist final moderador:
   │  │  │  │  ├─ ☐ Cumple políticas contenido
   │  │  │  │  ├─ ☐ Calidad técnica aceptable
   │  │  │  │  ├─ ☐ No plagio detectado
   │  │  │  │  ├─ ☐ Información precisa
   │  │  │  │  ├─ ☐ Precio razonable
   │  │  │  │  ├─ ☐ Categorización correcta
   │  │  │  │  └─ ☐ Listo para publicar
   │  │  │  ├─ Notas moderador (privadas, historial)
   │  │  │  ├─ Tags categorización:
   │  │  │  │  ├─ Calidad: Excelente/Buena/Aceptable/Mejorable
   │  │  │  │  ├─ Flags: Featured candidate, Needs improvement, Watch closely
   │  │  │  │  └─ Custom tags
   │  │  │  └─ Acciones:
   │  │  │     ├─ **Aprobar y publicar**:
   │  │  │     │  ├─ Publicar inmediatamente
   │  │  │     │  ├─ Marcar como "Featured" (opcional)
   │  │  │     │  ├─ Enviar felicitación instructor
   │  │  │     │  └─ Notificar seguidores instructor
   │  │  │     ├─ **Solicitar cambios**:
   │  │  │     │  ├─ Lista issues específicos (checkbox):
   │  │  │     │  │  ├─ Mejorar calidad audio lección X
   │  │  │     │  │  ├─ Corregir información lección Y
   │  │  │     │  │  ├─ Mejorar descripción curso
   │  │  │     │  │  ├─ Cambiar thumbnail
   │  │  │     │  │  ├─ Re-categorizar
   │  │  │     │  │  └─ Ajustar precio
   │  │  │     │  ├─ Mensaje personalizado instructor
   │  │  │     │  ├─ Deadline cambios (días)
   │  │  │     │  └─ Enviar notificación
   │  │  │     ├─ **Rechazar**:
   │  │  │     │  ├─ Razón rechazo (select):
   │  │  │     │  │  ├─ Plagio/Contenido copiado
   │  │  │     │  │  ├─ Calidad inaceptable
   │  │  │     │  │  ├─ Contenido inapropiado
   │  │  │     │  │  ├─ Información incorrecta/engañosa
   │  │  │     │  │  ├─ No cumple políticas
   │  │  │     │  │  ├─ Spam
   │  │  │     │  │  └─ Otra (especificar)
   │  │  │     │  ├─ Explicación detallada
   │  │  │     │  ├─ Aplicar strike instructor (toggle)
   │  │  │     │  ├─ Permitir reenvío (toggle)
   │  │  │     │  └─ Notificar instructor
   │  │  │     ├─ **Escalar revisión**:
   │  │  │     │  ├─ Asignar senior moderator
   │  │  │     │  ├─ Razón escalación
   │  │  │     │  └─ Notas adicionales
   │  │  │     └─ **Guardar borrador decisión**:
   │  │  │        └─ Continuar después
   │  │  │
   │  │  └─ Comparación side-by-side (opcional):
   │  │     ├─ Si detección plagio, mostrar fuente original vs curso
   │  │     └─ Highlight similaridades
   │  │
   │  └─ Analytics revisión cursos:
   │     ├─ Tiempo promedio revisión
   │     ├─ Tasa aprobación (%)
   │     ├─ Razones rechazo más comunes
   │     ├─ Categorías con más rechazos
   │     └─ Instructores problemáticos recurrentes
   │
   ├─ **Reportes de usuarios**:
   │  ├─ Queue reportes:
   │  │  ├─ Tabs: Todos | Pendientes | En revisión | Resueltos | Rechazados
   │  │  ├─ Filtros:
   │  │  │  ├─ Tipo contenido reportado:
   │  │  │  │  ├─ Curso completo
   │  │  │  │  ├─ Lección específica
   │  │  │  │  ├─ Comentario/Post foro
   │  │  │  │  ├─ Review/Valoración
   │  │  │  │  ├─ Perfil usuario
   │  │  │  │  ├─ Mensaje privado
   │  │  │  │  └─ Otro
   │  │  │  ├─ Razón reporte:
   │  │  │  │  ├─ Spam
   │  │  │  │  ├─ Contenido inapropiado/ofensivo
   │  │  │  │  ├─ Acoso/Bullying
   │  │  │  │  ├─ Desinformación
   │  │  │  │  ├─ Plagio/Copyright
   │  │  │  │  ├─ Contenido ilegal
   │  │  │  │  ├─ Fraude/Scam
   │  │  │  │  ├─ Suplantación identidad
   │  │  │  │  └─ Otro
   │  │  │  ├─ Prioridad (auto-calculada):
   │  │  │  │  ├─ Crítica: Contenido ilegal, múltiples reportes
   │  │  │  │  ├─ Alta: Acoso, fraude
   │  │  │  │  ├─ Media: Spam, inapropiado
   │  │  │  │  └─ Baja: Otros
   │  │  │  ├─ Reportado por (tipo usuario: estudiante/instructor)
   │  │  │  ├─ Fecha reporte (rango)
   │  │  │  └─ Asignado a moderador
   │  │  ├─ Lista reportes:
   │  │  │  ├─ Card reporte:
   │  │  │  │  ├─ ID reporte, Prioridad (badge color)
   │  │  │  │  ├─ Tipo contenido + Razón
   │  │  │  │  ├─ Reportado por: Usuario (nombre/email)
   │  │  │  │  ├─ Contra: Usuario/Contenido
   │  │  │  │  ├─ Fecha reporte
   │  │  │  │  ├─ Estado, Asignado a
   │  │  │  │  ├─ # Reportes similares (agrupados)
   │  │  │  │  └─ Acciones rápidas: Ver, Asignarme, Resolver
   │  │  │  └─ Agrupación inteligente:
   │  │  │     └─ Múltiples reportes mismo contenido consolidados
   │  │  └─ Búsqueda: Por usuario, contenido, palabras clave
   │  │
   │  ├─ Vista detalle reporte:
   │  │  ├─ **Información reporte**:
   │  │  │  ├─ Header:
   │  │  │  │  ├─ ID, Tipo, Razón
   │  │  │  │  ├─ Prioridad (editable)
   │  │  │  │  ├─ Estado (editable)
   │  │  │  │  └─ Asignado a (reasignar)
   │  │  │  ├─ Reportado por:
   │  │  │  │  ├─ Usuario (link perfil completo)
   │  │  │  │  ├─ Fecha/hora reporte
   │  │  │  │  ├─ Descripción detallada (texto libre)
   │  │  │  │  ├─ Evidencias adjuntas (screenshots, links)
   │  │  │  │  └─ Historial reportes este usuario (confiabilidad)
   │  │  │  └─ Reportes adicionales (si agrupados):
   │  │  │     ├─ # Reportes totales mismo contenido
   │  │  │     ├─ Lista usuarios reportantes
   │  │  │     └─ Razones similares/diferentes
   │  │  │
   │  │  ├─ **Contenido reportado** (visualización contextual):
   │  │  │  ├─ Si es curso/lección:
   │  │  │  │  ├─ Player video (timestamp específico si indicado)
   │  │  │  │  ├─ Transcripción (si disponible, highlight problema)
   │  │  │  │  ├─ Descripción, materiales
   │  │  │  │  └─ Contexto (sección curso, duración)
   │  │  │  ├─ Si es comentario/post:
   │  │  │  │  ├─ Texto completo
   │  │  │  │  ├─ Thread conversación (contexto)
   │  │  │  │  ├─ Autor (perfil, historial)
   │  │  │  │  └─ Fecha publicación
   │  │  │  ├─ Si es review:
   │  │  │  │  ├─ Texto review, rating
   │  │  │  │  ├─ Curso asociado
   │  │  │  │  ├─ Reviewer (verificar compra legítima)
   │  │  │  │  └─ Respuestas instructor
   │  │  │  └─ Si es perfil usuario:
   │  │  │     ├─ Info pública perfil
   │  │  │     ├─ Foto, bio, links
   │  │  │     └─ Actividad pública
   │  │  │
   │  │  ├─ **Análisis automático**:
   │  │  │  ├─ Content filtering score:
   │  │  │  │  ├─ Toxicidad (0-100%)
   │  │  │  │  ├─ Profanidad detectada
   │  │  │  │  ├─ Lenguaje odio
   │  │  │  │  ├─ Spam probability
   │  │  │  │  └─ Sentiment analysis
   │  │  │  ├─ Contexto adicional:
   │  │  │  │  ├─ Historial infracciones autor
   │  │  │  │  ├─ Precisión reportes del reportante (%)
   │  │  │  │  └─ Patrones similares detectados
   │  │  │  └─ Recomendación IA:
   │  │  │     ├─ Acción sugerida (remover/advertir/ignorar)
   │  │  │     ├─ Confidence (%)
   │  │  │     └─ Justificación
   │  │  │
   │  │  ├─ **Historial usuario reportado**:
   │  │  │  ├─ Reportes previos contra él
   │  │  │  ├─ Strikes/Warnings acumulados
   │  │  │  ├─ Suspensiones previas
   │  │  │  ├─ Contenido removido anteriormente
   │  │  │  └─ Patrón comportamiento (reincidente/primera vez)
   │  │  │
   │  │  ├─ **Timeline y notas moderación**:
   │  │  │  ├─ Historial acciones en este reporte
   │  │  │  ├─ Notas internas moderadores
   │  │  │  ├─ Cambios estado/asignación
   │  │  │  └─ Agregar nota nueva
   │  │  │
   │  │  └─ **Panel decisión**:
   │  │     ├─ Acciones disponibles:
   │  │     │  ├─ **Validar reporte y actuar**:
   │  │     │  │  ├─ Remover contenido:
   │  │     │  │  │  ├─ Eliminar inmediatamente (con/sin notificación)
   │  │     │  │  │  ├─ Ocultar temporalmente (pendiente revisión)
   │  │     │  │  │  └─ Razón eliminación (select + texto)
   │  │     │  │  ├─ Editar contenido (moderar, censurar partes)
   │  │     │  │  ├─ Aplicar sanción usuario:
   │  │     │  │  │  ├─ Warning (notificación formal)
   │  │     │  │  │  ├─ Strike (acumulativo, X strikes = ban)
   │  │     │  │  │  ├─ Suspensión temporal (duración días)
   │  │     │  │  │  ├─ Ban permanente
   │  │     │  │  │  ├─ Restricción publicar (shadow ban parcial)
   │  │     │  │  │  └─ Mensaje sanción personalizado
   │  │     │  │  ├─ Contactar usuario (solicitar corrección)
   │  │     │  │  ├─ Escalar a legal/compliance (grave)
   │  │     │  │  └─ Notificar reportante (acción tomada)
   │  │     │  ├─ **Rechazar reporte** (falso positivo):
   │  │     │  │  ├─ Razón rechazo
   │  │     │  │  ├─ Penalizar reportante (si spam reportes)
   │  │     │  │  └─ No notificar reportante (opcional)
   │  │     │  ├─ **Necesita más información**:
   │  │     │  │  ├─ Contactar reportante
   │  │     │  │  ├─ Solicitar evidencia adicional
   │  │     │  │  └─ Cambiar estado "Pending info"
   │  │     │  └─ **Escalar**:
   │  │     │     ├─ A senior moderator
   │  │     │     ├─ A equipo legal
   │  │     │     └─ A administración
   │  │     ├─ Opciones avanzadas:
   │  │     │  ├─ Crear regla automática (futuros casos similares)
   │  │     │  ├─ Agregar patrón a filtro IA
   │  │     │  └─ Reportar upstream (si contenido externo)
   │  │     └─ Confirmar y cerrar
   │  │
   │  └─ Analytics reportes:
   │     ├─ Volumen reportes (tendencia)
   │     ├─ Tasa validación (% reportes válidos)
   │     ├─ Tipos violaciones más comunes
   │     ├─ Usuarios más reportados
   │     ├─ Usuarios que más reportan (spam reporters)
   │     ├─ Tiempo promedio resolución
   │     └─ Eficacia filtros automáticos
   │
   ├─ **Moderación foros y comentarios**:
   │  ├─ Vista global comentarios:
   │  │  ├─ Tabs: Todos | Flagged | Spam | Pendientes | Aprobados | Eliminados
   │  │  ├─ Filtros:
   │  │  │  ├─ Curso/Lección específica
   │  │  │  ├─ Usuario (autor)
   │  │  │  ├─ Contiene palabras clave
   │  │  │  ├─ Toxicity score (>X%)
   │  │  │  ├─ # Reports recibidos
   │  │  │  └─ Fecha publicación
   │  │  ├─ Lista comentarios:
   │  │  │  ├─ Texto comentario
   │  │  │  ├─ Autor, fecha
   │  │  │  ├─ Contexto (curso, lección, thread)
   │  │  │  ├─ Flags automáticos:
   │  │  │  │  ├─ 🚩 Spam (%)
   │  │  │  │  ├─ 🚩 Toxicidad (%)
   │  │  │  │  ├─ 🚩 Profanidad
   │  │  │  │  ├─ 🚩 Links sospechosos
   │  │  │  │  └─ 🚩 Reportado por usuarios
   │  │  │  ├─ Votos (upvotes/downvotes)
   │  │  │  └─ Acciones: Aprobar, Editar, Eliminar, Ban usuario
   │  │  └─ Acciones masivas:
   │  │     ├─ Aprobar selección
   │  │     ├─ Eliminar selección
   │  │     └─ Marcar como spam
   │  ├─ Moderación pre-aprobación:
   │  │  ├─ Habilitar/deshabilitar por:
   │  │  │  ├─ Curso específico
   │  │  │  ├─ Usuario (con strikes previos)
   │  │  │  └─ Palabras clave trigger
   │  │  └─ Queue comentarios pendientes aprobación
   │  └─ Configuración filtros:
   │     ├─ Palabras prohibidas (blacklist):
   │     │  ├─ Lista términos (regex support)
   │     │  ├─ Acción: Auto-block/Flag para revisión
   │     │  └─ Excepciones contextuales
   │     ├─ Spam patterns:
   │     │  ├─ URLs no whitelisted
   │     │  ├─ Texto repetitivo
   │     │  ├─ Mayúsculas excesivas
   │     │  └─ Emojis flood
   │     ├─ Rate limiting comentarios:
   │     │  ├─ Max comentarios/hora por usuario
   │     │  └─ Cooldown entre comentarios (segundos)
   │     └─ Auto-moderation rules:
   │        ├─ Si toxicity >80% → Auto-block
   │        ├─ Si 3+ reportes → Flag para revisión
   │        └─ Si usuario nuevo + link → Require aprobación
   │
   ├─ **Moderación reviews/valoraciones**:
   │  ├─ Queue reviews:
   │  │  ├─ Filtros: Curso, Rating (1-5 estrellas), Reportados, Fecha
   │  │  ├─ Lista reviews:
   │  │  │  ├─ Texto review, rating
   │  │  │  ├─ Reviewer (verificar compra legítima badge)
   │  │  │  ├─ Curso, fecha
   │  │  │  ├─ # Reportes
   │  │  │  ├─ Flags:
   │  │  │  │  ├─ 🚩 Sospecha review falsa (no completó curso)
   │  │  │  │  ├─ 🚩 Spam/Promocional
   │  │  │  │  ├─ 🚩 Conflicto interés (competidor)
   │  │  │  │  ├─ 🚩 Lenguaje inapropiado
   │  │  │  │  └─ 🚩 Rating no coincide texto
   │  │  │  └─ Acciones: Aprobar, Editar, Eliminar, Investigar
   │  │  └─ Detección reviews fraudulentas:
   │  │     ├─ Patrones sospechosos:
   │  │     │  ├─ Múltiples 5★ mismo día (review bombing)
   │  │     │  ├─ Cuentas nuevas sin actividad
   │  │     │  ├─ IPs duplicadas
   │  │     │  ├─ Texto similar entre reviews
   │  │     │  └─ Reviews sin compra verificada
   │  │     ├─ Scoring credibilidad (0-100%)
   │  │     └─ Acción automática si score <30%
   │  ├─ Respuestas instructor a reviews:
   │  │  ├─ Moderar respuestas inapropiadas
   │  │  ├─ Flags: Defensivo, agresivo, spam
   │  │  └─ Eliminar si necesario
   │  └─ Analytics reviews:
   │     ├─ Distribución ratings (detección anomalías)
   │     ├─ Cursos con más reviews negativas (investigar)
   │     ├─ Tasa reviews removidas por curso
   │     └─ Efectividad detección fraude
   │
   ├─ **Gestión usuarios problemáticos**:
   │  ├─ Lista negra (blacklist):
   │  │  ├─ Usuarios baneados:
   │  │  │  ├─ Usuario, email, razón ban
   │  │  │  ├─ Tipo: Temporal/Permanente
   │  │  │  ├─ Fecha ban, duración (si temporal)
   │  │  │  ├─ Admin que aplicó ban
   │  │  │  ├─ # Strikes acumulados
   │  │  │  ├─ Apelaciones (si hay)
   │  │  │  └─ Acciones: Ver perfil, Editar ban, Levantar ban
   │  │  ├─ Filtros: Tipo ban, razón, fecha
   │  │  └─ Búsqueda: Email, nombre, IP
   │  ├─ Sistema strikes/warnings:
   │  │  ├─ Configuración:
   │  │  │  ├─ # Strikes → Suspensión temporal (ej: 3 strikes = 7 días)
   │  │  │  ├─ # Strikes → Ban permanente (ej: 5 strikes)
   │  │  │  ├─ Expiración strikes (90 días sin infracciones)
   │  │  │  └─ Severidad infracciones (menor, mayor, crítica)
   │  │  ├─ Historial strikes por usuario:
   │  │  │  ├─ Fecha, razón, severidad
   │  │  │  ├─ Contenido relacionado
   │  │  │  ├─ Moderador que aplicó
   │  │  │  └─ Estado (activo/expirado)
   │  │  └─ Dashboard usuarios en riesgo:
   │  │     └─ Lista usuarios con 2+ strikes (monitorear)
   │  ├─ Shadow banning (restricción silenciosa):
   │  │  ├─ Usuario no sabe que está limitado
   │  │  ├─ Restricciones aplicables:
   │  │  │  ├─ Comentarios solo visibles para él
   │  │  │  ├─ No puede enviar mensajes
   │  │  │  ├─ No puede publicar reviews
   │  │  │  └─ Rate limit severo
   │  │  └─ Usado para spammers (evitar que creen nueva cuenta)
   │  └─ Apelaciones:
   │     ├─ Lista apelaciones pendientes:
   │     │  ├─ Usuario, razón apelación
   │     │  ├─ Sanción original (ban/suspensión/eliminación contenido)
   │     │  ├─ Argumentos usuario
   │     │  ├─ Evidencia adicional (si proporcionó)
   │     │  ├─ Estado: Pendiente/En revisión/Aprobada/Rechazada
   │     │  └─ Fecha apelación
   │     ├─ Revisión apelación:
   │     │  ├─ Contexto decisión original
   │     │  ├─ Moderador original
   │     │  ├─ Evidencia completa
   │     │  ├─ Argumentos apelación
   │     │  └─ Decisión:
   │     │     ├─ Aprobar (levantar sanción)
   │     │     ├─ Aprobar parcialmente (reducir sanción)
   │     │     ├─ Rechazar (mantener)
   │     │     └─ Mensaje respuesta usuario
   │     └─ Analytics apelaciones:
   │        ├─ Tasa apelaciones aprobadas (calidad moderación)
   │        ├─ Moderadores con más apelaciones exitosas (revisar criterios)
   │        └─ Tiempo promedio resolución
   │
   └─ **Políticas y documentación**:
      ├─ Community Guidelines (editar):
      │  ├─ Contenido permitido/prohibido
      │  ├─ Conducta esperada
      │  ├─ Consecuencias infracciones
      │  └─ Versión pública (publicar/actualizar)
      ├─ Content Policy (instructores):
      │  ├─ Estándares calidad
      │  ├─ Derechos autor / Plagio
      │  ├─ Contenido prohibido
      │  └─ Proceso revisión
      ├─ Moderación guidelines (interna):
      │  ├─ Criterios decisiones
      │  ├─ Escalación procedures
      │  ├─ Tono comunicación usuarios
      │  └─ Best practices
      ├─ Ejemplos casos:
      │  ├─ Biblioteca casos resueltos
      │  ├─ Decisiones precedente
      │  └─ Casos difíciles (discusión)
      └─ Training moderadores:
         ├─ Onboarding nuevos moderadores
         ├─ Calibración sessions (consistencia)
         ├─ Updates políticas
         └─ Performance reviews

---

Sistema y Configuración Avanzada

🔥 /admin/sistema
   ├─ **Dashboard infraestructura**:
   │  ├─ Health check general:
   │  │  ├─ Status global: ✓ Operacional | ⚠️ Degradado | ❌ Fuera de servicio
   │  │  ├─ Uptime actual (días, horas, minutos)
   │  │  ├─ Uptime histórico: 7/30/90 días (%)
   │  │  ├─ SLA compliance (objetivo: 99.9%)
   │  │  └─ Último incidente (fecha, duración, causa)
   │  ├─ Servicios críticos (status individual):
   │  │  ├─ 🟢 Frontend (Next.js): Response time, Error rate
   │  │  ├─ 🟢 API Gateway: Requests/min, Latency p95/p99
   │  │  ├─ 🟢 Auth Service: Login rate, Success %, Active sessions
   │  │  ├─ 🟢 Course Service: CRUD ops/min, Cache hit rate
   │  │  ├─ 🟢 Media Service: Video streaming, Transcoding queue
   │  │  ├─ 🟢 Payment Service: Transactions/min, Success rate
   │  │  ├─ 🟢 Notification Service: Emails sent, Push delivered
   │  │  ├─ 🟢 Database (PostgreSQL): Connections, Query time, Replication lag
   │  │  ├─ 🟢 Cache (Redis): Hit rate, Memory usage, Evictions
   │  │  ├─ 🟢 Queue (RabbitMQ/SQS): Messages pending, Processing rate
   │  │  └─ 🟢 Storage (S3/compatible): Requests, Bandwidth, Errors
   │  ├─ Métricas infraestructura:
   │  │  ├─ Servidores/Containers:
   │  │  │  ├─ Total activos, Utilización promedio
   │  │  │  ├─ CPU: Uso actual/promedio (%)
   │  │  │  ├─ RAM: Usado/Total (GB), %
   │  │  │  ├─ Disk: Usado/Total (TB), % (alerta >80%)
   │  │  │  ├─ Network: Bandwidth in/out (Mbps)
   │  │  │  └─ Load average: 1/5/15 min
   │  │  ├─ Gráficos tiempo real:
   │  │  │  ├─ CPU usage (línea por servidor)
   │  │  │  ├─ Memory usage (stack area)
   │  │  │  ├─ Network I/O (entrada/salida)
   │  │  │  ├─ Disk I/O (read/write ops)
   │  │  │  └─ Request rate (requests/segundo)
   │  │  └─ Contenedores Docker (si aplica):
   │  │     ├─ Lista contenedores: Nombre, Status, CPU, RAM, Restarts
   │  │     ├─ Acciones: Ver logs, Restart, Stop, Escalar
   │  │     └─ Orquestación (Kubernetes/Docker Swarm):
   │  │        ├─ Pods/Services status
   │  │        ├─ Deployments (versiones activas)
   │  │        └─ Auto-scaling rules
   │  ├─ Alertas activas:
   │  │  ├─ Lista alertas (prioridad crítica primero):
   │  │  │  ├─ Servicio, métrica, valor actual vs umbral
   │  │  │  ├─ Severidad: Critical/Warning/Info
   │  │  │  ├─ Duración (tiempo en alerta)
   │  │  │  ├─ Estado: Firing/Resolved/Acknowledged
   │  │  │  └─ Acciones: Acknowledge, Silence, Escalate, View details
   │  │  └─ Configuración alertas:
   │  │     ├─ Definir nuevas alertas (thresholds)
   │  │     ├─ Canales notificación (email, Slack, PagerDuty, SMS)
   │  │     ├─ Escalation policies (quién notificar, cuándo)
   │  │     └─ Silencing rules (mantenimiento programado)
   │  └─ Quick actions:
   │     ├─ Restart servicios específicos
   │     ├─ Clear cache global
   │     ├─ Trigger backup manual
   │     ├─ Flush CDN cache
   │     └─ Run health checks

   ├─ **Monitoreo y observabilidad**:
   │  ├─ **Métricas (Prometheus/Grafana style)**:
   │  │  ├─ Dashboards predefinidos:
   │  │  │  ├─ Overview general
   │  │  │  ├─ Backend services (Go microservices)
   │  │  │  ├─ Frontend (Next.js, React)
   │  │  │  ├─ Base de datos
   │  │  │  ├─ Cache y mensajería
   │  │  │  ├─ Infraestructura servidores
   │  │  │  └─ Business metrics (conversiones, revenue, engagement)
   │  │  ├─ Custom dashboards:
   │  │  │  ├─ Constructor visual (drag & drop widgets)
   │  │  │  ├─ Tipos gráficos: Line, Bar, Gauge, Table, Heatmap
   │  │  │  ├─ Query builder (PromQL, SQL)
   │  │  │  ├─ Variables y templating
   │  │  │  └─ Compartir/exportar dashboard (JSON)
   │  │  ├─ Métricas disponibles:
   │  │  │  ├─ System: CPU, Memory, Disk, Network
   │  │  │  ├─ Application: Request rate, Error rate, Duration (RED)
   │  │  │  ├─ Business: Signups, Purchases, Active users
   │  │  │  └─ Custom: Instrumentación específica
   │  │  └─ Exportar datos:
   │  │     ├─ CSV/JSON (rango fechas)
   │  │     ├─ Integración BI tools (Tableau, Looker)
   │  │     └─ API access (Prometheus/OpenMetrics)
   │  │
   │  ├─ **Logs centralizados**:
   │  │  ├─ Agregación logs:
   │  │  │  ├─ Fuentes:
   │  │  │  │  ├─ Application logs (backend services)
   │  │  │  │  ├─ Web server logs (Nginx/Apache)
   │  │  │  │  ├─ Database logs (slow queries, errors)
   │  │  │  │  ├─ System logs (syslog, systemd)
   │  │  │  │  └─ Container logs (Docker, Kubernetes)
   │  │  │  ├─ Shipper: Fluentd, Logstash, Vector
   │  │  │  └─ Storage: ElasticSearch, Loki, CloudWatch
   │  │  ├─ Explorador logs:
   │  │  │  ├─ Query builder:
   │  │  │  │  ├─ Full-text search
   │  │  │  │  ├─ Filtros: Service, Level, Time range, Host
   │  │  │  │  ├─ Regex patterns
   │  │  │  │  └─ Structured logs (JSON fields)
   │  │  │  ├─ Vista resultados:
   │  │  │  │  ├─ Lista logs (timestamp, level, message)
   │  │  │  │  ├─ Expandir detalles (JSON viewer)
   │  │  │  │  ├─ Highlight matches
   │  │  │  │  ├─ Contexto (logs antes/después)
   │  │  │  │  └─ Live tail (stream tiempo real)
   │  │  │  ├─ Análisis:
   │  │  │  │  ├─ Aggregations (count, terms, stats)
   │  │  │  │  ├─ Histograma temporal
   │  │  │  │  ├─ Top values (errors, IPs, users)
   │  │  │  │  └─ Pattern detection (clustering similar logs)
   │  │  │  └─ Acciones:
   │  │  │     ├─ Guardar query (favoritos)
   │  │  │     ├─ Crear alerta desde query
   │  │  │     ├─ Compartir link (permalink)
   │  │  │     └─ Exportar resultados
   │  │  ├─ Log levels:
   │  │  │  ├─ ERROR (rojo): Errores críticos
   │  │  │  ├─ WARN (amarillo): Advertencias
   │  │  │  ├─ INFO (azul): Información general
   │  │  │  ├─ DEBUG (gris): Debugging detallado
   │  │  │  └─ Configurar verbosidad por servicio
   │  │  └─ Retención logs:
   │  │     ├─ Hot storage (búsqueda rápida): 7-30 días
   │  │     ├─ Warm storage (comprimido): 90 días
   │  │     ├─ Cold storage (archivo): 1+ año
   │  │     └─ Políticas eliminación (compliance)
   │  │
   │  ├─ **Tracing distribuido (APM)**:
   │  │  ├─ Proveedores: Jaeger, Zipkin, New Relic, Datadog
   │  │  ├─ Service map:
   │  │  │  ├─ Visualización dependencias servicios
   │  │  │  ├─ Latencia entre servicios
   │  │  │  ├─ Error rates por edge
   │  │  │  └─ Request flow (paths comunes)
   │  │  ├─ Traces explorer:
   │  │  │  ├─ Buscar traces (operation, service, duration, error)
   │  │  │  ├─ Flame graph (visualización jerárquica spans)
   │  │  │  ├─ Timeline detallado (duración cada span)
   │  │  │  ├─ Tags y metadata (user_id, request_id, etc.)
   │  │  │  └─ Logs correlacionados (mismo trace_id)
   │  │  ├─ Performance insights:
   │  │  │  ├─ Slowest operations (percentiles p50/p95/p99)
   │  │  │  ├─ Database queries más lentas
   │  │  │  ├─ External API calls (latencia terceros)
   │  │  │  └─ N+1 query detection
   │  │  └─ Error tracking:
   │  │     ├─ Stack traces completos
   │  │     ├─ Contexto request (headers, params, user)
   │  │     ├─ Agrupación errores similares
   │  │     └─ Integración Sentry/Bugsnag
   │  │
   │  └─ **Synthetic monitoring (Uptime checks)**:
   │     ├─ Endpoints monitoreados:
   │     │  ├─ Homepage, Login, API health
   │     │  ├─ Critical user flows (signup, checkout)
   │     │  └─ Third-party integrations
   │     ├─ Configuración checks:
   │     │  ├─ URL/endpoint
   │     │  ├─ Método HTTP, Headers, Body
   │     │  ├─ Assertions (status code, response time, body contains)
   │     │  ├─ Frecuencia (1/5/15 min)
   │     │  ├─ Locations (múltiples regiones geográficas)
   │     │  └─ Alertas (downtime, slow response)
   │     ├─ Resultados:
   │     │  ├─ Uptime % (por check, global)
   │     │  ├─ Response time (promedio, min, max)
   │     │  ├─ Incidents timeline
   │     │  └─ Availability por región
   │     └─ Status page público:
   │        ├─ Generar página status (status.example.com)
   │        ├─ Servicios monitoreados
   │        ├─ Historial incidentes
   │        └─ Subscribe notificaciones
   │
   ├─ **Base de datos**:
   │  ├─ **Visión general**:
   │  │  ├─ Instancias database:
   │  │  │  ├─ Primary (master): Status, Connections, CPU, RAM
   │  │  │  ├─ Replicas (read): Lista, Replication lag, Load
   │  │  │  └─ Connection pooling: Activas/Idle/Max
   │  │  ├─ Métricas principales:
   │  │  │  ├─ Total databases, Total tables
   │  │  │  ├─ Database size (GB), Growth rate
   │  │  │  ├─ Transactions/segundo (TPS)
   │  │  │  ├─ Queries/segundo (QPS)
   │  │  │  ├─ Cache hit ratio (%)
   │  │  │  ├─ Index usage (%)
   │  │  │  └─ Deadlocks/min
   │  │  └─ Gráficos:
   │  │     ├─ Query performance (tiempo ejecución)
   │  │     ├─ Connections usage
   │  │     ├─ Buffer pool/cache utilization
   │  │     └─ Replication lag (si multi-master/replicas)
   │  │
   │  ├─ **Slow queries**:
   │  │  ├─ Lista queries lentas:
   │  │  │  ├─ Query text (normalizada, sin valores específicos)
   │  │  │  ├─ Promedio duración (ms)
   │  │  │  ├─ # Ejecuciones
   │  │  │  ├─ Total tiempo acumulado
   │  │  │  ├─ Rows examined vs returned (eficiencia)
   │  │  │  ├─ Table(s) afectadas
   │  │  │  └─ Último ejecutada
   │  │  ├─ Análisis query:
   │  │  │  ├─ EXPLAIN plan (visualización árbol)
   │  │  │  ├─ Índices usados/faltantes
   │  │  │  ├─ Sugerencias optimización:
   │  │  │  │  ├─ "Agregar índice en columna X"
   │  │  │  │  ├─ "Evitar full table scan"
   │  │  │  │  ├─ "Reescribir query (subquery → JOIN)"
   │  │  │  │  └─ "Particionar tabla Y"
   │  │  │  └─ Ejecutar query test (en replica/staging)
   │  │  ├─ Configuración threshold:
   │  │  │  ├─ Duración mínima (ms) considerar "slow"
   │  │  │  ├─ Top N queries mostrar
   │  │  │  └─ Período análisis
   │  │  └─ Acciones:
   │  │     ├─ Crear índice sugerido (generar DDL)
   │  │     ├─ Añadir a query blacklist (kill automático)
   │  │     ├─ Crear ticket optimización
   │  │     └─ Exportar reporte
   │  │
   │  ├─ **Schema y gestión**:
   │  │  ├─ Navegador schema:
   │  │  │  ├─ Lista databases
   │  │  │  ├─ Por database: Lista tables (nombre, rows, size)
   │  │  │  ├─ Por table:
   │  │  │  │  ├─ Columnas (nombre, tipo, null, default, key)
   │  │  │  │  ├─ Índices (nombre, tipo, columnas, cardinality, size)
   │  │  │  │  ├─ Foreign keys (relaciones)
   │  │  │  │  ├─ Triggers
   │  │  │  │  ├─ Estadísticas (rows, data size, index size)
   │  │  │  │  └─ Fragmentación (% necesita optimize)
   │  │  │  └─ Acciones:
   │  │  │     ├─ Ver data (sample rows)
   │  │  │     ├─ Generar DDL (CREATE TABLE)
   │  │  │     ├─ Analyze table (update stats)
   │  │  │     ├─ Optimize table (desfragmentar)
   │  │  │     └─ Truncate/Drop (confirmación)
   │  │  ├─ Índices:
   │  │  │  ├─ Lista índices (todos o por tabla)
   │  │  │  ├─ Índices sin uso (candidatos eliminar)
   │  │  │  ├─ Índices duplicados/redundantes
   │  │  │  ├─ Sugerencias índices faltantes (basado slow queries)
   │  │  │  └─ Crear/eliminar índice (DDL)
   │  │  ├─ Query console:
   │  │  │  ├─ Editor SQL (syntax highlighting, autocomplete)
   │  │  │  ├─ Seleccionar database/connection
   │  │  │  ├─ Ejecutar query (límite rows safety)
   │  │  │  ├─ Resultados tabla (sort, filter, export CSV)
   │  │  │  ├─ EXPLAIN query
   │  │  │  ├─ Historial queries ejecutadas
   │  │  │  └─ Saved queries (snippets)
   │  │  └─ Mantenimiento:
   │  │     ├─ VACUUM/ANALYZE (PostgreSQL)
   │  │     ├─ OPTIMIZE TABLE (MySQL)
   │  │     ├─ Rebuild indexes
   │  │     ├─ Update statistics
   │  │     └─ Programar mantenimiento automático
   │  │
   │  ├─ **Backups y recuperación** (ver sección dedicada más abajo)
   │  │
   │  └─ **Configuración avanzada**:
   │     ├─ Parámetros database:
   │     │  ├─ Connections: max_connections, pool_size
   │     │  ├─ Memory: shared_buffers, work_mem, maintenance_work_mem
   │     │  ├─ WAL/Logging: wal_level, log_min_duration
   │     │  ├─ Autovacuum (PostgreSQL): Thresholds, scale_factor
   │     │  ├─ Query cache (MySQL)
   │     │  └─ Aplicar cambios (require restart advertencia)
   │     ├─ Replicación:
   │     │  ├─ Configurar read replicas
   │     │  ├─ Failover automático (Patroni, PgBouncer)
   │     │  ├─ Load balancing reads
   │     │  └─ Monitoring lag replication
   │     ├─ Particionamiento:
   │     │  ├─ Tables candidatas (grandes, por fecha)
   │     │  ├─ Estrategia: Range, List, Hash
   │     │  ├─ Crear particiones (automatizado)
   │     │  └─ Mantenimiento particiones (drop old)
   │     └─ Security:
   │        ├─ Users y roles database
   │        ├─ Permisos granulares (schemas, tables)
   │        ├─ SSL connections enforcement
   │        ├─ Audit logging (DDL/DML)
   │        └─ Encryption at rest
   │
   ├─ **Cache y performance**:
   │  ├─ **Redis/Cache layer**:
   │  │  ├─ Overview:
   │  │  │  ├─ Instancia(s) Redis: Status, Version
   │  │  │  ├─ Memory usado/total (MB/GB)
   │  │  │  ├─ Hit rate (%)
   │  │  │  ├─ Evictions (keys expulsadas)
   │  │  │  ├─ Commands/segundo
   │  │  │  ├─ Connected clients
   │  │  │  └─ Replication (master-slave si aplica)
   │  │  ├─ Key explorer:
   │  │  │  ├─ Lista keys (patrón búsqueda)
   │  │  │  ├─ Por key: Tipo, Size, TTL, Value (preview)
   │  │  │  ├─ Filtrar: Por patrón, Tipo (string/hash/list/set/zset)
   │  │  │  └─ Acciones: Ver, Editar, Renombrar, Delete, Set TTL
   │  │  ├─ Slow log:
   │  │  │  ├─ Comandos lentos (>threshold ms)
   │  │  │  ├─ Timestamp, duración, comando
   │  │  │  └─ Análisis patrones
   │  │  ├─ Métricas detalladas:
   │  │  │  ├─ Operaciones: GET, SET, DEL, etc. (counts)
   │  │  │  ├─ Keys por namespace (prefijos)
   │  │  │  ├─ Memory breakdown (keys, overhead)
   │  │  │  ├─ Network I/O
   │  │  │  └─ Persistence (RDB/AOF status)
   │  │  ├─ Configuración:
   │  │  │  ├─ Max memory policy (allkeys-lru, volatile-lru, etc.)
   │  │  │  ├─ Max memory limit
   │  │  │  ├─ TTL default
   │  │  │  ├─ Persistence: RDB snapshots, AOF
   │  │  │  └─ Replication setup
   │  │  └─ Acciones:
   │  │     ├─ Flush cache (all/database/patrón)
   │  │     ├─ Analyze memory (redis-cli --bigkeys)
   │  │     └─ Export/Import data
   │  │
   │  ├─ **CDN y edge caching**:
   │  │  ├─ Proveedor: Cloudflare, AWS CloudFront, Fastly
   │  │  ├─ Dashboard CDN:
   │  │  │  ├─ Requests servidos (total, cached, origin)
   │  │  │  ├─ Bandwidth saved (%)
   │  │  │  ├─ Cache hit ratio (%)
   │  │  │  ├─ Distribución geográfica requests
   │  │  │  └─ Top cached/uncached URLs
   │  │  ├─ Configuración caching:
   │  │  │  ├─ Rules por path/extension:
   │  │  │  │  ├─ Cache-Control headers
   │  │  │  │  ├─ TTL (segundos, minutos, horas, días)
   │  │  │  │  ├─ Bypass cache (query params, cookies)
   │  │  │  │  └─ Vary headers
   │  │  │  ├─ Tipos archivo:
   │  │  │  │  ├─ Static assets: Long TTL (imágenes, CSS, JS)
   │  │  │  │  ├─ HTML: Short TTL o no-cache
   │  │  │  │  ├─ API responses: Conditional (por endpoint)
   │  │  │  │  └─ Videos: Streaming optimizations
   │  │  │  └─ Advanced:
   │  │  │     ├─ Stale-while-revalidate
   │  │  │     ├─ Cache keys customization
   │  │  │     └─ Edge side includes (ESI)
   │  │  ├─ Purge cache:
   │  │  │  ├─ Purge all (nuclear option)
   │  │  │  ├─ Purge by URL (single file)
   │  │  │  ├─ Purge by path pattern (/blog/*)
   │  │  │  ├─ Purge by tag (cache tagging)
   │  │  │  └─ Programar purge automático (webhooks)
   │  │  └─ Performance optimizations:
   │  │     ├─ Image optimization (auto WebP/AVIF)
   │  │     ├─ Minification (HTML, CSS, JS)
   │  │     ├─ Brotli/Gzip compression
   │  │     ├─ HTTP/2, HTTP/3 (QUIC)
   │  │     └─ Early Hints (103 status)
   │  │
   │  └─ **Application-level caching**:
   │     ├─ Cache strategies:
   │     │  ├─ Lista cachés configurados:
   │     │  │  ├─ Courses catalog
   │     │  │  ├─ User profiles
   │     │  │  ├─ Categories/tags
   │     │  │  ├─ Analytics queries
   │     │  │  └─ API responses
   │     │  ├─ Por cache:
   │     │  │  ├─ TTL, Invalidación strategy
   │     │  │  ├─ Hit/Miss rate
   │     │  │  ├─ Size (MB), # Keys
   │     │  │  └─ Habilitar/Deshabilitar
   │     │  └─ Warmup cache (pre-populate common data)
   │     ├─ Invalidación:
   │     │  ├─ Manual (clear specific cache)
   │     │  ├─ Time-based (TTL)
   │     │  ├─ Event-based (on update/delete)
   │     │  └─ Tag-based (dependencies)
   │     └─ Monitoring:
   │        ├─ Cache efficiency metrics
   │        ├─ Latency comparison (cached vs uncached)
   │        └─ Memory usage per cache
   │
   ├─ **Backups y disaster recovery**:
   │  ├─ **Configuración backups**:
   │  │  ├─ Database backups:
   │  │  │  ├─ Frecuencia:
   │  │  │  │  ├─ Full backup: Diario (3 AM)
   │  │  │  │  ├─ Incremental: Cada 6 horas
   │  │  │  │  ├─ Transaction logs: Continuo (WAL archiving)
   │  │  │  │  └─ Custom schedule (cron expression)
   │  │  │  ├─ Retención:
   │  │  │  │  ├─ Diarios: 7 días
   │  │  │  │  ├─ Semanales: 4 semanas
   │  │  │  │  ├─ Mensuales: 12 meses
   │  │  │  │  └─ Anuales: Indefinido
   │  │  │  ├─ Destino:
   │  │  │  │  ├─ Primary: S3/GCS/Azure Blob (region)
   │  │  │  │  ├─ Secondary: Diferente región (DR)
   │  │  │  │  └─ Offsite: Glacier/Cold storage
   │  │  │  ├─ Compresión: Gzip, LZ4, Zstd
   │  │  │  ├─ Encriptación: AES-256, KMS
   │  │  │  └─ Verificación integridad (checksums)
   │  │  ├─ File backups:
   │  │  │  ├─ User uploads (videos, docs, images)
   │  │  │  ├─ Application files (código, configs)
   │  │  │  ├─ Frecuencia: Diaria incremental, semanal full
   │  │  │  ├─ Snapshot-based (si filesystem soporta)
   │  │  │  └─ Versioning habilitado
   │  │  └─ Configuración avanzada:
   │  │     ├─ Parallel backup jobs (performance)
   │  │     ├─ Throttling (limitar impacto producción)
   │  │     ├─ Pre/post scripts (hooks)
   │  │     └─ Exclude patterns (archivos temporales)
   │  │
   │  ├─ **Historial backups**:
   │  │  ├─ Lista backups:
   │  │  │  ├─ Tipo (Full/Incremental/Snapshot)
   │  │  │  ├─ Fecha/hora inicio y fin
   │  │  │  ├─ Tamaño (compressed/uncompressed)
   │  │  │  ├─ Duración proceso
   │  │  │  ├─ Estado: Success/Failed/In Progress
   │  │  │  ├─ Ubicación storage (path, bucket)
   │  │  │  ├─ Checksum (MD5/SHA256)
   │  │  │  └─ Retention expiry date
   │  │  ├─ Filtros: Tipo, Estado, Fecha, Exitosos/Fallidos
   │  │  ├─ Detalles backup:
   │  │  │  ├─ Logs ejecución (output completo)
   │  │  │  ├─ Metadata (versión DB, servidor, config)
   │  │  │  ├─ Files incluidos (manifest)
   │  │  │  └─ Verificación (integrity test result)
   │  │  └─ Acciones:
   │  │     ├─ Descargar backup (generate signed URL)
   │  │     ├─ Restore (ver sección siguiente)
   │  │     ├─ Verificar integridad (manual check)
   │  │     ├─ Extend retention (postpone deletion)
   │  │     └─ Delete backup (confirmación)
   │  │
   │  ├─ **Restore/Recuperación**:
   │  │  ├─ Wizard recuperación:
   │  │  │  ├─ **Paso 1**: Seleccionar backup
   │  │  │  │  ├─ Lista backups disponibles (filtrar por fecha)
   │  │  │  │  ├─ Preview metadata
   │  │  │  │  └─ Point-in-time recovery (si WAL disponible)
   │  │  │  ├─ **Paso 2**: Opciones recuperación
   │  │  │  │  ├─ Destino:
   │  │  │  │  │  ├─ Sobrescribir producción (⚠️ PELIGROSO)
   │  │  │  │  │  ├─ Nueva instancia (staging/test)
   │  │  │  │  │  └─ Servidor específico
   │  │  │  │  ├─ Alcance:
   │  │  │  │  │  ├─ Full restore (toda DB)
   │  │  │  │  │  ├─ Selective (databases/tables específicas)
   │  │  │  │  │  └─ Data only (sin schema)
   │  │  │  │  ├─ Opciones:
   │  │  │  │  │  ├─ Stop application (downtime controlado)
   │  │  │  │  │  ├─ Verify before restore (dry-run)
   │  │  │  │  │  └─ Backup current state antes restore
   │  │  │  │  └─ Confirmación (checkbox múltiples)
   │  │  │  ├─ **Paso 3**: Ejecución
   │  │  │  │  ├─ Progress bar (%, time remaining)
   │  │  │  │  ├─ Live log stream
   │  │  │  │  ├─ Cancelar restore (rollback)
   │  │  │  │  └─ Notificaciones (email/Slack al completar)
   │  │  │  └─ **Paso 4**: Post-restore
   │  │  │     ├─ Verify data integrity (checksums, counts)
   │  │  │     ├─ Rebuild indexes/statistics
   │  │  │     ├─ Test critical functionality
   │  │  │     └─ Resume application
   │  │  ├─ Point-in-Time Recovery (PITR):
   │  │  │  ├─ Seleccionar timestamp exacto (calendar + time picker)
   │  │  │  ├─ Preview: "Restaurar a 2024-12-10 14:35:22"
   │  │  │  ├─ Replay transaction logs hasta punto
   │  │  │  └─ Advertencia: Data posterior se perderá
   │  │  └─ Disaster Recovery Plan:
   │  │     ├─ Documento DR (procedimientos)
   │  │     ├─ RTO (Recovery Time Objective): Target <4 horas
   │  │     ├─ RPO (Recovery Point Objective): Target <1 hora
   │  │     ├─ Contact list (equipo emergencias)
   │  │     └─ Runbook (pasos automatizados)
   │  │
   │  ├─ **Testing y validación**:
   │  │  ├─ Automated backup tests:
   │  │  │  ├─ Programar test restore periódico (mensual)
   │  │  │  ├─ Restore a ambiente test
   │  │  │  ├─ Run integrity checks
   │  │  │  ├─ Run smoke tests (queries básicas)
   │  │  │  └─ Reporte resultado (success/failure)
   │  │  ├─ Manual test restore:
   │  │  │  ├─ Seleccionar backup aleatorio
   │  │  │  ├─ Restore en sandbox
   │  │  │  ├─ Verificación manual
   │  │  │  └─ Documentar resultado
   │  │  └─ Métricas validación:
   │  │     ├─ % Backups tested (objetivo: 100% últimos 90 días)
   │  │     ├─ % Successful restores
   │  │     └─ Avg restore time
   │  │
   │  └─ **Alertas y monitoreo**:
   │     ├─ Backup failures (email inmediato)
   │     ├─ Backup size anomalías (muy grande/pequeño)
   │     ├─ Backup duration exceeds threshold
   │     ├─ Storage space bajo (<20%)
   │     ├─ Backups no tested >30 días
   │     └─ Retención próxima expirar (backups importantes)
   │
   ├─ **Jobs programados (Cron/Scheduled tasks)**:
   │  ├─ Lista jobs:
   │  │  ├─ Tabla jobs:
   │  │  │  ├─ Nombre job, Descripción
   │  │  │  ├─ Schedule (cron expression, human-readable)
   │  │  │  ├─ Próxima ejecución (countdown)
   │  │  │  ├─ Última ejecución (timestamp, duración, estado)
   │  │  │  ├─ Estado: Enabled/Disabled/Running
   │  │  │  ├─ Timeout (max duración permitida)
   │  │  │  ├─ Retry policy (reintentos en fallo)
   │  │  │  └─ Comando/Script
   │  │  ├─ Jobs comunes:
   │  │  │  ├─ Database backups
   │  │  │  ├─ Send email notifications (digest diario)
   │  │  │  ├─ Generate reports (analytics)
   │  │  │  ├─ Clean expired sessions
   │  │  │  ├─ Process video transcoding queue
   │  │  │  ├─ Update search indexes
   │  │  │  ├─ Archive old data
   │  │  │  ├─ Renew SSL certificates
   │  │  │  ├─ Check external APIs health
   │  │  │  └─ Sync third-party data
   │  │  ├─ Acciones:
   │  │  │  ├─ Enable/Disable job
   │  │  │  ├─ Run now (manual trigger)
   │  │  │  ├─ Ver historial ejecuciones
   │  │  │  ├─ Editar schedule
   │  │  │  └─ Ver logs última ejecución
   │  │  └─ Filtros: Estado, Categoría, Schedule frequency
   │  ├─ Crear/Editar job:
   │  │  ├─ Información básica:
   │  │  │  ├─ Nombre único
   │  │  │  ├─ Descripción/Propósito
   │  │  │  ├─ Categoría (Backup, Maintenance, Reports, etc.)
   │  │  │  └─ Tags
   │  │  ├─ Schedule:
   │  │  │  ├─ Cron expression (con helper visual)
   │  │  │  ├─ Or presets: Hourly, Daily, Weekly, Monthly
   │  │  │  ├─ Timezone
   │  │  │  └─ Preview próximas 5 ejecuciones
   │  │  ├─ Ejecución:
   │  │  │  ├─ Tipo: Shell script, HTTP request, Internal function
   │  │  │  ├─ Comando/URL/Function
   │  │  │  ├─ Parámetros/Environment vars
   │  │  │  ├─ Working directory
   │  │  │  └─ Run as user (permissions)
   │  │  ├─ Configuración avanzada:
   │  │  │  ├─ Timeout (segundos)
   │  │  │  ├─ Max concurrent runs (prevent overlap)
   │  │  │  ├─ Retry:
   │  │  │  │  ├─ Max retries (0-5)
   │  │  │  │  ├─ Retry delay (minutos)
   │  │  │  │  └─ Backoff strategy (linear/exponential)
   │  │  │  ├─ Dependencies (wait for other jobs)
   │  │  │  └─ Conditional execution (only if X)
   │  │  ├─ Notificaciones:
   │  │  │  ├─ On success (opcional)
   │  │  │  ├─ On failure (recomendado)
   │  │  │  ├─ Canales: Email, Slack, Webhook
   │  │  │  └─ Recipients
   │  │  └─ Logging:
   │  │     ├─ Log level (ERROR/WARN/INFO/DEBUG)
   │  │     ├─ Retention (días)
   │  │     └─ Output redirection
   │  ├─ Historial ejecuciones:
   │  │  ├─ Por job o global
   │  │  ├─ Lista ejecuciones:
   │  │  │  ├─ Job, Timestamp inicio/fin
   │  │  │  ├─ Duración
   │  │  │  ├─ Estado: Success/Failed/Timeout/Canceled
   │  │  │  ├─ Exit code
   │  │  │  ├─ Output (stdout/stderr)
   │  │  │  └─ Retry attempt #
   │  │  ├─ Filtros: Job, Estado, Fecha
   │  │  ├─ Ver logs completos (modal/nueva página)
   │  │  └─ Estadísticas:
   │  │     ├─ Success rate (%)
   │  │     ├─ Avg duration
   │  │     └─ Failures last 30 days
   │  └─ Monitoreo jobs:
   │     ├─ Jobs fallando consistentemente (alerta)
   │     ├─ Jobs con duración creciente (trend)
   │     ├─ Jobs no ejecutados (missed schedules)
   │     └─ Dashboard tiempo real (running jobs)
   │
   └─ **Features y experimentos**:
      ├─ **Feature flags**:
      │  ├─ Lista features:
      │  │  ├─ Nombre, Descripción
      │  │  ├─ Estado global: ON/OFF/PERCENTAGE
      │  │  ├─ Rollout % (gradual deployment)
      │  │  ├─ Ambientes: Production, Staging, Development
      │  │  ├─ Targeting rules:
      │  │  │  ├─ User attributes (role, plan, location)
      │  │  │  ├─ Whitelist/Blacklist users
      │  │  │  └─ Random sampling (A/B test)
      │  │  ├─ Fecha creación, Última modificación
      │  │  └─ Creado por (admin)
      │  ├─ Crear/Editar feature flag:
      │  │  ├─ Identificador único (snake_case)
      │  │  ├─ Nombre display
      │  │  ├─ Descripción, Propósito
      │  │  ├─ Tipo: Boolean, String, Number, JSON
      │  │  ├─ Default value
      │  │  ├─ Rollout strategy:
      │  │  │  ├─ On/Off simple
      │  │  │  ├─ Percentage (0-100%)
      │  │  │  ├─ User targeting (conditions)
      │  │  │  └─ Scheduled (auto on/off dates)
      │  │  ├─ Environments override (different per env)
      │  │  └─ Tags, Owner
      │  ├─ Gestión rollout:
      │  │  ├─ Incrementar % gradualmente (10% → 25% → 50% → 100%)
      │  │  ├─ Monitor metrics (error rate, performance)
      │  │  ├─ Rollback instant (kill switch)
      │  │  └─ Logs cambios (audit trail)
      │  ├─ Evaluación feature:
      │  │  ├─ Test evaluación (simular user)
      │  │  ├─ Input: User ID, attributes
      │  │  ├─ Output: Feature value, Razón (matched rule)
      │  │  └─ Debug mode
      │  └─ Analytics:
      │     ├─ # Users affected by feature
      │     ├─ Distribution rollout actual
      │     ├─ Impact metrics (conversión, engagement)
      │     └─ Cleanup flags antiguos (>6 meses estables)
      │
      └─ **A/B Testing**:
         ├─ Lista experimentos:
         │  ├─ Nombre experimento, Hipótesis
         │  ├─ Estado: Draft/Running/Paused/Completed/Archived
         │  ├─ Variantes (Control, A, B, C...)
         │  ├─ Traffic split (%, por variante)
         │  ├─ Fecha inicio, Fecha fin (planned/actual)
         │  ├─ # Participantes (total, por variante)
         │  ├─ Primary metric (conversión, engagement, revenue)
         │  └─ Winner (si completado)
         ├─ Crear experimento:
         │  ├─ Básico:
         │  │  ├─ Nombre, Descripción, Hipótesis
         │  │  ├─ Objetivo (increase signups 10%)
         │  │  ├─ Owner (quien ejecuta)
         │  │  └─ Tags
         │  ├─ Variantes:
         │  │  ├─ Control (baseline)
         │  │  ├─ Variante A (descripción cambio)
         │  │  ├─ Variante B (opcional)
         │  │  ├─ Traffic allocation (%)
         │  │  └─ Associated feature flag
         │  ├─ Métricas:
         │  │  ├─ Primary: Conversion rate, Revenue, Engagement
         │  │  ├─ Secondary: Bounce rate, Time on page, etc.
         │  │  ├─ Guardrails: Error rate, Load time (no empeorar)
         │  │  └─ Minimum sample size (statistical power)
         │  ├─ Targeting:
         │  │  ├─ Audience (todos/segmento específico)
         │  │  ├─ Devices, Locations, User attributes
         │  │  └─ Exclusions (bots, internal users)
         │  ├─ Duración:
         │  │  ├─ Fecha inicio
         │  │  ├─ Duración estimada (días)
         │  │  └─ Auto-stop conditions (winner clear, max duration)
         │  └─ Configuración:
         │     ├─ Sticky assignment (user siempre ve misma variante)
         │     ├─ Quality assurance mode (preview variantes)
         │     └─ Notificaciones (milestones, completion)
         ├─ Resultados experimento:
         │  ├─ Dashboard tiempo real:
         │  │  ├─ Participantes por variante
         │  │  ├─ Métricas principales (por variante):
         │  │  │  ├─ Valor métrica
         │  │  │  ├─ Confidence interval (95%)
         │  │  │  ├─ Improvement vs control (+X%)
         │  │  │  └─ Statistical significance (p-value)
         │  │  ├─ Gráficos:
         │  │  │  ├─ Conversión funnel (por variante)
         │  │  │  ├─ Tendencia temporal
         │  │  │  └─ Distribución usuarios
         │  │  └─ Recommendation:
         │  │     ├─ "Variant A is winning with 95% confidence"
         │  │     ├─ "Not enough data yet (need X more days)"
         │  │     └─ "No significant difference detected"
         │  ├─ Análisis detallado:
         │  │  ├─ Segmentación resultados (por device, location, etc.)
         │  │  ├─ Outliers detection
         │  │  ├─ Novelty effect tracking
         │  │  └─ Export data (CSV/JSON)
         │  ├─ Decisión:
         │  │  ├─ Declare winner (aplicar variante ganadora)
         │  │  ├─ Inconclusive (necesita más tiempo)
         │  │  ├─ Abandon (ninguna mejora)
         │  │  └─ Post-mortem notes
         │  └─ Rollout winner:
         │     ├─ Gradual rollout a 100% users
         │     ├─ Update feature flag
         │     ├─ Remove experiment code (tech debt)
         │     └─ Share learnings (internal docs)
         └─ Biblioteca experimentos:
            ├─ Historial todos experimentos
            ├─ Learnings acumulados
            ├─ Templates experimentos comunes
            └─ Best practices documentation

---

