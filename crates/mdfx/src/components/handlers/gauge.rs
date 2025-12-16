//! Gauge (semi-circular meter) component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle gauge component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "gauge component requires a percentage argument".to_string(),
        ));
    }

    // Parse percentage (first arg)
    let percent: u8 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid percentage '{}' - must be a number 0-100",
            args[0]
        ))
    })?;
    let percent = percent.min(100);

    let size: u32 = params
        .get("size")
        .and_then(|v| v.parse().ok())
        .unwrap_or(80);

    let thickness: u32 = params
        .get("thickness")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);

    let track_color = params
        .get("track")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("slate"));

    let fill_color = params
        .get("fill")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("accent"));

    let show_label = params
        .get("label")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let label_color = params.get("label_color").map(|c| resolve_color(c));

    // Thumb (slider mode)
    let thumb_size: Option<u32> = params.get("thumb").and_then(|v| v.parse().ok());
    let thumb_color = params.get("thumb_color").map(|c| resolve_color(c));

    Ok(ComponentOutput::Primitive(Primitive::Gauge {
        percent,
        size,
        thickness,
        track_color,
        fill_color,
        show_label,
        label_color,
        thumb_size,
        thumb_color,
    }))
}
