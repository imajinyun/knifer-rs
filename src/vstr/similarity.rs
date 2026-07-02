use std::collections::{BTreeSet, HashMap};

/// Returns the Jaccard similarity of two strings by Unicode scalar-value set.
///
/// Unicode whitespace is ignored. Two empty effective inputs are considered
/// identical and return `1.0`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::jaccard_similarity("abc", "abc"), 1.0);
/// assert_eq!(vstr::jaccard_similarity("", " \n"), 1.0);
/// ```
#[must_use]
pub fn jaccard_similarity(left: &str, right: &str) -> f64 {
    let left = char_set_without_whitespace(left);
    let right = char_set_without_whitespace(right);
    jaccard(&left, &right)
}

/// Returns the Jaccard similarity of Unicode scalar-value n-gram sets.
///
/// Unicode whitespace is ignored before n-grams are built. If `n` is `0`, the
/// function returns `0.0`. Two empty effective inputs are considered identical.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::ngram_similarity("abcd", "abce", 2), 0.5);
/// assert_eq!(vstr::ngram_similarity("abc", "abc", 0), 0.0);
/// ```
#[must_use]
pub fn ngram_similarity(left: &str, right: &str, n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let left = ngram_set(left, n);
    let right = ngram_set(right, n);
    jaccard(&left, &right)
}

/// Returns the Unicode-aware Levenshtein edit distance between two strings.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(vstr::levenshtein_distance("你好", "您好"), 1);
/// ```
#[must_use]
pub fn levenshtein_distance(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();

    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }

    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current = vec![0; right.len() + 1];

    for (left_index, left_char) in left.iter().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right.iter().enumerate() {
            let cost = usize::from(left_char != right_char);
            current[right_index + 1] = (current[right_index] + 1)
                .min(previous[right_index + 1] + 1)
                .min(previous[right_index] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[right.len()]
}

/// Returns a normalized Levenshtein similarity score in `[0.0, 1.0]`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::levenshtein_similarity("", ""), 1.0);
/// assert!((vstr::levenshtein_similarity("kitten", "sitting") - 4.0 / 7.0).abs() < f64::EPSILON);
/// ```
#[must_use]
pub fn levenshtein_similarity(left: &str, right: &str) -> f64 {
    let left_len = left.chars().count();
    let right_len = right.chars().count();
    let max_len = left_len.max(right_len);
    if max_len == 0 {
        return 1.0;
    }

    let distance = levenshtein_distance(left, right);
    1.0 - (usize_to_f64(distance) / usize_to_f64(max_len))
}

/// Returns the optimal string alignment distance between two strings.
///
/// This is the restricted Damerau-Levenshtein distance: it adds transposition
/// of two adjacent characters to the insert, delete, and substitute edits, but
/// no substring is edited more than once. It is Unicode scalar-value aware.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::optimal_string_alignment("ca", "abc"), 3);
/// assert_eq!(vstr::optimal_string_alignment("ac", "ca"), 1);
/// ```
#[must_use]
pub fn optimal_string_alignment(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();

    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }

    let rows = left.len() + 1;
    let cols = right.len() + 1;
    let mut distances = vec![0_usize; rows * cols];

    for (col, value) in distances.iter_mut().take(cols).enumerate() {
        *value = col;
    }
    for row in 1..rows {
        distances[row * cols] = row;
    }

    for i in 1..rows {
        for j in 1..cols {
            let cost = usize::from(left[i - 1] != right[j - 1]);
            let mut value = (distances[(i - 1) * cols + j] + 1)
                .min(distances[i * cols + (j - 1)] + 1)
                .min(distances[(i - 1) * cols + (j - 1)] + cost);

            if i > 1 && j > 1 && left[i - 1] == right[j - 2] && left[i - 2] == right[j - 1] {
                value = value.min(distances[(i - 2) * cols + (j - 2)] + 1);
            }

            distances[i * cols + j] = value;
        }
    }

    distances[rows * cols - 1]
}

