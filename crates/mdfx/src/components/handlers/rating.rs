//! Rating component handler (stars, hearts, etc.)

use super::{get_string, parse_param, resolve_color_with_default};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle rating component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "rating component requires a value argument".to_string(),
        ));
    }

    // Parse value (first arg) - can be float like 3.5
    let value: f32 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid rating value '{}' - must be a number",
            args[0]
        ))
    })?;

    let max: u32 = parse_param(params, "max", 5);
    let size: u32 = parse_param(params, "size", 20);
    let spacing: u32 = parse_param(params, "spacing", 2);

    let fill_color = resolve_color_with_default(params, "fill", "warning", &resolve_color);
    let empty_color = resolve_color_with_default(params, "empty", "gray", &resolve_color);

    let icon = get_string(params, "icon", "star");

    Ok(ComponentOutput::Primitive(Primitive::Rating {
        value,
        max,
        size,
        fill_color,
        empty_color,
        icon,
        spacing,
    }))
}
