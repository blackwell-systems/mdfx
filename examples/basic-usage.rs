/// Basic usage examples for utf8fx library
use utf8fx::{Converter, TemplateParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a converter
    let converter = Converter::new()?;

    // Convert text directly
    println!("=== Direct Conversion ===");
    let result = converter.convert("HELLO WORLD", "mathbold")?;
    println!("Mathematical Bold: {}", result);

    let result = converter.convert("WARNING", "negative-squared")?;
    println!("Negative Squared: {}", result);

    let result = converter.convert("blackdot v4.0.0", "fullwidth")?;
    println!("Full-Width: {}", result);

    // Use aliases
    println!("\n=== Using Aliases ===");
    let result = converter.convert("Test", "mb")?; // mathbold
    println!("Alias 'mb': {}", result);

    let result = converter.convert("Code", "mono")?; // monospace
    println!("Alias 'mono': {}", result);

    // List available styles
    println!("\n=== Available Styles ===");
    for style in converter.list_styles() {
        println!("{}: {}", style.id, style.name);
    }

    // Process templates
    println!("\n=== Template Processing ===");
    let parser = TemplateParser::new()?;

    let markdown = "# {{mathbold}}TITLE{{/mathbold}}\n\nThis is {{italic}}emphasized{{/italic}}.";
    let processed = parser.process(markdown)?;
    println!("{}", processed);

    // Preserves code blocks
    println!("\n=== Code Block Preservation ===");
    let markdown_with_code = r#"Text {{mathbold}}styled{{/mathbold}}

```rust
let x = "{{mathbold}}not styled{{/mathbold}}";
```

Back to {{mathbold}}styled{{/mathbold}} text."#;

    let processed = parser.process(markdown_with_code)?;
    println!("{}", processed);

    Ok(())
}
