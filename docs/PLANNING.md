# mdfx - Project Planning Document

**Current Version:** 1.0.0
**Status:** Production Release - Multi-Backend Architecture Shipped
**Last Updated:** 2025-12-13

---

## Project Overview

**mdfx** is a Unicode text styling tool for markdown that uses a component-first architecture. Users write semantic `{{ui:*}}` elements that compile down to shields.io badges, decorative frames, and Unicode character transformations.

### Core Value Proposition

- **Component-first authoring** - Semantic templates (`{{ui:header}}`, not verbose primitives)
- **Design token system** - Named colors (`accent`, `success`) for consistency
- **Markdown preprocessing** - Process templates before rendering
- **CLI + Library** - Use standalone or embed in other tools
- **Extensible** - JSON-defined components (no recompilation needed)

### Target Users

1. **README authors** - Create distinctive project headers and tech stack badges
2. **Documentation writers** - Semantic section markers and callouts
3. **Status dashboards** - Visual system health indicators
4. **Markdown tool developers** - Embed mdfx in editors and generators

---

## v1.0.0 - Multi-Backend Architecture (Current)

**Released:** 2025-12-13

### Shipped Features

**UI Components (6 types):**
- `divider` - Themed color bars for section separation
- `swatch` - Single color blocks
- `tech` - Technology logo badges (2000+ via Simple Icons)
- `status` - Colored status indicators
- `header` - Gradient-framed bold headers
- `callout` - Framed messages with indicators

**Primitives (3 types):**
- `shields` - shields.io badge URL generation (4 primitives: block, twotone, bar, icon)
- `frames` - Decorative prefix/suffix (27 styles)
- `badges` - Enclosed alphanumerics (6 types: ①②③, ⒜⒝⒞)

**Text Styles (19 types):**
- mathbold, fullwidth, negative-squared, negative-circled, squared-latin, circled-latin
- small-caps, monospace, double-struck, sans-serif, sans-serif-bold, sans-serif-italic, sans-serif-bold-italic
- italic, bold-italic, script, bold-script, fraktur, bold-fraktur

**Design System:**
- 15 design tokens (palette.json)
- Component expansion model (data-driven, no code)
- Self-closing tags (`{{ui:divider/}}`)
- Generic closers (`{{/ui}}`)

**Infrastructure:**
- 152 tests passing
- CLI with process, convert, list commands
- Rust library API (ComponentsRenderer, ShieldsRenderer, Converter, FrameRenderer, BadgeRenderer)
- Embedded JSON data (~22KB)
- Code block preservation
- Stdin/stdout support

### Template Syntax (v1.0.0)

**UI Components (primary):**
```markdown
{{ui:divider/}}                           ← Self-closing
{{ui:tech:rust/}}                         ← With arg
{{ui:header}}TITLE{{/ui}}                 ← Block (generic closer)
{{ui:callout:warning}}Message{{/ui}}      ← Block with arg
```

**Primitives (escape hatch):**
```markdown
{{shields:block:color=accent:style=flat-square/}}
{{frame:gradient}}TEXT{{/frame}}
{{badge:circle}}1{{/badge}}
```

**Styles (character transformation):**
```markdown
{{mathbold}}TEXT{{/mathbold}}
{{mathbold:separator=dot}}TITLE{{/mathbold}}
{{mathbold:spacing=2}}SPACED{{/mathbold}}
```

### Architecture Highlights

**Three-layer system:**
1. UI components (user writes) → Expand to primitives
2. Primitives (rendering engines) → Generate output
3. Styles (character transforms) → Unicode mappings

**Expansion model:**
- `{{ui:header}}` expands to `{{frame:gradient}}{{mathbold:separator=dot}}$content{{/mathbold}}{{/frame}}`
- Recursive parsing handles nested templates
- Palette colors resolved at expansion time

**Parser priority:**
UI → Frame → Badge → Shields → Style

### Breaking Changes from Earlier Versions

**v1.0.0 introduces:**
- Component-first API (old `{{mathbold}}` syntax still works)
- New namespace: `{{ui:*}}` for components
- Self-closing syntax (`/}}`)
- Generic `{{/ui}}` closer (not `{{/ui:header}}`)
- Design tokens (palette.json)

**Backward compatibility:** Not a concern (started today)

---

## Future Version - User Extensibility (Planned)

**Target:** Q1 2026

### Planned Features

**User-provided components:**
- Load `./components.json` from project directory
- Merge with built-in components
- User components override built-in
- No recompilation required

**User-provided palette:**
- Load `./palette.json` from project directory
- Project-specific design tokens
- Per-project branding

**Native components:**
- `progress` - Progress bars with logic (`{{ui:progress:75/}}`)
- Rust-implemented handlers for complex components
- Mixed expand/native types in components.json

**Additional UI components:**
- `gradient-text` - Gradient-colored text spans
- `badge-inline` - Inline text badges
- `timeline` - Event timelines
- `comparison` - Before/after comparisons

