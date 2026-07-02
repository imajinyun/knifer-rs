//! fzf-style fuzzy subsequence matching and scoring.
//!
//! These helpers rank how well a short `pattern` fuzzily matches a longer
//! `text`, using the greedy algorithm and scoring model popularized by `fzf`'s
//! `FuzzyMatchV1`: a forward/backward subsequence scan followed by a single
//! scoring pass that rewards matches at word boundaries, `camelCase`
//! transitions, and consecutive runs while penalizing gaps.
//!
//! Matching uses fzf "smart case": the comparison is case-insensitive unless
//! `pattern` contains an uppercase character, in which case it is
//! case-sensitive. Case folding is a simple 1:1 fold on the first lowercase
//! scalar, which is sufficient for interactive ranking.

/// fzf scoring constants (`FuzzyMatchV1`).
const SCORE_MATCH: i32 = 16;
const SCORE_GAP_START: i32 = -3;
const SCORE_GAP_EXTENSION: i32 = -1;
const BONUS_BOUNDARY: i32 = SCORE_MATCH / 2;
const BONUS_NON_WORD: i32 = SCORE_MATCH;
const BONUS_CAMEL123: i32 = BONUS_BOUNDARY + SCORE_GAP_EXTENSION;
const BONUS_CONSECUTIVE: i32 = -(SCORE_GAP_START + SCORE_GAP_EXTENSION);
const BONUS_FIRST_CHAR_MULTIPLIER: i32 = 2;

/// Returns whether `pattern` fuzzily matches `text` as an ordered subsequence.
///
/// An empty pattern always matches. Matching uses fzf smart case (see the
/// module docs). This is faster than [`fuzzy_score`] when only a yes/no answer
/// is needed.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert!(vstr::fuzzy_match("src/main.rs", "smain"));
/// assert!(vstr::fuzzy_match("FooBarBaz", "fbb"));
/// assert!(!vstr::fuzzy_match("abc", "abcd"));
/// // Smart case: an uppercase pattern char forces a case-sensitive match.
/// assert!(!vstr::fuzzy_match("readme", "R"));
/// ```
#[must_use]
pub fn fuzzy_match(text: &str, pattern: &str) -> bool {
    let case_sensitive = is_case_sensitive(pattern);
    let mut pattern_chars = pattern.chars();
    let Some(first) = pattern_chars.next() else {
        return true;
    };

    let mut needle = fold(first, case_sensitive);
    for ch in text.chars() {
        if fold(ch, case_sensitive) == needle {
            match pattern_chars.next() {
                Some(next) => needle = fold(next, case_sensitive),
                None => return true,
            }
        }
    }
    false
}

/// Returns the fzf-style match score, or `None` when `pattern` does not match.
///
/// An empty pattern scores `0`. Higher scores indicate stronger matches:
/// boundary, `camelCase`, and consecutive-run matches earn bonuses while gaps
/// are penalized. Matching uses fzf smart case (see the module docs).
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// // A boundary match ("main" starting a path segment) beats a scattered one.
/// let boundary = vstr::fuzzy_score("src/main.rs", "main").unwrap();
/// let scattered = vstr::fuzzy_score("maintainer", "main").unwrap();
/// assert!(boundary >= scattered);
/// assert_eq!(vstr::fuzzy_score("abc", "xyz"), None);
/// assert_eq!(vstr::fuzzy_score("abc", ""), Some(0));
/// ```
#[must_use]
pub fn fuzzy_score(text: &str, pattern: &str) -> Option<i32> {
    fuzzy_indices(text, pattern).map(|(score, _)| score)
}

