use super::measure::char_len;
use super::predicate::is_blank;

/// Repeats `input` `count` times.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::repeat("ab", 3), "ababab");
/// assert_eq!(vstr::repeat("ab", 0), "");
/// ```
#[must_use]
pub fn repeat(input: &str, count: usize) -> String {
    input.repeat(count)
}

/// Pads `input` on the left until it reaches `target_len` Unicode scalar values.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::default_if_blank(" ", "fallback"), "fallback");
/// assert_eq!(vstr::default_if_blank("value", "fallback"), "value");
/// ```
#[must_use]
pub fn default_if_blank<'src>(input: &'src str, default: &'src str) -> &'src str {
    if is_blank(input) { default } else { input }
}
