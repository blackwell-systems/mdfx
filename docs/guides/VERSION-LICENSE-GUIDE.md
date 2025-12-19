# Version & License Badge Guide

Semantic badges for version numbers and software licenses with automatic color-coding.

## Table of Contents

- [Version Badges](#version-badges)
  - [Basic Syntax](#basic-syntax)
  - [Auto-Detection](#auto-detection)
  - [Status Override](#status-override)
  - [Version Parameters](#version-parameters)
  - [Custom Styling](#custom-styling)
- [License Badges](#license-badges)
  - [License Syntax](#license-syntax)
  - [License Categories](#license-categories)
  - [License Parameters](#license-parameters)
  - [Common Licenses](#common-licenses)
- [Combining with Tech Badges](#combining-with-tech-badges)
- [Tips & Tricks](#tips--tricks)

---

## Version Badges

### Basic Syntax

```markdown
{{ui:version:VERSION/}}
```

The version component automatically detects stability from the version string and applies semantic coloring.

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_aabe83ba70a3b3d5.svg) |
| `{{ui:version:2.5.3/}}` | ![](assets/version-license-guide/swatch_a0a50fd203a24a7e.svg) |
| `{{ui:version:10.0.0/}}` | ![](assets/version-license-guide/swatch_213bfcf59dc38777.svg) |

---

### Auto-Detection

Version strings are automatically parsed to determine status:

#### Stable Versions (Green)

Released production versions (1.x.x and higher):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_aabe83ba70a3b3d5.svg) |
| `{{ui:version:3.2.1/}}` | ![](assets/version-license-guide/swatch_8d966de58e20061f.svg) |
| `{{ui:version:12.0.0/}}` | ![](assets/version-license-guide/swatch_4514e2a1f2d21dc2.svg) |

#### Beta Versions (Yellow)

Pre-release testing versions (0.x.x or -beta/-rc suffix):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:0.9.0/}}` | ![](assets/version-license-guide/swatch_ab4c5ed84bef5518.svg) |
| `{{ui:version:2.0.0-beta/}}` | ![](assets/version-license-guide/swatch_b45e3454d750c219.svg) |
| `{{ui:version:1.5.0-beta.2/}}` | ![](assets/version-license-guide/swatch_24739011b62e236f.svg) |
| `{{ui:version:3.0.0-rc.1/}}` | ![](assets/version-license-guide/swatch_f38aa364e18633ae.svg) |
| `{{ui:version:2.0.0-preview/}}` | ![](assets/version-license-guide/swatch_39f77d895f5b9baf.svg) |

#### Alpha Versions (Orange)

Early development versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-alpha/}}` | ![](assets/version-license-guide/swatch_a0b481854d3215cb.svg) |
| `{{ui:version:2.0.0-alpha.3/}}` | ![](assets/version-license-guide/swatch_48e1d097bc5fef06.svg) |

#### Development Versions (Purple)

Unstable development builds:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-dev/}}` | ![](assets/version-license-guide/swatch_c2ab2fbb75f229c6.svg) |
| `{{ui:version:2.0.0-snapshot/}}` | ![](assets/version-license-guide/swatch_16c330642a77a05d.svg) |
| `{{ui:version:3.0.0-nightly/}}` | ![](assets/version-license-guide/swatch_00391c9381120ac2.svg) |

#### Deprecated Versions (Red)

End-of-life or unsupported versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-deprecated/}}` | ![](assets/version-license-guide/swatch_50224d739194a573.svg) |
| `{{ui:version:0.5.0-eol/}}` | ![](assets/version-license-guide/swatch_93609e163b299f76.svg) |

---

### Status Override

Override auto-detection with the `status` parameter:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:status=stable/}}` | ![](assets/version-license-guide/swatch_aabe83ba70a3b3d5.svg) |
| `{{ui:version:1.0.0:status=beta/}}` | ![](assets/version-license-guide/swatch_6b8db58e50b077b7.svg) |
| `{{ui:version:1.0.0:status=alpha/}}` | ![](assets/version-license-guide/swatch_1e7451b775cc2c12.svg) |
| `{{ui:version:1.0.0:status=dev/}}` | ![](assets/version-license-guide/swatch_495ecefcaa5ea923.svg) |
| `{{ui:version:1.0.0:status=deprecated/}}` | ![](assets/version-license-guide/swatch_1f25d8a75e3547c5.svg) |

---

### Version Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `version` | string | *required* | Version string (first positional argument) |
| `status` | enum | auto | Override: stable, beta, alpha, dev, deprecated |
| `bg` | color | auto | Custom background color |
| `text` | color | auto | Custom text color |
| `prefix` | string | "v" | Version prefix (use "" to disable) |
| `style` | enum | flat-square | Badge style |

---

### Custom Styling

#### Without Prefix

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:prefix=/}}` | ![](assets/version-license-guide/swatch_49ddda9a0211818d.svg) |
| `{{ui:version:2.5.0:prefix=/}}` | ![](assets/version-license-guide/swatch_fe5a232c1e03c3ea.svg) |

#### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_897e21ef13830271.svg) |
| `{{ui:version:2.0.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_e9ee9f827dc58e55.svg) |
| `{{ui:version:3.0.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_fe95d1519b1c7e68.svg) |

#### Badge Styles

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:style=flat/}}` | ![](assets/version-license-guide/swatch_68c24257a3406bb8.svg) |
| `{{ui:version:1.0.0:style=plastic/}}` | ![](assets/version-license-guide/swatch_fcdf995283615306.svg) |
| `{{ui:version:1.0.0:style=for-the-badge/}}` | ![](assets/version-license-guide/swatch_bdc37939443ed823.svg) |

---

## License Badges

### License Syntax

```markdown
{{ui:license:LICENSE/}}
```

The license component categorizes licenses and applies semantic coloring.

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_85a9fc02551e469b.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_0e50155d41187e8b.svg) |
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_380036f2d9c5a7e2.svg) |

---

### License Categories

#### Permissive Licenses (Green)

Open-source friendly, minimal restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_85a9fc02551e469b.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_0e50155d41187e8b.svg) |
| `{{ui:license:BSD-3-Clause/}}` | ![](assets/version-license-guide/swatch_8e1f3054d30c236f.svg) |
| `{{ui:license:BSD-2-Clause/}}` | ![](assets/version-license-guide/swatch_e77a49b75b71e6ef.svg) |
| `{{ui:license:ISC/}}` | ![](assets/version-license-guide/swatch_248b982c7f3a20b9.svg) |

#### Weak Copyleft (Blue)

File-level copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:LGPL-3.0/}}` | ![](assets/version-license-guide/swatch_68b37f22bceae35c.svg) |
| `{{ui:license:LGPL-2.1/}}` | ![](assets/version-license-guide/swatch_680acd57cc7d0352.svg) |
| `{{ui:license:MPL-2.0/}}` | ![](assets/version-license-guide/swatch_69e265cf569ea74e.svg) |
| `{{ui:license:EPL-2.0/}}` | ![](assets/version-license-guide/swatch_da9f5a73f4d97c9c.svg) |

#### Copyleft (Yellow)

Strong copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_380036f2d9c5a7e2.svg) |
| `{{ui:license:GPL-2.0/}}` | ![](assets/version-license-guide/swatch_9237b4fa33d80bfe.svg) |
| `{{ui:license:AGPL-3.0/}}` | ![](assets/version-license-guide/swatch_d002b9799507979f.svg) |

#### Public Domain (Cyan)

No restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:CC0/}}` | ![](assets/version-license-guide/swatch_83ce122ae83e6484.svg) |
| `{{ui:license:Unlicense/}}` | ![](assets/version-license-guide/swatch_589c7d76645e9243.svg) |

#### Proprietary (Gray)

Closed source:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:Proprietary/}}` | ![](assets/version-license-guide/swatch_30680d4d30855460.svg) |
| `{{ui:license:Commercial/}}` | ![](assets/version-license-guide/swatch_9255e7dbab0a43ea.svg) |

---

### License Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `license` | string | *required* | License identifier (first positional argument) |
| `label` | string | auto | Custom label (default: formatted license name) |
| `bg` | color | auto | Custom background color |
| `text` | color | auto | Custom text color |
| `style` | enum | flat-square | Badge style |

---

### Common Licenses

Quick reference for popular licenses:

| License | Category | Description |
|---------|----------|-------------|
| MIT | Permissive | Simple, do anything with attribution |
| Apache-2.0 | Permissive | Patent protection, attribution required |
| BSD-3-Clause | Permissive | Attribution, no endorsement |
| GPL-3.0 | Copyleft | Derivatives must be GPL |
| LGPL-3.0 | Weak Copyleft | Library linking exception |
| MPL-2.0 | Weak Copyleft | File-level copyleft |
| AGPL-3.0 | Copyleft | Network use triggers copyleft |
| CC0 | Public Domain | No restrictions whatsoever |

---

### Custom Labels

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT:label=MIT License/}}` | ![](assets/version-license-guide/swatch_b5e0e7b3039e8f56.svg) |
| `{{ui:license:Apache-2.0:label=Apache/}}` | ![](assets/version-license-guide/swatch_c1330940125b4509.svg) |
| `{{ui:license:GPL-3.0:label=GPLv3/}}` | ![](assets/version-license-guide/swatch_a7e18a850ad12f34.svg) |

### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_faa26cdc53a9bcfd.svg) |
| `{{ui:license:Apache-2.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_4e379cb21a7c794b.svg) |
| `{{ui:license:GPL-3.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_ddf6ead7897691f8.svg) |

---

## Combining with Tech Badges

Create comprehensive project headers with tech, version, and license badges:

### Project Header Example

```markdown
{{ui:tech:rust/}} {{ui:version:1.0.0/}} {{ui:license:MIT/}}
```

![](assets/version-license-guide/tech_9b07f32e2323dccd.svg) ![](assets/version-license-guide/swatch_aabe83ba70a3b3d5.svg) ![](assets/version-license-guide/swatch_85a9fc02551e469b.svg)

### Full Stack Example

```markdown
{{ui:tech:typescript/}} {{ui:tech:react/}} {{ui:version:2.5.0-beta/}} {{ui:license:Apache-2.0/}}
```

![](assets/version-license-guide/tech_b30721c0a0394c2e.svg) ![](assets/version-license-guide/tech_fa93a6b13b34f67b.svg) ![](assets/version-license-guide/swatch_b981b8418b8fb448.svg) ![](assets/version-license-guide/swatch_0e50155d41187e8b.svg)

### Deprecated Project

```markdown
{{ui:tech:python/}} {{ui:version:0.5.0:status=deprecated/}} {{ui:license:GPL-3.0/}}
```

![](assets/version-license-guide/tech_c5b0cf28158ee95f.svg) ![](assets/version-license-guide/swatch_5a807cf2c2ac9113.svg) ![](assets/version-license-guide/swatch_380036f2d9c5a7e2.svg)

---

## Tips & Tricks

### 1. Use Auto-Detection for Clean Source

Let mdfx detect the version status automatically:

```markdown
<!-- Clean source, smart colors -->
{{ui:version:1.0.0/}}       <!-- Green - stable -->
{{ui:version:0.9.0/}}       <!-- Yellow - 0.x is beta -->
{{ui:version:2.0.0-rc.1/}}  <!-- Yellow - prerelease -->
```

### 2. Version + URL for Releases

Combine with the `url` parameter for clickable release links:

```markdown
<!-- Links to GitHub release -->
{{ui:tech:rust:url=https://github.com/org/repo/}}
```

### 3. Consistent Badge Styles

Use the same style across all badges for visual consistency:

| Consistent Flat | Consistent For-The-Badge |
|-----------------|-------------------------|
| ![](assets/version-license-guide/tech_837edd35922729ee.svg) ![](assets/version-license-guide/swatch_68c24257a3406bb8.svg) ![](assets/version-license-guide/swatch_f10311d16e3ed593.svg) | ![](assets/version-license-guide/tech_cd9cdaecb20390d2.svg) ![](assets/version-license-guide/swatch_bdc37939443ed823.svg) ![](assets/version-license-guide/swatch_39b5ca5536add59f.svg) |

### 4. Custom Colors for Branding

Override default colors to match your project theme:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_adc184be7ad79224.svg) |
| `{{ui:license:MIT:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_56418742431319cf.svg) |

---

## See Also

- [Tech Badges](TECH-GUIDE.md) - Technology logo badges
- [Swatches](SWATCH-GUIDE.md) - Color block component
- [Components](COMPONENTS-GUIDE.md) - All UI components
- [Colors](COLORS-GUIDE.md) - Palette reference
