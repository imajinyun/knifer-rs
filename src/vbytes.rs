//! Byte-slice utilities for data that may not be valid UTF-8.
//!
//! All ranges are byte offsets. This facade intentionally stays separate from
//! [`crate::vstr`] so string helpers keep their valid UTF-8 semantics.

/// Returns the byte length of `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::byte_len(b"rust"), 4);
/// ```
#[must_use]
pub const fn byte_len(input: &[u8]) -> usize {
    input.len()
}

/// Returns `true` when `input` has no bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert!(vbytes::is_empty(b""));
/// assert!(!vbytes::is_empty(b" "));
/// ```
#[must_use]
pub const fn is_empty(input: &[u8]) -> bool {
    input.is_empty()
}

/// Returns `true` when `input` is valid UTF-8.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert!(vbytes::is_utf8("你好".as_bytes()));
/// assert!(!vbytes::is_utf8(&[0xff]));
/// ```
#[must_use]
pub const fn is_utf8(input: &[u8]) -> bool {
    core::str::from_utf8(input).is_ok()
}

/// Converts `input` to `&str` when it is valid UTF-8.
///
/// # Errors
///
/// Returns [`core::str::Utf8Error`] when `input` is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::to_str(b"rust").unwrap(), "rust");
/// assert!(vbytes::to_str(&[0xff]).is_err());
/// ```
pub const fn to_str(input: &[u8]) -> Result<&str, core::str::Utf8Error> {
    core::str::from_utf8(input)
}

/// Returns a borrowed byte range.
///
/// `from_index` is inclusive and `to_index` is exclusive. Negative indexes
/// count from the end, out-of-range indexes are clamped, and reversed ranges are
/// normalized.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::sub(b"abcdef", 1, 4), b"bcd");
/// assert_eq!(vbytes::sub(b"abcdef", -4, -1), b"cde");
/// assert_eq!(vbytes::sub(b"abcdef", 4, 1), b"bcd");
/// ```
#[must_use]
pub fn sub(input: &[u8], from_index: isize, to_index: isize) -> &[u8] {
    let len = input.len();
    if len == 0 {
        return input;
    }

    let start = normalize_index(from_index, len);
    let end = normalize_index(to_index, len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    &input[start..end]
}

/// Returns `input` without leading and trailing ASCII whitespace bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::trim_ascii(b" \tdata\n"), b"data");
/// ```
#[must_use]
pub fn trim_ascii(input: &[u8]) -> &[u8] {
    trim_ascii_end(trim_ascii_start(input))
}

/// Returns `input` without leading ASCII whitespace bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::trim_ascii_start(b" \tdata\n"), b"data\n");
/// ```
#[must_use]
pub fn trim_ascii_start(input: &[u8]) -> &[u8] {
    let start = input
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(input.len());
    &input[start..]
}

/// Returns `input` without trailing ASCII whitespace bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::trim_ascii_end(b" \tdata\n"), b" \tdata");
/// ```
#[must_use]
pub fn trim_ascii_end(input: &[u8]) -> &[u8] {
    let end = input
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map_or(0, |index| index + 1);
    &input[..end]
}

/// Returns `true` when `needle` occurs in `input`.
///
/// Empty needles always match.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert!(vbytes::contains(b"abc", b"bc"));
/// assert!(vbytes::contains(b"abc", b""));
/// ```
#[must_use]
pub fn contains(input: &[u8], needle: &[u8]) -> bool {
    find(input, needle).is_some()
}

/// Returns the first byte range where `needle` occurs in `input`.
///
/// Empty needles return `Some((0, 0))`.
///
/// The literal scan is shared with [`find_all`]. Enabling the `search-memchr`
/// feature routes it through the SIMD-accelerated `memchr::memmem` searcher
/// without changing any results.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::find(b"abcabc", b"bc"), Some((1, 3)));
/// ```
#[must_use]
pub fn find(input: &[u8], needle: &[u8]) -> Option<(usize, usize)> {
    if needle.is_empty() {
        return Some((0, 0));
    }

    raw_find(input, needle).map(|start| (start, start + needle.len()))
}

/// Returns all non-overlapping byte ranges where `needle` occurs in `input`.
///
/// Empty needles return an empty vector so callers cannot accidentally loop
/// forever.
///
/// Like [`find`], this uses the shared literal scan and honors the optional
/// `search-memchr` backend.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::find_all(b"aaaa", b"aa"), vec![(0, 2), (2, 4)]);
/// ```
#[must_use]
pub fn find_all(input: &[u8], needle: &[u8]) -> Vec<(usize, usize)> {
    if needle.is_empty() {
        return Vec::new();
    }

    let mut found = Vec::new();
    let mut offset = 0usize;
    while let Some(relative) = raw_find(&input[offset..], needle) {
        let start = offset + relative;
        let end = start + needle.len();
        found.push((start, end));
        offset = end;
    }
    found
}

/// Returns the first start index of a non-empty `needle` within `haystack`.
///
/// This is the single literal byte searcher shared by [`find`] and [`find_all`].
/// The default build scans with the standard library; the `search-memchr`
/// feature swaps in the SIMD-accelerated `memchr::memmem::find` searcher while
/// producing identical leftmost results. Callers guarantee `needle` is
/// non-empty.
#[cfg(not(feature = "search-memchr"))]
fn raw_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(feature = "search-memchr")]
fn raw_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    memchr::memmem::find(haystack, needle)
}

