use super::casefold::{prefix_end_ignore_case, suffix_start_ignore_case};

/// Returns `true` when `input` starts with `prefix`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::starts_with("knifer-rs", "knife"));
/// ```
#[must_use]
pub fn starts_with(input: &str, prefix: &str) -> bool {
    input.starts_with(prefix)
}

/// Returns `true` when `input` ends with `suffix`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ends_with("knifer-rs", "rs"));
/// ```
#[must_use]
pub fn ends_with(input: &str, suffix: &str) -> bool {
    input.ends_with(suffix)
}

/// Returns `true` when `input` starts with `prefix`, ignoring Unicode case.
///
/// This uses the same simple scalar-by-scalar case folding as
/// [`crate::vstr::equals_ignore_case`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::starts_with_ignore_case("Knifer-RS", "knife"));
/// assert!(vstr::starts_with_ignore_case("\u{212A}nife", "k"));
/// ```
#[must_use]
pub fn starts_with_ignore_case(input: &str, prefix: &str) -> bool {
    prefix_end_ignore_case(input, prefix).is_some()
}

/// Returns `true` when `input` ends with `suffix`, ignoring Unicode case.
///
/// This uses the same simple scalar-by-scalar case folding as
/// [`crate::vstr::equals_ignore_case`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ends_with_ignore_case("Knifer-RS", "RS"));
/// assert!(vstr::ends_with_ignore_case("abc\u{212A}", "k"));
/// ```
#[must_use]
pub fn ends_with_ignore_case(input: &str, suffix: &str) -> bool {
    suffix_start_ignore_case(input, suffix).is_some()
}

/// Returns `input` without `prefix` when it is present.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_prefix("knifer-rs", "knifer-"), "rs");
/// assert_eq!(vstr::remove_prefix("knifer-rs", "go-"), "knifer-rs");
/// ```
#[must_use]
pub fn remove_prefix<'src>(input: &'src str, prefix: &str) -> &'src str {
    input.strip_prefix(prefix).unwrap_or(input)
}

/// Returns `input` without `suffix` when it is present.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_suffix("knifer-rs", "-rs"), "knifer");
/// assert_eq!(vstr::remove_suffix("knifer-rs", "-go"), "knifer-rs");
/// ```
#[must_use]
pub fn remove_suffix<'src>(input: &'src str, suffix: &str) -> &'src str {
    input.strip_suffix(suffix).unwrap_or(input)
}

/// Strips `prefix` from `input`, ignoring Unicode case.
///
/// The returned slice preserves the original input casing. `None` is returned
/// when the prefix is not present.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::strip_prefix_ignore_case("Knifer-RS", "knife"), Some("r-RS"));
/// assert_eq!(vstr::strip_prefix_ignore_case("Knifer-RS", "go"), None);
/// ```
#[must_use]
pub fn strip_prefix_ignore_case<'src>(input: &'src str, prefix: &str) -> Option<&'src str> {
    prefix_end_ignore_case(input, prefix).map(|end| &input[end..])
}

/// Strips `suffix` from `input`, ignoring Unicode case.
///
/// The returned slice preserves the original input casing. `None` is returned
/// when the suffix is not present.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
/// assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "go"), None);
/// ```
#[must_use]
pub fn strip_suffix_ignore_case<'src>(input: &'src str, suffix: &str) -> Option<&'src str> {
    suffix_start_ignore_case(input, suffix).map(|start| &input[..start])
}

/// Adds `prefix` when `input` does not already start with it.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::add_prefix_if_not("path", "/"), "/path");
/// assert_eq!(vstr::add_prefix_if_not("/path", "/"), "/path");
/// ```
#[must_use]
pub fn add_prefix_if_not(input: &str, prefix: &str) -> String {
    if input.starts_with(prefix) {
        input.to_owned()
    } else {
        let mut output = String::with_capacity(prefix.len() + input.len());
        output.push_str(prefix);
        output.push_str(input);
        output
    }
}

/// Adds `suffix` when `input` does not already end with it.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::add_suffix_if_not("path", "/"), "path/");
/// assert_eq!(vstr::add_suffix_if_not("path/", "/"), "path/");
/// ```
#[must_use]
pub fn add_suffix_if_not(input: &str, suffix: &str) -> String {
    if input.ends_with(suffix) {
        input.to_owned()
    } else {
        let mut output = String::with_capacity(input.len() + suffix.len());
        output.push_str(input);
        output.push_str(suffix);
        output
    }
}
