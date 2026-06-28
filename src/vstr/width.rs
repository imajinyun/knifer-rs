use unicode_width::UnicodeWidthStr;

/// Returns the terminal display width of `input`.
///
/// This helper is available only with the `unicode-width` feature. It follows
/// the width rules provided by the `unicode-width` crate and is separate from
/// scalar-count helpers such as [`crate::vstr::char_len`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::display_width("abc你好"), 7);
/// # }
/// ```
#[must_use]
pub fn display_width(input: &str) -> usize {
    UnicodeWidthStr::width(input)
}

/// Returns a borrowed prefix whose terminal display width is at most `max_width`.
///
/// The returned slice never cuts through a UTF-8 character boundary. Prefixes
/// are measured as complete strings so emoji ZWJ sequences can follow the same
/// width rules as [`display_width`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::take_width("a你好", 3), "a你");
/// # }
/// ```
#[must_use]
pub fn take_width(input: &str, max_width: usize) -> &str {
    let mut end = 0usize;
    for (index, ch) in input.char_indices() {
        let next = index + ch.len_utf8();
        if display_width(&input[..next]) <= max_width {
            end = next;
        }
    }
    &input[..end]
}

/// Truncates text to at most `max_width` terminal display cells and appends a suffix.
///
/// If the suffix is wider than the budget, the suffix itself is truncated by
/// display width.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::truncate_width("你好Rust", 6, "..."), "你...");
/// assert_eq!(vstr::truncate_width("short", 10, "..."), "short");
/// # }
/// ```
#[must_use]
pub fn truncate_width(input: &str, max_width: usize, suffix: &str) -> String {
    if max_width == 0 {
        return String::new();
    }
    if display_width(input) <= max_width {
        return input.to_owned();
    }

    let suffix_width = display_width(suffix);
    if suffix_width >= max_width {
        return take_width(suffix, max_width).to_owned();
    }

    let keep_width = max_width - suffix_width;
    let mut output = String::from(take_width(input, keep_width));
    output.push_str(suffix);
    output
}

/// Wraps text into lines of at most `width` terminal display cells when possible.
///
/// Wrapping happens at Unicode whitespace boundaries. Words longer than
/// `width` are split by display width so the function always makes progress.
/// If one scalar value is wider than `width`, that scalar is emitted on its own
/// line.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::wrap_width("你好Rust world", 8), "你好Rust\nworld");
/// # }
/// ```
#[must_use]
pub fn wrap_width(input: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let mut lines = Vec::new();
    for paragraph in input.split('\n') {
        wrap_width_paragraph(paragraph, width, &mut lines);
    }
    lines.join("\n")
}

/// Wraps text by terminal display width and prefixes lines with indents.
///
/// `width` is the target total display width including indentation. If an
/// indent is equal to or wider than `width`, content still progresses at one
/// display-width chunk per line.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::wrap_width_with_indent("你好Rust world", 10, "* ", "  "),
///     "* 你好Rust\n  world"
/// );
/// # }
/// ```
#[must_use]
pub fn wrap_width_with_indent(
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
        wrap_width_paragraph_with_indent(
            paragraph,
            width,
            initial_indent,
            subsequent_indent,
            &mut rendered,
        );
    }
    rendered.join("\n")
}

fn wrap_width_paragraph(paragraph: &str, width: usize, lines: &mut Vec<String>) {
    if paragraph.is_empty() {
        lines.push(String::new());
        return;
    }

    let mut current = String::new();
    let mut current_width = 0usize;

    for word in paragraph.split_whitespace() {
        let word_width = display_width(word);
        if word_width > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current_width = 0;
            }
            push_wrapped_width_word(word, width, lines);
            continue;
        }

        if current.is_empty() {
            current.push_str(word);
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current.push(' ');
            current.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
            current_width = word_width;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }
}

fn wrap_width_paragraph_with_indent(
    paragraph: &str,
    width: usize,
    initial_indent: &str,
    subsequent_indent: &str,
    rendered: &mut Vec<String>,
) {
    let initial_width = content_width_after_indent(width, initial_indent);
    let mut paragraph_lines = Vec::new();
    wrap_width_paragraph(paragraph, initial_width, &mut paragraph_lines);

    if paragraph_lines.is_empty() {
        rendered.push(initial_indent.to_owned());
        return;
    }

    for (index, line) in paragraph_lines.iter().enumerate() {
        let indent = if index == 0 {
            initial_indent
        } else {
            subsequent_indent
        };
        let content_width = if index == 0 {
            initial_width
        } else {
            content_width_after_indent(width, subsequent_indent)
        };

        if index == 0 || display_width(line) <= content_width {
            push_indented_width_line(rendered, indent, line);
        } else {
            for subline in wrap_width(line, content_width).split('\n') {
                push_indented_width_line(rendered, indent, subline);
            }
        }
    }
}

fn push_wrapped_width_word(word: &str, width: usize, lines: &mut Vec<String>) {
    let mut remaining = word;
    while !remaining.is_empty() {
        let chunk = take_width_progress(remaining, width);
        lines.push(chunk.to_owned());
        remaining = &remaining[chunk.len()..];
    }
}

fn take_width_progress(input: &str, max_width: usize) -> &str {
    let prefix = take_width(input, max_width);
    if !prefix.is_empty() {
        return prefix;
    }

    input
        .char_indices()
        .nth(1)
        .map_or(input, |(index, _)| &input[..index])
}

fn content_width_after_indent(width: usize, indent: &str) -> usize {
    width.saturating_sub(display_width(indent)).max(1)
}

fn push_indented_width_line(rendered: &mut Vec<String>, indent: &str, line: &str) {
    let mut output = String::with_capacity(indent.len() + line.len());
    output.push_str(indent);
    output.push_str(line);
    rendered.push(output);
}
