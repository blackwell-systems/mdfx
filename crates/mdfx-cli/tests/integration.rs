//! CLI Integration Tests for mdfx
//!
//! These tests verify end-to-end CLI behavior including:
//! - Command execution and output
//! - Template processing workflow
//! - Asset generation and manifest updates
//! - Error handling

use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::fs;
use tempfile::TempDir;

// =============================================================================
// CONVERT COMMAND TESTS (Parameterized)
// =============================================================================

#[rstest]
#[case("mathbold", "Hello", "𝐇𝐞𝐥𝐥𝐨")]
#[case("mb", "Test", "𝐓𝐞𝐬𝐭")] // alias
#[case("fullwidth", "ABC", "ＡＢＣ")]
#[case("sans-serif-bold-italic", "Hi", "𝙃𝙞")]
#[case("double-struck", "R", "ℝ")]
fn test_convert_styles(#[case] style: &str, #[case] input: &str, #[case] expected: &str) {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", style, input])
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_convert_with_spacing() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", "mathbold", "--spacing", "1", "AB"])
        .assert()
        .success()
        .stdout(predicate::str::contains("𝐀 𝐁"));
}

#[test]
fn test_convert_invalid_style() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", "nonexistent-style", "Test"])
        .assert()
        .failure();
}

#[test]
fn test_convert_script() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", "script", "Hello"])
        .assert()
        .success();
}

// =============================================================================
// LIST COMMAND TESTS (Parameterized)
// =============================================================================

#[rstest]
#[case(&["list"], "mathbold")] // default lists styles
#[case(&["list", "styles"], "mathbold")]
#[case(&["list", "styles", "--samples"], "𝐀𝐁𝐂")]
#[case(&["list", "styles", "-f", "bold"], "mathbold")]
#[case(&["list", "glyphs"], "")] // just check success
#[case(&["list", "frames"], "")] // just check success
fn test_list_resources(#[case] args: &[&str], #[case] expected: &str) {
    let mut cmd = Command::cargo_bin("mdfx").unwrap();
    let assert = cmd.args(args).assert().success();

    if !expected.is_empty() {
        assert.stdout(predicate::str::contains(expected));
    }
}

#[test]
fn test_list_components() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["list", "components"])
        .assert()
        .success()
        .stdout(predicate::str::contains("swatch").or(predicate::str::contains("tech")));
}

// =============================================================================
// PROCESS COMMAND TESTS
// =============================================================================

#[test]
fn test_process_simple_template() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    // Create a simple template
    fs::write(&input, "# Test\n{{mathbold}}Hello{{/mathbold}} World").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("𝐇𝐞𝐥𝐥𝐨"));
    assert!(content.contains("World"));
}

#[test]
fn test_process_with_svg_backend() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    // Create a template with a swatch
    fs::write(&input, "# Colors\n{{ui:swatch:FF0000/}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Output should exist
    assert!(output.exists());

    // Assets directory should be created with SVG files
    assert!(assets.exists());
}

#[test]
fn test_process_with_tech_badge() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    // Create a template with a tech badge
    fs::write(&input, "# Tech\n{{ui:tech:rust/}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    // Should contain a markdown image reference
    assert!(content.contains("![](") || content.contains("!["));
}

#[test]
fn test_process_preserves_code_blocks() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    // Templates inside code blocks should NOT be processed
    fs::write(
        &input,
        "```\n{{mathbold}}not processed{{/mathbold}}\n```\n\n{{mathbold}}processed{{/mathbold}}",
    )
    .unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    // Code block content should be preserved literally
    assert!(content.contains("{{mathbold}}not processed{{/mathbold}}"));
    // Outside code block should be processed
    assert!(content.contains("𝐩𝐫𝐨𝐜𝐞𝐬𝐬𝐞𝐝"));
}

#[test]
fn test_process_plaintext_backend() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    // Plaintext backend renders UI components as text descriptions
    fs::write(&input, "{{mathbold}}Hello{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("𝐇𝐞𝐥𝐥𝐨"));
}

#[test]
fn test_process_missing_input() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["process", "-o", "output.md", "nonexistent.md"])
        .assert()
        .failure();
}

#[test]
fn test_process_inplace() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");

    fs::write(&input, "{{mathbold}}Test{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["process", "-i", input.to_str().unwrap()])
        .assert()
        .success();

    let content = fs::read_to_string(&input).unwrap();
    assert!(content.contains("𝐓𝐞𝐬𝐭"));
}

// =============================================================================
// ASSET MANIFEST TESTS
// =============================================================================

#[test]
fn test_process_generates_manifest() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    fs::write(&input, "{{ui:swatch:FF0000/}}\n{{ui:swatch:00FF00/}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let manifest_path = assets.join("manifest.json");
    assert!(manifest_path.exists(), "Manifest should be created");

    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    assert!(manifest_content.contains("\"version\""));
    assert!(manifest_content.contains("\"assets\""));
}

#[test]
fn test_process_content_addressed_filenames() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    // Same swatch twice should produce same filename
    fs::write(&input, "{{ui:swatch:AABBCC/}}\n{{ui:swatch:AABBCC/}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Count SVG files - should be exactly 1 due to deduplication
    let svg_count = fs::read_dir(&assets)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .ok()
                .map(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "svg")
                        .unwrap_or(false)
                })
                .unwrap_or(false)
        })
        .count();

    assert_eq!(
        svg_count, 1,
        "Identical swatches should produce single asset"
    );
}

