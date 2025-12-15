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

### Braille Patterns

Bar graph elements (fills from bottom-left, then bottom-right):

| Name | Character | Description |
|------|-----------|-------------|
| `braille.empty` | ⠀ | Empty (no dots) |
| `braille.bar.1` | ⡀ | 1/8 filled |
| `braille.bar.2` | ⡄ | 2/8 filled |
| `braille.bar.3` | ⡆ | 3/8 filled |
| `braille.bar.4` | ⡇ | 4/8 (left column) |
| `braille.bar.5` | ⣇ | 5/8 filled |
| `braille.bar.6` | ⣧ | 6/8 filled |
| `braille.bar.7` | ⣷ | 7/8 filled |
| `braille.bar.8` | ⣿ | Full (all dots) |
| `braille.full` | ⣿ | All dots filled |
| `braille.left` | ⡇ | Left column |
| `braille.right` | ⢸ | Right column |

### Box Drawing

Elements: `h` (horizontal), `v` (vertical), `tl/tr/bl/br` (corners), `cross`, `t-up/t-down/t-left/t-right` (T-junctions)

**Light** (`box.light.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.light.h` | ─ | `box.light.v` | │ |
| `box.light.tl` | ┌ | `box.light.tr` | ┐ |
| `box.light.bl` | └ | `box.light.br` | ┘ |
| `box.light.cross` | ┼ | `box.light.t-down` | ┬ |
| `box.light.t-up` | ┴ | `box.light.t-right` | ├ |
| `box.light.t-left` | ┤ | | |

**Heavy** (`box.heavy.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.heavy.h` | ━ | `box.heavy.v` | ┃ |
| `box.heavy.tl` | ┏ | `box.heavy.tr` | ┓ |
| `box.heavy.bl` | ┗ | `box.heavy.br` | ┛ |
| `box.heavy.cross` | ╋ | `box.heavy.t-down` | ┳ |
| `box.heavy.t-up` | ┻ | `box.heavy.t-right` | ┣ |
| `box.heavy.t-left` | ┫ | | |

**Double** (`box.double.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.double.h` | ═ | `box.double.v` | ║ |
| `box.double.tl` | ╔ | `box.double.tr` | ╗ |
| `box.double.bl` | ╚ | `box.double.br` | ╝ |
| `box.double.cross` | ╬ | `box.double.t-down` | ╦ |
| `box.double.t-up` | ╩ | `box.double.t-right` | ╠ |
| `box.double.t-left` | ╣ | | |

**Round corners** (`box.round.*`):

| Name | Character | Description |
|------|-----------|-------------|
| `box.round.tl` | ╭ | Rounded top-left |
| `box.round.tr` | ╮ | Rounded top-right |
| `box.round.bl` | ╰ | Rounded bottom-left |
| `box.round.br` | ╯ | Rounded bottom-right |

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

### Braille bar chart

```markdown
{{glyph:braille.bar.2/}}{{glyph:braille.bar.5/}}{{glyph:braille.bar.8/}}{{glyph:braille.bar.6/}}{{glyph:braille.bar.3/}}
```

Output: `⡄⣇⣿⣧⡆`

### Box drawing frame

```markdown
{{glyph:box.round.tl/}}{{glyph:box.light.h/}}{{glyph:box.light.h/}}{{glyph:box.round.tr/}}
{{glyph:box.light.v/}} Hi {{glyph:box.light.v/}}
{{glyph:box.round.bl/}}{{glyph:box.light.h/}}{{glyph:box.light.h/}}{{glyph:box.round.br/}}
```

Output:
```
╭──╮
│ Hi │
╰──╯
```
