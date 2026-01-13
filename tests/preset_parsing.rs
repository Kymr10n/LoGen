use logen::Preset;
use std::str::FromStr;

#[test]
fn test_parse_monogram_badge() {
    assert!(matches!(
        Preset::from_str("monogram-badge"),
        Ok(Preset::MonogramBadge)
    ));
}

#[test]
fn test_parse_monogram_alias() {
    assert!(matches!(
        Preset::from_str("monogram"),
        Ok(Preset::MonogramBadge)
    ));
}

#[test]
fn test_parse_badge_alias() {
    assert!(matches!(
        Preset::from_str("badge"),
        Ok(Preset::MonogramBadge)
    ));
}

#[test]
fn test_parse_case_insensitive() {
    assert!(matches!(
        Preset::from_str("MONOGRAM-BADGE"),
        Ok(Preset::MonogramBadge)
    ));
}

#[test]
fn test_parse_with_whitespace() {
    assert!(matches!(
        Preset::from_str("  monogram-badge  "),
        Ok(Preset::MonogramBadge)
    ));
}

#[test]
fn test_parse_unknown() {
    assert!(Preset::from_str("unknown").is_err());
}

#[test]
fn test_preset_id() {
    assert_eq!(Preset::MonogramBadge.id(), "monogram-badge");
}
