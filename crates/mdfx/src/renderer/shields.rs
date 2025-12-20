/// Shields.io backend for rendering primitives as badge URLs
///
/// This backend generates shields.io badge URLs wrapped in Markdown image syntax.
/// It's the default rendering backend for mdfx.
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};
use crate::shields::ShieldsRenderer;

/// Shields.io rendering backend (default)
pub struct ShieldsBackend {
    shields: ShieldsRenderer,
}

impl ShieldsBackend {
    /// Create a new shields backend
    pub fn new() -> Result<Self> {
        Ok(ShieldsBackend {
            shields: ShieldsRenderer::new()?,
        })
    }
}

impl Renderer for ShieldsBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let markdown = match primitive {
            Primitive::Swatch {
                color,
                style,
                icon,
                icon_color,
                label,
                logo_size,
                ..
            } => {
                // Handle different combinations of icon and label
                match (icon, label) {
                    // Both icon and label - render badge with icon and text
                    (Some(icon_name), Some(label_text)) => {
                        let logo_color = icon_color.as_deref().unwrap_or("FFFFFF");
                        self.shields.render_icon_with_label(
                            icon_name, label_text, color, logo_color, style,
                        )?
                    }
                    // Icon only - render icon chip
                    (Some(icon_name), None) => {
                        let logo_color = icon_color.as_deref().unwrap_or("FFFFFF");
                        self.shields.render_icon_with_size(
                            icon_name,
                            color,
                            logo_color,
                            style,
                            logo_size.as_deref(),
                        )?
                    }
                    // Label only - render labeled block
                    (None, Some(label_text)) => self
                        .shields
                        .render_labeled_block(color, label_text, style)?,
                    // Neither - render plain color block
                    (None, None) => self.shields.render_block(color, style)?,
                }
            }

            Primitive::Tech(cfg) => {
                self.shields
                    .render_icon(&cfg.name, &cfg.bg_color, &cfg.logo_color, &cfg.style)?
            }

