# LoGen (Rust) — Deterministic Logo Generator

![CI](https://github.com/YOUR_USERNAME/LoGen/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/YOUR_USERNAME/LoGen/branch/main/graph/badge.svg)](https://codecov.io/gh/YOUR_USERNAME/LoGen)

Deterministic logo generation from an input string with pluggable algorithms ("presets").

**Note**: Font assets are not included in the repository. Run `./setup-assets.sh` to download them before building.

## Features

- ✅ **Deterministic generation** - Same input = same output, every time
- ✅ **Multiple presets** - Badge, geometric patterns, and more
- ✅ **SVG & PNG output** - Vector-first with high-quality raster rendering
- ✅ **Full text rendering** - Embedded fonts with proper text layout
- ✅ **Variant support** - Generate alternatives from the same input
- ✅ **Transparent backgrounds** - Optional transparency for both formats
- ✅ **Color science** - Palette generation with proper contrast
- ✅ **Border/stroke support** - Optional borders for visual variety

## Available Presets

List all presets:
```bash
cargo run --bin LoGen -- --list-presets
```

**Current presets:**
- **`monogram-badge`** - Rounded badge with centered initials (circles or rounded rectangles)
- **`geometric-pattern`** - Overlapping geometric shapes with lettermark

## Requirements
- Rust stable (edition 2021)
- No system libraries required
- Font assets (automatically downloaded via setup script)

## Setup

First, download required font assets:
```bash
./setup-assets.sh
```

This will download the Roboto Bold font (~168KB) used for text rendering.

## Build
```bash
cargo build
```

## Usage

**Generate SVG:**
```bash
cargo run --bin LoGen -- --input "Acme Power" --preset monogram-badge --format svg --out ./acme.svg
```

**Generate PNG:**
```bash
cargo run --bin LoGen -- --input "Acme Power" --preset monogram-badge --format png --out ./acme.png --size 512
```

**With transparent background:**
```bash
cargo run --bin LoGen -- --input "Acme Power" --preset geometric-pattern --format png --out ./acme.png --size 512 --transparent
```

**Generate variants (same input, different outputs):**
```bash
cargo run --bin LoGen -- --input "Brand X" --preset monogram-badge --format png --out ./brand_v1.png --size 512 --variant 1
cargo run --bin LoGen -- --input "Brand X" --preset monogram-badge --format png --out ./brand_v2.png --size 512 --variant 2
```

**Try different presets:**
```bash
cargo run --bin LoGen -- --input "Creative Studio" --preset geometric-pattern --format svg --out ./creative.svg
```

**Run demo script:**
```bash
./generate_examples.sh
```
```bash
cargo run --bin LoGen -- --input "Acme Power" --preset monogram-badge --format svg --out ./acme.svg
```

Generate PNG:
```bash
cargo run --bin LoGen -- --input "Acme Power" --preset monogram-badge --format png --out ./acme.png --size 512
```

## Visual Studio / VS Code
- **Visual Studio Code**: install *rust-analyzer* extension.
- **Visual Studio 2022**: use the Rust tooling/extension of your choice; the project is standard Cargo.
  - Open the folder; `cargo build`/`cargo test` drive the build.

## Testing

Run tests:
```bash
cargo test
```

Run with coverage (requires `cargo-tarpaulin`):
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace --timeout 120
```

## Pre-commit Hooks

The repository includes a pre-commit hook that automatically runs tests, clippy, and format checks before each commit. 

**Important**: Run `./setup-assets.sh` before your first commit to ensure fonts are available for tests.

To bypass the hook (not recommended):
```bash
git commit --no-verify
```

## Continuous Integration

GitHub Actions CI runs on every push and pull request:
- ✅ Tests on Linux, macOS, and Windows
- ✅ Clippy linting with warnings as errors
- ✅ Code formatting checks
- ✅ Code coverage reporting

## Roadmap
See `docs/issue_list.md` for a suggested GitHub issue breakdown.
