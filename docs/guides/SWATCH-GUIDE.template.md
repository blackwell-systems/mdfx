# Swatch Complete Guide

The swatch component is mdfx's most versatile visual primitive. This guide covers every parameter and configuration option.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [All Parameters](#all-parameters)
- [Dimensions](#dimensions)
- [Styles](#styles)
- [Opacity](#opacity)
- [Borders](#borders)
- [Labels](#labels)
- [Icons](#icons)
- [Color Palette Reference](#color-palette-reference)
- [Complete Examples](#complete-examples)
- [Backend Differences](#backend-differences)
- [Tips & Tricks](#tips--tricks)

---

## Basic Syntax

```markdown
{{ui:swatch:COLOR/}}
```

Where `COLOR` is either:
- A **palette name**: `accent`, `success`, `warning`, `error`, `info`, `slate`
- A **hex color**: `FF6B35`, `1a1a2e`, `22C55E` (no `#` prefix)

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent/}}` | {{ui:swatch:accent/}} |
| `{{ui:swatch:success/}}` | {{ui:swatch:success/}} |
| `{{ui:swatch:warning/}}` | {{ui:swatch:warning/}} |
| `{{ui:swatch:error/}}` | {{ui:swatch:error/}} |
| `{{ui:swatch:FF6B35/}}` | {{ui:swatch:FF6B35/}} |
| `{{ui:swatch:1a1a2e/}}` | {{ui:swatch:1a1a2e/}} |

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

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent:width=8:height=8/}}` | {{ui:swatch:accent:width=8:height=8/}} |
| `{{ui:swatch:accent:width=20:height=20/}}` | {{ui:swatch:accent:width=20:height=20/}} |
| `{{ui:swatch:accent:width=40:height=20/}}` | {{ui:swatch:accent:width=40:height=20/}} |
| `{{ui:swatch:accent:width=100:height=20/}}` | {{ui:swatch:accent:width=100:height=20/}} |
| `{{ui:swatch:accent:width=200:height=10/}}` | {{ui:swatch:accent:width=200:height=10/}} |

### Wide Bars

```markdown
{{ui:swatch:success:width=200:height=10/}}
{{ui:swatch:info:width=200:height=10/}}
```

{{ui:swatch:success:width=200:height=10/}}
{{ui:swatch:info:width=200:height=10/}}

### Tall Columns

```markdown
{{ui:swatch:warning:width=20:height=60/}}
{{ui:swatch:error:width=20:height=60/}}
```

{{ui:swatch:warning:width=20:height=60/}} {{ui:swatch:error:width=20:height=60/}}

### Pixel Art

Create pixel art with small swatches:

```markdown
{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}
```

{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}

---

## Styles

The `style` parameter changes corner radius and effects:

| Style | Description | Example |
|-------|-------------|---------|
| `flat-square` | Sharp corners (default) | {{ui:swatch:accent:width=60:height=25:style=flat-square/}} |
| `flat` | Slightly rounded | {{ui:swatch:accent:width=60:height=25:style=flat/}} |
| `plastic` | Glossy 3D effect | {{ui:swatch:accent:width=60:height=25:style=plastic/}} |
| `for-the-badge` | Taller badge style | {{ui:swatch:accent:width=60:height=28:style=for-the-badge/}} |
| `social` | Pill/capsule shape | {{ui:swatch:accent:width=60:height=25:style=social/}} |

---

## Opacity

Control transparency with `opacity` (0.0 to 1.0):

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent:width=50:height=25:opacity=1.0/}}` | {{ui:swatch:accent:width=50:height=25:opacity=1.0/}} |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.75/}}` | {{ui:swatch:accent:width=50:height=25:opacity=0.75/}} |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.5/}}` | {{ui:swatch:accent:width=50:height=25:opacity=0.5/}} |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.25/}}` | {{ui:swatch:accent:width=50:height=25:opacity=0.25/}} |

### Depth Illusion

Create layered depth effects:

```markdown
{{ui:swatch:F41C80:width=200:height=30:opacity=1.0/}}
{{ui:swatch:F41C80:width=180:height=30:opacity=0.75/}}
{{ui:swatch:F41C80:width=160:height=30:opacity=0.50/}}
{{ui:swatch:F41C80:width=140:height=30:opacity=0.25/}}
```

{{ui:swatch:F41C80:width=200:height=30:opacity=1.0/}}
{{ui:swatch:F41C80:width=180:height=30:opacity=0.75/}}
{{ui:swatch:F41C80:width=160:height=30:opacity=0.50/}}
{{ui:swatch:F41C80:width=140:height=30:opacity=0.25/}}

### Invisible Spacers

Use `opacity=0` for invisible spacing:

```markdown
{{ui:swatch:000000:width=50:height=20:opacity=0/}}
```

---

## Borders

Add borders with `border` (color) and `border_width`:

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:1a1a2e:width=60:height=30:border=F41C80/}}` | {{ui:swatch:1a1a2e:width=60:height=30:border=F41C80/}} |
| `{{ui:swatch:1a1a2e:width=60:height=30:border=22C55E/}}` | {{ui:swatch:1a1a2e:width=60:height=30:border=22C55E/}} |
| `{{ui:swatch:1a1a2e:width=60:height=30:border=3B82F6/}}` | {{ui:swatch:1a1a2e:width=60:height=30:border=3B82F6/}} |

### Border Widths

| Syntax | Result |
|--------|--------|
| `...:border=FFFFFF:border_width=1/}}` | {{ui:swatch:292A2D:width=80:height=40:border=FFFFFF:border_width=1/}} |
| `...:border=FFFFFF:border_width=2/}}` | {{ui:swatch:292A2D:width=80:height=40:border=FFFFFF:border_width=2/}} |
| `...:border=FFFFFF:border_width=4/}}` | {{ui:swatch:292A2D:width=80:height=40:border=FFFFFF:border_width=4/}} |

### Glassmorphism Effect

```markdown
{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}
```

{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}

---

## Labels

Add text labels with `label` and optional `label_color`:

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent:width=80:height=30:label=ACTIVE/}}` | {{ui:swatch:accent:width=80:height=30:label=ACTIVE/}} |
| `{{ui:swatch:success:width=80:height=30:label=ONLINE/}}` | {{ui:swatch:success:width=80:height=30:label=ONLINE/}} |
| `{{ui:swatch:error:width=80:height=30:label=OFFLINE/}}` | {{ui:swatch:error:width=80:height=30:label=OFFLINE/}} |
| `{{ui:swatch:warning:width=80:height=30:label=PENDING/}}` | {{ui:swatch:warning:width=80:height=30:label=PENDING/}} |

