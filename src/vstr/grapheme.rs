use unicode_segmentation::UnicodeSegmentation;

/// Returns Unicode grapheme clusters from `input`.
///
/// This helper is available only with the `unicode-segmentation` feature.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::graphemes("e\u{301}🇨🇳"), vec!["e\u{301}", "🇨🇳"]);
/// # }
/// ```
#[must_use]
pub fn graphemes(input: &str) -> Vec<&str> {
    input.graphemes(true).collect()
}

/// Returns the number of Unicode grapheme clusters in `input`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::grapheme_len("e\u{301}🇨🇳"), 2);
/// # }
/// ```
#[must_use]
pub fn grapheme_len(input: &str) -> usize {
    input.graphemes(true).count()
}

/// Returns a borrowed prefix containing at most `count` grapheme clusters.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::take_graphemes("e\u{301}🇨🇳rust", 2), "e\u{301}🇨🇳");
/// # }
/// ```
#[must_use]
pub fn take_graphemes(input: &str, count: usize) -> &str {
    if count == 0 {
        return "";
    }

    match input.grapheme_indices(true).nth(count) {
        Some((index, _)) => &input[..index],
        None => input,
    }
}

/// Truncates text to at most `max_graphemes` grapheme clusters and appends a suffix.
///
/// The returned string never exceeds `max_graphemes` grapheme clusters. If the
/// suffix is longer than the budget, the suffix itself is truncated by grapheme
/// clusters.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::truncate_graphemes("e\u{301}🇨🇳rust", 4, "..."), "e\u{301}...");
/// assert_eq!(vstr::truncate_graphemes("short", 10, "..."), "short");
/// # }
/// ```
#[must_use]
pub fn truncate_graphemes(input: &str, max_graphemes: usize, suffix: &str) -> String {
    if max_graphemes == 0 {
        return String::new();
    }
    if grapheme_len(input) <= max_graphemes {
        return input.to_owned();
    }

    let suffix_len = grapheme_len(suffix);
    if suffix_len >= max_graphemes {
        return take_graphemes(suffix, max_graphemes).to_owned();
    }

    let keep = max_graphemes - suffix_len;
    let mut output = String::from(take_graphemes(input, keep));
    output.push_str(suffix);
    output
}

/// Reverses `input` by Unicode grapheme clusters.
///
/// Unlike [`reverse`](crate::vstr::reverse), which reverses by Unicode scalar
/// values, this keeps combining marks, flag sequences, and ZWJ emoji intact.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// // Scalar reverse would separate the combining accent from its base letter.
/// assert_eq!(vstr::reverse_graphemes("e\u{301}b"), "be\u{301}");
/// assert_eq!(vstr::reverse_graphemes("🇨🇳🇯🇵"), "🇯🇵🇨🇳");
/// # }
/// ```
#[must_use]
pub fn reverse_graphemes(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

/// Pads `input` on the left until it reaches `target_len` grapheme clusters.
///
/// This is the grapheme-cluster counterpart of
/// [`pad_left`](crate::vstr::pad_left), so combining marks and emoji sequences
/// count as a single unit when measuring the current length.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// // "e\u{301}" is one grapheme cluster, so only two pad chars are added.
/// assert_eq!(vstr::pad_left_graphemes("e\u{301}", 3, '*'), "**e\u{301}");
/// # }
/// ```
#[must_use]
pub fn pad_left_graphemes(input: &str, target_len: usize, pad: char) -> String {
    let input_len = grapheme_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output.push_str(input);
    output
}

/// Pads `input` on the right until it reaches `target_len` grapheme clusters.
///
/// This is the grapheme-cluster counterpart of
/// [`pad_right`](crate::vstr::pad_right).
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::pad_right_graphemes("e\u{301}", 3, '*'), "e\u{301}**");
/// # }
/// ```
#[must_use]
pub fn pad_right_graphemes(input: &str, target_len: usize, pad: char) -> String {
    let input_len = grapheme_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.push_str(input);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output
}

/// Centers `input` to `width` grapheme clusters using `pad`.
///
/// This is the grapheme-cluster counterpart of [`center`](crate::vstr::center).
/// When an odd number of padding characters is required, the extra padding is
/// added to the right side.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::center_graphemes("e\u{301}", 4, '-'), "-e\u{301}--");
/// # }
/// ```
#[must_use]
pub fn center_graphemes(input: &str, width: usize, pad: char) -> String {
    let len = grapheme_len(input);
    if len >= width {
        return input.to_owned();
    }

    let padding = width - len;
    let left = padding / 2;
    let right = padding - left;
    let mut output = String::with_capacity(input.len() + padding * pad.len_utf8());
    output.extend(std::iter::repeat_n(pad, left));
    output.push_str(input);
    output.extend(std::iter::repeat_n(pad, right));
    output
}

