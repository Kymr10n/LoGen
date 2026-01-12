#!/bin/bash
# Demo script to generate sample logos

echo "🎨 Generating sample logos..."

# Create examples directory
mkdir -p examples

# Build first
cargo build --release

# Monogram Badge examples
./target/release/logo-gen --input "Acme Corp" --preset monogram-badge --format png --out examples/monogram_acme.png --size 512
./target/release/logo-gen --input "Tech Startup" --preset monogram-badge --format svg --out examples/monogram_tech.svg
./target/release/logo-gen --input "Design Studio" --preset badge --format png --out examples/monogram_design.png --size 512 --transparent

# Geometric Pattern examples
./target/release/logo-gen --input "Creative Co" --preset geometric-pattern --format png --out examples/geometric_creative.png --size 512
./target/release/logo-gen --input "ArtLab" --preset geometric --format svg --out examples/geometric_art.svg
./target/release/logo-gen --input "ModernBrand" --preset pattern --format png --out examples/geometric_modern.png --size 512 --transparent

# Variant examples (same input, different outputs)
./target/release/logo-gen --input "Brand X" --preset monogram-badge --format png --out examples/brand_x_v1.png --size 512 --variant 1
./target/release/logo-gen --input "Brand X" --preset monogram-badge --format png --out examples/brand_x_v2.png --size 512 --variant 2
./target/release/logo-gen --input "Brand X" --preset geometric --format png --out examples/brand_x_geo_v1.png --size 512 --variant 3

echo "✅ Generated $(ls examples/ | wc -l) logos in examples/"
echo ""
echo "Examples:"
ls -lh examples/
