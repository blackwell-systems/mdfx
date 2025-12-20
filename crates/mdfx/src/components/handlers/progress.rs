//! Progress bar component handler

use super::{
    parse_bool, parse_param, parse_thumb_config, resolve_color_opt, resolve_color_with_default,
    resolve_color_with_fallback,
};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle progress component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "progress component requires a percentage argument".to_string(),
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

    let width: u32 = parse_param(params, "width", 100);
    let height: u32 = parse_param(params, "height", 10);
    let fill_height: u32 = parse_param(params, "fill_height", height);
    let rx: u32 = parse_param(params, "rx", 3);

    let track_color =
        resolve_color_with_fallback(params, &["track", "color"], "gray", &resolve_color);
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);

    let show_label = parse_bool(params, "label", false);
    let label_color = resolve_color_opt(params, "label_color", &resolve_color);
    let border_color = resolve_color_opt(params, "border", &resolve_color);
    let border_width: u32 = parse_param(
        params,
        "border_width",
        if border_color.is_some() { 1 } else { 0 },
    );

    // Parse thumb configuration (enables slider mode)
    let thumb = parse_thumb_config(params, &resolve_color);

    Ok(ComponentOutput::Primitive(Primitive::Progress {
        percent,
        width,
        height,
        track_color,
        fill_color,
        fill_height,
        rx,
        show_label,
        label_color,
        border_color,
        border_width,
        thumb,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn identity_color(c: &str) -> String {
        c.to_string()
    }

    // ========================================================================
    // Basic Progress Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("0", 0)]
    #[case("50", 50)]
    #[case("75", 75)]
    #[case("100", 100)]
    #[case("150", 100)] // clamped to 100
    fn test_handle_percent(#[case] input: &str, #[case] expected: u8) {
        let result = handle(&[input.to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress { percent, .. })) = result {
            assert_eq!(percent, expected);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_missing_args() {
        let result = handle(&[], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_invalid_percent() {
        let result = handle(&["abc".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    // ========================================================================
    // Size Parameters (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("width", "200", 200, 10, 10, 3)]
    #[case("height", "20", 100, 20, 20, 3)]
    #[case("rx", "5", 100, 10, 10, 5)]
    fn test_handle_size_params(
        #[case] key: &str,
        #[case] value: &str,
        #[case] expected_width: u32,
        #[case] expected_height: u32,
        #[case] expected_fill_height: u32,
        #[case] expected_rx: u32,
    ) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            width,
            height,
            fill_height,
            rx,
            ..
        })) = result
        {
            assert_eq!(width, expected_width);
            assert_eq!(height, expected_height);
            assert_eq!(fill_height, expected_fill_height);
            assert_eq!(rx, expected_rx);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_fill_height() {
        let mut params = HashMap::new();
        params.insert("height".to_string(), "20".to_string());
        params.insert("fill_height".to_string(), "16".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            height,
            fill_height,
            ..
        })) = result
        {
            assert_eq!(height, 20);
            assert_eq!(fill_height, 16); // custom fill height
        } else {
            panic!("Expected Progress primitive");
        }
    }

    // ========================================================================
    // Color Parameters
    // ========================================================================

    #[test]
    fn test_handle_colors() {
        let mut params = HashMap::new();
        params.insert("track".to_string(), "AABBCC".to_string());
        params.insert("fill".to_string(), "FF0000".to_string());

        let result = handle(&["75".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            track_color,
            fill_color,
            ..
        })) = result
        {
            assert_eq!(track_color, "AABBCC");
            assert_eq!(fill_color, "FF0000");
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_color_fallback() {
        // "color" should work as fallback for "track"
        let mut params = HashMap::new();
        params.insert("color".to_string(), "123456".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress { track_color, .. })) = result {
            assert_eq!(track_color, "123456");
        } else {
            panic!("Expected Progress primitive");
        }
    }

    // ========================================================================
    // Border Parameters
    // ========================================================================

    #[test]
    fn test_handle_border() {
        let mut params = HashMap::new();
        params.insert("border".to_string(), "000000".to_string());
        params.insert("border_width".to_string(), "2".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            border_color,
            border_width,
            ..
        })) = result
        {
            assert_eq!(border_color, Some("000000".to_string()));
            assert_eq!(border_width, 2);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_border_defaults_width_when_color_set() {
        let mut params = HashMap::new();
        params.insert("border".to_string(), "000000".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            border_color,
            border_width,
            ..
        })) = result
        {
            assert_eq!(border_color, Some("000000".to_string()));
            assert_eq!(border_width, 1); // default to 1 when border color is set
        } else {
            panic!("Expected Progress primitive");
        }
    }

    // ========================================================================
    // Label and Thumb
    // ========================================================================

    #[rstest]
    #[case("true", true)]
    #[case("1", true)]
    #[case("false", false)]
    fn test_handle_label(#[case] value: &str, #[case] expected: bool) {
        let mut params = HashMap::new();
        params.insert("label".to_string(), value.to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress { show_label, .. })) = result {
            assert_eq!(show_label, expected);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_thumb() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "14".to_string());
        params.insert("thumb_color".to_string(), "FFFFFF".to_string());
        params.insert("thumb_border".to_string(), "000000".to_string());
        params.insert("thumb_border_width".to_string(), "2".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress { thumb, .. })) = result {
            assert!(thumb.is_some());
            let thumb = thumb.unwrap();
            assert_eq!(thumb.size, 14);
            assert_eq!(thumb.color, Some("FFFFFF".to_string()));
            assert_eq!(thumb.border, Some("000000".to_string()));
            assert_eq!(thumb.border_width, 2);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_handle_defaults() {
        let result = handle(&["50".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Progress {
            width,
            height,
            rx,
            show_label,
            border_width,
            thumb,
            ..
        })) = result
        {
            assert_eq!(width, 100);
            assert_eq!(height, 10);
            assert_eq!(rx, 3);
            assert!(!show_label);
            assert_eq!(border_width, 0);
            assert!(thumb.is_none());
        } else {
            panic!("Expected Progress primitive");
        }
    }
}
