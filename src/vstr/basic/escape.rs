/// Escapes Rust regex metacharacters so `input` can be used as a literal pattern.
///
/// This helper is dependency-free and follows the metacharacter set used by the
/// Rust `regex` crate for literal escaping.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
/// ```
#[must_use]
pub fn escape_regex(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        if is_regex_meta(ch) {
            output.push('\\');
        }
        output.push(ch);
    }
    output
}

/// Alias for [`escape_regex`] using a common regex-library name.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::quote_meta("[rust]"), r"\[rust\]");
/// ```
#[must_use]
pub fn quote_meta(input: &str) -> String {
    escape_regex(input)
}

fn is_regex_meta(ch: char) -> bool {
    matches!(
        ch,
        '\\' | '.'
            | '+'
            | '*'
            | '?'
            | '('
            | ')'
            | '|'
            | '['
            | ']'
            | '{'
            | '}'
            | '^'
            | '$'
            | '#'
            | '&'
            | '-'
            | '~'
    )
}
