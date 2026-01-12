//! Deterministic logo generation library.
//!
//! This library generates consistent, reproducible logos from input strings using
//! cryptographic hashing and seeded randomization.

pub mod algorithms;
pub mod cli;
pub mod core;
pub mod render;

use thiserror::Error;

/// Output format for generated logos.
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Scalable Vector Graphics (XML-based vector format).
    Svg,
    /// Portable Network Graphics (raster format).
    Png,
}

/// Configuration options for logo rendering.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Output size in pixels (used for PNG; SVG uses it as width/height attributes).
    pub size_px: u32,
    /// Padding in [0.0..0.5] of the canvas size.
    pub padding_frac: f32,
    /// If set, XORed into the derived seed to allow variants per same input.
    pub variant: Option<u64>,
    /// Transparent background for PNG/SVG.
    pub transparent_background: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            size_px: 512,
            padding_frac: 0.12,
            variant: None,
            transparent_background: false,
        }
    }
}

/// Logo generation algorithm presets.
#[derive(Debug, Clone, Copy)]
pub enum Preset {
    /// Generates a badge with initials derived from the input.
    MonogramBadge,
    /// Generates overlapping geometric shapes with a lettermark.
    GeometricPattern,
}

impl Preset {
    pub fn id(&self) -> &'static str {
        match self {
            Preset::MonogramBadge => "monogram-badge",
            Preset::GeometricPattern => "geometric-pattern",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Preset::MonogramBadge => {
                "Rounded badge with centered initials extracted from input text"
            }
            Preset::GeometricPattern => {
                "Overlapping geometric shapes (circles/rectangles) with centered lettermark"
            }
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            Preset::MonogramBadge => "Badge",
            Preset::GeometricPattern => "Abstract",
        }
    }

    /// Returns all available presets.
    pub fn all() -> Vec<Preset> {
        vec![Preset::MonogramBadge, Preset::GeometricPattern]
    }
}

impl std::str::FromStr for Preset {
    type Err = LogoGenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "monogram-badge" | "monogram" | "badge" => Ok(Preset::MonogramBadge),
            "geometric-pattern" | "geometric" | "pattern" => Ok(Preset::GeometricPattern),
            _ => Err(LogoGenError::UnknownPreset(s.to_string())),
        }
    }
}

/// Errors that can occur during logo generation.
#[derive(Debug, Error)]
pub enum LogoGenError {
    /// The requested preset identifier is not recognized.
    #[error("unknown preset: {0}")]
    UnknownPreset(String),
    /// Invalid configuration options provided.
    #[error("invalid options: {0}")]
    InvalidOptions(String),
    /// Error during rendering phase.
    #[error("render error: {0}")]
    Render(String),
    /// I/O error (file operations).
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Main entry point for generating logos.
pub struct LogoGenerator;

impl LogoGenerator {
    /// Generate an SVG logo from the input string.
    ///
    /// # Arguments
    /// * `input` - The string to generate the logo from (deterministically hashed)
    /// * `preset` - The algorithm/style to use
    /// * `opts` - Rendering configuration options
    ///
    /// # Returns
    /// SVG as a UTF-8 string, or an error if generation fails.
    pub fn generate_svg(
        input: &str,
        preset: Preset,
        opts: &RenderOptions,
    ) -> Result<String, LogoGenError> {
        let scene = algorithms::build_scene(input, preset, opts)?;
        render::svg::render_svg(&scene, opts)
    }

    /// Generate a PNG logo from the input string.
    ///
    /// # Arguments
    /// * `input` - The string to generate the logo from (deterministically hashed)
    /// * `preset` - The algorithm/style to use
    /// * `opts` - Rendering configuration options
    ///
    /// # Returns
    /// PNG data as bytes, or an error if generation fails.
    pub fn generate_png(
        input: &str,
        preset: Preset,
        opts: &RenderOptions,
    ) -> Result<Vec<u8>, LogoGenError> {
        let scene = algorithms::build_scene(input, preset, opts)?;
        // Forward to the new API that accepts an optional embedded font.
        render::png::render_png(&scene, opts, None)
    }

