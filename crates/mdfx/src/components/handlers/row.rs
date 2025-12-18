//! Row layout component handler

use crate::components::{ComponentOutput, PostProcess};
use crate::error::Result;
use std::collections::HashMap;

/// Parse style hints from image alt text and convert to CSS
///
/// Supported hints:
/// - `ml-10` → `margin-left: -10px` (negative margin for left overlap)
/// - `mr-10` → `margin-right: -10px` (negative margin for right overlap)
///
/// These are used for chevron badge overlap effects.
fn parse_style_hints(alt: &str) -> String {
    let mut styles = Vec::new();

    if alt.contains("ml-10") {
        styles.push("margin-left: -10px");
    }
    if alt.contains("mr-10") {
        styles.push("margin-right: -10px");
    }

    styles.join("; ")
}

/// Handle row component expansion
pub fn handle(params: &HashMap<String, String>, content: Option<&str>) -> Result<ComponentOutput> {
    // Extract align parameter (default: center)
    let align = params
        .get("align")
        .cloned()
        .unwrap_or_else(|| "center".to_string());

    // Validate align value
    let align = match align.as_str() {
        "left" | "center" | "right" => align,
        _ => "center".to_string(),
    };

    // Content to be recursively parsed, then post-processed
    let template = content.unwrap_or("").to_string();

    Ok(ComponentOutput::TemplateDelayed {
        template,
        post_process: PostProcess::Row { align },
    })
}

/// Apply row formatting (wrap in HTML with alignment)
///
/// This is called AFTER recursive parsing to transform rendered content:
/// 1. Collapses whitespace/newlines to single spaces
/// 2. Converts markdown images `![alt](url)` to HTML `<img alt="alt" src="url">`
/// 3. Wraps with `<p align="...">...</p>`
///
/// This is necessary because GitHub Flavored Markdown doesn't parse
/// markdown syntax inside HTML blocks.
pub fn apply_row(content: &str, align: &str) -> String {
    // Step 1: Collapse whitespace/newlines to single spaces
    let collapsed: String = content.split_whitespace().collect::<Vec<_>>().join(" ");

    // Step 2: Convert markdown images to HTML img tags
    // Pattern: ![alt](url) or ![](url)
    let mut result = String::new();
    let mut remaining = collapsed.as_str();

    while let Some(start) = remaining.find("![") {
        // Add text before the image
        result.push_str(&remaining[..start]);

        let after_bang = &remaining[start + 2..];

        // Find closing ] for alt text
        if let Some(alt_end) = after_bang.find(']') {
            let alt = &after_bang[..alt_end];
            let after_alt = &after_bang[alt_end + 1..];

            // Expect ( after ]
            if let Some(after_paren) = after_alt.strip_prefix('(') {
                // Find closing )
                if let Some(url_end) = after_paren.find(')') {
                    let url = &after_paren[..url_end];

                    // Parse style hints from alt text (e.g., "ml-10 mr-10")
                    let style = parse_style_hints(alt);
                    let style_attr = if style.is_empty() {
                        String::new()
                    } else {
                        format!(r#" style="{}""#, style)
                    };

                    // Clean alt (without style hints) for accessibility
                    let clean_alt = alt
                        .replace("ml-10", "")
                        .replace("mr-10", "")
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");

                    // Convert to HTML img tag, with styles if present
                    result.push_str(&format!(
                        r#"<img alt="{}"{} src="{}">"#,
                        clean_alt, style_attr, url
                    ));
                    remaining = &after_paren[url_end + 1..];
                    continue;
                }
            }
        }

        // Malformed image syntax, keep as-is
        result.push_str("![");
        remaining = after_bang;
    }

    // Add any remaining text
    result.push_str(remaining);

    // Step 3: Wrap with alignment
    format!(
        r#"<p align="{}">
{}
</p>"#,
        align,
        result.trim()
    )
}
