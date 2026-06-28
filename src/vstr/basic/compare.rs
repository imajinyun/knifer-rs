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
