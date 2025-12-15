# Frame System

**Status:** v1.0.0 Shipped
**Last Updated:** 2025-12-14

---

## Overview

Frames add **decorative prefix/suffix** around text without changing the characters themselves. They're simple string concatenation: `prefix + content + suffix`.

## Template Syntax

```markdown
{{frame:style}}content{{/frame}}
```

**Example:**
```markdown
{{frame:gradient}}TITLE{{/frame}}
```

**Output:**
```
▓▒░ TITLE ░▒▓
```

### With Separator

Add `/separator=X` to insert a character between pattern glyphs:

```markdown
{{fr:gradient/separator=dot}}TITLE{{/}}
```

**Output:**
```
▓·▒·░ TITLE ░·▒·▓
```

Separator can be a named glyph (like `dot`, `dash`) or a literal character (like `·`, `-`).

### Glyph Frames with Separator

Glyph frames also support separators:

```markdown
{{fr:glyph:star*3/separator=dot}}Title{{/}}
```

**Output:**
```
★·★·★ Title ★·★·★
```

---

## Available Frame Styles (27)

All frames use simple string concatenation: `prefix + content + suffix`

**Categories:**
- **Gradient:** 3 styles (gradient, gradient-light, gradient-reverse)
- **Solid:** 3 styles (solid-left, solid-right, solid-both)
- **Lines:** 4 styles (line-light, line-bold, line-double, line-dashed)
- **Blocks:** 2 styles (block-top, block-bottom)
- **Arrows/Symbols:** 4 styles (arrow-right, dot, bullet, star)
- **Brackets:** 5 styles (lenticular, angle, guillemet, guillemet-single, heavy-quote)
- **Special:** 6 styles (diamond, triangle-right, finger, fisheye, asterism, arc-top, arc-bottom)

---

## Data Structure

Frames are defined in the unified `registry.json` using either **pattern+mode** (declarative) or **explicit prefix/suffix** format.

### Pattern + Mode Format (Recommended)

```json
{
  "frames": {
    "gradient": {
      "pattern": "▓▒░",
      "mode": "mirror",
      "description": "Gradient blocks from bold to light",
      "contexts": ["inline", "block"],
      "aliases": ["grad", "gradient-full"]
    },
    "line-bold": {
      "pattern": "━━━",
      "mode": "repeat",
      "description": "Bold horizontal lines",
      "contexts": ["inline", "block"],
      "aliases": ["bold-line"]
    },
    "solid-left": {
      "pattern": "█▌",
      "mode": "prefix-only",
      "description": "Solid block on the left",
      "contexts": ["inline", "block"],
      "aliases": ["solidleft", "left"]
    }
  }
}
```

### Suffix Modes

| Mode | Description | Example |
|------|-------------|---------|
| `mirror` | Suffix = pattern reversed | `▓▒░` → prefix=`▓▒░ `, suffix=` ░▒▓` |
| `repeat` | Suffix = same as pattern | `━━━` → prefix=`━━━ `, suffix=` ━━━` |
| `prefix-only` | No suffix | `█▌` → prefix=`█▌`, suffix=`` |
| `suffix-only` | No prefix | `▐█` → prefix=``, suffix=`▐█` |

### Explicit Prefix/Suffix Format

For asymmetric pairs (different glyphs for prefix/suffix):

```json
{
  "frames": {
    "star": {
      "prefix": "★ ",
      "suffix": " ☆",
      "description": "Black and white stars",
      "contexts": ["inline", "block"],
      "aliases": ["stars", "featured"]
    },
    "guillemet": {
      "prefix": "« ",
      "suffix": " »",
      "description": "French quotation marks",
      "contexts": ["inline", "block"],
      "aliases": ["french", "quote"]
    }
  }
}
```

### Key Points

- Data embedded at compile time via `include_str!()`
- **Pattern+mode** generates prefix/suffix at load time (DRY, declarative)
- **Explicit prefix/suffix** for asymmetric pairs like star (★/☆), diamond (◆/◇)
- Mirror mode automatically reverses Unicode characters
- Alias support for shorter names
- No width calculation - frames are applied as-is

---

## Composition

Frames can nest with other templates:

```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```

**Output:**
```
▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓
```

Parser processes in priority order:
1. UI templates (expand to primitives)
2. Frame templates (outer)
3. Shields templates (middle)
4. Style templates (inner)

mdfx composes features through **nesting**, not a separate DSL. Pipe syntax like `{{mathbold|frame:double}}` is not supported - keep it simple.

**Close-all syntax** for deeply nested frames:
```markdown
{{fr:gradient}}{{fr:star}}{{mathbold}}NESTED{{//}}
<!-- Closes mathbold, star, gradient in reverse order -->
```

---

## Use Cases

**Section headers:**
```markdown
{{frame:line-bold}}INSTALLATION{{/frame}}
```

**Warnings:**
```markdown
{{frame:solid-left}}{{negative-squared}}WARNING{{/negative-squared}}{{/frame}}
```

**Branded titles:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}PROJECT{{/mathbold}}{{/frame}}
```

---

## Limitations

- **Single-line only** - No multiline content support
- **No width calculation** - Frames applied as-is
- **No box borders** - Just prefix/suffix, not full rectangles
- **Unicode width variance** - Suffix may not align with wide Unicode characters
- **Best for monospace** - Designed for fixed-width display

These are intentional design constraints. Frames are simple prefix/suffix decoration.

---

## Implementation

Frames are trivial string concatenation:

```rust
pub fn apply_frame(&self, text: &str, frame_style: &str) -> Result<String> {
    let frame = self.get_frame(frame_style)?;
    Ok(format!("{}{}{}", frame.prefix, text, frame.suffix))
}
```

No width calculation, no wrapping, no complexity.

---

## References

- **Implementation:** `crates/mdfx/src/registry.rs` (Frame struct, SuffixMode enum)
- **Data:** `crates/mdfx/data/registry.json` (renderables.frames section)
- **Parser:** `crates/mdfx/src/parser.rs` (frame template parsing, close-all)
- **Tests:** `crates/mdfx/src/parser.rs` (frame template tests)
