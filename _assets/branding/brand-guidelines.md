# ACC LMS - Brand Guidelines

## 🎨 Paleta de Colores

### Colores Principales

| Color              | Hex Code  | RGB                  | Uso                         |
| ------------------ | --------- | -------------------- | --------------------------- |
| **Verde ACC**      | `#23a500` | `rgb(35, 165, 0)`    | Principal, CTA, Success     |
| **Naranja Accent** | `#ff7e05` | `rgb(255, 126, 5)`   | Accent, Warning, Highlights |
| **Gris Medio**     | `#585858` | `rgb(88, 88, 88)`    | Texto secundario, Borders   |
| **Gris Claro**     | `#a0a0a0` | `rgb(160, 160, 160)` | Texto terciario (dark mode) |
| **Negro Suave**    | `#1a1a1a` | `rgb(26, 26, 26)`    | Fondo dark theme            |
| **Negro**          | `#000000` | `rgb(0, 0, 0)`       | Texto principal (light)     |
| **Blanco**         | `#ffffff` | `rgb(255, 255, 255)` | Texto (dark), Backgrounds   |

---

## 🖼️ Reglas para Archivos SVG

### ⚠️ OBLIGATORIO para todos los SVG

| Regla          | Descripción                                                |
| -------------- | ---------------------------------------------------------- |
| **Tema**       | Dark theme (`#1a1a1a` como fondo)                          |
| **Degradados** | ❌ PROHIBIDOS - Solo colores sólidos                       |
| **Fuentes**    | Sans-serif únicamente: `Inter, Arial, sans-serif`          |
| **Opacidad**   | Permitida para efectos sutiles (máx. 0.2 para backgrounds) |

### Estructura de Comentario SVG

Todos los archivos SVG deben incluir este comentario al inicio:

```xml
<!--
  ACC LMS - [Nombre del Asset]
  Theme: Dark
  Gradients: NONE (solid colors only)
  Fonts: Sans-serif (Inter, Arial, system-ui)
-->
```

### ❌ Prohibido en SVG

```xml
<!-- NO USAR DEGRADADOS -->
<linearGradient id="gradient">...</linearGradient>
<radialGradient id="gradient">...</radialGradient>

<!-- NO USAR FUENTES CON SERIFA -->
font-family="Times New Roman"
font-family="Georgia"
font-family="serif"

<!-- NO USAR FONDOS CLAROS -->
fill="#ffffff" <!-- para backgrounds -->
```

### ✅ Permitido en SVG

```xml
<!-- COLORES SÓLIDOS -->
fill="#1a1a1a"
fill="#23a500"
fill="#ff7e05"

<!-- OPACIDAD PARA EFECTOS -->
fill="#23a500" opacity="0.15"

<!-- FUENTES SANS-SERIF -->
font-family="Inter, Arial, sans-serif"
```

---

#### Verde ACC (#23a500)

- Botones principales
- Enlaces importantes
- Estados de éxito
- Progress indicators
- Iconos principales

#### Naranja Accent (#ff7e05)

- Call-to-action secundarios
- Notificaciones importantes
- Badges y etiquetas
- Hover states
- Elementos interactivos

#### Gris Medio (#585858)

- Texto descriptivo
- Subtítulos
- Bordes sutiles
- Iconos secundarios
- Placeholders

#### Negro (#000000)

- Títulos principales
- Texto de lectura
- Navegación
- Headers importantes

#### Blanco (#ffffff)

- Fondos principales
- Cards y containers
- Texto sobre fondos oscuros
- Espacios negativos

## 🖼️ Recursos de Marca

### Logos

- `assets/logos/` - Versiones del logo en diferentes formatos
- Incluye versiones: horizontal, vertical, monocromo

### Banners

- `assets/banners/` - Banners para README, documentación
- Diferentes tamaños y orientaciones

### Iconografía

- `assets/icons/` - Iconos del sistema
- Consistentes con la marca

## 📐 Especificaciones Técnicas

### Typography Scale

```css
/* Jerarquía tipográfica sugerida */
h1: 2.5rem   /* Títulos principales */
h2: 2rem     /* Secciones importantes */
h3: 1.5rem   /* Subsecciones */
h4: 1.25rem  /* Elementos menores */
body: 1rem   /* Texto base */
small: 0.875rem /* Texto auxiliar */
```

### Spacing Scale

```css
/* Sistema de espaciado */
xs: 0.25rem  /* 4px */
sm: 0.5rem   /* 8px */
md: 1rem     /* 16px */
lg: 1.5rem   /* 24px */
xl: 2rem     /* 32px */
xxl: 3rem    /* 48px */
```

### Border Radius

```css
/* Consistencia en bordes */
small: 4px
medium: 8px
large: 12px
round: 50%
```

## 🎨 Ejemplos de Uso

### Botón Principal

```css
background: #23a500;
color: #ffffff;
border-radius: 8px;
```

### Botón Secundario

```css
background: #ff7e05;
color: #ffffff;
border-radius: 8px;
```

### Card/Container

```css
background: #ffffff;
border: 1px solid #585858;
border-radius: 12px;
```

### Alert Success

```css
background: rgba(35, 165, 0, 0.1);
border-left: 4px solid #23a500;
color: #000000;
```

### Alert Warning

```css
background: rgba(255, 126, 5, 0.1);
border-left: 4px solid #ff7e05;
color: #000000;
```

## 📱 Responsive Considerations

### Mobile First

- Priorizar legibilidad en pantallas pequeñas
- Mantener contraste adecuado
- Espaciado touch-friendly (min 44px)

### Accessibility

- Contraste mínimo: 4.5:1 para texto normal
- Contraste mínimo: 3:1 para texto grande
- No usar solo color para transmitir información

## 🔧 Implementación

### CSS Variables

```css
:root {
  --color-primary: #23a500;
  --color-accent: #ff7e05;
  --color-neutral: #585858;
  --color-text: #000000;
  --color-background: #ffffff;
}
```

### Tailwind Config

```javascript
module.exports = {
  theme: {
    colors: {
      'acc-green': '#23a500',
      'acc-orange': '#ff7e05',
      'acc-gray': '#585858',
      'acc-black': '#000000',
      'acc-white': '#ffffff',
    },
  },
};
```

### 🚫 Prohibiciones y Restricciones

#### ❌ Degradados (Gradients)

**PROHIBIDO** el uso de degradados en elementos de marca.

**Razones:**

- Impacto visual agresivo
- Problemas de accesibilidad
- Inconsistencia en diferentes dispositivos
- Dificultad de reproducción en print

**En su lugar usar:**

- Colores sólidos únicamente
- Transiciones suaves con opacidad
- Combinaciones balanceadas de colores

#### ❌ Otros elementos prohibidos

- Sombras excesivas (máximo 2px blur)
- Colores fuera de la paleta establecida
- Texto con contraste insuficiente
- Animaciones que parpadeen más de 3 veces por segundo

### ✅ Mejores Prácticas Visuales

#### Balance de Colores

- **Naranja**: 40% (predominante)
- **Verde**: 30% (secundario)
- **Gris**: 20% (soporte)
- **Negro/Blanco**: 10% (contraste)

#### Composición

- Usar elementos gráficos que enriquezcan (cerebro para IA, iconos educativos)
- Mantener jerarquía visual clara
- Respetar espacios en blanco
- Agrupar elementos relacionados
