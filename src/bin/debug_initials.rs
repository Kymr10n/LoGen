use logo_gen::RenderOptions;
use logo_gen::cli::debug_initials_svg;

fn main() {
    let input = "ABR";
    let opts = RenderOptions {
        size_px: 512,
        padding_frac: 0.08,
        variant: Some(1),
        transparent_background: false,
    };

    match debug_initials_svg(input, &opts) {
        Ok(svg) => {
            for line in svg.lines() {
                if line.contains("<text ") {
                    println!("SVG text line: {}", line);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
