//! Rating display SVG renderer (stars, hearts, etc.) with partial fill support

/// Render a rating display (stars, hearts, etc.) with partial fill support
pub fn render(
    value: f32,
    max: u32,
    size: u32,
    fill_color: &str,
    empty_color: &str,
    icon: &str,
    spacing: u32,
) -> String {
    let total_width = max * size + (max - 1) * spacing;
    let height = size;

    // Get the icon path based on type
    let icon_path = match icon {
        "heart" => "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z",
        "circle" => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z",
        _ => "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z", // star
    };

    let mut elements = Vec::new();

    // Add defs for clipping masks
    let mut defs = String::from("  <defs>\n");
    for i in 0..max {
        let x_offset = i * (size + spacing);
        defs.push_str(&format!(
            "    <clipPath id=\"clip-{}\"><rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\"/></clipPath>\n",
            i, x_offset, size, height
        ));
    }
    defs.push_str("  </defs>\n");
    elements.push(defs);

    // Render each icon
    for i in 0..max {
        let x_offset = i * (size + spacing);
        let icon_value = i as f32 + 1.0;

        // Calculate fill percentage for this icon
        let fill_percent = if value >= icon_value {
            1.0 // Fully filled
        } else if value > icon_value - 1.0 {
            value - (icon_value - 1.0) // Partial fill
        } else {
            0.0 // Empty
        };

        // Scale factor to fit icon in size (icons are 24x24 viewBox)
        let scale = size as f32 / 24.0;
        let transform = format!("translate({}, 0) scale({})", x_offset, scale);

        // Draw empty (background) icon
        elements.push(format!(
            "  <path d=\"{}\" fill=\"#{}\" transform=\"{}\"/>",
            icon_path, empty_color, transform
        ));

        // Draw filled portion with clip path
        if fill_percent > 0.0 {
            let clip_width = (size as f32 * fill_percent) as u32;
            elements.push(format!(
                "  <g clip-path=\"url(#partial-{})\"><path d=\"{}\" fill=\"#{}\" transform=\"{}\"/></g>",
                i, icon_path, fill_color, transform
            ));

            // Add the partial clip path to defs (insert after existing defs)
            let partial_clip = format!(
                "    <clipPath id=\"partial-{}\"><rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\"/></clipPath>",
                i, x_offset, clip_width, height
            );
            // We'll handle this differently - add all clip paths at once
            if let Some(defs_elem) = elements.get_mut(0) {
                // Insert before closing </defs>
                let insert_pos = defs_elem.rfind("  </defs>").unwrap_or(defs_elem.len());
                defs_elem.insert_str(insert_pos, &format!("{}\n", partial_clip));
            }
        }
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n{}</svg>",
        total_width, height, total_width, height,
        elements.join("\n")
    )
}
