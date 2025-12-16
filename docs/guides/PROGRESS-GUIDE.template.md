# Progress Bar Complete Guide

The progress bar component is a versatile horizontal bar for displaying percentages, skill levels, loading states, and interactive sliders. This guide covers every parameter and configuration option.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [All Parameters](#all-parameters)
- [Dimensions](#dimensions)
- [Colors](#colors)
- [Corner Radius](#corner-radius)
- [Floating Fill Effect](#floating-fill-effect)
- [Labels](#labels)
- [Borders](#borders)
- [Slider Mode](#slider-mode)
- [Thumb Shapes](#thumb-shapes)
- [Thumb Width (Oval/Pill)](#thumb-width-ovalpill)
- [Complete Examples](#complete-examples)
- [Backend Differences](#backend-differences)
- [Tips & Tricks](#tips--tricks)

---

## Basic Syntax

```markdown
{{ui:progress:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:progress:0/}}` | {{ui:progress:0/}} |
| `{{ui:progress:25/}}` | {{ui:progress:25/}} |
| `{{ui:progress:50/}}` | {{ui:progress:50/}} |
| `{{ui:progress:75/}}` | {{ui:progress:75/}} |
| `{{ui:progress:100/}}` | {{ui:progress:100/}} |

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
| `width` | number | 100 | Total width in pixels |
| `height` | number | 10 | Track height in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `fill_height` | number | height | Fill height (less creates floating effect) |
| `rx` | number | 3 | Corner radius |
| `label` | boolean | false | Show percentage label |
| `label_color` | color | white | Label text color |
| `border` | color | none | Border color |
| `border_width` | number | 1 | Border width in pixels |
| `thumb` | number | none | Thumb height (enables slider mode) |
| `thumb_width` | number | thumb | Thumb width (for oval/pill shapes) |
| `thumb_color` | color | fill | Thumb color |
| `thumb_shape` | string | circle | Thumb shape: circle, square, diamond |

---

## Dimensions

### Width Variations

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=80/}}` | {{ui:progress:50:width=80/}} |
| `{{ui:progress:50:width=120/}}` | {{ui:progress:50:width=120/}} |
| `{{ui:progress:50:width=200/}}` | {{ui:progress:50:width=200/}} |
| `{{ui:progress:50:width=300/}}` | {{ui:progress:50:width=300/}} |

### Height Variations

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:height=4/}}` | {{ui:progress:60:width=150:height=4/}} |
| `{{ui:progress:60:width=150:height=8/}}` | {{ui:progress:60:width=150:height=8/}} |
| `{{ui:progress:60:width=150:height=16/}}` | {{ui:progress:60:width=150:height=16/}} |
| `{{ui:progress:60:width=150:height=24/}}` | {{ui:progress:60:width=150:height=24/}} |

---

## Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:70:width=150:fill=accent/}}` | {{ui:progress:70:width=150:fill=accent/}} |
| `{{ui:progress:70:width=150:fill=success/}}` | {{ui:progress:70:width=150:fill=success/}} |
| `{{ui:progress:70:width=150:fill=warning/}}` | {{ui:progress:70:width=150:fill=warning/}} |
| `{{ui:progress:70:width=150:fill=error/}}` | {{ui:progress:70:width=150:fill=error/}} |
| `{{ui:progress:70:width=150:fill=info/}}` | {{ui:progress:70:width=150:fill=info/}} |
| `{{ui:progress:70:width=150:fill=cobalt/}}` | {{ui:progress:70:width=150:fill=cobalt/}} |

### Track Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:track=slate/}}` | {{ui:progress:60:width=150:track=slate/}} |
| `{{ui:progress:60:width=150:track=ink/}}` | {{ui:progress:60:width=150:track=ink/}} |
| `{{ui:progress:60:width=150:track=ui.panel/}}` | {{ui:progress:60:width=150:track=ui.panel/}} |
| `{{ui:progress:60:width=150:track=333333/}}` | {{ui:progress:60:width=150:track=333333/}} |

### Custom Hex Colors

```markdown
{{ui:progress:75:width=200:fill=FF6B35:track=1a1a2e/}}
{{ui:progress:60:width=200:fill=00FF41:track=0D0D0D/}}
{{ui:progress:85:width=200:fill=FF00FF:track=1a0a1a/}}
```

{{ui:progress:75:width=200:fill=FF6B35:track=1a1a2e/}}
{{ui:progress:60:width=200:fill=00FF41:track=0D0D0D/}}
{{ui:progress:85:width=200:fill=FF00FF:track=1a0a1a/}}

---

## Corner Radius

Control corner roundness with the `rx` parameter:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:65:width=150:height=12:rx=0/}}` | {{ui:progress:65:width=150:height=12:rx=0/}} |
| `{{ui:progress:65:width=150:height=12:rx=2/}}` | {{ui:progress:65:width=150:height=12:rx=2/}} |
| `{{ui:progress:65:width=150:height=12:rx=4/}}` | {{ui:progress:65:width=150:height=12:rx=4/}} |
| `{{ui:progress:65:width=150:height=12:rx=6/}}` | {{ui:progress:65:width=150:height=12:rx=6/}} |

### Pill Shape

Set `rx` to half of `height` for perfect pill shape:

```markdown
{{ui:progress:70:width=200:height=16:rx=8/}}
```

{{ui:progress:70:width=200:height=16:rx=8/}}

---

## Floating Fill Effect

When `fill_height` is less than `height`, the fill "floats" inside the track:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=180:height=16:fill_height=16/}}` | {{ui:progress:60:width=180:height=16:fill_height=16/}} |
| `{{ui:progress:60:width=180:height=16:fill_height=12/}}` | {{ui:progress:60:width=180:height=16:fill_height=12/}} |
| `{{ui:progress:60:width=180:height=16:fill_height=8/}}` | {{ui:progress:60:width=180:height=16:fill_height=8/}} |
| `{{ui:progress:60:width=180:height=16:fill_height=4/}}` | {{ui:progress:60:width=180:height=16:fill_height=4/}} |

### Elegant Thin Fill

```markdown
{{ui:progress:80:width=250:height=20:fill_height=6:fill=accent:track=ui.panel/}}
```

{{ui:progress:80:width=250:height=20:fill_height=6:fill=accent:track=ui.panel/}}

---

## Labels

Show percentage text with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:25:width=150:height=18:label=true/}}` | {{ui:progress:25:width=150:height=18:label=true/}} |
| `{{ui:progress:50:width=150:height=18:label=true/}}` | {{ui:progress:50:width=150:height=18:label=true/}} |
| `{{ui:progress:75:width=150:height=18:label=true/}}` | {{ui:progress:75:width=150:height=18:label=true/}} |
| `{{ui:progress:100:width=150:height=18:label=true/}}` | {{ui:progress:100:width=150:height=18:label=true/}} |

### Custom Label Colors

```markdown
{{ui:progress:65:width=180:height=20:label=true:label_color=000000:fill=warning/}}
```

{{ui:progress:65:width=180:height=20:label=true:label_color=000000:fill=warning/}}

---

## Borders

Add borders with `border` and `border_width`:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:height=12:border=accent/}}` | {{ui:progress:60:width=150:height=12:border=accent/}} |
| `{{ui:progress:60:width=150:height=12:border=success/}}` | {{ui:progress:60:width=150:height=12:border=success/}} |
| `{{ui:progress:60:width=150:height=12:border=FFFFFF/}}` | {{ui:progress:60:width=150:height=12:border=FFFFFF/}} |

### Border Widths

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=1/}}` | {{ui:progress:50:width=150:height=14:border=accent:border_width=1/}} |
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=2/}}` | {{ui:progress:50:width=150:height=14:border=accent:border_width=2/}} |
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=3/}}` | {{ui:progress:50:width=150:height=14:border=accent:border_width=3/}} |

---

## Slider Mode

Add a `thumb` parameter to enable slider mode with a draggable-style thumb indicator:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:30:width=180:height=6:thumb=14/}}` | {{ui:progress:30:width=180:height=6:thumb=14/}} |
| `{{ui:progress:50:width=180:height=6:thumb=14/}}` | {{ui:progress:50:width=180:height=6:thumb=14/}} |
| `{{ui:progress:70:width=180:height=6:thumb=14/}}` | {{ui:progress:70:width=180:height=6:thumb=14/}} |
| `{{ui:progress:90:width=180:height=6:thumb=14/}}` | {{ui:progress:90:width=180:height=6:thumb=14/}} |

### Thumb Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:height=6:thumb=10/}}` | {{ui:progress:50:width=180:height=6:thumb=10/}} |
| `{{ui:progress:50:width=180:height=6:thumb=14/}}` | {{ui:progress:50:width=180:height=6:thumb=14/}} |
| `{{ui:progress:50:width=180:height=6:thumb=18/}}` | {{ui:progress:50:width=180:height=6:thumb=18/}} |
| `{{ui:progress:50:width=180:height=6:thumb=22/}}` | {{ui:progress:50:width=180:height=6:thumb=22/}} |

### Custom Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=14:thumb_color=accent/}}` | {{ui:progress:50:width=180:thumb=14:thumb_color=accent/}} |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=success/}}` | {{ui:progress:50:width=180:thumb=14:thumb_color=success/}} |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=warning/}}` | {{ui:progress:50:width=180:thumb=14:thumb_color=warning/}} |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=white/}}` | {{ui:progress:50:width=180:thumb=14:thumb_color=white/}} |

---

## Thumb Shapes

Choose between circle, square, or diamond thumbs:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=circle/}}` | {{ui:progress:50:width=180:thumb=14:thumb_shape=circle/}} |
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=square/}}` | {{ui:progress:50:width=180:thumb=14:thumb_shape=square/}} |
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=diamond/}}` | {{ui:progress:50:width=180:thumb=14:thumb_shape=diamond/}} |

### Colored Shape Variations

```markdown
{{ui:progress:65:width=200:thumb=16:thumb_shape=diamond:thumb_color=warning/}}
{{ui:progress:45:width=200:thumb=16:thumb_shape=square:thumb_color=info/}}
```

{{ui:progress:65:width=200:thumb=16:thumb_shape=diamond:thumb_color=warning/}}
{{ui:progress:45:width=200:thumb=16:thumb_shape=square:thumb_color=info/}}

---

## Thumb Width (Oval/Pill)

Use `thumb_width` to create oval or pill-shaped thumbs:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=12:thumb_width=12/}}` | {{ui:progress:50:width=180:thumb=12:thumb_width=12/}} |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=18/}}` | {{ui:progress:50:width=180:thumb=12:thumb_width=18/}} |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=24/}}` | {{ui:progress:50:width=180:thumb=12:thumb_width=24/}} |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=32/}}` | {{ui:progress:50:width=180:thumb=12:thumb_width=32/}} |

### Tall Oval (Vertical)

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=20:thumb_width=10/}}` | {{ui:progress:50:width=180:thumb=20:thumb_width=10/}} |
| `{{ui:progress:50:width=180:thumb=24:thumb_width=12/}}` | {{ui:progress:50:width=180:thumb=24:thumb_width=12/}} |

