//! Sparkline chart component handler

use super::{get_string, parse_bool, parse_param, resolve_color_opt, resolve_color_with_default};
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

    let width: u32 = parse_param(params, "width", 100);
    let height: u32 = parse_param(params, "height", 20);
    let stroke_width: u32 = parse_param(params, "stroke_width", 2);
    let dot_radius: u32 = parse_param(params, "dot_radius", 2);

    let chart_type = get_string(params, "type", "line");
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);
    let stroke_color = resolve_color_opt(params, "stroke", &resolve_color);
    let track_color = resolve_color_opt(params, "track", &resolve_color);

    let show_dots = parse_bool(params, "dots", false);

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
