/// Per-call customization for emoji helpers.
///
/// Use [`with_emoji_matcher`] and [`with_emoji_replacer`] to build options
/// without exposing the internal representation.
pub struct EmojiOptions<'src> {
    matcher: Option<Box<EmojiMatcher<'src>>>,
    replacer: Option<Box<EmojiReplacer<'src>>>,
}

type EmojiMatcher<'src> = dyn Fn(&str) -> bool + 'src;
type EmojiReplacer<'src> = dyn Fn(&str) -> String + 'src;

impl<'src> EmojiOptions<'src> {
    /// Creates empty emoji options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            matcher: None,
            replacer: None,
        }
    }

    /// Sets the matcher used by [`contains_emoji_with_options`].
    #[must_use]
    pub fn with_matcher(mut self, matcher: impl Fn(&str) -> bool + 'src) -> Self {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Sets the replacer used by [`remove_emoji_with_options`].
    #[must_use]
    pub fn with_replacer(mut self, replacer: impl Fn(&str) -> String + 'src) -> Self {
        self.replacer = Some(Box::new(replacer));
        self
    }
}

impl Default for EmojiOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates emoji options with a custom matcher.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// let opts = vstr::with_emoji_matcher(|input| input.contains(":rocket:"));
/// assert!(vstr::contains_emoji_with_options("ship :rocket:", &opts));
/// ```
#[must_use]
pub fn with_emoji_matcher<'src>(matcher: impl Fn(&str) -> bool + 'src) -> EmojiOptions<'src> {
    EmojiOptions::new().with_matcher(matcher)
}

/// Creates emoji options with a custom replacer.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// let opts = vstr::with_emoji_replacer(|input| input.replace(":rocket:", ""));
/// assert_eq!(vstr::remove_emoji_with_options("ship :rocket:", &opts), "ship ");
/// ```
#[must_use]
pub fn with_emoji_replacer<'src>(replacer: impl Fn(&str) -> String + 'src) -> EmojiOptions<'src> {
    EmojiOptions::new().with_replacer(replacer)
}

/// Returns `true` when `input` contains an emoji-like Unicode scalar value.
///
/// This helper intentionally uses a conservative standard-library
/// implementation instead of a regex dependency. It covers the same daily-use
/// ranges as `knifer-go`: regional indicators, keycap bases, pictographs,
/// dingbats, and common symbol emoji ranges.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert!(vstr::contains_emoji("ship it 🚀"));
/// assert!(!vstr::contains_emoji("knifer-rs"));
/// ```
#[must_use]
pub fn contains_emoji(input: &str) -> bool {
    contains_emoji_with_options(input, &EmojiOptions::new())
}

/// Returns `true` when `input` contains emoji-like content with custom options.
///
/// If `options` does not contain a matcher, the default matcher from
/// [`contains_emoji`] is used.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// let opts = vstr::with_emoji_matcher(|input| input.contains(":rocket:"));
/// assert!(vstr::contains_emoji_with_options("ship :rocket:", &opts));
/// assert!(!vstr::contains_emoji_with_options("ship 🚀", &opts));
/// ```
#[must_use]
pub fn contains_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> bool {
    if let Some(matcher) = &options.matcher {
        return matcher(input);
    }

    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if is_keycap_sequence(&chars, index) || is_emoji_scalar(chars[index]) {
            return true;
        }
        index += 1;
    }
    false
}

/// Removes emoji-like Unicode scalar values and emoji modifiers from `input`.
///
/// This keeps ordinary text intact while dropping common emoji code points,
/// variation selectors, zero-width joiners, and combining keycap marks.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::remove_emoji("ship 🚀 now"), "ship  now");
/// assert_eq!(vstr::remove_emoji("ok"), "ok");
/// ```
#[must_use]
pub fn remove_emoji(input: &str) -> String {
    remove_emoji_with_options(input, &EmojiOptions::new())
}

/// Removes emoji-like content with custom options.
///
/// If `options` does not contain a replacer, the default replacer from
/// [`remove_emoji`] is used.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// let opts = vstr::with_emoji_replacer(|input| input.replace(":rocket:", ""));
/// assert_eq!(vstr::remove_emoji_with_options("ship :rocket:", &opts), "ship ");
/// assert_eq!(vstr::remove_emoji_with_options("ship 🚀", &opts), "ship 🚀");
/// ```
#[must_use]
pub fn remove_emoji_with_options(input: &str, options: &EmojiOptions<'_>) -> String {
    if let Some(replacer) = &options.replacer {
        return replacer(input);
    }

    let chars: Vec<char> = input.chars().collect();
    let mut output = String::with_capacity(input.len());
    let mut index = 0;

    while index < chars.len() {
        if let Some(next_index) = keycap_sequence_end(&chars, index) {
            index = next_index;
            continue;
        }

        let ch = chars[index];
        if is_emoji_scalar(ch) || is_emoji_modifier(ch) {
            index += 1;
            continue;
        }

        output.push(ch);
        index += 1;
    }

    output
}

fn is_keycap_sequence(chars: &[char], index: usize) -> bool {
    keycap_sequence_end(chars, index).is_some()
}

fn keycap_sequence_end(chars: &[char], index: usize) -> Option<usize> {
    let base = *chars.get(index)?;
    if !matches!(base, '#' | '*' | '0'..='9') {
        return None;
    }

    let mut next = index + 1;
    if matches!(chars.get(next), Some('\u{FE0E}' | '\u{FE0F}')) {
        next += 1;
    }
    if matches!(chars.get(next), Some('\u{20E3}')) {
        Some(next + 1)
    } else {
        None
    }
}

fn is_emoji_modifier(ch: char) -> bool {
    matches!(ch, '\u{FE0E}' | '\u{FE0F}' | '\u{200D}' | '\u{20E3}')
}

fn is_emoji_scalar(ch: char) -> bool {
    matches!(
        u32::from(ch),
        0x1F1E6..=0x1F1FF
            | 0x1F300..=0x1FAFF
            | 0x2600..=0x27BF
    )
}
