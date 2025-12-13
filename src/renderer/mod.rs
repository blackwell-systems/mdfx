/// Multi-backend rendering system for visual primitives.
///
/// This module defines the trait-based architecture for rendering primitives
/// to different output formats (shields.io URLs, local SVG files, etc.).

pub mod shields;

use crate::primitive::Primitive;
use crate::error::Result;

/// Represents the output of rendering a primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderedAsset {
    /// Inline Markdown (e.g., shields.io URL wrapped in ![](url))
    InlineMarkdown(String),

    /// File-based asset (e.g., generated SVG file)
    FileAsset {
        /// Relative path to the generated file
        path: String,
        /// Markdown to embed the file (e.g., ![](path))
        markdown: String,
    },
}

impl RenderedAsset {
    /// Get the Markdown representation of this asset
    pub fn to_markdown(&self) -> &str {
        match self {
            RenderedAsset::InlineMarkdown(md) => md,
            RenderedAsset::FileAsset { markdown, .. } => markdown,
        }
    }

    /// Check if this asset requires a file to be written
    pub fn is_file_based(&self) -> bool {
        matches!(self, RenderedAsset::FileAsset { .. })
    }
}

/// Trait for rendering primitives to output formats.
///
/// Implementations handle backend-specific logic:
/// - ShieldsBackend: Generates shields.io URLs
/// - SvgBackend (future): Generates local SVG files
pub trait Renderer {
    /// Render a primitive to an asset (inline or file-based)
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendered_asset_inline() {
        let asset = RenderedAsset::InlineMarkdown("![](https://example.com)".to_string());
        assert_eq!(asset.to_markdown(), "![](https://example.com)");
        assert!(!asset.is_file_based());
    }

    #[test]
    fn test_rendered_asset_file() {
        let asset = RenderedAsset::FileAsset {
            path: "assets/badge.svg".to_string(),
            markdown: "![](assets/badge.svg)".to_string(),
        };
        assert_eq!(asset.to_markdown(), "![](assets/badge.svg)");
        assert!(asset.is_file_based());
    }
}
