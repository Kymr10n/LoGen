use logo_gen::{LogoGenerator, Preset, RenderOptions};

#[test]
fn svg_is_deterministic_for_same_input() {
    let opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let a = LogoGenerator::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    let b = LogoGenerator::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    assert_eq!(a, b);
}

#[test]
fn variant_changes_output() {
    let mut opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    opts.variant = Some(1);
    let a = LogoGenerator::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    opts.variant = Some(2);
    let b = LogoGenerator::generate_svg("Acme Power", Preset::MonogramBadge, &opts).unwrap();
    assert_ne!(a, b);
}
