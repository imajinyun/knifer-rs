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
