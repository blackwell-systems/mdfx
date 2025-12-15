# Swatch Complete Guide

The swatch component is mdfx's most versatile visual primitive. This guide covers every parameter and configuration option.

---

## Basic Syntax

```markdown
{{ui:swatch:COLOR/}}
```

Where `COLOR` is either:
- A **palette name**: `accent`, `success`, `warning`, `error`, `info`, `slate`, `ui.bg`, `ui.surface`, `ui.panel`
- A **hex color**: `FF6B35`, `1a1a2e`, `22C55E` (no `#` prefix)

**Syntax:**

```markdown
{{ui:swatch:accent/}}          <!-- Uses palette accent color (F41C80) -->
{{ui:swatch:success/}}         <!-- Uses palette success (22C55E) -->
{{ui:swatch:FF6B35/}}          <!-- Direct hex color -->
{{ui:swatch:1a1a2e/}}          <!-- Dark color -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) ![](https://img.shields.io/badge/-%20-FF6B35?style=flat-square) ![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square)

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color` | hex/palette | *required* | The fill color (first positional argument) |
| `width` | integer | 20 | Width in pixels |
| `height` | integer | 20 | Height in pixels |
| `style` | enum | flat-square | Corner style and effects |
| `opacity` | float | 1.0 | Fill opacity (0.0 to 1.0) |
| `border` | hex/palette | none | Border color |
| `border_width` | integer | 1 | Border thickness in pixels |
| `label` | string | none | Text label centered on swatch |
| `label_color` | hex/palette | white | Label text color |
| `icon` | string | none | Icon name (displays abbreviation) |
| `icon_color` | hex/palette | white | Icon text color |

---

## Dimensions

### Width & Height

Control swatch size for any purpose:

**Syntax:**

```markdown
{{ui:swatch:accent:width=8:height=8/}}      <!-- Tiny indicator -->
{{ui:swatch:accent:width=20:height=20/}}    <!-- Standard badge -->
{{ui:swatch:accent:width=200:height=10/}}   <!-- Wide bar -->
{{ui:swatch:accent:width=10:height=100/}}   <!-- Tall column -->
{{ui:swatch:accent:width=300:height=80/}}   <!-- Large panel -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) Tiny

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) Standard

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) Wide bar

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) Tall

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) Large panel

### Pixel Art

Create pixel art with small swatches:

**Syntax:**

```markdown
{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-FF0000?style=flat-square)![](https://img.shields.io/badge/-%20-00FF00?style=flat-square)![](https://img.shields.io/badge/-%20-0000FF?style=flat-square)

---

## Styles

The `style` parameter changes corner radius and effects:

| Style | Corners | Effect | Best For |
|-------|---------|--------|----------|
| `flat-square` | Sharp (rx=0) | None | Modern, technical |
| `flat` | Rounded (rx=3) | None | Friendly, approachable |
| `plastic` | Rounded (rx=3) | Gradient shine | Glossy, 3D look |
| `for-the-badge` | Rounded (rx=3) | Taller (28px) | Headers, emphasis |
| `social` | Very rounded (rx=10) | Pill shape | Buttons, tags |

**Syntax:**

```markdown
{{ui:swatch:accent:style=flat-square/}}   <!-- Sharp corners -->
{{ui:swatch:accent:style=flat/}}          <!-- Slightly rounded -->
{{ui:swatch:accent:style=plastic/}}       <!-- Shiny 3D effect -->
{{ui:swatch:accent:style=for-the-badge/}} <!-- Taller badge -->
{{ui:swatch:accent:style=social/}}        <!-- Pill/capsule shape -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) flat-square
![](https://img.shields.io/badge/-%20-F41C80?style=flat) flat
![](https://img.shields.io/badge/-%20-F41C80?style=plastic) plastic
![](https://img.shields.io/badge/-%20-F41C80?style=for-the-badge) for-the-badge
![](https://img.shields.io/badge/-%20-F41C80?style=social) social

---

## Opacity

Control transparency with `opacity` (0.0 to 1.0):

**Syntax:**

```markdown
{{ui:swatch:accent:opacity=1.0/}}   <!-- Fully opaque (default) -->
{{ui:swatch:accent:opacity=0.75/}}  <!-- 75% visible -->
{{ui:swatch:accent:opacity=0.5/}}   <!-- Half transparent -->
{{ui:swatch:accent:opacity=0.25/}}  <!-- 25% visible -->
{{ui:swatch:accent:opacity=0.1/}}   <!-- Nearly invisible -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) 100%
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) 75%
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) 50%
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) 25%
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) 10%