### iOS-Style Wide Pill

```markdown
{{ui:progress:60:width=220:height=8:track=ui.panel:fill=info:thumb=18:thumb_width=30:thumb_color=white/}}
```

{{ui:progress:60:width=220:height=8:track=ui.panel:fill=info:thumb=18:thumb_width=30:thumb_color=white/}}

### Wide Square (Rounded Rectangle)

```markdown
{{ui:progress:45:width=200:thumb=14:thumb_width=24:thumb_shape=square:thumb_color=accent/}}
```

{{ui:progress:45:width=200:thumb=14:thumb_width=24:thumb_shape=square:thumb_color=accent/}}

---

## Complete Examples

### Skill Bars

```markdown
| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:95:width=120:fill=success/}} |
| Python | {{ui:progress:85:width=120:fill=info/}} |
| TypeScript | {{ui:progress:80:width=120:fill=cobalt/}} |
| Go | {{ui:progress:70:width=120:fill=warning/}} |
```

| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:95:width=120:fill=success/}} |
| Python | {{ui:progress:85:width=120:fill=info/}} |
| TypeScript | {{ui:progress:80:width=120:fill=cobalt/}} |
| Go | {{ui:progress:70:width=120:fill=warning/}} |

### Volume Sliders

```markdown
{{ui:progress:30:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:60:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:90:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
```