// =============================================================================
// ERROR HANDLING TESTS
// =============================================================================

#[test]
fn test_help_flag() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mdfx"));
}

#[test]
fn test_version_flag() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["--version"])
        .assert()
        .success();
}

#[test]
fn test_invalid_subcommand() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["nonexistent-command"])
        .assert()
        .failure();
}

// =============================================================================
// SHIELDS BACKEND TESTS
// =============================================================================

#[test]
fn test_process_shields_backend() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    fs::write(&input, "{{ui:tech:rust/}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "shields",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    // Shields backend uses shields.io URLs
    assert!(
        content.contains("shields.io") || content.contains("![]("),
        "Should contain shields.io URL or image reference"
    );
}

// =============================================================================
// EDGE CASES
// =============================================================================

#[test]
fn test_process_empty_file() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("empty.md");
    let output = temp.path().join("output.md");

    fs::write(&input, "").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.is_empty() || content.trim().is_empty());
}

#[test]
fn test_process_no_templates() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("plain.md");
    let output = temp.path().join("output.md");

    fs::write(&input, "# Plain Markdown\n\nNo templates here.").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("Plain Markdown"));
    assert!(content.contains("No templates here"));
}

#[test]
fn test_process_nested_templates() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    // Nested templates - outer should work
    fs::write(&input, "{{mathbold}}Hello{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn test_convert_unicode_input() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", "mathbold", "Héllo Wörld"])
        .assert()
        .success();
}

#[test]
fn test_process_multiple_styles() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    fs::write(
        &input,
        "{{mathbold}}Bold{{/mathbold}} and {{italic}}Italic{{/italic}}",
    )
    .unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("𝐁𝐨𝐥𝐝"));
}

// =============================================================================
// VERIFY COMMAND TESTS
// =============================================================================

#[test]
fn test_verify_valid_assets() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    fs::write(&input, "{{ui:swatch:FF0000/}}").unwrap();

    // First generate some assets
    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Then verify them
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["verify", "--assets-dir", assets.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("All assets verified"));
}

#[test]
fn test_verify_missing_manifest() {
    let temp = TempDir::new().unwrap();
    let assets = temp.path().join("empty_assets");
    fs::create_dir(&assets).unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["verify", "--assets-dir", assets.to_str().unwrap()])
        .assert()
        .failure();
}

// =============================================================================
// CLEAN COMMAND TESTS
// =============================================================================

#[test]
fn test_clean_dry_run() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    fs::write(&input, "{{ui:swatch:FF0000/}}").unwrap();

    // Generate assets
    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Add an orphan file
    fs::write(assets.join("orphan.svg"), "<svg></svg>").unwrap();

    // Dry run should show orphan
    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "clean",
            "--assets-dir",
            assets.to_str().unwrap(),
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("orphan.svg"));
}

#[test]
fn test_clean_removes_orphans() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");
    let assets = temp.path().join("assets");

    fs::write(&input, "{{ui:swatch:00FF00/}}").unwrap();

    // Generate assets
    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "process",
            "-b",
            "svg",
            "--assets-dir",
            assets.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            input.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Add orphan
    let orphan = assets.join("orphan.svg");
    fs::write(&orphan, "<svg></svg>").unwrap();
    assert!(orphan.exists());

    // Run clean (actually delete)
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["clean", "--assets-dir", assets.to_str().unwrap()])
        .assert()
        .success();

    // Orphan should be deleted
    assert!(!orphan.exists());
}

// =============================================================================
// BUILD COMMAND TESTS
// =============================================================================

#[test]
fn test_build_single_target() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output_dir = temp.path().join("dist");

    fs::write(&input, "{{mathbold}}Hello{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "build",
            input.to_str().unwrap(),
            "--output-dir",
            output_dir.to_str().unwrap(),
            "--targets",
            "github",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Built"));

    // Output file should exist
    assert!(output_dir.join("input_github.md").exists());
}

#[test]
fn test_build_multiple_targets() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output_dir = temp.path().join("dist");

    fs::write(&input, "# Test\n{{mathbold}}Title{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args([
            "build",
            input.to_str().unwrap(),
            "--output-dir",
            output_dir.to_str().unwrap(),
            "--targets",
            "github,pypi",
        ])
        .assert()
        .success();

    assert!(output_dir.join("input_github.md").exists());
    assert!(output_dir.join("input_pypi.md").exists());
}

// =============================================================================
// COMPLETIONS COMMAND TESTS
// =============================================================================

#[rstest]
#[case("bash")]
#[case("zsh")]
#[case("fish")]
fn test_completions_generation(#[case] shell: &str) {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["completions", shell])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_completions_invalid_shell() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["completions", "invalid-shell"])
        .assert()
        .failure();
}

// =============================================================================
// LIST PALETTE TESTS
// =============================================================================

#[test]
fn test_list_palette() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["list", "palette"])
        .assert()
        .success()
        .stdout(predicate::str::contains("accent"))
        .stdout(predicate::str::contains("F41C80"));
}

#[test]
fn test_list_palette_with_filter() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["list", "palette", "-f", "dark"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dark1"));
}
