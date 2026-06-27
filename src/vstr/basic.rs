/// Returns `true` when `input` has no bytes.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_empty(""));
/// assert!(!vstr::is_empty(" "));
/// ```
#[must_use]
pub const fn is_empty(input: &str) -> bool {
    input.is_empty()
}

/// Returns `true` when `input` has at least one byte.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_not_empty(" "));
/// assert!(!vstr::is_not_empty(""));
/// ```
#[must_use]
pub const fn is_not_empty(input: &str) -> bool {
    !input.is_empty()
}

/// Returns `true` when `input` is empty or only contains Unicode whitespace.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_blank(" \n\t"));
/// assert!(!vstr::is_blank("knifer-rs"));
/// ```
#[must_use]
pub fn is_blank(input: &str) -> bool {
    trim(input).is_empty()
}

/// Returns `true` when `input` contains at least one non-whitespace character.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_not_blank("knifer-rs"));
/// assert!(!vstr::is_not_blank(" \n\t"));
/// ```
#[must_use]
pub fn is_not_blank(input: &str) -> bool {
    !is_blank(input)
}

/// Returns `true` when any value in `values` is empty.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::has_empty(["a", ""]));
/// assert!(!vstr::has_empty(["a", " "]));
/// ```
#[must_use]
pub fn has_empty<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().any(str::is_empty)
}

/// Returns `true` when any value in `values` is blank.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::has_blank(["a", " "]));
/// assert!(!vstr::has_blank(["a", "b"]));
/// ```
#[must_use]
pub fn has_blank<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().any(is_blank)
}

/// Returns `true` when all values in `values` are empty.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_all_empty(["", ""]));
/// assert!(!vstr::is_all_empty(["", " "]));
/// ```
#[must_use]
pub fn is_all_empty<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().all(str::is_empty)
}

/// Returns `true` when all values in `values` are blank.
///
/// Empty iterators return `true`, matching [`Iterator::all`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::is_all_blank(["", " ", "\n"]));
/// assert!(!vstr::is_all_blank(["", "knifer-rs"]));
/// ```
#[must_use]
pub fn is_all_blank<'src, I>(values: I) -> bool
where
    I: IntoIterator<Item = &'src str>,
{
    values.into_iter().all(is_blank)
}

/// Returns `input` without leading and trailing Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim("  hello  "), "hello");
/// ```
#[must_use]
pub fn trim(input: &str) -> &str {
    input.trim()
}

/// Returns `input` without leading Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_start("  hello  "), "hello  ");
/// ```
#[must_use]
pub fn trim_start(input: &str) -> &str {
    input.trim_start()
}

/// Returns `input` without trailing Unicode whitespace.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_end("  hello  "), "  hello");
/// ```
#[must_use]
pub fn trim_end(input: &str) -> &str {
    input.trim_end()
}

/// Returns an owned `String` without leading and trailing Unicode whitespace.
///
/// Prefer [`trim`] when a borrowed result is enough.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// let value = String::from("  hello  ");
/// assert_eq!(vstr::trim_to_string(&value), "hello");
/// ```
#[must_use]
pub fn trim_to_string(input: &str) -> String {
    trim(input).to_owned()
}

/// Returns `input` without leading and trailing Unicode whitespace.
///
/// This is an alias for [`trim`] to align with `knifer-go`'s `TrimToEmpty`
/// API name. Rust string slices are never null, so the function can borrow
/// directly from the original input.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::trim_to_empty("  hello  "), "hello");
/// assert_eq!(vstr::trim_to_empty("   "), "");
/// ```
#[must_use]
pub fn trim_to_empty(input: &str) -> &str {
    trim(input)
}

/// Splits `input` by `separator`.
///
/// Empty input returns an empty vector. This mirrors common utility-library
/// behavior for data-cleanup paths where an empty field should produce no
/// parts.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::split("a,b", ","), vec!["a", "b"]);
/// assert!(vstr::split("", ",").is_empty());
/// ```
#[must_use]
pub fn split<'src>(input: &'src str, separator: &str) -> Vec<&'src str> {
    if input.is_empty() {
        Vec::new()
    } else {
        input.split(separator).collect()
    }
}

