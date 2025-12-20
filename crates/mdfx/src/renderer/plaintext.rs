/// Plain text backend for maximum compatibility (PyPI, ASCII-only contexts).
///
/// This backend renders primitives as plain text representations without
/// external dependencies or Unicode decorations. Useful for PyPI package
/// descriptions and other environments with limited rendering support.
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};

/// Plain text rendering backend.
///
/// Renders primitives as ASCII-compatible text representations:
/// - Swatches: `[#RRGGBB]` color codes
/// - Tech badges: `[Technology]` text labels
/// - Progress: `[=====>    ] 50%` ASCII bars
#[derive(Debug, Clone, Default)]
pub struct PlainTextBackend;

impl PlainTextBackend {
    /// Create a new plain text backend
    pub fn new() -> Self {
        PlainTextBackend
    }
}

impl Renderer for PlainTextBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let text = match primitive {
            Primitive::Swatch {
                color, label, icon, ..
            } => {
                // Icon takes precedence over label
                if let Some(icon_name) = icon {
                    format!("[#{} {}]", color, icon_name)
                } else if let Some(lbl) = label {
                    format!("[#{} {}]", color, lbl)
                } else {
                    format!("[#{}]", color)
                }
            }

            Primitive::Tech(cfg) => {
                if let Some(lbl) = &cfg.label {
                    format!("[{} | {}]", cfg.name, lbl)
                } else {
                    format!("[{}]", cfg.name)
                }
            }

            Primitive::Version(cfg) => {
                // Render as [v1.0.0] or [1.0.0]
                let prefix = cfg.prefix.as_deref().unwrap_or("v");
                if cfg.version.starts_with('v') || cfg.version.starts_with('V') || prefix.is_empty()
                {
                    format!("[{}]", cfg.version)
                } else {
                    format!("[{}{}]", prefix, cfg.version)
                }
            }

            Primitive::License(cfg) => {
                // Render as [MIT] or [Custom Label]
                if let Some(lbl) = &cfg.label {
                    format!("[{}]", lbl)
                } else {
                    format!("[{}]", cfg.license)
                }
            }

            Primitive::Progress { percent, .. } => {
                // Render as ASCII progress bar: [=====>    ] 50%
                let width = 10;
                let filled = (*percent as usize * width / 100).min(width);
                let empty = width - filled;
                let bar: String = "=".repeat(filled.saturating_sub(1))
                    + if filled > 0 { ">" } else { "" }
                    + &" ".repeat(empty);
                format!("[{}] {}%", bar, percent)
            }

            Primitive::Donut { percent, .. } => {
                // Render as ASCII donut: (75%)
                format!("({}%)", percent)
            }

            Primitive::Gauge { percent, .. } => {
                // Render as ASCII gauge: [75%]
                format!("[{}%]", percent)
            }

            Primitive::Sparkline { values, .. } => {
                // Render as ASCII sparkline using braille-like characters
                // ▁▂▃▄▅▆▇█
                if values.is_empty() {
                    return Ok(RenderedAsset::InlineMarkdown("▁".to_string()));
                }
                let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
                let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let range = if (max - min).abs() < 0.001 {
                    1.0
                } else {
                    max - min
                };
                let bars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let spark: String = values
                    .iter()
                    .map(|&v| {
                        let normalized = (v - min) / range;
                        let idx = ((normalized * 7.0).round() as usize).min(7);
                        bars[idx]
                    })
                    .collect();
                spark
            }

            Primitive::Rating {
                value, max, icon, ..
            } => {
                // Render as Unicode stars/hearts/circles
                let (filled_char, empty_char) = match icon.as_str() {
                    "heart" => ('♥', '♡'),
                    "circle" => ('●', '○'),
                    _ => ('★', '☆'), // star (default)
                };

                let filled = value.floor() as u32;
                let has_half = (value - value.floor()) >= 0.5;
                let empty = max
                    .saturating_sub(filled)
                    .saturating_sub(if has_half { 1 } else { 0 });

                let mut result = String::new();
                for _ in 0..filled.min(*max) {
                    result.push(filled_char);
                }
                if has_half && filled < *max {
                    // Use a half character or just show empty for simplicity
                    result.push(empty_char);
                }
                for _ in 0..empty {
                    result.push(empty_char);
                }
                result
            }

