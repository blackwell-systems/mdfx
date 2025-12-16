# Donut & Gauge Complete Guide

Donut and gauge components are circular visualizations perfect for dashboards, completion indicators, and interactive controls. This guide covers every parameter and configuration option for both.

## Table of Contents

- [Donut Component](#donut-component)
  - [Basic Donut Syntax](#basic-donut-syntax)
  - [Donut Parameters](#donut-parameters)
  - [Donut Sizes](#donut-sizes)
  - [Donut Thickness](#donut-thickness)
  - [Donut Colors](#donut-colors)
  - [Donut Labels](#donut-labels)
  - [Donut Slider Mode](#donut-slider-mode)
- [Gauge Component](#gauge-component)
  - [Basic Gauge Syntax](#basic-gauge-syntax)
  - [Gauge Parameters](#gauge-parameters)
  - [Gauge Sizes](#gauge-sizes)
  - [Gauge Thickness](#gauge-thickness)
  - [Gauge Colors](#gauge-colors)
  - [Gauge Labels](#gauge-labels)
  - [Gauge Slider Mode](#gauge-slider-mode)
- [Thumb Customization](#thumb-customization)
- [Complete Examples](#complete-examples)
- [Backend Differences](#backend-differences)

---

# Donut Component

A circular progress ring showing completion percentage.

## Basic Donut Syntax

```markdown
{{ui:donut:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:donut:0/}}` | {{ui:donut:0/}} |
| `{{ui:donut:25/}}` | {{ui:donut:25/}} |
| `{{ui:donut:50/}}` | {{ui:donut:50/}} |
| `{{ui:donut:75/}}` | {{ui:donut:75/}} |
| `{{ui:donut:100/}}` | {{ui:donut:100/}} |

---

## Donut Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
| `size` | number | 40 | Diameter in pixels |
| `thickness` | number | 4 | Ring thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label in center |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size (enables slider mode) |
| `thumb_color` | color | fill | Thumb color |

---

## Donut Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:donut:65:size=24/}}` | {{ui:donut:65:size=24/}} |
| `{{ui:donut:65:size=40/}}` | {{ui:donut:65:size=40/}} |
| `{{ui:donut:65:size=60/}}` | {{ui:donut:65:size=60/}} |
| `{{ui:donut:65:size=80/}}` | {{ui:donut:65:size=80/}} |
| `{{ui:donut:65:size=100/}}` | {{ui:donut:65:size=100/}} |

---

## Donut Thickness

| Syntax | Result |
|--------|--------|
| `{{ui:donut:70:size=60:thickness=2/}}` | {{ui:donut:70:size=60:thickness=2/}} |
| `{{ui:donut:70:size=60:thickness=4/}}` | {{ui:donut:70:size=60:thickness=4/}} |
| `{{ui:donut:70:size=60:thickness=8/}}` | {{ui:donut:70:size=60:thickness=8/}} |
| `{{ui:donut:70:size=60:thickness=12/}}` | {{ui:donut:70:size=60:thickness=12/}} |
| `{{ui:donut:70:size=60:thickness=16/}}` | {{ui:donut:70:size=60:thickness=16/}} |

### Thin Elegant Ring

```markdown
{{ui:donut:80:size=80:thickness=2:fill=accent/}}
```

{{ui:donut:80:size=80:thickness=2:fill=accent/}}

### Thick Chunky Ring

```markdown
{{ui:donut:60:size=80:thickness=18:fill=info/}}
```

{{ui:donut:60:size=80:thickness=18:fill=info/}}

---

## Donut Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:70:size=50:fill=accent/}}` | {{ui:donut:70:size=50:fill=accent/}} |
| `{{ui:donut:70:size=50:fill=success/}}` | {{ui:donut:70:size=50:fill=success/}} |
| `{{ui:donut:70:size=50:fill=warning/}}` | {{ui:donut:70:size=50:fill=warning/}} |
| `{{ui:donut:70:size=50:fill=error/}}` | {{ui:donut:70:size=50:fill=error/}} |
| `{{ui:donut:70:size=50:fill=info/}}` | {{ui:donut:70:size=50:fill=info/}} |
| `{{ui:donut:70:size=50:fill=cobalt/}}` | {{ui:donut:70:size=50:fill=cobalt/}} |

### Track Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=50:track=slate/}}` | {{ui:donut:60:size=50:track=slate/}} |
| `{{ui:donut:60:size=50:track=ink/}}` | {{ui:donut:60:size=50:track=ink/}} |
| `{{ui:donut:60:size=50:track=ui.panel/}}` | {{ui:donut:60:size=50:track=ui.panel/}} |

### Custom Hex Colors

```markdown
{{ui:donut:75:size=60:fill=FF6B35:track=1a1a2e/}}
{{ui:donut:60:size=60:fill=00FF41:track=0D0D0D/}}
{{ui:donut:85:size=60:fill=FF00FF:track=1a0a1a/}}
```

{{ui:donut:75:size=60:fill=FF6B35:track=1a1a2e/}} {{ui:donut:60:size=60:fill=00FF41:track=0D0D0D/}} {{ui:donut:85:size=60:fill=FF00FF:track=1a0a1a/}}

---

## Donut Labels

Show percentage in center with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:donut:25:size=60:label=true/}}` | {{ui:donut:25:size=60:label=true/}} |
| `{{ui:donut:50:size=60:label=true/}}` | {{ui:donut:50:size=60:label=true/}} |
| `{{ui:donut:75:size=60:label=true/}}` | {{ui:donut:75:size=60:label=true/}} |
| `{{ui:donut:100:size=60:label=true/}}` | {{ui:donut:100:size=60:label=true/}} |

### Custom Label Colors

```markdown
{{ui:donut:85:size=70:label=true:label_color=accent:fill=success/}}
```

{{ui:donut:85:size=70:label=true:label_color=accent:fill=success/}}

---

## Donut Slider Mode

Add a `thumb` parameter to show an indicator at the fill position:

| Syntax | Result |
|--------|--------|
| `{{ui:donut:30:size=60:thumb=12/}}` | {{ui:donut:30:size=60:thumb=12/}} |
| `{{ui:donut:50:size=60:thumb=12/}}` | {{ui:donut:50:size=60:thumb=12/}} |
| `{{ui:donut:70:size=60:thumb=12/}}` | {{ui:donut:70:size=60:thumb=12/}} |
| `{{ui:donut:90:size=60:thumb=12/}}` | {{ui:donut:90:size=60:thumb=12/}} |

### Thumb Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=60:thumb=8/}}` | {{ui:donut:60:size=60:thumb=8/}} |
| `{{ui:donut:60:size=60:thumb=12/}}` | {{ui:donut:60:size=60:thumb=12/}} |
| `{{ui:donut:60:size=60:thumb=16/}}` | {{ui:donut:60:size=60:thumb=16/}} |
| `{{ui:donut:60:size=60:thumb=20/}}` | {{ui:donut:60:size=60:thumb=20/}} |

---

# Gauge Component

A semi-circular meter for dashboard-style displays.

## Basic Gauge Syntax

```markdown
{{ui:gauge:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:0/}}` | {{ui:gauge:0/}} |
| `{{ui:gauge:25/}}` | {{ui:gauge:25/}} |
| `{{ui:gauge:50/}}` | {{ui:gauge:50/}} |
| `{{ui:gauge:75/}}` | {{ui:gauge:75/}} |
| `{{ui:gauge:100/}}` | {{ui:gauge:100/}} |

---

## Gauge Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
| `size` | number | 80 | Width in pixels |
| `thickness` | number | 8 | Arc thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label below arc |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size (enables slider mode) |
| `thumb_color` | color | fill | Thumb color |

---

## Gauge Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:65:size=50/}}` | {{ui:gauge:65:size=50/}} |
| `{{ui:gauge:65:size=80/}}` | {{ui:gauge:65:size=80/}} |
| `{{ui:gauge:65:size=100/}}` | {{ui:gauge:65:size=100/}} |
| `{{ui:gauge:65:size=120/}}` | {{ui:gauge:65:size=120/}} |

---

## Gauge Thickness

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:70:size=100:thickness=4/}}` | {{ui:gauge:70:size=100:thickness=4/}} |
| `{{ui:gauge:70:size=100:thickness=8/}}` | {{ui:gauge:70:size=100:thickness=8/}} |
| `{{ui:gauge:70:size=100:thickness=12/}}` | {{ui:gauge:70:size=100:thickness=12/}} |
| `{{ui:gauge:70:size=100:thickness=16/}}` | {{ui:gauge:70:size=100:thickness=16/}} |

### Hairline Gauge

```markdown
{{ui:gauge:80:size=120:thickness=3:fill=accent/}}
```

{{ui:gauge:80:size=120:thickness=3:fill=accent/}}

### Chunky Gauge

```markdown
{{ui:gauge:60:size=120:thickness=20:fill=info/}}
```

{{ui:gauge:60:size=120:thickness=20:fill=info/}}

---

## Gauge Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:70:size=80:fill=accent/}}` | {{ui:gauge:70:size=80:fill=accent/}} |
| `{{ui:gauge:70:size=80:fill=success/}}` | {{ui:gauge:70:size=80:fill=success/}} |
| `{{ui:gauge:70:size=80:fill=warning/}}` | {{ui:gauge:70:size=80:fill=warning/}} |
| `{{ui:gauge:70:size=80:fill=error/}}` | {{ui:gauge:70:size=80:fill=error/}} |
| `{{ui:gauge:70:size=80:fill=info/}}` | {{ui:gauge:70:size=80:fill=info/}} |

### Speedometer Style

```markdown
{{ui:gauge:25:size=100:thickness=10:fill=success/}}
{{ui:gauge:55:size=100:thickness=10:fill=warning/}}
{{ui:gauge:85:size=100:thickness=10:fill=error/}}
```

{{ui:gauge:25:size=100:thickness=10:fill=success/}} {{ui:gauge:55:size=100:thickness=10:fill=warning/}} {{ui:gauge:85:size=100:thickness=10:fill=error/}}

### Neon Colors

```markdown
{{ui:gauge:75:size=100:fill=00FF41:track=0D0D0D/}}
{{ui:gauge:60:size=100:fill=FF00FF:track=0D0D0D/}}
{{ui:gauge:85:size=100:fill=00FFFF:track=0D0D0D/}}
```

{{ui:gauge:75:size=100:fill=00FF41:track=0D0D0D/}} {{ui:gauge:60:size=100:fill=FF00FF:track=0D0D0D/}} {{ui:gauge:85:size=100:fill=00FFFF:track=0D0D0D/}}

---

## Gauge Labels

Show percentage with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:25:size=100:label=true/}}` | {{ui:gauge:25:size=100:label=true/}} |
| `{{ui:gauge:50:size=100:label=true/}}` | {{ui:gauge:50:size=100:label=true/}} |
| `{{ui:gauge:75:size=100:label=true/}}` | {{ui:gauge:75:size=100:label=true/}} |
| `{{ui:gauge:100:size=100:label=true/}}` | {{ui:gauge:100:size=100:label=true/}} |

---

## Gauge Slider Mode

Add a `thumb` parameter for interactive-style indicator:

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:30:size=100:thumb=14/}}` | {{ui:gauge:30:size=100:thumb=14/}} |
| `{{ui:gauge:50:size=100:thumb=14/}}` | {{ui:gauge:50:size=100:thumb=14/}} |
| `{{ui:gauge:70:size=100:thumb=14/}}` | {{ui:gauge:70:size=100:thumb=14/}} |
| `{{ui:gauge:90:size=100:thumb=14/}}` | {{ui:gauge:90:size=100:thumb=14/}} |

---

# Thumb Customization

Both donut and gauge support custom thumb colors:

### Donut Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=60:thumb=14:thumb_color=accent/}}` | {{ui:donut:60:size=60:thumb=14:thumb_color=accent/}} |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=success/}}` | {{ui:donut:60:size=60:thumb=14:thumb_color=success/}} |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=warning/}}` | {{ui:donut:60:size=60:thumb=14:thumb_color=warning/}} |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=white/}}` | {{ui:donut:60:size=60:thumb=14:thumb_color=white/}} |

### Gauge Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=accent/}}` | {{ui:gauge:60:size=100:thumb=16:thumb_color=accent/}} |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=success/}}` | {{ui:gauge:60:size=100:thumb=16:thumb_color=success/}} |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=warning/}}` | {{ui:gauge:60:size=100:thumb=16:thumb_color=warning/}} |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=error/}}` | {{ui:gauge:60:size=100:thumb=16:thumb_color=error/}} |

### Contrasting Thumb

```markdown
{{ui:donut:75:size=70:fill=info:thumb=16:thumb_color=accent/}}
{{ui:gauge:65:size=110:fill=success:thumb=18:thumb_color=error/}}
```

{{ui:donut:75:size=70:fill=info:thumb=16:thumb_color=accent/}} {{ui:gauge:65:size=110:fill=success:thumb=18:thumb_color=error/}}

---

# Complete Examples

### Dashboard Metrics

```markdown
| Metric | Status |
|--------|--------|
| CPU | {{ui:gauge:73:size=60:thickness=6:fill=info/}} |
| Memory | {{ui:gauge:45:size=60:thickness=6:fill=success/}} |
| Disk | {{ui:gauge:88:size=60:thickness=6:fill=warning/}} |
| Network | {{ui:gauge:32:size=60:thickness=6:fill=cobalt/}} |
```

| Metric | Status |
|--------|--------|
| CPU | {{ui:gauge:73:size=60:thickness=6:fill=info/}} |
| Memory | {{ui:gauge:45:size=60:thickness=6:fill=success/}} |
| Disk | {{ui:gauge:88:size=60:thickness=6:fill=warning/}} |
| Network | {{ui:gauge:32:size=60:thickness=6:fill=cobalt/}} |

### Task Completion Donuts

```markdown
| Task | Progress |
|------|----------|
| Research | {{ui:donut:100:size=40:fill=success:label=true/}} |
| Design | {{ui:donut:75:size=40:fill=info:label=true/}} |
| Development | {{ui:donut:45:size=40:fill=warning:label=true/}} |
| Testing | {{ui:donut:20:size=40:fill=error:label=true/}} |
```

| Task | Progress |
|------|----------|
| Research | {{ui:donut:100:size=40:fill=success:label=true/}} |
| Design | {{ui:donut:75:size=40:fill=info:label=true/}} |
| Development | {{ui:donut:45:size=40:fill=warning:label=true/}} |
| Testing | {{ui:donut:20:size=40:fill=error:label=true/}} |

### Volume Control Wheel

```markdown
{{ui:donut:30:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
{{ui:donut:60:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
{{ui:donut:90:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
```

{{ui:donut:30:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}} {{ui:donut:60:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}} {{ui:donut:90:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}

### Speedometer Dashboard

```markdown
{{ui:gauge:20:size=120:thickness=12:fill=22C55E:label=true/}}
{{ui:gauge:60:size=120:thickness=12:fill=EAB308:label=true/}}
{{ui:gauge:95:size=120:thickness=12:fill=EF4444:label=true/}}
```

{{ui:gauge:20:size=120:thickness=12:fill=22C55E:label=true/}} {{ui:gauge:60:size=120:thickness=12:fill=EAB308:label=true/}} {{ui:gauge:95:size=120:thickness=12:fill=EF4444:label=true/}}

### Comparison: Donut vs Gauge vs Progress

Same data, different visualizations:

| Task | Donut | Gauge | Bar |
|------|-------|-------|-----|
| API | {{ui:donut:85:size=36:thickness=5:fill=success/}} | {{ui:gauge:85:size=60:thickness=6:fill=success/}} | {{ui:progress:85:width=80:fill=success/}} |
| Tests | {{ui:donut:72:size=36:thickness=5:fill=info/}} | {{ui:gauge:72:size=60:thickness=6:fill=info/}} | {{ui:progress:72:width=80:fill=info/}} |
| Docs | {{ui:donut:45:size=36:thickness=5:fill=warning/}} | {{ui:gauge:45:size=60:thickness=6:fill=warning/}} | {{ui:progress:45:width=80:fill=warning/}} |

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Limited to percentage text:

```bash
mdfx process template.md
```

### SVG Backend

Full circular rendering with all features:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Generates local `.svg` files with arcs, labels, and thumb indicators.

---

## See Also

- [Progress Guide](PROGRESS-GUIDE.md)
- [Swatch Guide](SWATCH-GUIDE.md)
- [Components Reference](../COMPONENTS.md)
- [CLI Guide](CLI-GUIDE.md)
