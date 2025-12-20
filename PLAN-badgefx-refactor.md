# badgefx Refactor: Consolidate All Badge Types

## Goal

Move version and license badge functionality from `mdfx` to `badgefx`, making badgefx the single source for all badge generation.

## Current State

```
badgefx/
  - TechBadge, BadgeBuilder (tech badges only)
  - render.rs (SVG rendering for tech badges)
  - style.rs (BadgeStyle, Corners, Border, Chevron)

mdfx/
  - Primitive::Swatch (generic colored box with label)
  - handlers/version.rs (status detection, uses Swatch)
  - handlers/license.rs (category detection, uses Swatch)
  - renderer/svg/swatch.rs (renders Swatch)
```

## Target State

```
badgefx/
  - TechBadge, TechBuilder (tech badges)
  - VersionBadge, VersionBuilder (version badges)
  - LicenseBadge, LicenseBuilder (license badges)
  - Common: BadgeStyle, Corners, Border
  - render.rs (unified SVG rendering)

mdfx/
  - handlers/version.rs → calls badgefx::version()
  - handlers/license.rs → calls badgefx::license()
  - Remove Primitive::Swatch for badges (keep for non-badge swatches)
```

## New badgefx API

```rust
use badgefx::{badge, version, license, BadgeStyle};

// Tech badges (existing API preserved)
let svg = badge("rust").render();
let svg = badge("typescript")
    .label("v5.0")
    .style(BadgeStyle::Flat)
    .render();

// Version badges (NEW)
let svg = version("1.0.0").render();           // green (stable)
let svg = version("2.0.0-beta").render();      // yellow (auto-detected)
let svg = version("0.5.0").render();           // yellow (0.x = beta)
let svg = version("1.0.0")
    .status(Status::Deprecated)                // override auto-detection
    .style(BadgeStyle::FlatSquare)
    .render();

// License badges (NEW)
let svg = license("MIT").render();             // green (permissive)
let svg = license("GPL-3.0").render();         // yellow (copyleft)
let svg = license("Apache-2.0")
    .style(BadgeStyle::Flat)
    .bg_color("#custom")
    .render();
```

## Badge Types Share Common Traits

All badges support:
- `.style(BadgeStyle)` - flat, flat-square, plastic, etc.
- `.bg_color(color)` - override background
- `.text_color(color)` - override text
- `.border(color, width)` - add border
- `.corners(Corners)` - corner radius control
- `.render()` -> String (SVG)
- `.render_to_file(path)` -> io::Result<()>

Type-specific:
- TechBadge: `.label()`, `.logo_color()`, `.chevron()`, `.raised()`, `.divider()`
- VersionBadge: `.status()`, `.prefix()` (v, V, or custom)
- LicenseBadge: `.category()` (override auto-detection)

## Implementation Steps

### Phase 1: Add Version/License to badgefx

1. **Create `badgefx/src/version.rs`**
   - Move `VersionStatus` enum from mdfx
   - Move `detect_status()` function
   - Create `VersionBadge` struct + `VersionBuilder`
   - Implement rendering (reuse tech badge SVG infrastructure)

2. **Create `badgefx/src/license.rs`**
   - Move `LicenseCategory` enum from mdfx
   - Move `categorize_license()` function
   - Create `LicenseBadge` struct + `LicenseBuilder`
   - Implement rendering

3. **Update `badgefx/src/lib.rs`**
   - Export new modules
   - Add convenience functions: `version()`, `license()`

4. **Update `badgefx/src/render.rs`**
   - Generalize to handle all badge types
   - Extract shared rendering logic

### Phase 2: Simplify mdfx Integration

5. **Update `mdfx/src/components/handlers/version.rs`**
   - Remove local status detection (now in badgefx)
   - Return `Primitive::Version(VersionConfig)` or call badgefx directly

6. **Update `mdfx/src/components/handlers/license.rs`**
   - Remove local category detection (now in badgefx)
   - Return `Primitive::License(LicenseConfig)` or call badgefx directly

7. **Update `mdfx/src/primitive.rs`**
   - Add `Primitive::Version(VersionConfig)`
   - Add `Primitive::License(LicenseConfig)`
   - Or: remove these and have handlers return SVG directly

8. **Update `mdfx/src/renderer/svg/`**
   - Add version/license renderers that call badgefx
   - Or: handlers return pre-rendered SVG

### Phase 3: Clean Up

9. **Remove duplication**
   - Delete status/category detection from mdfx
   - Ensure single source of truth in badgefx

10. **Update tests**
    - Move version/license unit tests to badgefx
    - Update mdfx integration tests

11. **Update documentation**
    - badgefx README with all badge types
    - LSP completions (already complete)

## Design Decisions

### Q1: Primitive vs Direct SVG?

**Option A**: Keep Primitives
```rust
// mdfx handler returns
Ok(ComponentOutput::Primitive(Primitive::Version { ... }))
// mdfx renderer calls badgefx
```

**Option B**: Direct SVG
```rust
// mdfx handler calls badgefx directly
let svg = badgefx::version(&version_str).render();
Ok(ComponentOutput::Text(svg))
```

**Decision**: Option A (primitives) for consistency with other components, but primitives call badgefx.

### Q2: Shared Trait or Separate Structs?

**Option A**: Trait-based
```rust
trait Badge {
    fn render(&self) -> String;
}
impl Badge for TechBadge { ... }
impl Badge for VersionBadge { ... }
```

**Option B**: Separate builders (current approach)
```rust
// Each has its own builder, no shared trait needed
badge("rust").render()
version("1.0").render()
```

**Decision**: Option B - separate builders are simpler and more idiomatic Rust.

### Q3: Style Sharing

Version and license badges should support the same styles as tech badges:
- flat, flat-square, plastic, for-the-badge, social

All use the same gradient/shadow rendering from `style.rs`.

## Files Changed

### badgefx (additions)
- `src/version.rs` (new)
- `src/license.rs` (new)
- `src/lib.rs` (update exports)
- `src/render.rs` (generalize)

### mdfx (modifications)
- `src/components/handlers/version.rs` (simplify)
- `src/components/handlers/license.rs` (simplify)
- `src/primitive.rs` (add Version/License variants)
- `src/renderer/svg/mod.rs` (add renderers)

### Tests
- badgefx: add version/license snapshot tests
- mdfx: update integration tests

## Breaking Changes

Since there are no users:
- Rename `badge()` to `tech()` for consistency? Or keep as `badge()`?
- Simplify `TechBadge` if needed
- Any API cleanup desired

## Estimated Scope

- ~400 lines moved from mdfx to badgefx
- ~200 lines new rendering code
- ~100 lines test updates
- Total: ~700 lines changed
