//! Waveform visualization component handler

use super::{parse_bool, parse_param, resolve_color_opt, resolve_color_with_fallback};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle waveform component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "waveform component requires values argument".to_string(),
        ));
    }

    // Parse values (comma-separated, can be negative)
    let values: Vec<f32> = args[0]
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        return Err(Error::ParseError(
            "waveform values must contain at least one number".to_string(),
        ));
    }

    let width: u32 = parse_param(params, "width", 100);
    let height: u32 = parse_param(params, "height", 40);
    let spacing: u32 = parse_param(params, "spacing", 1);

    // bar_width with "bar" alias
    let bar_width: u32 = params
        .get("bar_width")
        .or_else(|| params.get("bar"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    let positive_color =
        resolve_color_with_fallback(params, &["positive", "up"], "success", &resolve_color);
    let negative_color =
        resolve_color_with_fallback(params, &["negative", "down"], "error", &resolve_color);
    let track_color = resolve_color_opt(params, "track", &resolve_color);

    let show_center_line = parse_bool(params, "center", false);
    let center_line_color = resolve_color_opt(params, "center_color", &resolve_color);

    Ok(ComponentOutput::Primitive(Primitive::Waveform {
        values,
        width,
        height,
        positive_color,
        negative_color,
        bar_width,
        spacing,
        track_color,
        show_center_line,
        center_line_color,
    }))
}
