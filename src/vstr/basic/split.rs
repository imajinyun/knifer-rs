/// Splits `input` by `separator`.
///
/// Empty input returns an empty vector. This mirrors common utility-library
/// behavior for data-cleanup paths where an empty field should produce no
/// parts.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::split("a,b", ","), vec!["a", "b"]);
/// assert!(vstr::split("", ",").is_empty());
/// ```
#[must_use]
pub fn split<'src>(input: &'src str, separator: &str) -> Vec<&'src str> {
    if input.is_empty() {
        Vec::new()
    } else {
        input.split(separator).collect()
    }
}

/// Splits `input`, trims each part, and drops blank parts.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::split_trim(" a, ,b ", ","), vec!["a", "b"]);
/// ```
#[must_use]
pub fn split_trim<'src>(input: &'src str, separator: &str) -> Vec<&'src str> {
    input
        .split(separator)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect()
}

/// Splits `input` once at the first `separator`.
///
/// `None` is returned when the separator is empty or missing. The returned
/// slices borrow from `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::split_once("a=b=c", "="), Some(("a", "b=c")));
/// assert_eq!(vstr::split_once("abc", "="), None);
/// ```
#[must_use]
pub fn split_once<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)> {
    if separator.is_empty() {
        return None;
    }

    input
        .find(separator)
        .map(|index| (&input[..index], &input[index + separator.len()..]))
}

/// Splits `input` once at the last `separator`.
///
/// `None` is returned when the separator is empty or missing. The returned
/// slices borrow from `input`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::split_once_last("a=b=c", "="), Some(("a=b", "c")));
/// assert_eq!(vstr::split_once_last("abc", "="), None);
/// ```
#[must_use]
pub fn split_once_last<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)> {
    if separator.is_empty() {
        return None;
    }

    input
        .rfind(separator)
        .map(|index| (&input[..index], &input[index + separator.len()..]))
}
