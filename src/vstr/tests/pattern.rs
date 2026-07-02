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

#[cfg(feature = "pattern-regex")]
#[test]
fn vregex_reuses_a_compiled_pattern_across_inputs() {
    let digits = VRegex::new(r"\d+").unwrap();
    assert_eq!(digits.as_str(), r"\d+");
    assert_eq!(digits.capture_count(), 1);

    assert!(digits.is_match("ticket-42"));
    assert!(!digits.is_match("ticket"));
    assert_eq!(digits.find("ticket-42"), Some((7, 9)));
    assert_eq!(digits.find("ticket"), None);
    assert_eq!(
        digits.find_all("a1 b22 c333"),
        vec![(1, 2), (4, 6), (8, 11)]
    );
    assert_eq!(
        digits.replace_all("ticket-42 user-7", "#"),
        "ticket-# user-#"
    );
}

#[cfg(feature = "pattern-regex")]
#[test]
fn vregex_captures_report_group_byte_ranges_and_missing_groups() {
    let date = VRegex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    assert_eq!(date.capture_count(), 4);
    assert_eq!(
        date.captures("2026-06-27"),
        Some(vec![
            Some((0, 10)),
            Some((0, 4)),
            Some((5, 7)),
            Some((8, 10))
        ])
    );
    assert_eq!(date.captures("not-a-date"), None);
    assert_eq!(date.replace_all("2026-06-27", "$2/$3/$1"), "06/27/2026");

    let optional = VRegex::new(r"(a)(b)?c").unwrap();
    assert_eq!(
        optional.captures("ac"),
        Some(vec![Some((0, 2)), Some((0, 1)), None])
    );
}

#[cfg(feature = "pattern-regex")]
#[test]
fn vregex_matches_unicode_empty_and_multibyte_inputs() {
    let han = VRegex::new(r"\p{Han}+").unwrap();
    assert_eq!(han.find("你好Rust"), Some((0, 6)));
    assert_eq!(
        han.replace_all("你好 Rust 世界", "[$0]"),
        "[你好] Rust [世界]"
    );

    let empty = VRegex::new(r"").unwrap();
    assert_eq!(empty.find_all("ab"), vec![(0, 0), (1, 1), (2, 2)]);
    assert_eq!(empty.replace_all("ab", "|"), "|a|b|");

    assert_eq!(VRegex::new("[").unwrap_err().pattern(), "[");
    assert!(!VRegex::new("[").unwrap_err().message().is_empty());
}
