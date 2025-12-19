//! Tech group component handler
//!
//! Automatically applies corner presets to a group of badges:
//! - First badge: corners=left (rounded left, square right)
//! - Middle badges: corners=none (square corners)
//! - Last badge: corners=right (square left, rounded right)
//!
//! Supports: tech, version, and license badges.
//!
//! Style inheritance: params set on the group are inherited by child badges
//! unless the badge specifies its own value.
//!
//! This creates a seamless "pill" group when badges are placed side-by-side.

use crate::components::ComponentOutput;
use crate::error::Result;
use std::collections::HashMap;

/// Component types that can participate in badge groups
const GROUPABLE_COMPONENTS: &[&str] = &["tech", "version", "license"];

/// Parameters that can be inherited from group to child badges
const INHERITABLE_PARAMS: &[&str] = &[
    "bg",
    "border",
    "border_width",
    "text_color",
    "text",
    "color",
    "logo",
    "font",
    "font_family",
    "style",
    "divider",
    "raised",
    "logo_size",
    "icon_size",
];

/// Find all groupable component invocations in content.
/// Returns (start, end) positions for each match.
fn find_groupable_components(content: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut pos = 0;

    while let Some(start_rel) = content[pos..].find("{{ui:") {
        let start = pos + start_rel;

        // Find the closing }}
        if let Some(end_rel) = content[start..].find("}}") {
            let end = start + end_rel + 2;

            // Extract component type (between "{{ui:" and next ":" or "/" or "}")
            let after_prefix = &content[start + 5..end];
            let type_end = after_prefix
                .find(|c| c == ':' || c == '/' || c == '}')
                .unwrap_or(after_prefix.len());
            let comp_type = &after_prefix[..type_end];

            if GROUPABLE_COMPONENTS.contains(&comp_type) {
                results.push((start, end));
            }

            pos = end;
        } else {
            break;
        }
    }

    results
}

