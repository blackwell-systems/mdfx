# Glyphs Guide

Named Unicode characters for use in templates, separators, and frames.

## Syntax

```markdown
{{glyph:name/}}
```

Or as a separator in style templates:

```markdown
{{mathbold:separator=dot}}HELLO{{/mathbold}}
```

---

## Available Glyphs

### Separators

Short names for common inline characters:

| Name | Character | Usage |
|------|-----------|-------|
| `dot` | · | `separator=dot` |
| `bullet` | • | `separator=bullet` |
| `dash` | ─ | `separator=dash` |
| `bolddash` | ━ | `separator=bolddash` |
| `arrow` | → | `separator=arrow` |
| `star` | ★ | `separator=star` |
| `diamond` | ◆ | `separator=diamond` |
| `pipe` | \| | `separator=pipe` |
| `tilde` | ~ | `separator=tilde` |

### Block Elements

Numbers represent eighths (1 = 1/8, 4 = 1/2, 7 = 7/8):

| Name | Character | Description |
|------|-----------|-------------|
| `block.full` | █ | Full block |
| `block.upper.1` | ▔ | Upper 1/8 |
| `block.upper.4` | ▀ | Upper half |
| `block.lower.1` | ▁ | Lower 1/8 |
| `block.lower.2` | ▂ | Lower 1/4 |
| `block.lower.3` | ▃ | Lower 3/8 |
| `block.lower.4` | ▄ | Lower half |
| `block.lower.5` | ▅ | Lower 5/8 |
| `block.lower.6` | ▆ | Lower 3/4 |
| `block.lower.7` | ▇ | Lower 7/8 |
| `block.left.1` | ▏ | Left 1/8 |
| `block.left.2` | ▎ | Left 1/4 |
| `block.left.3` | ▍ | Left 3/8 |
| `block.left.4` | ▌ | Left half |
| `block.left.5` | ▋ | Left 5/8 |
| `block.left.6` | ▊ | Left 3/4 |
| `block.left.7` | ▉ | Left 7/8 |
| `block.right.1` | ▕ | Right 1/8 |
| `block.right.4` | ▐ | Right half |

### Shades

| Name | Character | Description |
|------|-----------|-------------|
| `shade.light` | ░ | Light shade |
| `shade.medium` | ▒ | Medium shade |
| `shade.dark` | ▓ | Dark shade |

### Quadrants

Grid positions: 1=top-left, 2=top-right, 3=bottom-left, 4=bottom-right

```
1 | 2
-----
3 | 4
```

| Name | Character | Filled positions |
|------|-----------|------------------|
| `quad.1` | ▘ | Top-left |
| `quad.2` | ▝ | Top-right |
| `quad.3` | ▖ | Bottom-left |
| `quad.4` | ▗ | Bottom-right |
| `quad.1-4` | ▚ | Diagonal (TL + BR) |
| `quad.2-3` | ▞ | Diagonal (TR + BL) |
| `quad.1-3-4` | ▙ | All except TR |
| `quad.1-2-3` | ▛ | All except BR |
| `quad.1-2-4` | ▜ | All except BL |
| `quad.2-3-4` | ▟ | All except TL |

---

## Examples

### Gradient border

```markdown
{{glyph:shade.dark/}}{{glyph:shade.medium/}}{{glyph:shade.light/}} Title {{glyph:shade.light/}}{{glyph:shade.medium/}}{{glyph:shade.dark/}}
```

Output: `▓▒░ Title ░▒▓`

### Progress bar

```markdown
{{glyph:block.full/}}{{glyph:block.full/}}{{glyph:block.full/}}{{glyph:block.left.4/}}{{glyph:block.left.1/}}
```

Output: `███▌▏`

### Styled text with separator

```markdown
{{mathbold:separator=star}}HELLO{{/mathbold}}
```

Output: `𝐇★𝐄★𝐋★𝐋★𝐎`