/// Returns the unrestricted Damerau-Levenshtein distance between two strings.
///
/// Unlike [`optimal_string_alignment`], this allows a substring to be edited
/// more than once, matching the full Damerau-Levenshtein metric. It is Unicode
/// scalar-value aware.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::damerau_levenshtein_distance("ca", "abc"), 2);
/// assert_eq!(vstr::damerau_levenshtein_distance("kitten", "sitting"), 3);
/// ```
#[must_use]
pub fn damerau_levenshtein_distance(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();

    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }

    let height = right.len() + 2;
    let mut matrix = vec![0_usize; (left.len() + 2) * height];
    let max_distance = left.len() + right.len();

    matrix[0] = max_distance;
    for i in 0..=left.len() {
        matrix[(i + 1) * height] = max_distance;
        matrix[(i + 1) * height + 1] = i;
    }
    for j in 0..=right.len() {
        matrix[j + 1] = max_distance;
        matrix[height + (j + 1)] = j;
    }

    let mut last_row: HashMap<char, usize> = HashMap::new();

    for i in 1..=left.len() {
        let mut last_match_col = 0;
        for j in 1..=right.len() {
            let last_match_row = *last_row.get(&right[j - 1]).unwrap_or(&0);
            let cost = usize::from(left[i - 1] != right[j - 1]);

            let substitution = matrix[i * height + j] + cost;
            let insertion = matrix[(i + 1) * height + j] + 1;
            let deletion = matrix[i * height + (j + 1)] + 1;
            let transposition = matrix[last_match_row * height + last_match_col]
                + (i - last_match_row - 1)
                + 1
                + (j - last_match_col - 1);

            matrix[(i + 1) * height + (j + 1)] =
                substitution.min(insertion).min(deletion).min(transposition);

            if cost == 0 {
                last_match_col = j;
            }
        }
        last_row.insert(left[i - 1], i);
    }

    matrix[(left.len() + 1) * height + (right.len() + 1)]
}

/// Returns the Jaro similarity of two strings in `[0.0, 1.0]`.
///
/// The Jaro metric rewards matching characters within a sliding window and
/// penalizes transpositions. Two empty inputs are considered identical and
/// return `1.0`. It is Unicode scalar-value aware.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::jaro_similarity("", ""), 1.0);
/// assert!((vstr::jaro_similarity("dwayne", "duane") - 0.822_222).abs() < 1e-5);
/// ```
#[must_use]
pub fn jaro_similarity(left: &str, right: &str) -> f64 {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    jaro_from_chars(&left, &right).0
}

/// Returns the Jaro-Winkler similarity of two strings in `[0.0, 1.0]`.
///
/// This boosts the [`jaro_similarity`] score for strings that share a common
/// prefix (up to four characters), which suits human names well. It is Unicode
/// scalar-value aware.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::jaro_winkler_similarity("", ""), 1.0);
/// assert!((vstr::jaro_winkler_similarity("dwayne", "duane") - 0.840).abs() < 1e-3);
/// ```
#[must_use]
pub fn jaro_winkler_similarity(left: &str, right: &str) -> f64 {
    const SCALING: f64 = 0.1;

    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let (jaro, matches) = jaro_from_chars(&left, &right);

    if matches == 0 {
        return jaro;
    }

    let mut prefix = 0;
    for (a, b) in left.iter().zip(right.iter()).take(4) {
        if a == b {
            prefix += 1;
        } else {
            break;
        }
    }

    jaro + usize_to_f64(prefix) * SCALING * (1.0 - jaro)
}

/// Returns the Sørensen-Dice coefficient of two strings in `[0.0, 1.0]`.
///
/// The coefficient compares the multiset of adjacent character bigrams. Two
/// empty inputs are considered identical and return `1.0`; a single-character
/// input has no bigrams and returns `0.0` unless both sides are equal. It is
/// Unicode scalar-value aware.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::sorensen_dice("night", "night"), 1.0);
/// assert!((vstr::sorensen_dice("night", "nacht") - 0.25).abs() < f64::EPSILON);
/// ```
#[must_use]
pub fn sorensen_dice(left: &str, right: &str) -> f64 {
    if left == right {
        return 1.0;
    }

    let left_bigrams = bigram_counts(left);
    let right_bigrams = bigram_counts(right);
    if left_bigrams.is_empty() || right_bigrams.is_empty() {
        return 0.0;
    }

    let mut intersection = 0_usize;
    for (bigram, left_count) in &left_bigrams {
        if let Some(right_count) = right_bigrams.get(bigram) {
            intersection += (*left_count).min(*right_count);
        }
    }

    let total: usize = left_bigrams.values().sum::<usize>() + right_bigrams.values().sum::<usize>();
    2.0 * usize_to_f64(intersection) / usize_to_f64(total)
}

