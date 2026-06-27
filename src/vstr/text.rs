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

/// Abbreviates text by keeping both ends and inserting `marker` in the middle.
///
/// The returned string never exceeds `max_chars` Unicode scalar values unless
/// `marker` itself is longer than the budget, in which case the marker is
/// truncated.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::abbreviate_middle("abcdefghijklmnopqrstuvwxyz", 10, "..."), "abcd...xyz");
/// assert_eq!(vstr::abbreviate_middle("short", 10, "..."), "short");
/// ```
#[must_use]
pub fn abbreviate_middle(input: &str, max_chars: usize, marker: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if input.chars().count() <= max_chars {
        return input.to_owned();
    }

    let marker_len = marker.chars().count();
    if marker_len >= max_chars {
        return truncate(marker, max_chars).to_owned();
    }

    let keep = max_chars - marker_len;
    let front_chars = keep.div_ceil(2);
    let back_chars = keep / 2;
    let front = take_chars(input, front_chars);
    let back = take_last_chars(input, back_chars);

    let mut output = String::with_capacity(front.len() + marker.len() + back.len());
    output.push_str(front);
    output.push_str(marker);
    output.push_str(back);
    output
}

/// Limits text to at most `max_words` Unicode-whitespace separated words.
///
/// A suffix is appended only when words were omitted.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::limit_words("hello rust utility toolkit", 2, "..."), "hello rust...");
/// assert_eq!(vstr::limit_words("hello rust", 3, "..."), "hello rust");
/// ```
#[must_use]
pub fn limit_words(input: &str, max_words: usize, suffix: &str) -> String {
    if max_words == 0 {
        return String::new();
    }

    let mut parts = input.split_whitespace();
    let selected: Vec<&str> = parts.by_ref().take(max_words).collect();
    if selected.is_empty() {
        return String::new();
    }

    let mut output = selected.join(" ");
    if parts.next().is_some() {
        output.push_str(suffix);
    }
    output
}

/// Returns a short excerpt around the first occurrence of `needle`.
///
/// The excerpt is at most `max_chars` Unicode scalar values when possible. If
/// `needle` is empty or missing, this falls back to [`truncate_with_suffix`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::excerpt("hello rust utility toolkit", "utility", 14, "..."), "...st utility too...");
/// assert_eq!(vstr::excerpt("hello rust", "go", 8, "..."), "hello...");
/// ```
#[must_use]
pub fn excerpt(input: &str, needle: &str, max_chars: usize, marker: &str) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let Some(byte_index) = input.find(needle).filter(|_| !needle.is_empty()) else {
        return truncate_with_suffix(input, max_chars, marker);
    };

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let start_char = chars
        .iter()
        .position(|(index, _)| *index == byte_index)
        .unwrap_or(0);
    let needle_chars = needle.chars().count();
    let input_chars = chars.len();

    if input_chars <= max_chars {
        return input.to_owned();
    }

    let context_budget = max_chars.saturating_sub(needle_chars).max(1);
    let before = context_budget / 2;
    let mut start = start_char.saturating_sub(before);
    let mut end = (start + max_chars).min(input_chars);
    if end < start_char + needle_chars {
        end = (start_char + needle_chars).min(input_chars);
        start = end.saturating_sub(max_chars);
    }

    let mut output = String::new();
    if start > 0 {
        output.push_str(marker);
    }
    output.push_str(slice_chars(input, start, end));
    if end < input_chars {
        output.push_str(marker);
    }
    output
}

/// Masks the middle of `input`, preserving `visible_start` and `visible_end` characters.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::mask("13800138000", 3, 4, '*'), "138****8000");
/// assert_eq!(vstr::mask("short", 10, 10, '*'), "short");
/// ```
#[must_use]
pub fn mask(input: &str, visible_start: usize, visible_end: usize, mask: char) -> String {
    let len = input.chars().count();
    if visible_start + visible_end >= len {
        return input.to_owned();
    }

    let start = take_chars(input, visible_start);
    let end = take_last_chars(input, visible_end);
    let mask_count = len - visible_start - visible_end;
    let mut output = String::with_capacity(input.len());
    output.push_str(start);
    output.extend(std::iter::repeat_n(mask, mask_count));
    output.push_str(end);
    output
}

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

