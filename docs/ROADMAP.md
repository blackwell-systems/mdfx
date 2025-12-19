# mdfx Roadmap

Planned features for future versions.

---

## v1.1.0 - Discoverability & Caching

### CLI Discovery Commands ✅

Explore all available resources with `mdfx list`:

```bash
mdfx list                    # List styles (default)
mdfx list styles --samples   # List styles with samples
mdfx list components         # List all UI components
mdfx list glyphs             # List named glyphs
mdfx list frames             # List frame styles
mdfx list palette            # List palette colors
mdfx list glyphs -f star     # Filter by name pattern
```

### Enhanced Asset Management ✅

Content-addressed filenames, atomic writes, incremental manifests, and provenance tracking:

- **Content-addressed filenames**: SHA-256 based, stable across Rust versions
- **Atomic manifest writes**: Crash-safe with temp file + rename
- **Incremental updates**: `manifest.merge()` for efficient rebuilds
- **Provenance tracking**: Source files, timestamps, generator version
- **Manifest v1.1.0**: New schema with `total_size_bytes`, `generator_version`, per-asset metadata

See [ASSETS-GUIDE.md](guides/ASSETS-GUIDE.md) for full documentation.

---

## v1.2.0 - Tooling

### Inline SVG Mode

Embed SVGs as data URIs for single-file output:

```bash
mdfx process --backend svg-inline input.md
```

### Template Formatter

Normalize template formatting for team consistency:

```bash
mdfx fmt README.template.md
mdfx fmt --check README.template.md  # CI mode
```

### Strict Mode

Fail on warnings for CI enforcement:

```bash
mdfx process --strict input.md
```

---

## v1.3.0+ - Extensions

- Spacer primitive for layout control
- Rule primitive for lines
- BadgeGroup for consistent badge spacing
- **Template includes** - `{{include:_header.md/}}` to compose documents from partials
- **Conditional blocks** - `{{#if target=github}}GitHub-only content{{/if}}`

---

## Contributing

Feature requests: https://github.com/blackwell-systems/mdfx/issues
