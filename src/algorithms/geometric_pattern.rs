use rand::Rng;

use super::{DrawOp, Scene};
use crate::core::{geometry, palette, typography};
use crate::{LogoGenError, RenderOptions};

/// Probability of using circles instead of rectangles.
const CIRCLE_PROBABILITY: f64 = 0.4;

/// Minimum number of geometric shapes to generate.
const MIN_SHAPES: usize = 3;
/// Maximum number of geometric shapes to generate.
const MAX_SHAPES: usize = 6;

/// Minimum shape size as fraction of canvas.
const MIN_SHAPE_SIZE: f32 = 0.15;
/// Maximum shape size as fraction of canvas.
const MAX_SHAPE_SIZE: f32 = 0.35;

/// Text size as fraction of canvas width.
const TEXT_SIZE_FRACTION: f32 = 0.18;

/// Simple geometric pattern: overlapping shapes with a centered lettermark.
pub fn build<R: Rng>(
    normalized: &str,
    rng: &mut R,
    opts: &RenderOptions,
) -> Result<Scene, LogoGenError> {
    let size = opts.size_px;
    let w = size as f32;
    let h = size as f32;

    let palette = palette::derive_palette(rng, opts.transparent_background);
    let typo = typography::Typography::default();

    let num_shapes = rng.gen_range(MIN_SHAPES..=MAX_SHAPES);
    let mut ops = vec![DrawOp::Background {
        color: palette.background,
    }];

    // Generate random geometric shapes
    for i in 0..num_shapes {
        let shape_w = rng.gen_range(MIN_SHAPE_SIZE..MAX_SHAPE_SIZE) * w;
        let shape_h = rng.gen_range(MIN_SHAPE_SIZE..MAX_SHAPE_SIZE) * h;

        let x = rng.gen_range(0.0..=(w - shape_w));
        let y = rng.gen_range(0.0..=(h - shape_h));

        // Vary between primary, secondary, and tertiary colors
        let color = match i % 3 {
            0 => palette.primary,
            1 => palette.secondary,
            _ => palette.tertiary,
        };

        let shape = if rng.gen_bool(CIRCLE_PROBABILITY) {
            geometry::Shape::Circle(geometry::Circle {
                cx: x + shape_w / 2.0,
                cy: y + shape_h / 2.0,
                r: shape_w.min(shape_h) / 2.0,
            })
        } else {
            let corner_radius = rng.gen_range(0.0..0.3) * shape_w.min(shape_h);
            geometry::Shape::Rect {
                rect: geometry::Rect {
                    x,
                    y,
                    w: shape_w,
                    h: shape_h,
                },
                rx: corner_radius,
                ry: corner_radius,
            }
        };

        ops.push(DrawOp::ShapeFill { shape, color });
    }

    // Extract first letter or first two letters for lettermark
    let lettermark: String = normalized
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let lettermark = if lettermark.is_empty() {
        "?".to_string()
    } else {
        lettermark
    };

    // Add centered text on top
    let font_size = TEXT_SIZE_FRACTION * w;
    ops.push(DrawOp::Text {
        text: lettermark,
        x: w / 2.0,
        y: h / 2.0 + font_size * 0.35,
        font_family: typo.family.to_string(),
        font_weight: 700,
        font_size,
        color: palette.text_color,
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
    use crate::RenderOptions;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn build_returns_scene_with_shapes() {
        let mut rng = ChaCha8Rng::seed_from_u64(456);
        let opts = RenderOptions {
            size_px: 256,
            padding_frac: 0.1,
            variant: None,
            transparent_background: false,
        };
        let scene = build("TestCompany", &mut rng, &opts).expect("build failed");

        assert_eq!(scene.width, 256);
        assert_eq!(scene.height, 256);

        // Should have background + multiple shapes + text
        assert!(scene.ops.len() >= 5);

        let has_text = scene
            .ops
            .iter()
            .any(|op| matches!(op, crate::algorithms::DrawOp::Text { .. }));
        assert!(has_text, "expected a Text draw op in the scene");
    }
}
