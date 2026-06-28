/// Returns `input` without leading and trailing Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim("  hello  "), "hello");
/// ```
#[must_use]
pub fn trim(input: &str) -> &str {
    input.trim()
}

/// Returns `input` without leading Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_start("  hello  "), "hello  ");
/// ```
#[must_use]
pub fn trim_start(input: &str) -> &str {
    input.trim_start()
}

/// Returns `input` without trailing Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_end("  hello  "), "  hello");
/// ```
#[must_use]
pub fn trim_end(input: &str) -> &str {
    input.trim_end()
}

/// Returns an owned `String` without leading and trailing Unicode whitespace.
///
/// Prefer [`trim`] when a borrowed result is enough.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// let value = String::from("  hello  ");
/// assert_eq!(vstr::trim_to_string(&value), "hello");
/// ```
#[must_use]
pub fn trim_to_string(input: &str) -> String {
    trim(input).to_owned()
}

/// Returns `input` without leading and trailing Unicode whitespace.
///
/// This is an alias for [`trim`] to align with `knifer-go`'s `TrimToEmpty`
/// API name. Rust string slices are never null, so the function can borrow
/// directly from the original input.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_to_empty("  hello  "), "hello");
/// assert_eq!(vstr::trim_to_empty("   "), "");
/// ```
#[must_use]
pub fn trim_to_empty(input: &str) -> &str {
    trim(input)
}
