use super::casefold::chars_equal_ignore_case;

/// Returns `true` when both strings are equal, ignoring Unicode case.
///
/// This follows Rust's standard Unicode case mappings without expanding a
/// single scalar into multiple comparison characters. It covers common
/// `strings.EqualFold`-style simple folds such as final sigma and Kelvin sign,
/// while keeping German `ß` distinct from `ss`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::equals_ignore_case("Knifer-RS", "knifer-rs"));
/// assert!(vstr::equals_ignore_case("Σ", "ς"));
/// ```
#[must_use]
pub fn equals_ignore_case(left: &str, right: &str) -> bool {
    left.chars().eq(right.chars()) || {
        let mut left = left.chars();
        let mut right = right.chars();

        loop {
            match (left.next(), right.next()) {
                (Some(left), Some(right)) if chars_equal_ignore_case(left, right) => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

/// Reverses `input` by Unicode scalar values.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::reverse("ab你好"), "好你ba");
/// ```
#[must_use]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/// Returns the longest common leading substring of `left` and `right`.
///
/// The result borrows from `left` and always ends on a Unicode scalar boundary,
/// so it never splits a multi-byte character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::common_prefix("knifer-rs", "knifer-go"), "knifer-");
/// assert_eq!(vstr::common_prefix("你好世界", "你好朋友"), "你好");
/// assert_eq!(vstr::common_prefix("abc", "xyz"), "");
/// ```
#[must_use]
pub fn common_prefix<'src>(left: &'src str, right: &str) -> &'src str {
    let mut end = 0;
    for ((index, left_char), right_char) in left.char_indices().zip(right.chars()) {
        if left_char != right_char {
            break;
        }
        end = index + left_char.len_utf8();
    }
    &left[..end]
}

/// Returns the longest common trailing substring of `left` and `right`.
///
/// The result borrows from `left` and always starts on a Unicode scalar
/// boundary, so it never splits a multi-byte character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::common_suffix("knifer-rs", "wrapper-rs"), "er-rs");
/// assert_eq!(vstr::common_suffix("读写世界", "编写世界"), "写世界");
/// assert_eq!(vstr::common_suffix("abc", "xyz"), "");
/// ```
#[must_use]
pub fn common_suffix<'src>(left: &'src str, right: &str) -> &'src str {
    let mut start = left.len();
    let mut left_chars = left.char_indices().rev();
    let mut right_chars = right.chars().rev();
    loop {
        match (left_chars.next(), right_chars.next()) {
            (Some((index, left_char)), Some(right_char)) if left_char == right_char => {
                start = index;
            }
            _ => break,
        }
    }
    &left[start..]
}

/// Returns the part of `right` that differs after the common prefix with `left`.
///
/// This mirrors Apache Commons `StringUtils.difference`: when the strings share
/// a leading run, the shared run is dropped from `right`. The result borrows
/// from `right` and always starts on a Unicode scalar boundary.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::difference("i am a machine", "i am a robot"), "robot");
/// assert_eq!(vstr::difference("", "abc"), "abc");
/// assert_eq!(vstr::difference("abc", "abc"), "");
/// ```
#[must_use]
pub fn difference<'src>(left: &str, right: &'src str) -> &'src str {
    let shared = common_prefix(right, left).len();
    &right[shared..]
}
