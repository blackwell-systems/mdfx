//! Test utilities and macros for mdfx
//!
//! Provides helper macros and shared fixtures to reduce boilerplate in tests.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::test_utils::*;
//!
//! // Simple input -> expected test
//! test_process!("{{mathbold}}HELLO{{/mathbold}}" => "𝐇𝐄𝐋𝐋𝐎");
//!
//! // Test that processing fails
//! test_process_err!("{{unknown}}text{{/unknown}}");
//!
//! // Test with custom name
//! test_process!(test_bold_text, "{{mathbold}}X{{/mathbold}}" => "𝐗");
//! ```

use lazy_static::lazy_static;

use crate::Converter;
use crate::Registry;
use crate::ShieldsRenderer;

// ============================================================================
// Shared Test Fixtures (lazy static singletons)
// ============================================================================
//
// Note: TemplateParser and ComponentsRenderer contain Box<dyn Renderer> which
// is not Sync, so they cannot be shared via lazy_static. Tests that need these
// should create their own instances or use the macros which create per-test instances.

lazy_static! {
    /// Shared Converter instance for tests
    pub static ref TEST_CONVERTER: Converter = Converter::new().expect("Failed to create test converter");

    /// Shared Registry instance for tests
    pub static ref TEST_REGISTRY: Registry = Registry::new().expect("Failed to create test registry");

    /// Shared ShieldsRenderer instance for tests
    pub static ref TEST_SHIELDS: ShieldsRenderer = ShieldsRenderer::new().expect("Failed to create test shields renderer");
}

// ============================================================================
// Parser Test Macros
// ============================================================================

/// Test that processing input produces expected output.
///
/// # Variants
///
/// - `test_process!(input => expected)` - anonymous test assertion
/// - `test_process!(name, input => expected)` - named test function
///
/// # Example
///
/// ```rust,ignore
/// test_process!("{{mathbold}}A{{/mathbold}}" => "𝐀");
/// ```
#[macro_export]
macro_rules! test_process {
    // Anonymous assertion (use inside a test function)
    ($input:expr => $expected:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert_eq!(result, $expected, "Input: {}", $input);
    }};

    // Named test function
    ($name:ident, $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input).expect("Processing failed");
            assert_eq!(result, $expected, "Input: {}", $input);
        }
    };
}

/// Test that processing input fails with an error.
///
/// # Example
///
/// ```rust,ignore
/// test_process_err!("{{unknown}}text{{/unknown}}");
/// ```
#[macro_export]
macro_rules! test_process_err {
    // Anonymous assertion
    ($input:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input);
        assert!(result.is_err(), "Expected error for input: {}", $input);
    }};

    // Named test function
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input);
            assert!(result.is_err(), "Expected error for input: {}", $input);
        }
    };
}

/// Test that input is preserved unchanged (useful for code blocks, etc.)
///
/// # Example
///
/// ```rust,ignore
/// test_process_unchanged!("```\n{{mathbold}}CODE{{/mathbold}}\n```");
/// ```
#[macro_export]
macro_rules! test_process_unchanged {
    ($input:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert_eq!(result, $input, "Expected unchanged output for: {}", $input);
    }};

    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input).expect("Processing failed");
            assert_eq!(result, $input, "Expected unchanged output for: {}", $input);
        }
    };
}

/// Test multiple input/output pairs in a single test.
///
/// # Example
///
/// ```rust,ignore
/// test_process_cases!(
///     "{{mathbold}}A{{/mathbold}}" => "𝐀",
///     "{{mathbold}}B{{/mathbold}}" => "𝐁",
///     "{{mathbold}}C{{/mathbold}}" => "𝐂",
/// );
/// ```
#[macro_export]
macro_rules! test_process_cases {
    ($($input:expr => $expected:expr),+ $(,)?) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        $(
            let result = parser.process($input).expect(&format!("Processing failed for: {}", $input));
            assert_eq!(result, $expected, "Input: {}", $input);
        )+
    }};
}

/// Test that processing output contains expected substrings.
///
/// # Example
///
/// ```rust,ignore
/// test_process_contains!("{{ui:swatch:pink/}}" => ["![](", "F41C80"]);
/// ```
#[macro_export]
macro_rules! test_process_contains {
    ($input:expr => [$($expected:expr),+ $(,)?]) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        $(
            assert!(
                result.contains($expected),
                "Expected output to contain '{}'\nInput: {}\nGot: {}",
                $expected, $input, result
            );
        )+
    }};
}

