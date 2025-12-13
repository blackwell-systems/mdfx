# Badge Style Control - Feature Planning

**Status:** Not implemented in v1.0.0  
**Target:** v1.1.0  
**Priority:** HIGH - Critical for design flexibility

---

## Problem Statement

mdfx can generate color blocks in any hex color, but all blocks have the same visual style (flat-square). This limits design composition - like having only one type of Minecraft brick.

**Users need:**
- Different block shapes (square, rounded, tall)
- Different visual styles (flat, shiny, textured)
- Mix and match for complex designs

---

## Current Limitations

### Shields.io Backend
Hardcoded to `style=flat-square` in all URL generation:
```markdown
{{ui:swatch:F41C80/}}
→ https://img.shields.io/badge/-%20-F41C80?style=flat-square
```

### SVG Backend
Only generates flat-square style rectangles:
```svg
<svg width="40" height="20">
  <rect width="40" height="20" fill="#F41C80"/>
</svg>
```

**Both backends need style control.**

---

## Shields.io Styles Available

### flat-square (Current Default)
```
style=flat-square
```
Sharp corners, compact, modern

### flat (Rounded Corners)  
```
style=flat
```
Soft corners, friendly, approachable

### for-the-badge (Tall Blocks)
```
style=for-the-badge
```
Tall, bold, prominent - great for hero sections

### plastic (Shiny/3D)
```
style=plastic
```
Glossy effect, depth, retro feel

### social
```
style=social
```
Social media style, condensed

---

## SVG Backend Style Variations

The SVG backend should generate different shapes to match shields.io styles:

### flat-square
```svg
<rect width="40" height="20" rx="0" fill="#F41C80"/>
```
No border radius

### flat (rounded)
```svg
<rect width="40" height="20" rx="3" fill="#F41C80"/>
```
3px border radius

### for-the-badge (tall)
```svg
<rect width="40" height="28" rx="0" fill="#F41C80"/>
```
Taller (28px vs 20px)

### plastic (gradient)
```svg
<defs>
  <linearGradient id="grad" x1="0%" y1="0%" x2="0%" y2="100%">
    <stop offset="0%" style="stop-color:#F86CA7" />
    <stop offset="100%" style="stop-color:#F41C80" />
  </linearGradient>
</defs>
<rect width="40" height="20" rx="3" fill="url(#grad)"/>
```
Vertical gradient for shine effect

---

## Proposed User-Facing Syntax

### UI Component (Recommended)
```markdown
{{ui:swatch:F41C80/}}                          → flat-square (default)
{{ui:swatch:F41C80:style=flat/}}              → rounded corners
{{ui:swatch:F41C80:style=for-the-badge/}}     → tall block
{{ui:swatch:F41C80:style=plastic/}}           → shiny gradient
```

### Primitive (Advanced Users)
```markdown
{{shields:block:color=F41C80:style=flat/}}
```

### Component Definition (components.json)
```json
{
  "swatch": {
    "type": "native",
    "self_closing": true,
    "description": "Single color block",
    "args": ["color", "style?"],
    "default_style": "flat-square"
  }
}
```

---

## Design Examples with Style Control

### Gradient Bar (Mixed Heights)
```markdown
{{ui:swatch:F41C80/}} {{ui:swatch:EAB308:style=for-the-badge/}} {{ui:swatch:22C55E/}}
```
Middle block taller for emphasis

### Soft Color Palette (Rounded)
```markdown
{{ui:swatch:F41C80:style=flat/}} {{ui:swatch:EAB308:style=flat/}} {{ui:swatch:22C55E:style=flat/}}
```
Friendly, approachable palette

### Hero Section (Tall Blocks)
```markdown
## {{ui:swatch:F41C80:style=for-the-badge/}} Features
```
Prominent visual accent

### Retro Style (Shiny)
```markdown
{{ui:swatch:F41C80:style=plastic/}} {{ui:swatch:EAB308:style=plastic/}} {{ui:swatch:22C55E:style=plastic/}}
```
3D glossy effect

---

## Implementation Tasks

### Phase 1: Shields.io Backend (Easier)

**1. Update Primitive enum** (`src/primitive.rs`)
```rust
pub enum Primitive {
    Swatch { 
        color: String,
        style: String,  // Add this field
    },
    // ... other variants
}
```

**2. Update ComponentsRenderer** (`src/components.rs`)
```rust
// Parse style from args
let style = args.get(1).unwrap_or("flat-square");
ComponentOutput::Primitive(Primitive::Swatch {
    color: color.to_string(),
    style: style.to_string(),
})
```

**3. Update ShieldsRenderer** (`src/renderer/shields.rs`)
```rust
Primitive::Swatch { color, style } => {
    self.render_block(color, style)  // Pass style through
}
```

**4. Add tests**
- Test all 5 shields.io styles
- Test default behavior (flat-square)
- Test invalid style handling

