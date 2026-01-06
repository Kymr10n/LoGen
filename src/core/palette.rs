use rand::Rng;

// Palette generation constraints for logo-appropriate colors.
const MIN_SATURATION: f32 = 0.55;
const MAX_SATURATION: f32 = 0.80;
const MIN_LIGHTNESS: f32 = 0.38;
const MAX_LIGHTNESS: f32 = 0.58;

const MIN_HUE_OFFSET: f32 = 20.0;
const MAX_HUE_OFFSET: f32 = 90.0;
const SECONDARY_SATURATION_FACTOR: f32 = 0.9;
const SECONDARY_LIGHTNESS_FACTOR: f32 = 1.1;

const BACKGROUND_HUE_OFFSET: f32 = 180.0;
const MIN_BACKGROUND_SATURATION: f32 = 0.12;
const MAX_BACKGROUND_SATURATION: f32 = 0.25;
const MIN_BACKGROUND_LIGHTNESS: f32 = 0.92;
const MAX_BACKGROUND_LIGHTNESS: f32 = 0.98;

/// Simple RGB color.
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Minimal HSL->RGB conversion. Good enough for stubs.
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Rgb {
    fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0/2.0 { return q; }
        if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
        p
    }

    let h = (h % 360.0) / 360.0;
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);

    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        return Rgb { r: v, g: v, b: v };
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0/3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0/3.0);

    Rgb {
        r: (r * 255.0).round() as u8,
        g: (g * 255.0).round() as u8,
        b: (b * 255.0).round() as u8,
    }
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub background: Option<Rgb>,
    pub primary: Rgb,
    pub secondary: Rgb,
}

pub fn derive_palette<R: Rng>(rng: &mut R, transparent_background: bool) -> Palette {
    // Curated ranges; keep it "logo-like".
    let hue = rng.gen_range(0.0..360.0);
    let s = rng.gen_range(MIN_SATURATION..MAX_SATURATION);
    let l = rng.gen_range(MIN_LIGHTNESS..MAX_LIGHTNESS);

    let primary = hsl_to_rgb(hue, s, l);
    let secondary = hsl_to_rgb(
        hue + rng.gen_range(MIN_HUE_OFFSET..MAX_HUE_OFFSET),
        (s * SECONDARY_SATURATION_FACTOR).clamp(0.0, 1.0),
        (l * SECONDARY_LIGHTNESS_FACTOR).clamp(0.0, 1.0)
    );

    let background = if transparent_background {
        None
    } else {
        Some(hsl_to_rgb(
            hue + BACKGROUND_HUE_OFFSET,
            rng.gen_range(MIN_BACKGROUND_SATURATION..MAX_BACKGROUND_SATURATION),
            rng.gen_range(MIN_BACKGROUND_LIGHTNESS..MAX_BACKGROUND_LIGHTNESS)
        ))
    };

    Palette { background, primary, secondary }
}
