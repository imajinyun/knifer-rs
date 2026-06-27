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