    /// Generate a PNG logo, allowing the caller to provide optional font bytes
    /// (as a `'static` slice). If `font_bytes` is `None`, the renderer will
    /// attempt to load a runtime font from `assets/fonts/` or fall back to the
    /// embedded font if available.
    pub fn generate_png_with_font(
        input: &str,
        preset: Preset,
        opts: &RenderOptions,
        font_bytes: Option<&[u8]>,
    ) -> Result<Vec<u8>, LogoGenError> {
        let scene = algorithms::build_scene(input, preset, opts)?;
        render::png::render_png(&scene, opts, font_bytes)
    }

    /// Generate a PNG logo accepting owned font bytes. The owned `Vec<u8>` is
    /// borrowed for the duration of the call, avoiding leaking memory.
    pub fn generate_png_with_owned_font(
        input: &str,
        preset: Preset,
        opts: &RenderOptions,
        font_bytes: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, LogoGenError> {
        let fb_ref = font_bytes.as_deref();
        Self::generate_png_with_font(input, preset, opts, fb_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_format_debug() {
        let svg = OutputFormat::Svg;
        let png = OutputFormat::Png;
        assert_eq!(format!("{:?}", svg), "Svg");
        assert_eq!(format!("{:?}", png), "Png");
    }

    #[test]
    fn output_format_clone() {
        let svg = OutputFormat::Svg;
        let cloned = svg;
        assert!(matches!(cloned, OutputFormat::Svg));
    }

    #[test]
    fn render_options_default() {
        let opts = RenderOptions::default();
        assert_eq!(opts.size_px, 512);
        assert!((opts.padding_frac - 0.12).abs() < 0.001);
        assert!(opts.variant.is_none());
        assert!(!opts.transparent_background);
    }

    #[test]
    fn render_options_clone() {
        let opts = RenderOptions {
            size_px: 256,
            padding_frac: 0.15,
            variant: Some(42),
            transparent_background: true,
        };
        let cloned = opts.clone();
        assert_eq!(cloned.size_px, 256);
        assert!((cloned.padding_frac - 0.15).abs() < 0.001);
        assert_eq!(cloned.variant, Some(42));
        assert!(cloned.transparent_background);
    }

    #[test]
    fn preset_debug() {
        let mb = Preset::MonogramBadge;
        let gp = Preset::GeometricPattern;
        assert_eq!(format!("{:?}", mb), "MonogramBadge");
        assert_eq!(format!("{:?}", gp), "GeometricPattern");
    }

    #[test]
    fn preset_clone() {
        let mb = Preset::MonogramBadge;
        let cloned = mb;
        assert!(matches!(cloned, Preset::MonogramBadge));
    }

    #[test]
    fn preset_id() {
        assert_eq!(Preset::MonogramBadge.id(), "monogram-badge");
        assert_eq!(Preset::GeometricPattern.id(), "geometric-pattern");
    }

    #[test]
    fn preset_description() {
        let mb_desc = Preset::MonogramBadge.description();
        assert!(mb_desc.contains("badge"));
        assert!(mb_desc.contains("initials"));

        let gp_desc = Preset::GeometricPattern.description();
        assert!(gp_desc.contains("geometric"));
        assert!(gp_desc.contains("shapes"));
    }

    #[test]
    fn preset_category() {
        assert_eq!(Preset::MonogramBadge.category(), "Badge");
        assert_eq!(Preset::GeometricPattern.category(), "Abstract");
    }

    #[test]
    fn preset_all() {
        let presets = Preset::all();
        assert_eq!(presets.len(), 2);
        assert!(matches!(presets[0], Preset::MonogramBadge));
        assert!(matches!(presets[1], Preset::GeometricPattern));
    }

    #[test]
    fn preset_from_str_monogram_badge() {
        let p1: Preset = "monogram-badge".parse().expect("parse");
        assert!(matches!(p1, Preset::MonogramBadge));

        let p2: Preset = "monogram".parse().expect("parse");
        assert!(matches!(p2, Preset::MonogramBadge));

        let p3: Preset = "badge".parse().expect("parse");
        assert!(matches!(p3, Preset::MonogramBadge));

        let p4: Preset = "MONOGRAM-BADGE".parse().expect("parse");
        assert!(matches!(p4, Preset::MonogramBadge));

        let p5: Preset = "  monogram  ".parse().expect("parse");
        assert!(matches!(p5, Preset::MonogramBadge));
    }

    #[test]
    fn preset_from_str_geometric_pattern() {
        let p1: Preset = "geometric-pattern".parse().expect("parse");
        assert!(matches!(p1, Preset::GeometricPattern));

        let p2: Preset = "geometric".parse().expect("parse");
        assert!(matches!(p2, Preset::GeometricPattern));

        let p3: Preset = "pattern".parse().expect("parse");
        assert!(matches!(p3, Preset::GeometricPattern));

        let p4: Preset = "GEOMETRIC-PATTERN".parse().expect("parse");
        assert!(matches!(p4, Preset::GeometricPattern));
    }

    #[test]
    fn preset_from_str_unknown() {
        let r: Result<Preset, _> = "no-such-preset".parse();
        assert!(r.is_err());
        if let Err(LogoGenError::UnknownPreset(name)) = r {
            assert_eq!(name, "no-such-preset");
        } else {
            panic!("Expected UnknownPreset error");
        }
    }

    #[test]
    fn error_display_unknown_preset() {
        let err = LogoGenError::UnknownPreset("test".to_string());
        assert_eq!(err.to_string(), "unknown preset: test");
    }

    #[test]
    fn error_display_invalid_options() {
        let err = LogoGenError::InvalidOptions("bad size".to_string());
        assert_eq!(err.to_string(), "invalid options: bad size");
    }

    #[test]
    fn error_display_render() {
        let err = LogoGenError::Render("failed to draw".to_string());
        assert_eq!(err.to_string(), "render error: failed to draw");
    }

    #[test]
    fn error_debug() {
        let err = LogoGenError::UnknownPreset("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("UnknownPreset"));
    }

    #[test]
    fn generate_svg_monogram_badge() {
        let opts = RenderOptions::default();
        let svg =
            LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).expect("svg gen");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn generate_svg_geometric_pattern() {
        let opts = RenderOptions::default();
        let svg =
            LogoGenerator::generate_svg("Test", Preset::GeometricPattern, &opts).expect("svg gen");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn generate_svg_with_variant() {
        let opts = RenderOptions {
            variant: Some(42),
            ..Default::default()
        };
        let svg =
            LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).expect("svg gen");
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn generate_svg_transparent() {
        let opts = RenderOptions {
            transparent_background: true,
            ..Default::default()
        };
        let svg =
            LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).expect("svg gen");
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn generate_png_monogram_badge() {
        let opts = RenderOptions::default();
        let png =
            LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).expect("png gen");
        assert!(!png.is_empty());
        assert_eq!(&png[1..4], b"PNG");
    }

    #[test]
    fn generate_png_geometric_pattern() {
        let opts = RenderOptions::default();
        let png =
            LogoGenerator::generate_png("Test", Preset::GeometricPattern, &opts).expect("png gen");
        assert!(!png.is_empty());
        assert_eq!(&png[1..4], b"PNG");
    }

    #[test]
    fn generate_png_with_variant() {
        let opts = RenderOptions {
            variant: Some(99),
            ..Default::default()
        };
        let png =
            LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).expect("png gen");
        assert!(!png.is_empty());
    }

    #[test]
    fn generate_png_with_font_none() {
        let opts = RenderOptions::default();
        let png = LogoGenerator::generate_png_with_font("Test", Preset::MonogramBadge, &opts, None)
            .expect("png gen");
        assert!(!png.is_empty());
        assert_eq!(&png[1..4], b"PNG");
    }

    #[test]
    fn generate_png_with_owned_font_none() {
        let opts = RenderOptions::default();
        let png =
            LogoGenerator::generate_png_with_owned_font("Test", Preset::MonogramBadge, &opts, None)
                .expect("png gen");
        assert!(!png.is_empty());
        assert_eq!(&png[1..4], b"PNG");
    }

    #[test]
    fn generate_png_different_sizes() {
        let sizes = [128, 256, 512, 1024];
        for size in sizes {
            let opts = RenderOptions {
                size_px: size,
                ..Default::default()
            };
            let png =
                LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).expect("png gen");
            assert!(!png.is_empty());
        }
    }

    #[test]
    fn generate_png_with_padding() {
        let opts = RenderOptions {
            size_px: 256,
            padding_frac: 0.2,
            ..Default::default()
        };
        let png =
            LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).expect("png gen");
        assert!(!png.is_empty());
    }
}
