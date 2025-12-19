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
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) |
| `{{ui:version:2.5.3/}}` | ![](assets/version-license-guide/swatch_917e7814b4eedee0.svg) |
| `{{ui:version:10.0.0/}}` | ![](assets/version-license-guide/swatch_c7184df89fbd256f.svg) |

---

### Auto-Detection

Version strings are automatically parsed to determine status:

#### Stable Versions (Green)

Released production versions (1.x.x and higher):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) |
| `{{ui:version:3.2.1/}}` | ![](assets/version-license-guide/swatch_1bc06e5eb7da0260.svg) |
| `{{ui:version:12.0.0/}}` | ![](assets/version-license-guide/swatch_bfd4f536cc724fa2.svg) |

#### Beta Versions (Yellow)

Pre-release testing versions (0.x.x or -beta/-rc suffix):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:0.9.0/}}` | ![](assets/version-license-guide/swatch_3c6482f977f7ff4c.svg) |
| `{{ui:version:2.0.0-beta/}}` | ![](assets/version-license-guide/swatch_b916a8cb32408a2e.svg) |
| `{{ui:version:1.5.0-beta.2/}}` | ![](assets/version-license-guide/swatch_c8ea17f990f8896f.svg) |
| `{{ui:version:3.0.0-rc.1/}}` | ![](assets/version-license-guide/swatch_3e46f9dd4b6279d9.svg) |
| `{{ui:version:2.0.0-preview/}}` | ![](assets/version-license-guide/swatch_4b74263ca0bd16fb.svg) |

#### Alpha Versions (Orange)

Early development versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-alpha/}}` | ![](assets/version-license-guide/swatch_8360a2df6a3a09a9.svg) |
| `{{ui:version:2.0.0-alpha.3/}}` | ![](assets/version-license-guide/swatch_f3ed2cbde31d89e3.svg) |

#### Development Versions (Purple)

Unstable development builds:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-dev/}}` | ![](assets/version-license-guide/swatch_10934951eb9b45a2.svg) |
| `{{ui:version:2.0.0-snapshot/}}` | ![](assets/version-license-guide/swatch_b003e78043030954.svg) |
| `{{ui:version:3.0.0-nightly/}}` | ![](assets/version-license-guide/swatch_a404e9bf1aa845af.svg) |

#### Deprecated Versions (Red)

End-of-life or unsupported versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-deprecated/}}` | ![](assets/version-license-guide/swatch_9669089e3a0febc6.svg) |
| `{{ui:version:0.5.0-eol/}}` | ![](assets/version-license-guide/swatch_7aaeaea7287162ee.svg) |

---

### Status Override

Override auto-detection with the `status` parameter:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:status=stable/}}` | ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) |
| `{{ui:version:1.0.0:status=beta/}}` | ![](assets/version-license-guide/swatch_9cf2674a1fa00a77.svg) |
| `{{ui:version:1.0.0:status=alpha/}}` | ![](assets/version-license-guide/swatch_4d5a109448317622.svg) |
| `{{ui:version:1.0.0:status=dev/}}` | ![](assets/version-license-guide/swatch_6e47135ce5eb8926.svg) |
| `{{ui:version:1.0.0:status=deprecated/}}` | ![](assets/version-license-guide/swatch_a74eb32da9fe75a0.svg) |

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
| `{{ui:version:1.0.0:prefix=/}}` | ![](assets/version-license-guide/swatch_491303093c8f1c8a.svg) |
| `{{ui:version:2.5.0:prefix=/}}` | ![](assets/version-license-guide/swatch_5d14fd810511fbc1.svg) |

#### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_4cea3692a9161030.svg) |
| `{{ui:version:2.0.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_f6c472ffb2214d47.svg) |
| `{{ui:version:3.0.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_9812d266d09d0a63.svg) |

#### Badge Styles

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:style=flat/}}` | ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) |
| `{{ui:version:1.0.0:style=plastic/}}` | ![](assets/version-license-guide/swatch_15d0b88a9cd9222a.svg) |
| `{{ui:version:1.0.0:style=for-the-badge/}}` | ![](assets/version-license-guide/swatch_7829ebd7840d7397.svg) |

---

## License Badges

### License Syntax

```markdown
{{ui:license:LICENSE/}}
```

The license component categorizes licenses and applies semantic coloring.

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_00b3009047beb542.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_d40867a144d33145.svg) |
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_1d89a9075027cb37.svg) |

---

### License Categories

#### Permissive Licenses (Green)

Open-source friendly, minimal restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_00b3009047beb542.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_d40867a144d33145.svg) |
| `{{ui:license:BSD-3-Clause/}}` | ![](assets/version-license-guide/swatch_c270e776b2fd5226.svg) |
| `{{ui:license:BSD-2-Clause/}}` | ![](assets/version-license-guide/swatch_2b7a966e37b9c89f.svg) |
| `{{ui:license:ISC/}}` | ![](assets/version-license-guide/swatch_4ef889f88f2927f1.svg) |

