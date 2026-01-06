use crate::algorithms::{DrawOp, Scene};
use crate::core::geometry::Shape;
use crate::{LogoGenError, RenderOptions};

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn render_svg(scene: &Scene, _opts: &RenderOptions) -> Result<String, LogoGenError> {
    let w = scene.width;
    let h = scene.height;

    let mut out = String::new();
    out.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    out.push('\n');
    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#,
    ));
    out.push('\n');

    for op in &scene.ops {
        match op {
            DrawOp::Background { color } => {
                if let Some(c) = color {
                    out.push_str(&format!(
                        r#"<rect x="0" y="0" width="{w}" height="{h}" fill="{}"/>"#,
                        c.to_hex()
                    ));
                    out.push('\n');
                }
            }
            DrawOp::ShapeFill { shape, color } => match shape {
                Shape::Circle(circ) => {
                    out.push_str(&format!(
                        r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}"/>"#,
                        circ.cx,
                        circ.cy,
                        circ.r,
                        color.to_hex()
                    ));
                    out.push('\n');
                }
                Shape::Rect { rect, rx, ry } => {
                    out.push_str(&format!(
                        r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="{:.2}" ry="{:.2}" fill="{}"/>"#,
                        rect.x, rect.y, rect.w, rect.h, rx, ry, color.to_hex()
                    ));
                    out.push('\n');
                }
            },
            DrawOp::Text {
                text,
                x,
                y,
                font_family,
                font_weight,
                font_size,
                color,
                anchor_middle,
            } => {
                let anchor = if *anchor_middle { "middle" } else { "start" };
                out.push_str(&format!(
                    r#"<text x="{:.2}" y="{:.2}" text-anchor="{anchor}" font-family="{}" font-weight="{}" font-size="{:.2}" fill="{}">{}</text>"#,
                    x, y, esc(font_family), font_weight, font_size, color.to_hex(), esc(text)
                ));
                out.push('\n');
            }
        }
    }

    out.push_str("</svg>\n");
    Ok(out)
}
