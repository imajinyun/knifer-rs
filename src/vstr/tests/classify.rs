use crate::vstr::*;

#[test]
fn character_classification_helpers_follow_unicode_semantics() {
    assert!(is_blank_char(' '));
    assert!(is_blank_char('\u{3000}'));
    assert!(is_letter('你'));
    assert!(is_letter('A'));
    assert!(!is_letter('Ⅷ'));
    assert!(is_digit('9'));
    assert!(is_digit('٣'));
    assert!(is_digit('９'));
    assert!(!is_digit('Ⅷ'));
    assert!(!is_digit('七'));
    assert!(is_ascii('A'));
    assert!(!is_ascii('你'));
    assert!(is_letter_or_digit('A'));
    assert!(is_letter_or_digit('9'));
    assert!(is_letter_or_digit('٣'));
    assert!(is_letter_or_digit('七'));
    assert!(!is_letter_or_digit('Ⅷ'));
    assert!(!is_letter_or_digit('-'));
}
