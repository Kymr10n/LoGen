/// Typography is intentionally minimal in this stub.
///
/// For production:
/// - Curate a small font set, embed them, and select deterministically
/// - Add shaping with rustybuzz, glyph coverage checks, kerning, etc.
#[derive(Debug, Clone)]
pub struct Typography {
    pub family: &'static str,
    pub weight: u16,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            family: "system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif",
            weight: 700,
        }
    }
}