/// Returns `input` without `prefix` when it is present.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::strip_prefix(b"abc", b"a"), Some(&b"bc"[..]));
/// assert_eq!(vbytes::strip_prefix(b"abc", b"x"), None);
/// ```
#[must_use]
pub fn strip_prefix<'src>(input: &'src [u8], prefix: &[u8]) -> Option<&'src [u8]> {
    input.strip_prefix(prefix)
}

/// Returns `input` without `suffix` when it is present.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::strip_suffix(b"abc", b"c"), Some(&b"ab"[..]));
/// assert_eq!(vbytes::strip_suffix(b"abc", b"x"), None);
/// ```
#[must_use]
pub fn strip_suffix<'src>(input: &'src [u8], suffix: &[u8]) -> Option<&'src [u8]> {
    input.strip_suffix(suffix)
}

/// Replaces all non-overlapping `from` byte sequences with `to`.
///
/// Empty `from` returns `input` unchanged.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::replace_all(b"aaaa", b"aa", b"b"), b"bb");
/// ```
#[must_use]
pub fn replace_all(input: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    if from.is_empty() {
        return input.to_vec();
    }

    let mut output = Vec::with_capacity(input.len());
    let mut offset = 0usize;
    while offset < input.len() {
        let Some((start, end)) =
            find(&input[offset..], from).map(|(start, end)| (offset + start, offset + end))
        else {
            output.extend_from_slice(&input[offset..]);
            return output;
        };
        output.extend_from_slice(&input[offset..start]);
        output.extend_from_slice(to);
        offset = end;
    }
    output
}

/// Decodes `input` as UTF-8, replacing invalid sequences with the Unicode
/// replacement character `U+FFFD`.
///
/// Invalid bytes are replaced following the Unicode "substitution of maximal
/// subparts" rule, so this matches [`String::from_utf8_lossy`] and yields one
/// replacement char per maximal invalid subsequence.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::chars(b"ab"), vec!['a', 'b']);
/// assert_eq!(vbytes::chars(&[b'a', 0xff, b'b']), vec!['a', '\u{fffd}', 'b']);
/// ```
#[must_use]
pub fn chars(input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    for chunk in input.utf8_chunks() {
        output.extend(chunk.valid().chars());
        if !chunk.invalid().is_empty() {
            output.push(char::REPLACEMENT_CHARACTER);
        }
    }
    output
}

/// Decodes `input` as UTF-8 into `(start, end, char)` triples using byte
/// offsets.
///
/// `start` is inclusive and `end` is exclusive. Invalid sequences follow the
/// Unicode "substitution of maximal subparts" rule: each maximal invalid
/// subsequence yields a single `U+FFFD` whose range spans all of its bytes.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(
///     vbytes::char_indices("é".as_bytes()),
///     vec![(0, 2, 'é')]
/// );
/// assert_eq!(
///     vbytes::char_indices(&[b'a', 0xff, 0xfe, b'b']),
///     vec![(0, 1, 'a'), (1, 2, '\u{fffd}'), (2, 3, '\u{fffd}'), (3, 4, 'b')]
/// );
/// ```
#[must_use]
pub fn char_indices(input: &[u8]) -> Vec<(usize, usize, char)> {
    let mut output = Vec::new();
    let mut offset = 0usize;
    for chunk in input.utf8_chunks() {
        let valid = chunk.valid();
        for (relative, ch) in valid.char_indices() {
            let start = offset + relative;
            output.push((start, start + ch.len_utf8(), ch));
        }
        offset += valid.len();

        let invalid = chunk.invalid();
        if !invalid.is_empty() {
            output.push((offset, offset + invalid.len(), char::REPLACEMENT_CHARACTER));
            offset += invalid.len();
        }
    }
    output
}

