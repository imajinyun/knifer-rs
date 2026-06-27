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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
/// use knifer_rs::vstr;
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
