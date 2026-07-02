use crate::vstr::*;

#[test]
fn transliterate_renders_non_latin_scripts_to_ascii() {
    // Latin diacritics are folded like `deburr`.
    assert_eq!(transliterate("Crème Brûlée"), "Creme Brulee");
    assert_eq!(transliterate("Déjà Vu"), "Deja Vu");
    // Non-Latin scripts are approximated to ASCII (where `deburr` preserves them).
    assert_eq!(transliterate("你好"), "Ni Hao");
    assert_eq!(transliterate("Москва"), "Moskva");
    // The result is always ASCII.
    assert!(transliterate("日本語 €").is_ascii());
    assert_eq!(transliterate(""), "");
}

#[test]
fn deburr_preserves_scripts_that_transliterate_maps() {
    // Contrast: the default `deburr` keeps non-Latin scripts untouched.
    assert_eq!(deburr("你好"), "你好");
    assert_eq!(transliterate("你好"), "Ni Hao");
}

#[test]
fn slugify_ascii_transliterates_before_slugging() {
    assert_eq!(slugify_ascii("Crème Brûlée"), "creme-brulee");
    assert_eq!(slugify_ascii("你好 Rust"), "ni-hao-rust");
    assert_eq!(slugify_ascii("Москва 2026"), "moskva-2026");
    assert_eq!(slugify_ascii(""), "");
}

#[test]
fn slugify_ascii_with_separator_uses_custom_separator() {
    assert_eq!(
        slugify_ascii_with_separator("你好 Rust", '_'),
        "ni_hao_rust"
    );
    assert_eq!(slugify_ascii_with_separator("Déjà Vu", '.'), "deja.vu");
    // Alphanumeric/whitespace separators fall back to '-'.
    assert_eq!(
        slugify_ascii_with_separator("你好 Rust", 'x'),
        "ni-hao-rust"
    );
}

#[test]
fn slugify_ascii_differs_from_scalar_slugify_for_non_latin() {
    // Scalar slugify preserves CJK; the ASCII variant transliterates it.
    assert_eq!(slugify("你好 Rust"), "你好-rust");
    assert_eq!(slugify_ascii("你好 Rust"), "ni-hao-rust");
}
