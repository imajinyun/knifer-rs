/// Collapses consecutive occurrences of `ch` into one occurrence.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
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
/// Latin diacritics and ligatures are folded to ASCII first (for example `é`
/// becomes `e`), then letters and decimal digits are lower-cased and preserved.
/// Other runs of characters become one separator. Non-Latin scripts such as CJK
/// are preserved rather than transliterated.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::slugify("Hello, Rust World!"), "hello-rust-world");
/// assert_eq!(vstr::slugify("Crème Brûlée"), "creme-brulee");
/// assert_eq!(vstr::slugify("你好 Rust"), "你好-rust");
/// ```
#[must_use]
pub fn slugify(input: &str) -> String {
    slugify_with_separator(input, '-')
}

/// Converts text into a dependency-free Unicode slug with a custom separator.
///
/// If `separator` is alphanumeric or whitespace, `-` is used instead. Like
/// [`slugify`], Latin diacritics are folded to ASCII before slugging.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::slugify_with_separator("Hello, Rust World!", '_'), "hello_rust_world");
/// assert_eq!(vstr::slugify_with_separator("Déjà Vu", '_'), "deja_vu");
/// ```
#[must_use]
pub fn slugify_with_separator(input: &str, separator: char) -> String {
    let separator = if separator.is_alphanumeric() || separator.is_whitespace() {
        '-'
    } else {
        separator
    };

    let folded = super::deburr(input);
    let mut output = String::with_capacity(folded.len());
    let mut pending_separator = false;

    for ch in folded.chars() {
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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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

/// Wraps `input` with `marker` on both sides only where it is missing.
///
/// This mirrors Apache Commons `StringUtils.wrapIfMissing` for a symmetric
/// marker: the prefix marker is added only when absent and the suffix marker is
/// added only when absent, so an already-wrapped value is returned unchanged. An
/// empty input or empty marker is returned unchanged.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::wrap_if_missing("path", "/"), "/path/");
/// assert_eq!(vstr::wrap_if_missing("/path/", "/"), "/path/");
/// assert_eq!(vstr::wrap_if_missing("/path", "/"), "/path/");
/// assert_eq!(vstr::wrap_if_missing("/", "/"), "/");
/// ```
#[must_use]
pub fn wrap_if_missing(input: &str, marker: &str) -> String {
    if input.is_empty() || marker.is_empty() {
        return input.to_owned();
    }

    let wrap_start = !input.starts_with(marker);
    let wrap_end = !input.ends_with(marker);
    if !wrap_start && !wrap_end {
        return input.to_owned();
    }

    let extra = usize::from(wrap_start) + usize::from(wrap_end);
    let mut output = String::with_capacity(input.len() + marker.len() * extra);
    if wrap_start {
        output.push_str(marker);
    }
    output.push_str(input);
    if wrap_end {
        output.push_str(marker);
    }
    output
}

fn leading_ascii_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}
