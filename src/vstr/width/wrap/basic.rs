use super::super::measure::{display_width, take_width};

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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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

pub(super) fn take_width_progress(input: &str, max_width: usize) -> &str {
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
