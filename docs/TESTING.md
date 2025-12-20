# Testing Guide for mdfx

This document describes the testing strategy, tools, and practices used in the mdfx project.

## Testing Philosophy

The mdfx project employs a multi-layered testing approach:

1. **Unit Tests** - Test individual functions and modules in isolation
2. **Parameterized Tests** - Use rstest for data-driven testing of similar cases
3. **Snapshot Tests** - Capture and verify SVG output stability
4. **Parity Tests** - Ensure badgefx produces identical output to mdfx renderers
5. **Integration Tests** - End-to-end CLI command verification

## Test Locations

| Test Type | Location | Purpose |
|-----------|----------|---------|
| Unit tests | `**/src/**/*.rs` (inline) | Module-level functionality |
| Parity tests | `crates/badgefx/tests/parity_test.rs` | badgefx ↔ mdfx output matching |
| Integration tests | `crates/mdfx-cli/tests/integration.rs` | CLI end-to-end workflows |
| Badge snapshots | `crates/badgefx/src/snapshots/` | Badge SVG stability |
| Primitive snapshots | `crates/mdfx/src/renderer/svg/snapshots/` | Primitive SVG stability |

## Running Tests

### All Tests

```bash
# Run all tests (debug mode - faster compilation)
cargo test

# Run all tests (release mode - faster execution)
cargo test --release

# Run with verbose output
cargo test --release --verbose
```

### Specific Crate Tests

```bash
# Test specific crate
cargo test --package mdfx --release
cargo test --package badgefx --release
cargo test --package mdfx-cli --release
cargo test --package mdfx-colors --release
cargo test --package mdfx-icons --release
cargo test --package mdfx-fetch --release
```

### Pattern Matching

```bash
# Run tests matching a pattern
cargo test --package mdfx parser:: --release
cargo test --release -- snapshot
cargo test --release -- test_convert
```

### Integration Tests Only

```bash
# Run CLI integration tests
cargo test --package mdfx-cli --release --test integration
```

## Parameterized Testing with rstest

We use the `rstest` crate for parameterized testing. This reduces code duplication when testing similar scenarios.

### When to Use rstest

- Multiple tests that differ only by input/output values
- Testing the same logic with different configurations
- Validating behavior across a range of inputs

### Pattern

```rust
use rstest::rstest;

// Instead of separate test functions:
// #[test] fn test_style_flat() { ... }
// #[test] fn test_style_plastic() { ... }

// Use a single parameterized test:
#[rstest]
#[case("flat", BadgeStyle::Flat, 20)]
#[case("plastic", BadgeStyle::Plastic, 22)]
#[case("social", BadgeStyle::Social, 20)]
fn test_style_metrics(
    #[case] name: &str,
    #[case] style: BadgeStyle,
    #[case] expected_height: u32,
) {
    let metrics = SvgMetrics::from_style(style);
    assert_eq!(metrics.height as u32, expected_height, "Height for {}", name);
}
```

### Test Module Setup

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Tests here...
}
```

## Snapshot Testing with insta

We use the `insta` crate to capture and verify SVG output. Snapshots detect unintended rendering changes.

### Running Snapshot Tests

```bash
# Run snapshot tests
cargo test --package badgefx --release -- snapshot
cargo test --package mdfx --release -- snapshot

# Run all snapshot tests
cargo test --release -- snapshot
```

### Reviewing Snapshots

```bash
# Interactive review (recommended)
cargo insta review

# Accept all pending snapshots
cargo insta accept --all

# Reject all pending snapshots
cargo insta reject --all
```

### Creating New Snapshots

```rust
use insta::assert_snapshot;