/// Handle tech-group component expansion
///
/// Transforms content containing badges to automatically apply
/// appropriate corner presets for a connected badge group.
///
/// Style inheritance: Any params on the group (bg, border, text_color, etc.)
/// are inherited by child badges unless the badge specifies its own value.
pub fn handle(params: &HashMap<String, String>, content: Option<&str>) -> Result<ComponentOutput> {
    let content = content.unwrap_or("");

    // Find all groupable component invocations
    let matches = find_groupable_components(content);
    let count = matches.len();

    if count == 0 {
        // No tech badges found, return content as-is
        return Ok(ComponentOutput::Template(content.to_string()));
    }

    // Extract gap parameter (spacing between badges in pixels, for row layout)
    let gap = params
        .get("gap")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    // Collect inheritable params from group
    let inherited: Vec<(&str, &str)> = INHERITABLE_PARAMS
        .iter()
        .filter_map(|&key| params.get(key).map(|v| (key, v.as_str())))
        .collect();

    let mut result = content.to_string();

    // Process badges in reverse order to preserve positions
    for (i, &(start, end)) in matches.iter().enumerate().rev() {
        let full_match = &content[start..end];

        // Determine which corner preset to apply
        let corner_preset = if count == 1 {
            // Single badge: keep all corners rounded (no modification needed)
            None
        } else if i == 0 {
            // First badge: rounded left, square right
            Some("left")
        } else if i == count - 1 {
            // Last badge: square left, rounded right
            Some("right")
        } else {
            // Middle badges: no rounded corners
            Some("none")
        };

        // Build params to inject (inherited + corners)
        let mut inject_params = String::new();

        // Add inherited params if not already specified in the badge
        for (key, value) in &inherited {
            let check_key = format!("{}=", key);
            if !full_match.contains(&check_key) {
                inject_params.push_str(&format!(":{}={}", key, value));
            }
        }

        // Add corners if needed
        if let Some(preset) = corner_preset {
            if !full_match.contains("corners=") {
                inject_params.push_str(&format!(":corners={}", preset));
            }
        }

        // Apply modifications if we have any params to inject
        if !inject_params.is_empty() {
            let modified = if let Some(inner) = full_match.strip_suffix("/}}") {
                // Self-closing tag
                format!("{}{}/}}}}", inner, inject_params)
            } else if let Some(inner) = full_match.strip_suffix("}}") {
                // Regular closing tag
                format!("{}{}}}}}", inner, inject_params)
            } else {
                full_match.to_string()
            };

            result.replace_range(start..end, &modified);
        }
    }

    // If gap is specified, wrap in row for proper alignment
    if gap > 0 {
        // Add spacing between badges using row component
        result = format!("{{{{ui:row}}}}{}{}", result, "{{/ui}}");
    }

    Ok(ComponentOutput::Template(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_badge_unchanged() {
        let params = HashMap::new();
        let content = "{{ui:tech:rust/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // Single badge should not have corners modified
            assert_eq!(template, "{{ui:tech:rust/}}");
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_two_badges_corners() {
        let params = HashMap::new();
        let content = "{{ui:tech:rust/}}{{ui:tech:typescript/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            assert!(template.contains("corners=left"));
            assert!(template.contains("corners=right"));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_three_badges_corners() {
        let params = HashMap::new();
        let content = "{{ui:tech:rust/}}{{ui:tech:typescript/}}{{ui:tech:docker/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            assert!(template.contains("corners=left"));
            assert!(template.contains("corners=none"));
            assert!(template.contains("corners=right"));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_existing_corners_not_overwritten() {
        let params = HashMap::new();
        let content = "{{ui:tech:rust:corners=all/}}{{ui:tech:typescript/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // First badge already has corners, should not be modified
            assert!(template.contains("corners=all"));
            assert!(template.contains("corners=right"));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_style_inheritance() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "1a1a2e".to_string());
        params.insert("border".to_string(), "00ff00".to_string());
        let content = "{{ui:tech:rust/}}{{ui:tech:go/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // Both badges should inherit bg and border
            assert!(template.contains("bg=1a1a2e"));
            assert!(template.contains("border=00ff00"));
            // Should appear twice (once per badge)
            assert_eq!(template.matches("bg=1a1a2e").count(), 2);
            assert_eq!(template.matches("border=00ff00").count(), 2);
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_style_inheritance_with_override() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "1a1a2e".to_string());
        let content = "{{ui:tech:rust/}}{{ui:tech:go:bg=custom/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // First badge inherits bg
            assert!(template.contains("rust") && template.contains("bg=1a1a2e"));
            // Second badge keeps its own bg=custom (not overwritten)
            assert!(template.contains("bg=custom"));
            // Inherited bg should only appear once (not on the overridden badge)
            assert_eq!(template.matches("bg=1a1a2e").count(), 1);
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_style_inheritance_single_badge() {
        let mut params = HashMap::new();
        params.insert("border".to_string(), "ff0000".to_string());
        let content = "{{ui:tech:rust/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // Single badge should still inherit styles
            assert!(template.contains("border=ff0000"));
            // But should not have corners modified
            assert!(!template.contains("corners="));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_mixed_badge_types() {
        let params = HashMap::new();
        let content = "{{ui:version:1.0.0/}}{{ui:tech:rust/}}{{ui:license:MIT/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // First (version): left corners
            assert!(template.contains("version:1.0.0:corners=left"));
            // Middle (tech): no corners
            assert!(template.contains("tech:rust:corners=none"));
            // Last (license): right corners
            assert!(template.contains("license:MIT:corners=right"));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_mixed_with_style_inheritance() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "1a1a2e".to_string());
        let content = "{{ui:version:2.0.0/}}{{ui:tech:docker/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // Both should inherit bg
            assert_eq!(template.matches("bg=1a1a2e").count(), 2);
            // Corner assignments
            assert!(template.contains("corners=left"));
            assert!(template.contains("corners=right"));
        } else {
            panic!("Expected Template output");
        }
    }

    #[test]
    fn test_ignores_non_groupable_components() {
        let params = HashMap::new();
        let content = "{{ui:swatch:ff0000/}}{{ui:tech:rust/}}";

        let result = handle(&params, Some(content)).unwrap();

        if let ComponentOutput::Template(template) = result {
            // Only tech badge should be found (single badge = no corners)
            assert!(!template.contains("corners="));
            // Swatch should be unchanged
            assert!(template.contains("{{ui:swatch:ff0000/}}"));
        } else {
            panic!("Expected Template output");
        }
    }
}
