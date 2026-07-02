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

/// Selects the line-breaking strategy used when wrapping a paragraph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WrapAlgorithm {
    /// Greedy first-fit: put each word on the current line while it fits, then
    /// break. Fast, single pass, and matches [`super::wrap`].
    FirstFit,
    /// Optimal-fit: choose line breaks that minimize the sum of squared
    /// trailing slack across every line except the last, giving more balanced
    /// (less ragged) paragraphs. Applies to collapsed-whitespace wrapping; the
    /// last line is never penalized and over-long words follow the long-word
    /// policy.
    OptimalFit,
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
    /// Line-breaking strategy.
    pub wrap_algorithm: WrapAlgorithm,
    /// Extra scalar separators where words may be wrapped without inserting a
    /// space. Separators are retained at the end of the preceding segment.
    pub word_separators: &'src [char],
}

impl<'src> WrapOptions<'src> {
    /// Creates wrapping options with scalar width and default wrapping behavior.
    ///
    /// Defaults match [`super::wrap`]: whitespace is collapsed, long words are
    /// split by scalar value, and greedy first-fit line breaking is used.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vstr::WrapOptions;
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
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separators: &[],
        }
    }

    /// Sets separate indentation for first and following lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vstr::WrapOptions;
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
    /// use kniferrs::vstr::{WhitespaceMode, WrapOptions};
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
    /// use kniferrs::vstr::{LongWordPolicy, WrapOptions};
    ///
    /// let options = WrapOptions::new(10).with_long_word_policy(LongWordPolicy::Preserve);
    /// assert_eq!(options.long_word_policy, LongWordPolicy::Preserve);
    /// ```
    #[must_use]
    pub const fn with_long_word_policy(mut self, long_word_policy: LongWordPolicy) -> Self {
        self.long_word_policy = long_word_policy;
        self
    }

    /// Sets the line-breaking strategy.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vstr::{WrapAlgorithm, WrapOptions};
    ///
    /// let options = WrapOptions::new(10).with_wrap_algorithm(WrapAlgorithm::OptimalFit);
    /// assert_eq!(options.wrap_algorithm, WrapAlgorithm::OptimalFit);
    /// ```
    #[must_use]
    pub const fn with_wrap_algorithm(mut self, wrap_algorithm: WrapAlgorithm) -> Self {
        self.wrap_algorithm = wrap_algorithm;
        self
    }

    /// Sets extra word separators where wrapping may break without adding
    /// spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use kniferrs::vstr::WrapOptions;
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
