use logen::{LogoGenError, LogoGenerator, Preset, RenderOptions};

#[test]
fn test_empty_input() {
    let opts = RenderOptions::default();
    let result = LogoGenerator::generate_svg("", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_whitespace_only_input() {
    let opts = RenderOptions::default();
    let result = LogoGenerator::generate_svg("   \t\n  ", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_padding_too_large() {
    let opts = RenderOptions {
        padding_frac: 0.6,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_padding_negative() {
    let opts = RenderOptions {
        padding_frac: -0.1,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_size_too_small() {
    let opts = RenderOptions {
        size_px: 32,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_size_too_large() {
    let opts = RenderOptions {
        size_px: 10000,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(matches!(result, Err(LogoGenError::InvalidOptions(_))));
}

#[test]
fn test_valid_edge_case_padding() {
    let opts = RenderOptions {
        padding_frac: 0.5,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(result.is_ok());
}

#[test]
fn test_valid_edge_case_size_min() {
    let opts = RenderOptions {
        size_px: 64,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(result.is_ok());
}

#[test]
fn test_valid_edge_case_size_max() {
    let opts = RenderOptions {
        size_px: 8192,
        ..Default::default()
    };
    let result = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts);
    assert!(result.is_ok());
}
