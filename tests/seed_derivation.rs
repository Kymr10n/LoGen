use logen::core::seed::{derive_seed_32, derive_seed_u64};

#[test]
fn test_seed_deterministic() {
    let seed1 = derive_seed_32("test", None);
    let seed2 = derive_seed_32("test", None);
    assert_eq!(seed1, seed2);
}

#[test]
fn test_seed_different_inputs() {
    let seed1 = derive_seed_32("test1", None);
    let seed2 = derive_seed_32("test2", None);
    assert_ne!(seed1, seed2);
}

#[test]
fn test_seed_variant_changes_output() {
    let seed1 = derive_seed_32("test", None);
    let seed2 = derive_seed_32("test", Some(1));
    assert_ne!(seed1, seed2);
}

#[test]
fn test_seed_different_variants() {
    let seed1 = derive_seed_32("test", Some(1));
    let seed2 = derive_seed_32("test", Some(2));
    assert_ne!(seed1, seed2);
}

#[test]
fn test_seed_u64_deterministic() {
    let seed1 = derive_seed_u64("test", None);
    let seed2 = derive_seed_u64("test", None);
    assert_eq!(seed1, seed2);
}

#[test]
fn test_seed_32_length() {
    let seed = derive_seed_32("test", None);
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_seed_u64_different_inputs() {
    let seed1 = derive_seed_u64("input1", None);
    let seed2 = derive_seed_u64("input2", None);
    assert_ne!(seed1, seed2);
}

#[test]
fn test_seed_u64_variant_changes() {
    let seed1 = derive_seed_u64("test", Some(1));
    let seed2 = derive_seed_u64("test", Some(2));
    assert_ne!(seed1, seed2);
}

#[test]
fn test_seed_empty_string() {
    // Empty strings should still produce valid seeds
    let seed = derive_seed_32("", None);
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_seed_unicode_input() {
    let seed1 = derive_seed_32("Hello 世界", None);
    let seed2 = derive_seed_32("Hello 世界", None);
    assert_eq!(seed1, seed2);
}

#[test]
fn test_seed_long_input() {
    let long_input = "a".repeat(1000);
    let seed1 = derive_seed_32(&long_input, None);
    let seed2 = derive_seed_32(&long_input, None);
    assert_eq!(seed1, seed2);
}

#[test]
fn test_seed_variant_zero_same_as_none() {
    let seed_none = derive_seed_32("test", None);
    let seed_zero = derive_seed_32("test", Some(0));
    // Variant 0 XORs with 0 which leaves the seed unchanged, so it's effectively the same as None
    assert_eq!(seed_none, seed_zero);
}
