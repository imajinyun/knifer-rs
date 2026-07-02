/// Returns a substring by Unicode scalar-value indexes.
///
/// `from_index` is inclusive and `to_index` is exclusive. Negative indexes
/// count from the end, out-of-range indexes are clamped, and reversed ranges
/// are normalized.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::sub("abcdef", 1, 4), "bcd");
/// assert_eq!(vstr::sub("你好世界", 1, -1), "好世");
/// assert_eq!(vstr::sub("abcdef", 4, 1), "bcd");
/// ```
#[must_use]
pub fn sub(input: &str, from_index: isize, to_index: isize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let start = normalize_index(from_index, len);
    let end = normalize_index(to_index, len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    chars[start..end].iter().collect()
}

/// Returns text before `separator`.
///
/// When `use_last_separator` is `true`, the last separator is used. If
/// `separator` is empty or not found, `input` is returned unchanged.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::sub_before("a/b/c", "/", false), "a");
/// assert_eq!(vstr::sub_before("a/b/c", "/", true), "a/b");
/// assert_eq!(vstr::sub_before("a/b/c", "|", false), "a/b/c");
/// ```
#[must_use]
pub fn sub_before<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str {
    if input.is_empty() || separator.is_empty() {
        return input;
    }

    let index = if use_last_separator {
        input.rfind(separator)
    } else {
        input.find(separator)
    };

    index.map_or(input, |index| &input[..index])
}

/// Returns text after `separator`.
///
/// When `use_last_separator` is `true`, the last separator is used. If
/// `separator` is empty or not found, an empty string slice is returned.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::sub_after("a/b/c", "/", false), "b/c");
/// assert_eq!(vstr::sub_after("a/b/c", "/", true), "c");
/// assert_eq!(vstr::sub_after("a/b/c", "|", false), "");
/// ```
#[must_use]
pub fn sub_after<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str {
    if input.is_empty() || separator.is_empty() {
        return "";
    }

    let index = if use_last_separator {
        input.rfind(separator)
    } else {
        input.find(separator)
    };

    index.map_or("", |index| &input[index + separator.len()..])
}

/// Returns text before the first `separator`.
///
/// This is a concise alias for `sub_before(input, separator, false)`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::before("a/b/c", "/"), "a");
/// assert_eq!(vstr::before("a/b/c", "|"), "a/b/c");
/// ```
#[must_use]
pub fn before<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_before(input, separator, false)
}

/// Returns text before the last `separator`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::before_last("a/b/c", "/"), "a/b");
/// ```
#[must_use]
pub fn before_last<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_before(input, separator, true)
}

/// Returns text after the first `separator`.
///
/// This is a concise alias for `sub_after(input, separator, false)`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::after("a/b/c", "/"), "b/c");
/// assert_eq!(vstr::after("a/b/c", "|"), "");
/// ```
#[must_use]
pub fn after<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_after(input, separator, false)
}

/// Returns text after the last `separator`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::after_last("a/b/c", "/"), "c");
/// ```
#[must_use]
pub fn after_last<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_after(input, separator, true)
}

/// Returns text between the first `start` and the following `end` marker.
///
/// `None` is returned when either marker is empty or missing. The returned
/// slice borrows from `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::between("id=[42]", "[", "]"), Some("42"));
/// assert_eq!(vstr::between("id=42", "[", "]"), None);
/// ```
#[must_use]
pub fn between<'src>(input: &'src str, start: &str, end: &str) -> Option<&'src str> {
    if start.is_empty() || end.is_empty() {
        return None;
    }

    let content_start = input.find(start)? + start.len();
    let rest = &input[content_start..];
    let content_end = rest.find(end)?;
    Some(&rest[..content_end])
}

fn normalize_index(index: isize, len: usize) -> usize {
    let len = isize::try_from(len).unwrap_or(isize::MAX);
    let normalized = if index < 0 { len + index } else { index };
    usize::try_from(normalized.clamp(0, len)).unwrap_or(usize::MAX)
}
