# {{mathbold}}UNICODE STYLING DEMO{{/mathbold}}

This demonstrates the {{negative-squared}}UTF8FX{{/negative-squared}} template processor in action.

## {{italic}}Available Styles{{/italic}}

### Bold Styles

- {{mathbold}}Mathematical Bold{{/mathbold}} - Professional and clean
- {{sans-serif-bold}}Sans-Serif Bold{{/sans-serif-bold}} - Modern and strong
- {{fullwidth}}Full-Width{{/fullwidth}} - Substantial emphasis

### Boxed Styles

- {{negative-squared}}NEGATIVE SQUARED{{/negative-squared}} - Maximum contrast
- {{negative-circled}}NEGATIVE CIRCLED{{/negative-circled}} - Bold and rounded
- {{squared-latin}}SQUARED LATIN{{/squared-latin}} - Elegant boxes

### Technical Styles

- {{monospace}}Monospace{{/monospace}} - Code-like appearance
- {{double-struck}}Double-Struck{{/double-struck}} - Outline style

### Elegant Styles

- {{small-caps}}small caps{{/small-caps}} - Subtle and refined
- {{italic}}Italic{{/italic}} - Flowing emphasis
- {{bold-italic}}Bold Italic{{/bold-italic}} - Strong and flowing

## Code Preservation

The parser intelligently preserves code blocks:

```bash
# This template is NOT processed
echo "{{mathbold}}Hello{{/mathbold}}"
```

And inline `{{mathbold}}code{{/mathbold}}` is also preserved!

## Real-World Example

Create distinctive headers:

### {{mathbold}}IMPORTANT SECTION{{/mathbold}}

Use {{negative-squared}}WARNING{{/negative-squared}} for alerts.

Mix styles: {{mathbold}}Bold{{/mathbold}} and {{italic}}italic{{/italic}} work together.

## Usage

```bash
# Process to stdout
utf8fx process input.md

# Save to new file
utf8fx process input.md -o output.md

# Modify in-place
utf8fx process input.md --in-place
```
