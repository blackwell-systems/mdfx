//! Inlay hints for mdfx templates
//!
//! Provides live inline color swatches that update as you type.

use crate::lsp::parser::find_templates;
use crate::lsp::preview::resolve_color_hex;
use crate::lsp::semantic_tokens::is_color_param;
use std::collections::HashMap;
use tower_lsp::lsp_types::*;

/// Generate inlay hints for a document
/// Shows color swatches next to color parameter values
pub fn generate_inlay_hints(
    text: &str,
    palette: &HashMap<String, String>,
    range: &Range,
) -> Vec<InlayHint> {
    let mut hints = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        let line_num = line_num as u32;

        // Skip lines outside the requested range
        if line_num < range.start.line || line_num > range.end.line {
            continue;
        }

        // Find all templates in this line
        for (_start, _is_closing, _is_self_closing, _is_malformed, content, _end) in
            find_templates(line)
        {
            // Look for color parameters in the template content
            // Format: param=value or param=value:next
            for part in content.split(':') {
                if let Some((param, value)) = part.split_once('=') {
                    if is_color_param(param) {
                        // Calculate position after the value
                        if let Some(param_start) = line.find(part) {
                            let value_end = param_start + part.len();

                            // Resolve the color (palette name or hex)
                            let hex = resolve_color_hex(value, palette);

                            // Only show hint for valid-looking colors
                            if is_valid_hex(&hex) {
                                hints.push(InlayHint {
                                    position: Position {
                                        line: line_num,
                                        character: value_end as u32,
                                    },
                                    label: InlayHintLabel::String(" ■".to_string()),
                                    kind: Some(InlayHintKind::TYPE),
                                    text_edits: None,
                                    tooltip: Some(InlayHintTooltip::String(format!("#{}", hex))),
                                    padding_left: Some(false),
                                    padding_right: Some(true),
                                    data: Some(serde_json::json!({ "color": hex })),
                                });
                            }
                        }
                    }
                }
            }
        }

        // Also handle standalone swatch: templates
        if let Some(swatch_start) = line.find("swatch:") {
            let after_swatch = &line[swatch_start + 7..];
            // Extract color until : or / or }}
            let color_end = after_swatch
                .find([':', '/', '}'])
                .unwrap_or(after_swatch.len());
            let color = &after_swatch[..color_end];

            if !color.is_empty() {
                let hex = resolve_color_hex(color, palette);
                if is_valid_hex(&hex) {
                    let pos = swatch_start + 7 + color_end;
                    hints.push(InlayHint {
                        position: Position {
                            line: line_num,
                            character: pos as u32,
                        },
                        label: InlayHintLabel::String(" ■".to_string()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String(format!("#{}", hex))),
                        padding_left: Some(false),
                        padding_right: Some(true),
                        data: Some(serde_json::json!({ "color": hex })),
                    });
                }
            }
        }
    }

    hints
}

/// Check if a string looks like a valid hex color (3 or 6 hex chars)
fn is_valid_hex(s: &str) -> bool {
    let len = s.len();
    (len == 3 || len == 6) && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_hex() {
        assert!(is_valid_hex("FF0000"));
        assert!(is_valid_hex("abc"));
        assert!(is_valid_hex("123456"));
        assert!(!is_valid_hex("GGGGGG"));
        assert!(!is_valid_hex("12345")); // wrong length
        assert!(!is_valid_hex("1234567")); // wrong length
    }

    #[test]
    fn test_inlay_hints_for_tech_badge() {
        let palette = HashMap::new();
        let text = "{{ui:tech:rust:bg=FF5500:border=00FF00/}}";
        let range = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 100,
            },
        };

        let hints = generate_inlay_hints(text, &palette, &range);

        // Should have hints for bg and border colors
        assert!(
            hints.len() >= 2,
            "Expected at least 2 hints, got {}",
            hints.len()
        );
    }

    #[test]
    fn test_inlay_hints_with_palette() {
        let mut palette = HashMap::new();
        palette.insert("accent".to_string(), "FF5500".to_string());

        let text = "{{ui:tech:rust:bg=accent/}}";
        let range = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 100,
            },
        };

        let hints = generate_inlay_hints(text, &palette, &range);

        // Should resolve accent to FF5500 and show hint
        assert!(!hints.is_empty());
        if let Some(InlayHintTooltip::String(tooltip)) = &hints[0].tooltip {
            assert!(tooltip.contains("FF5500"));
        }
    }
}
