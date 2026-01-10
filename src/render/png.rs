use crate::algorithms::{DrawOp, Scene};
use crate::core::geometry::Shape;
use crate::{LogoGenError, RenderOptions};
use ab_glyph::{FontRef, PxScale};
use image::{ImageEncoder, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect as IRect;
use std::path::Path;

// Optional embedded font bytes controlled by the `embed-font` Cargo feature.
// In CI or environments where the font file is not present we fall back to
// `None` so tests/builds don't fail. If you need embedding, place the font
// at `assets/fonts/LiberationSans-Bold.ttf` and enable the feature.
const EMBED_FONT_BYTES: Option<&[u8]> = None;

pub fn render_png(
    scene: &Scene,
    _opts: &RenderOptions,
    font_bytes: Option<&[u8]>,
) -> Result<Vec<u8>, LogoGenError> {
    let mut img = RgbaImage::new(scene.width, scene.height);

    for px in img.pixels_mut() {
        *px = Rgba([0, 0, 0, 0]);
    }

    // Determine font to use: prefer `font_bytes` passed by caller (already
    // leaked to 'static), otherwise attempt to load a runtime font from
    // `assets/fonts/`, falling back to the embedded bytes if available.
    let font = if let Some(bytes) = font_bytes {
        FontRef::try_from_slice(bytes).ok()
    } else {
        let runtime_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/fonts/LiberationSans-Bold.ttf");
        if let Ok(bytes) = std::fs::read(&runtime_path) {
            // Leak the bytes so we can obtain a &'static slice for FontRef.
            let boxed = bytes.into_boxed_slice();
            let leaked: &'static [u8] = Box::leak(boxed);
            FontRef::try_from_slice(leaked)
                .ok()
                .or_else(|| EMBED_FONT_BYTES.and_then(|b| FontRef::try_from_slice(b).ok()))
        } else {
            EMBED_FONT_BYTES.and_then(|b| FontRef::try_from_slice(b).ok())
        }
    };

    for op in &scene.ops {
        match op {
            DrawOp::Background { color } => {
                if let Some(c) = color {
                    fill_rect(
                        &mut img,
                        0.0,
                        0.0,
                        scene.width as f32,
                        scene.height as f32,
                        Rgba([c.r, c.g, c.b, 255]),
                    );
                }
            }
            DrawOp::ShapeFill { shape, color } => {
                let rgba = Rgba([color.r, color.g, color.b, 255]);
                match shape {
                    Shape::Circle(circ) => {
                        draw_filled_circle_mut(
                            &mut img,
                            (circ.cx as i32, circ.cy as i32),
                            circ.r as i32,
                            rgba,
                        );
                    }
                    Shape::Rect { rect, rx, ry } => {
                        if *rx > 0.0 || *ry > 0.0 {
                            draw_rounded_rect(&mut img, rect.x, rect.y, rect.w, rect.h, *rx, rgba);
                        } else if rect.w > 0.0 && rect.h > 0.0 {
                            draw_filled_rect_mut(
                                &mut img,
                                IRect::at(rect.x as i32, rect.y as i32)
                                    .of_size(rect.w as u32, rect.h as u32),
                                rgba,
                            );
                        }
                    }
                }
            }
            DrawOp::Text {
                text,
                x,
                y,
                font_size,
                color,
                anchor_middle,
                ..
            } => {
                // If font failed to load, skip drawing text rather than erroring
                if let Some(ref font) = font {
                    let scale = PxScale::from(*font_size);
                    let rgba = Rgba([color.r, color.g, color.b, 255]);
                    let (text_x, text_y) = if *anchor_middle {
                        let text_width = measure_text_width(font, scale, text);
                        ((x - text_width / 2.0) as i32, (y - font_size * 0.35) as i32)
                    } else {
                        (*x as i32, *y as i32)
                    };
                    draw_text_mut(&mut img, rgba, text_x, text_y, scale, &font, text);
                }
            }
        }
    }

    let mut buf = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buf);
        encoder
            .write_image(
                &img,
                scene.width,
                scene.height,
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e: image::ImageError| LogoGenError::Render(e.to_string()))?;
    }
    Ok(buf)
}