/// Wraps text and prefixes the first and following lines with separate indents.
///
/// `width` is the target total line width including indentation. If an indent
/// is wider than `width`, that line still keeps its indent and wraps content at
/// one scalar value per line.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::wrap_with_indent("hello rust world", 12, "* ", "  "),
///     "* hello rust\n  world"
/// );
/// ```
#[must_use]
pub fn wrap_with_indent(
    input: &str,
    width: usize,
    initial_indent: &str,
    subsequent_indent: &str,
) -> String {
    if width == 0 {
        return String::new();
    }

    let mut rendered = Vec::new();
    for paragraph in input.split('\n') {
        let content_width = width.saturating_sub(initial_indent.chars().count()).max(1);
        let mut paragraph_lines = Vec::new();
        wrap_paragraph(paragraph, content_width, &mut paragraph_lines);

        if paragraph_lines.is_empty() {
            rendered.push(initial_indent.to_owned());
            continue;
        }

        for (index, line) in paragraph_lines.iter().enumerate() {
            let indent = if index == 0 {
                initial_indent
            } else {
                subsequent_indent
            };
            let adjusted = if index == 0 {
                line.to_owned()
            } else {
                let content_width = width
                    .saturating_sub(subsequent_indent.chars().count())
                    .max(1);
                if line.chars().count() <= content_width {
                    line.to_owned()
                } else {
                    wrap(line, content_width)
                }
            };
            for subline in adjusted.split('\n') {
                let mut output = String::with_capacity(indent.len() + subline.len());
                output.push_str(indent);
                output.push_str(subline);
                rendered.push(output);
            }
        }
    }

    rendered.join("\n")
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

/// Returns Unicode-whitespace separated words.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::words("hello  Rust\n世界"), vec!["hello", "Rust", "世界"]);
/// ```
#[must_use]
pub fn words(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

/// Returns upper-cased initials from Unicode-whitespace separated words.
///
/// Empty words are ignored.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::initials("rust string toolkit"), "RST");
/// assert_eq!(vstr::initials("你好 rust"), "你R");
/// ```
#[must_use]
pub fn initials(input: &str) -> String {
    let mut output = String::new();
    for word in input.split_whitespace() {
        if let Some(ch) = word.chars().next() {
            output.extend(ch.to_uppercase());
        }
    }
    output
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

/// Returns `true` when `input` is a palindrome after light text cleanup.
///
/// Only Unicode letters and decimal digits participate in the comparison.
/// Whitespace and punctuation are ignored. Case comparison follows
/// `knifer-rs`'s simple case-insensitive scalar behavior.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_palindrome("A man, a plan, a canal: Panama"));
/// assert!(vstr::is_palindrome("上海自来水来自海上"));
/// assert!(!vstr::is_palindrome("knifer-rs"));
/// ```
#[must_use]
pub fn is_palindrome(input: &str) -> bool {
    let chars: Vec<char> = input
        .chars()
        .filter(|ch| super::is_letter_or_digit(*ch))
        .collect();

    chars
        .iter()
        .zip(chars.iter().rev())
        .take(chars.len() / 2)
        .all(|(left, right)| super::equals_ignore_case(&left.to_string(), &right.to_string()))
}

/// Extracts all Unicode decimal digits from `input`.
///
/// This follows the same decimal-digit definition as [`crate::vstr::is_digit`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::extract_digits("id=42, رقم=٣"), "42٣");
/// ```
#[must_use]
pub fn extract_digits(input: &str) -> String {
    input.chars().filter(|ch| super::is_digit(*ch)).collect()
}

/// Removes ASCII punctuation from `input`.
///
/// Unicode punctuation is preserved so the helper stays small and predictable
/// without Unicode data dependencies.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_ascii_punctuation("Hello, Rust! 你好，世界！"), "Hello Rust 你好，世界！");
/// ```
#[must_use]
pub fn remove_ascii_punctuation(input: &str) -> String {
    input
        .chars()
        .filter(|ch| !ch.is_ascii_punctuation())
        .collect()
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

fn leading_ascii_spaces(line: &str) -> usize {
    line.bytes().take_while(|byte| *byte == b' ').count()
}

fn take_last_chars(input: &str, count: usize) -> &str {
    if count == 0 {
        return "";
    }

    let mut seen = 0;
    for (index, _) in input.char_indices().rev() {
        seen += 1;
        if seen == count {
            return &input[index..];
        }
    }
    input
}

fn slice_chars(input: &str, start: usize, end: usize) -> &str {
    if start >= end {
        return "";
    }

    let start_byte = input
        .char_indices()
        .nth(start)
        .map_or(input.len(), |(index, _)| index);
    let end_byte = input
        .char_indices()
        .nth(end)
        .map_or(input.len(), |(index, _)| index);
    &input[start_byte..end_byte]
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
