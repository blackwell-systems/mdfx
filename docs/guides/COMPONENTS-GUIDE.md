# Components Guide

Components are reusable UI elements that render to visual primitives like badges, dividers, and status indicators. They use the `{{ui:component}}` namespace.

## Basic Syntax

**Self-closing:**
```markdown
{{ui:component:arg1:arg2/}}
```

**With content:**
```markdown
{{ui:component:arg}}Content here{{/ui}}
```

**With optional parameters:**
```markdown
{{ui:component:arg:param=value/}}
```

---

## Native Components

### swatch

Renders a colored block. The foundation for visual elements.

**Syntax:**
```markdown
{{ui:swatch:color/}}
{{ui:swatch:color:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color` | string | required | Palette name or hex code |
| `style` | string | flat-square | Badge style |
| `width` | number | 40 | Width in pixels |
| `height` | number | 20 | Height in pixels |
| `opacity` | float | 1.0 | Transparency (0.0-1.0) |
| `border` | string | none | Border color |
| `border_width` | number | 1 | Border thickness |
| `label` | string | none | Text overlay |
| `label_color` | string | white | Label text color |
| `icon` | string | none | Simple Icons logo |
| `icon_color` | string | white | Icon color |

**Syntax:**
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:FF5500/}}
{{ui:swatch:success:style=plastic/}}
{{ui:swatch:info:width=100:height=30:label=Status/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) ![](https://img.shields.io/badge/-%20-FF5500?style=flat-square) ![](https://img.shields.io/badge/-%20-22C55E?style=plastic) ![](https://img.shields.io/badge/-Status-3B82F6?style=flat-square)

See [SWATCH-GUIDE.md](SWATCH-GUIDE.md) for complete documentation.

---

### divider

Creates a gradient color bar for section separation.

**Syntax:**
```markdown
{{ui:divider/}}
{{ui:divider:style=plastic/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `style` | string | flat-square | Badge style |

**Colors:** Uses theme gradient: `ui.bg` → `ui.surface` → `accent` → `ui.panel`

**Rendered:**

![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

---

### tech

Displays a technology logo badge using Simple Icons.

**Syntax:**
```markdown
{{ui:tech:logo-name/}}
{{ui:tech:logo-name:style=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `logo` | string | required | Simple Icons name (rust, python, docker, etc.) |
| `style` | string | flat-square | Badge style |

**Syntax:**
```markdown
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:typescript/}}
{{ui:tech:docker:style=for-the-badge/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=rust&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=python&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=for-the-badge&logo=docker&logoColor=FFFFFF&label=&labelColor=292A2D)

**Common logos:** rust, python, typescript, javascript, go, docker, kubernetes, react, vue, svelte, nodejs, postgresql, redis, github, gitlab

**Note:** Background uses `ui.bg` color for consistent dark theme appearance.

---

### status

Renders a colored status indicator block.

**Syntax:**
```markdown
{{ui:status:level/}}
{{ui:status:level:style=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `level` | string | required | Status level (success, warning, error, info) |
| `style` | string | flat-square | Badge style |

**Syntax:**
```markdown
{{ui:status:success/}} All tests passing
{{ui:status:warning/}} Deprecated feature
{{ui:status:error/}} Build failed
{{ui:status:info/}} New version available
```

**Rendered:**

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) All tests passing
![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) Deprecated feature
![](https://img.shields.io/badge/-%20-EF4444?style=flat-square) Build failed
![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) New version available

---

### row

Wraps content in an HTML container with horizontal alignment. Converts markdown images to HTML for GitHub compatibility.

**Syntax:**
```markdown
{{ui:row:align=value}}
content
{{/ui}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `align` | enum | center | Horizontal alignment (left, center, right) |

**Syntax:**
```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-%20-F41C80?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-22C55E?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-EAB308?style=flat-square">
</p>

---

## Expand Components

Components that expand to templates with other mdfx syntax.

### header

Section header with gradient frame and bold mathematical text.

**Syntax:**
```markdown
{{ui:header}}Title Text{{/ui}}
```

**Expands to:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}Title Text{{/mathbold}}{{/frame}}
```

**Rendered:**

▓▒░ 𝐆·𝐄·𝐓·𝐓·𝐈·𝐍·𝐆· ·𝐒·𝐓·𝐀·𝐑·𝐓·𝐄·𝐃 ░▒▓

---

### section

Creates a markdown heading with gradient divider underneath.

**Syntax:**
```markdown
{{ui:section:Title Text/}}
```

**Expands to:**
```markdown
## Title Text
{{ui:divider/}}
```

**Rendered:**

## Installation
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

---

### callout

Callout box with status color indicator.

**Syntax:**
```markdown
{{ui:callout:level}}Content{{/ui}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | string | Status color (success, warning, error, info) |

**Rendered:**

█▌![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) This action cannot be undone.

█▌![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) Check the documentation for more details.

█▌![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) Operation completed successfully!

---

### callout-github

GitHub-style blockquote callout with status emoji.

**Syntax:**
```markdown
{{ui:callout-github:type}}Content{{/ui}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `type` | string | Status type (success, warning, error, info) |

**Rendered:**

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
> Check the documentation for more details.

---

### statusitem

Inline status indicator with label and description.

**Syntax:**
```markdown
{{ui:statusitem:Label:level:Description text/}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | string | Display label |
| `level` | string | Status color |
| `text` | string | Description |

**Syntax:**
```markdown
{{ui:statusitem:Build:success:Completed in 2.3s/}}
{{ui:statusitem:Tests:warning:3 skipped/}}
{{ui:statusitem:Deploy:error:Connection failed/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Build**: Completed in 2.3s
![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) **Tests**: 3 skipped
![](https://img.shields.io/badge/-%20-EF4444?style=flat-square) **Deploy**: Connection failed

---

## Badge Styles

All components that render badges support these styles:

| Style | Example |
|-------|---------|
| `flat-square` | ![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) |
| `flat` | ![](https://img.shields.io/badge/-%20-F41C80?style=flat) |
| `plastic` | ![](https://img.shields.io/badge/-%20-F41C80?style=plastic) |
| `for-the-badge` | ![](https://img.shields.io/badge/-%20-F41C80?style=for-the-badge) |
| `social` | ![](https://img.shields.io/badge/-%20-F41C80?style=social) |

---

## Practical Examples

### Tech Stack Display

**Syntax:**
```markdown
{{ui:row:align=center}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=rust&logoColor=FFFFFF&label=&labelColor=292A2D"> <img alt="" src="https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=292A2D"> <img alt="" src="https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=docker&logoColor=FFFFFF&label=&labelColor=292A2D">
</p>

### Status Dashboard

**Syntax:**
```markdown
| Service | Status |
|---------|--------|
| API | {{ui:status:success/}} |
| Database | {{ui:status:success/}} |
| Cache | {{ui:status:warning/}} |
```

**Rendered:**

| Service | Status |
|---------|--------|
| API | ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) |
| Database | ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) |
| Cache | ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) |

### Section with Divider

**Rendered:**

## Features
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

- Fast compilation
- Type safety
- Zero-cost abstractions

### Color Palette Row

**Syntax:**
```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}} {{ui:swatch:error/}} {{ui:swatch:info/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-%20-F41C80?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-22C55E?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-EAB308?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-EF4444?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-%20-3B82F6?style=flat-square">
</p>

### Build Status Line

**Syntax:**
```markdown
{{ui:statusitem:Build:success:v1.2.3/}} {{ui:statusitem:Coverage:info:94%/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Build**: v1.2.3 ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Coverage**: 94%

---

## Top-Level Templates

These templates don't use the `ui:` prefix.

### kbd

Renders keyboard keys with native HTML `<kbd>` tags. GitHub renders these with special keyboard styling.

**Syntax:**
```markdown
{{kbd:keys/}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `keys` | string | Key or key combination (use `+` for combos) |

**Syntax:**
```markdown
Press {{kbd:Enter/}} to continue
Copy with {{kbd:Ctrl+C/}} or {{kbd:⌘+C/}}
Open palette: {{kbd:Ctrl+Shift+P/}}
```

**Rendered:**

Press <kbd>Enter</kbd> to continue
Copy with <kbd>Ctrl</kbd>+<kbd>C</kbd> or <kbd>⌘</kbd>+<kbd>C</kbd>
Open palette: <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd>

**Notes:**
- Splits on `+` and wraps each key in `<kbd>` tags
- Works with Unicode symbols (⌘, ⌥, ⇧, etc.)
- Preserved in code blocks (not processed)

---

## Component Reference

| Component | Type | Self-Closing | Context |
|-----------|------|--------------|---------|
| `swatch` | native | yes | inline, block |
| `divider` | native | yes | block |
| `tech` | native | yes | inline, block |
| `status` | native | yes | inline, block |
| `row` | native | no | block |
| `header` | expand | no | block |
| `section` | expand | yes | block |
| `callout` | expand | no | block |
| `callout-github` | expand | no | block |
| `statusitem` | expand | yes | inline, block |
| `kbd` | top-level | yes | inline |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Status semantics** - success=green, warning=yellow, error=red, info=blue
4. **Section organization** - Use `{{ui:section:Title/}}` for consistent heading styles
5. **Inline vs block** - Most components work in both contexts; row is block-only

---

<p align="center">
ʀᴇɴᴅᴇʀᴇᴅ ᴡɪᴛʜ ᴍᴅꜰx
</p>
