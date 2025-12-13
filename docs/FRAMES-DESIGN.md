# Frames Design Document

**Scope:** Inline frames (prefix/suffix decoration) - Implemented
**Status:** v1.0.0 Shipped
**Last Updated:** 2025-12-12

---

## What's Implemented (v1.0.0)

### Inline Frames

Frames add **decorative prefix/suffix** around text without changing the characters themselves.

**Template Syntax:**
```markdown
{{frame:style}}content{{/frame}}
```

**Example:**
```markdown
{{frame:gradient}}TITLE{{/frame}}
→ ▓▒░ TITLE ░▒▓
```

### Frame Types (27 styles)

All frames use simple string concatenation: `prefix + content + suffix`

**Categories:**
- **Gradient:** 3 styles (gradient, gradient-light, gradient-reverse)
- **Solid:** 3 styles (solid-left, solid-right, solid-both)
- **Lines:** 4 styles (line-light, line-bold, line-double, line-dashed)
- **Blocks:** 2 styles (block-top, block-bottom)
- **Arrows/Symbols:** 4 styles (arrow-right, dot, bullet, star)
- **Brackets:** 5 styles (lenticular, angle, guillemet, guillemet-single, heavy-quote)
- **Special:** 6 styles (diamond, triangle-right, finger, fisheye, asterism, arc-top, arc-bottom)

### Data Structure (frames.json)

Current implementation uses embedded JSON with this structure:

```json
{
  "version": "1.0.0",
  "frames": {
    "gradient": {
      "id": "gradient",
      "name": "Gradient",
      "description": "Gradient blocks from bold to light",
      "prefix": "▓▒░ ",
      "suffix": " ░▒▓",
      "aliases": ["grad", "gradient-full"]
    },
    "solid-left": {
      "id": "solid-left",
      "name": "Solid Left",
      "description": "Solid block on the left",
      "prefix": "█▌",
      "suffix": "",
      "aliases": ["solidleft", "left"]
    }
  }
}
```

**Key points:**
- Data embedded at compile time via `include_str!()`
- Each frame has `prefix` and `suffix` (both can be empty string)
- Alias support for shorter names
- No width calculation - frames are applied as-is

### Parser Behavior

**Template Parsing:**
```rust
// Parser recognizes {{frame:style}}
parse_frame_at() → FrameData {
    frame_style: "gradient",
    content: "TITLE",
    end_pos: 35
}

// Renderer applies
apply_frame("TITLE", "gradient") → "▓▒░ TITLE ░▒▓"
```

