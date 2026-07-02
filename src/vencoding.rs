//! Encoding helpers for byte-oriented text boundaries.
//!
//! This facade handles BOM sniffing and UTF-8 validation without turning
//! [`crate::vstr`] into an encoding-conversion module.

use std::borrow::Cow;

/// A byte order mark recognized at the start of a byte slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Bom {
    /// UTF-8 byte order mark: `EF BB BF`.
    Utf8,
    /// UTF-16 little-endian byte order mark: `FF FE`.
    Utf16Le,
    /// UTF-16 big-endian byte order mark: `FE FF`.
    Utf16Be,
    /// UTF-32 little-endian byte order mark: `FF FE 00 00`.
    Utf32Le,
    /// UTF-32 big-endian byte order mark: `00 00 FE FF`.
    Utf32Be,
}

impl Bom {
    /// Returns the length of this BOM in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vencoding::Bom;
    ///
    /// assert_eq!(Bom::Utf8.byte_len(), 3);
    /// assert_eq!(Bom::Utf16Le.byte_len(), 2);
    /// ```
    #[must_use]
    pub const fn byte_len(self) -> usize {
        match self {
            Self::Utf8 => 3,
            Self::Utf16Le | Self::Utf16Be => 2,
            Self::Utf32Le | Self::Utf32Be => 4,
        }
    }

    /// Returns the canonical encoding label for this BOM.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vencoding::Bom;
    ///
    /// assert_eq!(Bom::Utf16Be.encoding_name(), "UTF-16BE");
    /// ```
    #[must_use]
    pub const fn encoding_name(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf16Le => "UTF-16LE",
            Self::Utf16Be => "UTF-16BE",
            Self::Utf32Le => "UTF-32LE",
            Self::Utf32Be => "UTF-32BE",
        }
    }
}

/// Result of scanning a byte slice for an initial BOM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BomScan<'src> {
    /// Detected initial BOM, if one is present.
    pub bom: Option<Bom>,
    /// Input bytes after removing the initial BOM, or the original input.
    pub content: &'src [u8],
}

/// Returns the initial byte order mark when `input` starts with one.
///
/// UTF-32 BOMs are checked before UTF-16 BOMs because their prefixes overlap.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding::{self, Bom};
///
/// assert_eq!(vencoding::detect_bom(&[0xEF, 0xBB, 0xBF, b'a']), Some(Bom::Utf8));
/// assert_eq!(vencoding::detect_bom(b"plain"), None);
/// ```
#[must_use]
pub const fn detect_bom(input: &[u8]) -> Option<Bom> {
    if input.len() >= 4 {
        if input[0] == 0xFF && input[1] == 0xFE && input[2] == 0x00 && input[3] == 0x00 {
            return Some(Bom::Utf32Le);
        }
        if input[0] == 0x00 && input[1] == 0x00 && input[2] == 0xFE && input[3] == 0xFF {
            return Some(Bom::Utf32Be);
        }
    }

    if input.len() >= 3 && input[0] == 0xEF && input[1] == 0xBB && input[2] == 0xBF {
        return Some(Bom::Utf8);
    }

    if input.len() >= 2 {
        if input[0] == 0xFF && input[1] == 0xFE {
            return Some(Bom::Utf16Le);
        }
        if input[0] == 0xFE && input[1] == 0xFF {
            return Some(Bom::Utf16Be);
        }
    }

    None
}

/// Scans `input` and returns both the detected BOM and content bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding::{self, Bom};
///
/// let scan = vencoding::scan_bom(&[0xEF, 0xBB, 0xBF, b'a']);
/// assert_eq!(scan.bom, Some(Bom::Utf8));
/// assert_eq!(scan.content, b"a");
/// ```
#[must_use]
pub fn scan_bom(input: &[u8]) -> BomScan<'_> {
    let bom = detect_bom(input);
    let content = bom.map_or(input, |bom| &input[bom.byte_len()..]);
    BomScan { bom, content }
}

/// Returns `input` without an initial BOM when one is present.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding;
///
/// assert_eq!(vencoding::strip_bom(&[0xEF, 0xBB, 0xBF, b'a']), b"a");
/// assert_eq!(vencoding::strip_bom(b"plain"), b"plain");
/// ```
#[must_use]
pub fn strip_bom(input: &[u8]) -> &[u8] {
    scan_bom(input).content
}

/// Returns `true` when `input` is valid UTF-8.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding;
///
/// assert!(vencoding::is_utf8("hello".as_bytes()));
/// assert!(!vencoding::is_utf8(&[0xff]));
/// ```
#[must_use]
pub const fn is_utf8(input: &[u8]) -> bool {
    core::str::from_utf8(input).is_ok()
}