/// Masks the middle of `input`, keeping `visible_start` and `visible_end`
/// leading and trailing grapheme clusters.
///
/// This is the grapheme-cluster counterpart of [`mask`](crate::vstr::mask), so
/// combining marks, flag sequences, and ZWJ emoji are never split when counting
/// the visible units or the masked run.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// // Each flag is one grapheme cluster: keep the first and last, mask the rest.
/// assert_eq!(vstr::mask_graphemes("🇨🇳🇯🇵🇰🇷🇺🇸", 1, 1, '*'), "🇨🇳**🇺🇸");
/// assert_eq!(vstr::mask_graphemes("short", 10, 10, '*'), "short");
/// # }
/// ```
#[must_use]
pub fn mask_graphemes(input: &str, visible_start: usize, visible_end: usize, mask: char) -> String {
    let len = grapheme_len(input);
    if visible_start + visible_end >= len {
        return input.to_owned();
    }

    let start = take_graphemes(input, visible_start);
    let end = take_last_graphemes(input, visible_end);
    let mask_count = len - visible_start - visible_end;
    let mut output = String::with_capacity(input.len());
    output.push_str(start);
    output.extend(std::iter::repeat_n(mask, mask_count));
    output.push_str(end);
    output
}

/// Returns a borrowed suffix containing at most `count` grapheme clusters.
fn take_last_graphemes(input: &str, count: usize) -> &str {
    if count == 0 {
        return "";
    }

    let total = grapheme_len(input);
    if count >= total {
        return input;
    }

    match input.grapheme_indices(true).nth(total - count) {
        Some((index, _)) => &input[index..],
        None => input,
    }
}

/// Returns Unicode words from `input` using UAX #29 word boundaries.
///
/// This helper is available only with the `unicode-segmentation` feature. It
/// returns only word-like segments that contain alphabetic or numeric
/// characters; punctuation and whitespace are skipped. This is boundary
/// detection, not dictionary-based natural-language tokenization.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::unicode_words("The quick (\"brown\") fox can't jump 32.3 feet"),
///     vec!["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet"]
/// );
/// # }
/// ```
#[must_use]
pub fn unicode_words(input: &str) -> Vec<&str> {
    input.unicode_words().collect()
}

/// Returns the number of Unicode words in `input` using UAX #29 word boundaries.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unicode_word_len("hello 世界 32.3"), 4);
/// # }
/// ```
#[must_use]
pub fn unicode_word_len(input: &str) -> usize {
    input.unicode_words().count()
}

/// Returns Unicode words and their byte offsets using UAX #29 word boundaries.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unicode_word_indices("hi 世界"), vec![(0, "hi"), (3, "世"), (6, "界")]);
/// # }
/// ```
#[must_use]
pub fn unicode_word_indices(input: &str) -> Vec<(usize, &str)> {
    input.unicode_word_indices().collect()
}

/// Splits `input` on Unicode word boundaries and keeps separators.
///
/// Unlike [`unicode_words`], this returns punctuation and whitespace segments so
/// concatenating the output reconstructs the original input.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::split_word_bounds("Hi, 世界!"), vec!["Hi", ",", " ", "世", "界", "!"]);
/// # }
/// ```
#[must_use]
pub fn split_word_bounds(input: &str) -> Vec<&str> {
    input.split_word_bounds().collect()
}

/// Splits `input` on Unicode word boundaries and returns byte offsets.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::split_word_bound_indices("Hi, 世界!"),
///     vec![(0, "Hi"), (2, ","), (3, " "), (4, "世"), (7, "界"), (10, "!")]
/// );
/// # }
/// ```
#[must_use]
pub fn split_word_bound_indices(input: &str) -> Vec<(usize, &str)> {
    input.split_word_bound_indices().collect()
}

/// Returns Unicode sentences from `input` using UAX #29 sentence boundaries.
///
/// This helper is available only with the `unicode-segmentation` feature. It
/// returns only sentence-like segments that contain alphabetic or numeric
/// characters; separator-only segments are skipped.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::unicode_sentences("Mr. Fox jumped. [...] The dog was too lazy."),
///     vec!["Mr. ", "Fox jumped. ", "The dog was too lazy."]
/// );
/// # }
/// ```
#[must_use]
pub fn unicode_sentences(input: &str) -> Vec<&str> {
    input.unicode_sentences().collect()
}

/// Returns the number of Unicode sentences in `input` using UAX #29 boundaries.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::unicode_sentence_len("One. Two? Three!"), 3);
/// # }
/// ```
#[must_use]
pub fn unicode_sentence_len(input: &str) -> usize {
    input.unicode_sentences().count()
}

/// Splits `input` on Unicode sentence boundaries and keeps separator segments.
///
/// Unlike [`unicode_sentences`], this returns separator-only segments so
/// concatenating the output reconstructs the original input.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::split_sentence_bounds("Mr. Fox jumped. [...] The dog was too lazy."),
///     vec!["Mr. ", "Fox jumped. ", "[...] ", "The dog was too lazy."]
/// );
/// # }
/// ```
#[must_use]
pub fn split_sentence_bounds(input: &str) -> Vec<&str> {
    input.split_sentence_bounds().collect()
}

/// Splits `input` on Unicode sentence boundaries and returns byte offsets.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "unicode-segmentation")]
/// # {
/// use kniferrs::vstr;
///
/// assert_eq!(
///     vstr::split_sentence_bound_indices("Mr. Fox jumped. [...] The dog was too lazy."),
///     vec![
///         (0, "Mr. "),
///         (4, "Fox jumped. "),
///         (16, "[...] "),
///         (22, "The dog was too lazy.")
///     ]
/// );
/// # }
/// ```
#[must_use]
pub fn split_sentence_bound_indices(input: &str) -> Vec<(usize, &str)> {
    input.split_sentence_bound_indices().collect()
}