/// Returns a deterministic 64-bit `SimHash` for text.
///
/// Whitespace-separated lower-cased fields are used as tokens. If the text has
/// no fields, non-space Unicode scalar values are used as fallback tokens. Empty
/// or blank input returns `0`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::sim_hash(""), 0);
/// assert_eq!(vstr::sim_hash("Rust"), vstr::sim_hash("rust"));
/// ```
#[must_use]
pub fn sim_hash(input: &str) -> u64 {
    let tokens = sim_hash_tokens(input);
    if tokens.is_empty() {
        return 0;
    }

    let mut vector = [0_i32; 64];
    for token in tokens {
        let hash = fnv1a64(&token);
        for (index, weight) in vector.iter_mut().enumerate() {
            if hash & (1_u64 << index) != 0 {
                *weight += 1;
            } else {
                *weight -= 1;
            }
        }
    }

    let mut result = 0_u64;
    for (index, weight) in vector.into_iter().enumerate() {
        if weight > 0 {
            result |= 1_u64 << index;
        }
    }
    result
}

/// Returns the number of different bits between two `u64` values.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::hamming_distance64(0b1010, 0b0011), 2);
/// ```
#[must_use]
pub const fn hamming_distance64(left: u64, right: u64) -> u32 {
    (left ^ right).count_ones()
}

fn char_set_without_whitespace(input: &str) -> BTreeSet<char> {
    input.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn ngram_set(input: &str, n: usize) -> BTreeSet<Vec<char>> {
    let chars: Vec<char> = input.chars().filter(|ch| !ch.is_whitespace()).collect();
    if chars.is_empty() {
        return BTreeSet::new();
    }
    if chars.len() < n {
        return BTreeSet::from([chars]);
    }

    chars.windows(n).map(<[char]>::to_vec).collect()
}

fn jaccard<T: Ord>(left: &BTreeSet<T>, right: &BTreeSet<T>) -> f64 {
    if left.is_empty() && right.is_empty() {
        return 1.0;
    }

    let intersection = left.intersection(right).count();
    let union = left.union(right).count();
    usize_to_f64(intersection) / usize_to_f64(union)
}

/// Returns the Jaro similarity and the number of matching characters.
fn jaro_from_chars(left: &[char], right: &[char]) -> (f64, usize) {
    if left.is_empty() && right.is_empty() {
        return (1.0, 0);
    }
    if left.is_empty() || right.is_empty() {
        return (0.0, 0);
    }

    let search_range = (left.len().max(right.len()) / 2).saturating_sub(1);

    let mut left_matched = vec![false; left.len()];
    let mut right_matched = vec![false; right.len()];
    let mut matches = 0_usize;

    for (i, left_char) in left.iter().enumerate() {
        let start = i.saturating_sub(search_range);
        let end = (i + search_range + 1).min(right.len());
        for j in start..end {
            if !right_matched[j] && *left_char == right[j] {
                left_matched[i] = true;
                right_matched[j] = true;
                matches += 1;
                break;
            }
        }
    }

    if matches == 0 {
        return (0.0, 0);
    }

    let mut transpositions = 0_usize;
    let mut right_index = 0;
    for (i, matched) in left_matched.iter().enumerate() {
        if *matched {
            while !right_matched[right_index] {
                right_index += 1;
            }
            if left[i] != right[right_index] {
                transpositions += 1;
            }
            right_index += 1;
        }
    }

    let matches_f = usize_to_f64(matches);
    let half_transpositions = usize_to_f64(transpositions / 2);
    let jaro = (matches_f / usize_to_f64(left.len())
        + matches_f / usize_to_f64(right.len())
        + (matches_f - half_transpositions) / matches_f)
        / 3.0;

    (jaro, matches)
}

/// Returns the multiset of adjacent character bigrams keyed by the pair.
fn bigram_counts(input: &str) -> HashMap<(char, char), usize> {
    let chars: Vec<char> = input.chars().collect();
    let mut counts = HashMap::new();
    for window in chars.windows(2) {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }
    counts
}

#[allow(clippy::cast_precision_loss)]
fn usize_to_f64(value: usize) -> f64 {
    value as f64
}

fn sim_hash_tokens(input: &str) -> Vec<String> {
    let lower = input.to_lowercase();
    let fields: Vec<String> = lower.split_whitespace().map(str::to_owned).collect();
    if !fields.is_empty() {
        return fields;
    }

    lower
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_string())
        .collect()
}

const fn fnv1a64(input: &str) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    let bytes = input.as_bytes();
    let mut hash = FNV_OFFSET_BASIS;
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        index += 1;
    }
    hash
}
