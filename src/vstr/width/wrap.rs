use super::measure::{display_width, take_width};
use crate::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions};

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

/// Wraps text with explicit display-width layout options.
///
/// This helper mirrors [`crate::vstr::wrap_with_options`] but measures line
/// budgets with terminal display width.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-width")]
/// # {
/// use knifer_rs::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions, wrap_width_with_options};
///
/// let options = WrapOptions::new(6)
///     .with_word_separators(&['/'])
///     .with_whitespace_mode(WhitespaceMode::Preserve)
///     .with_long_word_policy(LongWordPolicy::Preserve);
///
/// assert_eq!(wrap_width_with_options("路径/api  用户", &options), "路径/\napi  \n用户");
/// # }
/// ```
#[must_use]
pub fn wrap_width_with_options(input: &str, options: &WrapOptions<'_>) -> String {
    if options.width == 0 {
        return String::new();
    }

    let mut rendered = Vec::new();
    for paragraph in input.split('\n') {
        match options.whitespace_mode {
            WhitespaceMode::Collapse => {
                wrap_width_collapse_paragraph_with_options(paragraph, options, &mut rendered);
            }
            WhitespaceMode::Preserve => {
                wrap_width_preserve_paragraph_with_options(paragraph, options, &mut rendered);
            }
        }
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

#[derive(Clone, Copy)]
struct WidthCollapseWrapToken<'src> {
    text: &'src str,
    prefix_space: bool,
}

fn wrap_width_collapse_paragraph_with_options(
    paragraph: &str,
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
    if paragraph.is_empty() {
        let mut line_index = 0;
        push_width_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let tokens = width_collapse_wrap_tokens(paragraph, options.word_separators);
    if tokens.is_empty() {
        let mut line_index = 0;
        push_width_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let mut line_index = 0usize;
    let mut current = String::new();
    let mut current_width = 0usize;

    for token in tokens {
        let token_width = display_width(token.text);
        let active_width = width_options_content_width(options, line_index);
        let prefix_width = usize::from(!current.is_empty() && token.prefix_space);

        if token_width > active_width {
            if !current.is_empty() {
                push_width_options_line(rendered, options, &mut line_index, &current);
                current.clear();
                current_width = 0;
            }
            push_long_width_token(rendered, options, &mut line_index, token.text);
        } else if current_width + prefix_width + token_width <= active_width {
            if prefix_width == 1 {
                current.push(' ');
            }
            current.push_str(token.text);
            current_width += prefix_width + token_width;
        } else {
            push_width_options_line(rendered, options, &mut line_index, &current);
            current.clear();
            current.push_str(token.text);
            current_width = token_width;
        }
    }

    if !current.is_empty() {
        push_width_options_line(rendered, options, &mut line_index, &current);
    }
}

fn wrap_width_preserve_paragraph_with_options(
    paragraph: &str,
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
    if paragraph.is_empty() {
        let mut line_index = 0;
        push_width_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let mut line_index = 0usize;
    let mut current = String::new();
    let mut current_width = 0usize;

    for token in width_preserve_wrap_tokens(paragraph, options.word_separators) {
        let token_width = display_width(token);
        let active_width = width_options_content_width(options, line_index);

        if token_width > active_width {
            if !current.is_empty() {
                push_width_options_line(rendered, options, &mut line_index, &current);
                current.clear();
                current_width = 0;
            }
            push_long_width_token(rendered, options, &mut line_index, token);
        } else if current_width + token_width <= active_width || current.is_empty() {
            current.push_str(token);
            current_width += token_width;
        } else {
            push_width_options_line(rendered, options, &mut line_index, &current);
            current.clear();
            current.push_str(token);
            current_width = token_width;
        }
    }

    if !current.is_empty() {
        push_width_options_line(rendered, options, &mut line_index, &current);
    }
}

fn width_collapse_wrap_tokens<'src>(
    paragraph: &'src str,
    word_separators: &[char],
) -> Vec<WidthCollapseWrapToken<'src>> {
    let mut tokens = Vec::new();
    for (word_index, word) in paragraph.split_whitespace().enumerate() {
        push_width_collapse_word_segments(word, word_index > 0, word_separators, &mut tokens);
    }
    tokens
}

fn push_width_collapse_word_segments<'src>(
    word: &'src str,
    prefix_space: bool,
    word_separators: &[char],
    tokens: &mut Vec<WidthCollapseWrapToken<'src>>,
) {
    let mut start = 0usize;
    let mut segment_prefix_space = prefix_space;
    for (index, ch) in word.char_indices() {
        if word_separators.contains(&ch) {
            let end = index + ch.len_utf8();
            tokens.push(WidthCollapseWrapToken {
                text: &word[start..end],
                prefix_space: segment_prefix_space,
            });
            start = end;
            segment_prefix_space = false;
        }
    }
    if start < word.len() {
        tokens.push(WidthCollapseWrapToken {
            text: &word[start..],
            prefix_space: segment_prefix_space,
        });
    }
}

fn width_preserve_wrap_tokens<'src>(
    paragraph: &'src str,
    word_separators: &[char],
) -> Vec<&'src str> {
    let mut tokens = Vec::new();
    let mut start = 0usize;
    let mut current_is_whitespace = None;

    for (index, ch) in paragraph.char_indices() {
        let is_whitespace = ch.is_whitespace();
        match current_is_whitespace {
            None => current_is_whitespace = Some(is_whitespace),
            Some(previous) if previous != is_whitespace => {
                tokens.push(&paragraph[start..index]);
                start = index;
                current_is_whitespace = Some(is_whitespace);
            }
            Some(_) => {}
        }

        if !is_whitespace && word_separators.contains(&ch) {
            let end = index + ch.len_utf8();
            tokens.push(&paragraph[start..end]);
            start = end;
            current_is_whitespace = None;
        }
    }

    if start < paragraph.len() {
        tokens.push(&paragraph[start..]);
    }

    tokens
}

fn push_long_width_token(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    token: &str,
) {
    match options.long_word_policy {
        LongWordPolicy::Preserve => push_width_options_line(rendered, options, line_index, token),
        LongWordPolicy::Break => push_width_chunks(rendered, options, line_index, token),
    }
}

fn push_width_chunks(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    token: &str,
) {
    let mut remaining = token;
    while !remaining.is_empty() {
        let active_width = width_options_content_width(options, *line_index);
        let chunk = take_width_progress(remaining, active_width);
        push_width_options_line(rendered, options, line_index, chunk);
        remaining = &remaining[chunk.len()..];
    }
}

fn push_width_options_line(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    content: &str,
) {
    let indent = width_options_indent(options, *line_index);
    let mut output = String::with_capacity(indent.len() + content.len());
    output.push_str(indent);
    output.push_str(content);
    rendered.push(output);
    *line_index += 1;
}

fn width_options_content_width(options: &WrapOptions<'_>, line_index: usize) -> usize {
    options
        .width
        .saturating_sub(display_width(width_options_indent(options, line_index)))
        .max(1)
}

fn width_options_indent<'src>(options: &WrapOptions<'src>, line_index: usize) -> &'src str {
    if line_index == 0 {
        options.initial_indent
    } else {
        options.subsequent_indent
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
