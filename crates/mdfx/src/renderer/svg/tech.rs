//! Tech badge SVG renderer

use super::swatch::SvgMetrics;

/// Render a tech badge (with placeholder for logo)
pub fn render(name: &str, bg_color: &str, _logo_color: &str, style: &str) -> String {
    let metrics = SvgMetrics::from_style(style);
    // MVP: render with text instead of logo
    // Full implementation would require bundling Simple Icons SVGs
    let font_size = if metrics.height > 24 { 16 } else { 12 };
    let y_pos = metrics.height / 2 + font_size / 3;

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"80\" height=\"{}\" viewBox=\"0 0 80 {}\">\n\
  <rect width=\"80\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>\n\
  <text x=\"40\" y=\"{}\" text-anchor=\"middle\" fill=\"white\" font-family=\"Arial, sans-serif\" font-size=\"{}\">{}</text>\n\
</svg>",
        metrics.height,
        metrics.height,
        metrics.height,
        bg_color,
        metrics.rx,
        y_pos,
        font_size,
        name.to_uppercase()
    )
}
