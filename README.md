# logo_gen (Rust) — Project Stub

Deterministic logo generation from an input string with pluggable algorithms ("presets").

This stub compiles and produces:
- **SVG** output (vector-first, minimal scene graph)
- **PNG** output (placeholder raster renderer; currently background + simple mark)

The focus is **architecture and extensibility**. Visual fidelity will be implemented incrementally.

## Requirements
- Rust stable (edition 2021)
- No system libraries required

## Build
```bash
cargo build
```

## Run (CLI)
Generate SVG:
```bash
cargo run --bin logo-gen -- --input "Acme Power" --preset monogram-badge --format svg --out ./acme.svg
```

Generate PNG:
```bash
cargo run --bin logo-gen -- --input "Acme Power" --preset monogram-badge --format png --out ./acme.png --size 512
```

## Visual Studio / VS Code
- **Visual Studio Code**: install *rust-analyzer* extension.
- **Visual Studio 2022**: use the Rust tooling/extension of your choice; the project is standard Cargo.
  - Open the folder; `cargo build`/`cargo test` drive the build.

## Roadmap
See `docs/issue_list.md` for a suggested GitHub issue breakdown.