### Status Labels

```markdown
{{ui:swatch:22C55E:width=120:height=35:label=BUILD PASSING/}}
{{ui:swatch:EF4444:width=120:height=35:label=BUILD FAILED/}}
```

{{ui:swatch:22C55E:width=120:height=35:label=BUILD PASSING/}} {{ui:swatch:EF4444:width=120:height=35:label=BUILD FAILED/}}

### Custom Label Colors

```markdown
{{ui:swatch:1a1a2e:width=150:height=40:label=DARK MODE:label_color=FFFFFF/}}
```

{{ui:swatch:1a1a2e:width=150:height=40:label=DARK MODE:label_color=FFFFFF/}}

---

## Icons

Add icon abbreviations with `icon` and optional `icon_color`:

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:000000:width=60:height=40:icon=rust:icon_color=DEA584/}}` | {{ui:swatch:000000:width=60:height=40:icon=rust:icon_color=DEA584/}} |
| `{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF/}}` | {{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF/}} |
| `{{ui:swatch:3776AB:width=60:height=40:icon=python:icon_color=FFD43B/}}` | {{ui:swatch:3776AB:width=60:height=40:icon=python:icon_color=FFD43B/}} |
| `{{ui:swatch:2496ED:width=60:height=40:icon=docker:icon_color=FFFFFF/}}` | {{ui:swatch:2496ED:width=60:height=40:icon=docker:icon_color=FFFFFF/}} |

**Note:** Icons render as 3-letter abbreviations. Full SVG icons require bundling icon libraries.

---

## Color Palette Reference

Built-in palette colors:

| Name | Hex | Swatch |
|------|-----|--------|
| `accent` | F41C80 | {{ui:swatch:accent:width=40:height=20/}} |
| `success` | 22C55E | {{ui:swatch:success:width=40:height=20/}} |
| `warning` | EAB308 | {{ui:swatch:warning:width=40:height=20/}} |
| `error` | EF4444 | {{ui:swatch:error:width=40:height=20/}} |
| `info` | 3B82F6 | {{ui:swatch:info:width=40:height=20/}} |
| `slate` | 6B7280 | {{ui:swatch:slate:width=40:height=20/}} |

---

## Complete Examples

### Status Dashboard

```markdown
{{ui:swatch:22C55E:width=12:height=12:style=social/}} API Server: Online
{{ui:swatch:22C55E:width=12:height=12:style=social/}} Database: Healthy
{{ui:swatch:EAB308:width=12:height=12:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:width=12:height=12:style=social/}} Queue: Critical
```

{{ui:swatch:22C55E:width=12:height=12:style=social/}} API Server: Online
{{ui:swatch:22C55E:width=12:height=12:style=social/}} Database: Healthy
{{ui:swatch:EAB308:width=12:height=12:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:width=12:height=12:style=social/}} Queue: Critical

### Progress Bar

```markdown
{{ui:swatch:22C55E:width=150:height=20/}}{{ui:swatch:333333:width=50:height=20/}}
```

{{ui:swatch:22C55E:width=150:height=20/}}{{ui:swatch:333333:width=50:height=20/}}

### Color Palette Display

```markdown
{{ui:swatch:1a1a2e:width=50:height=60/}}{{ui:swatch:2d2d44:width=50:height=60/}}{{ui:swatch:4a4a6a:width=50:height=60/}}{{ui:swatch:6b6b8d:width=50:height=60/}}
```

{{ui:swatch:1a1a2e:width=50:height=60/}}{{ui:swatch:2d2d44:width=50:height=60/}}{{ui:swatch:4a4a6a:width=50:height=60/}}{{ui:swatch:6b6b8d:width=50:height=60/}}

### Heat Map Row

```markdown
{{ui:swatch:0a0a0a:width=25:height=25/}}{{ui:swatch:3a1010:width=25:height=25/}}{{ui:swatch:8B2500:width=25:height=25/}}{{ui:swatch:CD3700:width=25:height=25/}}{{ui:swatch:FF4500:width=25:height=25/}}{{ui:swatch:FF6347:width=25:height=25/}}{{ui:swatch:FFD700:width=25:height=25/}}
```

{{ui:swatch:0a0a0a:width=25:height=25/}}{{ui:swatch:3a1010:width=25:height=25/}}{{ui:swatch:8B2500:width=25:height=25/}}{{ui:swatch:CD3700:width=25:height=25/}}{{ui:swatch:FF4500:width=25:height=25/}}{{ui:swatch:FF6347:width=25:height=25/}}{{ui:swatch:FFD700:width=25:height=25/}}

### Tech Stack

```markdown
{{ui:swatch:DEA584:width=50:height=35:icon=rust:icon_color=000000/}}
{{ui:swatch:F7DF1E:width=50:height=35:icon=js:icon_color=000000/}}
{{ui:swatch:3178C6:width=50:height=35:icon=ts:icon_color=FFFFFF/}}
```

{{ui:swatch:DEA584:width=50:height=35:icon=rust:icon_color=000000/}} {{ui:swatch:F7DF1E:width=50:height=35:icon=js:icon_color=000000/}} {{ui:swatch:3178C6:width=50:height=35:icon=ts:icon_color=FFFFFF/}}

### Hazard Warnings

```markdown
{{ui:swatch:FFFF00:width=80:height=40:border=000000:border_width=2:label=CAUTION/}}
{{ui:swatch:FF6600:width=80:height=40:border=000000:border_width=2:label=WARNING/}}
{{ui:swatch:FF0000:width=80:height=40:border=000000:border_width=2:label=DANGER/}}
```

{{ui:swatch:FFFF00:width=80:height=40:border=000000:border_width=2:label=CAUTION/}} {{ui:swatch:FF6600:width=80:height=40:border=000000:border_width=2:label=WARNING/}} {{ui:swatch:FF0000:width=80:height=40:border=000000:border_width=2:label=DANGER/}}

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Limited to basic color and style:

```bash
mdfx process template.md
```

### SVG Backend

Full control over all parameters:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Generates local `.svg` files with custom dimensions, opacity, borders, labels, and icons.

---

## Tips & Tricks

### 1. Invisible Spacers for Layout

```markdown
{{ui:swatch:000:width=50:height=1:opacity=0/}}
```

### 2. Border as Highlight

```markdown
{{ui:swatch:1a1a2e:width=200:height=50:border=F41C80:border_width=3:label=HIGHLIGHTED/}}
```

{{ui:swatch:1a1a2e:width=200:height=50:border=F41C80:border_width=3:label=HIGHLIGHTED/}}

### 3. Combine Multiple Parameters

```markdown
{{ui:swatch:1a1a2e:width=250:height=60:style=social:opacity=0.9:border=00FF00:border_width=2:label=COMPLETE/}}
```

{{ui:swatch:1a1a2e:width=250:height=60:style=social:opacity=0.9:border=00FF00:border_width=2:label=COMPLETE/}}

---

## See Also

- [Components Reference](../COMPONENTS.md)
- [Frames Guide](FRAMES-GUIDE.md)
- [CLI Guide](CLI-GUIDE.md)
