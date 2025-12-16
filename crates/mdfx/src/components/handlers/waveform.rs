//! Waveform visualization component handler

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

    let width: u32 = params
        .get("width")
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);

    let height: u32 = params
        .get("height")
        .and_then(|v| v.parse().ok())
        .unwrap_or(40);

    let positive_color = params
        .get("positive")
        .or_else(|| params.get("up"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("success"));

    let negative_color = params
        .get("negative")
        .or_else(|| params.get("down"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("error"));

    let bar_width: u32 = params
        .get("bar_width")
        .or_else(|| params.get("bar"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    let spacing: u32 = params
        .get("spacing")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);

    let track_color = params.get("track").map(|c| resolve_color(c));

    let show_center_line = params
        .get("center")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let center_line_color = params.get("center_color").map(|c| resolve_color(c));

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
