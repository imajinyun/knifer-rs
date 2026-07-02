use super::super::measure::display_width;
use super::basic::take_width_progress;
use super::tokens::{width_collapse_wrap_tokens, width_preserve_wrap_tokens};
use crate::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions};

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
/// use kniferrs::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions, wrap_width_with_options};
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