            Primitive::Waveform { values, .. } => {
                // Render as Unicode bar characters based on value magnitude
                // Use block characters: ▁▂▃▄▅▆▇█ for positive, ▔ for negative center
                if values.is_empty() {
                    return Ok(RenderedAsset::InlineMarkdown("▔".to_string()));
                }
                let max_abs = values
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0f32, f32::max)
                    .max(0.001);
                let bars_pos = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let wave: String = values
                    .iter()
                    .map(|&v| {
                        let normalized = (v / max_abs).abs();
                        let idx = ((normalized * 7.0).round() as usize).min(7);
                        if v >= 0.0 {
                            bars_pos[idx]
                        } else {
                            // For negative, use same bars but could differentiate
                            bars_pos[idx]
                        }
                    })
                    .collect();
                wave
            }
        };

        Ok(RenderedAsset::InlineMarkdown(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::{LicenseConfig, TechConfig, VersionConfig};
    use rstest::rstest;

    // ========================================================================
    // Swatch Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("F41C80", None, None, "[#F41C80]")] // simple swatch
    #[case("FF6B35", Some("v1.0"), None, "[#FF6B35 v1.0]")] // with label
    #[case("F41C80", None, Some("rust"), "[#F41C80 rust]")] // with icon
    #[case("ABC123", Some("label"), Some("icon"), "[#ABC123 icon]")] // icon takes precedence
    fn test_plaintext_swatch(
        #[case] color: &str,
        #[case] label: Option<&str>,
        #[case] icon: Option<&str>,
        #[case] expected: &str,
    ) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Swatch {
            color: color.to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: None,
            height: None,
            border_color: None,
            border_width: None,
            label: label.map(String::from),
            label_color: None,
            icon: icon.map(String::from),
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Tech Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("rust", None, "[rust]")] // name only
    #[case("rust", Some("v1.80"), "[rust | v1.80]")] // with label
    #[case("python", Some("3.12"), "[python | 3.12]")] // different tech
    fn test_plaintext_tech(
        #[case] name: &str,
        #[case] label: Option<&str>,
        #[case] expected: &str,
    ) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Tech(TechConfig {
            name: name.to_string(),
            label: label.map(String::from),
            ..Default::default()
        });
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Version Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("1.0.0", None, "[v1.0.0]")] // default prefix
    #[case("v2.0.0", None, "[v2.0.0]")] // already has v prefix
    #[case("V3.0.0", None, "[V3.0.0]")] // uppercase V
    #[case("1.0.0", Some(""), "[1.0.0]")] // empty prefix
    #[case("1.0.0", Some("ver"), "[ver1.0.0]")] // custom prefix
    fn test_plaintext_version(
        #[case] version: &str,
        #[case] prefix: Option<&str>,
        #[case] expected: &str,
    ) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Version(VersionConfig {
            version: version.to_string(),
            prefix: prefix.map(String::from),
            ..Default::default()
        });
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // License Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("MIT", None, "[MIT]")] // license only
    #[case("GPL-3.0", None, "[GPL-3.0]")]
    #[case("MIT", Some("Open Source"), "[Open Source]")] // custom label
    fn test_plaintext_license(
        #[case] license: &str,
        #[case] label: Option<&str>,
        #[case] expected: &str,
    ) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::License(LicenseConfig {
            license: license.to_string(),
            label: label.map(String::from),
            ..Default::default()
        });
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Progress Bar Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, "[          ] 0%")] // 0% = no fill
    #[case(50, "[====>     ] 50%")]
    #[case(100, "[=========>] 100%")]
    fn test_plaintext_progress(#[case] percent: u8, #[case] expected: &str) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Progress {
            percent,
            width: 100,
            height: 10,
            track_color: "gray".to_string(),
            fill_color: "pink".to_string(),
            fill_height: 10,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Donut Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, "(0%)")]
    #[case(50, "(50%)")]
    #[case(100, "(100%)")]
    fn test_plaintext_donut(#[case] percent: u8, #[case] expected: &str) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Donut {
            percent,
            size: 40,
            thickness: 4,
            track_color: "gray".to_string(),
            fill_color: "pink".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Gauge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, "[0%]")]
    #[case(75, "[75%]")]
    #[case(100, "[100%]")]
    fn test_plaintext_gauge(#[case] percent: u8, #[case] expected: &str) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Gauge {
            percent,
            size: 80,
            thickness: 8,
            track_color: "gray".to_string(),
            fill_color: "pink".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Sparkline Rendering
    // ========================================================================

    #[test]
    fn test_plaintext_sparkline_empty() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Sparkline {
            values: vec![],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "pink".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "▁");
    }

    #[test]
    fn test_plaintext_sparkline_values() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Sparkline {
            values: vec![1.0, 5.0, 3.0, 7.0],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "pink".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        let asset = backend.render(&primitive).unwrap();
        // Should contain 4 bar characters
        assert_eq!(asset.to_markdown().chars().count(), 4);
    }

    #[test]
    fn test_plaintext_sparkline_flat() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Sparkline {
            values: vec![5.0, 5.0, 5.0],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "pink".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        let asset = backend.render(&primitive).unwrap();
        // All same value should produce same bars
        let s = asset.to_markdown();
        assert_eq!(s.chars().count(), 3);
    }

    // ========================================================================
    // Rating Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(5.0, 5, "star", "★★★★★")]
    #[case(3.0, 5, "star", "★★★☆☆")]
    #[case(3.5, 5, "star", "★★★☆☆")] // half shows as empty
    #[case(0.0, 5, "star", "☆☆☆☆☆")]
    #[case(4.0, 5, "heart", "♥♥♥♥♡")]
    #[case(3.0, 5, "circle", "●●●○○")]
    fn test_plaintext_rating(
        #[case] value: f32,
        #[case] max: u32,
        #[case] icon: &str,
        #[case] expected: &str,
    ) {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Rating {
            value,
            max,
            size: 20,
            fill_color: "warning".to_string(),
            empty_color: "gray".to_string(),
            icon: icon.to_string(),
            spacing: 2,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), expected);
    }

    // ========================================================================
    // Waveform Rendering
    // ========================================================================

    #[test]
    fn test_plaintext_waveform_empty() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Waveform {
            values: vec![],
            width: 100,
            height: 40,
            positive_color: "success".to_string(),
            negative_color: "error".to_string(),
            bar_width: 3,
            spacing: 1,
            track_color: None,
            show_center_line: false,
            center_line_color: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "▔");
    }

    #[test]
    fn test_plaintext_waveform_values() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Waveform {
            values: vec![1.0, -1.0, 0.5, -0.5],
            width: 100,
            height: 40,
            positive_color: "success".to_string(),
            negative_color: "error".to_string(),
            bar_width: 3,
            spacing: 1,
            track_color: None,
            show_center_line: false,
            center_line_color: None,
        };
        let asset = backend.render(&primitive).unwrap();
        // Should contain 4 bar characters
        assert_eq!(asset.to_markdown().chars().count(), 4);
    }
}
