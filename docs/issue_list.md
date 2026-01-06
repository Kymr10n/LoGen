# GitHub Task / Issue List (Proposed)

This backlog assumes an incremental path from a compiling stub to a production-grade deterministic logo generator.

## Epic 1 — Core determinism & normalization
1. **Define normalization policy (Unicode, casefolding, word splitting)**
   - Acceptance: documented rules + unit tests covering edge cases (emoji, umlauts, punctuation).
2. **Seed derivation hardening**
   - Acceptance: stable seed across platforms; optional `variant` behavior documented and tested.
3. **Golden test harness**
   - Acceptance: snapshot tests for SVG text output; pixel tests for PNG (where deterministic).

## Epic 2 — Rendering architecture (vector-first)
4. **Formalize Scene Graph**
   - Add: layers, opacity, strokes, transforms, gradients, clip paths.
   - Acceptance: serialized debug view + render parity tests.
5. **SVG renderer: gradients + clip paths**
   - Acceptance: correct SVG output with minimal, valid XML.
6. **PNG renderer: vector rasterization backend selection**
   - Option A: `resvg` (SVG -> raster) to guarantee parity
   - Option B: `tiny-skia` draw ops renderer
   - Acceptance: visually consistent output vs SVG reference.

## Epic 3 — Typography (production)
7. **Font strategy**
   - Curated font set embedded (e.g. via `include_bytes!`) and licensing documented.
8. **Text shaping + glyph coverage**
   - Use `ttf-parser` + `rustybuzz` (or equivalent).
   - Acceptance: multi-script inputs do not panic; predictable fallbacks.
9. **PNG text rendering**
   - Acceptance: text aligns with SVG baseline behavior; no blurry output at 512/1024.

## Epic 4 — Color science & aesthetics
10. **Move palette to perceptual color space (OKLCH)**
    - Acceptance: conversions tested; palette constraints documented.
11. **Contrast & accessibility heuristics**
    - Acceptance: guarantee minimum contrast for foreground/background where applicable.
12. **Gradient library**
    - Acceptance: linear/radial gradients with controlled parameter ranges.

## Epic 5 — Algorithms / Presets
13. **Preset: MonogramBadge v1 (current) → v2**
    - Rounded-rect vs circle; border/stroke; subtle notch; balanced typography.
14. **Preset: GlyphGridPattern**
    - Tiled pattern derived from input; overlay lettermark; density controls.
15. **Preset: RibbonMark**
    - Bezier ribbons with deterministic control points; clipping + knockout.
16. **Preset registry & configuration**
    - Acceptance: list presets via CLI; stable preset IDs; versioned defaults.

## Epic 6 — API & Packaging
17. **Public API stability**
    - Acceptance: semver policy; `RenderOptions` backwards compatibility.
18. **WASM target (optional)**
    - Acceptance: generate SVG in-browser; deterministic parity tests.
19. **Performance & memory**
    - Benchmarks: 256/512/1024; caps on allocations; time budget documented.

## Epic 7 — Developer Experience
20. **CLI enhancements**
    - `--list-presets`, `--dump-scene`, `--batch` directory mode.
21. **CI pipeline**
    - `cargo fmt`, `clippy`, tests, release artifact build.
22. **Documentation & examples**
    - Gallery generation script; README with examples; licensing notes.
