# Política de Seguridad

## 🛡️ Resumen de Seguridad

ACC LMS implementa **seguridad multicapa** con protección contra los principales vectores de ataque. Todas las políticas detalladas están disponibles en [`_docs/cybersecurity-policies.md`](_docs/cybersecurity-policies.md).

## 🔐 Implementaciones de Seguridad

### ✅ **Autenticación y Autorización**

- JWT con rotación automática (15 min access, 7 días refresh)
- Argon2 para hash de passwords (64MB memory cost)
- Row Level Security (RLS) en PostgreSQL
- Principio de menor privilegio

### ✅ **Protección de APIs**

- Rate limiting: 50 req/s general, 3 req/m login
- CORS configurado estrictamente
- Input validation con Pydantic
- SQL injection protection (prepared statements)

### ✅ **Infraestructura Segura**

- Firewall UFW configurado
- SSH hardening (claves, puerto no estándar)
- SSL/TLS obligatorio con Let's Encrypt
- Contenedores no-root con capabilities limitadas

### ✅ **Monitoreo y Respuesta**

- Fail2Ban para detección de intrusos
- Logging centralizado de eventos de seguridad
- Health checks con métricas de seguridad
- Plan de respuesta a incidentes

## 🚨 Reportar Vulnerabilidades

**Para reportar vulnerabilidades de seguridad:**

1. **Email privado**: [security@acc-lms.com]
2. **No crear issues públicos** para problemas de seguridad
3. **Tiempo de respuesta**: 48 horas máximo

### 📋 Información a Incluir

- Descripción detallada de la vulnerabilidad
- Pasos para reproducir el problema
- Impacto potencial estimado
- Cualquier mitigación temporal conocida

## 🏆 Reconocimiento

Reconocemos públicamente a los investigadores de seguridad que reportan vulnerabilidades responsablemente (si lo desean).

## 📞 Contacto de Emergencia

- **Email**: security@acc-lms.com
- **Tiempo de respuesta**: 24 horas para vulnerabilidades críticas
- **Escalamiento**: CTO/Tech Lead para incidentes de alto impacto

---

**Documentación completa**: [`_docs/cybersecurity-policies.md`](_docs/cybersecurity-policies.md)  
**Última actualización**: 2025-08-08