#### Weak Copyleft (Blue)

File-level copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:LGPL-3.0/}}` | ![](assets/version-license-guide/swatch_9daf821d1b5d90f9.svg) |
| `{{ui:license:LGPL-2.1/}}` | ![](assets/version-license-guide/swatch_c4b15d4dd17dbfb8.svg) |
| `{{ui:license:MPL-2.0/}}` | ![](assets/version-license-guide/swatch_7fa9d74778e73b06.svg) |
| `{{ui:license:EPL-2.0/}}` | ![](assets/version-license-guide/swatch_f8bc6c7584e26d0d.svg) |

#### Copyleft (Yellow)

Strong copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_1d89a9075027cb37.svg) |
| `{{ui:license:GPL-2.0/}}` | ![](assets/version-license-guide/swatch_c0eb241fc470a537.svg) |
| `{{ui:license:AGPL-3.0/}}` | ![](assets/version-license-guide/swatch_0009b6c0d1f3b667.svg) |

#### Public Domain (Cyan)

No restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:CC0/}}` | ![](assets/version-license-guide/swatch_548fb59ac01c8b78.svg) |
| `{{ui:license:Unlicense/}}` | ![](assets/version-license-guide/swatch_ad6e526b20506398.svg) |

#### Proprietary (Gray)

Closed source:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:Proprietary/}}` | ![](assets/version-license-guide/swatch_0ac405a64c3452da.svg) |
| `{{ui:license:Commercial/}}` | ![](assets/version-license-guide/swatch_974e7ba011456d31.svg) |

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
| `{{ui:license:MIT:label=MIT License/}}` | ![](assets/version-license-guide/swatch_b323a1c375b0bb64.svg) |
| `{{ui:license:Apache-2.0:label=Apache/}}` | ![](assets/version-license-guide/swatch_e1f9d2512e0cbba0.svg) |
| `{{ui:license:GPL-3.0:label=GPLv3/}}` | ![](assets/version-license-guide/swatch_ef86f6d0372995bb.svg) |

### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_34085aace07002f2.svg) |
| `{{ui:license:Apache-2.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_c2418a4eb8800b19.svg) |
| `{{ui:license:GPL-3.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_a5603170fb078674.svg) |

---

## Combining with Tech Badges

Create comprehensive project headers with tech, version, and license badges:

### Project Header Example

```markdown
{{ui:tech:rust/}} {{ui:version:1.0.0/}} {{ui:license:MIT/}}
```

![](assets/version-license-guide/tech_9b07f32e2323dccd.svg) ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) ![](assets/version-license-guide/swatch_00b3009047beb542.svg)

### Full Stack Example

```markdown
{{ui:tech:typescript/}} {{ui:tech:react/}} {{ui:version:2.5.0-beta/}} {{ui:license:Apache-2.0/}}
```

![](assets/version-license-guide/tech_b30721c0a0394c2e.svg) ![](assets/version-license-guide/tech_fa93a6b13b34f67b.svg) ![](assets/version-license-guide/swatch_103e4f7a79b79310.svg) ![](assets/version-license-guide/swatch_d40867a144d33145.svg)

### Deprecated Project

```markdown
{{ui:tech:python/}} {{ui:version:0.5.0:status=deprecated/}} {{ui:license:GPL-3.0/}}
```

![](assets/version-license-guide/tech_c5b0cf28158ee95f.svg) ![](assets/version-license-guide/swatch_989ecfd1926548b5.svg) ![](assets/version-license-guide/swatch_1d89a9075027cb37.svg)

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
| ![](assets/version-license-guide/tech_837edd35922729ee.svg) ![](assets/version-license-guide/swatch_d3def3eda904b9f7.svg) ![](assets/version-license-guide/swatch_00b3009047beb542.svg) | ![](assets/version-license-guide/tech_cd9cdaecb20390d2.svg) ![](assets/version-license-guide/swatch_7829ebd7840d7397.svg) ![](assets/version-license-guide/swatch_c542818011354b9b.svg) |

### 4. Custom Colors for Branding

Override default colors to match your project theme:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_e65f929d1a77716c.svg) |
| `{{ui:license:MIT:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_c96def03bf42bb0d.svg) |

---

## See Also

- [Tech Badges](TECH-GUIDE.md) - Technology logo badges
- [Swatches](SWATCH-GUIDE.md) - Color block component
- [Components](COMPONENTS-GUIDE.md) - All UI components
- [Colors](COLORS-GUIDE.md) - Palette reference
