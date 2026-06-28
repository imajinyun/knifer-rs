use crate::vstr::*;

#[cfg(feature = "pattern-regex")]
#[test]
fn pattern_regex_helpers_wrap_regex_matching_and_errors() {
    assert!(contains_pattern("ticket-42", r"\d+").unwrap());
    assert!(!contains_pattern("ticket", r"\d+").unwrap());
    assert_eq!(find_pattern("ticket-42", r"\d+").unwrap(), Some((7, 9)));
    assert_eq!(find_pattern("ticket", r"\d+").unwrap(), None);
    assert_eq!(
        find_all_patterns("a1 b22 c333", r"\d+").unwrap(),
        vec![(1, 2), (4, 6), (8, 11)]
    );
    assert_eq!(
        replace_pattern("ticket-42 user-7", r"\d+", "#").unwrap(),
        "ticket-# user-#"
    );
    assert_eq!(
        replace_pattern("2026-06-27", r"(\d{4})-(\d{2})-(\d{2})", "$2/$3/$1").unwrap(),
        "06/27/2026"
    );

    let err = contains_pattern("ticket-42", "[").unwrap_err();
    assert_eq!(err.pattern(), "[");
    assert!(!err.message().is_empty());
    assert!(err.to_string().contains("invalid regex pattern"));
}

#[cfg(feature = "pattern-regex")]
#[test]
fn pattern_regex_golden_cases_cover_unicode_empty_and_multibyte_ranges() {
    assert!(contains_pattern("编号-٤٢", r"\p{Nd}+").unwrap());
    assert_eq!(find_pattern("你好Rust", r"\p{Han}+").unwrap(), Some((0, 6)));
    assert_eq!(
        find_all_patterns("a你好b世界", r"\p{Han}").unwrap(),
        vec![(1, 4), (4, 7), (8, 11), (11, 14)]
    );
    assert_eq!(
        replace_pattern("你好 Rust 世界", r"\p{Han}+", "[$0]").unwrap(),
        "[你好] Rust [世界]"
    );

    assert_eq!(
        find_all_patterns("ab", r"").unwrap(),
        vec![(0, 0), (1, 1), (2, 2)]
    );
    assert_eq!(replace_pattern("ab", r"", "|").unwrap(), "|a|b|");
    assert_eq!(
        replace_pattern(
            "abc",
            r"(?P<head>a)(?P<body>b)(?P<tail>c)",
            "$tail-$head-$body"
        )
        .unwrap(),
        "c-a-b"
    );

    for pattern in ["[", r"(?P<dup>a)(?P<dup>b)", r"(?P<bad>a", r"\p{NotAClass}"] {
        let err = find_all_patterns("abc", pattern).unwrap_err();
        assert_eq!(err.pattern(), pattern);
        assert!(!err.message().is_empty());
    }
}