**Recursive Processing:**
Frames can contain other templates:
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
→ ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓
```

Parser processes in priority order:
1. UI templates (expand to primitives)
2. Frame templates (outer)
3. Badge templates (middle)
4. Shields templates (visual primitives)
5. Style templates (inner)

This prevents parsing ambiguity and enables component expansion.

### Composition Model (Canonical)

utf8fx composes features through **nesting**, not a separate DSL.

**Example:**
```markdown
{{frame:double}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```

**Rationale:**
- Reuses existing recursive parser behavior
- No new grammar or pipe syntax needed
- Templates remain readable
- Order is explicit: outer → inner

**Not supported:** Pipe syntax like `{{mathbold|frame:double}}` - keep it simple.

### Testing

113 tests covering:
- All 27 frame types
- Frame + style composition
- Frame + badge composition
- Frame + separator composition
- Recursive nesting
- Error handling (unknown frames, unclosed tags)

### Use Cases

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

### Limitations

**Known constraints:**
- No multiline content support
- No automatic width calculation
- No box borders (top/bottom)
- Suffix may not align if content contains wide Unicode
- Best for single-line, monospace text

These are intentional - inline frames are simple prefix/suffix decoration.

---

## What's Planned (v1.1.0+)

### Future: Brackets & Boxes

**Scope:** Full rectangular boxes with borders.

**Not yet designed:** The sections below are proposals, not commitments.

### Proposed: Inline Brackets

Similar to frames but with specific bracket pairs:

```markdown
{{bracket:curly}}content{{/bracket}} → { content }
{{bracket:square}}content{{/bracket}} → [ content ]
{{bracket:angle}}content{{/bracket}} → ⟨ content ⟩
```

**Implementation approach:**
- Separate `brackets.json` or add to `frames.json` as category
- Reuse FrameRenderer (brackets are just prefix/suffix)
- Add bracket-specific error handling

**Open questions:**
- Do we need this? Frames already support bracket styles like `lenticular` (【】)
- Would users prefer `{{frame:square}}` over new `{{bracket:}}` syntax?

### Proposed: Single-Line Boxes

Full boxes with top/bottom borders:

```markdown
{{box:double}}WARNING{{/box}}

→ ╔═════════╗
  ║ WARNING ║
  ╚═════════╝
```

**Implementation challenges:**

1. **Width calculation:**
   - Need Unicode display width (not `char.len()`)
   - Use `unicode-width` crate
   - Account for wide chars (CJK), emoji, combining marks

2. **Box sizing:**
   ```rust
   // Naive (wrong)
   let width = content.chars().count();

   // Better
   use unicode_width::UnicodeWidthStr;
   let width = content.width();
   ```

3. **Padding/alignment:**
   ```markdown
   {{box:double:padding=2:align=center}}TEXT{{/box}}
   ```

4. **GitHub compatibility:**
   - GitHub markdown width: ~90 chars
   - Mobile views: ~60 chars
   - Add `--width-limit` validation (warning only)

**Data structure proposal:**
```json
{
  "boxes": {
    "double": {
      "top_left": "╔",
      "top": "═",
      "top_right": "╗",
      "left": "║",
      "right": "║",
      "bottom_left": "╚",
      "bottom": "═",
      "bottom_right": "╝"
    }
  }
}
```

**Open questions:**
- Separate `boxes.json` or extend `frames.json`?
- How to handle wrapping long text?
- Should we auto-fit to terminal width?

### Proposed: Multiline Boxes

Boxes with automatic text wrapping:

```markdown
{{box:heavy:width=60}}
This is a very long sentence that will automatically
wrap within the box at the specified width.
{{/box}}
```

**Major complexity:**
- Word wrapping algorithm
- Unicode width for every character
- Handling existing line breaks
- Performance impact

**Recommendation:** Ship v1.1 with single-line boxes first. Multiline is v2.0+.

### Composition Strategy

**If we add boxes, keep nesting:**
```markdown
{{box:double}}{{frame:gradient}}{{mathbold}}TITLE{{/mathbold}}{{/frame}}{{/box}}
```

**Do not add pipe syntax** like `{{mathbold|box:double}}` - it's a new grammar with edge cases.

### Validation Mode (Good Idea)

For GitHub width limits:
```bash
utf8fx process README.md --validate-width github
# Warning: Box on line 42 is 95 chars (GitHub limit: 90)
```

**Implementation:**
- Warning only (don't fail)
- Presets: `github`, `mobile`, `terminal-80`
- Check after rendering, before output

### Open Design Questions

**Before implementing boxes:**

1. **JSON file structure:**
   - Option A: Keep `frames.json` simple (prefix/suffix only), add `boxes.json`
   - Option B: Extend `frames.json` with `"type": "inline" | "box"`

2. **Template syntax:**
   - Keep `{{frame:}}` for inline, add `{{box:}}` for bordered? (Recommended)
   - Or use `{{frame:double:boxed=true}}`? (Confusing)

3. **Unicode width library:**
   - Use `unicode-width` crate (adds ~30KB to binary)
   - Or naive `char.len()` with documented limitations?

4. **Testing strategy:**
   - How to test visual box rendering?
   - Snapshot tests with fixed-width examples?

### Roadmap

**v1.1.0 (Proposed):**
- Single-line boxes with border characters
- Unicode width calculation
- `--validate-width` flag

**v2.0.0 (Proposed):**
- Multiline boxes with text wrapping
- Advanced alignment/padding options
- Box nesting rules

**No timelines** - these are proposals, not commitments.

---

## Design Principles

These apply to both implemented and planned features:

### 1. Component Separation

**Converter** → Character transformation (styles)
**FrameRenderer** → Prefix/suffix decoration (frames)
**BoxRenderer** (future) → Multi-line bordered boxes

Each has single responsibility.

### 2. Consistent Template Syntax

**Pattern:** `{{type:style}}content{{/type}}`

**Examples:**
- `{{mathbold}}TEXT{{/mathbold}}` - style
- `{{frame:gradient}}TEXT{{/frame}}` - frame
- `{{box:double}}TEXT{{/box}}` - box (future)

**With parameters:** `{{type:style:param=value}}`

### 3. Composition via Nesting

**Do:** Nest templates explicitly
```markdown
{{frame:gradient}}{{mathbold}}TEXT{{/mathbold}}{{/frame}}
```

**Don't:** Invent pipe syntax
```markdown
{{mathbold|frame:gradient}}TEXT{{/mathbold}}  (Not supported)
```

### 4. Data-Driven Configuration

**All character mappings in JSON:**
- `styles.json` - Character transformations
- `frames.json` - Frame decorations
- `boxes.json` - Box borders (future)

**Not in code** - enables user extensions without recompiling... wait, we use `include_str!()` so users DO need to recompile. That's fine - keeps deployment simple.

### 5. Fail Fast

**Error for unknown templates:**
```rust
{{frame:invalid}}TEXT{{/frame}}
→ Error::UnknownFrame("invalid")
```

**Not silent fallback** - users should know when they typo.

---

## Technical Notes

### Why Inline Frames Work

Inline frames are **trivial** - just string concatenation:

```rust
pub fn apply_frame(&self, text: &str, frame_style: &str) -> Result<String> {
    let frame = self.get_frame(frame_style)?;
    Ok(format!("{}{}{}", frame.prefix, text, frame.suffix))
}
```

No width calculation, no wrapping, no complexity.

### Why Boxes Are Hard

Boxes require:
1. **Unicode width** - not same as `char.len()`
2. **Border drawing** - top/bottom with correct width
3. **Padding logic** - spaces inside borders
4. **Alignment** - center/left/right
5. **Platform testing** - terminal vs GitHub vs mobile

Each adds complexity.

### Performance Impact

**Inline frames:** ~O(1) string concatenation
**Boxes:** ~O(n) where n = content length (width calculation)

Boxes will be slower. Document this.

---

## References

- **Current implementation:** `src/frames.rs`
- **Current data:** `data/frames.json`
- **Tests:** `src/parser.rs` (frame template tests)
- **Unicode width:** https://crates.io/crates/unicode-width
- **Box drawing chars:** U+2500–U+257F

---

**Document Status:** Reflects v1.0.0 reality + proposals for v1.1+
