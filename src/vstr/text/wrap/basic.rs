/// Wraps text into lines of at most `width` Unicode scalar values when possible.
///
/// Wrapping happens at Unicode whitespace boundaries. Words longer than `width`
/// are split by scalar value so the function always makes progress. Consecutive
/// whitespace inside a paragraph is normalized to one ASCII space because words
/// are collected with [`str::split_whitespace`]. This helper does not measure
/// grapheme clusters or terminal display width.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
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
/// one scalar value per line. Content width is measured in Unicode scalar
/// values, not grapheme clusters or terminal display cells.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
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

pub(super) fn wrap_paragraph(paragraph: &str, width: usize, lines: &mut Vec<String>) {
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
