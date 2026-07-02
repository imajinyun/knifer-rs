/// Returns a borrowed prefix containing at most `max_chars` Unicode scalar values.
///
/// This never cuts through a UTF-8 character boundary.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::truncate("你好Rust", 3), "你好R");
/// ```
#[must_use]
pub fn truncate(input: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }

    match input.char_indices().nth(max_chars) {
        Some((index, _)) => &input[..index],
        None => input,
    }
}

/// Returns a borrowed prefix containing at most `count` Unicode scalar values.
///
/// This is a readability alias for [`truncate`] when the operation is about
/// selecting characters rather than shortening display text.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::take_chars("你好Rust", 3), "你好R");
/// ```
#[must_use]
pub fn take_chars(input: &str, count: usize) -> &str {
    truncate(input, count)
}

/// Drops at most `count` Unicode scalar values from the start of `input`.
///
/// This never cuts through a UTF-8 character boundary.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::drop_chars("你好Rust", 2), "Rust");
/// ```
#[must_use]
pub fn drop_chars(input: &str, count: usize) -> &str {
    if count == 0 {
        return input;
    }

    match input.char_indices().nth(count) {
        Some((index, _)) => &input[index..],
        None => "",
    }
}

/// Truncates text to at most `max_chars` Unicode scalar values and appends a suffix.
///
/// The returned string never exceeds `max_chars` scalar values. If `suffix` is
/// longer than the budget, the suffix itself is truncated. This helper is not
/// grapheme-aware: a multi-scalar user-perceived character can be split.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::truncate_with_suffix("你好Rust", 5, "..."), "你好...");
/// assert_eq!(vstr::truncate_with_suffix("short", 10, "..."), "short");
/// ```
#[must_use]
pub fn truncate_with_suffix(input: &str, max_chars: usize, suffix: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if input.chars().count() <= max_chars {
        return input.to_owned();
    }

    let suffix_len = suffix.chars().count();
    if suffix_len >= max_chars {
        return truncate(suffix, max_chars).to_owned();
    }

    let keep_chars = max_chars - suffix_len;
    let mut output = String::from(truncate(input, keep_chars));
    output.push_str(suffix);
    output
}

/// Abbreviates text by keeping both ends and inserting `marker` in the middle.
///
/// The returned string never exceeds `max_chars` Unicode scalar values unless
/// `marker` itself is longer than the budget, in which case the marker is
/// truncated.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::abbreviate_middle("abcdefghijklmnopqrstuvwxyz", 10, "..."), "abcd...xyz");
/// assert_eq!(vstr::abbreviate_middle("short", 10, "..."), "short");
/// ```
#[must_use]
pub fn abbreviate_middle(input: &str, max_chars: usize, marker: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if input.chars().count() <= max_chars {
        return input.to_owned();
    }

    let marker_len = marker.chars().count();
    if marker_len >= max_chars {
        return truncate(marker, max_chars).to_owned();
    }

    let keep = max_chars - marker_len;
    let front_chars = keep.div_ceil(2);
    let back_chars = keep / 2;
    let front = take_chars(input, front_chars);
    let back = take_last_chars(input, back_chars);

    let mut output = String::with_capacity(front.len() + marker.len() + back.len());
    output.push_str(front);
    output.push_str(marker);
    output.push_str(back);
    output
}

/// Limits text to at most `max_words` Unicode-whitespace separated words.
///
/// A suffix is appended only when words were omitted.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::limit_words("hello rust utility toolkit", 2, "..."), "hello rust...");
/// assert_eq!(vstr::limit_words("hello rust", 3, "..."), "hello rust");
/// ```
#[must_use]
pub fn limit_words(input: &str, max_words: usize, suffix: &str) -> String {
    if max_words == 0 {
        return String::new();
    }

    let mut parts = input.split_whitespace();
    let selected: Vec<&str> = parts.by_ref().take(max_words).collect();
    if selected.is_empty() {
        return String::new();
    }

    let mut output = selected.join(" ");
    if parts.next().is_some() {
        output.push_str(suffix);
    }
    output
}

/// Returns a short excerpt around the first occurrence of `needle`.
///
/// The excerpt is at most `max_chars` Unicode scalar values when possible. If
/// `needle` is empty or missing, this falls back to [`truncate_with_suffix`].
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::excerpt("hello rust utility toolkit", "utility", 14, "..."), "...st utility too...");
/// assert_eq!(vstr::excerpt("hello rust", "go", 8, "..."), "hello...");
/// ```
#[must_use]
pub fn excerpt(input: &str, needle: &str, max_chars: usize, marker: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let Some(byte_index) = input.find(needle).filter(|_| !needle.is_empty()) else {
        return truncate_with_suffix(input, max_chars, marker);
    };

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let start_char = chars
        .iter()
        .position(|(index, _)| *index == byte_index)
        .unwrap_or(0);
    let needle_chars = needle.chars().count();
    let input_chars = chars.len();

    if input_chars <= max_chars {
        return input.to_owned();
    }

    let context_budget = max_chars.saturating_sub(needle_chars).max(1);
    let before = context_budget / 2;
    let mut start = start_char.saturating_sub(before);
    let mut end = (start + max_chars).min(input_chars);
    if end < start_char + needle_chars {
        end = (start_char + needle_chars).min(input_chars);
        start = end.saturating_sub(max_chars);
    }

    let mut output = String::new();
    if start > 0 {
        output.push_str(marker);
    }
    output.push_str(slice_chars(input, start, end));
    if end < input_chars {
        output.push_str(marker);
    }
    output
}

/// Masks the middle of `input`, preserving `visible_start` and `visible_end` characters.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::mask("13800138000", 3, 4, '*'), "138****8000");
/// assert_eq!(vstr::mask("short", 10, 10, '*'), "short");
/// ```
#[must_use]
pub fn mask(input: &str, visible_start: usize, visible_end: usize, mask: char) -> String {
    let len = input.chars().count();
    if visible_start + visible_end >= len {
        return input.to_owned();
    }

    let start = take_chars(input, visible_start);
    let end = take_last_chars(input, visible_end);
    let mask_count = len - visible_start - visible_end;
    let mut output = String::with_capacity(input.len());
    output.push_str(start);
    output.extend(std::iter::repeat_n(mask, mask_count));
    output.push_str(end);
    output
}

pub(super) fn slice_chars(input: &str, start: usize, end: usize) -> &str {
    if start >= end {
        return "";
    }

    let start_byte = input
        .char_indices()
        .nth(start)
        .map_or(input.len(), |(index, _)| index);
    let end_byte = input
        .char_indices()
        .nth(end)
        .map_or(input.len(), |(index, _)| index);
    &input[start_byte..end_byte]
}

fn take_last_chars(input: &str, count: usize) -> &str {
    if count == 0 {
        return "";
    }

    let mut seen = 0;
    for (index, _) in input.char_indices().rev() {
        seen += 1;
        if seen == count {
            return &input[index..];
        }
    }
    input
}