### Depth Illusion

Create layered depth effects:

**Syntax:**

```markdown
{{ui:swatch:F41C80:width=200:height=40:opacity=1.0/}}
{{ui:swatch:F41C80:width=180:height=40:opacity=0.75/}}
{{ui:swatch:F41C80:width=160:height=40:opacity=0.50/}}
{{ui:swatch:F41C80:width=140:height=40:opacity=0.25/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)

### Invisible Spacers

Use `opacity=0` for invisible spacing:

```markdown
{{ui:swatch:000000:width=50:height=20:opacity=0/}}  <!-- Invisible spacer -->
```

---

## Borders

Add borders with `border` (color) and `border_width`:

**Syntax:**

```markdown
{{ui:swatch:1a1a2e:border=F41C80/}}                                         <!-- Simple border -->
{{ui:swatch:1a1a2e:border=F41C80:border_width=3/}}                          <!-- Thick border -->
{{ui:swatch:1a1a2e:width=100:height=60:border=22C55E:border_width=5/}}      <!-- Very thick frame -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Simple
![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Thick
![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Very thick

### Border Styles

**Syntax:**

```markdown
{{ui:swatch:1a1a2e:width=80:height=40:border=333333:border_width=1/}}  <!-- Subtle outline -->
{{ui:swatch:1a1a2e:width=80:height=40:border=FFFFFF:border_width=3/}}  <!-- Bold frame -->
{{ui:swatch:1a1a2e:width=80:height=40:border=F41C80:border_width=2/}}  <!-- Accent highlight -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Subtle
![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Bold
![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square) Accent

---

## Labels

Add text labels with `label` and optional `label_color`:

**Syntax:**

```markdown
{{ui:swatch:accent:width=80:height=30:label=ACTIVE/}}
{{ui:swatch:1a1a2e:width=100:height=40:label=DARK MODE:label_color=FFFFFF/}}
{{ui:swatch:22C55E:width=120:height=35:label=ONLINE/}}
{{ui:swatch:EF4444:width=120:height=35:label=OFFLINE/}}
{{ui:swatch:EAB308:width=120:height=35:label=PENDING/}}
```

**Rendered:**

![](https://img.shields.io/badge/-ACTIVE-F41C80?style=flat-square)
![](https://img.shields.io/badge/-DARK%20MODE-1A1A2E?style=flat-square)
![](https://img.shields.io/badge/-ONLINE-22C55E?style=flat-square)
![](https://img.shields.io/badge/-OFFLINE-EF4444?style=flat-square)
![](https://img.shields.io/badge/-PENDING-EAB308?style=flat-square)

### Complex Labels

**Syntax:**

```markdown
{{ui:swatch:22C55E:width=100:height=30:label=99.9%/}}
{{ui:swatch:3B82F6:width=140:height=30:label=Coverage 94%/}}
{{ui:swatch:1a1a2e:width=200:height=30:border=00FF00:border_width=1:label=ACCESS GRANTED/}}
```

**Rendered:**

![](https://img.shields.io/badge/-99.9%-22C55E?style=flat-square)
![](https://img.shields.io/badge/-Coverage%2094%-3B82F6?style=flat-square)
![](https://img.shields.io/badge/-ACCESS%20GRANTED-1A1A2E?style=flat-square)

---

## Icons

Add icon abbreviations with `icon` and optional `icon_color`:

**Syntax:**

```markdown
{{ui:swatch:000000:width=60:height=40:icon=rust:icon_color=DEA584/}}
{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF/}}
{{ui:swatch:2496ED:width=60:height=40:icon=docker:icon_color=FFFFFF/}}
{{ui:swatch:000000:width=60:height=40:icon=github:icon_color=FFFFFF/}}
{{ui:swatch:FC6D26:width=60:height=40:icon=gitlab:icon_color=FFFFFF/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=rust&logoColor=DEA584&label=&labelColor=000000)
![](https://img.shields.io/badge/-%20-3178C6?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=3178C6)
![](https://img.shields.io/badge/-%20-2496ED?style=flat-square&logo=docker&logoColor=FFFFFF&label=&labelColor=2496ED)
![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=github&logoColor=FFFFFF&label=&labelColor=000000)
![](https://img.shields.io/badge/-%20-FC6D26?style=flat-square&logo=gitlab&logoColor=FFFFFF&label=&labelColor=FC6D26)

**Note:** Icons render as 3-letter abbreviations (e.g., "RUS" for rust). Full SVG icons require bundling icon libraries.

---

## Color Palette Reference

Built-in palette colors:

| Name | Hex | Swatch |
|------|-----|--------|
| `accent` | F41C80 | ![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) |
| `success` | 22C55E | ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) |
| `warning` | EAB308 | ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) |
| `error` | EF4444 | ![](https://img.shields.io/badge/-%20-EF4444?style=flat-square) |
| `info` | 3B82F6 | ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) |
| `slate` | 6B7280 | ![](https://img.shields.io/badge/-%20-6B7280?style=flat-square) |
| `ui.bg` | 292A2D | ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square) |
| `ui.surface` | 292C34 | ![](https://img.shields.io/badge/-%20-292C34?style=flat-square) |
| `ui.panel` | 282F3C | ![](https://img.shields.io/badge/-%20-282F3C?style=flat-square) |
| `white` | FFFFFF | ![](https://img.shields.io/badge/-%20-FFFFFF?style=flat-square) |
| `black` | 000000 | ![](https://img.shields.io/badge/-%20-000000?style=flat-square) |

---

## Complete Examples

### Status Dashboard

**Syntax:**

```markdown
{{ui:swatch:22C55E:width=15:height=15:style=social/}} API Server: Online
{{ui:swatch:22C55E:width=15:height=15:style=social/}} Database: Healthy
{{ui:swatch:EAB308:width=15:height=15:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:width=15:height=15:style=social/}} Queue: Critical
```

**Rendered:**

![](https://img.shields.io/badge/-%20-22C55E?style=social) API Server: Online
![](https://img.shields.io/badge/-%20-22C55E?style=social) Database: Healthy
![](https://img.shields.io/badge/-%20-EAB308?style=social) Cache: Degraded
![](https://img.shields.io/badge/-%20-EF4444?style=social) Queue: Critical

### Progress Bar

**Syntax:**

```markdown
{{ui:swatch:22C55E:width=200:height=20:label=Progress 75%/}}{{ui:swatch:333333:width=67:height=20/}}
```

**Rendered:**

![](https://img.shields.io/badge/-Progress%2075%-22C55E?style=flat-square)![](https://img.shields.io/badge/-%20-333333?style=flat-square)

### Color Palette Documentation

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:1a1a2e:width=60:height=80:label=Primary/}}
{{ui:swatch:2d2d44:width=60:height=80:label=Secondary/}}
{{ui:swatch:4a4a6a:width=60:height=80:label=Tertiary/}}
{{ui:swatch:6b6b8d:width=60:height=80:label=Muted/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-Primary-1A1A2E?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Secondary-2D2D44?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Tertiary-4A4A6A?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Muted-6B6B8D?style=flat-square">
</p>

### Heat Map Row

**Syntax:**

```markdown
{{ui:swatch:0a0a0a:width=30:height=30/}}{{ui:swatch:1a0a0a:width=30:height=30/}}{{ui:swatch:3a1010:width=30:height=30/}}{{ui:swatch:5a1a1a:width=30:height=30/}}{{ui:swatch:8B2500:width=30:height=30/}}{{ui:swatch:CD3700:width=30:height=30/}}{{ui:swatch:FF4500:width=30:height=30/}}{{ui:swatch:FF6347:width=30:height=30/}}{{ui:swatch:FFD700:width=30:height=30/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-1A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-3A1010?style=flat-square)![](https://img.shields.io/badge/-%20-5A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-8B2500?style=flat-square)![](https://img.shields.io/badge/-%20-CD3700?style=flat-square)![](https://img.shields.io/badge/-%20-FF4500?style=flat-square)![](https://img.shields.io/badge/-%20-FF6347?style=flat-square)![](https://img.shields.io/badge/-%20-FFD700?style=flat-square)

### Tech Stack Monument

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:DEA584:width=60:height=40:icon=rust:icon_color=000000:border=DEA584:border_width=2/}}
{{ui:swatch:F7DF1E:width=60:height=40:icon=javascript:icon_color=000000:border=F7DF1E:border_width=2/}}
{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF:border=3178C6:border_width=2/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-%20-DEA584?style=flat-square&logo=rust&logoColor=000000&label=&labelColor=DEA584"> <img alt="" src="https://img.shields.io/badge/-%20-F7DF1E?style=flat-square&logo=javascript&logoColor=000000&label=&labelColor=F7DF1E"> <img alt="" src="https://img.shields.io/badge/-%20-3178C6?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=3178C6">
</p>

### Hazard Warnings

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:FFFF00:width=100:height=50:border=000000:border_width=3:label=RADIATION/}}
{{ui:swatch:FF6600:width=100:height=50:border=000000:border_width=3:label=BIOHAZARD/}}
{{ui:swatch:FF0000:width=100:height=50:border=000000:border_width=3:label=DANGER/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-RADIATION-FFFF00?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-BIOHAZARD-FF6600?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-DANGER-FF0000?style=flat-square">
</p>

### Glassmorphism Effect

**Syntax:**

```markdown
{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}
{{ui:swatch:FFFFFF:width=180:height=50:opacity=0.2:border=FFFFFF:border_width=1:label=Glass Card/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-FFFFFF?style=flat-square)
![](https://img.shields.io/badge/-Glass%20Card-FFFFFF?style=flat-square)

### Pixel Art Eye

**Syntax:**

```markdown
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}
{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)
![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)
![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-DC143C?style=flat-square)![](https://img.shields.io/badge/-%20-DC143C?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)
![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-DC143C?style=flat-square)![](https://img.shields.io/badge/-%20-DC143C?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)
![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-1A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-C0C0C0?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)
![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-708090?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Limited to:
- Color
- Style
- Basic dimensions through badge sizing

```bash
mdfx process template.md
```

### SVG Backend

Full control over all parameters:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Generates local `.svg` files with:
- Custom width/height
- Opacity
- Borders
- Labels
- Icons

---

## Tips & Tricks

### 1. Combine with Row for Centering

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:accent:width=100:height=50:label=CENTERED/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-CENTERED-F41C80?style=flat-square">
</p>

### 2. Use Opacity for Layering

Stack swatches with decreasing opacity for depth:

**Syntax:**

```markdown
{{ui:swatch:accent:width=100:height=30:opacity=1.0/}}
{{ui:swatch:accent:width=90:height=30:opacity=0.7/}}
{{ui:swatch:accent:width=80:height=30:opacity=0.4/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)
![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)

### 3. Invisible Spacers for Layout

```markdown
{{ui:swatch:000:width=50:height=1:opacity=0/}}  <!-- Horizontal space -->
```

### 4. Border as Highlight

Use bright borders on dark swatches:

**Syntax:**

```markdown
{{ui:swatch:1a1a2e:width=200:height=60:border=F41C80:border_width=3:label=HIGHLIGHTED/}}
```

**Rendered:**

![](https://img.shields.io/badge/-HIGHLIGHTED-1A1A2E?style=flat-square)

### 5. Combine Multiple Parameters

**Syntax:**

```markdown
{{ui:swatch:1a1a2e:width=300:height=80:style=social:opacity=0.9:border=00FF00:border_width=2:label=COMPLETE EXAMPLE/}}
```

**Rendered:**

![](https://img.shields.io/badge/-COMPLETE%20EXAMPLE-1A1A2E?style=social)

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md)
- [API Guide](../API-GUIDE.md)
- [Examples README](../../examples/README.md)

---

<p align="center">
ʀᴇɴᴅᴇʀᴇᴅ ᴡɪᴛʜ ᴍᴅꜰx
</p>
