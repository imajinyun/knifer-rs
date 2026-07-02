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

/// Unescapes HTML entities: named, decimal, and hexadecimal references.
///
/// Decoding is a single left-to-right pass, so each entity is expanded exactly
/// once (`&amp;lt;` becomes `&lt;`, not `<`). Three reference forms are
/// supported:
///
/// - Named references from a curated table (for example `&amp;`, `&lt;`,
///   `&nbsp;`, `&copy;`, `&mdash;`, `&hellip;`); the set is documented in the
///   `vstr` HTML notes rather than the full WHATWG list.
/// - Decimal numeric references `&#NNN;` (for example `&#39;`).
/// - Hexadecimal numeric references `&#xNN;` / `&#XNN;` (for example `&#x27;`).
///
/// Text that is not a recognized entity — including a bare `&`, an unknown name,
/// or a numeric reference that is not a valid Unicode scalar value (such as a
/// surrogate) — is preserved verbatim.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unescape_html("&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;"), "<b>Tom&Jerry</b>");
/// assert_eq!(vstr::unescape_html("caf&#233; &#x1F600;"), "café 😀");
/// assert_eq!(vstr::unescape_html("2&nbsp;&mdash;&nbsp;3"), "2\u{a0}—\u{a0}3");
/// assert_eq!(vstr::unescape_html("Tom & Jerry"), "Tom & Jerry");
/// ```
#[must_use]
pub fn unescape_html(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut index = 0;

    while index < input.len() {
        if bytes[index] == b'&' {
            if let Some((decoded, next_index)) = decode_html_entity(input, index) {
                output.push(decoded);
                index = next_index;
                continue;
            }
        }

        // `&` is ASCII, so any non-entity byte here begins a full character.
        let Some(ch) = input[index..].chars().next() else {
            break;
        };
        output.push(ch);
        index += ch.len_utf8();
    }

    output
}

/// Removes HTML/XML tags, keeping the text between them.
///
/// A tag is any `<...>` span. Quotes inside a tag are respected, so a `>` within
/// a `"`- or `'`-quoted attribute value does not close the tag, and `<!-- ... -->`
/// comments are dropped whole even when they contain `>`. A `<` with no closing
/// `>` (or an unterminated comment) is kept as literal text.
///
/// Like PHP's `strip_tags`, this treats every `<...>` span as a tag, so prose
/// such as `a < b > c` is affected. Entities are not decoded and whitespace is
/// not collapsed; pipe the result through [`unescape_html`] when needed.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::strip_tags("<b>Hello</b> <i>World</i>"), "Hello World");
/// assert_eq!(vstr::strip_tags(r#"<a title="x > y">link</a>"#), "link");
/// assert_eq!(vstr::strip_tags("keep <!-- x > y --> me"), "keep  me");
/// assert_eq!(vstr::strip_tags("1 < 2 and unclosed"), "1 < 2 and unclosed");
/// ```
#[must_use]
pub fn strip_tags(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut index = 0;

    while index < input.len() {
        if bytes[index] == b'<' {
            if let Some(next_index) = skip_html_tag(input, index) {
                index = next_index;
                continue;
            }

            output.push('<');
            index += 1;
            continue;
        }

        let Some(ch) = input[index..].chars().next() else {
            break;
        };
        output.push(ch);
        index += ch.len_utf8();
    }

    output
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

/// Decodes a single HTML entity starting at the `&` byte at `index`.
///
/// Returns the decoded scalar and the index just past the terminating `;` when
/// `input[index..]` begins with a recognized named, decimal, or hexadecimal
/// reference. Returns `None` for anything else so the caller can copy the `&`
/// literally.
fn decode_html_entity(input: &str, index: usize) -> Option<(char, usize)> {
    let rest = &input[index + 1..];
    let end = rest.find(';')?;
    let body = &rest[..end];
    let after = index + 1 + end + 1;

    if let Some(number) = body.strip_prefix('#') {
        let scalar = if let Some(hex) = number.strip_prefix(['x', 'X']) {
            parse_scalar(hex, 16)?
        } else {
            parse_scalar(number, 10)?
        };
        return Some((scalar, after));
    }

    named_entity(body).map(|decoded| (decoded, after))
}

/// Parses `digits` in `radix` into a Unicode scalar value.
///
/// Returns `None` for empty input, non-digit bytes, overflow, or code points
/// that are not valid Unicode scalar values (such as surrogates).
fn parse_scalar(digits: &str, radix: u32) -> Option<char> {
    if digits.is_empty() {
        return None;
    }

    let mut value = 0_u32;
    for byte in digits.bytes() {
        let digit = char::from(byte).to_digit(radix)?;
        value = value.checked_mul(radix)?;
        value = value.checked_add(digit)?;
    }
    char::from_u32(value)
}

/// Maps a curated set of named HTML entities to their scalar values.
///
/// This is a pragmatic business subset, not the full WHATWG named-character
/// table. Names are matched without the surrounding `&`/`;`.
fn named_entity(name: &str) -> Option<char> {
    let decoded = match name {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => '\u{a0}',
        "copy" => '©',
        "reg" => '®',
        "trade" => '™',
        "hellip" => '…',
        "mdash" => '—',
        "ndash" => '–',
        "lsquo" => '\u{2018}',
        "rsquo" => '\u{2019}',
        "ldquo" => '\u{201c}',
        "rdquo" => '\u{201d}',
        "laquo" => '«',
        "raquo" => '»',
        "middot" => '·',
        "bull" => '•',
        "deg" => '°',
        "plusmn" => '±',
        "times" => '×',
        "divide" => '÷',
        "frac12" => '½',
        "frac14" => '¼',
        "frac34" => '¾',
        "euro" => '€',
        "pound" => '£',
        "yen" => '¥',
        "cent" => '¢',
        "sect" => '§',
        "para" => '¶',
        "dagger" => '†',
        "Dagger" => '‡',
        _ => return None,
    };
    Some(decoded)
}

/// Returns the index just past a `<...>` tag starting at `index`.
///
/// Handles quoted attribute values and `<!-- ... -->` comments so a `>` inside
/// them does not close the tag early. Returns `None` when the tag (or comment)
/// is never terminated, so the caller keeps the `<` as literal text.
fn skip_html_tag(input: &str, index: usize) -> Option<usize> {
    let bytes = input.as_bytes();

    if input[index..].starts_with("<!--") {
        return input[index + 4..]
            .find("-->")
            .map(|offset| index + 4 + offset + 3);
    }

    let mut cursor = index + 1;
    let mut quote: Option<u8> = None;
    while cursor < bytes.len() {
        let byte = bytes[cursor];
        match quote {
            Some(active) => {
                if byte == active {
                    quote = None;
                }
            }
            None => match byte {
                b'"' | b'\'' => quote = Some(byte),
                b'>' => return Some(cursor + 1),
                _ => {}
            },
        }
        cursor += 1;
    }

    None
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