/// Returns the fzf-style score and the byte offsets of the matched characters.
///
/// Returns `None` when `pattern` does not match. An empty pattern returns
/// `Some((0, vec![]))`. Each returned index is a byte offset in `text` that
/// lands on a Unicode scalar boundary, in ascending order, suitable for
/// highlighting matched characters. Matching uses fzf smart case (see the
/// module docs).
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// let (score, indices) = vstr::fuzzy_indices("FooBar", "fb").unwrap();
/// assert!(score > 0);
/// assert_eq!(indices, vec![0, 3]);
/// assert_eq!(vstr::fuzzy_indices("abc", "z"), None);
/// ```
#[must_use]
pub fn fuzzy_indices(text: &str, pattern: &str) -> Option<(i32, Vec<usize>)> {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    if pattern_chars.is_empty() {
        return Some((0, Vec::new()));
    }

    let case_sensitive = is_case_sensitive(pattern);
    let text_chars: Vec<(usize, char)> = text.char_indices().collect();

    // Forward pass: find the earliest subsequence and the end just past it.
    let mut pidx = 0;
    let mut start = None;
    let mut end = None;
    for (position, &(_, ch)) in text_chars.iter().enumerate() {
        if fold(ch, case_sensitive) == fold(pattern_chars[pidx], case_sensitive) {
            if start.is_none() {
                start = Some(position);
            }
            pidx += 1;
            if pidx == pattern_chars.len() {
                end = Some(position + 1);
                break;
            }
        }
    }

    let mut start = start?;
    let end = end?;

    // Backward pass: tighten the start so the span is as short as possible.
    let mut pidx = pattern_chars.len() - 1;
    for position in (start..end).rev() {
        let ch = text_chars[position].1;
        if fold(ch, case_sensitive) == fold(pattern_chars[pidx], case_sensitive) {
            if pidx == 0 {
                start = position;
                break;
            }
            pidx -= 1;
        }
    }

    Some(calculate_score(
        &text_chars,
        &pattern_chars,
        start,
        end,
        case_sensitive,
    ))
}

/// Scores the matched span `[start, end)` and records matched byte offsets.
fn calculate_score(
    text_chars: &[(usize, char)],
    pattern: &[char],
    start: usize,
    end: usize,
    case_sensitive: bool,
) -> (i32, Vec<usize>) {
    let mut pidx = 0;
    let mut score = 0;
    let mut in_gap = false;
    let mut consecutive = 0;
    let mut first_bonus = 0;
    let mut positions = Vec::with_capacity(pattern.len());

    let mut prev_class = if start > 0 {
        char_class(text_chars[start - 1].1)
    } else {
        CharClass::NonWord
    };

    for &(byte_index, ch) in &text_chars[start..end] {
        let class = char_class(ch);
        if fold(ch, case_sensitive) == fold(pattern[pidx], case_sensitive) {
            positions.push(byte_index);
            score += SCORE_MATCH;

            let mut bonus = bonus_for(prev_class, class);
            if consecutive == 0 {
                first_bonus = bonus;
            } else {
                if bonus >= BONUS_BOUNDARY && bonus > first_bonus {
                    first_bonus = bonus;
                }
                bonus = bonus.max(first_bonus).max(BONUS_CONSECUTIVE);
            }

            if pidx == 0 {
                score += bonus * BONUS_FIRST_CHAR_MULTIPLIER;
            } else {
                score += bonus;
            }

            in_gap = false;
            consecutive += 1;
            pidx += 1;
        } else {
            score += if in_gap {
                SCORE_GAP_EXTENSION
            } else {
                SCORE_GAP_START
            };
            in_gap = true;
            consecutive = 0;
            first_bonus = 0;
        }
        prev_class = class;
    }

    (score, positions)
}

/// Character classes used to award fzf boundary and `camelCase` bonuses.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CharClass {
    NonWord,
    Lower,
    Upper,
    Number,
}

fn char_class(ch: char) -> CharClass {
    if ch.is_alphabetic() {
        if ch.is_uppercase() {
            CharClass::Upper
        } else {
            CharClass::Lower
        }
    } else if ch.is_numeric() {
        CharClass::Number
    } else {
        CharClass::NonWord
    }
}

/// Awards an edge-triggered bonus based on the previous and current classes.
fn bonus_for(prev: CharClass, class: CharClass) -> i32 {
    if prev == CharClass::NonWord && class != CharClass::NonWord {
        BONUS_BOUNDARY
    } else if (prev == CharClass::Lower && class == CharClass::Upper)
        || (prev != CharClass::Number && class == CharClass::Number)
    {
        BONUS_CAMEL123
    } else if class == CharClass::NonWord {
        BONUS_NON_WORD
    } else {
        0
    }
}

/// fzf smart case: case-sensitive only when the pattern has an uppercase char.
fn is_case_sensitive(pattern: &str) -> bool {
    pattern.chars().any(char::is_uppercase)
}

/// Folds `ch` for case-insensitive comparison unless `case_sensitive` is set.
fn fold(ch: char, case_sensitive: bool) -> char {
    if case_sensitive {
        ch
    } else {
        ch.to_lowercase().next().unwrap_or(ch)
    }
}
