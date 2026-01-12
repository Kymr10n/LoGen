use rand::Rng;

use super::{DrawOp, Scene};
use crate::core::{geometry, palette, typography};
use crate::{LogoGenError, RenderOptions};

/// Probability of generating a circular badge instead of rounded rectangle.
const CIRCLE_PROBABILITY: f64 = 0.35;

/// Probability of adding a border/stroke to the badge.
const BORDER_PROBABILITY: f64 = 0.5;

/// Border width as fraction of shape size.
const BORDER_WIDTH_FRACTION: f32 = 0.025;

/// Minimum corner radius as fraction of badge width.
const MIN_CORNER_RADIUS: f32 = 0.16;
/// Maximum corner radius as fraction of badge width.
const MAX_CORNER_RADIUS: f32 = 0.22;

/// Minimum font size as fraction of canvas width.
const MIN_FONT_SIZE: f32 = 0.52;
/// Maximum font size as fraction of canvas width.
const MAX_FONT_SIZE: f32 = 0.62;

/// Baseline adjustment factor for vertical text centering.
const TEXT_BASELINE_ADJUST: f32 = 0.35;

fn initials_from_normalized(s: &str) -> String {
    // If input contains multiple words, take the first alnum letter of the
    // first two words (classic initials). If the input is a single word,
    // include up to three alphanumeric characters from that word (so
    // e.g. "ABR" becomes "ABR"). Fallback to first two alnum chars or
    // "?" if nothing is available.
    let words: Vec<&str> = s.split(' ').filter(|w| !w.is_empty()).collect();
    let mut init = String::new();

    if words.len() >= 2 {
        for w in words.iter().take(2) {
            if let Some(ch) = w.chars().find(|c| c.is_alphanumeric()) {
                init.push(ch);
            }
        }
    } else {
        // Single-word: include up to 3 alphanumeric characters from the
        // normalized string to preserve short names like "ABR".
        for ch in s.chars().filter(|c| c.is_alphanumeric()).take(3) {
            init.push(ch);
        }
    }

    if init.is_empty() {
        for ch in s.chars().filter(|c| c.is_alphanumeric()).take(2) {
            init.push(ch);
        }
    }

    if init.is_empty() {
        init.push('?');
    }

    init.to_uppercase()
}

/// Simple "Monogram Badge" preset: rounded rect + initials.
/// All choices are deterministic via the supplied RNG.
pub fn build<R: Rng>(
    normalized: &str,
    rng: &mut R,
    opts: &RenderOptions,
) -> Result<Scene, LogoGenError> {
    let size = opts.size_px;
    let w = size as f32;
    let h = size as f32;

    let pad = (opts.padding_frac * w).round();
    let inner = geometry::Rect {
        x: pad,
        y: pad,
        w: w - 2.0 * pad,
        h: h - 2.0 * pad,
    };

    let palette = palette::derive_palette(rng, opts.transparent_background);
    let typo = typography::Typography::default();

    // Badge shape variation (rounded rect vs circle) — keep constrained.
    let use_circle = rng.gen_bool(CIRCLE_PROBABILITY);
    let badge_shape = if use_circle {
        geometry::Shape::Circle(geometry::Circle {
            cx: w / 2.0,
            cy: h / 2.0,
            r: (inner.w.min(inner.h) / 2.0),
        })
    } else {
        let rx = rng.gen_range(MIN_CORNER_RADIUS..MAX_CORNER_RADIUS) * inner.w;
        geometry::Shape::Rect {
            rect: inner,
            rx,
            ry: rx,
        }
    };

    let initials = initials_from_normalized(normalized);
    let font_size = rng.gen_range(MIN_FONT_SIZE..MAX_FONT_SIZE) * w;

    let add_border = rng.gen_bool(BORDER_PROBABILITY);
    let mut ops = vec![
        DrawOp::Background {
            color: palette.background,
        },
        DrawOp::ShapeFill {
            shape: badge_shape,
            color: palette.primary,
        },
    ];

    // Optionally add a border
    if add_border {
        let border_width = w * BORDER_WIDTH_FRACTION;
        ops.push(DrawOp::ShapeStroke {
            shape: badge_shape,
            color: palette.tertiary,
            width: border_width,
        });
    }

    // Centered text
    ops.push(DrawOp::Text {
        text: initials,
        x: w / 2.0,
        y: h / 2.0 + font_size * TEXT_BASELINE_ADJUST,
        font_family: typo.family.to_string(),
        font_weight: typo.weight,
        font_size,
        color: palette.secondary,
        anchor_middle: true,
    });

    Ok(Scene {
        width: size,
        height: size,
        ops,
    })
}

#[cfg(test)]
mod tests {
    use super::build;
    use super::initials_from_normalized;
    use crate::RenderOptions;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn initials_multiple_words() {
        let s = "Alice Bob";
        assert_eq!(initials_from_normalized(s), "AB");
    }

    #[test]
    fn initials_single_word_three_chars() {
        let s = "abr";
        assert_eq!(initials_from_normalized(s), "ABR");
    }

    #[test]
    fn initials_no_alnum() {
        let s = "!! --";
        assert_eq!(initials_from_normalized(s), "?");
    }

    #[test]
    fn build_returns_scene_with_text() {
        let mut rng = ChaCha8Rng::seed_from_u64(123);
        let opts = RenderOptions {
            size_px: 128,
            padding_frac: 0.1,
            variant: None,
            transparent_background: false,
        };
        let scene = build("Alice", &mut rng, &opts).expect("build failed");
        // there should be a text op present and the width/height match
        assert_eq!(scene.width, 128);
        assert_eq!(scene.height, 128);
        let has_text = scene
            .ops
            .iter()
            .any(|op| matches!(op, crate::algorithms::DrawOp::Text { .. }));
        assert!(has_text, "expected a Text draw op in the scene");
    }
}
