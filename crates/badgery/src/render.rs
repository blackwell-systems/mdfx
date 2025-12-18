//! SVG rendering for technology badges - extracted from mdfx tech.rs

use std::path::Path;
use std::io;

use crate::badge::TechBadge;
use crate::style::{SvgMetrics, BadgeStyle};
use crate::shapes::{rounded_rect_path, chevron_path_with_overlap};

/// Render a badge to SVG string
pub fn render(badge: &TechBadge) -> String {
    let label = badge.display_label();
    let icon_size = badge.logo_size.to_pixels();
    let font_size = calculate_font_size(badge.style);
    
    // Check if we have an icon
    let icon_path = mdfx_icons::icon_path(&badge.name);
    
    let metrics = SvgMetrics::calculate(
        &label, 
        icon_size as f32, 
        font_size as f32, 
        badge.style, 
        icon_path.is_some()
    );
    
    match (icon_path, badge.label.as_ref()) {
        // Icon + Label: Two-segment badge
        (Some(path), Some(_)) | (Some(path), None) => {
            render_with_icon(badge, &metrics, &label, path)
        },
        // No icon found: Text-only badge
        (None, _) => render_text_only(badge, &metrics, &label),
    }
}

/// Calculate appropriate font size based on badge style
fn calculate_font_size(style: BadgeStyle) -> u32 {
    match style {
        BadgeStyle::ForTheBadge => 11,
        BadgeStyle::Social => 11,
        _ => 10,
    }
}

/// Render badge with icon and optional text
fn render_with_icon(badge: &TechBadge, metrics: &SvgMetrics, label: &str, icon_path: &str) -> String {
    let height = metrics.height;
    let icon_size = badge.logo_size.to_pixels() as f32;
    
    // Calculate layout
    let icon_width = (icon_size * 2.5).ceil() as u32 + 1;
    let label_width = estimate_text_width(label) + 16;
    let has_label = !label.is_empty();
    let total_width = icon_width + if has_label { label_width } else { 0 };
    
    let icon_x = (icon_width as f32 - icon_size) / 2.0;
    let icon_y = (height - icon_size) / 2.0;
    let font_size = calculate_font_size(badge.style);
    let text_x = icon_width + label_width / 2;
    let text_y = height as u32 / 2 + font_size / 3;
    
    // Colors
    let bg_color = badge.effective_bg_color().unwrap_or_else(|| "#555".to_string());
    let logo_color = badge.logo_color.as_deref()
        .or_else(|| mdfx_icons::brand_contrast_color(&badge.name))
        .unwrap_or("#FFFFFF");
    
    let default_right_bg = darken_color(&bg_color, 0.15);
    let right_bg = default_right_bg;
    let text_color = badge.text_color.as_deref()
        .unwrap_or_else(|| mdfx_colors::contrast_color(&right_bg));
    
    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");
    
    // Start building SVG
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        total_width, height as u32, total_width, height as u32
    );
    
    // Add gradients for plastic style
    if badge.style.has_gradients() {
        svg.push_str(&create_gradient_defs());
    }
    
    // Background shapes
    if let Some(chevron) = &badge.chevron {
        // Chevron style background
        let (left_path, _, _) = chevron_path_with_overlap(
            0.0, 0.0, icon_width as f32, height, chevron.direction, chevron.depth
        );
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            left_path, bg_color
        ));
        
        if badge.label.is_some() {
            let (right_path, _, _) = chevron_path_with_overlap(
                icon_width as f32, 0.0, label_width as f32, height, chevron.direction, chevron.depth
            );
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}"/>"#,
                right_path, right_bg
            ));
        }
    } else {
        // Regular rounded rectangles
        let corners = badge.corners.as_ref().map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
            .unwrap_or([metrics.radius as u32; 4]);
            
        // Left segment (icon)
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            rounded_rect_path(0.0, 0.0, icon_width as f32, height, corners),
            bg_color
        ));
        
        // Right segment (text) if label exists
        if has_label {
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}"/>"#,
                rounded_rect_path(icon_width as f32, 0.0, label_width as f32, height, corners),
                right_bg
            ));
        }
    }
    
    // Apply raised effect if specified
    let icon_y_adjusted = if let Some(raised_px) = badge.raised {
        icon_y - raised_px as f32 / 2.0
    } else {
        icon_y
    };
    
    // Icon
    let scale = icon_size / 24.0;
    svg.push_str(&format!(
        r#"<g transform="translate({}, {}) scale({})">"#,
        icon_x, icon_y_adjusted, scale
    ));
    svg.push_str(&format!(
        r#"<path d="{}" fill="{}"/>"#,
        icon_path, logo_color
    ));
    svg.push_str("</g>");
    
    // Text label (if present)
    if has_label {
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
            text_x,
            text_y,
            font_family,
            font_size,
            text_color,
            escape_xml(label)
        ));
    }
    
    // Border if specified
    if let Some(border) = &badge.border {
        let border_path = if let Some(chevron) = &badge.chevron {
            // Chevron border
            let (path, _, _) = chevron_path_with_overlap(
                border.width as f32 / 2.0, 
                border.width as f32 / 2.0,
                total_width as f32 - border.width as f32,
                height - border.width as f32,
                chevron.direction,
                chevron.depth
            );
            path
        } else {
            // Regular border
            let corners = badge.corners.as_ref().map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
                .unwrap_or([metrics.radius as u32; 4]);
            rounded_rect_path(
                border.width as f32 / 2.0,
                border.width as f32 / 2.0,
                total_width as f32 - border.width as f32,
                height - border.width as f32,
                corners
            )
        };
        
        svg.push_str(&format!(
            r#"<path d="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
            border_path, border.color, border.width
        ));
    }
    
    // Apply plastic style effects
    if badge.style.has_gradients() {
        svg.push_str(&format!(
            r#"<path d="{}" fill="url(#bg-gradient)" opacity="0.1"/>"#,
            rounded_rect_path(0.0, 0.0, total_width as f32, height, [metrics.radius as u32; 4])
        ));
    }
    
    // Outline mode
    if badge.outline {
        svg.push_str(&format!(
            r#"<path d="{}" fill="none" stroke="{}" stroke-width="2"/>"#,
            rounded_rect_path(1.0, 1.0, total_width as f32 - 2.0, height - 2.0, [metrics.radius as u32; 4]),
            bg_color
        ));
    }
    
    svg.push_str("</svg>");
    svg
}

