use super::trim::trim;

/// Returns `true` when `input` has no bytes.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_empty(""));
/// assert!(!vstr::is_empty(" "));
/// ```
#[must_use]
pub const fn is_empty(input: &str) -> bool {
    input.is_empty()
}

/// Returns `true` when `input` has at least one byte.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_not_empty(" "));
/// assert!(!vstr::is_not_empty(""));
/// ```
#[must_use]
pub const fn is_not_empty(input: &str) -> bool {
    !input.is_empty()
}

/// Returns `true` when `input` is empty or only contains Unicode whitespace.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_blank(" \n\t"));
/// assert!(!vstr::is_blank("knifer-rs"));
/// ```
#[must_use]
pub fn is_blank(input: &str) -> bool {
    trim(input).is_empty()
}

/// Returns `true` when `input` contains at least one non-whitespace character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_not_blank("knifer-rs"));
/// assert!(!vstr::is_not_blank(" \n\t"));
/// ```
#[must_use]
pub fn is_not_blank(input: &str) -> bool {
    !is_blank(input)
}

/// Returns `true` when any value in `values` is empty.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::has_empty(["a", ""]));
/// assert!(!vstr::has_empty(["a", " "]));
/// ```
#[must_use]
pub fn has_empty<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().any(str::is_empty)
}

/// Returns `true` when any value in `values` is blank.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::has_blank(["a", " "]));
/// assert!(!vstr::has_blank(["a", "b"]));
/// ```
#[must_use]
pub fn has_blank<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().any(is_blank)
}

/// Returns `true` when all values in `values` are empty.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_all_empty(["", ""]));
/// assert!(!vstr::is_all_empty(["", " "]));
/// ```
#[must_use]
pub fn is_all_empty<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().all(str::is_empty)
}

/// Returns `true` when all values in `values` are blank.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_all_blank(["", " ", "\n"]));
/// assert!(!vstr::is_all_blank(["", "knifer-rs"]));
/// ```
#[must_use]
pub fn is_all_blank<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().all(is_blank)
}
