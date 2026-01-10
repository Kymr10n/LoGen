use logo_gen::{algorithms, Preset, RenderOptions};

fn main() {
    let input = "ABR";
    let opts = RenderOptions {
        size_px: 512,
        padding_frac: 0.08,
        variant: Some(1),
        transparent_background: false,
    };

    match algorithms::build_scene(input, Preset::MonogramBadge, &opts) {
        Ok(scene) => {
            println!("Scene width={} height={}", scene.width, scene.height);
            for op in scene.ops.iter() {
                match op {
                    logo_gen::algorithms::DrawOp::Text { text, x, y, font_size, .. } => {
                        println!("Text op: '{}' @ ({},{}) size={}", text, x, y, font_size);
                    }
                    _ => {}
                }
            }
            // Render SVG and print the exact <text> line
            match logo_gen::render::svg::render_svg(&scene, &opts) {
                Ok(svg) => {
                    for line in svg.lines() {
                        if line.contains("<text ") {
                            println!("SVG text line: {}", line);
                        }
                    }
                }
                Err(e) => eprintln!("SVG render error: {}", e),
            }
        }
        Err(e) => eprintln!("Error building scene: {}", e),
    }
}