            // Version badges - render as simple version label
            // Uses badgefx status detection for color if not overridden
            Primitive::Version(cfg) => {
                let status = badgefx::version::detect_status(&cfg.version);
                let bg_color = cfg.bg_color.as_deref().unwrap_or_else(|| status.bg_color());
                let prefix = cfg.prefix.as_deref().unwrap_or("v");
                let label = if cfg.version.starts_with('v')
                    || cfg.version.starts_with('V')
                    || prefix.is_empty()
                {
                    cfg.version.clone()
                } else {
                    format!("{}{}", prefix, cfg.version)
                };
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style={})",
                    label.replace('-', "--"),
                    label.replace('-', "--"),
                    bg_color,
                    cfg.style
                )
            }

            // License badges - render as license label
            // Uses badgefx category detection for color if not overridden
            Primitive::License(cfg) => {
                let category = badgefx::license::categorize(&cfg.license);
                let bg_color = cfg
                    .bg_color
                    .as_deref()
                    .unwrap_or_else(|| category.bg_color());
                let label = cfg
                    .label
                    .clone()
                    .unwrap_or_else(|| badgefx::license::format_name(&cfg.license));
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style={})",
                    label.replace('-', "--").replace(' ', "%20"),
                    label.replace('-', "--").replace(' ', "%20"),
                    bg_color,
                    cfg.style
                )
            }

            // Progress bars use a simple percentage badge as shields.io fallback
            // Full progress bar rendering requires SVG backend
            Primitive::Progress {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Donut charts use a circular percentage badge as shields.io fallback
            // Full donut rendering requires SVG backend
            Primitive::Donut {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Gauge uses a percentage badge as shields.io fallback
            // Full gauge (semi-circle) rendering requires SVG backend
            Primitive::Gauge {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Sparkline uses a chart indicator as shields.io fallback
            // Full sparkline rendering requires SVG backend
            Primitive::Sparkline {
                values, fill_color, ..
            } => {
                // Create a simple text representation of the data range
                let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
                let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let label = if values.is_empty() {
                    "chart".to_string()
                } else {
                    format!("{:.0}..{:.0}", min, max)
                };
                format!(
                    "![](https://img.shields.io/badge/📈-{}-{}?style=flat-square)",
                    label.replace(' ', "%20"),
                    fill_color
                )
            }

            // Rating uses a star badge as shields.io fallback
            // Full rating rendering requires SVG backend
            Primitive::Rating {
                value,
                max,
                fill_color,
                ..
            } => {
                let label = format!("{:.1}/{}", value, max);
                format!(
                    "![](https://img.shields.io/badge/⭐-{}-{}?style=flat-square)",
                    label.replace('.', "%2E"),
                    fill_color
                )
            }

            // Waveform uses audio emoji badge as shields.io fallback
            // Full waveform rendering requires SVG backend
            Primitive::Waveform {
                values,
                positive_color,
                ..
            } => {
                let label = format!("{}pts", values.len());
                format!(
                    "![](https://img.shields.io/badge/🎵-{}-{}?style=flat-square)",
                    label, positive_color
                )
            }
        };

        Ok(RenderedAsset::InlineMarkdown(markdown))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::{LicenseConfig, TechConfig, VersionConfig};
    use rstest::rstest;

    #[test]
    fn test_shields_backend_creation() {
        let backend = ShieldsBackend::new();
        assert!(backend.is_ok());
    }

    #[test]
    fn test_render_swatch_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::simple_swatch("2B6CB0", "flat-square");

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("https://img.shields.io/badge/"));
        assert!(markdown.contains("2B6CB0"));
        assert!(markdown.contains("style=flat-square"));
        assert!(!result.is_file_based());
    }

    #[test]
    fn test_render_tech_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Tech(TechConfig {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "FFFFFF".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("logo=rust"));
        assert!(markdown.contains("logoColor=FFFFFF"));
        assert!(markdown.contains("000000"));
    }

    #[test]
    fn test_render_with_palette_colors() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::simple_swatch("cobalt", "flat-square");

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        // cobalt should resolve to 2B6CB0
        assert!(markdown.contains("2B6CB0"));
    }

    // ========================================================================
    // Version Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("1.0.0", None, "22C55E")] // stable = green
    #[case("0.1.0", None, "EAB308")] // 0.x = beta = yellow
    #[case("2.0.0-beta", None, "EAB308")] // beta = yellow
    #[case("1.0.0", Some("FF0000"), "FF0000")] // custom color override
    fn test_render_version(
        #[case] version: &str,
        #[case] bg_color: Option<&str>,
        #[case] expected_color: &str,
    ) {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Version(VersionConfig {
            version: version.to_string(),
            bg_color: bg_color.map(String::from),
            style: "flat".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("shields.io"));
        assert!(markdown.contains(expected_color));
    }

    #[test]
    fn test_render_version_escapes_dashes() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Version(VersionConfig {
            version: "1.0.0-beta".to_string(),
            style: "flat".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        // dashes should be escaped as --
        assert!(markdown.contains("--"));
    }

    // ========================================================================
    // License Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("MIT", None, "22C55E")] // permissive = green
    #[case("GPL-3.0", None, "EAB308")] // copyleft = yellow
    #[case("LGPL-3.0", None, "3B82F6")] // weak copyleft = blue
    #[case("MIT", Some("FF0000"), "FF0000")] // custom color override
    fn test_render_license(
        #[case] license: &str,
        #[case] bg_color: Option<&str>,
        #[case] expected_color: &str,
    ) {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::License(LicenseConfig {
            license: license.to_string(),
            bg_color: bg_color.map(String::from),
            style: "flat".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("shields.io"));
        assert!(markdown.contains(expected_color));
    }

    #[test]
    fn test_render_license_custom_label() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::License(LicenseConfig {
            license: "MIT".to_string(),
            label: Some("Open Source".to_string()),
            style: "flat".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("Open%20Source"));
    }

    // ========================================================================
    // Progress Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, "pink")]
    #[case(50, "pink")]
    #[case(100, "success")]
    fn test_render_progress(#[case] percent: u8, #[case] fill_color: &str) {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Progress {
            percent,
            width: 100,
            height: 10,
            track_color: "gray".to_string(),
            fill_color: fill_color.to_string(),
            fill_height: 10,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: None,
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("shields.io"));
        assert!(markdown.contains(&format!("{}%25", percent)));
        assert!(markdown.contains(fill_color));
    }

    // ========================================================================
    // Donut Badge Rendering (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, "pink")]
    #[case(75, "success")]
    fn test_render_donut(#[case] percent: u8, #[case] fill_color: &str) {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Donut {
            percent,
            size: 40,
            thickness: 4,
            track_color: "gray".to_string(),
            fill_color: fill_color.to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("shields.io"));
        assert!(markdown.contains(&format!("{}%25", percent)));
    }

    // ========================================================================
    // Gauge Badge Rendering
    // ========================================================================

    #[test]
    fn test_render_gauge() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Gauge {
            percent: 75,
            size: 80,
            thickness: 8,
            track_color: "gray".to_string(),
            fill_color: "pink".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("shields.io"));
        assert!(markdown.contains("75%25"));
        assert!(markdown.contains("pink"));
    }

    // ========================================================================
    // Sparkline Badge Rendering
    // ========================================================================

    #[test]
    fn test_render_sparkline_empty() {
        let backend = ShieldsBackend::new().unwrap();
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

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("chart"));
        assert!(markdown.contains("📈"));
    }

    #[test]
    fn test_render_sparkline_with_values() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Sparkline {
            values: vec![1.0, 5.0, 3.0, 10.0],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "success".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("1..10")); // min..max
        assert!(markdown.contains("success"));
    }

    // ========================================================================
    // Rating Badge Rendering
    // ========================================================================

    #[test]
    fn test_render_rating() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Rating {
            value: 4.5,
            max: 5,
            size: 20,
            fill_color: "warning".to_string(),
            empty_color: "gray".to_string(),
            icon: "star".to_string(),
            spacing: 2,
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("⭐"));
        assert!(markdown.contains("4%2E5")); // escaped dot
        assert!(markdown.contains("warning"));
    }

    // ========================================================================
    // Waveform Badge Rendering
    // ========================================================================

    #[test]
    fn test_render_waveform() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Waveform {
            values: vec![1.0, -1.0, 0.5, -0.5, 0.8],
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

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("🎵"));
        assert!(markdown.contains("5pts"));
        assert!(markdown.contains("success"));
    }
}
