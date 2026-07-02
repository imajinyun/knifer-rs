use unicode_width::UnicodeWidthStr;

/// Returns the terminal display width of `input`.
///
/// This helper is available only with the `unicode-width` feature. It follows
/// the width rules provided by the `unicode-width` crate and is separate from
/// scalar-count helpers such as [`crate::vstr::char_len`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::display_width("abc你好"), 7);
/// # }
/// ```
#[must_use]
pub fn display_width(input: &str) -> usize {
    UnicodeWidthStr::width(input)
}

/// Returns a borrowed prefix whose terminal display width is at most `max_width`.
///
/// The returned slice never cuts through a UTF-8 character boundary. Prefixes
/// are measured as complete strings so emoji ZWJ sequences can follow the same
/// width rules as [`display_width`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::take_width("a你好", 3), "a你");
/// # }
/// ```
#[must_use]
pub fn take_width(input: &str, max_width: usize) -> &str {
    let mut end = 0usize;
    for (index, ch) in input.char_indices() {
        let next = index + ch.len_utf8();
        if display_width(&input[..next]) <= max_width {
            end = next;
        }
    }
    &input[..end]
}

/// Truncates text to at most `max_width` terminal display cells and appends a suffix.
///
/// If the suffix is wider than the budget, the suffix itself is truncated by
/// display width.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::truncate_width("你好Rust", 6, "..."), "你...");
/// assert_eq!(vstr::truncate_width("short", 10, "..."), "short");
/// # }
/// ```
#[must_use]
pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String {
    if max_width == 0 {
        return String::new();
    }
    if display_width(input) <= max_width {
        return input.to_owned();
    }

    let suffix_width = display_width(suffix);
    if suffix_width >= max_width {
        return take_width(suffix, max_width).to_owned();
    }

    let keep_width = max_width - suffix_width;
    let mut output = String::from(take_width(input, keep_width));
    output.push_str(suffix);
    output
}
