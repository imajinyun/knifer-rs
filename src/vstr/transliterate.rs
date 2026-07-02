//! Optional full transliteration helpers behind the `transliterate` feature.
//!
//! The default [`deburr`](super::deburr) helper folds only Latin-1 and common
//! diacritics, so scripts such as CJK, Cyrillic, and Greek pass through
//! unchanged. When the `transliterate` feature is enabled, these helpers use the
//! `deunicode` crate's transliteration tables to render a best-effort ASCII
//! approximation of arbitrary Unicode, which makes ASCII slugging truly global.
//!
//! Transliteration is inherently lossy and opinionated (for example `北` becomes
//! `Bei` and `€` becomes `EUR`); use [`deburr`](super::deburr) instead when you
//! only want to strip Latin diacritics while preserving other scripts.

/// Transliterates arbitrary Unicode text to a best-effort ASCII approximation.
///
/// Unlike [`deburr`](super::deburr), which only folds Latin diacritics, this
/// renders non-Latin scripts phonetically or by name (for example `你好` becomes
/// `Ni Hao` and `€` becomes `EUR`). The result is always ASCII. This helper is
/// available only with the `transliterate` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "transliterate")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::transliterate("Crème Brûlée"), "Creme Brulee");
/// assert_eq!(vstr::transliterate("你好"), "Ni Hao");
/// assert_eq!(vstr::transliterate("Москва"), "Moskva");
/// # }
/// ```
#[must_use]
pub fn transliterate(input: &str) -> String {
    deunicode::deunicode(input)
}

/// Converts text into an ASCII slug, transliterating non-Latin scripts first.
///
/// This is the transliterating counterpart to [`slugify`](super::slugify): the
/// input is first rendered to ASCII with [`transliterate`], then slugged with
/// `-` separators. Where [`slugify`](super::slugify) preserves scripts such as
/// CJK, this maps them to ASCII so the slug is always URL-safe ASCII. This
/// helper is available only with the `transliterate` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "transliterate")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::slugify_ascii("Crème Brûlée"), "creme-brulee");
/// assert_eq!(vstr::slugify_ascii("你好 Rust"), "ni-hao-rust");
/// # }
/// ```
#[must_use]
pub fn slugify_ascii(input: &str) -> String {
    slugify_ascii_with_separator(input, '-')
}

/// Converts text into an ASCII slug with a custom separator, transliterating
/// non-Latin scripts first.
///
/// If `separator` is alphanumeric or whitespace, `-` is used instead, matching
/// [`slugify_with_separator`](super::slugify_with_separator). This helper is
/// available only with the `transliterate` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "transliterate")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::slugify_ascii_with_separator("你好 Rust", '_'), "ni_hao_rust");
/// # }
/// ```
#[must_use]
pub fn slugify_ascii_with_separator(input: &str, separator: char) -> String {
    let ascii = transliterate(input);
    super::slugify_with_separator(&ascii, separator)
}