**SVG Backend:**
- Local SVG file generation (alternative to shields.io)
- Hash-based filenames for deterministic builds
- Support for offline documentation
- Phase 1: swatch/divider/status (solid colors)
- Phase 2: tech badges (requires logo bundling)

### Technical Implementation

**SVG Backend:**
```rust
// src/renderer/svg.rs
pub struct SvgBackend {
    assets_dir: PathBuf,
    cache: HashMap<String, String>,  // hash → filename
}

impl Renderer for SvgBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let hash = compute_hash(primitive);
        let filename = format!("{}_{}.svg", primitive_type, &hash[..8]);
        let path = self.assets_dir.join(&filename);

        if !path.exists() {
            let svg = generate_svg(primitive)?;
            fs::write(&path, svg)?;
        }

        Ok(RenderedAsset::FileAsset {
            path: format!("assets/{}", filename),
            markdown: format!("![](assets/{})", filename),
        })
    }
}
```

**CLI Usage:**
```bash
mdfx process --backend svg --assets-dir ./docs/ui input.md
```

**Project-local config loading:**
```rust
// 1. Load embedded data/components.json
// 2. Check for ./components.json
// 3. Merge (user overrides built-in)
```

**Native component example:**
```json
{
  "progress": {
    "type": "native",
    "handler": "progress_bar"
  }
}
```

```rust
fn progress_bar(args: &[String]) -> Result<String> {
    let value = args[0].parse::<f32>()?;
    let filled = (value / 100.0 * 5.0).round() as usize;
    let empty = 5 - filled;
    let colors = vec!["success"; filled]
        .into_iter()
        .chain(vec!["slate"; empty])
        .collect();
    // Return shields:bar with computed colors
}
```

---

## Future Version - Tooling & Integrations (Planned)

**Target:** Q2 2026

### Planned Features

**VS Code Extension:**
- Inline preview of rendered templates
- Component snippets
- Design token autocomplete
- Live reload on save

**Watch Mode:**
```bash
mdfx watch README.template.md
# Auto-regenerates README.md on changes
```

**Component Gallery:**
- Web UI showcasing components
- Copy/paste snippets
- User-submitted components
- npm/crates.io package manager integration

**Preset Collections:**
- "Tech Stack" - Common tech logos
- "Status Dashboard" - Health indicators
- "Release Notes" - Callouts and badges
- "Documentation" - Headers and dividers

### Technical Implementation

**VS Code extension:**
- Language server for template validation
- Webview for live preview
- Built on WASM bindings

**Watch mode:**
```rust
// Use notify crate for file watching
// Debounce changes (500ms)
// Re-process on template file change
```

**Gallery:**
- Static site with component demos
- JSON API for component discovery
- Community submissions via PR

---

## Future Version - Advanced Composition (Planned)

**Target:** Q3 2026

### Planned Features

**Multiline components:**
```markdown
{{ui:table}}
| Name | Value |
| A    | 1     |
| B    | 2     |
{{/ui}}
```

**Conditional rendering:**
```markdown
{{ui:status:{{env:CI_STATUS}}/}}  ← env var substitution
```

**Component nesting rules:**
- Document safe nesting patterns
- Add validation warnings
- Prevent common mistakes

**Advanced shields:**
- Custom shields.io styles
- Dynamic badge content
- Badge composition

### Technical Challenges

**Multiline parsing:**
- Current parser is line-based
- Need block-level parsing
- Preserve Markdown structure

**Variable substitution:**
- Security: sanitize env vars
- Scope: env, config, computed
- Syntax: `{{var:name}}` or `${name}`?

---

## v1.0.0 - Production Ready (Target: Q4 2026)

### Stability Criteria

**API stability:**
- Template syntax finalized (no breaking changes)
- Rust library API stable (semantic versioning)
- Component schema v1.0 locked

**Performance:**
- <100ms for typical README processing
- <1s for 1000+ line documents
- <10MB memory usage

**Quality:**
- 95%+ test coverage
- All examples working
- Documentation complete
- Security audit passed

**Ecosystem:**
- VS Code extension published
- 20+ community-contributed components
- Integration guides (Hugo, Jekyll, MkDocs)
- 100+ GitHub stars

### Breaking Changes Policy (Post-1.0)

**Will never break:**
- UI component syntax (`{{ui:*}}`)
- Primitive syntax (`{{shields:*}}`, `{{frame:*}}`, `{{badge:*}}`)
- Style syntax (`{{mathbold}}`)
- Design token names

**May evolve (with deprecation warnings):**
- Built-in component implementations
- Palette colors (add new, deprecate old)
- CLI flags (add new, keep old)

**Can change freely:**
- Internal Rust API (not public)
- JSON schema versions (forward-compatible)
- Rendering output (visual improvements)

---

## Long-Term Vision (2027+)

### WASM Bindings

**Use cases:**
- Browser-based markdown editors
- Real-time preview in web apps
- Cloudflare Workers / edge functions

**API:**
```javascript
import init, { process_template } from 'mdfx';
await init();
const result = process_template('{{ui:header}}TITLE{{/ui}}');
```

