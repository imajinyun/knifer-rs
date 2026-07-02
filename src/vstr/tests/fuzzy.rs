use crate::vstr::*;

#[test]
fn fuzzy_match_respects_subsequence_and_smart_case() {
    assert!(fuzzy_match("src/main.rs", "smain"));
    assert!(fuzzy_match("FooBarBaz", "fbb"));
    assert!(fuzzy_match("anything", ""));
    assert!(fuzzy_match("café", "cf"));

    // Order matters and every pattern char must appear.
    assert!(!fuzzy_match("abc", "acb"));
    assert!(!fuzzy_match("abc", "abcd"));

    // Smart case: lowercase pattern is case-insensitive; uppercase is strict.
    assert!(fuzzy_match("README", "readme"));
    assert!(fuzzy_match("README", "RE"));
    assert!(!fuzzy_match("readme", "R"));
}

#[test]
fn fuzzy_score_handles_empty_and_no_match() {
    assert_eq!(fuzzy_score("abc", ""), Some(0));
    assert_eq!(fuzzy_score("", "a"), None);
    assert_eq!(fuzzy_score("abc", "xyz"), None);
    assert!(fuzzy_score("abc", "abc").unwrap() > 0);
}

#[test]
fn fuzzy_indices_report_scalar_boundaries_in_order() {
    let (score, indices) = fuzzy_indices("FooBar", "fb").unwrap();
    assert!(score > 0);
    assert_eq!(indices, vec![0, 3]);

    // Multi-byte text: indices are byte offsets on scalar boundaries.
    // "café_menu": é occupies bytes 3-4, so '_' is byte 5 and 'm' is byte 6.
    let (_, indices) = fuzzy_indices("café_menu", "cm").unwrap();
    assert_eq!(indices, vec![0, 6]);
    for &index in &indices {
        assert!("café_menu".is_char_boundary(index));
    }

    // The backward pass tightens the span to the last valid subsequence.
    let (_, indices) = fuzzy_indices("aabaa", "aa").unwrap();
    assert_eq!(indices, vec![0, 1]);

    assert!(fuzzy_indices("abc", "z").is_none());
    assert_eq!(fuzzy_indices("abc", ""), Some((0, Vec::new())));
}

#[test]
fn fuzzy_score_prefers_boundary_and_consecutive_matches() {
    // Boundary hit ("/main") outranks a mid-word scatter.
    let boundary = fuzzy_score("src/main.rs", "main").unwrap();
    let scattered = fuzzy_score("maintainer", "main").unwrap();
    assert!(boundary >= scattered);

    // camelCase transitions score better than an all-lowercase blob.
    let camel = fuzzy_score("FooBarBaz", "fbb").unwrap();
    let blob = fuzzy_score("foobarbaz", "fbb").unwrap();
    assert!(camel >= blob);

    // Consecutive characters beat gapped ones for the same needle.
    let consecutive = fuzzy_score("abcdef", "abc").unwrap();
    let gapped = fuzzy_score("axbxcx", "abc").unwrap();
    assert!(consecutive > gapped);
}

#[test]
fn fuzzy_score_golden_ranking_is_stable() {
    // fzf-style ranking of a candidate list against the query "app".
    let query = "app";
    let mut ranked: Vec<(&str, i32)> = ["app.rs", "src/app.rs", "happy", "a_p_p", "grep"]
        .into_iter()
        .filter_map(|candidate| fuzzy_score(candidate, query).map(|score| (candidate, score)))
        .collect();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(right.0)));

    let order: Vec<&str> = ranked.iter().map(|(candidate, _)| *candidate).collect();
    assert_eq!(order, vec!["app.rs", "src/app.rs", "a_p_p", "happy"]);
    assert_eq!(fuzzy_score("grep", query), None);
}
