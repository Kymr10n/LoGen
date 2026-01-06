use blake3::Hasher;

/// Normalize input so logos remain stable for semantically identical strings.
///
/// This is intentionally conservative; evolve as needed.
pub fn normalize_input(input: &str) -> String {
    // 1) Trim
    let trimmed = input.trim();

    // 2) Collapse whitespace to single spaces
    let mut out = String::with_capacity(trimmed.len());
    let mut last_was_ws = false;

    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !last_was_ws {
                out.push(' ');
                last_was_ws = true;
            }
        } else {
            last_was_ws = false;
            out.push(ch);
        }
    }

    out
}

/// Derive a 32-byte seed from input (and optional variant), stable across runs.
pub fn derive_seed_32(normalized: &str, variant: Option<u64>) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(normalized.as_bytes());

    let mut seed = *hasher.finalize().as_bytes();

    if let Some(v) = variant {
        // XOR in the variant into the first 8 bytes (little endian)
        let vbytes = v.to_le_bytes();
        for i in 0..8 {
            seed[i] ^= vbytes[i];
        }
    }

    seed
}

/// Convenience: derive a u64 from the seed.
pub fn derive_seed_u64(normalized: &str, variant: Option<u64>) -> u64 {
    let seed32 = derive_seed_32(normalized, variant);
    let mut b = [0u8; 8];
    b.copy_from_slice(&seed32[..8]);
    u64::from_le_bytes(b)
}
