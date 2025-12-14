# Scorecard de Separación Monorepo - ACC LMS

## 📊 Métricas de Evaluación Mensual

### 🔧 Métricas Técnicas (Peso: 40%)

| Métrica           | Umbral Crítico | Actual | Score |
| ----------------- | -------------- | ------ | ----- |
| Tamaño repo       | 10 GB          | -      | 🟢    |
| Tiempo build      | 45 min         | -      | 🟢    |
| Tiempo clone      | 10 min         | -      | 🟢    |
| Test suite        | 30 min         | -      | 🟢    |
| Pipeline failures | 20%            | -      | 🟢    |

### 👥 Métricas de Equipo (Peso: 35%)

| Métrica                 | Umbral Crítico | Actual | Score |
| ----------------------- | -------------- | ------ | ----- |
| Desarrolladores activos | 50             | 2-5    | 🟢    |
| Teams paralelos         | 8              | 1      | 🟢    |
| Commits/día             | 200            | 5-20   | 🟢    |
| PRs simultáneos         | 30             | 2-5    | 🟢    |
| Conflictos merge/sprint | 50             | 0-5    | 🟢    |

### 📈 Métricas de Producto (Peso: 25%)

| Métrica                 | Umbral Crítico | Actual | Score |
| ----------------------- | -------------- | ------ | ----- |
| Servicios en producción | 40             | 0      | 🟢    |
| Equipos especializados  | 5              | 1      | 🟢    |
| SLA diferenciados       | 3              | 1      | 🟢    |
| Release cycles          | 2              | 1      | 🟢    |

## 🚨 **Trigger de Separación**

**Separar cuando:**

- Score total: 🔴 Rojo en > 60% métricas
- O cualquier métrica crítica individual

**Colores:**

- 🟢 Verde: 0-70% del umbral
- 🟡 Amarillo: 70-90% del umbral
- 🔴 Rojo: >90% del umbral

## 📅 Evaluación

**Frecuencia:** Mensual  
**Responsable:** Tech Lead  
**Decisión:** Consenso del equipo

---

**Estado actual:** 🟢 MONOREPO SALUDABLE  
**Próxima evaluación:** [Mes + 1]
