# Asset Management Guide

This guide covers mdfx's asset generation, tracking, and management system.

## Overview

When using the SVG backend, mdfx generates asset files (SVGs) and tracks them in a manifest. The system provides:

- **Content-addressed filenames**: Stable, deterministic names based on SHA-256
- **Manifest tracking**: JSON manifest with metadata for each asset
- **Provenance tracking**: Know which templates generated which assets
- **Atomic writes**: Safe manifest updates that survive crashes
- **Incremental updates**: Merge new assets without full regeneration

## Content-Addressed Filenames

Asset filenames are derived from the SHA-256 hash of the rendered SVG content:

```
{type}_{sha256-first-16-chars}.svg

Examples:
  swatch_a3f8e2d1c4b5a6f7.svg
  tech_7b9c4e1f2a3d5678.svg
  progress_1234567890abcdef.svg
```

### Benefits

1. **Stability**: Same content always produces the same filename, regardless of:
   - Rust version
   - Build environment
   - Time of generation

2. **Deduplication**: Identical SVG content (even from different primitives) shares the same file

3. **Cache-friendly**: CI/CD caches remain valid across Rust toolchain updates

### Previous Behavior

Prior to v1.1.0, filenames used Rust's `DefaultHasher` which could change between Rust versions, causing unnecessary cache invalidation.

## Manifest Schema (v1.1.0)

```json
{
  "version": "1.1.0",
  "created_at": "2025-12-19T10:30:00Z",
  "backend": "svg",
  "assets_dir": "assets",
  "total_assets": 42,
  "total_size_bytes": 125000,
  "generator_version": "1.0.0",
  "assets": [
    {
      "path": "assets/swatch_a3f8e2d1c4b5a6f7.svg",
      "sha256": "a3f8e2d1c4b5a6f7...(full 64 chars)",
      "type": "swatch",
      "primitive": {
        "kind": "Swatch",
        "color": "F41C80",
        "style": "flat-square"
      },
      "size_bytes": 1234,
      "source_files": ["README.md", "docs/intro.md"],
      "generated_at": "2025-12-19T10:30:00Z",
      "generator_version": "1.0.0"
    }
  ]
}
```

### Fields

| Field | Description |
|-------|-------------|
| `version` | Manifest schema version (currently "1.1.0") |
| `created_at` | RFC3339 timestamp of manifest creation/update |
| `backend` | Rendering backend ("svg") |
| `assets_dir` | Relative path to assets directory |
| `total_assets` | Number of assets in manifest |
| `total_size_bytes` | Sum of all asset sizes |
| `generator_version` | mdfx version that generated the manifest |
| `assets` | Array of asset entries |

### Asset Entry Fields

| Field | Description |
|-------|-------------|
| `path` | Relative path to asset file |
| `sha256` | Full SHA-256 hash for verification |
| `type` | Asset type (swatch, tech, progress, etc.) |
| `primitive` | Parameters used to generate the asset |
| `size_bytes` | File size in bytes |
| `source_files` | Template files that reference this asset |
| `generated_at` | When this specific asset was generated |
| `generator_version` | mdfx version that generated this asset |

## CLI Commands

### Generate Assets

```bash
# Generate assets with manifest
mdfx process -b svg --assets-dir assets -o output.md input.template.md
```

### Verify Assets

Check that all manifest assets exist with correct hashes:

```bash
mdfx verify assets/manifest.json

# Output:
# ✓ 42 assets valid
# ✗ 2 assets missing
# ✗ 1 hash mismatch
```

### Clean Stale Assets

Remove assets not referenced in templates:

```bash
# Preview what would be deleted
mdfx clean --scan "*.md" --dry-run assets

# Actually delete stale assets
mdfx clean --scan "*.md" assets
```

## Library API

### Creating Manifests

```rust
use mdfx::manifest::{AssetManifest, AssetEntry, content_addressed_filename};

// Create new manifest
let mut manifest = AssetManifest::new("svg", "assets");

// Add asset with source tracking
manifest.add_asset_with_source(
    "assets/swatch_abc123.svg".to_string(),
    svg_bytes,
    &primitive,
    "swatch".to_string(),
    Some("README.md".to_string()),
);
```

