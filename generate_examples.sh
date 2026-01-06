#!/bin/bash
# Demo script to generate sample logos

echo "🎨 Generating sample logos..."

# Create examples directory
mkdir -p examples

# Generate various logos
./target/release/logo-gen --input "Acme Corp" --preset monogram-badge --format png --out examples/acme.png --size 512
./target/release/logo-gen --input "Tech Startup" --preset monogram-badge --format png --out examples/tech_startup.png --size 512  
./target/release/logo-gen --input "Design Studio" --preset monogram-badge --format svg --out examples/design_studio.svg
./target/release/logo-gen --input "Coffee Shop" --preset monogram-badge --format png --out examples/coffee_shop.png --size 512 --transparent
./target/release/logo-gen --input "Open Source" --preset monogram-badge --format png --out examples/open_source.png --size 1024

# Same input, different variants
./target/release/logo-gen --input "Brand X" --preset monogram-badge --format png --out examples/brand_x_v1.png --size 512 --variant 1
./target/release/logo-gen --input "Brand X" --preset monogram-badge --format png --out examples/brand_x_v2.png --size 512 --variant 2
./target/release/logo-gen --input "Brand X" --preset monogram-badge --format png --out examples/brand_x_v3.png --size 512 --variant 3

echo "✅ Generated $(ls examples/ | wc -l) logos in examples/"
echo ""
echo "Examples:"
ls -lh examples/
