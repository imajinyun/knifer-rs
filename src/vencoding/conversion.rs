//! Optional legacy encoding conversion helpers behind the `encoding` feature.
//!
//! The default [`vencoding`](super) facade only sniffs BOMs and validates UTF-8.
//! When the `encoding` feature is enabled, these helpers decode and encode
//! WHATWG-labeled legacy encodings (for example GBK, `Shift_JIS`, windows-1252,
//! and ISO-8859-1) over the `encoding_rs` crate, without exposing its types on
//! the public API.

use std::borrow::Cow;

/// Error returned by the optional legacy-encoding conversion helpers.
///
/// This type keeps the public error boundary small instead of exposing the
/// underlying `encoding_rs` types. It is available only with the `encoding`
/// feature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncodingError {
    label: String,
    kind: EncodingErrorKind,
}

/// Reason an [`EncodingError`] was produced.
///
/// This is available only with the `encoding` feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodingErrorKind {
    /// The requested encoding label is not a WHATWG Encoding Standard label.
    UnknownLabel,
    /// The input bytes were malformed for the requested encoding.
    ///
    /// Only strict decoding reports this; lossy decoding uses replacement
    /// characters instead.
    MalformedInput,
}

impl EncodingError {
    fn unknown_label(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            kind: EncodingErrorKind::UnknownLabel,
        }
    }

    fn malformed_input(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            kind: EncodingErrorKind::MalformedInput,
        }
    }

    /// Returns the encoding label that triggered this error.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "encoding")]
    /// # {
    /// use kniferrs::vencoding;
    ///
    /// let err = vencoding::decode(b"x", "not-an-encoding").unwrap_err();
    /// assert_eq!(err.label(), "not-an-encoding");
    /// # }
    /// ```
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the reason this error was produced.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "encoding")]
    /// # {
    /// use kniferrs::vencoding::{self, EncodingErrorKind};
    ///
    /// let err = vencoding::decode(b"x", "not-an-encoding").unwrap_err();
    /// assert_eq!(err.kind(), EncodingErrorKind::UnknownLabel);
    /// # }
    /// ```
    #[must_use]
    pub const fn kind(&self) -> EncodingErrorKind {
        self.kind
    }
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            EncodingErrorKind::UnknownLabel => {
                write!(formatter, "unknown encoding label {:?}", self.label)
            }
            EncodingErrorKind::MalformedInput => {
                write!(formatter, "malformed input for encoding {:?}", self.label)
            }
        }
    }
}

impl std::error::Error for EncodingError {}

/// Resolves a WHATWG encoding label to its canonical name.
///
/// Labels are matched case-insensitively with surrounding whitespace ignored,
/// following the WHATWG Encoding Standard (so `latin1`, `ISO-8859-1`, and
/// `csisolatin1` all resolve to `windows-1252`). Returns `None` for unknown
/// labels. This helper is available only with the `encoding` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "encoding")]
/// # {
/// use kniferrs::vencoding;
///
/// assert_eq!(vencoding::encoding_name("gbk"), Some("GBK"));
/// assert_eq!(vencoding::encoding_name("latin1"), Some("windows-1252"));
/// assert_eq!(vencoding::encoding_name("shift-jis"), Some("Shift_JIS"));
/// assert_eq!(vencoding::encoding_name("not-an-encoding"), None);
/// # }
/// ```
#[must_use]
pub fn encoding_name(label: &str) -> Option<&'static str> {
    encoding_rs::Encoding::for_label(label.as_bytes()).map(encoding_rs::Encoding::name)
}

fn lookup(label: &str) -> Result<&'static encoding_rs::Encoding, EncodingError> {
    encoding_rs::Encoding::for_label(label.as_bytes())
        .ok_or_else(|| EncodingError::unknown_label(label))
}

/// Decodes legacy `input` bytes to UTF-8 using the named encoding.
///
/// The `label` is any WHATWG Encoding Standard label (for example `gbk`,
/// `shift_jis`, `windows-1252`, or `iso-8859-1`). Malformed sequences are
/// replaced with `U+FFFD` rather than reported as errors, matching how web
/// browsers decode legacy content. A leading UTF-8, UTF-16LE, or UTF-16BE byte
/// order mark overrides the requested label, so BOM-prefixed content decodes
/// correctly. Use [`decode_strict`] when malformed bytes must be rejected.
///
/// This helper is available only with the `encoding` feature.
///
/// # Errors
///
/// Returns [`EncodingError`] with [`EncodingErrorKind::UnknownLabel`] when
/// `label` is not a recognized encoding label.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "encoding")]
/// # {
/// use kniferrs::vencoding;
///
/// assert_eq!(vencoding::decode(&[0x63, 0x61, 0x66, 0xE9], "windows-1252").unwrap(), "café");
/// assert_eq!(vencoding::decode(&[0xD6, 0xD0, 0xCE, 0xC4], "gbk").unwrap(), "中文");
/// assert!(vencoding::decode(b"x", "not-an-encoding").is_err());
/// # }
/// ```
pub fn decode<'src>(input: &'src [u8], label: &str) -> Result<Cow<'src, str>, EncodingError> {
    let encoding = lookup(label)?;
    let (text, _actual, _had_errors) = encoding.decode(input);
    Ok(text)
}

