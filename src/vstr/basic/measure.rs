/// Returns the byte length of `input`.
///
/// For Unicode scalar-value length, use [`char_len`].
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::byte_len("你好"), 6);
/// ```
#[must_use]
pub const fn byte_len(input: &str) -> usize {
    input.len()
}

/// Returns the number of Unicode scalar values in `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::char_len("你好"), 2);
/// ```
#[must_use]
pub fn char_len(input: &str) -> usize {
    input.chars().count()
}

/// Returns the number of Unicode scalar values in `input`.
///
/// This is an alias for [`char_len`] to align with `knifer-go`'s `Length` API
/// name.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::length("你好"), 2);
/// ```
#[must_use]
pub fn length(input: &str) -> usize {
    char_len(input)
}

/// Returns the number of Unicode scalar values in `input`.
///
/// This is an alias for [`char_len`] to align with `knifer-go`'s `RuneLen` API
/// name.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::rune_len("你好"), 2);
/// ```
#[must_use]
pub fn rune_len(input: &str) -> usize {
    char_len(input)
}
