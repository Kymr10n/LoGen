use std::fs;
use std::path::Path;

use crate::{LogoGenError, LogoGenerator, OutputFormat, Preset, RenderOptions};

/// Write a generated logo to `out_path` using provided format and options.
pub fn write_logo_file(
    input: &str,
    preset: Preset,
    format: OutputFormat,
    out_path: &Path,
    opts: &RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Svg => {
            let svg = LogoGenerator::generate_svg(input, preset, opts)?;
            fs::write(out_path, svg)?;
        }
        OutputFormat::Png => {
            let png = LogoGenerator::generate_png(input, preset, opts)?;
            fs::write(out_path, png)?;
        }
    }
    Ok(())
}

/// Produce the SVG string used by the `debug_initials` binary.
pub fn debug_initials_svg(input: &str, opts: &RenderOptions) -> Result<String, LogoGenError> {
    let scene = crate::algorithms::build_scene(input, Preset::MonogramBadge, opts)?;
    crate::render::svg::render_svg(&scene, opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn write_svg_file_creates_file() {
        let opts = RenderOptions::default();
        let out = env::temp_dir().join("logen_test_out.svg");
        let _ = fs::remove_file(&out);
        write_logo_file(
            "TestSVG",
            Preset::MonogramBadge,
            OutputFormat::Svg,
            &out,
            &opts,
        )
        .expect("write svg");
        let s = fs::read_to_string(&out).expect("read svg");
        assert!(s.contains("<svg"));
        let _ = fs::remove_file(&out);
    }

    #[test]
    fn debug_initials_returns_svg() {
        let opts = RenderOptions::default();
        let svg = debug_initials_svg("ABR", &opts).expect("svg");
        assert!(svg.contains("<svg") || svg.contains("<svg"));
    }
}
