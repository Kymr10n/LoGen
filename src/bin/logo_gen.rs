use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use logo_gen::{LogoGenerator, Preset, RenderOptions, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
enum FormatArg {
    Svg,
    Png,
}

impl From<FormatArg> for OutputFormat {
    fn from(v: FormatArg) -> Self {
        match v {
            FormatArg::Svg => OutputFormat::Svg,
            FormatArg::Png => OutputFormat::Png,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "logo-gen")]
#[command(about = "Deterministic logo generator (PNG + SVG) — stub", long_about = None)]
struct Args {
    /// Input string used to generate the logo deterministically.
    #[arg(long)]
    input: String,

    /// Preset / algorithm identifier (e.g. monogram-badge).
    #[arg(long, default_value = "monogram-badge")]
    preset: String,

    /// Output format.
    #[arg(long, value_enum, default_value_t = FormatArg::Svg)]
    format: FormatArg,

    /// Output file path.
    #[arg(long)]
    out: PathBuf,

    /// Output size in pixels (PNG; also used as SVG dimensions).
    #[arg(long, default_value_t = 512)]
    size: u32,

    /// Padding as fraction of canvas size.
    #[arg(long, default_value_t = 0.12)]
    padding: f32,

    /// Optional variant for same input (allows different deterministic outputs).
    #[arg(long)]
    variant: Option<u64>,

    /// Transparent background (PNG and SVG).
    #[arg(long, default_value_t = false)]
    transparent: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let preset: Preset = args.preset.parse()?;

    let opts = RenderOptions {
        size_px: args.size,
        padding_frac: args.padding,
        variant: args.variant,
        transparent_background: args.transparent,
    };

    match OutputFormat::from(args.format) {
        OutputFormat::Svg => {
            let svg = LogoGenerator::generate_svg(&args.input, preset, &opts)?;
            fs::write(&args.out, svg)?;
        }
        OutputFormat::Png => {
            let png = LogoGenerator::generate_png(&args.input, preset, &opts)?;
            fs::write(&args.out, png)?;
        }
    }

    Ok(())
}
