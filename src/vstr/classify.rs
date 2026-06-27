/// Returns `true` when `ch` is Unicode whitespace.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_blank_char(' '));
/// ```
#[must_use]
pub fn is_blank_char(ch: char) -> bool {
    ch.is_whitespace()
}

/// Returns `true` when `ch` is a Unicode alphabetic character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_letter('你'));
/// assert!(vstr::is_letter('A'));
/// ```
#[must_use]
pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic()
}

/// Returns `true` when `ch` is a Unicode numeric character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_digit('9'));
/// ```
#[must_use]
pub fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}

/// Returns `true` when `ch` is an ASCII character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_ascii('A'));
/// assert!(!vstr::is_ascii('你'));
/// ```
#[must_use]
pub const fn is_ascii(ch: char) -> bool {
    ch.is_ascii()
}

/// Returns `true` when `ch` is a Unicode alphabetic or numeric character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_letter_or_digit('A'));
/// assert!(vstr::is_letter_or_digit('9'));
/// ```
#[must_use]
pub fn is_letter_or_digit(ch: char) -> bool {
    ch.is_alphanumeric()
}
