use unicode_normalization::UnicodeNormalization;
use unicode_normalization::{
    is_nfc as uni_is_nfc, is_nfd as uni_is_nfd, is_nfkc as uni_is_nfkc, is_nfkd as uni_is_nfkd,
};

/// Returns `input` in Unicode Normalization Form C (NFC).
///
/// NFC applies canonical decomposition followed by canonical composition, which
/// is the form most systems store and compare text in. Use it to make visually
/// identical strings compare equal, for example combining an `e` with a
/// combining acute accent into a single precomposed `é`.
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::nfc("e\u{301}"), "\u{e9}");
/// assert_eq!(vstr::nfc("\u{e9}"), "\u{e9}");
/// # }
/// ```
#[must_use]
pub fn nfc(input: &str) -> String {
    input.nfc().collect()
}

/// Returns `input` in Unicode Normalization Form D (NFD).
///
/// NFD applies canonical decomposition, splitting precomposed characters into a
/// base character followed by combining marks. Use it when you need consistent
/// decomposed sequences, for example before stripping combining marks.
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::nfd("\u{e9}"), "e\u{301}");
/// assert_eq!(vstr::nfd("e\u{301}"), "e\u{301}");
/// # }
/// ```
#[must_use]
pub fn nfd(input: &str) -> String {
    input.nfd().collect()
}

/// Returns `input` in Unicode Normalization Form KC (NFKC).
///
/// NFKC applies compatibility decomposition followed by canonical composition.
/// It folds compatibility variants such as full-width Latin letters and ligature
/// characters into their canonical equivalents, which is useful for lenient
/// search and matching.
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::nfkc("\u{ff21}\u{ff22}\u{ff23}"), "ABC");
/// assert_eq!(vstr::nfkc("\u{fb01}"), "fi");
/// # }
/// ```
#[must_use]
pub fn nfkc(input: &str) -> String {
    input.nfkc().collect()
}

/// Returns `input` in Unicode Normalization Form KD (NFKD).
///
/// NFKD applies compatibility decomposition without recomposition, producing the
/// most decomposed form. Use it when downstream processing wants base characters
/// and combining marks separated and compatibility variants expanded.
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::nfkd("\u{ff21}"), "A");
/// assert_eq!(vstr::nfkd("\u{e9}"), "e\u{301}");
/// # }
/// ```
#[must_use]
pub fn nfkd(input: &str) -> String {
    input.nfkd().collect()
}

/// Reports whether `input` is already in Unicode Normalization Form C (NFC).
///
/// This is a quick check that avoids allocating a normalized copy. It returns
/// `true` when [`nfc`] would return an identical string, which lets callers skip
/// normalization work for already-normalized input.
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_nfc("\u{e9}"));
/// assert!(!vstr::is_nfc("e\u{301}"));
/// # }
/// ```
#[must_use]
pub fn is_nfc(input: &str) -> bool {
    uni_is_nfc(input)
}

/// Reports whether `input` is already in Unicode Normalization Form D (NFD).
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_nfd("e\u{301}"));
/// assert!(!vstr::is_nfd("\u{e9}"));
/// # }
/// ```
#[must_use]
pub fn is_nfd(input: &str) -> bool {
    uni_is_nfd(input)
}

/// Reports whether `input` is already in Unicode Normalization Form KC (NFKC).
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_nfkc("ABC"));
/// assert!(!vstr::is_nfkc("\u{ff21}\u{ff22}\u{ff23}"));
/// # }
/// ```
#[must_use]
pub fn is_nfkc(input: &str) -> bool {
    uni_is_nfkc(input)
}

/// Reports whether `input` is already in Unicode Normalization Form KD (NFKD).
///
/// This helper is available only with the `unicode-normalization` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-normalization")]
/// # {
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_nfkd("e\u{301}"));
/// assert!(!vstr::is_nfkd("\u{ff21}"));
/// # }
/// ```
#[must_use]
pub fn is_nfkd(input: &str) -> bool {
    uni_is_nfkd(input)
}
