use super::options::{LongWordPolicy, WhitespaceMode, WrapOptions};
use super::tokens::{collapse_wrap_tokens, preserve_wrap_tokens};

/// Wraps text with explicit scalar layout options.
///
/// This helper keeps [`super::wrap`] stable while exposing policy choices for
/// word separators, whitespace preservation, indentation, and long-word
/// handling.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions, wrap_with_options};
///
/// let options = WrapOptions::new(8)
///     .with_word_separators(&['/'])
///     .with_whitespace_mode(WhitespaceMode::Preserve)
///     .with_long_word_policy(LongWordPolicy::Preserve);
///
/// assert_eq!(wrap_with_options("api/v1  users", &options), "api/v1  \nusers");
/// ```
#[must_use]
pub fn wrap_with_options(input: &str, options: &WrapOptions<'_>) -> String {
    if options.width == 0 {
        return String::new();
    }

    let mut rendered = Vec::new();
    for paragraph in input.split('\n') {
        match options.whitespace_mode {
            WhitespaceMode::Collapse => {
                wrap_collapse_paragraph_with_options(paragraph, options, &mut rendered);
            }
            WhitespaceMode::Preserve => {
                wrap_preserve_paragraph_with_options(paragraph, options, &mut rendered);
            }
        }
    }
    rendered.join("\n")
}

fn wrap_collapse_paragraph_with_options(
    paragraph: &str,
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
    if paragraph.is_empty() {
        let mut line_index = 0;
        push_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let tokens = collapse_wrap_tokens(paragraph, options.word_separators);
    if tokens.is_empty() {
        let mut line_index = 0;
        push_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let mut line_index = 0usize;
    let mut current = String::new();
    let mut current_len = 0usize;

    for token in tokens {
        let token_len = token.text.chars().count();
        let active_width = options_content_width(options, line_index);
        let prefix_len = usize::from(!current.is_empty() && token.prefix_space);

        if token_len > active_width {
            if !current.is_empty() {
                push_options_line(rendered, options, &mut line_index, &current);
                current.clear();
                current_len = 0;
            }
            push_long_scalar_token(rendered, options, &mut line_index, token.text);
        } else if current_len + prefix_len + token_len <= active_width {
            if prefix_len == 1 {
                current.push(' ');
            }
            current.push_str(token.text);
            current_len += prefix_len + token_len;
        } else {
            push_options_line(rendered, options, &mut line_index, &current);
            current.clear();
            current.push_str(token.text);
            current_len = token_len;
        }
    }

    if !current.is_empty() {
        push_options_line(rendered, options, &mut line_index, &current);
    }
}

fn wrap_preserve_paragraph_with_options(
    paragraph: &str,
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
    if paragraph.is_empty() {
        let mut line_index = 0;
        push_options_line(rendered, options, &mut line_index, "");
        return;
    }

    let mut line_index = 0usize;
    let mut current = String::new();
    let mut current_len = 0usize;

    for token in preserve_wrap_tokens(paragraph, options.word_separators) {
        let token_len = token.chars().count();
        let active_width = options_content_width(options, line_index);

        if token_len > active_width {
            if !current.is_empty() {
                push_options_line(rendered, options, &mut line_index, &current);
                current.clear();
                current_len = 0;
            }
            push_long_scalar_token(rendered, options, &mut line_index, token);
        } else if current_len + token_len <= active_width || current.is_empty() {
            current.push_str(token);
            current_len += token_len;
        } else {
            push_options_line(rendered, options, &mut line_index, &current);
            current.clear();
            current.push_str(token);
            current_len = token_len;
        }
    }

    if !current.is_empty() {
        push_options_line(rendered, options, &mut line_index, &current);
    }
}

fn push_long_scalar_token(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    token: &str,
) {
    match options.long_word_policy {
        LongWordPolicy::Preserve => push_options_line(rendered, options, line_index, token),
        LongWordPolicy::Break => push_scalar_chunks(rendered, options, line_index, token),
    }
}

fn push_scalar_chunks(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    token: &str,
) {
    let mut current = String::new();
    let mut current_len = 0usize;

    for ch in token.chars() {
        let active_width = options_content_width(options, *line_index);
        if current_len == active_width {
            push_options_line(rendered, options, line_index, &current);
            current.clear();
            current_len = 0;
        }
        current.push(ch);
        current_len += 1;
    }

    if !current.is_empty() {
        push_options_line(rendered, options, line_index, &current);
    }
}

fn push_options_line(
    rendered: &mut Vec<String>,
    options: &WrapOptions<'_>,
    line_index: &mut usize,
    content: &str,
) {
    let indent = options_indent(options, *line_index);
    let mut output = String::with_capacity(indent.len() + content.len());
    output.push_str(indent);
    output.push_str(content);
    rendered.push(output);
    *line_index += 1;
}

fn options_content_width(options: &WrapOptions<'_>, line_index: usize) -> usize {
    options
        .width
        .saturating_sub(options_indent(options, line_index).chars().count())
        .max(1)
}

fn options_indent<'src>(options: &WrapOptions<'src>, line_index: usize) -> &'src str {
    if line_index == 0 {
        options.initial_indent
    } else {
        options.subsequent_indent
    }
}
