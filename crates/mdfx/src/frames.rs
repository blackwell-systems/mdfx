use crate::error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Frame renderer for adding decorative elements around text
pub struct FrameRenderer {
    frames: HashMap<String, FrameStyle>,
}

/// A frame style definition
#[derive(Debug, Clone, Deserialize)]
pub struct FrameStyle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prefix: String,
    pub suffix: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Frame data loaded from frames.json
#[derive(Debug, Deserialize)]
struct FramesData {
    #[allow(dead_code)]
    version: String,
    frames: HashMap<String, FrameStyle>,
}

impl FrameRenderer {
    /// Create a new frame renderer by loading frames.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/frames.json");
        let frames_data: FramesData = serde_json::from_str(data)
            .map_err(|e| Error::ParseError(format!("Failed to parse frames.json: {}", e)))?;

        Ok(FrameRenderer {
            frames: frames_data.frames,
        })
    }

    /// Apply a frame around text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to frame
    /// * `frame_style` - The frame style ID or alias (e.g., "gradient", "solid-left")
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::FrameRenderer;
    ///
    /// let renderer = FrameRenderer::new().unwrap();
    /// let result = renderer.apply_frame("Title", "gradient").unwrap();
    /// assert_eq!(result, "▓▒░ Title ░▒▓");
    /// ```
    pub fn apply_frame(&self, text: &str, frame_style: &str) -> Result<String> {
        let style = self.get_frame(frame_style)?;
        Ok(format!("{}{}{}", style.prefix, text, style.suffix))
    }

    /// Get a frame style by ID or alias
    pub fn get_frame(&self, name: &str) -> Result<&FrameStyle> {
        // First try direct lookup
        if let Some(frame) = self.frames.get(name) {
            return Ok(frame);
        }

        // Then try aliases
        self.frames
            .values()
            .find(|f| f.aliases.contains(&name.to_string()))
            .ok_or_else(|| Error::UnknownFrame(name.to_string()))
    }

    /// Check if a frame style exists
    pub fn has_frame(&self, name: &str) -> bool {
        self.get_frame(name).is_ok()
    }

    /// List all available frames
    pub fn list_frames(&self) -> Vec<&FrameStyle> {
        let mut frames: Vec<_> = self.frames.values().collect();
        frames.sort_by(|a, b| a.id.cmp(&b.id));
        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_renderer_new() {
        let renderer = FrameRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_apply_gradient_frame() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Test", "gradient").unwrap();
        assert_eq!(result, "▓▒░ Test ░▒▓");
    }

    #[test]
    fn test_apply_solid_left_frame() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Important", "solid-left").unwrap();
        assert_eq!(result, "█▌Important");
    }

    #[test]
    fn test_apply_dashed_frame() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Section", "dashed").unwrap();
        assert_eq!(result, "━━━ Section ━━━");
    }

    #[test]
    fn test_frame_alias() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Test", "grad").unwrap();
        assert_eq!(result, "▓▒░ Test ░▒▓");
    }

    #[test]
    fn test_unknown_frame() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Test", "invalid-frame");
        assert!(result.is_err());
    }

    #[test]
    fn test_has_frame() {
        let renderer = FrameRenderer::new().unwrap();
        assert!(renderer.has_frame("gradient"));
        assert!(renderer.has_frame("grad"));
        assert!(!renderer.has_frame("nonexistent"));
    }

    #[test]
    fn test_list_frames() {
        let renderer = FrameRenderer::new().unwrap();
        let frames = renderer.list_frames();
        assert!(!frames.is_empty());
    }
}
