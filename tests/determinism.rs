use logen::{LoGen, Preset, RenderOptions};

#[test]
fn svg_is_deterministic_for_same_input() {
    let opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let a = LoGen::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    let b = LoGen::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    assert_eq!(a, b);
}

#[test]
fn png_is_deterministic_for_same_input() {
    let opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let a = LoGen::generate_png("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    let b = LoGen::generate_png("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    assert_eq!(a, b);
}

#[test]
fn variant_changes_output() {
    let mut opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    opts.variant = Some(1);
    let a = LoGen::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    opts.variant = Some(2);
    let b = LoGen::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    assert_ne!(a, b);
}

#[test]
fn variant_png_changes_output() {
    let mut opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    opts.variant = Some(1);
    let a = LoGen::generate_png("Test", Preset::MonogramBadge, &opts).unwrap();
    opts.variant = Some(2);
    let b = LoGen::generate_png("Test", Preset::MonogramBadge, &opts).unwrap();
    assert_ne!(a, b);
}

#[test]
fn different_inputs_produce_different_outputs() {
    let opts = RenderOptions::default();
    let a = LoGen::generate_svg("Input A", Preset::MonogramBadge, &opts).unwrap();
    let b = LoGen::generate_svg("Input B", Preset::MonogramBadge, &opts).unwrap();
    assert_ne!(a, b);
}

#[test]
fn same_input_different_sizes_consistent() {
    let opts_256 = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let opts_512 = RenderOptions {
        size_px: 512,
        ..Default::default()
    };
    let a1 = LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_256).unwrap();
    let a2 = LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_256).unwrap();
    let b1 = LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_512).unwrap();
    let b2 = LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_512).unwrap();

    // Same size should be identical
    assert_eq!(a1, a2);
    assert_eq!(b1, b2);
    // Different sizes should differ
    assert_ne!(a1, b1);
}

#[test]
fn transparency_setting_changes_output() {
    let opts_opaque = RenderOptions {
        transparent_background: false,
        ..Default::default()
    };
    let opaque = LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_opaque).unwrap();

    let opts_transparent = RenderOptions {
        transparent_background: true,
        ..Default::default()
    };
    let transparent =
        LoGen::generate_svg("Test", Preset::MonogramBadge, &opts_transparent).unwrap();

    assert_ne!(opaque, transparent);
}
