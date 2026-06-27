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

/// Returns a borrowed prefix containing at most `max_chars` Unicode scalar values.
///
/// This never cuts through a UTF-8 character boundary.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::truncate("你好Rust", 3), "你好R");
/// ```
#[must_use]
pub fn truncate(input: &str, max_chars: usize) -> &str {
    if max_chars == 0 {
        return "";
    }

    match input.char_indices().nth(max_chars) {
        Some((index, _)) => &input[..index],
        None => input,
    }
}

/// Returns a borrowed prefix containing at most `count` Unicode scalar values.
///
/// This is a readability alias for [`truncate`] when the operation is about
/// selecting characters rather than shortening display text.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::take_chars("你好Rust", 3), "你好R");
/// ```
#[must_use]
pub fn take_chars(input: &str, count: usize) -> &str {
    truncate(input, count)
}

/// Drops at most `count` Unicode scalar values from the start of `input`.
///
/// This never cuts through a UTF-8 character boundary.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::drop_chars("你好Rust", 2), "Rust");
/// ```
#[must_use]
pub fn drop_chars(input: &str, count: usize) -> &str {
    if count == 0 {
        return input;
    }

    match input.char_indices().nth(count) {
        Some((index, _)) => &input[index..],
        None => "",
    }
}

/// Truncates text to at most `max_chars` Unicode scalar values and appends a suffix.
///
/// The returned string never exceeds `max_chars` scalar values. If `suffix` is
/// longer than the budget, the suffix itself is truncated.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::truncate_with_suffix("你好Rust", 5, "..."), "你好...");
/// assert_eq!(vstr::truncate_with_suffix("short", 10, "..."), "short");
/// ```
#[must_use]
pub fn truncate_with_suffix(input: &str, max_chars: usize, suffix: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if input.chars().count() <= max_chars {
        return input.to_owned();
    }

    let suffix_len = suffix.chars().count();
    if suffix_len >= max_chars {
        return truncate(suffix, max_chars).to_owned();
    }

    let keep_chars = max_chars - suffix_len;
    let mut output = String::from(truncate(input, keep_chars));
    output.push_str(suffix);
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
        if super::is_letter_or_digit(ch) {
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

/// Wraps text into lines of at most `width` Unicode scalar values when possible.
///
/// Wrapping happens at Unicode whitespace boundaries. Words longer than `width`
/// are split by scalar value so the function always makes progress.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::wrap("hello rust world", 10), "hello rust\nworld");
/// ```
#[must_use]
pub fn wrap(input: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let mut lines = Vec::new();
    for paragraph in input.split('\n') {
        wrap_paragraph(paragraph, width, &mut lines);
    }
    lines.join("\n")
}

/// Returns text lines using [`str::lines`] semantics.
///
/// Empty input has no lines and a trailing newline does not produce an extra
/// empty line.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::lines("a\nb\n"), vec!["a", "b"]);
/// ```
#[must_use]
pub fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Returns non-blank text lines after trimming Unicode whitespace.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::non_blank_lines(" a \n\n b "), vec!["a", "b"]);
/// ```
#[must_use]
pub fn non_blank_lines(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

/// Splits `input` into Unicode scalar values.
///
/// For grapheme-cluster segmentation, use a dedicated Unicode segmentation
/// crate. `knifer-rs` keeps this MVP helper dependency-free.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::chars("a你"), vec!['a', '你']);
/// ```
#[must_use]
pub fn chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

/// Counts Unicode-whitespace separated words.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::word_count("hello  Rust\n世界"), 3);
/// ```
#[must_use]
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

/// Counts text lines.
///
/// Empty input has zero lines. A trailing newline does not add an extra line,
/// matching [`str::lines`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::line_count("a\nb\n"), 2);
/// ```
#[must_use]
pub fn line_count(input: &str) -> usize {
    input.lines().count()
}

fn leading_ascii_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}

fn wrap_paragraph(paragraph: &str, width: usize, lines: &mut Vec<String>) {
    if paragraph.is_empty() {
        lines.push(String::new());
        return;
    }

    let mut current = String::new();
    let mut current_len = 0;

    for word in paragraph.split_whitespace() {
        let word_len = word.chars().count();
        if word_len > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current_len = 0;
            }
            push_wrapped_long_word(word, width, lines);
            continue;
        }

        if current.is_empty() {
            current.push_str(word);
            current_len = word_len;
        } else if current_len + 1 + word_len <= width {
            current.push(' ');
            current.push_str(word);
            current_len += 1 + word_len;
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
            current_len = word_len;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }
}

fn push_wrapped_long_word(word: &str, width: usize, lines: &mut Vec<String>) {
    let mut current = String::new();
    let mut current_len = 0;

    for ch in word.chars() {
        if current_len == width {
            lines.push(std::mem::take(&mut current));
            current_len = 0;
        }
        current.push(ch);
        current_len += 1;
    }

    if !current.is_empty() {
        lines.push(current);
    }
}