### Component Marketplace

**Features:**
- Discover components by category
- Install via CLI: `mdfx install awesome-header`
- Rate and review components
- Automatic updates

**Moderation:**
- Security review for native components
- Expand-only components auto-approved
- Community voting

### AI-Powered Component Generation

**Concept:**
```bash
mdfx generate "Create a component for quarterly earnings with green/red indicators"
# Generates component definition
# User reviews and saves to components.json
```

**Implementation:**
- LLM prompts for component generation
- Validation of generated JSON
- Safety checks (no code execution)

### Multi-Language Support

**Expand beyond Markdown:**
- HTML templates (`<mdfx-header>TITLE</mdfx-header>`)
- LaTeX (`\mdfxHeader{TITLE}`)
- reStructuredText
- AsciiDoc

**Technical:**
- Plugin architecture for parsers
- Shared rendering core
- Per-format output adapters

---

## Technical Roadmap

### Current Architecture (v1.0.0)

**5 Core Components:**
1. ComponentsRenderer - Expand UI to primitives
2. ShieldsRenderer - Generate shields.io URLs
3. FrameRenderer - Add prefix/suffix decorations
4. BadgeRenderer - Enclosed alphanumerics
5. Converter - Character transformations

**Data Files (embedded):**
- components.json (6 components)
- palette.json (15 colors)
- shields.json (4 styles + palette)
- frames.json (27 frames)
- badges.json (6 types)
- styles.json (19 styles)

**Total:** ~22KB embedded JSON

### Planned Improvements

**Future Version - Config loading:**
```
1. Load embedded JSON (defaults)
2. Check ./mdfx/components.json
3. Check ./components.json
4. Merge (user > project > defaults)
```

**Future Version - Caching:**
```rust
// Cache parsed templates in memory
// LRU eviction for long-running processes
// Invalidate on file change
```

**Future Version - Parallel processing:**
```rust
// Process multiple files concurrently
// Shared renderer instances (Arc<Renderer>)
// Rayon for parallel template parsing
```

---

## Success Metrics

### v1.0.0 (Current)

- ✓ 152 tests passing
- ✓ 6 UI components shipped
- ✓ 4,000+ lines of documentation
- ⏳ GitHub release published
- ⏳ crates.io publish
- ⏳ Initial community feedback

### Future Version Targets

- 20+ total components (6 built-in + community)
- 200+ tests
- 5+ production users
- 50+ GitHub stars
- 10+ community PRs

### v1.0.0 Targets

- 100+ components (marketplace)
- 500+ tests
- 100+ production users
- 500+ GitHub stars
- VS Code extension: 1,000+ installs
- Featured in "Awesome Markdown" lists

---

## Open Questions

### Template Syntax

**Q: Should we support shorthand for common patterns?**
```markdown
# Option A: Current (explicit)
{{ui:header}}TITLE{{/ui}}

# Option B: Shorthand (proposed)
{{h1}}TITLE{{/h1}}
```
**Decision:** Keep explicit for now. Shorthand can be aliases in components.json.

**Q: Parameterized components?**
```markdown
{{ui:header:color=cobalt:style=bold}}TITLE{{/ui}}
```
**Decision:** v0.2+. Need to design param override system.

### Design Tokens

**Q: Support theming (light/dark)?**
```json
{
  "themes": {
    "light": { "ui.bg": "F5F5F5" },
    "dark": { "ui.bg": "292A2D" }
  }
}
```
**Decision:** v0.3+. Requires theme switching API.

**Q: Gradient support?**
```markdown
{{ui:swatch:gradient(accent,cobalt)/}}
```
**Decision:** Maybe v0.4+. Complex to implement in shields.io.

### Performance

**Q: Cache rendered output?**
- Pro: Faster regeneration
- Con: Invalidation complexity
- **Decision:** v0.3+ (watch mode needs this)

**Q: Lazy-load JSON data?**
- Pro: Faster startup
- Con: Complexity, embedded data already cheap
- **Decision:** No. Embedded JSON is <25KB, fine to load eagerly.

---

## Community & Contribution

### How to Contribute (v1.0.0)

**Component ideas:**
- Open issue with use case
- Show example desired syntax
- Explain expansion to primitives

**Bug reports:**
- Minimal reproduction
- Expected vs actual output
- mdfx version

**Documentation:**
- Typo fixes always welcome
- New examples encouraged
- Tutorial improvements

### Future Contribution Paths (v0.2+)

**Component Marketplace:**
- Submit components via PR
- Include demo and description
- Follow component schema

**Translations:**
- Translate error messages
- Localize documentation
- Multi-language examples

---

## References

- **Architecture:** [ARCHITECTURE.md](ARCHITECTURE.md)
- **Components Design:** [COMPONENTS.md](COMPONENTS.md)
- **API Guide:** [API-GUIDE.md](API-GUIDE.md)
- **Examples:** [../examples/README.md](../examples/README.md)

---

**Document Status:** Reflects v1.0.0 reality + roadmap through v1.0.0 and beyond