/// Splits `input` into lines using [`str::lines`] semantics on bytes.
///
/// Lines are split at `\n`, and a single trailing `\r` before the `\n` is
/// removed. The final line terminator is optional and terminators are not
/// included in the returned slices. Empty input yields no lines.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(vbytes::lines(b"a\nb\n"), vec![&b"a"[..], &b"b"[..]]);
/// assert_eq!(vbytes::lines(b"a\r\nb"), vec![&b"a"[..], &b"b"[..]]);
/// assert!(vbytes::lines(b"").is_empty());
/// ```
#[must_use]
pub fn lines(input: &[u8]) -> Vec<&[u8]> {
    let mut output = Vec::new();
    let mut start = 0usize;
    for (index, byte) in input.iter().enumerate() {
        if *byte != b'\n' {
            continue;
        }
        let mut end = index;
        if end > start && input[end - 1] == b'\r' {
            end -= 1;
        }
        output.push(&input[start..end]);
        start = index + 1;
    }
    if start < input.len() {
        output.push(&input[start..]);
    }
    output
}

/// Splits `input` into fields separated by runs of ASCII whitespace.
///
/// Leading, trailing, and repeated ASCII whitespace never produce empty fields,
/// mirroring [`str::split_whitespace`] but on raw bytes so invalid UTF-8 is
/// preserved.
///
/// # Examples
///
/// ```
/// use kniferrs::vbytes;
///
/// assert_eq!(
///     vbytes::fields(b"  a\tb \n c "),
///     vec![&b"a"[..], &b"b"[..], &b"c"[..]]
/// );
/// assert!(vbytes::fields(b" \t\n").is_empty());
/// ```
#[must_use]
pub fn fields(input: &[u8]) -> Vec<&[u8]> {
    input
        .split(u8::is_ascii_whitespace)
        .filter(|field| !field.is_empty())
        .collect()
}