fn fill_rect(img: &mut RgbaImage, x: f32, y: f32, w: f32, h: f32, rgba: Rgba<u8>) {
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let x0 = x.max(0.0).floor() as i32;
    let y0 = y.max(0.0).floor() as i32;
    let x1 = (x + w).min(img.width() as f32).ceil() as i32;
    let y1 = (y + h).min(img.height() as f32).ceil() as i32;
    for yy in y0..y1 {
        for xx in x0..x1 {
            if xx >= 0 && yy >= 0 && xx < img.width() as i32 && yy < img.height() as i32 {
                img.put_pixel(xx as u32, yy as u32, rgba);
            }
        }
    }
}

fn draw_rounded_rect(
    img: &mut RgbaImage,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    rgba: Rgba<u8>,
) {
    let r = radius.min(w / 2.0).min(h / 2.0);
    fill_rect(img, x + r, y, w - 2.0 * r, h, rgba);
    fill_rect(img, x, y + r, r, h - 2.0 * r, rgba);
    fill_rect(img, x + w - r, y + r, r, h - 2.0 * r, rgba);
    draw_rounded_corner(img, x + r, y + r, r, rgba, true, true);
    draw_rounded_corner(img, x + w - r, y + r, r, rgba, false, true);
    draw_rounded_corner(img, x + r, y + h - r, r, rgba, true, false);
    draw_rounded_corner(img, x + w - r, y + h - r, r, rgba, false, false);
}

fn draw_rounded_corner(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    radius: f32,
    rgba: Rgba<u8>,
    left: bool,
    top: bool,
) {
    let r2 = radius * radius;
    let r_i = radius.floor() as i32;
    for dy in -r_i..=r_i {
        for dx in -r_i..=r_i {
            let dist2 = (dx as f32) * (dx as f32) + (dy as f32) * (dy as f32);
            if dist2 <= r2 {
                let px = cx + dx as f32;
                let py = cy + dy as f32;
                let in_quadrant = if left && top {
                    dx <= 0 && dy <= 0
                } else if !left && top {
                    dx >= 0 && dy <= 0
                } else if left && !top {
                    dx <= 0 && dy >= 0
                } else {
                    dx >= 0 && dy >= 0
                };
                if in_quadrant
                    && px >= 0.0
                    && py >= 0.0
                    && (px as u32) < img.width()
                    && (py as u32) < img.height()
                {
                    let dist = dist2.sqrt();
                    let alpha = if dist > radius - 1.0 {
                        ((radius - dist) * 255.0).clamp(0.0, 255.0) as u8
                    } else {
                        255
                    };
                    let pixel = img.get_pixel_mut(px as u32, py as u32);
                    *pixel = blend_rgba(*pixel, rgba, alpha);
                }
            }
        }
    }
}

fn blend_rgba(bg: Rgba<u8>, fg: Rgba<u8>, alpha: u8) -> Rgba<u8> {
    if alpha == 0 {
        return bg;
    }
    if alpha == 255 {
        return fg;
    }
    let a = alpha as f32 / 255.0;
    let inv_a = 1.0 - a;
    Rgba([
        (fg[0] as f32 * a + bg[0] as f32 * inv_a) as u8,
        (fg[1] as f32 * a + bg[1] as f32 * inv_a) as u8,
        (fg[2] as f32 * a + bg[2] as f32 * inv_a) as u8,
        255,
    ])
}

fn measure_text_width(font: &FontRef, scale: PxScale, text: &str) -> f32 {
    use ab_glyph::{Font, ScaleFont};
    let scaled_font = font.as_scaled(scale);
    let mut width = 0.0;
    for ch in text.chars() {
        let glyph_id = font.glyph_id(ch);
        width += scaled_font.h_advance(glyph_id);
    }
    width
}