/// Test that processing output does NOT contain certain substrings.
///
/// # Example
///
/// ```rust,ignore
/// test_process_not_contains!("input" => ["should not appear"]);
/// ```
#[macro_export]
macro_rules! test_process_not_contains {
    ($input:expr => [$($unexpected:expr),+ $(,)?]) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        $(
            assert!(
                !result.contains($unexpected),
                "Output should NOT contain '{}'\nInput: {}\nGot: {}",
                $unexpected, $input, result
            );
        )+
    }};
}

/// Test output starts with prefix and ends with suffix.
///
/// # Example
///
/// ```rust,ignore
/// test_process_bookends!("{{frame:gradient}}X{{/}}" => starts "▓", ends "▓");
/// ```
#[macro_export]
macro_rules! test_process_bookends {
    ($input:expr => starts $prefix:expr, ends $suffix:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert!(
            result.starts_with($prefix),
            "Expected output to start with '{}'\nInput: {}\nGot: {}",
            $prefix, $input, result
        );
        assert!(
            result.ends_with($suffix),
            "Expected output to end with '{}'\nInput: {}\nGot: {}",
            $suffix, $input, result
        );
    }};

    ($input:expr => starts $prefix:expr, ends $suffix:expr, contains [$($expected:expr),+ $(,)?]) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert!(
            result.starts_with($prefix),
            "Expected output to start with '{}'\nInput: {}\nGot: {}",
            $prefix, $input, result
        );
        assert!(
            result.ends_with($suffix),
            "Expected output to end with '{}'\nInput: {}\nGot: {}",
            $suffix, $input, result
        );
        $(
            assert!(
                result.contains($expected),
                "Expected output to contain '{}'\nInput: {}\nGot: {}",
                $expected, $input, result
            );
        )+
    }};
}

// ============================================================================
// Converter Test Macros
// ============================================================================

/// Test converter output for a style.
///
/// # Example
///
/// ```rust,ignore
/// test_convert!("mathbold", "ABC" => "𝐀𝐁𝐂");
/// ```
#[macro_export]
macro_rules! test_convert {
    ($style:expr, $input:expr => $expected:expr) => {{
        let converter = $crate::Converter::new().expect("Failed to create converter");
        let result = converter
            .convert($input, $style)
            .expect("Conversion failed");
        assert_eq!(result, $expected, "Style: {}, Input: {}", $style, $input);
    }};
}

/// Test that a style conversion fails.
#[macro_export]
macro_rules! test_convert_err {
    ($style:expr, $input:expr) => {{
        let converter = $crate::Converter::new().expect("Failed to create converter");
        let result = converter.convert($input, $style);
        assert!(result.is_err(), "Expected error for style: {}", $style);
    }};
}

// ============================================================================
// Registry Test Macros
// ============================================================================

/// Test that a registry lookup returns Some.
///
/// # Example
///
/// ```rust,ignore
/// test_registry_some!(glyph, "dot");
/// test_registry_some!(style, "mathbold");
/// test_registry_some!(frame, "gradient");
/// ```
#[macro_export]
macro_rules! test_registry_some {
    (glyph, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.glyph($key).is_some(),
            "Expected glyph '{}' to exist",
            $key
        );
    }};
    (style, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.style($key).is_some(),
            "Expected style '{}' to exist",
            $key
        );
    }};
    (frame, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.frame($key).is_some(),
            "Expected frame '{}' to exist",
            $key
        );
    }};
    (component, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.component($key).is_some(),
            "Expected component '{}' to exist",
            $key
        );
    }};
    (separator, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.separator($key).is_some(),
            "Expected separator '{}' to exist",
            $key
        );
    }};
}

/// Test that a registry lookup returns None.
///
/// # Example
///
/// ```rust,ignore
/// test_registry_none!(glyph, "nonexistent");
/// ```
#[macro_export]
macro_rules! test_registry_none {
    (glyph, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.glyph($key).is_none(),
            "Expected glyph '{}' to NOT exist",
            $key
        );
    }};
    (style, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.style($key).is_none(),
            "Expected style '{}' to NOT exist",
            $key
        );
    }};
    (frame, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.frame($key).is_none(),
            "Expected frame '{}' to NOT exist",
            $key
        );
    }};
    (component, $key:expr) => {{
        let registry = $crate::Registry::new().expect("Failed to create registry");
        assert!(
            registry.component($key).is_none(),
            "Expected component '{}' to NOT exist",
            $key
        );
    }};
}

// ============================================================================
// Shields Test Macros
// ============================================================================

