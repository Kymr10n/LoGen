use logo_gen::{LogoGenerator, Preset, RenderOptions};

#[test]
fn test_svg_output_starts_with_xml_declaration() {
    let opts = RenderOptions::default();
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.starts_with("<?xml"));
}

#[test]
fn test_svg_contains_svg_tag() {
    let opts = RenderOptions::default();
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_svg_contains_dimensions() {
    let opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.contains("width=\"256\""));
    assert!(svg.contains("height=\"256\""));
}

#[test]
fn test_png_output_is_valid() {
    let opts = RenderOptions::default();
    let png = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).unwrap();
    // PNG signature: 89 50 4E 47 0D 0A 1A 0A
    assert_eq!(
        &png[0..8],
        &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    );
}

#[test]
fn test_transparent_background_svg() {
    let opts = RenderOptions {
        transparent_background: true,
        ..Default::default()
    };
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    // Should not contain a background rect
    let rect_count = svg.matches("<rect").count();
    // Should have shape rect but not background rect
    assert_eq!(rect_count, 1);
}

#[test]
fn test_opaque_background_svg() {
    let opts = RenderOptions {
        transparent_background: false,
        ..Default::default()
    };
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    // Should contain background rect
    let rect_count = svg.matches("<rect").count();
    // Should have both background and shape rect
    assert_eq!(rect_count, 2);
}
