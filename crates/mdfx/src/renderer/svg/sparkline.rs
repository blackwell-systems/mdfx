//! Sparkline chart SVG renderer (line, bar, or area)

/// Render a sparkline chart (line, bar, or area)
#[allow(clippy::too_many_arguments)]
pub fn render(
    values: &[f32],
    width: u32,
    height: u32,
    chart_type: &str,
    fill_color: &str,
    stroke_color: Option<&str>,
    stroke_width: u32,
    track_color: Option<&str>,
    show_dots: bool,
    dot_radius: u32,
) -> String {
    if values.is_empty() {
        // Empty sparkline: just return empty box
        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\"></svg>",
            width, height, width, height
        );
    }

    // Find min/max for normalization
    let min_val = values.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_val = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = if (max_val - min_val).abs() < 0.001 {
        1.0 // Avoid division by zero for flat data
    } else {
        max_val - min_val
    };

    // Padding to prevent clipping
    let padding = stroke_width.max(dot_radius) + 1;
    let usable_height = height - padding * 2;
    let usable_width = width - padding * 2;

    // Calculate points
    let num_points = values.len();
    let x_step = if num_points > 1 {
        usable_width as f32 / (num_points - 1) as f32
    } else {
        usable_width as f32
    };

    let points: Vec<(f32, f32)> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let x = padding as f32 + i as f32 * x_step;
            // Invert Y because SVG Y-axis is top-down
            let normalized = (v - min_val) / range;
            let y = padding as f32 + usable_height as f32 * (1.0 - normalized);
            (x, y)
        })
        .collect();

    // Build background if track_color is provided
    let track_elem = if let Some(tc) = track_color {
        format!(
            "  <rect width=\"{}\" height=\"{}\" fill=\"#{}\"/>\n",
            width, height, tc
        )
    } else {
        String::new()
    };

    // Build chart based on type
    let chart_elem = match chart_type {
        "bar" => {
            // Bar chart: vertical bars for each value
            let bar_width = (usable_width as f32 / num_points as f32 * 0.8).max(2.0);
            let bar_gap = (usable_width as f32 / num_points as f32 * 0.2).max(1.0);
            let bars: Vec<String> = values
                .iter()
                .enumerate()
                .map(|(i, &v)| {
                    let normalized = (v - min_val) / range;
                    let bar_height = (usable_height as f32 * normalized).max(1.0);
                    let x = padding as f32 + i as f32 * (bar_width + bar_gap);
                    let y = padding as f32 + usable_height as f32 - bar_height;
                    format!(
                        "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#{}\"/>",
                        x, y, bar_width, bar_height, fill_color
                    )
                })
                .collect();
            bars.join("\n")
        }
        "area" => {
            // Area chart: filled polygon under line
            if points.is_empty() {
                String::new()
            } else {
                let mut path_points: Vec<String> = points
                    .iter()
                    .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                    .collect();

                // Close the area: go down to bottom, across, and back up
                let last_x = points.last().map(|(x, _)| *x).unwrap_or(0.0);
                let first_x = points.first().map(|(x, _)| *x).unwrap_or(0.0);
                let bottom_y = height as f32 - padding as f32;

                path_points.push(format!("{:.1},{:.1}", last_x, bottom_y));
                path_points.push(format!("{:.1},{:.1}", first_x, bottom_y));

                let stroke_col = stroke_color.unwrap_or(fill_color);
                let area_path = format!(
                    "  <polygon points=\"{}\" fill=\"#{}\" fill-opacity=\"0.3\"/>",
                    path_points.join(" "),
                    fill_color
                );
                let line_path = format!(
                    "  <polyline points=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\"/>",
                    points
                        .iter()
                        .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                        .collect::<Vec<_>>()
                        .join(" "),
                    stroke_col,
                    stroke_width
                );
                format!("{}\n{}", area_path, line_path)
            }
        }
        _ => {
            // Default: line chart
            if points.is_empty() {
                String::new()
            } else {
                let stroke_col = stroke_color.unwrap_or(fill_color);
                let points_str: String = points
                    .iter()
                    .map(|(x, y)| format!("{:.1},{:.1}", x, y))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    "  <polyline points=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
                    points_str, stroke_col, stroke_width
                )
            }
        }
    };

    // Build dots if requested (for line/area types)
    let dots_elem = if show_dots && chart_type != "bar" {
        let dots: Vec<String> = points
            .iter()
            .map(|(x, y)| {
                format!(
                    "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{}\" fill=\"#{}\"/>",
                    x, y, dot_radius, fill_color
                )
            })
            .collect();
        format!("\n{}", dots.join("\n"))
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n{}{}{}\n</svg>",
        width, height, width, height, track_elem, chart_elem, dots_elem
    )
}
