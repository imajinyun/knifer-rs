/// Inserts `text` at a Unicode scalar-value index.
///
/// `char_index` counts Unicode scalar values, so the insertion never splits a
/// multi-byte character. Indexes past the end are clamped, so a large index
/// appends `text`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::insert("abcd", 2, "XY"), "abXYcd");
/// assert_eq!(vstr::insert("你好", 1, "-"), "你-好");
/// assert_eq!(vstr::insert("abc", 99, "!"), "abc!");
/// ```
#[must_use]
pub fn insert(input: &str, char_index: usize, text: &str) -> String {
    let byte_index = char_to_byte(input, char_index);
    let mut output = String::with_capacity(input.len() + text.len());
    output.push_str(&input[..byte_index]);
    output.push_str(text);
    output.push_str(&input[byte_index..]);
    output
}

/// Replaces the scalar range `[start, end)` with `overlay`, clamping leniently.
///
/// This mirrors Apache Commons `StringUtils.overlay`: `start` and `end` count
/// Unicode scalar values, out-of-range indexes are clamped, and a reversed range
/// is normalized. Use [`replace_range`] when an invalid range should be rejected
/// instead of clamped.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::overlay("abcdef", "ZZ", 2, 4), "abZZef");
/// assert_eq!(vstr::overlay("abcdef", "ZZ", 4, 2), "abZZef");
/// assert_eq!(vstr::overlay("abc", "ZZ", 2, 99), "abZZ");
/// ```
#[must_use]
pub fn overlay(input: &str, overlay: &str, start: usize, end: usize) -> String {
    let char_len = input.chars().count();
    let start = start.min(char_len);
    let end = end.min(char_len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    let byte_start = char_to_byte(input, start);
    let byte_end = char_to_byte(input, end);
    let mut output = String::with_capacity(byte_start + overlay.len() + (input.len() - byte_end));
    output.push_str(&input[..byte_start]);
    output.push_str(overlay);
    output.push_str(&input[byte_end..]);
    output
}

/// Removes the scalar range `[start, end)`, clamping leniently.
///
/// `start` and `end` count Unicode scalar values. Out-of-range indexes are
/// clamped and a reversed range is normalized, matching [`overlay`] with an
/// empty replacement.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::remove_range("abcdef", 2, 4), "abef");
/// assert_eq!(vstr::remove_range("你好世界", 1, 3), "你界");
/// assert_eq!(vstr::remove_range("abc", 1, 99), "a");
/// ```
#[must_use]
pub fn remove_range(input: &str, start: usize, end: usize) -> String {
    overlay(input, "", start, end)
}

/// Replaces the scalar range `[start, end)` with `replacement`, strictly.
///
/// Unlike [`overlay`], this returns `None` when the range is invalid — that is
/// when `start > end` or `end` exceeds the scalar length. `start` and `end`
/// count Unicode scalar values, so the result never splits a multi-byte
/// character.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::replace_range("abcdef", 2, 4, "ZZ"), Some("abZZef".to_string()));
/// assert_eq!(vstr::replace_range("abc", 1, 1, "-"), Some("a-bc".to_string()));
/// assert_eq!(vstr::replace_range("abc", 2, 1, "-"), None);
/// assert_eq!(vstr::replace_range("abc", 0, 99, "-"), None);
/// ```
#[must_use]
pub fn replace_range(input: &str, start: usize, end: usize, replacement: &str) -> Option<String> {
    let char_len = input.chars().count();
    if start > end || end > char_len {
        return None;
    }

    let byte_start = char_to_byte(input, start);
    let byte_end = char_to_byte(input, end);
    let mut output =
        String::with_capacity(byte_start + replacement.len() + (input.len() - byte_end));
    output.push_str(&input[..byte_start]);
    output.push_str(replacement);
    output.push_str(&input[byte_end..]);
    Some(output)
}

/// Splits `input` into borrowed pieces of at most `size` Unicode scalar values.
///
/// The final piece may be shorter when the length is not a multiple of `size`.
/// A `size` of zero and an empty input both return an empty vector. Pieces never
/// split a multi-byte character.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::chunk("abcdefg", 3), vec!["abc", "def", "g"]);
/// assert_eq!(vstr::chunk("你好世界", 2), vec!["你好", "世界"]);
/// assert!(vstr::chunk("abc", 0).is_empty());
/// ```
#[must_use]
pub fn chunk(input: &str, size: usize) -> Vec<&str> {
    if size == 0 || input.is_empty() {
        return Vec::new();
    }

    let mut boundaries: Vec<usize> = input.char_indices().map(|(index, _)| index).collect();
    boundaries.push(input.len());

    let char_count = boundaries.len() - 1;
    let mut pieces = Vec::with_capacity(char_count.div_ceil(size));
    let mut start = 0;
    while start < char_count {
        let end = (start + size).min(char_count);
        pieces.push(&input[boundaries[start]..boundaries[end]]);
        start = end;
    }
    pieces
}

/// Maps a Unicode scalar-value index to a byte index, clamping to `input.len()`.
fn char_to_byte(input: &str, char_index: usize) -> usize {
    input
        .char_indices()
        .nth(char_index)
        .map_or(input.len(), |(index, _)| index)
}