/// Splits `input`, trims each part, and drops blank parts.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::split_trim(" a, ,b ", ","), vec!["a", "b"]);
/// ```
#[must_use]
pub fn split_trim<'src>(input: &'src str, separator: &str) -> Vec<&'src str> {
    input
        .split(separator)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect()
}

/// Returns a substring by Unicode scalar-value indexes.
///
/// `from_index` is inclusive and `to_index` is exclusive. Negative indexes
/// count from the end, out-of-range indexes are clamped, and reversed ranges
/// are normalized.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::sub("abcdef", 1, 4), "bcd");
/// assert_eq!(vstr::sub("你好世界", 1, -1), "好世");
/// assert_eq!(vstr::sub("abcdef", 4, 1), "bcd");
/// ```
#[must_use]
pub fn sub(input: &str, from_index: isize, to_index: isize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let start = normalize_index(from_index, len);
    let end = normalize_index(to_index, len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    chars[start..end].iter().collect()
}

/// Returns text before `separator`.
///
/// When `use_last_separator` is `true`, the last separator is used. If
/// `separator` is empty or not found, `input` is returned unchanged.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::sub_before("a/b/c", "/", false), "a");
/// assert_eq!(vstr::sub_before("a/b/c", "/", true), "a/b");
/// assert_eq!(vstr::sub_before("a/b/c", "|", false), "a/b/c");
/// ```
#[must_use]
pub fn sub_before<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str {
    if input.is_empty() || separator.is_empty() {
        return input;
    }

    let index = if use_last_separator {
        input.rfind(separator)
    } else {
        input.find(separator)
    };

    index.map_or(input, |index| &input[..index])
}

/// Returns text after `separator`.
///
/// When `use_last_separator` is `true`, the last separator is used. If
/// `separator` is empty or not found, an empty string slice is returned.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::sub_after("a/b/c", "/", false), "b/c");
/// assert_eq!(vstr::sub_after("a/b/c", "/", true), "c");
/// assert_eq!(vstr::sub_after("a/b/c", "|", false), "");
/// ```
#[must_use]
pub fn sub_after<'src>(input: &'src str, separator: &str, use_last_separator: bool) -> &'src str {
    if input.is_empty() || separator.is_empty() {
        return "";
    }

    let index = if use_last_separator {
        input.rfind(separator)
    } else {
        input.find(separator)
    };

    index.map_or("", |index| &input[index + separator.len()..])
}

/// Returns text before the first `separator`.
///
/// This is a concise alias for `sub_before(input, separator, false)`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::before("a/b/c", "/"), "a");
/// assert_eq!(vstr::before("a/b/c", "|"), "a/b/c");
/// ```
#[must_use]
pub fn before<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_before(input, separator, false)
}

/// Returns text before the last `separator`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::before_last("a/b/c", "/"), "a/b");
/// ```
#[must_use]
pub fn before_last<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_before(input, separator, true)
}

/// Returns text after the first `separator`.
///
/// This is a concise alias for `sub_after(input, separator, false)`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::after("a/b/c", "/"), "b/c");
/// assert_eq!(vstr::after("a/b/c", "|"), "");
/// ```
#[must_use]
pub fn after<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_after(input, separator, false)
}

/// Returns text after the last `separator`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::after_last("a/b/c", "/"), "c");
/// ```
#[must_use]
pub fn after_last<'src>(input: &'src str, separator: &str) -> &'src str {
    sub_after(input, separator, true)
}

/// Returns text between the first `start` and the following `end` marker.
///
/// `None` is returned when either marker is empty or missing. The returned
/// slice borrows from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::between("id=[42]", "[", "]"), Some("42"));
/// assert_eq!(vstr::between("id=42", "[", "]"), None);
/// ```
#[must_use]
pub fn between<'src>(input: &'src str, start: &str, end: &str) -> Option<&'src str> {
    if start.is_empty() || end.is_empty() {
        return None;
    }

    let content_start = input.find(start)? + start.len();
    let rest = &input[content_start..];
    let content_end = rest.find(end)?;
    Some(&rest[..content_end])
}

/// Repeats `input` `count` times.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::repeat("ab", 3), "ababab");
/// assert_eq!(vstr::repeat("ab", 0), "");
/// ```
#[must_use]
pub fn repeat(input: &str, count: usize) -> String {
    input.repeat(count)
}

