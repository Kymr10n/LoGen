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

#[test]
fn test_svg_contains_text_element() {
    let opts = RenderOptions::default();
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.contains("<text"));
}

#[test]
fn test_png_size_256() {
    let opts = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let png = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).unwrap();
    // Should have PNG header
    assert!(png[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    // Should have reasonable size (not empty)
    assert!(png.len() > 1000);
}

#[test]
fn test_png_size_1024() {
    let opts = RenderOptions {
        size_px: 1024,
        ..Default::default()
    };
    let png = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(png[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    // Larger size should produce larger file
    assert!(png.len() > 5000);
}

#[test]
fn test_svg_viewbox() {
    let opts = RenderOptions {
        size_px: 512,
        ..Default::default()
    };
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.contains("viewBox=\"0 0 512 512\""));
}

#[test]
fn test_svg_xmlns() {
    let opts = RenderOptions::default();
    let svg = LogoGenerator::generate_svg("Test", Preset::MonogramBadge, &opts).unwrap();
    assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
}

#[test]
fn test_png_different_sizes_different_output() {
    let opts_small = RenderOptions {
        size_px: 256,
        ..Default::default()
    };
    let opts_large = RenderOptions {
        size_px: 512,
        ..Default::default()
    };
    let png_small = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts_small).unwrap();
    let png_large = LogoGenerator::generate_png("Test", Preset::MonogramBadge, &opts_large).unwrap();
    assert_ne!(png_small, png_large);
}
