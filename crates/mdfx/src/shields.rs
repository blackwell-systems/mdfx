use crate::error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Shields renderer for generating shields.io badge Markdown
///
/// ShieldsRenderer creates shields.io badge URLs as Markdown image links.
/// These badges serve as design primitives: dividers, progress bars, headers, icons.
///
/// Unlike styles (character transformation) or frames (prefix/suffix),
/// shields generate external image links that render as visual elements in Markdown.
pub struct ShieldsRenderer {
    palette: HashMap<String, String>,
    styles: HashMap<String, ShieldStyle>,
}

/// A shield style definition
#[derive(Debug, Clone, Deserialize)]
pub struct ShieldStyle {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Intermediate structure to parse registry.json for shields
#[derive(Debug, Deserialize)]
struct RegistryShieldsExtract {
    palette: HashMap<String, String>,
    shield_styles: HashMap<String, ShieldStyle>,
}

impl ShieldsRenderer {
    /// Create a new shields renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/registry.json");
        let registry: RegistryShieldsExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!("Failed to parse registry.json for shields: {}", e))
        })?;

        Ok(ShieldsRenderer {
            palette: registry.palette,
            styles: registry.shield_styles,
        })
    }

    /// Render a single color block
    ///
    /// # Arguments
    ///
    /// * `color` - Palette name or hex code (e.g., "cobalt" or "2B6CB0")
    /// * `style` - Shield style ("flat-square", "for-the-badge", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_block("2B6CB0", "flat-square").unwrap();
    /// assert!(result.contains("https://img.shields.io/badge/"));
    /// ```
    pub fn render_block(&self, color: &str, style: &str) -> Result<String> {
        let resolved_color = self.resolve_color(color)?;
        let resolved_style = self.resolve_style(style)?;

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}",
            resolved_color, resolved_style
        );

        Ok(format!("![]({})", url))
    }

    /// Render a colored block with a text label
    ///
    /// # Arguments
    ///
    /// * `color` - Background color (palette name or hex)
    /// * `label` - Text to display on the badge
    /// * `style` - Shield style
    pub fn render_labeled_block(&self, color: &str, label: &str, style: &str) -> Result<String> {
        let resolved_color = self.resolve_color(color)?;
        let resolved_style = self.resolve_style(style)?;

        // URL-encode the label for safe inclusion in URL
        let encoded_label = label
            .replace(' ', "%20")
            .replace('-', "--")
            .replace('_', "__");

        let url = format!(
            "https://img.shields.io/badge/-{}-{}?style={}",
            encoded_label, resolved_color, resolved_style
        );

        Ok(format!("![]({})", url))
    }

    /// Render a two-tone block (left/right colors)
    ///
    /// # Arguments
    ///
    /// * `left_color` - Left side color (palette name or hex)
    /// * `right_color` - Right side color (palette name or hex)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_twotone("111111", "2B6CB0", "flat-square").unwrap();
    /// assert!(result.contains("labelColor=111111"));
    /// assert!(result.contains("-2B6CB0?"));  // Right color appears in badge path
    /// ```
    pub fn render_twotone(
        &self,
        left_color: &str,
        right_color: &str,
        style: &str,
    ) -> Result<String> {
        let left = self.resolve_color(left_color)?;
        let right = self.resolve_color(right_color)?;
        let resolved_style = self.resolve_style(style)?;

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}&label=&labelColor={}",
            right, resolved_style, left
        );

        Ok(format!("![]({})", url))
    }

    /// Render a bar of multiple colored blocks
    ///
    /// # Arguments
    ///
    /// * `colors` - Slice of colors (palette names or hex codes)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let colors = vec!["22C55E".to_string(), "F59E0B".to_string()];
    /// let result = renderer.render_bar(&colors, "flat-square").unwrap();
    /// assert!(result.contains("22C55E"));
    /// assert!(result.contains("F59E0B"));
    /// ```
    pub fn render_bar(&self, colors: &[String], style: &str) -> Result<String> {
        let resolved_style = self.resolve_style(style)?;
        let mut blocks = Vec::new();

        for color in colors {
            let resolved_color = self.resolve_color(color)?;
            let url = format!(
                "https://img.shields.io/badge/-%20-{}?style={}",
                resolved_color, resolved_style
            );
            blocks.push(format!("![]({})", url));
        }

        Ok(blocks.join(""))
    }

    /// Render a multi-color bar with optional separator between blocks
    ///
    /// # Arguments
    ///
    /// * `colors` - Slice of colors (palette names or hex codes)
    /// * `style` - Shield style
    /// * `separator` - Optional separator string between blocks (e.g., " " for space)
    pub fn render_bar_with_separator(
        &self,
        colors: &[String],
        style: &str,
        separator: Option<&str>,
    ) -> Result<String> {
        let resolved_style = self.resolve_style(style)?;
        let mut blocks = Vec::new();

        for color in colors {
            let resolved_color = self.resolve_color(color)?;
            let url = format!(
                "https://img.shields.io/badge/-%20-{}?style={}",
                resolved_color, resolved_style
            );
            blocks.push(format!("![]({})", url));
        }

        Ok(blocks.join(separator.unwrap_or("")))
    }

    /// Render an icon chip (logo-only badge)
    ///
    /// # Arguments
    ///
    /// * `logo` - Simple Icons slug (e.g., "rust", "python", "amazonaws")
    /// * `bg_color` - Background color (palette name or hex)
    /// * `logo_color` - Logo color (palette name or hex)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_icon("rust", "000000", "white", "flat-square").unwrap();
    /// assert!(result.contains("logo=rust"));
    /// assert!(result.contains("logoColor="));
    /// ```
    pub fn render_icon(
        &self,
        logo: &str,
        bg_color: &str,
        logo_color: &str,
        style: &str,
    ) -> Result<String> {
        self.render_icon_with_size(logo, bg_color, logo_color, style, None)
    }

    /// Render an icon chip with optional logo size
    ///
    /// # Arguments
    ///
    /// * `logo` - Simple Icons slug (e.g., "rust", "python", "amazonaws")
    /// * `bg_color` - Background color (palette name or hex)
    /// * `logo_color` - Logo color (palette name or hex)
    /// * `style` - Shield style
    /// * `logo_size` - Optional logo size ("auto" for adaptive sizing)
    pub fn render_icon_with_size(
        &self,
        logo: &str,
        bg_color: &str,
        logo_color: &str,
        style: &str,
        logo_size: Option<&str>,
    ) -> Result<String> {
        let bg = self.resolve_color(bg_color)?;
        let logo_col = self.resolve_color(logo_color)?;
        let resolved_style = self.resolve_style(style)?;

        let logo_size_param = match logo_size {
            Some(size) => format!("&logoSize={}", size),
            None => String::new(),
        };

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}&logo={}&logoColor={}&label=&labelColor={}{}",
            bg, resolved_style, logo, logo_col, bg, logo_size_param
        );

        Ok(format!("![]({})", url))
    }

    /// Render an icon chip with a text label
    ///
    /// # Arguments
    ///
    /// * `logo` - Simple Icons slug (e.g., "rust", "python")
    /// * `label` - Text to display on the badge
    /// * `bg_color` - Background color (palette name or hex)
    /// * `logo_color` - Logo color (palette name or hex)
    /// * `style` - Shield style
    pub fn render_icon_with_label(
        &self,
        logo: &str,
        label: &str,
        bg_color: &str,
        logo_color: &str,
        style: &str,
    ) -> Result<String> {
        let bg = self.resolve_color(bg_color)?;
        let logo_col = self.resolve_color(logo_color)?;
        let resolved_style = self.resolve_style(style)?;

        // URL encode the label for spaces and special chars
        let encoded_label = label.replace(' ', "%20");

        let url = format!(
            "https://img.shields.io/badge/-{}-{}?style={}&logo={}&logoColor={}",
            encoded_label, bg, resolved_style, logo, logo_col
        );

        Ok(format!("![]({})", url))
    }

    /// Resolve a color from palette name or pass through hex code
    ///
    /// # Arguments
    ///
    /// * `color` - Palette name (e.g., "cobalt") or hex code (e.g., "2B6CB0")
    ///
    /// # Returns
    ///
    /// Hex code (6 characters, no #)
    pub fn resolve_color(&self, color: &str) -> Result<String> {
        // Try palette lookup first
        if let Some(hex) = self.palette.get(color) {
            return Ok(hex.clone());
        }

        // Validate as hex code (must be 6 hex digits)
        if color.len() == 6 && color.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(color.to_uppercase());
        }

        Err(Error::InvalidColor(format!(
            "{} (use palette name or 6-digit hex)",
            color
        )))
    }

    /// Resolve a style by ID or alias
    fn resolve_style(&self, style: &str) -> Result<String> {
        // Try direct lookup
        if self.styles.contains_key(style) {
            return Ok(style.to_string());
        }

        // Try aliases
        for shield_style in self.styles.values() {
            if shield_style.aliases.contains(&style.to_string()) {
                return Ok(shield_style.id.clone());
            }
        }

        Err(Error::UnknownShieldStyle(style.to_string()))
    }

    /// Check if a shield style exists
    pub fn has_style(&self, name: &str) -> bool {
        self.resolve_style(name).is_ok()
    }

    /// List all available shield styles
    pub fn list_styles(&self) -> Vec<&ShieldStyle> {
        let mut styles: Vec<_> = self.styles.values().collect();
        styles.sort_by(|a, b| a.id.cmp(&b.id));
        styles
    }

    /// List all palette colors
    pub fn list_palette(&self) -> Vec<(&String, &String)> {
        let mut colors: Vec<_> = self.palette.iter().collect();
        colors.sort_by(|a, b| a.0.cmp(b.0));
        colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Basic Setup Tests
    // ========================================================================

    #[test]
    fn test_shields_renderer_new() {
        let renderer = ShieldsRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_list_styles() {
        let renderer = ShieldsRenderer::new().unwrap();
        let styles = renderer.list_styles();
        assert!(!styles.is_empty());
    }

    #[test]
    fn test_list_palette() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = renderer.list_palette();
        assert!(!colors.is_empty());
        assert!(colors.iter().any(|(name, _)| *name == "cobalt"));
    }

    // ========================================================================
    // Color Resolution (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("cobalt", "2B6CB0")]
    #[case("pink", "F41C80")]
    #[case("abc123", "ABC123")]
    #[case("FFFFFF", "FFFFFF")]
    #[case("000000", "000000")]
    fn test_resolve_color(#[case] input: &str, #[case] expected: &str) {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.resolve_color(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("invalid")]
    #[case("not-a-color")]
    #[case("GGGGGG")]
    #[case("12345")]
    fn test_resolve_color_invalid(#[case] input: &str) {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.resolve_color(input);
        assert!(result.is_err());
    }

    // ========================================================================
    // Style Existence (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("flat-square", true)]
    #[case("flat", true)]
    #[case("square", true)] // alias
    #[case("for-the-badge", true)]
    #[case("nonexistent", false)]
    fn test_has_style(#[case] style: &str, #[case] expected: bool) {
        let renderer = ShieldsRenderer::new().unwrap();
        assert_eq!(renderer.has_style(style), expected);
    }

    // ========================================================================
    // Render Block Tests
    // ========================================================================

    #[test]
    fn test_render_block() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block("2B6CB0", "flat-square").unwrap();
        assert!(result.contains("https://img.shields.io/badge/"));
        assert!(result.contains("2B6CB0"));
        assert!(result.contains("style=flat-square"));
    }

    #[test]
    fn test_render_block_with_palette_name() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block("cobalt", "flat-square").unwrap();
        assert!(result.contains("2B6CB0"));
    }

    #[test]
    fn test_style_alias() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block("cobalt", "square").unwrap();
        assert!(result.contains("style=flat-square"));

        let result_flat = renderer.render_block("cobalt", "flat").unwrap();
        assert!(result_flat.contains("style=flat"));
    }

    // ========================================================================
    // Render Twotone Tests
    // ========================================================================

    #[test]
    fn test_render_twotone() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_twotone("111111", "2B6CB0", "flat-square")
            .unwrap();
        assert!(result.contains("labelColor=111111"));
        assert!(result.contains("-2B6CB0?"));
    }

    // ========================================================================
    // Render Bar Tests
    // ========================================================================

    #[test]
    fn test_render_bar() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec![
            "22C55E".to_string(),
            "F59E0B".to_string(),
            "334155".to_string(),
        ];
        let result = renderer.render_bar(&colors, "flat-square").unwrap();
        assert!(result.contains("22C55E"));
        assert!(result.contains("F59E0B"));
        assert!(result.contains("334155"));
        assert_eq!(result.matches("![](").count(), 3);
    }

    #[test]
    fn test_render_bar_with_separator() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec!["FF0000".to_string(), "00FF00".to_string()];
        let result = renderer
            .render_bar_with_separator(&colors, "flat-square", Some(" "))
            .unwrap();
        assert!(result.contains("FF0000"));
        assert!(result.contains("00FF00"));
        assert!(result.contains(") ![]("));
    }

    #[test]
    fn test_render_bar_with_separator_none() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec!["FF0000".to_string(), "00FF00".to_string()];
        let result = renderer
            .render_bar_with_separator(&colors, "flat-square", None)
            .unwrap();
        assert!(result.contains(")![]("));
    }

    // ========================================================================
    // Render Icon Tests
    // ========================================================================

    #[test]
    fn test_render_icon() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon("rust", "000000", "FFFFFF", "flat-square")
            .unwrap();
        assert!(result.contains("logo=rust"));
        assert!(result.contains("logoColor=FFFFFF"));
        assert!(result.contains("000000"));
    }

    #[test]
    fn test_render_icon_with_logo_size() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon_with_size("rust", "000000", "FFFFFF", "flat-square", Some("auto"))
            .unwrap();
        assert!(result.contains("logo=rust"));
        assert!(result.contains("logoSize=auto"));
    }

    #[test]
    fn test_render_icon_without_logo_size() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon_with_size("rust", "000000", "FFFFFF", "flat-square", None)
            .unwrap();
        assert!(result.contains("logo=rust"));
        assert!(!result.contains("logoSize="));
    }

    #[test]
    fn test_render_icon_with_label() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon_with_label("rust", "Rust", "DEA584", "FFFFFF", "flat-square")
            .unwrap();
        assert!(result.contains("logo=rust"));
        assert!(result.contains("-Rust-"));
        assert!(result.contains("DEA584"));
        assert!(result.contains("logoColor=FFFFFF"));
    }

    #[test]
    fn test_render_icon_with_label_spaces() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon_with_label("python", "Python 3", "3776AB", "FFFFFF", "flat-square")
            .unwrap();
        assert!(result.contains("Python%203"));
    }

    // ========================================================================
    // Render Labeled Block Tests
    // ========================================================================

    #[test]
    fn test_render_labeled_block() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_labeled_block("2B6CB0", "Hello World", "flat-square")
            .unwrap();
        assert!(result.contains("https://img.shields.io/badge/"));
        assert!(result.contains("Hello%20World"));
        assert!(result.contains("2B6CB0"));
    }

    #[test]
    fn test_render_labeled_block_special_chars() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_labeled_block("FF0000", "test-value_here", "flat-square")
            .unwrap();
        assert!(result.contains("test--value__here"));
    }

    // ========================================================================
    // Error Cases (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("FF0000", "nonexistent-style")]
    #[case("not-a-color", "flat-square")]
    fn test_render_block_errors(#[case] color: &str, #[case] style: &str) {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block(color, style);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_twotone_invalid_style() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_twotone("FF0000", "00FF00", "bad-style");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_bar_invalid_color() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec!["FF0000".to_string(), "invalid".to_string()];
        let result = renderer.render_bar(&colors, "flat-square");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_labeled_block_invalid_style() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_labeled_block("FF0000", "Test", "bad-style");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_icon_invalid_color() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_icon("rust", "invalid", "FFFFFF", "flat-square");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_icon_with_label_invalid_style() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result =
            renderer.render_icon_with_label("rust", "Rust", "000000", "FFFFFF", "bad-style");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_bar_with_separator_invalid_style() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec!["FF0000".to_string()];
        let result = renderer.render_bar_with_separator(&colors, "invalid", None);
        assert!(result.is_err());
    }
}
