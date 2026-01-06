use crate::{LogoGenError, RenderOptions};
use crate::algorithms::{Scene, DrawOp};
use crate::core::geometry::Shape;
use image::ImageEncoder;

/// NOTE: This is a **placeholder** PNG renderer.
/// It currently draws only the background and a filled shape.
/// Text is intentionally skipped (adding robust font rendering is a later issue).
pub fn render_png(scene: &Scene, _opts: &RenderOptions) -> Result<Vec<u8>, LogoGenError> {
    let mut img = image::RgbaImage::new(scene.width, scene.height);

    // Default transparent
    for px in img.pixels_mut() {
        *px = image::Rgba([0, 0, 0, 0]);
    }

    for op in &scene.ops {
        match op {
            DrawOp::Background { color } => {
                if let Some(c) = color {
                    fill_rect(&mut img, 0.0, 0.0, scene.width as f32, scene.height as f32, [c.r, c.g, c.b, 255]);
                }
            }
            DrawOp::ShapeFill { shape, color } => {
                match shape {
                    Shape::Circle(circ) => fill_circle(&mut img, circ.cx, circ.cy, circ.r, [color.r, color.g, color.b, 255]),
                    Shape::Rect { rect, rx: _, ry: _ } => fill_rect(&mut img, rect.x, rect.y, rect.w, rect.h, [color.r, color.g, color.b, 255]),
                }
            }
            DrawOp::Text { .. } => {
                // Intentionally not rendered in this stub.
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

fn fill_rect(img: &mut image::RgbaImage, x: f32, y: f32, w: f32, h: f32, rgba: [u8; 4]) {
    let x0 = x.max(0.0).floor() as i32;
    let y0 = y.max(0.0).floor() as i32;
    let x1 = (x + w).min(img.width() as f32).ceil() as i32;
    let y1 = (y + h).min(img.height() as f32).ceil() as i32;

    for yy in y0..y1 {
        for xx in x0..x1 {
            if xx >= 0 && yy >= 0 {
                img.put_pixel(xx as u32, yy as u32, image::Rgba(rgba));
            }
        }
    }
}

fn fill_circle(img: &mut image::RgbaImage, cx: f32, cy: f32, r: f32, rgba: [u8; 4]) {
    let r2 = r * r;
    let x0 = (cx - r).max(0.0).floor() as i32;
    let y0 = (cy - r).max(0.0).floor() as i32;
    let x1 = (cx + r).min(img.width() as f32 - 1.0).ceil() as i32;
    let y1 = (cy + r).min(img.height() as f32 - 1.0).ceil() as i32;

    for yy in y0..=y1 {
        for xx in x0..=x1 {
            let dx = xx as f32 + 0.5 - cx;
            let dy = yy as f32 + 0.5 - cy;
            if (dx * dx + dy * dy) <= r2 {
                img.put_pixel(xx as u32, yy as u32, image::Rgba(rgba));
            }
        }
    }
}
