use logo_gen::core::seed::normalize_input;

#[test]
fn test_normalize_trims_whitespace() {
    assert_eq!(normalize_input("  hello  "), "hello");
}

#[test]
fn test_normalize_collapses_whitespace() {
    assert_eq!(normalize_input("hello    world"), "hello world");
}

#[test]
fn test_normalize_multiple_types_whitespace() {
    assert_eq!(normalize_input("hello\t\n  \r\nworld"), "hello world");
}

#[test]
fn test_normalize_preserves_special_chars() {
    assert_eq!(normalize_input("Acme™ Corp®"), "Acme™ Corp®");
}

#[test]
fn test_normalize_preserves_unicode() {
    assert_eq!(normalize_input("Café ☕ Société"), "Café ☕ Société");
}

#[test]
fn test_normalize_empty() {
    assert_eq!(normalize_input(""), "");
}

#[test]
fn test_normalize_whitespace_only() {
    assert_eq!(normalize_input("   \t\n  "), "");
}
