use super::support::*;
use crate::vstr::*;

#[test]
fn knifer_go_vstr_golden_fixtures_cover_similarity() {
    assert_approx_eq(jaccard_similarity("abc", "bcd"), 0.5);
    assert_approx_eq(ngram_similarity("abcd", "abef", 2), 0.2);
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_approx_eq(levenshtein_similarity("你好世界", "你好呀"), 0.5);
    assert_eq!(hamming_distance64(0, u64::MAX), 64);
    assert_ne!(sim_hash("go knifer"), 0);
}

#[test]
fn jaccard_similarity_uses_non_whitespace_char_sets() {
    assert_approx_eq(jaccard_similarity("abc", "abc"), 1.0);
    assert_approx_eq(jaccard_similarity("ab", "bc"), 1.0 / 3.0);
    assert_approx_eq(jaccard_similarity("", " \n"), 1.0);
    assert_approx_eq(jaccard_similarity("你 好", "你好"), 1.0);
}

#[test]
fn ngram_similarity_uses_non_whitespace_ngram_sets() {
    assert_approx_eq(ngram_similarity("abcd", "abce", 2), 0.5);
    assert_approx_eq(ngram_similarity("a b c d", "abcd", 2), 1.0);
    assert_approx_eq(ngram_similarity("abc", "abc", 0), 0.0);
    assert_approx_eq(ngram_similarity("", " ", 2), 1.0);
    assert_approx_eq(ngram_similarity("ab", "ac", 3), 0.0);
    assert_approx_eq(ngram_similarity("ab", "ab", 3), 1.0);
}

#[test]
fn levenshtein_helpers_are_unicode_aware() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("你好", "您好"), 1);
    assert_eq!(levenshtein_distance("", "abc"), 3);
    assert_approx_eq(levenshtein_similarity("", ""), 1.0);
    assert_approx_eq(levenshtein_similarity("kitten", "sitting"), 4.0 / 7.0);
}

#[test]
fn sim_hash_is_deterministic_and_case_insensitive() {
    assert_eq!(sim_hash(""), 0);
    assert_eq!(sim_hash(" \n\t"), 0);
    assert_eq!(sim_hash("Rust"), sim_hash("rust"));
    assert_eq!(sim_hash("hello world"), sim_hash("HELLO   WORLD"));
    assert_ne!(sim_hash("hello world"), sim_hash("different text"));
}

#[test]
fn hamming_distance_counts_different_bits() {
    assert_eq!(hamming_distance64(0b1010, 0b0011), 2);
    assert_eq!(hamming_distance64(0, 0), 0);
    assert_eq!(hamming_distance64(u64::MAX, 0), 64);
}

#[test]
fn optimal_string_alignment_matches_strsim_reference() {
    // Cross-checked against the `strsim` crate reference values.
    assert_eq!(optimal_string_alignment("", ""), 0);
    assert_eq!(optimal_string_alignment("", "abc"), 3);
    assert_eq!(optimal_string_alignment("abc", ""), 3);
    assert_eq!(optimal_string_alignment("ac", "ca"), 1);
    assert_eq!(optimal_string_alignment("ca", "abc"), 3);
    assert_eq!(optimal_string_alignment("kitten", "sitting"), 3);
    assert_eq!(optimal_string_alignment("你好世界", "你好呀"), 2);
}

#[test]
fn damerau_levenshtein_allows_repeated_transpositions() {
    // OSA reports 3 for "ca" -> "abc"; unrestricted Damerau reports 2.
    assert_eq!(damerau_levenshtein_distance("ca", "abc"), 2);
    assert_eq!(damerau_levenshtein_distance("", ""), 0);
    assert_eq!(damerau_levenshtein_distance("", "abc"), 3);
    assert_eq!(damerau_levenshtein_distance("abc", ""), 3);
    assert_eq!(damerau_levenshtein_distance("ac", "ca"), 1);
    assert_eq!(damerau_levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(damerau_levenshtein_distance("你好", "您好"), 1);
}

#[test]
fn jaro_matches_strsim_reference() {
    assert_approx_eq(jaro_similarity("", ""), 1.0);
    assert_approx_eq(jaro_similarity("abc", ""), 0.0);
    assert_approx_eq(jaro_similarity("abc", "abc"), 1.0);
    assert!((jaro_similarity("dwayne", "duane") - 0.822_222_222_222_222).abs() < 1e-12);
    assert!((jaro_similarity("martha", "marhta") - 0.944_444_444_444_444).abs() < 1e-12);
    assert!((jaro_similarity("dixon", "dicksonx") - 0.766_666_666_666_666).abs() < 1e-12);
}

#[test]
fn jaro_winkler_boosts_common_prefix() {
    assert_approx_eq(jaro_winkler_similarity("", ""), 1.0);
    assert_approx_eq(jaro_winkler_similarity("abc", "abc"), 1.0);
    assert!((jaro_winkler_similarity("dwayne", "duane") - 0.840).abs() < 1e-3);
    assert!((jaro_winkler_similarity("martha", "marhta") - 0.961_111).abs() < 1e-5);
    // No common prefix: Jaro-Winkler equals Jaro.
    assert_approx_eq(
        jaro_winkler_similarity("aaa", "bbb"),
        jaro_similarity("aaa", "bbb"),
    );
}

#[test]
fn sorensen_dice_compares_bigram_multisets() {
    assert_approx_eq(sorensen_dice("", ""), 1.0);
    assert_approx_eq(sorensen_dice("night", "night"), 1.0);
    assert_approx_eq(sorensen_dice("night", "nacht"), 0.25);
    assert_approx_eq(sorensen_dice("a", "a"), 1.0);
    // Single characters have no bigrams and are unequal.
    assert_approx_eq(sorensen_dice("a", "b"), 0.0);
    assert_approx_eq(sorensen_dice("night", "day"), 0.0);
}
