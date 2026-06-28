use knifer_rs::vstr::{MatchKind, VStrMatch, VStrMatcher};

const SEEDS: &str = include_str!("../corpus/matcher.txt");

fn corpus() -> [(&'static str, &'static [&'static str]); 10] {
    [
        ("", &["a", "aa"]),
        ("aaaa", &["a", "aa"]),
        ("hello rust go", &["go", "rust"]),
        ("你好你好", &["你", "你好"]),
        ("emoji 🚀🚀", &["🚀", "🚀🚀"]),
        ("e\u{301}clair", &["e", "e\u{301}", "\u{301}"]),
        ("skip-empty", &["", "skip", "-"]),
        ("a--a__a", &["a", "--", "_"]),
        ("👨\u{200d}👩 family", &["👨", "👨\u{200d}", "family"]),
        ("mixed 你好 rust 🚀", &["mixed", "你好", "rust", "🚀"]),
    ]
}

fn main() {
    let replacements = ["", "A", "AA", "N", "R", "-", "_", "X"];

    for (input, needles) in corpus() {
        assert_matcher_invariants(input, needles, replacements);
    }

    let seed_needles = ["a", "aa", "你", "你好", "🚀", "e", "\u{301}", "--", "_"];
    for input in SEEDS.lines() {
        assert_matcher_invariants(input, &seed_needles, replacements);
    }
}

fn assert_matcher_invariants(input: &str, needles: &[&str], replacements: [&str; 8]) {
    for kind in [MatchKind::LeftmostFirst, MatchKind::LeftmostLongest] {
        let matcher = VStrMatcher::with_kind(needles.iter().copied(), kind);

        assert_eq!(
            matcher.len(),
            needles.iter().filter(|needle| !needle.is_empty()).count()
        );
        assert_eq!(matcher.is_empty(), needles.iter().all(|needle| needle.is_empty()));

        let all = matcher.find_all(input);
        assert_non_overlapping(input, &all);
        assert_eq!(matcher.find(input), all.first().cloned());

        let overlapping = matcher.find_overlapping(input);
        assert_sorted_by_start(&overlapping);
        for matched in &overlapping {
            assert_valid_match(input, matched);
            assert!(input[matched.start..matched.end] == *matched.needle);
        }

        for matched in &all {
            assert_valid_match(input, matched);
            assert!(input[matched.start..matched.end] == *matched.needle);
        }

        let replaced = matcher.replace_all(input, replacements);
        assert!(replaced.is_char_boundary(0) && replaced.is_char_boundary(replaced.len()));
    }
}

fn assert_valid_match(input: &str, matched: &VStrMatch<'_>) {
    assert!(input.is_char_boundary(matched.start));
    assert!(input.is_char_boundary(matched.end));
    assert!(matched.start < matched.end);
    assert!(matched.end <= input.len());
}

fn assert_non_overlapping(input: &str, matches: &[VStrMatch<'_>]) {
    let mut previous_end = 0;
    for matched in matches {
        assert_valid_match(input, matched);
        assert!(previous_end <= matched.start);
        previous_end = matched.end;
    }
}

fn assert_sorted_by_start(matches: &[VStrMatch<'_>]) {
    for pair in matches.windows(2) {
        assert!(pair[0].start <= pair[1].start);
    }
}
