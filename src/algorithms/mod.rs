//! Logo generation algorithms and scene graph.
//!
//! Each algorithm (preset) builds a scene graph from normalized input
//! and a seeded RNG for deterministic output.

use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

use crate::{LogoGenError, Preset, RenderOptions};
use crate::core::seed::{normalize_input, derive_seed_32};

pub mod monogram_badge;

/// Minimal scene graph for this stub.
#[derive(Debug, Clone)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub ops: Vec<DrawOp>,
}

#[derive(Debug, Clone)]
pub enum DrawOp {
    Background { color: Option<crate::core::palette::Rgb> },
    ShapeFill { shape: crate::core::geometry::Shape, color: crate::core::palette::Rgb },
    Text {
        text: String,
        x: f32,
        y: f32,
        font_family: String,
        font_weight: u16,
        font_size: f32,
        color: crate::core::palette::Rgb,
        anchor_middle: bool,
    },
}

pub fn build_scene(input: &str, preset: Preset, opts: &RenderOptions) -> Result<Scene, LogoGenError> {
    if !(0.0..=0.5).contains(&opts.padding_frac) {
        return Err(LogoGenError::InvalidOptions(
            format!("padding_frac must be within [0.0..0.5], got {}", opts.padding_frac)
        ));
    }
    if opts.size_px < 64 || opts.size_px > 8192 {
        return Err(LogoGenError::InvalidOptions(
            format!("size_px must be within [64..8192], got {}", opts.size_px)
        ));
    }

    let normalized = normalize_input(input);
    if normalized.is_empty() {
        return Err(LogoGenError::InvalidOptions(
            "input string is empty or contains only whitespace".into()
        ));
    }
    let seed = derive_seed_32(&normalized, opts.variant);
    let mut rng = ChaCha20Rng::from_seed(seed);

    match preset {
        Preset::MonogramBadge => monogram_badge::build(&normalized, &mut rng, opts),
    }
}
