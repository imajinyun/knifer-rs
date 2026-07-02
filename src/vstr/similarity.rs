use std::collections::BTreeSet;

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
