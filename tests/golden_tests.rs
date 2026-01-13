use logen::{LoGen, Preset, RenderOptions};
use std::fs;

/// Golden test: ensures output remains stable across code changes.
/// If this fails after an intentional change, regenerate the golden files.
const GOLDEN_DIR: &str = "tests/golden";

#[test]
fn test_monogram_badge_golden_svg() {
    let opts = RenderOptions {
        size_px: 256,
        padding_frac: 0.12,
        variant: Some(42), // Fixed seed for determinism
        transparent_background: false,
    };

    let svg =
        LoGen::generate_svg("Acme Corp", Preset::MonogramBadge, &opts).expect("svg generation");

    let golden_path = format!("{}/monogram_badge_256.svg", GOLDEN_DIR);

    if !std::path::Path::new(&golden_path).exists() {
        // Generate golden file if it doesn't exist
        fs::create_dir_all(GOLDEN_DIR).ok();
        fs::write(&golden_path, &svg).expect("write golden file");
        println!("Generated golden file: {}", golden_path);
    } else {
        let golden = fs::read_to_string(&golden_path).expect("read golden file");
        assert_eq!(
            svg, golden,
            "SVG output differs from golden file. If this is intentional, delete the golden file and rerun tests."
        );
    }
}

#[test]
fn test_geometric_pattern_golden_svg() {
    let opts = RenderOptions {
        size_px: 256,
        padding_frac: 0.12,
        variant: Some(99), // Fixed seed
        transparent_background: false,
    };

    let svg =
        LoGen::generate_svg("TestCo", Preset::GeometricPattern, &opts).expect("svg generation");

    let golden_path = format!("{}/geometric_pattern_256.svg", GOLDEN_DIR);

    if !std::path::Path::new(&golden_path).exists() {
        fs::create_dir_all(GOLDEN_DIR).ok();
        fs::write(&golden_path, &svg).expect("write golden file");
        println!("Generated golden file: {}", golden_path);
    } else {
        let golden = fs::read_to_string(&golden_path).expect("read golden file");
        assert_eq!(
            svg, golden,
            "SVG output differs from golden file. If this is intentional, delete the golden file and rerun tests."
        );
    }
}
