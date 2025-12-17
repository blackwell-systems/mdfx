# unicode-decor Crate Extraction Plan

Extract reusable Unicode text styling and glyph functionality from mdfx into a standalone crate.

## Scope

### Included
- **24 text styles** - mathbold, italic, fullwidth, circled, fraktur, etc.
- **531 named glyphs** - arrows, box-drawing, stars, checkmarks, currency, etc.
- **VS15 handling** - internal, transparent to callers

### Excluded (stays in mdfx)
- Frames (prefix/suffix decorations)
- Frame combinators (`gradient+star`)
- Frame modifiers (reverse, count, separator, spacing)
- Template syntax (`{{mathbold}}...{{/}}`)
- Components (swatch, tech, rating, etc.)

## API Design

### Core Types

```rust
/// Text style variants
#[non_exhaustive]
pub enum Style {
    MathBold,
    MathItalic,
    MathBoldItalic,
    Fullwidth,
    SmallCaps,
    Circled,
    CircledNegative,
    Squared,
    SquaredNegative,
    Fraktur,
    FrakturBold,
    Script,
    ScriptBold,
    DoubleStruck,
    Monospace,
    Regional,
    Strikethrough,
    // ... etc
}

/// Error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownGlyph(pub String);
```

### Core Functions

```rust
/// Convert text to a Unicode style
pub fn style(text: &str, style: Style) -> String;

/// Look up a named glyph (with VS15 applied internally)
pub fn glyph(name: &str) -> Result<&'static str, UnknownGlyph>;

/// List all available glyph names (for discoverability/tests)
pub fn glyph_names() -> impl Iterator<Item = &'static str>;

/// List all available styles
pub fn style_names() -> impl Iterator<Item = Style>;
```

### Usage Examples

```rust
use unicode_decor::{style, glyph, Style};

// Text styling
let bold = style("Hello", Style::MathBold);      // 𝐇𝐞𝐥𝐥𝐨
let fancy = style("RUST", Style::Circled);       // ⓇⓊⓈⓉ
let wide = style("Hi", Style::Fullwidth);        // Ｈｉ

// Named glyphs
let star = glyph("star.filled")?;                // ★
let arrow = glyph("arrow.double.right")?;        // ⇒
let check = glyph("check")?;                     // ✓

// Discovery
for name in glyph_names().filter(|n| n.starts_with("arrow.")) {
    println!("{}: {}", name, glyph(name).unwrap());
}
```

## Design Decisions

### 1. Name Normalization

**Decision:** Normalize input names
- Lowercase
- Treat `-` as `_` (so `arrow-right` matches `arrow_right`)
- Keep `.` significant (hierarchical namespacing)

```rust
glyph("star.filled")     // canonical
glyph("Star.Filled")     // works (lowercased)
glyph("star-filled")     // fails (- not normalized to .)
```

### 2. VS15 Handling

**Decision:** Apply VS15 internally, transparently

- Callers never see or think about variation selectors
- `glyph()` returns characters with VS15 already appended
- Ensures consistent text rendering across platforms

### 3. Stability Promise

**Decision:** Append-only registry

- Existing glyph/style names NEVER change
- New glyphs/styles can be added in minor versions
- Safe to depend on without fear of breakage

### 4. Error Handling

**Decision:** `Result` for glyphs, infallible for styles

- `glyph(name)` returns `Result<&'static str, UnknownGlyph>` (runtime lookup)
- `style(text, Style)` is infallible (compile-time enum)

### 5. Data Storage

**Decision:** Compile-time static data

- Use `phf` (perfect hash function) for glyph lookup
- Or `match` + `const` for styles
- Zero runtime allocation for lookups

## File Structure

```
unicode-decor/
├── Cargo.toml
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── src/
│   ├── lib.rs          # Public API, re-exports
│   ├── style.rs        # Style enum + conversion logic
│   ├── glyph.rs        # Glyph lookup + VS15 handling
│   └── data/
│       ├── styles.rs   # Style character mappings (generated)
│       └── glyphs.rs   # Glyph registry (generated)
├── build.rs            # Optional: generate from JSON
└── tests/
    └── integration.rs
```

## Implementation Steps

### Phase 1: Setup (~1 hour)
- [ ] Create new crate: `cargo new unicode-decor --lib`
- [ ] Set up Cargo.toml (edition 2021, zero deps initially)
- [ ] Add dual MIT/Apache-2.0 license
- [ ] Create basic module structure

### Phase 2: Extract Styles (~2 hours)
- [ ] Define `Style` enum with all 24 variants
- [ ] Extract character mapping tables from mdfx registry
- [ ] Implement `style()` function
- [ ] Add `style_names()` iterator
- [ ] Port relevant tests from mdfx

### Phase 3: Extract Glyphs (~2 hours)
- [ ] Extract 531 glyphs from mdfx registry
- [ ] Decide on data format (phf, match, lazy_static)
- [ ] Implement name normalization
- [ ] Implement `glyph()` with VS15 handling
- [ ] Add `glyph_names()` iterator
- [ ] Port relevant tests from mdfx

### Phase 4: Polish (~1-2 hours)
- [ ] Write comprehensive README with examples
- [ ] Add doc comments to all public items
- [ ] Add category/keyword metadata for crates.io
- [ ] Run clippy, fix warnings
- [ ] Test on stable/beta/nightly

### Phase 5: Publish (~30 min)
- [ ] `cargo publish --dry-run`
- [ ] Verify crates.io rendering
- [ ] Publish v0.1.0
- [ ] Tag release in git

### Phase 6: Migrate mdfx (optional, later)
- [ ] Add `unicode-decor` as mdfx dependency
- [ ] Replace style conversion code with crate
- [ ] Replace glyph lookup with crate
- [ ] Remove duplicated data from mdfx registry
- [ ] Update mdfx tests

## Estimated Effort

| Phase | Time |
|-------|------|
| Setup | 1 hour |
| Styles | 2 hours |
| Glyphs | 2 hours |
| Polish | 1-2 hours |
| Publish | 30 min |
| **Total** | **6-8 hours** |

## Open Questions

1. **Crate name:** `unicode-decor`, `fancy-text`, `unibox`, `prettychars`?
2. **Glyph namespacing:** Keep current dot-notation (`arrow.double.right`) or flatten?
3. **Optional features:** Gate large glyph categories behind features to reduce binary size?
4. **MSRV:** 1.70? 1.75? Latest stable?

## Success Criteria

- [ ] Zero dependencies (or only `phf` for lookup)
- [ ] Compiles on stable Rust
- [ ] All 24 styles work correctly
- [ ] All 531 glyphs accessible
- [ ] VS15 applied transparently
- [ ] Comprehensive docs with examples
- [ ] Published to crates.io
