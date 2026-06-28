/// Collapses consecutive Unicode whitespace into a single ASCII space.
///
/// Leading and trailing whitespace are removed.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::normalize_whitespace("  hello\n\tRust  "), "hello Rust");
/// ```
#[must_use]
pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Removes all Unicode whitespace from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_whitespace(" a\n b\t "), "ab");
/// ```
#[must_use]
pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|ch| !ch.is_whitespace()).collect()
}

/// Normalizes CRLF and CR line endings to LF.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::normalize_newlines("a\r\nb\rc"), "a\nb\nc");
/// ```
#[must_use]
pub fn normalize_newlines(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\r' {
            if chars.peek() == Some(&'\n') {
                let _ = chars.next();
            }
            output.push('\n');
        } else {
            output.push(ch);
        }
    }

    output
}

/// Trims Unicode whitespace from both ends of every line.
///
/// Line separators are preserved, including a final trailing newline.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_lines("  a  \n\tb\t\n"), "a\nb\n");
/// ```
#[must_use]
pub fn trim_lines(input: &str) -> String {
    input
        .split('\n')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Removes leading and trailing blank lines while borrowing from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_blank_lines("\n  \nhello\n\n"), "hello");
/// ```
#[must_use]
pub fn trim_blank_lines(input: &str) -> &str {
    let ranges = line_ranges(input);
    let Some(first) = ranges.iter().position(|(_, _, is_blank)| !is_blank) else {
        return "";
    };
    let last = ranges
        .iter()
        .rposition(|(_, _, is_blank)| !is_blank)
        .unwrap_or(first);

    &input[ranges[first].0..ranges[last].1]
}

fn line_ranges(input: &str) -> Vec<(usize, usize, bool)> {
    let mut ranges = Vec::new();
    let mut offset = 0;

    for line in input.split_inclusive('\n') {
        let start = offset;
        offset += line.len();

        let without_lf = line.strip_suffix('\n').unwrap_or(line);
        let content = without_lf.strip_suffix('\r').unwrap_or(without_lf);
        let end = start + content.len();
        ranges.push((start, end, content.trim().is_empty()));
    }

    ranges
}