/// Pads `input` on the left until it reaches `target_len` Unicode scalar values.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::pad_left("42", 5, '0'), "00042");
/// assert_eq!(vstr::pad_left("你好", 3, '*'), "*你好");
/// ```
#[must_use]
pub fn pad_left(input: &str, target_len: usize, pad: char) -> String {
    let input_len = char_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output.push_str(input);
    output
}

/// Pads `input` on the right until it reaches `target_len` Unicode scalar values.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::pad_right("42", 5, '0'), "42000");
/// assert_eq!(vstr::pad_right("你好", 3, '*'), "你好*");
/// ```
#[must_use]
pub fn pad_right(input: &str, target_len: usize, pad: char) -> String {
    let input_len = char_len(input);
    if input_len >= target_len {
        return input.to_owned();
    }

    let pad_count = target_len - input_len;
    let mut output = String::with_capacity(input.len() + pad.len_utf8() * pad_count);
    output.push_str(input);
    output.extend(std::iter::repeat_n(pad, pad_count));
    output
}

/// Returns `default` when `input` is empty, otherwise returns `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::default_if_empty("", "fallback"), "fallback");
/// assert_eq!(vstr::default_if_empty(" ", "fallback"), " ");
/// ```
#[must_use]
pub fn default_if_empty<'src>(input: &'src str, default: &'src str) -> &'src str {
    if input.is_empty() { default } else { input }
}

/// Returns `default` when `input` is blank, otherwise returns `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::default_if_blank(" ", "fallback"), "fallback");
/// assert_eq!(vstr::default_if_blank("value", "fallback"), "value");
/// ```
#[must_use]
pub fn default_if_blank<'src>(input: &'src str, default: &'src str) -> &'src str {
    if is_blank(input) { default } else { input }
}

/// Returns `true` when `input` contains `needle`.
///
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
/// boundary as [`equals_ignore_case`].
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

/// Returns `true` when `input` starts with `prefix`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::starts_with("knifer-rs", "knife"));
/// ```
#[must_use]
pub fn starts_with(input: &str, prefix: &str) -> bool {
    input.starts_with(prefix)
}

/// Returns `true` when `input` ends with `suffix`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ends_with("knifer-rs", "rs"));
/// ```
#[must_use]
pub fn ends_with(input: &str, suffix: &str) -> bool {
    input.ends_with(suffix)
}

/// Returns `true` when `input` starts with `prefix`, ignoring Unicode case.
///
/// This uses the same simple scalar-by-scalar case folding as
/// [`equals_ignore_case`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::starts_with_ignore_case("Knifer-RS", "knife"));
/// assert!(vstr::starts_with_ignore_case("\u{212A}nife", "k"));
/// ```
#[must_use]
pub fn starts_with_ignore_case(input: &str, prefix: &str) -> bool {
    prefix_end_ignore_case(input, prefix).is_some()
}

/// Returns `true` when `input` ends with `suffix`, ignoring Unicode case.
///
/// This uses the same simple scalar-by-scalar case folding as
/// [`equals_ignore_case`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ends_with_ignore_case("Knifer-RS", "RS"));
/// assert!(vstr::ends_with_ignore_case("abc\u{212A}", "k"));
/// ```
#[must_use]
pub fn ends_with_ignore_case(input: &str, suffix: &str) -> bool {
    suffix_start_ignore_case(input, suffix).is_some()
}