/// Decodes legacy `input` bytes to UTF-8, rejecting malformed sequences.
///
/// Unlike [`decode`], no replacement characters are inserted: any byte sequence
/// that is not valid for the requested encoding produces an error. This variant
/// does not perform BOM sniffing; strip a byte order mark with
/// [`strip_bom`](super::strip_bom) first if the payload may carry one.
///
/// This helper is available only with the `encoding` feature.
///
/// # Errors
///
/// Returns [`EncodingError`] with [`EncodingErrorKind::UnknownLabel`] when
/// `label` is unknown, or [`EncodingErrorKind::MalformedInput`] when `input`
/// contains a sequence that is invalid for the encoding.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "encoding")]
/// # {
/// use kniferrs::vencoding::{self, EncodingErrorKind};
///
/// assert_eq!(vencoding::decode_strict(&[0x93, 0xFA, 0x96, 0x7B], "shift_jis").unwrap(), "日本");
/// let err = vencoding::decode_strict(&[0x81], "shift_jis").unwrap_err();
/// assert_eq!(err.kind(), EncodingErrorKind::MalformedInput);
/// # }
/// ```
pub fn decode_strict<'src>(
    input: &'src [u8],
    label: &str,
) -> Result<Cow<'src, str>, EncodingError> {
    let encoding = lookup(label)?;
    encoding
        .decode_without_bom_handling_and_without_replacement(input)
        .ok_or_else(|| EncodingError::malformed_input(label))
}

/// Encodes UTF-8 `input` into the named legacy encoding.
///
/// The `label` is any WHATWG Encoding Standard label. Characters that cannot be
/// represented in the target encoding are emitted as HTML numeric character
/// references (for example `&#128512;`), matching the WHATWG encode behavior, so
/// encoding is lossy for unmappable characters. Round-tripping is exact for
/// characters the target encoding can represent.
///
/// This helper is available only with the `encoding` feature.
///
/// # Errors
///
/// Returns [`EncodingError`] with [`EncodingErrorKind::UnknownLabel`] when
/// `label` is not a recognized encoding label.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "encoding")]
/// # {
/// use kniferrs::vencoding;
///
/// assert_eq!(vencoding::encode("café", "windows-1252").unwrap(), vec![0x63, 0x61, 0x66, 0xE9]);
/// assert_eq!(vencoding::encode("中文", "gbk").unwrap(), vec![0xD6, 0xD0, 0xCE, 0xC4]);
/// assert!(vencoding::encode("x", "not-an-encoding").is_err());
/// # }
/// ```
pub fn encode<'src>(input: &'src str, label: &str) -> Result<Cow<'src, [u8]>, EncodingError> {
    let encoding = lookup(label)?;
    let (bytes, _actual, _had_unmappable) = encoding.encode(input);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vencoding_resolves_whatwg_labels_case_insensitively() {
        assert_eq!(encoding_name("gbk"), Some("GBK"));
        assert_eq!(encoding_name("  GBK  "), Some("GBK"));
        assert_eq!(encoding_name("latin1"), Some("windows-1252"));
        assert_eq!(encoding_name("ISO-8859-1"), Some("windows-1252"));
        assert_eq!(encoding_name("shift-jis"), Some("Shift_JIS"));
        assert_eq!(encoding_name("utf-8"), Some("UTF-8"));
        assert_eq!(encoding_name("not-an-encoding"), None);
    }

    #[test]
    fn vencoding_decodes_legacy_bytes_to_utf8() {
        assert_eq!(
            decode(&[0x63, 0x61, 0x66, 0xE9], "windows-1252").unwrap(),
            "café"
        );
        assert_eq!(decode(&[0xD6, 0xD0, 0xCE, 0xC4], "gbk").unwrap(), "中文");
        assert_eq!(
            decode(&[0x93, 0xFA, 0x96, 0x7B], "shift_jis").unwrap(),
            "日本"
        );
        assert_eq!(decode(&[0xC0, 0xC0], "iso-8859-1").unwrap(), "ÀÀ");
    }

    #[test]
    fn vencoding_decode_replaces_malformed_but_strict_rejects() {
        // Lossy decode substitutes U+FFFD for an invalid Shift_JIS lead byte.
        let lossy = decode(&[0x81], "shift_jis").unwrap();
        assert!(lossy.contains('\u{FFFD}'));

        let err = decode_strict(&[0x81], "shift_jis").unwrap_err();
        assert_eq!(err.kind(), EncodingErrorKind::MalformedInput);
        assert_eq!(err.label(), "shift_jis");

        assert_eq!(
            decode_strict(&[0x93, 0xFA, 0x96, 0x7B], "shift_jis").unwrap(),
            "日本"
        );
    }

    #[test]
    fn vencoding_encodes_utf8_into_legacy_bytes() {
        assert_eq!(
            encode("café", "windows-1252").unwrap().into_owned(),
            vec![0x63, 0x61, 0x66, 0xE9]
        );
        assert_eq!(
            encode("中文", "gbk").unwrap().into_owned(),
            vec![0xD6, 0xD0, 0xCE, 0xC4]
        );
        // Unmappable characters become HTML numeric references, matching WHATWG.
        assert_eq!(
            encode("😀", "windows-1252").unwrap().into_owned(),
            b"&#128512;"
        );
    }

    #[test]
    fn vencoding_round_trips_representable_text() {
        for (text, label) in [
            ("café résumé", "windows-1252"),
            ("Grüße açaí", "iso-8859-1"),
            ("简体中文与标点，。", "gbk"),
            ("日本語のテスト", "shift_jis"),
        ] {
            let bytes = encode(text, label).unwrap();
            let back = decode_strict(&bytes, label).unwrap();
            assert_eq!(back, text, "round-trip mismatch for {label}");
        }
    }

    #[test]
    fn vencoding_reports_unknown_labels() {
        let err = decode(b"x", "not-an-encoding").unwrap_err();
        assert_eq!(err.kind(), EncodingErrorKind::UnknownLabel);
        assert!(decode_strict(b"x", "not-an-encoding").is_err());
        assert!(encode("x", "not-an-encoding").is_err());
    }
}
