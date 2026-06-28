/// Controls how wrapping treats whitespace inside a paragraph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhitespaceMode {
    /// Collapse Unicode whitespace runs to one ASCII space between words.
    Collapse,
    /// Preserve whitespace runs as input text while still wrapping by budget.
    Preserve,
}

/// Controls how wrapping treats words that exceed the active line width.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LongWordPolicy {
    /// Split long words so wrapping always makes line-width progress.
    Break,
    /// Keep long words intact, even when a line exceeds the configured width.
    Preserve,
}

/// Options for strategy-based scalar wrapping.
///
/// Width is measured in Unicode scalar values. Display-width wrapping is
/// provided separately by `wrap_width_with_options` when the `unicode-width`
/// feature is enabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WrapOptions<'src> {
    /// Target total line width, including indentation.
    pub width: usize,
    /// Prefix for the first rendered line of each paragraph.
    pub initial_indent: &'src str,
    /// Prefix for following rendered lines of each paragraph.
    pub subsequent_indent: &'src str,
    /// Whitespace handling policy.
    pub whitespace_mode: WhitespaceMode,
    /// Long-word handling policy.
    pub long_word_policy: LongWordPolicy,
    /// Extra scalar separators where words may be wrapped without inserting a
    /// space. Separators are retained at the end of the preceding segment.
    pub word_separators: &'src [char],
}

impl<'src> WrapOptions<'src> {
    /// Creates wrapping options with scalar width and default wrapping behavior.
    ///
    /// Defaults match [`wrap`]: whitespace is collapsed and long words are split
    /// by scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::WrapOptions;
    ///
    /// let options = WrapOptions::new(8);
    /// assert_eq!(options.width, 8);
    /// ```
    #[must_use]
    pub const fn new(width: usize) -> Self {
        Self {
            width,
            initial_indent: "",
            subsequent_indent: "",
            whitespace_mode: WhitespaceMode::Collapse,
            long_word_policy: LongWordPolicy::Break,
            word_separators: &[],
        }
    }

    /// Sets separate indentation for first and following lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::WrapOptions;
    ///
    /// let options = WrapOptions::new(10).with_indent("> ", "  ");
    /// assert_eq!(options.initial_indent, "> ");
    /// ```
    #[must_use]
    pub const fn with_indent(
        mut self,
        initial_indent: &'src str,
        subsequent_indent: &'src str,
    ) -> Self {
        self.initial_indent = initial_indent;
        self.subsequent_indent = subsequent_indent;
        self
    }

    /// Sets the whitespace handling policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::{WhitespaceMode, WrapOptions};
    ///
    /// let options = WrapOptions::new(10).with_whitespace_mode(WhitespaceMode::Preserve);
    /// assert_eq!(options.whitespace_mode, WhitespaceMode::Preserve);
    /// ```
    #[must_use]
    pub const fn with_whitespace_mode(mut self, whitespace_mode: WhitespaceMode) -> Self {
        self.whitespace_mode = whitespace_mode;
        self
    }

    /// Sets the long-word handling policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::{LongWordPolicy, WrapOptions};
    ///
    /// let options = WrapOptions::new(10).with_long_word_policy(LongWordPolicy::Preserve);
    /// assert_eq!(options.long_word_policy, LongWordPolicy::Preserve);
    /// ```
    #[must_use]
    pub const fn with_long_word_policy(mut self, long_word_policy: LongWordPolicy) -> Self {
        self.long_word_policy = long_word_policy;
        self
    }

    /// Sets extra word separators where wrapping may break without adding
    /// spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::WrapOptions;
    ///
    /// let options = WrapOptions::new(8).with_word_separators(&['/', '-']);
    /// assert_eq!(options.word_separators, &['/', '-']);
    /// ```
    #[must_use]
    pub const fn with_word_separators(mut self, word_separators: &'src [char]) -> Self {
        self.word_separators = word_separators;
        self
    }
}

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

/// Wraps text with explicit scalar layout options.
///
/// This helper keeps [`wrap`] stable while exposing policy choices for word
/// separators, whitespace preservation, indentation, and long-word handling.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr::{LongWordPolicy, WhitespaceMode, WrapOptions, wrap_with_options};
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

#[derive(Clone, Copy)]
struct CollapseWrapToken<'src> {
    text: &'src str,
    prefix_space: bool,
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

fn collapse_wrap_tokens<'src>(
    paragraph: &'src str,
    word_separators: &[char],
) -> Vec<CollapseWrapToken<'src>> {
    let mut tokens = Vec::new();
    for (word_index, word) in paragraph.split_whitespace().enumerate() {
        push_collapse_word_segments(word, word_index > 0, word_separators, &mut tokens);
    }
    tokens
}

fn push_collapse_word_segments<'src>(
    word: &'src str,
    prefix_space: bool,
    word_separators: &[char],
    tokens: &mut Vec<CollapseWrapToken<'src>>,
) {
    let mut start = 0usize;
    let mut segment_prefix_space = prefix_space;
    for (index, ch) in word.char_indices() {
        if word_separators.contains(&ch) {
            let end = index + ch.len_utf8();
            tokens.push(CollapseWrapToken {
                text: &word[start..end],
                prefix_space: segment_prefix_space,
            });
            start = end;
            segment_prefix_space = false;
        }
    }
    if start < word.len() {
        tokens.push(CollapseWrapToken {
            text: &word[start..],
            prefix_space: segment_prefix_space,
        });
    }
}

fn preserve_wrap_tokens<'src>(paragraph: &'src str, word_separators: &[char]) -> Vec<&'src str> {
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
