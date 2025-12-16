//! Tech badge component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;

/// Handle tech component expansion
pub fn handle(
    args: &[String],
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "tech component requires a technology name argument".to_string(),
        ));
    }
    let name = args[0].clone();
    let bg_color = resolve_color("ui.bg");
    let logo_color = resolve_color("white");
    Ok(ComponentOutput::Primitive(Primitive::Tech {
        name,
        bg_color,
        logo_color,
        style: style.to_string(),
    }))
}
