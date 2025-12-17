# mdfx Documentation

A markdown compiler that transforms template syntax into styled output with Unicode typography, SVG components, and decorative frames.

## Quick Start

```bash
cargo install mdfx-cli
```

Create `input.md`:
```markdown
# {{frame:gradient}}{{mathbold}}PROJECT{{/mathbold}}{{/frame}}

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
- **30+ Frames** - Decorative borders, glyph frames, modifiers
- **739 Glyphs** - Symbols, arrows, chess pieces, keyboard keys
- **Data Visualization** - Progress bars, donuts, gauges, sparklines
- **5 Targets** - GitHub, GitLab, npm, PyPI, local docs
- **LSP Server** - IDE autocompletion for VS Code

## Guides

### UI Components

| Guide | Description |
|-------|-------------|
| [Components](guides/COMPONENTS-GUIDE.md) | Overview of all UI components |
| [Swatches](guides/SWATCH-GUIDE.md) | Color blocks for palettes |
| [Progress](guides/PROGRESS-GUIDE.md) | Progress bars and sliders |
| [Donut & Gauge](guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](guides/SPARKLINE-GUIDE.md) | Inline data visualization |
| [Data Viz](guides/DATA-VIZ-GUIDE.md) | Complete data visualization guide |

### Text & Symbols

| Guide | Description |
|-------|-------------|
| [Text Styles](guides/TEXT-STYLES-GUIDE.md) | 24 Unicode typography styles |
| [Frames](guides/FRAMES-GUIDE.md) | Decorative Unicode borders |
| [Glyphs](guides/GLYPHS-GUIDE.md) | 739 Unicode symbols |

### Tools & Integration

| Guide | Description |
|-------|-------------|
| [CLI Guide](guides/CLI-GUIDE.md) | Command-line interface |
| [LSP Guide](guides/LSP-GUIDE.md) | Language server for IDEs |

## Reference

- [Template Syntax](TEMPLATE-SYNTAX.md) - Full syntax specification
- [API Guide](API-GUIDE.md) - Library usage
- [Architecture](ARCHITECTURE.md) - System design
- [Targets](TARGETS.md) - Multi-target rendering

## CLI Commands

```bash
mdfx convert --style mathbold "TEXT"   # Convert text
mdfx list                               # List styles
mdfx process input.md -o output.md     # Process file
mdfx process --backend svg input.md    # Local SVG files
mdfx build input.md --all-targets      # Multi-target build
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
