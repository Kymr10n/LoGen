use logen::core::seed::normalize_input;

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

#[test]
fn test_normalize_mixed_punctuation() {
    assert_eq!(normalize_input("Hello, World!"), "Hello, World!");
}

#[test]
fn test_normalize_leading_trailing_mixed() {
    assert_eq!(normalize_input("\n\t  Test  \r\n"), "Test");
}

#[test]
fn test_normalize_numbers_preserved() {
    assert_eq!(normalize_input("  Test 123  "), "Test 123");
}

#[test]
fn test_normalize_emoji() {
    assert_eq!(normalize_input("Hello 👋 World 🌍"), "Hello 👋 World 🌍");
}

#[test]
fn test_normalize_consecutive_spaces() {
    assert_eq!(normalize_input("a     b     c"), "a b c");
}
