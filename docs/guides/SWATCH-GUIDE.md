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
| `{{ui:swatch:accent/}}` | ![](docs/guides/assets/swatch-guide/swatch_8010e28a060480ec.svg) |
| `{{ui:swatch:success/}}` | ![](docs/guides/assets/swatch-guide/swatch_9548868f54f0a66e.svg) |
| `{{ui:swatch:warning/}}` | ![](docs/guides/assets/swatch-guide/swatch_e4795ff410c7b4fe.svg) |
| `{{ui:swatch:error/}}` | ![](docs/guides/assets/swatch-guide/swatch_e666c671e27adcb2.svg) |
| `{{ui:swatch:FF6B35/}}` | ![](docs/guides/assets/swatch-guide/swatch_3d20307c5841ba1a.svg) |
| `{{ui:swatch:1a1a2e/}}` | ![](docs/guides/assets/swatch-guide/swatch_a38ae1a73b6d0db8.svg) |

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
| `{{ui:swatch:accent:width=8:height=8/}}` | ![](docs/guides/assets/swatch-guide/swatch_f2850eb05da6202f.svg) |
| `{{ui:swatch:accent:width=20:height=20/}}` | ![](docs/guides/assets/swatch-guide/swatch_d72d096a850f1aca.svg) |
| `{{ui:swatch:accent:width=40:height=20/}}` | ![](docs/guides/assets/swatch-guide/swatch_b9d5e624537c531e.svg) |
| `{{ui:swatch:accent:width=100:height=20/}}` | ![](docs/guides/assets/swatch-guide/swatch_b0dbba0d4f1f94da.svg) |
| `{{ui:swatch:accent:width=200:height=10/}}` | ![](docs/guides/assets/swatch-guide/swatch_4ba3d9f7aaf6a356.svg) |

### Wide Bars

```markdown
{{ui:swatch:success:width=200:height=10/}}
{{ui:swatch:info:width=200:height=10/}}
```

![](docs/guides/assets/swatch-guide/swatch_c0dfa498b4254034.svg)
![](docs/guides/assets/swatch-guide/swatch_843d5d5252eef802.svg)

### Tall Columns

```markdown
{{ui:swatch:warning:width=20:height=60/}}
{{ui:swatch:error:width=20:height=60/}}
```

![](docs/guides/assets/swatch-guide/swatch_f07558fe91949d69.svg) ![](docs/guides/assets/swatch-guide/swatch_8aa8fc1a7b89499e.svg)

### Pixel Art

Create pixel art with small swatches:

```markdown
{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}
```

![](docs/guides/assets/swatch-guide/swatch_e23fee9c050c7a1d.svg)![](docs/guides/assets/swatch-guide/swatch_6d533bd20b4347c3.svg)![](docs/guides/assets/swatch-guide/swatch_6e5285ba68468042.svg)

---

## Styles

The `style` parameter changes corner radius and effects:

| Style | Description | Example |
|-------|-------------|---------|
| `flat-square` | Sharp corners (default) | ![](docs/guides/assets/swatch-guide/swatch_187fadcf52235b27.svg) |
| `flat` | Slightly rounded | ![](docs/guides/assets/swatch-guide/swatch_bcea76ace6be4ef4.svg) |
| `plastic` | Glossy 3D effect | ![](docs/guides/assets/swatch-guide/swatch_e777e4bb4af61a9d.svg) |
| `for-the-badge` | Taller badge style | ![](docs/guides/assets/swatch-guide/swatch_c1cd5f9b1b48cdd3.svg) |
| `social` | Pill/capsule shape | ![](docs/guides/assets/swatch-guide/swatch_3fb4d725530e28cb.svg) |

---

## Opacity