/// Test that shields rendering succeeds and contains expected substrings.
///
/// # Example
///
/// ```rust,ignore
/// test_shields_contains!(render_block("2B6CB0", "flat-square") => ["shields.io", "2B6CB0"]);
/// ```
#[macro_export]
macro_rules! test_shields_contains {
    (render_block($color:expr, $style:expr) => [$($expected:expr),+ $(,)?]) => {{
        let renderer = $crate::ShieldsRenderer::new().expect("Failed to create shields renderer");
        let result = renderer.render_block($color, $style).expect("render_block failed");
        $(
            assert!(
                result.contains($expected),
                "Expected render_block output to contain '{}'\nGot: {}",
                $expected, result
            );
        )+
    }};
    (render_twotone($left:expr, $right:expr, $style:expr) => [$($expected:expr),+ $(,)?]) => {{
        let renderer = $crate::ShieldsRenderer::new().expect("Failed to create shields renderer");
        let result = renderer.render_twotone($left, $right, $style).expect("render_twotone failed");
        $(
            assert!(
                result.contains($expected),
                "Expected render_twotone output to contain '{}'\nGot: {}",
                $expected, result
            );
        )+
    }};
    (render_icon($logo:expr, $bg:expr, $logo_color:expr, $style:expr) => [$($expected:expr),+ $(,)?]) => {{
        let renderer = $crate::ShieldsRenderer::new().expect("Failed to create shields renderer");
        let result = renderer.render_icon($logo, $bg, $logo_color, $style).expect("render_icon failed");
        $(
            assert!(
                result.contains($expected),
                "Expected render_icon output to contain '{}'\nGot: {}",
                $expected, result
            );
        )+
    }};
}

/// Test that a shields color resolves correctly.
///
/// # Example
///
/// ```rust,ignore
/// test_shields_color!("cobalt" => "2B6CB0");
/// test_shields_color!("ABC123" => "ABC123");
/// ```
#[macro_export]
macro_rules! test_shields_color {
    ($input:expr => $expected:expr) => {{
        let renderer = $crate::ShieldsRenderer::new().expect("Failed to create shields renderer");
        let result = renderer
            .resolve_color($input)
            .expect("Color resolution failed");
        assert_eq!(
            result, $expected,
            "Color '{}' should resolve to '{}'",
            $input, $expected
        );
    }};
}

/// Test that a shields color resolution fails.
///
/// # Example
///
/// ```rust,ignore
/// test_shields_color_err!("invalid");
/// ```
#[macro_export]
macro_rules! test_shields_color_err {
    ($input:expr) => {{
        let renderer = $crate::ShieldsRenderer::new().expect("Failed to create shields renderer");
        let result = renderer.resolve_color($input);
        assert!(
            result.is_err(),
            "Expected color '{}' to fail resolution",
            $input
        );
    }};
}

// ============================================================================
// Module Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_basic() {
        test_process!("{{mathbold}}A{{/mathbold}}" => "𝐀");
    }

    #[test]
    fn test_macro_unchanged() {
        test_process_unchanged!("```\n{{mathbold}}CODE{{/mathbold}}\n```");
    }

    #[test]
    fn test_macro_error() {
        // Unclosed tag causes an error
        test_process_err!("{{mathbold}}text without closing tag");
    }

    #[test]
    fn test_macro_cases() {
        test_process_cases!(
            "{{mathbold}}A{{/mathbold}}" => "𝐀",
            "{{mathbold}}B{{/mathbold}}" => "𝐁",
            "plain text" => "plain text",
        );
    }

    #[test]
    fn test_convert_macro() {
        test_convert!("mathbold", "ABC" => "𝐀𝐁𝐂");
    }

    // Test that lazy static fixtures are accessible
    #[test]
    fn test_fixtures_accessible() {
        assert!(TEST_CONVERTER.convert("A", "mathbold").is_ok());
        assert!(TEST_REGISTRY.glyph("dot").is_some());
        assert!(TEST_SHIELDS.render_block("F41C80", "flat-square").is_ok());
    }

    #[test]
    fn test_registry_macros() {
        test_registry_some!(glyph, "dot");
        test_registry_some!(style, "mathbold");
        test_registry_some!(frame, "gradient");
        test_registry_none!(glyph, "nonexistent_glyph_xyz");
    }

    #[test]
    fn test_shields_macros() {
        test_shields_contains!(render_block("2B6CB0", "flat-square") => ["shields.io", "2B6CB0"]);
        test_shields_color!("cobalt" => "2B6CB0");
    }
}
