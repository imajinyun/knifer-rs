use super::casefold::prefix_end_ignore_case;

/// Replaces the first occurrence of `from` with `to`.
///
/// If `from` is empty or missing, `input` is returned unchanged.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::replace_first("go go rust", "go", "rs"), "rs go rust");
/// assert_eq!(vstr::replace_first("rust", "go", "rs"), "rust");
/// ```
#[must_use]
pub fn replace_first(input: &str, from: &str, to: &str) -> String {
    replace_at(input, from, to, input.find(from))
}

/// Replaces the last occurrence of `from` with `to`.
///
/// If `from` is empty or missing, `input` is returned unchanged.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::replace_last("go go rust", "go", "rs"), "go rs rust");
/// assert_eq!(vstr::replace_last("rust", "go", "rs"), "rust");
/// ```
#[must_use]
pub fn replace_last(input: &str, from: &str, to: &str) -> String {
    replace_at(input, from, to, input.rfind(from))
}

/// Replaces all non-overlapping occurrences of `from`, ignoring Unicode case.
///
/// Matching uses simple scalar-by-scalar case folding, the same compatibility
/// boundary as [`equals_ignore_case`]. Replaced text is not searched again.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::replace_ignore_case("Go go Rust", "go", "rs"), "rs rs Rust");
/// assert_eq!(vstr::replace_ignore_case("abc\u{212A}", "k", "K"), "abcK");
/// ```
#[must_use]
pub fn replace_ignore_case(input: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return input.to_owned();
    }

    let mut output = String::with_capacity(input.len());
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Some(match_end) = prefix_end_ignore_case(remaining, from) {
            output.push_str(to);
            remaining = &remaining[match_end..];
        } else if let Some(ch) = remaining.chars().next() {
            output.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }

    output
}

/// Replaces multiple literal needles in a single left-to-right pass.
///
/// Empty needles are ignored. When several needles match at the same position,
/// the first replacement from the iterator wins. Replaced text is not searched
/// again, so the result is deterministic and independent of chained
/// replacement side effects.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::replace_many("hello rust world", [("hello", "hi"), ("world", "team")]),
///     "hi rust team"
/// );
/// assert_eq!(vstr::replace_many("aaaa", [("aa", "b"), ("a", "c")]), "bb");
/// ```
#[must_use]
pub fn replace_many<'src, I>(input: &str, replacements: I) -> String
where
    I: IntoIterator<Item = (&'src str, &'src str)>,
{
    let replacements: Vec<(&str, &str)> = replacements
        .into_iter()
        .filter(|(from, _)| !from.is_empty())
        .collect();
    if replacements.is_empty() {
        return input.to_owned();
    }

    let mut output = String::with_capacity(input.len());
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Some((from, to)) = replacements
            .iter()
            .find(|(from, _)| remaining.starts_with(*from))
        {
            output.push_str(to);
            remaining = &remaining[from.len()..];
        } else if let Some(ch) = remaining.chars().next() {
            output.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }

    output
}

fn replace_at(input: &str, from: &str, to: &str, index: Option<usize>) -> String {
    if from.is_empty() {
        return input.to_owned();
    }

    let Some(index) = index else {
        return input.to_owned();
    };

    let mut output = String::with_capacity(input.len() - from.len() + to.len());
    output.push_str(&input[..index]);
    output.push_str(to);
    output.push_str(&input[index + from.len()..]);
    output
}
