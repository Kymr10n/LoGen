use logo_gen::core::seed::{derive_seed_32, derive_seed_u64};

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
