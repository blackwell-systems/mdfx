//! Sparkline chart component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle sparkline component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "sparkline component requires comma-separated values".to_string(),
        ));
    }

    // Parse values from first arg (comma-separated)
    let values: Vec<f32> = args[0]
        .split(',')
        .filter_map(|s| s.trim().parse::<f32>().ok())
        .collect();

    if values.is_empty() {
        return Err(Error::ParseError(
            "sparkline requires at least one numeric value".to_string(),
        ));
    }

    let width: u32 = params
        .get("width")
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);

    let height: u32 = params
        .get("height")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let chart_type = params
        .get("type")
        .cloned()
        .unwrap_or_else(|| "line".to_string());

    let fill_color = params
        .get("fill")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("accent"));

    let stroke_color = params.get("stroke").map(|c| resolve_color(c));

    let stroke_width: u32 = params
        .get("stroke_width")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    let track_color = params.get("track").map(|c| resolve_color(c));

    let show_dots = params
        .get("dots")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let dot_radius: u32 = params
        .get("dot_radius")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    Ok(ComponentOutput::Primitive(Primitive::Sparkline {
        values,
        width,
        height,
        chart_type,
        fill_color,
        stroke_color,
        stroke_width,
        track_color,
        show_dots,
        dot_radius,
    }))
}
