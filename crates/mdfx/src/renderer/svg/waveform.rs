//! Waveform visualization SVG renderer with bars above/below center

/// Render a waveform visualization with bars above/below center
#[allow(clippy::too_many_arguments)]
pub fn render(
    values: &[f32],
    width: u32,
    height: u32,
    positive_color: &str,
    negative_color: &str,
    bar_width: u32,
    spacing: u32,
    track_color: Option<&str>,
    show_center_line: bool,
    center_line_color: Option<&str>,
) -> String {
    if values.is_empty() {
        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\"></svg>",
            width, height, width, height
        );
    }

    let mut elements = Vec::new();

    // Optional background
    if let Some(track) = track_color {
        elements.push(format!(
            "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>",
            width, height, track
        ));
    }

    // Find the max absolute value for normalization
    let max_abs = values
        .iter()
        .map(|v| v.abs())
        .fold(0.0f32, f32::max)
        .max(0.001); // Prevent division by zero

    let center_y = height as f32 / 2.0;
    let half_height = center_y;

    // Calculate number of bars that can fit
    let num_bars = values.len();
    let total_bar_space =
        num_bars as u32 * bar_width + (num_bars.saturating_sub(1)) as u32 * spacing;

    // Calculate starting x to center the waveform
    let start_x = if total_bar_space < width {
        (width - total_bar_space) / 2
    } else {
        0
    };

    // Draw each bar
    for (i, &value) in values.iter().enumerate() {
        let x = start_x + i as u32 * (bar_width + spacing);
        let normalized = value / max_abs; // -1.0 to 1.0
        let bar_height = (normalized.abs() * half_height).max(1.0);

        let (y, color) = if normalized >= 0.0 {
            // Positive: draw upward from center
            (center_y - bar_height, positive_color)
        } else {
            // Negative: draw downward from center
            (center_y, negative_color)
        };

        elements.push(format!(
            "  <rect x=\"{}\" y=\"{:.1}\" width=\"{}\" height=\"{:.1}\" fill=\"#{}\"/>",
            x, y, bar_width, bar_height, color
        ));
    }

    // Optional center line
    if show_center_line {
        let line_color = center_line_color.unwrap_or("6B7280"); // slate default
        elements.push(format!(
            "  <line x1=\"0\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" stroke=\"#{}\" stroke-width=\"1\" opacity=\"0.5\"/>",
            center_y, width, center_y, line_color
        ));
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n{}\n</svg>",
        width, height, width, height,
        elements.join("\n")
    )
}