/// Validates `input` as UTF-8 and returns a borrowed string slice.
///
/// # Errors
///
/// Returns [`core::str::Utf8Error`] when `input` is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding;
///
/// assert_eq!(vencoding::validate_utf8(b"hello").unwrap(), "hello");
/// assert!(vencoding::validate_utf8(&[0xff]).is_err());
/// ```
pub const fn validate_utf8(input: &[u8]) -> Result<&str, core::str::Utf8Error> {
    core::str::from_utf8(input)
}

/// Strips an initial BOM and validates the remaining bytes as UTF-8.
///
/// # Errors
///
/// Returns [`core::str::Utf8Error`] when the bytes after the BOM are not valid
/// UTF-8.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding;
///
/// assert_eq!(
///     vencoding::validate_utf8_without_bom(&[0xEF, 0xBB, 0xBF, b'a']).unwrap(),
///     "a",
/// );
/// ```
pub fn validate_utf8_without_bom(input: &[u8]) -> Result<&str, core::str::Utf8Error> {
    validate_utf8(strip_bom(input))
}

/// Decodes `input` as UTF-8, replacing invalid sequences with `U+FFFD`.
///
/// Valid UTF-8 returns a borrowed string slice.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use kniferrs::vencoding;
///
/// assert!(matches!(vencoding::decode_utf8_lossy(b"hello"), Cow::Borrowed("hello")));
/// assert_eq!(vencoding::decode_utf8_lossy(&[b'a', 0xff, b'b']), "a\u{FFFD}b");
/// ```
#[must_use]
pub fn decode_utf8_lossy(input: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(input)
}

/// Strips an initial BOM and decodes the remaining bytes as UTF-8 lossily.
///
/// # Examples
///
/// ```
/// use kniferrs::vencoding;
///
/// assert_eq!(
///     vencoding::decode_utf8_lossy_without_bom(&[0xEF, 0xBB, 0xBF, b'a', 0xff]),
///     "a\u{FFFD}",
/// );
/// ```
#[must_use]
pub fn decode_utf8_lossy_without_bom(input: &[u8]) -> Cow<'_, str> {
    decode_utf8_lossy(strip_bom(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vencoding_detects_bom_prefixes_with_overlapping_order() {
        assert_eq!(detect_bom(&[0xEF, 0xBB, 0xBF, b'a']), Some(Bom::Utf8));
        assert_eq!(detect_bom(&[0xFF, 0xFE, b'a']), Some(Bom::Utf16Le));
        assert_eq!(detect_bom(&[0xFE, 0xFF, b'a']), Some(Bom::Utf16Be));
        assert_eq!(
            detect_bom(&[0xFF, 0xFE, 0x00, 0x00, b'a']),
            Some(Bom::Utf32Le)
        );
        assert_eq!(
            detect_bom(&[0x00, 0x00, 0xFE, 0xFF, b'a']),
            Some(Bom::Utf32Be)
        );
        assert_eq!(detect_bom(b"plain"), None);
    }

    #[test]
    fn vencoding_scans_and_strips_bom_without_decoding_as_str() {
        let scan = scan_bom(&[0xEF, 0xBB, 0xBF, b'a', 0xff]);

        assert_eq!(scan.bom, Some(Bom::Utf8));
        assert_eq!(scan.content, &[b'a', 0xff]);
        assert_eq!(strip_bom(&[0xFE, 0xFF, b'a']), b"a");
        assert_eq!(strip_bom(b"plain"), b"plain");
        assert_eq!(Bom::Utf32Le.byte_len(), 4);
        assert_eq!(Bom::Utf32Be.encoding_name(), "UTF-32BE");
    }

    #[test]
    fn vencoding_validates_and_lossily_decodes_utf8_boundaries() {
        assert!(is_utf8("你好".as_bytes()));
        assert!(!is_utf8(&[0xff]));
        assert_eq!(validate_utf8(b"hello").unwrap(), "hello");
        assert!(validate_utf8(&[0xff]).is_err());
        assert_eq!(
            validate_utf8_without_bom(&[0xEF, 0xBB, 0xBF, b'h', b'i']).unwrap(),
            "hi"
        );
        assert_eq!(decode_utf8_lossy(&[b'a', 0xff, b'b']), "a\u{FFFD}b");
        assert_eq!(
            decode_utf8_lossy_without_bom(&[0xEF, 0xBB, 0xBF, b'a', 0xff]),
            "a\u{FFFD}"
        );
    }
}
