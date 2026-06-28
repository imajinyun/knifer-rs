/// Collapses consecutive occurrences of `ch` into one occurrence.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::collapse_repeated_char("a---b----c", '-'), "a-b-c");
/// ```
#[must_use]
pub fn collapse_repeated_char(input: &str, ch: char) -> String {
    let mut output = String::with_capacity(input.len());
    let mut previous_was_target = false;

    for current in input.chars() {
        if current == ch {
            if !previous_was_target {
                output.push(current);
            }
            previous_was_target = true;
        } else {
            output.push(current);
            previous_was_target = false;
        }
    }

    output
}

/// Converts text into a dependency-free Unicode slug separated by `-`.
///
/// Letters and decimal digits are lower-cased and preserved. Other runs of
/// characters become one separator. This function intentionally does not
/// transliterate non-ASCII text.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::slugify("Hello, Rust World!"), "hello-rust-world");
/// assert_eq!(vstr::slugify("你好 Rust"), "你好-rust");
/// ```
#[must_use]
pub fn slugify(input: &str) -> String {
    slugify_with_separator(input, '-')
}

/// Converts text into a dependency-free Unicode slug with a custom separator.
///
/// If `separator` is alphanumeric or whitespace, `-` is used instead.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::slugify_with_separator("Hello, Rust World!", '_'), "hello_rust_world");
/// ```
#[must_use]
pub fn slugify_with_separator(input: &str, separator: char) -> String {
    let separator = if separator.is_alphanumeric() || separator.is_whitespace() {
        '-'
    } else {
        separator
    };

    let mut output = String::with_capacity(input.len());
    let mut pending_separator = false;

    for ch in input.chars() {
        if super::super::is_letter_or_digit(ch) {
            if pending_separator && !output.is_empty() {
                output.push(separator);
            }
            output.extend(ch.to_lowercase());
            pending_separator = false;
        } else if !output.is_empty() {
            pending_separator = true;
        }
    }

    output
}

/// Prefixes every line with `prefix`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::indent("a\nb", "  "), "  a\n  b");
/// ```
#[must_use]
pub fn indent(input: &str, prefix: &str) -> String {
    input
        .split('\n')
        .map(|line| {
            let mut output = String::with_capacity(prefix.len() + line.len());
            output.push_str(prefix);
            output.push_str(line);
            output
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Centers `input` to `width` Unicode scalar values using `pad`.
///
/// When an odd number of padding characters is required, the extra padding is
/// added to the right side.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::center("rust", 8, '-'), "--rust--");
/// assert_eq!(vstr::center("rust", 9, '-'), "--rust---");
/// ```
#[must_use]
pub fn center(input: &str, width: usize, pad: char) -> String {
    let len = input.chars().count();
    if len >= width {
        return input.to_owned();
    }

    let padding = width - len;
    let left = padding / 2;
    let right = padding - left;
    let mut output = String::with_capacity(input.len() + padding * pad.len_utf8());
    output.extend(std::iter::repeat_n(pad, left));
    output.push_str(input);
    output.extend(std::iter::repeat_n(pad, right));
    output
}

/// Removes the smallest common leading ASCII-space indentation from non-blank lines.
///
/// Blank lines are preserved as empty lines in the result.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::dedent("    a\n      b"), "a\n  b");
/// ```
#[must_use]
pub fn dedent(input: &str) -> String {
    let min_indent = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(leading_ascii_spaces)
        .min()
        .unwrap_or(0);

    if min_indent == 0 {
        return input.to_owned();
    }

    input
        .split('\n')
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else {
                &line[min_indent.min(line.len())..]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Surrounds `input` with `left` and `right`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::surround("value", "[", "]"), "[value]");
/// ```
#[must_use]
pub fn surround(input: &str, left: &str, right: &str) -> String {
    let mut output = String::with_capacity(left.len() + input.len() + right.len());
    output.push_str(left);
    output.push_str(input);
    output.push_str(right);
    output
}

/// Removes surrounding `left` and `right` markers when both are present.
///
/// The returned slice borrows from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::unsurround("[value]", "[", "]"), Some("value"));
/// assert_eq!(vstr::unsurround("value]", "[", "]"), None);
/// ```
#[must_use]
pub fn unsurround<'src>(input: &'src str, left: &str, right: &str) -> Option<&'src str> {
    if left.is_empty() && right.is_empty() {
        return Some(input);
    }
    let inner = input.strip_prefix(left)?;
    inner.strip_suffix(right)
}

fn leading_ascii_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}
