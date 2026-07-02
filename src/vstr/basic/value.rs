use super::measure::char_len;
use super::predicate::is_blank;

/// Repeats `input` `count` times.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::repeat("ab", 3), "ababab");
/// assert_eq!(vstr::repeat("ab", 0), "");
/// ```
#[must_use]
pub fn repeat(input: &str, count: usize) -> String {
    input.repeat(count)
}

/// Rotates `input` by `shift` Unicode scalar values.
///
/// This mirrors Apache Commons `StringUtils.rotate`: a positive `shift` moves
/// characters to the right (the tail wraps to the front) and a negative `shift`
/// moves them to the left. The shift is taken modulo the scalar length, so large
/// magnitudes wrap cleanly and empty input is returned unchanged.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::rotate("abcdefg", 2), "fgabcde");
/// assert_eq!(vstr::rotate("abcdefg", -2), "cdefgab");
/// assert_eq!(vstr::rotate("你好世界", 1), "界你好世");
/// ```
#[must_use]
pub fn rotate(input: &str, shift: isize) -> String {
    let len = char_len(input);
    if len == 0 {
        return input.to_owned();
    }

    let magnitude = shift.unsigned_abs() % len;
    let normalized = if magnitude == 0 {
        0
    } else if shift >= 0 {
        magnitude
    } else {
        len - magnitude
    };
    if normalized == 0 {
        return input.to_owned();
    }

    let split = len - normalized;
    let boundary = input
        .char_indices()
        .nth(split)
        .map_or(input.len(), |(index, _)| index);
    let mut output = String::with_capacity(input.len());
    output.push_str(&input[boundary..]);
    output.push_str(&input[..boundary]);
    output
}

/// Pads `input` on the left until it reaches `target_len` Unicode scalar values.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::pad_left("42", 5, '0'), "00042");
/// assert_eq!(vstr::pad_left("你好", 3, '*'), "*你好");
/// ```
#[must_use]
pub fn pad_left(input: &str, target_len: usize, pad: char) -> String {
    let input_len = char_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output.push_str(input);
    output
}

/// Pads `input` on the right until it reaches `target_len` Unicode scalar values.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::pad_right("42", 5, '0'), "42000");
/// assert_eq!(vstr::pad_right("你好", 3, '*'), "你好*");
/// ```
#[must_use]
pub fn pad_right(input: &str, target_len: usize, pad: char) -> String {
    let input_len = char_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.push_str(input);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output
}

/// Returns `default` when `input` is empty, otherwise returns `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::default_if_empty("", "fallback"), "fallback");
/// assert_eq!(vstr::default_if_empty(" ", "fallback"), " ");
/// ```
#[must_use]
pub fn default_if_empty<'src>(input: &'src str, default: &'src str) -> &'src str {
    if input.is_empty() { default } else { input }
}

/// Returns `default` when `input` is blank, otherwise returns `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::default_if_blank(" ", "fallback"), "fallback");
/// assert_eq!(vstr::default_if_blank("value", "fallback"), "value");
/// ```
#[must_use]
pub fn default_if_blank<'src>(input: &'src str, default: &'src str) -> &'src str {
    if is_blank(input) { default } else { input }
}
