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
