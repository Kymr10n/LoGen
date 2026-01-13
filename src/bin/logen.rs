use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use logen::cli::write_logo_file;
use logen::{OutputFormat, Preset, RenderOptions};

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
#[command(name = "LoGen")]
#[command(about = "Deterministic logo generator (PNG + SVG)", long_about = None)]
struct Args {
    /// List all available presets and exit.
    #[arg(long)]
    list_presets: bool,

    /// Input string used to generate the logo deterministically.
    #[arg(long)]
    input: Option<String>,

    /// Preset / algorithm identifier (e.g. monogram-badge).
    #[arg(long, default_value = "monogram-badge")]
    preset: String,

    /// Output format.
    #[arg(long, value_enum, default_value_t = FormatArg::Svg)]
    format: FormatArg,

    /// Output file path.
    #[arg(long)]
    out: Option<PathBuf>,

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

    if args.list_presets {
        println!("Available presets:\n");
        for preset in Preset::all() {
            println!("  {} [{}]", preset.id(), preset.category());
            println!("    {}", preset.description());
            println!();
        }
        return Ok(());
    }

    let input = args
        .input
        .ok_or("--input is required (or use --list-presets)")?;
    let out = args.out.ok_or("--out is required")?;
    let preset: Preset = args.preset.parse()?;

    let opts = RenderOptions {
        size_px: args.size,
        padding_frac: args.padding,
        variant: args.variant,
        transparent_background: args.transparent,
    };

    write_logo_file(&input, preset, OutputFormat::from(args.format), &out, &opts)?;

    Ok(())
}
