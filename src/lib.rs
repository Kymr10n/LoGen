//! Deterministic logo generation library.
//!
//! This library generates consistent, reproducible logos from input strings using
//! cryptographic hashing and seeded randomization.

pub mod algorithms;
pub mod core;
pub mod render;
pub mod cli;

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
}

impl Preset {
    pub fn id(&self) -> &'static str {
        match self {
            Preset::MonogramBadge => "monogram-badge",
        }
    }
}

impl std::str::FromStr for Preset {
    type Err = LogoGenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "monogram-badge" | "monogram" | "badge" => Ok(Preset::MonogramBadge),
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
    fn preset_from_str_known() {
        let p: Preset = "monogram-badge".parse().expect("parse preset");
        match p {
            Preset::MonogramBadge => {}
        }
    }

    #[test]
    fn preset_from_str_unknown() {
        let r: Result<Preset, _> = "no-such-preset".parse();
        assert!(r.is_err());
    }
    #[test]
    fn generate_svg_and_png_ok() {
        let opts = RenderOptions::default();
        let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).expect("svg gen");
        assert!(svg.contains("<svg") || svg.contains("<svg"), "svg should contain svg tag");

        let png = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).expect("png gen");
        assert!(!png.is_empty(), "png bytes should not be empty");
    }
}