#[test]
fn snapshot_my_feature() {
    let output = render_something();
    assert_snapshot!("my_feature_name", output);
}
```

### When to Update Snapshots

- After intentional changes to SVG rendering
- When adding new rendering features
- **Never** blindly accept - always review the diff

### CI Enforcement

CI checks for pending `.snap.new` files and fails if any exist. This ensures all snapshot changes are reviewed before merging.

## Integration Testing

Integration tests verify end-to-end CLI behavior using `assert_cmd`.

### Test Categories

| Category | Tests | Purpose |
|----------|-------|---------|
| Convert | 6 | Style conversion commands |
| List | 5 | Resource listing commands |
| Process | 12 | Template processing workflows |
| Error handling | 3 | Invalid inputs and edge cases |
| Edge cases | 5 | Empty files, unicode, etc. |

### Running Integration Tests

```bash
cargo test --package mdfx-cli --release --test integration
```

### Writing Integration Tests

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_convert_mathbold() {
    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["convert", "--style", "mathbold", "Hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("𝐇𝐞𝐥𝐥𝐨"));
}

#[test]
fn test_process_template() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output = temp.path().join("output.md");

    fs::write(&input, "{{mathbold}}Test{{/mathbold}}").unwrap();

    Command::cargo_bin("mdfx")
        .unwrap()
        .args(["process", "-o", output.to_str().unwrap(), input.to_str().unwrap()])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("𝐓𝐞𝐬𝐭"));
}
```

## Parity Testing

Parity tests ensure `badgefx` produces identical SVG output to mdfx's original renderers.

### Location

`crates/badgefx/tests/parity_test.rs`

### Coverage

- 43+ test cases covering all badge configurations
- Neon Tech Showcase examples (cyber stack, ghost protocol, etc.)
- Chevron variations, custom colors, borders
- All badge styles

### Running Parity Tests

```bash
cargo test --package badgefx --release --test parity_test
```

## Code Coverage

### Setup

```bash
# Install coverage tool
cargo install cargo-llvm-cov
```

### Running Coverage

```bash
# Generate coverage report
cargo llvm-cov --release

# Generate HTML report
cargo llvm-cov --release --html

# Generate LCOV format (for CI)
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```

### Coverage Goals

| Module | Target | Notes |
|--------|--------|-------|
| Core library (mdfx) | >80% | Critical paths |
| Badge rendering (badgefx) | >80% | Rendering logic |
| Colors (mdfx-colors) | >90% | Pure functions |
| Icons (mdfx-icons) | >70% | Data lookups |
| Fetch (mdfx-fetch) | >70% | External API handling |
| CLI (mdfx-cli) | <50% | Integration tests preferred |

## CI/CD Integration

### GitHub Actions Workflow

The CI pipeline runs:

1. **Test job** - All unit and integration tests on Linux, Windows, macOS
2. **Lint job** - `cargo fmt`, `cargo clippy`, snapshot file check
3. **Build job** - Release builds on all platforms
4. **Coverage job** - Code coverage with Codecov upload

### Snapshot File Check

CI fails if pending `.snap.new` files are committed:

```yaml
- name: Check for pending snapshots
  run: |
    if find . -name "*.snap.new" | grep -q .; then
      echo "Error: Found pending snapshot files"
      exit 1
    fi
```

## Best Practices

### Do

- Use rstest for parameterized tests
- Add snapshot tests for new rendering features
- Run `cargo insta review` before committing
- Test error paths and edge cases
- Keep integration tests focused and fast

### Don't

- Write individual test functions for variations of the same test
- Accept snapshots without reviewing the diff
- Skip tests in CI
- Ignore failing tests locally

## Troubleshooting

### Snapshot Tests Failing

```bash
# Review what changed
cargo insta review

# If changes are intentional, accept them
cargo insta accept --all

# If not, fix the code
```

### Integration Tests Timeout

```bash
# Run with more threads
cargo test --release -- --test-threads=8

# Run single test
cargo test --release test_name
```

### Coverage Not Generating

```bash
# Ensure llvm-tools-preview is installed
rustup component add llvm-tools-preview

# Reinstall cargo-llvm-cov
cargo install cargo-llvm-cov --force
```

## Adding New Tests

### For New Features

1. Add unit tests in the module
2. Add integration tests if CLI-facing
3. Add snapshot tests if visual output
4. Update coverage if needed

### For Bug Fixes

1. Add regression test that fails without the fix
2. Apply the fix
3. Verify test passes

### For Refactoring

1. Ensure existing tests pass
2. Add snapshot tests before refactoring (if visual)
3. Run full test suite after changes
