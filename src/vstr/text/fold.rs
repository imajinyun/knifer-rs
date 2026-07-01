//! Dependency-free ASCII folding for common Latin diacritics and ligatures.
//!
//! This mirrors the practical scope of lodash `deburr`: it maps precomposed
//! Latin-1 Supplement and Latin Extended-A letters to their closest ASCII
//! spelling and removes combining diacritical marks. It is intentionally **not**
//! a full transliteration engine like `unidecode`; non-Latin scripts (CJK,
//! Cyrillic, Greek, emoji, and so on) are preserved unchanged.

/// Maps a single Latin letter to its ASCII fold, if one is defined.
///
/// Returns `None` for characters that are not part of the folding table, so the
/// caller can decide whether to keep or drop them.
fn deburr_letter(ch: char) -> Option<&'static str> {
    // Each ASCII output appears in exactly one arm (Latin-1 Supplement and
    // Latin Extended-A merged) so folding stays a single lookup per scalar.
    let mapped = match ch {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' | 'Ą' => "A",
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => "a",
        'Ç' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' => "C",
        'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => "c",
        'Ð' | 'Ď' | 'Đ' => "D",
        'ð' | 'ď' | 'đ' => "d",
        'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => "E",
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => "e",
        'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => "G",
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => "g",
        'Ĥ' | 'Ħ' => "H",
        'ĥ' | 'ħ' => "h",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' => "I",
        'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => "i",
        'Ĵ' => "J",
        'ĵ' => "j",
        'Ķ' => "K",
        'ķ' | 'ĸ' => "k",
        'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => "L",
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => "l",
        'Ñ' | 'Ń' | 'Ņ' | 'Ň' | 'Ŋ' => "N",
        'ñ' | 'ń' | 'ņ' | 'ň' | 'ŋ' => "n",
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ŏ' | 'Ő' => "O",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => "o",
        'Ŕ' | 'Ŗ' | 'Ř' => "R",
        'ŕ' | 'ŗ' | 'ř' => "r",
        'Ś' | 'Ŝ' | 'Ş' | 'Š' => "S",
        'ś' | 'ŝ' | 'ş' | 'š' | 'ſ' => "s",
        'Ţ' | 'Ť' | 'Ŧ' => "T",
        'ţ' | 'ť' | 'ŧ' => "t",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => "U",
        'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => "u",
        'Ŵ' => "W",
        'ŵ' => "w",
        'Ý' | 'Ŷ' | 'Ÿ' => "Y",
        'ý' | 'ÿ' | 'ŷ' => "y",
        'Ź' | 'Ż' | 'Ž' => "Z",
        'ź' | 'ż' | 'ž' => "z",
        'Æ' => "Ae",
        'æ' => "ae",
        'Þ' => "Th",
        'þ' => "th",
        'ß' => "ss",
        'Ĳ' => "IJ",
        'ĳ' => "ij",
        'Œ' => "Oe",
        'œ' => "oe",
        'ŉ' => "'n",
        _ => return None,
    };
    Some(mapped)
}

/// Returns `true` when `ch` is a combining diacritical mark that folding drops.
fn is_combining_mark(ch: char) -> bool {
    ('\u{0300}'..='\u{036f}').contains(&ch) || ('\u{fe20}'..='\u{fe2f}').contains(&ch)
}

/// Converts common Latin diacritics and ligatures to their ASCII spelling.
///
/// Precomposed Latin letters are mapped (for example `é` becomes `e` and `Æ`
/// becomes `Ae`) and standalone combining marks are removed. Characters outside
/// the Latin folding table, including CJK, Greek, Cyrillic, digits, and emoji,
/// are preserved unchanged. This is a daily-business fold, not a full
/// transliteration of every script.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::deburr("déjà vu"), "deja vu");
/// assert_eq!(vstr::deburr("Æther Œuvre ß"), "Aether Oeuvre ss");
/// assert_eq!(vstr::deburr("e\u{0301}"), "e");
/// assert_eq!(vstr::deburr("你好 café"), "你好 cafe");
/// ```
#[must_use]
pub fn deburr(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        if let Some(mapped) = deburr_letter(ch) {
            output.push_str(mapped);
        } else if is_combining_mark(ch) {
            // Drop standalone combining marks so decomposed accents disappear.
        } else {
            output.push(ch);
        }
    }
    output
}

/// Removes Latin accents and diacritics, keeping the base letters.
///
/// This is an alias of [`deburr`] provided for discoverability; it shares the
/// same daily-business folding scope, so ligatures such as `Æ` still expand to
/// their ASCII spelling.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_accents("Crème Brûlée"), "Creme Brulee");
/// ```
#[must_use]
pub fn remove_accents(input: &str) -> String {
    deburr(input)
}