### Phase 2: SVG Backend (More Complex)

**1. Update SvgBackend** (`src/renderer/svg.rs`)
```rust
fn render_swatch(&self, color: &str, style: &str) -> Vec<u8> {
    match style {
        "flat" => self.render_rounded_rect(color, 3),
        "for-the-badge" => self.render_tall_rect(color, 28),
        "plastic" => self.render_gradient_rect(color),
        _ => self.render_flat_square(color),
    }
}
```

**2. Implement style renderers**
- `render_flat_square(color)` - Simple rect, no radius
- `render_rounded_rect(color, radius)` - With rx attribute
- `render_tall_rect(color, height)` - Taller dimensions  
- `render_gradient_rect(color)` - Linear gradient for shine

**3. Deterministic hashing**
```rust
// Include style in hash
let hash_input = format!("swatch_{}_{}", color, style);
```

**4. Add SVG tests**
- Verify correct SVG output for each style
- Test hash uniqueness per style
- Test manifest tracking with styles

### Phase 3: Documentation

**1. Update TEMPLATE-SYNTAX.md**
- Document style parameter
- Show all 5 style examples
- Explain defaults

**2. Update README.md**
- Add style examples to Color Block section
- Show mixed-style compositions

**3. Update API-GUIDE.md**
- Rust API for style parameter
- Backend differences

**4. Update CHANGELOG.md**
- Feature: Badge style control
- Breaking: None (backward compatible)

---

## Backward Compatibility

**Default behavior:** If no style specified, use `flat-square` (current behavior).

This makes the feature **100% backward compatible**. All existing templates work unchanged:
```markdown
{{ui:swatch:F41C80/}}  
→ Still generates flat-square (no breaking change)
```

---

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_swatch_with_style() {
    let parser = TemplateParser::new()?;
    let input = "{{ui:swatch:F41C80:style=flat/}}";
    let output = parser.process(input)?;
    assert!(output.contains("style=flat"));
}

#[test]
fn test_swatch_default_style() {
    let parser = TemplateParser::new()?;
    let input = "{{ui:swatch:F41C80/}}";
    let output = parser.process(input)?;
    assert!(output.contains("style=flat-square"));  // Default
}
```

### Integration Tests
```rust
#[test]
fn test_all_shields_styles() {
    for style in ["flat", "flat-square", "for-the-badge", "plastic", "social"] {
        let input = format!("{{{{ui:swatch:F41C80:style={}/}}}}", style);
        let output = parser.process(&input)?;
        assert!(output.contains(&format!("style={}", style)));
    }
}
```

### SVG Output Tests
```rust
#[test]
fn test_svg_rounded_style() {
    let backend = SvgBackend::new("assets")?;
    let primitive = Primitive::Swatch { 
        color: "F41C80".to_string(),
        style: "flat".to_string(),
    };
    let svg = backend.render(&primitive)?;
    
    let svg_str = String::from_utf8(svg.bytes)?;
    assert!(svg_str.contains("rx=\"3\""));  // Rounded corners
}
```

---

## Estimated Effort

**Phase 1 (Shields.io):** ~4 hours
- Simple parameter passing
- URL generation already exists
- Just needs wiring

**Phase 2 (SVG):** ~8 hours  
- New SVG generation logic
- Gradient rendering
- Deterministic hashing updates

**Phase 3 (Documentation):** ~2 hours
- Update 4 doc files
- Add examples

**Total:** ~14 hours for complete implementation

---

## Priority Justification

**HIGH priority because:**

1. **User request:** "its very important... think like minecraft. we need different bricks"
2. **Design flexibility:** Currently limited to one visual style
3. **Low risk:** Backward compatible (defaults to current behavior)
4. **High value:** Unlocks creative design possibilities
5. **Infrastructure exists:** Shields.io already supports it, just not exposed

**Blockers:** None - can implement immediately after v1.0.0 release

---

## Alternative Considered: Custom Badge Width

Another dimension of control:
```markdown
{{ui:swatch:F41C80:width=80/}}  → Wider block
{{ui:swatch:F41C80:width=20/}}  → Narrower block
```

**Decision:** Defer to v1.2. Style control is more impactful first.

---

## Success Criteria

**Feature is complete when:**
1. Users can specify 5 shields.io styles via syntax
2. SVG backend generates matching visual styles
3. Default behavior unchanged (flat-square)
4. All tests passing (20+ new tests)
5. Documentation updated (4 files)
6. Example gallery shows mixed styles

**User validation:**
```markdown
{{ui:swatch:F41C80:style=flat/}} {{ui:swatch:EAB308:style=for-the-badge/}} {{ui:swatch:22C55E:style=plastic/}}
```
Should render three blocks with visibly different styles.

---

**Next Steps:** Add to ROADMAP.md under v1.1.0 High Priority
