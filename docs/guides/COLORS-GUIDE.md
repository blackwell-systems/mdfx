# Colors Guide

mdfx uses a simple color system: named tokens resolve to hex values, or pass through raw hex directly.

## Using Colors

Anywhere a color parameter is accepted:

```markdown
{{ui:swatch:success/}}          <!-- palette token -->
{{ui:swatch:FF5500/}}           <!-- direct hex -->
{{ui:progress:75:fill=cobalt/}}
{{ui:donut:50:track=333333/}}
{{ui:tech:rust:bg=1a1a1a:border=61DAFB/}}
```

## Resolution

1. Look up name in palette → if found, use hex value
2. If not found, treat input as raw hex (must be 6 digits, no `#`)

## Built-in Palette

| Token | Hex | Purpose |
|-------|-----|---------|
| `success` | 22C55E | Positive/complete state |
| `warning` | EAB308 | Caution state |
| `error` | EF4444 | Danger/failure state |
| `info` | 3B82F6 | Informational state |
| `white` | FFFFFF | Pure white |
| `black` | 000000 | Pure black |
| `ink` | 111111 | Near-black (text) |
| `gray` | 6B7280 | Neutral gray |
| `slate` | 475569 | Slate gray |
| `cobalt` | 2B6CB0 | Blue accent |
| `plum` | 6B46C1 | Purple accent |
| `pink` | F41C80 | Pink/magenta accent |
| `accent` | F41C80 | Primary accent (same as pink) |
| `dark1` | 292A2D | Dark background level 1 |
| `dark2` | 292C34 | Dark background level 2 |
| `dark3` | 282F3C | Dark background level 3 |
| `dark4` | 263143 | Dark background level 4 |

## Custom Palettes

Create a JSON file with your colors:

```json
{
  "brand": "FF5500",
  "background": "1a1a2e",
  "highlight": "00FF41"
}
```

Use via CLI:

```bash
mdfx process --palette brand.json input.md
```

Your custom tokens merge with (and override) built-in tokens.

## Where Colors Apply

| Component | Parameters |
|-----------|------------|
| `swatch` | color (main), `border`, `label_color`, `icon_color` |
| `progress` | `fill`, `track`, `border`, `thumb_color`, `label_color` |
| `donut` | `fill`, `track`, `thumb_color`, `label_color` |
| `gauge` | `fill`, `track`, `thumb_color`, `label_color` |
| `sparkline` | `fill`, `stroke`, `track` |
| `waveform` | `positive`, `negative`, `track`, `center_color` |
| `rating` | `fill`, `empty` |
| `tech` | `bg`, `bg_left`, `bg_right`, `logo`, `text`, `text_color`, `border` |

## Examples

```markdown
<!-- Semantic colors for status -->
{{ui:progress:100:fill=success/}}
{{ui:progress:50:fill=warning/}}
{{ui:progress:25:fill=error/}}

<!-- Custom hex for branding -->
{{ui:swatch:FF6B35/}} {{ui:swatch:1a1a2e/}} {{ui:swatch:00FF41/}}

<!-- Mix palette and hex -->
{{ui:progress:75:fill=cobalt:track=1a1a1a/}}

<!-- Tech badges with custom colors -->
{{ui:tech:rust:bg=dark1:border=accent/}}
{{ui:tech:typescript:bg_left=007ACC:bg_right=1a1a1a/}}
```