Control transparency with `opacity` (0.0 to 1.0):

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent:width=50:height=25:opacity=1.0/}}` | ![](docs/guides/assets/swatch-guide/swatch_2069ae55fccfac21.svg) |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.75/}}` | ![](docs/guides/assets/swatch-guide/swatch_300578deb26bfc0b.svg) |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.5/}}` | ![](docs/guides/assets/swatch-guide/swatch_e2ab9ea78450e514.svg) |
| `{{ui:swatch:accent:width=50:height=25:opacity=0.25/}}` | ![](docs/guides/assets/swatch-guide/swatch_b66fee6a77ee6730.svg) |

### Depth Illusion

Create layered depth effects:

```markdown
{{ui:swatch:F41C80:width=200:height=30:opacity=1.0/}}
{{ui:swatch:F41C80:width=180:height=30:opacity=0.75/}}
{{ui:swatch:F41C80:width=160:height=30:opacity=0.50/}}
{{ui:swatch:F41C80:width=140:height=30:opacity=0.25/}}
```

![](docs/guides/assets/swatch-guide/swatch_334ab4b1edae93a4.svg)
![](docs/guides/assets/swatch-guide/swatch_473ae5dc815994f5.svg)
![](docs/guides/assets/swatch-guide/swatch_445e6b19941cda20.svg)
![](docs/guides/assets/swatch-guide/swatch_6b2d0fd72bbd7c2a.svg)

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
| `{{ui:swatch:1a1a2e:width=60:height=30:border=F41C80/}}` | ![](docs/guides/assets/swatch-guide/swatch_e70394d38af4d9c6.svg) |
| `{{ui:swatch:1a1a2e:width=60:height=30:border=22C55E/}}` | ![](docs/guides/assets/swatch-guide/swatch_6fc31497c91e1da5.svg) |
| `{{ui:swatch:1a1a2e:width=60:height=30:border=3B82F6/}}` | ![](docs/guides/assets/swatch-guide/swatch_105bbb2534d6a7f6.svg) |

### Border Widths

| Syntax | Result |
|--------|--------|
| `...:border=FFFFFF:border_width=1/}}` | ![](docs/guides/assets/swatch-guide/swatch_197fa9af9eec2c3e.svg) |
| `...:border=FFFFFF:border_width=2/}}` | ![](docs/guides/assets/swatch-guide/swatch_8065cddbe122d51b.svg) |
| `...:border=FFFFFF:border_width=4/}}` | ![](docs/guides/assets/swatch-guide/swatch_bfb12525111cdf6f.svg) |

### Glassmorphism Effect

```markdown
{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}
```

![](docs/guides/assets/swatch-guide/swatch_a16abeb735756af6.svg)

---

## Labels

Add text labels with `label` and optional `label_color`:

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:accent:width=80:height=30:label=ACTIVE/}}` | ![](docs/guides/assets/swatch-guide/swatch_e6b13611298cdb70.svg) |
| `{{ui:swatch:success:width=80:height=30:label=ONLINE/}}` | ![](docs/guides/assets/swatch-guide/swatch_6adb7527dece1638.svg) |
| `{{ui:swatch:error:width=80:height=30:label=OFFLINE/}}` | ![](docs/guides/assets/swatch-guide/swatch_7bd8f708d3cac5ed.svg) |
| `{{ui:swatch:warning:width=80:height=30:label=PENDING/}}` | ![](docs/guides/assets/swatch-guide/swatch_2c6f418e8b247bef.svg) |

### Status Labels

```markdown
{{ui:swatch:22C55E:width=120:height=35:label=BUILD PASSING/}}
{{ui:swatch:EF4444:width=120:height=35:label=BUILD FAILED/}}
```

![](docs/guides/assets/swatch-guide/swatch_1784cf31e43b0eae.svg) ![](docs/guides/assets/swatch-guide/swatch_24faeb097a53bd01.svg)

### Custom Label Colors

```markdown
{{ui:swatch:1a1a2e:width=150:height=40:label=DARK MODE:label_color=FFFFFF/}}
```

![](docs/guides/assets/swatch-guide/swatch_ac7eb2d4287124a5.svg)

---

## Icons

Add icon abbreviations with `icon` and optional `icon_color`:

| Syntax | Result |
|--------|--------|
| `{{ui:swatch:000000:width=60:height=40:icon=rust:icon_color=DEA584/}}` | ![](docs/guides/assets/swatch-guide/swatch_25d63c00d887063e.svg) |
| `{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF/}}` | ![](docs/guides/assets/swatch-guide/swatch_f7631bc52605e401.svg) |
| `{{ui:swatch:3776AB:width=60:height=40:icon=python:icon_color=FFD43B/}}` | ![](docs/guides/assets/swatch-guide/swatch_8de39ee9eb0d8872.svg) |
| `{{ui:swatch:2496ED:width=60:height=40:icon=docker:icon_color=FFFFFF/}}` | ![](docs/guides/assets/swatch-guide/swatch_5451a80f4ae790ba.svg) |

**Note:** Icons render as 3-letter abbreviations. Full SVG icons require bundling icon libraries.

---

## Color Palette Reference

Built-in palette colors:

| Name | Hex | Swatch |
|------|-----|--------|
| `accent` | F41C80 | ![](docs/guides/assets/swatch-guide/swatch_b9d5e624537c531e.svg) |
| `success` | 22C55E | ![](docs/guides/assets/swatch-guide/swatch_c0397877dee4cc0e.svg) |
| `warning` | EAB308 | ![](docs/guides/assets/swatch-guide/swatch_20c9e19d412a74be.svg) |
| `error` | EF4444 | ![](docs/guides/assets/swatch-guide/swatch_67b59d12305ed09.svg) |
| `info` | 3B82F6 | ![](docs/guides/assets/swatch-guide/swatch_224b8da9039b2c7b.svg) |
| `slate` | 6B7280 | ![](docs/guides/assets/swatch-guide/swatch_dbbccfab56ff1adf.svg) |

