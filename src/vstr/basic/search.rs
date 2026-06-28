use super::casefold::prefix_end_ignore_case;

/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains("knifer-rs", "rs"));
/// ```
#[must_use]
pub fn contains(input: &str, needle: &str) -> bool {
    input.contains(needle)
}

/// Returns `true` when `input` contains any value in `needles`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_any("knifer-rs", ["go", "rs"]));
/// assert!(!vstr::contains_any("knifer-rs", ["java", "py"]));
/// ```
#[must_use]
pub fn contains_any<'src, I>(input: &str, needles: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    needles.into_iter().any(|needle| contains(input, needle))
}

/// Returns `true` when `input` contains every value in `needles`.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_all("knifer-rs", ["knife", "rs"]));
/// assert!(!vstr::contains_all("knifer-rs", ["knife", "go"]));
/// ```
#[must_use]
pub fn contains_all<'src, I>(input: &str, needles: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    needles.into_iter().all(|needle| contains(input, needle))
}

/// Returns `true` when `input` contains none of the values in `needles`.
///
/// Empty iterators return `true`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_none("knifer-rs", ["go", "java"]));
/// assert!(!vstr::contains_none("knifer-rs", ["go", "rs"]));
/// ```
#[must_use]
pub fn contains_none<'src, I>(input: &str, needles: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    needles.into_iter().all(|needle| !contains(input, needle))
}

/// Returns `true` when `input` contains `needle`, ignoring Unicode case.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_ignore_case("Knifer-RS", "rs"));
/// ```
#[must_use]
pub fn contains_ignore_case(input: &str, needle: &str) -> bool {
    input.to_lowercase().contains(&needle.to_lowercase())
}

/// Returns `true` when `input` contains any needle, ignoring Unicode case.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_any_ignore_case("Knifer-RS", ["go", "RS"]));
/// assert!(!vstr::contains_any_ignore_case("Knifer-RS", ["go", "java"]));
/// ```
#[must_use]
pub fn contains_any_ignore_case<'src, I>(input: &str, needles: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    let input = input.to_lowercase();
    needles
        .into_iter()
        .any(|needle| input.contains(&needle.to_lowercase()))
}

/// Returns `true` when `input` contains every needle, ignoring Unicode case.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::contains_all_ignore_case("Knifer-RS", ["knife", "RS"]));
/// assert!(!vstr::contains_all_ignore_case("Knifer-RS", ["knife", "go"]));
/// ```
#[must_use]
pub fn contains_all_ignore_case<'src, I>(input: &str, needles: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    let input = input.to_lowercase();
    needles
        .into_iter()
        .all(|needle| input.contains(&needle.to_lowercase()))
}

/// Returns the first non-empty needle found in `input`.
///
/// The returned tuple is `(needle, start, end)`, where `start` and `end` are
/// byte indexes into `input`. If multiple needles start at the same byte index,
/// the first needle from the iterator wins. Empty needles are ignored.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::find_any("hello rust", ["go", "rust"]), Some(("rust", 6, 10)));
/// assert_eq!(vstr::find_any("hello rust", ["", "go"]), None);
/// ```
#[must_use]
pub fn find_any<'needle, I>(input: &str, needles: I) -> Option<(&'needle str, usize, usize)>
where
    I: IntoIterator<Item = &'needle str>,
{
    let mut best = None;

    for needle in needles {
        if needle.is_empty() {
            continue;
        }
        let Some(start) = input.find(needle) else {
            continue;
        };

        if best.is_none_or(|(_, best_start, _)| start < best_start) {
            best = Some((needle, start, start + needle.len()));
        }
    }

    best
}

/// Counts non-overlapping matches of `needle` in `input`.
///
/// Empty needles return zero to avoid surprising infinite-match semantics.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::count_matches("aaaa", "aa"), 2);
/// assert_eq!(vstr::count_matches("你好你好", "你好"), 2);
/// ```
#[must_use]
pub fn count_matches(input: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }

    input.matches(needle).count()
}

/// Returns byte ranges for all non-overlapping matches of `needle`.
///
/// Empty needles return no ranges to avoid surprising infinite-match semantics.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::find_all("aaaa", "aa"), vec![(0, 2), (2, 4)]);
/// assert_eq!(vstr::find_all("你好你好", "你好"), vec![(0, 6), (6, 12)]);
/// ```
#[must_use]
pub fn find_all(input: &str, needle: &str) -> Vec<(usize, usize)> {
    if needle.is_empty() {
        return Vec::new();
    }

    input
        .match_indices(needle)
        .map(|(start, matched)| (start, start + matched.len()))
        .collect()
}

/// Returns byte ranges for all non-overlapping case-insensitive matches.
///
/// Matching uses simple scalar-by-scalar case folding, the same compatibility
/// boundary as [`crate::vstr::equals_ignore_case`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::find_all_ignore_case("Go go Rust", "go"), vec![(0, 2), (3, 5)]);
/// assert_eq!(vstr::find_all_ignore_case("abc\u{212A}", "k"), vec![(3, 6)]);
/// ```
#[must_use]
pub fn find_all_ignore_case(input: &str, needle: &str) -> Vec<(usize, usize)> {
    if needle.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut remaining = input;
    let mut offset = 0;

    while !remaining.is_empty() {
        if let Some(match_end) = prefix_end_ignore_case(remaining, needle) {
            ranges.push((offset, offset + match_end));
            remaining = &remaining[match_end..];
            offset += match_end;
        } else if let Some(ch) = remaining.chars().next() {
            remaining = &remaining[ch.len_utf8()..];
            offset += ch.len_utf8();
        }
    }

    ranges
}
