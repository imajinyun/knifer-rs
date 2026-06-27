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

/// Returns `true` when `ch` is a Unicode letter.
///
/// This follows Go's `unicode.IsLetter` shape for daily-use text helpers:
/// ordinary letters are accepted, while numeric letters such as Roman numerals
/// are not.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_letter('你'));
/// assert!(vstr::is_letter('A'));
/// assert!(!vstr::is_letter('Ⅷ'));
/// ```
#[must_use]
pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() && !ch.is_numeric()
}

/// Returns `true` when `ch` is a Unicode decimal digit.
///
/// This aligns with Go's `unicode.IsDigit`: decimal digits such as ASCII `9`
/// and Arabic-Indic `٣` are accepted, while numeric letters and fractions are
/// not.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_digit('9'));
/// assert!(vstr::is_digit('٣'));
/// assert!(!vstr::is_digit('Ⅷ'));
/// ```
#[must_use]
pub fn is_digit(ch: char) -> bool {
    matches!(
        ch,
        '0'..='9'
            | '\u{0660}'..='\u{0669}'
            | '\u{06F0}'..='\u{06F9}'
            | '\u{07C0}'..='\u{07C9}'
            | '\u{0966}'..='\u{096F}'
            | '\u{09E6}'..='\u{09EF}'
            | '\u{0A66}'..='\u{0A6F}'
            | '\u{0AE6}'..='\u{0AEF}'
            | '\u{0B66}'..='\u{0B6F}'
            | '\u{0BE6}'..='\u{0BEF}'
            | '\u{0C66}'..='\u{0C6F}'
            | '\u{0CE6}'..='\u{0CEF}'
            | '\u{0D66}'..='\u{0D6F}'
            | '\u{0DE6}'..='\u{0DEF}'
            | '\u{0E50}'..='\u{0E59}'
            | '\u{0ED0}'..='\u{0ED9}'
            | '\u{0F20}'..='\u{0F29}'
            | '\u{1040}'..='\u{1049}'
            | '\u{1090}'..='\u{1099}'
            | '\u{17E0}'..='\u{17E9}'
            | '\u{1810}'..='\u{1819}'
            | '\u{1946}'..='\u{194F}'
            | '\u{19D0}'..='\u{19D9}'
            | '\u{1A80}'..='\u{1A89}'
            | '\u{1A90}'..='\u{1A99}'
            | '\u{1B50}'..='\u{1B59}'
            | '\u{1BB0}'..='\u{1BB9}'
            | '\u{1C40}'..='\u{1C49}'
            | '\u{1C50}'..='\u{1C59}'
            | '\u{A620}'..='\u{A629}'
            | '\u{A8D0}'..='\u{A8D9}'
            | '\u{A900}'..='\u{A909}'
            | '\u{A9D0}'..='\u{A9D9}'
            | '\u{A9F0}'..='\u{A9F9}'
            | '\u{AA50}'..='\u{AA59}'
            | '\u{ABF0}'..='\u{ABF9}'
            | '\u{FF10}'..='\u{FF19}'
            | '\u{104A0}'..='\u{104A9}'
            | '\u{10D30}'..='\u{10D39}'
            | '\u{11066}'..='\u{1106F}'
            | '\u{110F0}'..='\u{110F9}'
            | '\u{11136}'..='\u{1113F}'
            | '\u{111D0}'..='\u{111D9}'
            | '\u{112F0}'..='\u{112F9}'
            | '\u{11450}'..='\u{11459}'
            | '\u{114D0}'..='\u{114D9}'
            | '\u{11650}'..='\u{11659}'
            | '\u{116C0}'..='\u{116C9}'
            | '\u{11730}'..='\u{11739}'
            | '\u{118E0}'..='\u{118E9}'
            | '\u{11950}'..='\u{11959}'
            | '\u{11C50}'..='\u{11C59}'
            | '\u{11D50}'..='\u{11D59}'
            | '\u{11DA0}'..='\u{11DA9}'
            | '\u{11F50}'..='\u{11F59}'
            | '\u{16A60}'..='\u{16A69}'
            | '\u{16AC0}'..='\u{16AC9}'
            | '\u{16B50}'..='\u{16B59}'
            | '\u{1D7CE}'..='\u{1D7FF}'
            | '\u{1E140}'..='\u{1E149}'
            | '\u{1E2F0}'..='\u{1E2F9}'
            | '\u{1E4F0}'..='\u{1E4F9}'
            | '\u{1E950}'..='\u{1E959}'
            | '\u{1FBF0}'..='\u{1FBF9}'
    )
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

/// Returns `true` when `ch` is a Unicode alphabetic character or decimal digit.
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
    is_letter(ch) || is_digit(ch)
}
