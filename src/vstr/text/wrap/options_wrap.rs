use super::options::{LongWordPolicy, WhitespaceMode, WrapAlgorithm, WrapOptions};
use super::tokens::{CollapseWrapToken, collapse_wrap_tokens, preserve_wrap_tokens};

/// Sentinel line-break cost meaning "unreachable" in the optimal-fit DP.
const OPTIMAL_FIT_INFINITY: u128 = u128::MAX;

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

    match options.wrap_algorithm {
        WrapAlgorithm::FirstFit => collapse_first_fit(&tokens, options, rendered),
        WrapAlgorithm::OptimalFit => collapse_optimal_fit(&tokens, options, rendered),
    }
}

fn collapse_first_fit(
    tokens: &[CollapseWrapToken<'_>],
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
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

/// Chooses line breaks that minimize the sum of squared trailing slack across
/// every line except the last, giving a more balanced paragraph than greedy
/// first-fit. Multi-token lines must fit the active width; an over-long token
/// always occupies its own line and follows the long-word policy at render time.
fn collapse_optimal_fit(
    tokens: &[CollapseWrapToken<'_>],
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
) {
    let token_count = tokens.len();
    let lengths: Vec<usize> = tokens
        .iter()
        .map(|token| token.text.chars().count())
        .collect();

    // The only line that starts at token 0 is the first rendered line, so a line
    // starting at token `i` uses the initial indent when `i == 0` and the
    // subsequent indent otherwise.
    let content_width = |start: usize| options_content_width(options, usize::from(start != 0));

    // cost[i] is the minimum total penalty for wrapping tokens[i..], and
    // next_break[i] is the exclusive end of the first line in that optimum.
    let mut cost = vec![OPTIMAL_FIT_INFINITY; token_count + 1];
    let mut next_break = vec![token_count; token_count + 1];
    cost[token_count] = 0;

    for start in (0..token_count).rev() {
        let available = content_width(start);
        let mut line_width = 0usize;
        for end in (start + 1)..=token_count {
            let index = end - 1;
            if index > start && tokens[index].prefix_space {
                line_width += 1;
            }
            line_width += lengths[index];

            let single = end == start + 1;
            if !single && line_width > available {
                // Multi-token lines must fit; a longer prefix cannot shrink.
                break;
            }
            if cost[end] == OPTIMAL_FIT_INFINITY {
                continue;
            }

            let is_last = end == token_count;
            let line_cost = if is_last {
                0
            } else {
                let slack = u128::from(
                    u64::try_from(available.saturating_sub(line_width)).unwrap_or(u64::MAX),
                );
                slack.saturating_mul(slack)
            };
            let total = line_cost.saturating_add(cost[end]);
            if total < cost[start] {
                cost[start] = total;
                next_break[start] = end;
            }
        }
    }

    let mut line_index = 0usize;
    let mut start = 0usize;
    while start < token_count {
        let end = next_break[start];
        render_optimal_line(
            tokens,
            &lengths,
            options,
            rendered,
            &mut line_index,
            start,
            end,
        );
        start = end;
    }
}

fn render_optimal_line(
    tokens: &[CollapseWrapToken<'_>],
    lengths: &[usize],
    options: &WrapOptions<'_>,
    rendered: &mut Vec<String>,
    line_index: &mut usize,
    start: usize,
    end: usize,
) {
    let active_width = options_content_width(options, *line_index);
    if end - start == 1 && lengths[start] > active_width {
        push_long_scalar_token(rendered, options, line_index, tokens[start].text);
        return;
    }

    let mut line = String::new();
    for (offset, token) in tokens[start..end].iter().enumerate() {
        if offset > 0 && token.prefix_space {
            line.push(' ');
        }
        line.push_str(token.text);
    }
    push_options_line(rendered, options, line_index, &line);
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
