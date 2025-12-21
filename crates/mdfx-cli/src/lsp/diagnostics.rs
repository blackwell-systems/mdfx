//! Diagnostic generation for the mdfx LSP
//!
//! Provides validation and error reporting for mdfx templates.

use crate::lsp::parser::{extract_tag_name, find_templates, is_inherently_self_closing};
use mdfx::components::params;
use mdfx::Registry;
use mdfx_icons::list_icons;
use std::collections::HashSet;
use tower_lsp::lsp_types::*;

/// Generate diagnostics for template syntax errors
pub fn generate_diagnostics(registry: &Registry, text: &str, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Collect valid tech names for diagnostics
    let icon_list = list_icons();
    let valid_tech_names: HashSet<&str> = icon_list.iter().map(|s| s.as_ref()).collect();

    // Track open tags for matching: (tag_name, line, start_col, end_col)
    let mut tag_stack: Vec<(String, u32, u32, u32)> = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        for (start, is_closing_tag, is_self_closing, is_malformed, content, end) in
            find_templates(line)
        {
            let start_col = start as u32;
            let end_col = end as u32;
            let line_num = line_num as u32;

            // Handle malformed templates (missing `}}`)
            if is_malformed {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_num,
                            character: start_col,
                        },
                        end: Position {
                            line: line_num,
                            character: end_col,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("mdfx".to_string()),
                    message: format!(
                        "Malformed template '{{{{{}' - missing closing '}}}}'",
                        content
                    ),
                    ..Default::default()
                });
                continue;
            }

            // Handle closing tags - check for matching open tag
            if is_closing_tag {
                // Universal closer {{//}} closes any open tag
                if content.is_empty() {
                    if tag_stack.is_empty() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: "Universal closer {{//}} with no open tag to close"
                                .to_string(),
                            ..Default::default()
                        });
                    } else {
                        tag_stack.pop();
                    }
                    continue;
                }

                // Extract tag name for matching
                let close_tag_name = extract_tag_name(content);

                if let Some((open_tag_name, open_line, open_start, open_end)) = tag_stack.pop() {
                    if open_tag_name != close_tag_name {
                        // Mismatched tags
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: format!(
                                "Mismatched closing tag '{{{{/{}}}}}, expected '{{{{/{}}}}}'",
                                close_tag_name, open_tag_name
                            ),
                            related_information: Some(vec![DiagnosticRelatedInformation {
                                location: Location {
                                    uri: uri.clone(),
                                    range: Range {
                                        start: Position {
                                            line: open_line,
                                            character: open_start,
                                        },
                                        end: Position {
                                            line: open_line,
                                            character: open_end,
                                        },
                                    },
                                },
                                message: format!("Opening tag '{{{{{}}}}}' is here", open_tag_name),
                            }]),
                            ..Default::default()
                        });
                    }
                } else {
                    // No open tag to close
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Closing tag '{{{{/{}}}}}' with no matching open tag",
                            close_tag_name
                        ),
                        ..Default::default()
                    });
                }
                continue;
            }

            // Handle opening tags (non-self-closing)
            // Skip inherently self-closing templates (ui:, glyph:, swatch:)
            if !is_self_closing {
                if is_inherently_self_closing(content) {
                    // Warn that this template should use /}} syntax
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: end_col - 2,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: "This template should be self-closing. Use '/}}' instead of '}}'"
                            .to_string(),
                        ..Default::default()
                    });
                } else {
                    let tag_name = extract_tag_name(content);
                    tag_stack.push((tag_name, line_num, start_col, end_col));
                }
            }

            // Content validation for opening/self-closing tags
            // Check tech badges: {{ui:tech:NAME...}}
            if let Some(rest) = content.strip_prefix("ui:tech:") {
                let tech_name = rest.split(':').next().unwrap_or("");
                // Case-insensitive check - icons list is lowercase
                if !tech_name.is_empty()
                    && !valid_tech_names.contains(tech_name.to_lowercase().as_str())
                {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Unknown tech badge '{}'. Use autocomplete to see available badges.",
                            tech_name
                        ),
                        ..Default::default()
                    });
                }

                // Validate tech badge parameters
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[
                        ("style", &["flat", "flat-square", "plastic", "for-the-badge", "social", "outline", "ghost"]),
                        ("chevron", &["left", "right", "both"]),
                        ("corners", &["left", "right", "none", "all"]),
                        ("border_full", &["true", "false"]),
                        ("divider", &["true", "false"]),
                    ],
                    &[
                        ("rx", 0, 50),
                        ("height", 10, 100),
                        ("border_width", 0, 10),
                        ("logo_size", 8, 32),
                        ("raised", 0, 20),
                    ],
                );
            }
            // Validate visualization component parameters
            else if let Some(rest) = content.strip_prefix("ui:progress:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[
                        ("label", &["true", "false"]),
                        ("thumb", &["true", "false"]),
                        ("thumb_shape", &["circle", "square", "diamond"]),
                    ],
                    &[
                        ("width", 10, 1000),
                        ("height", 4, 100),
                        ("rx", 0, 50),
                        ("thumb_size", 4, 50),
                        ("thumb_border_width", 0, 10),
                    ],
                );
            } else if let Some(rest) = content.strip_prefix("ui:donut:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[
                        ("label", &["true", "false"]),
                        ("thumb", &["true", "false"]),
                    ],
                    &[
                        ("size", 10, 500),
                        ("thickness", 1, 50),
                        ("thumb_size", 2, 30),
                    ],
                );
            } else if let Some(rest) = content.strip_prefix("ui:gauge:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[
                        ("label", &["true", "false"]),
                        ("thumb", &["true", "false"]),
                    ],
                    &[
                        ("size", 20, 500),
                        ("thickness", 2, 50),
                        ("thumb_size", 2, 30),
                    ],
                );
            } else if let Some(rest) = content.strip_prefix("ui:sparkline:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[
                        ("type", &["line", "bar", "area"]),
                        ("dots", &["true", "false"]),
                    ],
                    &[
                        ("width", 20, 1000),
                        ("height", 10, 200),
                        ("stroke_width", 1, 10),
                        ("dot_size", 1, 20),
                    ],
                );
            } else if let Some(rest) = content.strip_prefix("ui:rating:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[("icon", &["star", "heart", "circle"])],
                    &[("max", 1, 20), ("size", 8, 100), ("gap", 0, 20)],
                );
            } else if let Some(rest) = content.strip_prefix("ui:waveform:") {
                validate_params(
                    rest,
                    &mut diagnostics,
                    line_num,
                    start_col,
                    end_col,
                    &[("center", &["true", "false"])],
                    &[
                        ("width", 20, 1000),
                        ("height", 10, 200),
                        ("bar_width", 1, 20),
                        ("bar", 1, 20),
                        ("gap", 0, 20),
                    ],
                );
            }
            // Check glyphs: {{glyph:NAME/}}
            else if let Some(glyph_name) = content.strip_prefix("glyph:") {
                if !glyph_name.is_empty() && registry.glyph(glyph_name).is_none() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Unknown glyph '{}'. Use autocomplete to see available glyphs.",
                            glyph_name
                        ),
                        ..Default::default()
                    });
                }
            }
            // Check live badges: {{ui:live:SOURCE:QUERY:METRIC/}}
            else if let Some(rest) = content.strip_prefix("ui:live:") {
                let parts: Vec<&str> = rest.split(':').collect();
                let valid_sources: Vec<&str> = params::valid_live_sources().collect();

                if parts.is_empty() || parts[0].is_empty() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("mdfx".to_string()),
                        message:
                            "Incomplete live badge syntax. Expected: {{ui:live:source:query:metric/}}"
                                .to_string(),
                        ..Default::default()
                    });
                } else {
                    let source = parts[0];

                    if !valid_sources.contains(&source) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: format!(
                                "Unknown live source '{}'. Valid sources: {}",
                                source,
                                valid_sources.join(", ")
                            ),
                            ..Default::default()
                        });
                    } else if parts.len() > 2 {
                        let metric = parts[2];
                        if !params::is_valid_metric(source, metric) {
                            let valid_metrics: Vec<&str> = params::metrics_for_source(source)
                                .map(|m| m.iter().map(|(name, _)| *name).collect())
                                .unwrap_or_default();
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: line_num,
                                        character: start_col,
                                    },
                                    end: Position {
                                        line: line_num,
                                        character: end_col,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                source: Some("mdfx".to_string()),
                                message: format!(
                                    "Unknown metric '{}' for source '{}'. Valid metrics: {}",
                                    metric,
                                    source,
                                    valid_metrics.join(", ")
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    // Report any unclosed tags
    for (tag_name, line_num, start_col, end_col) in tag_stack {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: line_num,
                    character: start_col,
                },
                end: Position {
                    line: line_num,
                    character: end_col,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("mdfx".to_string()),
            message: format!(
                "Unclosed tag '{{{{{}}}}}' - missing '{{{{/{}}}}}' or '{{{{//}}}}'",
                tag_name, tag_name
            ),
            ..Default::default()
        });
    }

    diagnostics
}

/// Validate parameter values in a template
fn validate_params(
    content: &str,
    diagnostics: &mut Vec<Diagnostic>,
    line_num: u32,
    start_col: u32,
    end_col: u32,
    enum_params: &[(&str, &[&str])],
    range_params: &[(&str, i64, i64)],
) {
    // Parse key=value pairs from content
    for part in content.split(':') {
        if let Some((key, value)) = part.split_once('=') {
            // Check enum parameters
            for (param_name, valid_values) in enum_params {
                if key == *param_name && !valid_values.contains(&value) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Invalid value '{}' for parameter '{}'. Valid values: {}",
                            value,
                            key,
                            valid_values.join(", ")
                        ),
                        ..Default::default()
                    });
                }
            }

            // Check numeric range parameters
            for (param_name, min, max) in range_params {
                if key == *param_name {
                    if let Ok(num) = value.parse::<i64>() {
                        if num < *min || num > *max {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: line_num,
                                        character: start_col,
                                    },
                                    end: Position {
                                        line: line_num,
                                        character: end_col,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                source: Some("mdfx".to_string()),
                                message: format!(
                                    "Value {} for '{}' is outside recommended range ({}-{})",
                                    num, key, min, max
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }
}
