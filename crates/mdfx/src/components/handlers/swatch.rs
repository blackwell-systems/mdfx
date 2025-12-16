//! Swatch component handler - colored rectangles

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle swatch component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "swatch component requires a color argument".to_string(),
        ));
    }

    let color = resolve_color(&args[0]);

    // Parse optional SVG-only parameters
    let opacity = params
        .get("opacity")
        .and_then(|v| v.parse::<f32>().ok())
        .map(|o| o.clamp(0.0, 1.0));
    let width = params.get("width").and_then(|v| v.parse::<u32>().ok());
    let height = params.get("height").and_then(|v| v.parse::<u32>().ok());
    let border_color = params.get("border").map(|v| resolve_color(v));
    let border_width = params
        .get("border_width")
        .and_then(|v| v.parse::<u32>().ok());
    let label = params.get("label").cloned();
    let label_color = params.get("label_color").map(|v| resolve_color(v));
    let icon = params.get("icon").cloned();
    let icon_color = params.get("icon_color").map(|v| resolve_color(v));

    // SVG-only advanced parameters
    let rx = params.get("rx").and_then(|v| v.parse::<u32>().ok());
    let ry = params.get("ry").and_then(|v| v.parse::<u32>().ok());
    let shadow = params.get("shadow").cloned();
    let gradient = params.get("gradient").cloned();
    let stroke_dash = params.get("stroke_dash").cloned();

    // Per-side borders (format: "color/width" or just "color")
    let border_top = params.get("border_top").cloned();
    let border_right = params.get("border_right").cloned();
    let border_bottom = params.get("border_bottom").cloned();
    let border_left = params.get("border_left").cloned();

    // Shields.io-only parameter
    let logo_size = params.get("logo_size").cloned();

    // Style can come from params or use default
    let style = params
        .get("style")
        .cloned()
        .unwrap_or_else(|| style.to_string());

    Ok(ComponentOutput::Primitive(Primitive::Swatch {
        color,
        style,
        opacity,
        width,
        height,
        border_color,
        border_width,
        label,
        label_color,
        icon,
        icon_color,
        rx,
        ry,
        shadow,
        gradient,
        stroke_dash,
        logo_size,
        border_top,
        border_right,
        border_bottom,
        border_left,
    }))
}