---

## Complete Examples

### Status Dashboard

```markdown
{{ui:swatch:22C55E:width=12:height=12:style=social/}} API Server: Online
{{ui:swatch:22C55E:width=12:height=12:style=social/}} Database: Healthy
{{ui:swatch:EAB308:width=12:height=12:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:width=12:height=12:style=social/}} Queue: Critical
```

![](docs/guides/assets/swatch-guide/swatch_b51ae78bbddddf56.svg) API Server: Online
![](docs/guides/assets/swatch-guide/swatch_b51ae78bbddddf56.svg) Database: Healthy
![](docs/guides/assets/swatch-guide/swatch_5935bad76cab4e14.svg) Cache: Degraded
![](docs/guides/assets/swatch-guide/swatch_aa967ee39be7fdb1.svg) Queue: Critical

### Progress Bar

```markdown
{{ui:swatch:22C55E:width=150:height=20/}}{{ui:swatch:333333:width=50:height=20/}}
```

![](docs/guides/assets/swatch-guide/swatch_58170cb2d9364f72.svg)![](docs/guides/assets/swatch-guide/swatch_19c0b18c4e263066.svg)

### Color Palette Display

```markdown
{{ui:swatch:1a1a2e:width=50:height=60/}}{{ui:swatch:2d2d44:width=50:height=60/}}{{ui:swatch:4a4a6a:width=50:height=60/}}{{ui:swatch:6b6b8d:width=50:height=60/}}
```

![](docs/guides/assets/swatch-guide/swatch_3d53c3d32e63b5a9.svg)![](docs/guides/assets/swatch-guide/swatch_e93088831b38961e.svg)![](docs/guides/assets/swatch-guide/swatch_94e3acb31f72174.svg)![](docs/guides/assets/swatch-guide/swatch_f4cdbfc3b2f04480.svg)

### Heat Map Row

```markdown
{{ui:swatch:0a0a0a:width=25:height=25/}}{{ui:swatch:3a1010:width=25:height=25/}}{{ui:swatch:8B2500:width=25:height=25/}}{{ui:swatch:CD3700:width=25:height=25/}}{{ui:swatch:FF4500:width=25:height=25/}}{{ui:swatch:FF6347:width=25:height=25/}}{{ui:swatch:FFD700:width=25:height=25/}}
```

![](docs/guides/assets/swatch-guide/swatch_4d6f36f8da90bd84.svg)![](docs/guides/assets/swatch-guide/swatch_f1e0ff7f09474ab9.svg)![](docs/guides/assets/swatch-guide/swatch_a1319f3bf493fd9d.svg)![](docs/guides/assets/swatch-guide/swatch_67bd1fa73f0e4cc9.svg)![](docs/guides/assets/swatch-guide/swatch_7221875061703aa9.svg)![](docs/guides/assets/swatch-guide/swatch_f09981457c1867.svg)![](docs/guides/assets/swatch-guide/swatch_418a9aaabbd2315b.svg)

### Tech Stack

```markdown
{{ui:swatch:DEA584:width=50:height=35:icon=rust:icon_color=000000/}}
{{ui:swatch:F7DF1E:width=50:height=35:icon=js:icon_color=000000/}}
{{ui:swatch:3178C6:width=50:height=35:icon=ts:icon_color=FFFFFF/}}
```

![](docs/guides/assets/swatch-guide/swatch_b68f50980587a665.svg) ![](docs/guides/assets/swatch-guide/swatch_22c890b2a6ddfb97.svg) ![](docs/guides/assets/swatch-guide/swatch_2533c2fbe877fba1.svg)

### Hazard Warnings

```markdown
{{ui:swatch:FFFF00:width=80:height=40:border=000000:border_width=2:label=CAUTION/}}
{{ui:swatch:FF6600:width=80:height=40:border=000000:border_width=2:label=WARNING/}}
{{ui:swatch:FF0000:width=80:height=40:border=000000:border_width=2:label=DANGER/}}
```

![](docs/guides/assets/swatch-guide/swatch_a547b13cf8d1bee.svg) ![](docs/guides/assets/swatch-guide/swatch_fb7576333d736ef5.svg) ![](docs/guides/assets/swatch-guide/swatch_c548076f36f6ac3c.svg)

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

![](docs/guides/assets/swatch-guide/swatch_2f20d2053e228edf.svg)

### 3. Combine Multiple Parameters

```markdown
{{ui:swatch:1a1a2e:width=250:height=60:style=social:opacity=0.9:border=00FF00:border_width=2:label=COMPLETE/}}
```

![](docs/guides/assets/swatch-guide/swatch_8b24e350f6530ecd.svg)

---

## See Also

- [Components Reference](../COMPONENTS.md)
- [Frames Guide](FRAMES-GUIDE.md)
- [CLI Guide](CLI-GUIDE.md)