/// Returns `true` when both strings are equal, ignoring Unicode case.
///
/// This follows Rust's standard Unicode case mappings without expanding a
/// single scalar into multiple comparison characters. It covers common
/// `strings.EqualFold`-style simple folds such as final sigma and Kelvin sign,
/// while keeping German `ß` distinct from `ss`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::equals_ignore_case("Knifer-RS", "knifer-rs"));
/// assert!(vstr::equals_ignore_case("Σ", "ς"));
/// ```
#[must_use]
pub fn equals_ignore_case(left: &str, right: &str) -> bool {
    left.chars().eq(right.chars()) || {
        let mut left = left.chars();
        let mut right = right.chars();

        loop {
            match (left.next(), right.next()) {
                (Some(left), Some(right)) if chars_equal_ignore_case(left, right) => {}
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

fn chars_equal_ignore_case(left: char, right: char) -> bool {
    left == right
        || left.to_lowercase().eq(right.to_lowercase())
        || left.to_uppercase().eq(right.to_uppercase())
}

/// Reverses `input` by Unicode scalar values.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::reverse("ab你好"), "好你ba");
/// ```
#[must_use]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/// Replaces `{}` placeholders with display arguments in order.
///
/// Use `\{` to escape a literal opening brace. Extra arguments are ignored and
/// placeholders without arguments are kept as `{}`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(
///     vstr::format("name={}, age={}", &[&"tom", &12]),
///     "name=tom, age=12"
/// );
/// assert_eq!(vstr::format(r"\{} {}", &[&"x"]), "{} x");
/// ```
#[must_use]
pub fn format(template: &str, args: &[&dyn std::fmt::Display]) -> String {
    if template.is_empty() || args.is_empty() {
        return template.to_owned();
    }

    let mut output = String::with_capacity(template.len());
    let mut arg_index = 0;
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek() == Some(&'{') {
            output.push('{');
            chars.next();
            continue;
        }

        if ch == '{' && chars.peek() == Some(&'}') {
            chars.next();
            if let Some(arg) = args.get(arg_index) {
                output.push_str(&arg.to_string());
                arg_index += 1;
            } else {
                output.push_str("{}");
            }
            continue;
        }

        output.push(ch);
    }

    output
}

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

/// Escapes Rust regex metacharacters so `input` can be used as a literal pattern.
///
/// This helper is dependency-free and follows the metacharacter set used by the
/// Rust `regex` crate for literal escaping.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
/// ```
#[must_use]
pub fn escape_regex(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        if is_regex_meta(ch) {
            output.push('\\');
        }
        output.push(ch);
    }
    output
}

/// Alias for [`escape_regex`] using a common regex-library name.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::quote_meta("[rust]"), r"\[rust\]");
/// ```
#[must_use]
pub fn quote_meta(input: &str) -> String {
    escape_regex(input)
}

/// Returns `input` without `prefix` when it is present.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_prefix("knifer-rs", "knifer-"), "rs");
/// assert_eq!(vstr::remove_prefix("knifer-rs", "go-"), "knifer-rs");
/// ```
#[must_use]
pub fn remove_prefix<'src>(input: &'src str, prefix: &str) -> &'src str {
    input.strip_prefix(prefix).unwrap_or(input)
}

/// Returns `input` without `suffix` when it is present.
///
/// This function borrows from the original string and does not allocate.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::remove_suffix("knifer-rs", "-rs"), "knifer");
/// assert_eq!(vstr::remove_suffix("knifer-rs", "-go"), "knifer-rs");
/// ```
#[must_use]
pub fn remove_suffix<'src>(input: &'src str, suffix: &str) -> &'src str {
    input.strip_suffix(suffix).unwrap_or(input)
}

/// Splits `input` once at the first `separator`.
///
/// `None` is returned when the separator is empty or missing. The returned
/// slices borrow from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::split_once("a=b=c", "="), Some(("a", "b=c")));
/// assert_eq!(vstr::split_once("abc", "="), None);
/// ```
#[must_use]
pub fn split_once<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)> {
    if separator.is_empty() {
        return None;
    }

    input
        .find(separator)
        .map(|index| (&input[..index], &input[index + separator.len()..]))
}

/// Splits `input` once at the last `separator`.
///
/// `None` is returned when the separator is empty or missing. The returned
/// slices borrow from `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::split_once_last("a=b=c", "="), Some(("a=b", "c")));
/// assert_eq!(vstr::split_once_last("abc", "="), None);
/// ```
#[must_use]
pub fn split_once_last<'src>(input: &'src str, separator: &str) -> Option<(&'src str, &'src str)> {
    if separator.is_empty() {
        return None;
    }

    input
        .rfind(separator)
        .map(|index| (&input[..index], &input[index + separator.len()..]))
}

/// Strips `prefix` from `input`, ignoring Unicode case.
///
/// The returned slice preserves the original input casing. `None` is returned
/// when the prefix is not present.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::strip_prefix_ignore_case("Knifer-RS", "knife"), Some("r-RS"));
/// assert_eq!(vstr::strip_prefix_ignore_case("Knifer-RS", "go"), None);
/// ```
#[must_use]
pub fn strip_prefix_ignore_case<'src>(input: &'src str, prefix: &str) -> Option<&'src str> {
    prefix_end_ignore_case(input, prefix).map(|end| &input[end..])
}