/// Render text-only badge (no icon available)
fn render_text_only(badge: &TechBadge, metrics: &SvgMetrics, label: &str) -> String {
    let height = metrics.height;
    let label_width = estimate_text_width(label) + 24; // Extra padding for text-only
    let font_size = calculate_font_size(badge.style);
    
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        label_width, height as u32, label_width, height as u32
    );
    
    // Background
    let bg_color = badge.effective_bg_color().unwrap_or_else(|| "#555".to_string());
    
    if let Some(chevron) = &badge.chevron {
        let (path, _, _) = chevron_path_with_overlap(
            0.0, 0.0, label_width as f32, height, chevron.direction, chevron.depth
        );
        svg.push_str(&format!(r#"<path d="{}" fill="{}"/>"#, path, bg_color));
    } else {
        let corners = badge.corners.as_ref().map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
            .unwrap_or([metrics.radius as u32; 4]);
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            rounded_rect_path(0.0, 0.0, label_width as f32, height, corners),
            bg_color
        ));
    }
    
    // Text
    let text_color = badge.text_color.as_deref()
        .unwrap_or_else(|| mdfx_colors::contrast_color(&bg_color));
    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");
    
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
        label_width as f32 / 2.0,
        height / 2.0,
        font_family,
        font_size,
        text_color,
        escape_xml(label)
    ));
    
    svg.push_str("</svg>");
    svg
}

/// Estimate text width in pixels (rough approximation)
fn estimate_text_width(text: &str) -> u32 {
    (text.chars().count() as f32 * 6.5) as u32
}

/// Darken a hex color by the specified amount
fn darken_color(hex: &str, amount: f32) -> String {
    mdfx_colors::darken(hex, amount)
}

/// Create gradient definitions for plastic style
fn create_gradient_defs() -> String {
    r#"<defs>
        <linearGradient id="bg-gradient" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:rgba(255,255,255,0.15);stop-opacity:1"/>
            <stop offset="100%" style="stop-color:rgba(0,0,0,0.15);stop-opacity:1"/>
        </linearGradient>
    </defs>"#.to_string()
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Render badge to file
pub fn render_to_file(badge: &TechBadge, path: impl AsRef<Path>) -> io::Result<()> {
    let svg = render(badge);
    std::fs::write(path, svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::badge::BadgeBuilder;
    use crate::style::BadgeStyle;

    #[test]
    fn test_render_with_icon() {
        let badge = BadgeBuilder::new("rust").build();
        let svg = render(&badge);
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Rust"));
    }

    #[test]
    fn test_render_text_only() {
        let badge = BadgeBuilder::new("unknown-tech").build();
        let svg = render(&badge);
        
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Unknown-tech"));
    }

    #[test]
    fn test_custom_styling() {
        let badge = BadgeBuilder::new("typescript")
            .label("TypeScript v5.0")
            .style(BadgeStyle::Plastic)
            .bg_color("#3178C6")
            .text_color("#FFFFFF")
            .build();
        
        let svg = render(&badge);
        assert!(svg.contains("TypeScript v5.0"));
        assert!(svg.contains("#3178C6"));
    }

    #[test]
    fn test_estimate_text_width() {
        assert_eq!(estimate_text_width("Rust"), 26); // 4 chars * 6.5
        assert_eq!(estimate_text_width("TypeScript"), 65); // 10 chars * 6.5
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("C++ & Rust"), "C++ &amp; Rust");
        assert_eq!(escape_xml("<script>"), "&lt;script&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_font_sizes() {
        assert_eq!(calculate_font_size(BadgeStyle::Flat), 10);
        assert_eq!(calculate_font_size(BadgeStyle::ForTheBadge), 11);
        assert_eq!(calculate_font_size(BadgeStyle::Social), 11);
    }

    #[test]
    fn test_render_with_chevron() {
        let badge = BadgeBuilder::new("rust")
            .chevron(crate::style::Chevron::right(8.0))
            .build();
        
        let svg = render(&badge);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Rust"));
    }

    #[test]
    fn test_render_with_border() {
        let badge = BadgeBuilder::new("python")
            .border("#FF0000", 2)
            .build();
        
        let svg = render(&badge);
        assert!(svg.contains("stroke=\"#FF0000\""));
        assert!(svg.contains("stroke-width=\"2\""));
    }
}