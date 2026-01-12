//! Benchmarks for logo generation performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use logo_gen::{LogoGenerator, Preset, RenderOptions};

fn bench_svg_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("svg_generation");

    for size in [256, 512, 1024] {
        group.bench_function(format!("monogram_{}px", size), |b| {
            let opts = RenderOptions {
                size_px: size,
                ..Default::default()
            };
            b.iter(|| {
                LogoGenerator::generate_svg(
                    black_box("Test Company"),
                    black_box(Preset::MonogramBadge),
                    black_box(&opts),
                )
            });
        });

        group.bench_function(format!("geometric_{}px", size), |b| {
            let opts = RenderOptions {
                size_px: size,
                ..Default::default()
            };
            b.iter(|| {
                LogoGenerator::generate_svg(
                    black_box("Test Company"),
                    black_box(Preset::GeometricPattern),
                    black_box(&opts),
                )
            });
        });
    }

    group.finish();
}

fn bench_png_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("png_generation");

    for size in [256, 512, 1024] {
        group.bench_function(format!("monogram_{}px", size), |b| {
            let opts = RenderOptions {
                size_px: size,
                ..Default::default()
            };
            b.iter(|| {
                LogoGenerator::generate_png(
                    black_box("Test Company"),
                    black_box(Preset::MonogramBadge),
                    black_box(&opts),
                )
            });
        });

        group.bench_function(format!("geometric_{}px", size), |b| {
            let opts = RenderOptions {
                size_px: size,
                ..Default::default()
            };
            b.iter(|| {
                LogoGenerator::generate_png(
                    black_box("Test Company"),
                    black_box(Preset::GeometricPattern),
                    black_box(&opts),
                )
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_svg_generation, bench_png_generation);
criterion_main!(benches);
