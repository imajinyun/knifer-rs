use super::support::*;
use crate::vstr::*;

#[test]
fn reusable_matcher_finds_and_replaces_left_to_right() {
    let matcher = VStrMatcher::new(["go", "rust"]);
    assert_eq!(matcher.kind(), MatchKind::LeftmostFirst);
    assert_eq!(matcher.len(), 2);
    assert!(!matcher.is_empty());
    assert_eq!(
        matcher.find("hello rust go"),
        Some(VStrMatch {
            needle: "rust",
            pattern_index: 1,
            start: 6,
            end: 10,
        })
    );
    assert_eq!(
        matcher.find_all("go rust go"),
        vec![
            VStrMatch {
                needle: "go",
                pattern_index: 0,
                start: 0,
                end: 2,
            },
            VStrMatch {
                needle: "rust",
                pattern_index: 1,
                start: 3,
                end: 7,
            },
            VStrMatch {
                needle: "go",
                pattern_index: 0,
                start: 8,
                end: 10,
            },
        ]
    );
    assert_eq!(
        matcher.replace_all("go rust go", ["rs", "RUST"]),
        "rs RUST rs"
    );
    assert_eq!(VStrMatcher::new([""]).find("rust"), None);
}

#[test]
fn reusable_matcher_uses_leftmost_first_by_default() {
    let leftmost_first = VStrMatcher::new(["a", "aa"]);
    assert_eq!(leftmost_first.find("aaaa").unwrap().needle, "a");
    assert_eq!(
        leftmost_first.find_all("aaaa"),
        vec![
            VStrMatch {
                needle: "a",
                pattern_index: 0,
                start: 0,
                end: 1,
            },
            VStrMatch {
                needle: "a",
                pattern_index: 0,
                start: 1,
                end: 2,
            },
            VStrMatch {
                needle: "a",
                pattern_index: 0,
                start: 2,
                end: 3,
            },
            VStrMatch {
                needle: "a",
                pattern_index: 0,
                start: 3,
                end: 4,
            },
        ]
    );
}

#[test]
fn reusable_matcher_supports_leftmost_longest_and_overlap() {
    let leftmost_longest = VStrMatcher::with_kind(["a", "aa"], MatchKind::LeftmostLongest);
    assert_eq!(leftmost_longest.find("aaaa").unwrap().needle, "aa");
    assert_eq!(
        leftmost_longest.find_all("aaaa"),
        vec![
            VStrMatch {
                needle: "aa",
                pattern_index: 1,
                start: 0,
                end: 2,
            },
            VStrMatch {
                needle: "aa",
                pattern_index: 1,
                start: 2,
                end: 4,
            },
        ]
    );
    assert_eq!(
        leftmost_longest.find_overlapping("aaaa"),
        vec![
            VStrMatch {
                needle: "aa",
                pattern_index: 1,
                start: 0,
                end: 2,
            },
            VStrMatch {
                needle: "aa",
                pattern_index: 1,
                start: 1,
                end: 3,
            },
            VStrMatch {
                needle: "aa",
                pattern_index: 1,
                start: 2,
                end: 4,
            },
            VStrMatch {
                needle: "a",
                pattern_index: 0,
                start: 3,
                end: 4,
            },
        ]
    );
    assert_eq!(leftmost_longest.replace_all("aaaa", ["x"]), "aaaa");
}

#[test]
fn reusable_matcher_backend_parity_matrix_locks_public_semantics() {
    let cases = [
        (
            "empty and duplicate needles",
            "",
            &["", "a", "a"][..],
            &["", "x", "y"][..],
        ),
        (
            "leftmost-first and leftmost-longest tie",
            "aaaa",
            &["a", "aa", "aaa"][..],
            &["x", "y"][..],
        ),
        (
            "multibyte byte offsets",
            "你好你好",
            &["你", "你好", "好你"][..],
            &["N", "NH", "HN"][..],
        ),
        (
            "mixed ascii unicode",
            "go🚀rust🚀go",
            &["🚀", "go", "rust"][..],
            &["R"][..],
        ),
    ];

    for (name, input, needles, replacements) in cases {
        for kind in [MatchKind::LeftmostFirst, MatchKind::LeftmostLongest] {
            let matcher = VStrMatcher::with_kind(needles.iter().copied(), kind);
            assert_eq!(
                matcher.find(input),
                expected_matcher_find(input, needles, kind),
                "{name:?} {kind:?} find"
            );
            assert_eq!(
                matcher.find_all(input),
                expected_matcher_find_all(input, needles, kind),
                "{name:?} {kind:?} find_all"
            );
            assert_eq!(
                matcher.find_overlapping(input),
                expected_matcher_find_overlapping(input, needles, kind),
                "{name:?} {kind:?} find_overlapping"
            );
            assert_eq!(
                matcher.replace_all(input, replacements.iter().copied()),
                expected_matcher_replace_all(input, needles, kind, replacements),
                "{name:?} {kind:?} replace_all"
            );
        }
    }
}
