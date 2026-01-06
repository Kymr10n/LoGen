# Embedded Fonts

This directory contains fonts embedded in the binary for deterministic, cross-platform logo rendering.

## LiberationSans-Bold.ttf (actually Roboto-Bold.ttf)

- **License**: Apache License 2.0
- **Source**: [Google Fonts - Roboto](https://github.com/googlefonts/roboto)
- **Purpose**: Default bold font for monogram text rendering
- **File Size**: ~168KB

The font is embedded at compile time via `include_bytes!()` macro and does not require external files at runtime.

## License

Roboto is licensed under the Apache License 2.0, which permits:
- Embedding in software
- Redistribution
- Modification
- Commercial use

See: https://www.apache.org/licenses/LICENSE-2.0