### Atomic Writes

```rust
// Safe write that survives crashes
manifest.write_atomic(&path)?;

// Regular write (faster but not crash-safe)
manifest.write(&path)?;
```

### Incremental Updates

```rust
use std::collections::HashSet;

// Load existing manifest
let mut manifest = AssetManifest::load(&path)?;

// Define which paths should exist after merge
let keep_paths: HashSet<String> = new_assets
    .iter()
    .map(|a| a.path.clone())
    .collect();

// Merge new assets, removing stale ones
manifest.merge(new_assets, Some(keep_paths));

// Write updated manifest
manifest.write_atomic(&path)?;
```

### Content-Addressed Filenames

```rust
use mdfx::manifest::content_addressed_filename;

let svg_bytes = render_svg(&primitive);
let filename = content_addressed_filename(&svg_bytes, "swatch");
// Returns: "swatch_a3f8e2d1c4b5a6f7.svg"
```

### Verification

```rust
let manifest = AssetManifest::load(&path)?;
let results = manifest.verify(&base_dir);

for result in results {
    match result {
        VerificationResult::Valid { path } => println!("✓ {}", path),
        VerificationResult::Missing { path } => println!("✗ Missing: {}", path),
        VerificationResult::HashMismatch { path, expected, actual } => {
            println!("✗ Hash mismatch: {} (expected {}, got {})", path, expected, actual);
        }
        VerificationResult::ReadError { path, error } => {
            println!("✗ Read error: {}: {}", path, error);
        }
    }
}
```

### Statistics

```rust
let stats = manifest.stats();
println!("Total assets: {}", stats.total_assets);
println!("Total size: {} bytes", stats.total_size_bytes);

for (asset_type, type_stats) in &stats.by_type {
    println!("  {}: {} files, {} bytes",
        asset_type, type_stats.count, type_stats.total_bytes);
}

if let Some(largest) = &stats.largest_asset {
    println!("Largest asset: {}", largest);
}
```

## Migration from v1.0.0

When upgrading from manifest v1.0.0:

1. **Filenames will change**: First re-render will generate new filenames
2. **Clean up old assets**: Run `mdfx clean --scan "*.md" assets`
3. **Manifest auto-migrates**: Loading a v1.0.0 manifest upgrades it to v1.1.0

```bash
# Full migration workflow
rm -rf assets/              # Option 1: Delete all and regenerate
mdfx process -b svg ...

# OR

mdfx process -b svg ...     # Option 2: Regenerate, then clean
mdfx clean --scan "*.md" assets
```

## Best Practices

### CI/CD Caching

```yaml
# GitHub Actions example
- uses: actions/cache@v3
  with:
    path: assets/
    key: mdfx-assets-${{ hashFiles('**/*.template.md') }}
    restore-keys: |
      mdfx-assets-
```

The content-addressed filenames ensure cache hits even after Rust updates.

### Git Tracking

Track generated assets in git for:
- Immediate rendering on GitHub
- Reproducible documentation builds
- Avoiding regeneration in CI

```gitignore
# Don't ignore assets
!assets/
!assets/**/*.svg
!assets/manifest.json
```

### Verification in CI

```bash
# Fail if assets are missing or corrupted
mdfx verify assets/manifest.json || exit 1
```

## Troubleshooting

### "Hash mismatch" errors

The file on disk doesn't match what was generated. Causes:
- Manual file editing
- Git merge conflicts
- Partial/corrupted write

**Fix**: Delete the affected asset and regenerate.

### "Missing" assets

Referenced in manifest but not on disk. Causes:
- Deleted files
- Incomplete git clone (LFS issues)
- Corrupted cache

**Fix**: Regenerate assets with `mdfx process`.

### Large asset directory

Many stale assets accumulating over time.

**Fix**:
```bash
mdfx clean --scan "*.md" assets
```
