# mdfx Documentation

A markdown compiler that transforms template syntax into styled output with Unicode typography, SVG components, and decorative frames.

## Quick Start

```bash
cargo install mdfx-cli
```

Create `input.md`:
```markdown
# {{fr:gradient}}{{mathbold}}PROJECT{{/mathbold}}{{/fr}}

{{ui:progress:75:width=200/}}
{{ui:tech:rust/}} {{ui:tech:python/}}
```

Process:
```bash
mdfx process input.md -o output.md
mdfx process input.md -o output.md --backend svg --assets-dir assets/
```

## Features

- **24 Unicode Styles** - mathbold, script, fraktur, strikethrough, and more
- **29 Frames** - Decorative borders, glyph frames, modifiers
- **531 Glyphs** - Symbols, arrows, chess pieces, keyboard keys
- **10 Components** - Tech badges, progress bars, donuts, sparklines, waveforms
- **5 Targets** - GitHub, GitLab, npm, PyPI, local docs
- **LSP Server** - IDE autocompletion with contextual suggestions

## Guides

### Visual Components

| Guide | Description |
|-------|-------------|
| [Tech Badges](guides/TECH-GUIDE.md) | 90+ technology logos with brand colors |
| [Swatches](guides/SWATCH-GUIDE.md) | Color blocks for palettes |
| [Progress](guides/PROGRESS-GUIDE.md) | Progress bars and sliders |
| [Donut & Gauge](guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](guides/SPARKLINE-GUIDE.md) | Inline line/bar charts |
| [Waveforms](guides/WAVEFORM-GUIDE.md) | Audio-style visualizations |
| [Ratings](guides/DATA-VIZ-GUIDE.md#rating) | Star/heart ratings |
| [Components](guides/COMPONENTS-GUIDE.md) | Overview of all components |

### Text & Symbols

| Guide | Description |
|-------|-------------|
| [Text Styles](guides/TEXT-STYLES-GUIDE.md) | 24 Unicode typography styles |
| [Frames](guides/FRAMES-GUIDE.md) | Decorative Unicode borders |
| [Glyphs](guides/GLYPHS-GUIDE.md) | 531 Unicode symbols |
| [Colors](guides/COLORS-GUIDE.md) | Palette system and theming |

### Tools & Integration

| Guide | Description |
|-------|-------------|
| [CLI Guide](guides/CLI-GUIDE.md) | Command-line interface |
| [LSP Guide](guides/LSP-GUIDE.md) | Language server for IDEs |
| [Asset Management](guides/ASSETS-GUIDE.md) | SVG generation and manifests |

## Reference

- [Template Syntax](TEMPLATE-SYNTAX.md) - Full syntax specification
- [API Guide](API-GUIDE.md) - Library usage
- [Architecture](ARCHITECTURE.md) - System design
- [Targets](TARGETS.md) - Multi-target rendering

## CLI Commands

```bash
mdfx convert --style mathbold "TEXT"      # Convert text
mdfx list                                  # List styles
mdfx list components                       # List UI components
mdfx list glyphs -f star                   # Search glyphs
mdfx process input.md -o output.md         # Process file
mdfx process --backend svg input.md        # Local SVG files
mdfx build input.md --all-targets          # Multi-target build
mdfx verify assets/manifest.json           # Verify assets
mdfx clean --scan "*.md" assets            # Clean stale assets
```

## Library

```rust
use mdfx::{Converter, TemplateParser};

let converter = Converter::new()?;
let bold = converter.convert("HELLO", "mathbold")?;
// "𝐇𝐄𝐋𝐋𝐎"

let parser = TemplateParser::new()?;
let output = parser.process("{{mathbold}}TITLE{{/mathbold}}")?;
// "𝐓𝐈𝐓𝐋𝐄"
```

## Links

- [Examples](https://github.com/blackwell-systems/mdfx/tree/main/examples)
- [GitHub](https://github.com/blackwell-systems/mdfx)
- [Changelog](https://github.com/blackwell-systems/mdfx/blob/main/CHANGELOG.md)