fn normalize_index(index: isize, len: usize) -> usize {
    if index < 0 {
        len.saturating_sub(index.unsigned_abs())
    } else {
        usize::try_from(index).map_or(len, |index| index.min(len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vbytes_core_helpers_use_byte_semantics() {
        let invalid = [b'a', 0xff, b'b'];

        assert_eq!(byte_len(&invalid), 3);
        assert!(!is_empty(&invalid));
        assert!(!is_utf8(&invalid));
        assert!(to_str(&invalid).is_err());
        assert_eq!(to_str("你好".as_bytes()).unwrap(), "你好");

        assert_eq!(sub(&invalid, 1, 2), &[0xff]);
        assert_eq!(sub(&invalid, -2, -1), &[0xff]);
        assert_eq!(sub(&invalid, 3, 0), invalid);
    }

    #[test]
    fn vbytes_trim_and_prefix_suffix_borrow_input() {
        let input = b" \tdata\n";

        assert_eq!(trim_ascii(input), b"data");
        assert_eq!(trim_ascii_start(input), b"data\n");
        assert_eq!(trim_ascii_end(input), b" \tdata");
        assert_eq!(trim_ascii(b" \n\t"), b"");
        assert_eq!(strip_prefix(b"abc", b"a"), Some(&b"bc"[..]));
        assert_eq!(strip_suffix(b"abc", b"c"), Some(&b"ab"[..]));
        assert_eq!(strip_prefix(b"abc", b"x"), None);
        assert_eq!(strip_suffix(b"abc", b"x"), None);
    }

    #[test]
    fn vbytes_search_and_replace_are_non_overlapping() {
        assert!(contains(b"abc", b""));
        assert!(contains(b"abc", b"bc"));
        assert!(!contains(b"abc", b"bd"));
        assert_eq!(find(b"abcabc", b"bc"), Some((1, 3)));
        assert_eq!(find(b"abc", b""), Some((0, 0)));
        assert_eq!(find(b"abc", b"d"), None);
        assert_eq!(find_all(b"aaaa", b"aa"), vec![(0, 2), (2, 4)]);
        assert_eq!(
            find_all(b"aaaa", b"a"),
            vec![(0, 1), (1, 2), (2, 3), (3, 4)]
        );
        assert!(find_all(b"abc", b"").is_empty());
        assert_eq!(replace_all(b"aaaa", b"aa", b"b"), b"bb");
        assert_eq!(replace_all(b"abc", b"", b"x"), b"abc");
        assert_eq!(replace_all(&[0xff, b'a', 0xff], &[0xff], b"?"), b"?a?");
    }

    #[test]
    fn vbytes_lossy_decoding_uses_maximal_subpart_replacement() {
        assert_eq!(chars(b"ab"), vec!['a', 'b']);
        assert_eq!(chars("héllo".as_bytes()), vec!['h', 'é', 'l', 'l', 'o']);
        assert_eq!(chars(&[b'a', 0xff, b'b']), vec!['a', '\u{fffd}', 'b']);
        // Two separate invalid bytes yield two replacement chars, matching
        // String::from_utf8_lossy maximal-subpart behavior.
        assert_eq!(
            chars(&[b'a', 0xff, 0xfe, b'b']),
            vec!['a', '\u{fffd}', '\u{fffd}', 'b']
        );
        assert_eq!(
            String::from_utf8_lossy(&[b'a', 0xff, 0xfe, b'b'])
                .chars()
                .collect::<Vec<_>>(),
            chars(&[b'a', 0xff, 0xfe, b'b'])
        );

        assert_eq!(char_indices("é".as_bytes()), vec![(0, 2, 'é')]);
        assert_eq!(
            char_indices(&[b'a', 0xff, 0xfe, b'b']),
            vec![
                (0, 1, 'a'),
                (1, 2, '\u{fffd}'),
                (2, 3, '\u{fffd}'),
                (3, 4, 'b')
            ]
        );
        assert!(chars(b"").is_empty());
        assert!(char_indices(b"").is_empty());
    }

    #[test]
    fn vbytes_lines_and_fields_preserve_invalid_bytes() {
        assert_eq!(lines(b"a\nb\n"), vec![&b"a"[..], &b"b"[..]]);
        assert_eq!(lines(b"a\r\nb"), vec![&b"a"[..], &b"b"[..]]);
        assert_eq!(lines(b"a\n\nb"), vec![&b"a"[..], &b""[..], &b"b"[..]]);
        assert_eq!(lines(&[0xff, b'\n', 0xfe]), vec![&[0xff][..], &[0xfe][..]]);
        assert!(lines(b"").is_empty());

        assert_eq!(
            fields(b"  a\tb \n c "),
            vec![&b"a"[..], &b"b"[..], &b"c"[..]]
        );
        assert_eq!(fields(&[0xff, b' ', 0xfe]), vec![&[0xff][..], &[0xfe][..]]);
        assert!(fields(b" \t\n").is_empty());
        assert!(fields(b"").is_empty());
    }

    // Independent naive oracle for the shared literal byte searcher. It never
    // uses `raw_find`, so it validates both the default scan and the optional
    // `search-memchr` backend against the same leftmost, non-overlapping rule.
    fn oracle_find_all(haystack: &[u8], needle: &[u8]) -> Vec<(usize, usize)> {
        if needle.is_empty() {
            return Vec::new();
        }
        let mut ranges = Vec::new();
        let mut offset = 0usize;
        while offset + needle.len() <= haystack.len() {
            if &haystack[offset..offset + needle.len()] == needle {
                ranges.push((offset, offset + needle.len()));
                offset += needle.len();
            } else {
                offset += 1;
            }
        }
        ranges
    }

    #[test]
    fn vbytes_literal_search_matches_naive_oracle() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"", b"a"),
            (b"abc", b""),
            (b"abcabcabc", b"abc"),
            (b"aaaa", b"aa"),
            (b"aaaaa", b"aa"),
            (b"the quick brown fox", b"o"),
            (b"mississippi", b"issi"),
            (&[0xff, b'a', 0xff, 0xfe, b'a', 0xff], &[0xff]),
            (&[0x00, 0x01, 0x00, 0x01, 0x00], &[0x00, 0x01]),
            (b"needle", b"needle"),
            (b"short", b"longer-needle"),
        ];

        for (haystack, needle) in cases {
            let expected = oracle_find_all(haystack, needle);
            assert_eq!(
                find_all(haystack, needle),
                expected,
                "find_all mismatch for {haystack:?} / {needle:?}"
            );

            let expected_find = if needle.is_empty() {
                Some((0, 0))
            } else {
                expected.first().copied()
            };
            assert_eq!(
                find(haystack, needle),
                expected_find,
                "find mismatch for {haystack:?} / {needle:?}"
            );
            assert_eq!(contains(haystack, needle), find(haystack, needle).is_some());
        }
    }
}
