/// Escapes HTML-sensitive characters.
///
/// Escaped characters are `&`, `<`, `>`, `"`, and `'`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::escape_html("<a href='x'>"), "&lt;a href=&#39;x&#39;&gt;");
/// ```
#[must_use]
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(ch),
        }
    }
    output
}

/// Unescapes common HTML entities.
///
/// Supported entities are `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&#39;`, and
/// `&#x27;`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unescape_html("&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;"), "<b>Tom&Jerry</b>");
/// ```
#[must_use]
pub fn unescape_html(input: &str) -> String {
    input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&amp;", "&")
}

/// Escapes non-ASCII characters as Java-style `\uXXXX` sequences.
///
/// Characters above `U+FFFF` are encoded as UTF-16 surrogate pairs.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::escape_unicode("Rust你好"), "Rust\\u4F60\\u597D");
/// assert_eq!(vstr::escape_unicode("🚀"), "\\uD83D\\uDE80");
/// ```
#[must_use]
pub fn escape_unicode(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii() {
            output.push(ch);
        } else {
            let mut encoded = [0_u16; 2];
            for unit in ch.encode_utf16(&mut encoded) {
                push_unicode_escape(&mut output, *unit);
            }
        }
    }
    output
}

/// Decodes Java-style `\uXXXX` sequences.
///
/// Malformed escapes and isolated surrogate code units are preserved verbatim
/// because Rust strings cannot contain invalid Unicode scalar values.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unescape_unicode("Rust\\u4F60\\u597D"), "Rust你好");
/// assert_eq!(vstr::unescape_unicode("\\uD83D\\uDE80"), "🚀");
/// ```
#[must_use]
pub fn unescape_unicode(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut index = 0;

    while index < input.len() {
        let Some((unit, next_index)) = parse_unicode_escape(input, index) else {
            if let Some(ch) = input[index..].chars().next() {
                output.push(ch);
                index += ch.len_utf8();
            } else {
                break;
            }
            continue;
        };

        if is_high_surrogate(unit) {
            if let Some((low, after_low_index)) = parse_unicode_escape(input, next_index) {
                if let Some(decoded) = decode_surrogate_pair(unit, low) {
                    output.push(decoded);
                    index = after_low_index;
                    continue;
                }
            }

            output.push_str(&input[index..next_index]);
            index = next_index;
            continue;
        }

        if is_low_surrogate(unit) {
            output.push_str(&input[index..next_index]);
        } else if let Some(ch) = char::from_u32(u32::from(unit)) {
            output.push(ch);
        } else {
            output.push_str(&input[index..next_index]);
        }
        index = next_index;
    }

    output
}

fn push_unicode_escape(output: &mut String, unit: u16) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    output.push('\\');
    output.push('u');
    output.push(char::from(HEX[usize::from((unit >> 12) & 0xF)]));
    output.push(char::from(HEX[usize::from((unit >> 8) & 0xF)]));
    output.push(char::from(HEX[usize::from((unit >> 4) & 0xF)]));
    output.push(char::from(HEX[usize::from(unit & 0xF)]));
}

fn parse_unicode_escape(input: &str, index: usize) -> Option<(u16, usize)> {
    let bytes = input.as_bytes();
    if index + 6 > bytes.len() || bytes[index] != b'\\' || bytes[index + 1] != b'u' {
        return None;
    }

    let mut value = 0_u16;
    for byte in &bytes[index + 2..index + 6] {
        value = value.checked_mul(16)?;
        value = value.checked_add(u16::from(hex_value(*byte)?))?;
    }
    Some((value, index + 6))
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

const fn is_high_surrogate(unit: u16) -> bool {
    unit >= 0xD800 && unit <= 0xDBFF
}

const fn is_low_surrogate(unit: u16) -> bool {
    unit >= 0xDC00 && unit <= 0xDFFF
}

fn decode_surrogate_pair(high: u16, low: u16) -> Option<char> {
    if !is_high_surrogate(high) || !is_low_surrogate(low) {
        return None;
    }

    let high = u32::from(high) - 0xD800;
    let low = u32::from(low) - 0xDC00;
    let code_point = 0x10000 + ((high << 10) | low);
    char::from_u32(code_point)
}