/// Strips `suffix` from `input`, ignoring Unicode case.
///
/// The returned slice preserves the original input casing. `None` is returned
/// when the suffix is not present.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
/// assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "go"), None);
/// ```
#[must_use]
pub fn strip_suffix_ignore_case<'src>(input: &'src str, suffix: &str) -> Option<&'src str> {
    suffix_start_ignore_case(input, suffix).map(|start| &input[..start])
}

/// Adds `prefix` when `input` does not already start with it.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::add_prefix_if_not("path", "/"), "/path");
/// assert_eq!(vstr::add_prefix_if_not("/path", "/"), "/path");
/// ```
#[must_use]
pub fn add_prefix_if_not(input: &str, prefix: &str) -> String {
    if input.starts_with(prefix) {
        input.to_owned()
    } else {
        let mut output = String::with_capacity(prefix.len() + input.len());
        output.push_str(prefix);
        output.push_str(input);
        output
    }
}

/// Adds `suffix` when `input` does not already end with it.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::add_suffix_if_not("path", "/"), "path/");
/// assert_eq!(vstr::add_suffix_if_not("path/", "/"), "path/");
/// ```
#[must_use]
pub fn add_suffix_if_not(input: &str, suffix: &str) -> String {
    if input.ends_with(suffix) {
        input.to_owned()
    } else {
        let mut output = String::with_capacity(input.len() + suffix.len());
        output.push_str(input);
        output.push_str(suffix);
        output
    }
}

/// Returns the byte length of `input`.
///
/// For Unicode scalar-value length, use [`char_len`].
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::byte_len("你好"), 6);
/// ```
#[must_use]
pub const fn byte_len(input: &str) -> usize {
    input.len()
}

/// Returns the number of Unicode scalar values in `input`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::char_len("你好"), 2);
/// ```
#[must_use]
pub fn char_len(input: &str) -> usize {
    input.chars().count()
}

/// Returns the number of Unicode scalar values in `input`.
///
/// This is an alias for [`char_len`] to align with `knifer-go`'s `Length` API
/// name.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::length("你好"), 2);
/// ```
#[must_use]
pub fn length(input: &str) -> usize {
    char_len(input)
}

/// Returns the number of Unicode scalar values in `input`.
///
/// This is an alias for [`char_len`] to align with `knifer-go`'s `RuneLen` API
/// name.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::rune_len("你好"), 2);
/// ```
#[must_use]
pub fn rune_len(input: &str) -> usize {
    char_len(input)
}

fn normalize_index(index: isize, len: usize) -> usize {
    let len = isize::try_from(len).unwrap_or(isize::MAX);
    let normalized = if index < 0 { len + index } else { index };
    usize::try_from(normalized.clamp(0, len)).unwrap_or(usize::MAX)
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

fn is_regex_meta(ch: char) -> bool {
    matches!(
        ch,
        '\\' | '.'
            | '+'
            | '*'
            | '?'
            | '('
            | ')'
            | '|'
            | '['
            | ']'
            | '{'
            | '}'
            | '^'
            | '$'
            | '#'
            | '&'
            | '-'
            | '~'
    )
}

fn prefix_end_ignore_case(input: &str, prefix: &str) -> Option<usize> {
    if prefix.is_empty() {
        return Some(0);
    }

    let mut input_chars = input.char_indices();
    for prefix_ch in prefix.chars() {
        let (_, input_ch) = input_chars.next()?;
        if !chars_equal_ignore_case(input_ch, prefix_ch) {
            return None;
        }
    }

    Some(input_chars.next().map_or(input.len(), |(index, _)| index))
}

fn suffix_start_ignore_case(input: &str, suffix: &str) -> Option<usize> {
    if suffix.is_empty() {
        return Some(input.len());
    }

    let mut input_chars = input.char_indices().rev();
    let mut start = None;
    for suffix_ch in suffix.chars().rev() {
        let (index, input_ch) = input_chars.next()?;
        if !chars_equal_ignore_case(input_ch, suffix_ch) {
            return None;
        }
        start = Some(index);
    }

    start
}
