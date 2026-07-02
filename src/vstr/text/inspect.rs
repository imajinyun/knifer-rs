/// Returns text lines using [`str::lines`] semantics.
///
/// Empty input has no lines and a trailing newline does not produce an extra
/// empty line.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::lines("a\nb\n"), vec!["a", "b"]);
/// ```
#[must_use]
pub fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Returns non-blank text lines after trimming Unicode whitespace.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::non_blank_lines(" a \n\n b "), vec!["a", "b"]);
/// ```
#[must_use]
pub fn non_blank_lines(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

/// Returns Unicode-whitespace separated words.
///
/// This is the dependency-free tokenizer: it splits on Unicode whitespace only
/// (via [`str::split_whitespace`]) and never breaks inside a whitespace-free
/// run. Punctuation stays attached to the surrounding word, and scripts without
/// spaces (such as CJK) are returned as a single token. For UAX #29
/// word-boundary tokenization that splits punctuation and CJK, use the
/// feature-gated `unicode_words` (enabled by the `unicode-segmentation`
/// feature).
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::words("hello  Rust\n世界"), vec!["hello", "Rust", "世界"]);
/// // Whitespace-based: punctuation stays attached and CJK stays as one token.
/// assert_eq!(vstr::words("Rust-go, 世界!"), vec!["Rust-go,", "世界!"]);
/// ```
#[must_use]
pub fn words(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

/// Returns upper-cased initials from Unicode-whitespace separated words.
///
/// Empty words are ignored.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::initials("rust string toolkit"), "RST");
/// assert_eq!(vstr::initials("你好 rust"), "你R");
/// ```
#[must_use]
pub fn initials(input: &str) -> String {
    let mut output = String::new();
    for word in input.split_whitespace() {
        if let Some(ch) = word.chars().next() {
            output.extend(ch.to_uppercase());
        }
    }
    output
}

/// Splits `input` into Unicode scalar values.
///
/// For grapheme-cluster segmentation, use a dedicated Unicode segmentation
/// crate. `knifer-rs` keeps this MVP helper dependency-free.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::chars("a你"), vec!['a', '你']);
/// ```
#[must_use]
pub fn chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

/// Returns `true` when `input` is a palindrome after light text cleanup.
///
/// Only Unicode letters and decimal digits participate in the comparison.
/// Whitespace and punctuation are ignored. Case comparison follows
/// `knifer-rs`'s simple case-insensitive scalar behavior.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert!(vstr::is_palindrome("A man, a plan, a canal: Panama"));
/// assert!(vstr::is_palindrome("上海自来水来自海上"));
/// assert!(!vstr::is_palindrome("knifer-rs"));
/// ```
#[must_use]
pub fn is_palindrome(input: &str) -> bool {
    let chars: Vec<char> = input
        .chars()
        .filter(|ch| super::super::is_letter_or_digit(*ch))
        .collect();

    chars
        .iter()
        .zip(chars.iter().rev())
        .take(chars.len() / 2)
        .all(|(left, right)| {
            super::super::equals_ignore_case(&left.to_string(), &right.to_string())
        })
}

/// Extracts all Unicode decimal digits from `input`.
///
/// This follows the same decimal-digit definition as [`crate::vstr::is_digit`].
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::extract_digits("id=42, رقم=٣"), "42٣");
/// ```
#[must_use]
pub fn extract_digits(input: &str) -> String {
    input
        .chars()
        .filter(|ch| super::super::is_digit(*ch))
        .collect()
}

/// Removes ASCII punctuation from `input`.
///
/// Unicode punctuation is preserved so the helper stays small and predictable
/// without Unicode data dependencies.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::remove_ascii_punctuation("Hello, Rust! 你好，世界！"), "Hello Rust 你好，世界！");
/// ```
#[must_use]
pub fn remove_ascii_punctuation(input: &str) -> String {
    input
        .chars()
        .filter(|ch| !ch.is_ascii_punctuation())
        .collect()
}

/// Counts Unicode-whitespace separated words.
///
/// Uses the same whitespace-only tokenization as [`words`], so it counts
/// whitespace-free runs, not UAX #29 word boundaries. For a boundary-aware
/// count, use the feature-gated `unicode_word_len` (enabled by the
/// `unicode-segmentation` feature).
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::word_count("hello  Rust\n世界"), 3);
/// ```
#[must_use]
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

/// Counts text lines.
///
/// Empty input has zero lines. A trailing newline does not add an extra line,
/// matching [`str::lines`].
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::line_count("a\nb\n"), 2);
/// ```
#[must_use]
pub fn line_count(input: &str) -> usize {
    input.lines().count()
}
