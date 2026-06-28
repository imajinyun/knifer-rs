use super::support::*;
use crate::vstr::*;

#[test]
fn property_style_substring_helpers_keep_scalar_boundaries() {
    let mut rng = DeterministicRng::new(0x5eed_0001);

    for _ in 0..256 {
        let input = rng.string(24);
        let len = input.chars().count();
        let from = rng.index_around(len);
        let to = rng.index_around(len);

        let actual = sub(&input, from, to);
        let expected = expected_sub(&input, from, to);
        assert_eq!(actual, expected);

        let take_count = rng.usize(len + 6);
        let taken = take_chars(&input, take_count);
        assert!(input.starts_with(taken));
        assert_eq!(taken.chars().count(), take_count.min(len));

        let drop_count = rng.usize(len + 6);
        let dropped = drop_chars(&input, drop_count);
        assert!(input.ends_with(dropped));
        assert_eq!(dropped.chars().count(), len.saturating_sub(drop_count));
    }
}

#[test]
fn property_style_replacement_and_escaping_helpers_are_stable() {
    let mut rng = DeterministicRng::new(0x5eed_0002);
    let replacements = [("a", "A"), ("你", "N"), ("🚀", "R"), ("--", "-")];

    for _ in 0..256 {
        let input = rng.string(32);
        let replaced = replace_many(&input, replacements);

        assert!(!replaced.contains('a'));
        assert!(!replaced.contains('你'));
        assert!(!replaced.contains('🚀'));
        assert_eq!(replace_many(&input, [("", "x")]), input);

        let escaped = escape_regex(&input);
        assert_eq!(escaped, quote_meta(&input));
        assert!(escaped.chars().count() >= input.chars().count());
        for ch in ".+*?^$()[]{}|\\".chars() {
            if input.contains(ch) {
                assert!(escaped.contains(&format!("\\{ch}")));
            }
        }
    }
}

#[test]
fn property_style_reusable_matcher_preserves_match_contracts() {
    let mut rng = DeterministicRng::new(0x5eed_0005);
    let candidates = ["", "a", "aa", "你", "你好", "🚀", "e", "\u{301}", "--", "_"];
    let replacements = ["", "A", "AA", "N", "NH", "R", "E", "'", "-", "U"];

    for _ in 0..256 {
        let input = rng.string(36);
        let mut needles = Vec::new();
        for _ in 0..rng.usize(8).saturating_add(1) {
            needles.push(candidates[rng.usize(candidates.len())]);
        }

        for kind in [MatchKind::LeftmostFirst, MatchKind::LeftmostLongest] {
            let matcher = VStrMatcher::with_kind(needles.iter().copied(), kind);

            assert_eq!(
                matcher.len(),
                needles.iter().filter(|needle| !needle.is_empty()).count()
            );
            assert_eq!(
                matcher.is_empty(),
                needles.iter().all(|needle| needle.is_empty())
            );
            assert_eq!(
                matcher.find(&input),
                expected_matcher_find(&input, &needles, kind)
            );
            assert_eq!(
                matcher.find_all(&input),
                expected_matcher_find_all(&input, &needles, kind)
            );
            assert_eq!(
                matcher.find_overlapping(&input),
                expected_matcher_find_overlapping(&input, &needles, kind)
            );
            assert_eq!(
                matcher.replace_all(&input, replacements),
                expected_matcher_replace_all(&input, &needles, kind, &replacements)
            );
        }
    }
}

#[test]
fn property_style_ant_path_literal_patterns_match_themselves() {
    let mut rng = DeterministicRng::new(0x5eed_0003);

    for _ in 0..256 {
        let path = rng.path(5);

        assert!(ant_path_match(&path, &path));
        assert!(ant_path_match("/**", &path));
        assert!(ant_path_match_with_separator(
            &path.replace('/', "."),
            &path.replace('/', "."),
            "."
        ));

        if let Some((prefix, _)) = path.rsplit_once('/') {
            let pattern = format!("{prefix}/**");
            assert!(ant_path_match(&pattern, &path));
        }
    }
}

#[cfg(feature = "unicode-width")]
#[test]
fn property_style_unicode_width_helpers_respect_display_boundaries() {
    let mut rng = DeterministicRng::new(0x5eed_0004);
    let fixed_inputs = [
        "",
        "abc",
        "你好世界",
        "e\u{301}clair",
        "👨\u{200d}👩\u{200d}👧\u{200d}👦 family",
        "🇨🇳 flag",
        "a   b\t\tc",
        "supercalifragilistic",
        "🚀🚀go",
        "\u{3000}wide space",
    ];

    for input in fixed_inputs {
        assert_unicode_width_properties(input, 0..=12);
    }

    for _ in 0..256 {
        let input = rng.string(40);
        let max_width = rng.usize(18);

        assert_unicode_width_properties(&input, max_width..=max_width);
    }
}