{{ui:progress:30:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:60:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:90:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}

### Loading States

```markdown
{{ui:progress:10:width=200:height=6:fill=info/}}
{{ui:progress:45:width=200:height=6:fill=info/}}
{{ui:progress:80:width=200:height=6:fill=info/}}
{{ui:progress:100:width=200:height=6:fill=success/}}
```

{{ui:progress:10:width=200:height=6:fill=info/}}
{{ui:progress:45:width=200:height=6:fill=info/}}
{{ui:progress:80:width=200:height=6:fill=info/}}
{{ui:progress:100:width=200:height=6:fill=success/}}

### Neon Style

```markdown
{{ui:progress:75:width=220:height=8:fill=00FF41:track=0D0D0D/}}
{{ui:progress:60:width=220:height=8:fill=FF00FF:track=0D0D0D/}}
{{ui:progress:85:width=220:height=8:fill=00FFFF:track=0D0D0D/}}
```

{{ui:progress:75:width=220:height=8:fill=00FF41:track=0D0D0D/}}
{{ui:progress:60:width=220:height=8:fill=FF00FF:track=0D0D0D/}}
{{ui:progress:85:width=220:height=8:fill=00FFFF:track=0D0D0D/}}

### Music Player Seek Bar

```markdown
{{ui:progress:35:width=280:height=4:track=slate:fill=success:thumb=14:thumb_width=22/}}
```

{{ui:progress:35:width=280:height=4:track=slate:fill=success:thumb=14:thumb_width=22/}}

### System Resource Meters

| Resource | Usage |
|----------|-------|
| CPU | {{ui:progress:73:width=100:height=12:fill=info:label=true/}} |
| Memory | {{ui:progress:45:width=100:height=12:fill=success:label=true/}} |
| Disk | {{ui:progress:88:width=100:height=12:fill=warning:label=true/}} |

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Limited features - shows percentage as text badge:

```bash
mdfx process template.md
```

### SVG Backend

Full control over all parameters including sliders:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Generates local `.svg` files with tracks, fills, labels, borders, and thumb indicators.

---

## Tips & Tricks

### 1. Minimal Track Slider

```markdown
{{ui:progress:50:width=200:height=2:thumb=16:thumb_color=accent/}}
```

{{ui:progress:50:width=200:height=2:thumb=16:thumb_color=accent/}}

### 2. Thick Track with Small Thumb

```markdown
{{ui:progress:65:width=200:height=16:thumb=12:thumb_color=white/}}
```

{{ui:progress:65:width=200:height=16:thumb=12:thumb_color=white/}}

### 3. Color-Coded Status

```markdown
{{ui:progress:25:width=150:fill=error/}}
{{ui:progress:55:width=150:fill=warning/}}
{{ui:progress:85:width=150:fill=success/}}
```

{{ui:progress:25:width=150:fill=error/}}
{{ui:progress:55:width=150:fill=warning/}}
{{ui:progress:85:width=150:fill=success/}}

### 4. Contrast Border Effect

```markdown
{{ui:progress:70:width=200:height=14:fill=accent:border=white:border_width=2/}}
```

{{ui:progress:70:width=200:height=14:fill=accent:border=white:border_width=2/}}

---

## See Also

- [Donut & Gauge Guide](DONUT-GAUGE-GUIDE.md)
- [Swatch Guide](SWATCH-GUIDE.md)
- [Components Reference](../COMPONENTS.md)
- [CLI Guide](CLI-GUIDE.md)
