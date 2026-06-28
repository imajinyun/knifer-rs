use super::*;

#[test]
fn empty_and_blank_predicates_distinguish_spaces() {
    assert!(is_empty(""));
    assert!(!is_empty(" "));
    assert!(!is_not_empty(""));
    assert!(is_not_empty(" "));
    assert!(is_blank(" \n\t"));
    assert!(!is_blank("knifer-rs"));
    assert!(!is_not_blank(" \n\t"));
    assert!(is_not_blank("knifer-rs"));
}

#[test]
fn batch_predicates_follow_empty_and_blank_semantics() {
    assert!(has_empty(["a", ""]));
    assert!(!has_empty(["a", " "]));
    assert!(has_blank(["a", " "]));
    assert!(!has_blank(["a", "b"]));
    assert!(is_all_empty(["", ""]));
    assert!(!is_all_empty(["", " "]));
    assert!(is_all_blank(["", " ", "\n"]));
    assert!(!is_all_blank(["", "knifer-rs"]));
}

#[test]
fn trim_removes_ascii_spaces() {
    assert_eq!(trim("  knifer-rs  "), "knifer-rs");
}

#[test]
fn trim_removes_unicode_whitespace() {
    assert_eq!(trim("\n\t knifer-rs \u{3000}"), "knifer-rs");
}

#[test]
fn trim_keeps_inner_whitespace() {
    assert_eq!(trim("  knifer rs  "), "knifer rs");
}

#[test]
fn trim_handles_blank_and_empty_input() {
    assert_eq!(trim("   "), "");
    assert_eq!(trim(""), "");
}

#[test]
fn trim_to_string_returns_owned_value() {
    let input = String::from("  knifer-rs  ");
    let output = trim_to_string(&input);

    assert_eq!(output, "knifer-rs");
    assert_eq!(input, "  knifer-rs  ");
    assert_eq!(trim_to_empty("  knifer-rs  "), "knifer-rs");
    assert_eq!(trim_to_empty("   "), "");
}

#[test]
fn trim_start_and_trim_end_only_remove_one_side() {
    assert_eq!(trim_start("  knifer-rs  "), "knifer-rs  ");
    assert_eq!(trim_end("  knifer-rs  "), "  knifer-rs");
}

#[test]
fn split_returns_empty_vec_for_empty_input() {
    assert_eq!(split("a,b", ","), vec!["a", "b"]);
    assert!(split("", ",").is_empty());
    assert_eq!(split("a,,b", ","), vec!["a", "", "b"]);
}

#[test]
fn split_trim_drops_blank_parts_after_trimming() {
    assert_eq!(split_trim(" a, ,b,\n c ", ","), vec!["a", "b", "c"]);
    assert!(split_trim(" , \n, ", ",").is_empty());
}

#[test]
fn sub_uses_char_indexes_and_normalizes_ranges() {
    assert_eq!(sub("abcdef", 1, 4), "bcd");
    assert_eq!(sub("你好世界", 1, -1), "好世");
    assert_eq!(sub("abcdef", 4, 1), "bcd");
    assert_eq!(sub("abcdef", -20, 20), "abcdef");
    assert_eq!(sub("", 0, 1), "");
    assert_eq!(sub("abcdef", 2, 2), "");
}

#[test]
fn separator_substrings_use_first_or_last_match() {
    assert_eq!(sub_before("a/b/c", "/", false), "a");
    assert_eq!(sub_before("a/b/c", "/", true), "a/b");
    assert_eq!(sub_before("a/b/c", "|", false), "a/b/c");
    assert_eq!(sub_before("a/b/c", "", false), "a/b/c");
    assert_eq!(sub_after("a/b/c", "/", false), "b/c");
    assert_eq!(sub_after("a/b/c", "/", true), "c");
    assert_eq!(sub_after("a/b/c", "|", false), "");
    assert_eq!(sub_after("a/b/c", "", false), "");
}

#[test]
fn concise_substring_aliases_and_between_borrow_original_input() {
    assert_eq!(before("a/b/c", "/"), "a");
    assert_eq!(before("a/b/c", "|"), "a/b/c");
    assert_eq!(before_last("a/b/c", "/"), "a/b");
    assert_eq!(after("a/b/c", "/"), "b/c");
    assert_eq!(after("a/b/c", "|"), "");
    assert_eq!(after_last("a/b/c", "/"), "c");
    assert_eq!(between("id=[42]", "[", "]"), Some("42"));
    assert_eq!(between("a<你好>b", "<", ">"), Some("你好"));
    assert_eq!(between("id=42", "[", "]"), None);
    assert_eq!(between("id=[]", "[", "]"), Some(""));
    assert_eq!(between("id=[42]", "", "]"), None);
}

#[test]
fn repeat_and_pad_helpers_return_owned_strings() {
    assert_eq!(repeat("ab", 3), "ababab");
    assert_eq!(repeat("ab", 0), "");
    assert_eq!(pad_left("42", 5, '0'), "00042");
    assert_eq!(pad_right("42", 5, '0'), "42000");
    assert_eq!(pad_left("你好", 3, '*'), "*你好");
    assert_eq!(pad_right("你好", 3, '*'), "你好*");
    assert_eq!(pad_left("ready", 3, '*'), "ready");
    assert_eq!(pad_right("ready", 3, '*'), "ready");
}

#[test]
fn case_helpers_normalize_common_word_boundaries() {
    assert_eq!(to_snake_case("helloWorld ID"), "hello_world_id");
    assert_eq!(to_underline_case("helloWorld ID"), "hello_world_id");
    assert_eq!(to_kebab_case("helloWorld ID"), "hello-world-id");
    assert_eq!(to_camel_case("hello_world-id"), "helloWorldId");
    assert_eq!(to_pascal_case("hello_world-id"), "HelloWorldId");
    assert_eq!(to_camel_case("HelloWorld"), "helloWorld");
    assert_eq!(to_pascal_case("helloWorld"), "HelloWorld");
    assert_eq!(to_dot_case("helloWorld ID"), "hello.world.id");
    assert_eq!(to_path_case("helloWorld ID"), "hello/world/id");
    assert_eq!(to_snake_case("HTTPServerID"), "http_server_id");
    assert_eq!(to_kebab_case("HTTPServerID"), "http-server-id");
    assert_eq!(to_screaming_snake_case("HTTPServerID"), "HTTP_SERVER_ID");
    assert_eq!(to_screaming_kebab_case("HTTPServerID"), "HTTP-SERVER-ID");
    assert_eq!(to_train_case("HTTPServerID"), "Http-Server-Id");
    assert_eq!(to_cobol_case("HTTPServerID"), "HTTP-SERVER-ID");
    assert_eq!(to_title_case("hello_world-id"), "Hello World Id");
    assert_eq!(to_sentence_case("hello_world-ID"), "Hello world id");
    assert_eq!(
        to_snake_case("  hello--rust_world  "),
        "__hello__rust_world__"
    );
}

#[test]
fn case_helpers_handle_empty_and_unicode_words() {
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_kebab_case(""), "");
    assert_eq!(to_camel_case(""), "");
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_snake_case("你好 Rust"), "你好_rust");
    assert_eq!(to_pascal_case("你好 rust"), "你好Rust");
    assert_eq!(capitalize("rUST"), "Rust");
    assert_eq!(capitalize("你好"), "你好");
    assert_eq!(uncapitalize("Rust"), "rust");
    assert_eq!(uncapitalize("HTTPServer"), "hTTPServer");
    assert_eq!(swap_case("Rust 你好"), "rUST 你好");
    assert_eq!(swap_case("Straße"), "sTRASSE");
}

#[test]
fn case_conversion_cross_crate_fixtures_lock_acronym_number_separator_unicode() {
    assert_eq!(to_snake_case("XMLHttpRequest2"), "xml_http_request2");
    assert_eq!(to_kebab_case("HTTPRequest2XX"), "http-request2-xx");
    assert_eq!(to_train_case("userID2FA"), "User-Id2-Fa");
    assert_eq!(to_screaming_snake_case("http2ServerID"), "HTTP2_SERVER_ID");

    assert_eq!(to_camel_case("http_server-id 42"), "httpServerId42");
    assert_eq!(to_pascal_case("http_server-id 42"), "HttpServerId42");
    assert_eq!(to_dot_case("http_server-id 42"), "http.server.id.42");
    assert_eq!(to_path_case("http_server-id 42"), "http/server/id/42");

    assert_eq!(
        to_snake_case("already__split--case  "),
        "already__split__case__"
    );
    assert_eq!(
        to_kebab_case("already__split--case  "),
        "already--split--case--"
    );
    assert_eq!(
        to_title_case("already__split--case  "),
        "Already Split Case"
    );
    assert_eq!(
        to_sentence_case("already__split--CASE  "),
        "Already  split  case  "
    );

    assert_eq!(to_snake_case("StraßeHTTP"), "straße_http");
    assert_eq!(to_screaming_snake_case("StraßeHTTP"), "STRASSE_HTTP");
    assert_eq!(to_camel_case("你好_rust-world"), "你好RustWorld");
    assert_eq!(to_pascal_case("你好_rust-world"), "你好RustWorld");
}

#[test]
fn default_helpers_return_fallback_only_when_needed() {
    assert_eq!(default_if_empty("", "fallback"), "fallback");
    assert_eq!(default_if_empty(" ", "fallback"), " ");
    assert_eq!(default_if_blank(" ", "fallback"), "fallback");
    assert_eq!(default_if_blank("value", "fallback"), "value");
}

#[test]
fn contains_helpers_support_single_any_and_all() {
    assert!(contains("knifer-rs", "rs"));
    assert!(contains_any("knifer-rs", ["go", "rs"]));
    assert!(!contains_any("knifer-rs", ["java", "py"]));
    assert!(contains_all("knifer-rs", ["knife", "rs"]));
    assert!(!contains_all("knifer-rs", ["knife", "go"]));
    assert!(contains_none("knifer-rs", ["go", "java"]));
    assert!(!contains_none("knifer-rs", ["go", "rs"]));
    assert!(contains_ignore_case("Knifer-RS", "rs"));
    assert!(!contains_ignore_case("Knifer-RS", "go"));
    assert!(contains_any_ignore_case("Knifer-RS", ["go", "RS"]));
    assert!(!contains_any_ignore_case("Knifer-RS", ["go", "java"]));
    assert!(contains_all_ignore_case("Knifer-RS", ["knife", "RS"]));
    assert!(!contains_all_ignore_case("Knifer-RS", ["knife", "go"]));
    assert_eq!(
        find_any("hello rust", ["go", "rust"]),
        Some(("rust", 6, 10))
    );
    assert_eq!(
        find_any("abc rust go", ["go", "rust"]),
        Some(("rust", 4, 8))
    );
    assert_eq!(find_any("hello rust", ["", "go"]), None);
    assert_eq!(count_matches("aaaa", "aa"), 2);
    assert_eq!(count_matches("你好你好", "你好"), 2);
    assert_eq!(count_matches("abc", ""), 0);
    assert_eq!(find_all("aaaa", "aa"), vec![(0, 2), (2, 4)]);
    assert_eq!(find_all("你好你好", "你好"), vec![(0, 6), (6, 12)]);
    assert!(find_all("abc", "").is_empty());
    assert_eq!(
        find_all_ignore_case("Go go Rust", "go"),
        vec![(0, 2), (3, 5)]
    );
    assert_eq!(find_all_ignore_case("abc\u{212A}", "k"), vec![(3, 6)]);
    assert!(find_all_ignore_case("abc", "").is_empty());
}

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
fn equality_and_reverse_helpers_are_unicode_aware() {
    assert!(equals_ignore_case("Knifer-RS", "knifer-rs"));
    assert!(equals_ignore_case("Straße", "straße"));
    assert!(equals_ignore_case("Σ", "ς"));
    assert!(equals_ignore_case("\u{212A}", "k"));
    assert!(!equals_ignore_case("Straße", "strasse"));
    assert_eq!(reverse("ab你好"), "好你ba");
}

#[test]
fn format_replaces_placeholders_in_order() {
    assert_eq!(
        format("name={}, age={}", &[&"tom", &12]),
        "name=tom, age=12"
    );
    assert_eq!(format(r"\{} {}", &[&"x"]), "{} x");
    assert_eq!(format("{} {}", &[&"x"]), "x {}");
    assert_eq!(format("{} {}", &[&"x", &"y", &"z"]), "x y");
    assert_eq!(format("", &[&"x"]), "");
    assert_eq!(format("{}", &[]), "{}");
}

#[test]
fn replace_helpers_support_first_last_and_case_insensitive_rewrites() {
    assert_eq!(replace_first("go go rust", "go", "rs"), "rs go rust");
    assert_eq!(replace_first("rust", "go", "rs"), "rust");
    assert_eq!(replace_first("rust", "", "rs"), "rust");
    assert_eq!(replace_last("go go rust", "go", "rs"), "go rs rust");
    assert_eq!(replace_last("rust", "go", "rs"), "rust");
    assert_eq!(replace_last("rust", "", "rs"), "rust");
    assert_eq!(replace_ignore_case("Go go Rust", "go", "rs"), "rs rs Rust");
    assert_eq!(replace_ignore_case("abc\u{212A}", "k", "K"), "abcK");
    assert_eq!(replace_ignore_case("rust", "", "rs"), "rust");
    assert_eq!(
        replace_many("hello rust world", [("hello", "hi"), ("world", "team")]),
        "hi rust team"
    );
    assert_eq!(replace_many("aaaa", [("aa", "b"), ("a", "c")]), "bb");
    assert_eq!(replace_many("aaaa", [("a", "c"), ("aa", "b")]), "cccc");
    assert_eq!(replace_many("rust", [("", "x")]), "rust");
    assert_eq!(escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
    assert_eq!(
        escape_regex(r".*?^$#&-~\[]{}|"),
        r"\.\*\?\^\$\#\&\-\~\\\[\]\{\}\|"
    );
    assert_eq!(quote_meta("[rust]"), r"\[rust\]");
}

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

#[test]
fn prefix_and_suffix_helpers_handle_existing_markers() {
    assert!(starts_with("knifer-rs", "knife"));
    assert!(ends_with("knifer-rs", "rs"));
    assert!(starts_with_ignore_case("Knifer-RS", "knife"));
    assert!(starts_with_ignore_case("\u{212A}nifer", "k"));
    assert!(!starts_with_ignore_case("Knifer-RS", "go"));
    assert!(starts_with_ignore_case("Knifer-RS", ""));
    assert!(ends_with_ignore_case("Knifer-RS", "RS"));
    assert!(ends_with_ignore_case("abc\u{212A}", "k"));
    assert!(!ends_with_ignore_case("Knifer-RS", "go"));
    assert!(ends_with_ignore_case("Knifer-RS", ""));
    assert_eq!(remove_prefix("knifer-rs", "knifer-"), "rs");
    assert_eq!(remove_prefix("knifer-rs", "go-"), "knifer-rs");
    assert_eq!(remove_suffix("knifer-rs", "-rs"), "knifer");
    assert_eq!(remove_suffix("knifer-rs", "-go"), "knifer-rs");
    assert_eq!(split_once("a=b=c", "="), Some(("a", "b=c")));
    assert_eq!(split_once("abc", "="), None);
    assert_eq!(split_once("abc", ""), None);
    assert_eq!(split_once_last("a=b=c", "="), Some(("a=b", "c")));
    assert_eq!(split_once_last("abc", "="), None);
    assert_eq!(split_once_last("abc", ""), None);
    assert_eq!(strip_prefix_ignore_case("Knifer-RS", "knife"), Some("r-RS"));
    assert_eq!(strip_prefix_ignore_case("Knifer-RS", "go"), None);
    assert_eq!(strip_prefix_ignore_case("Knifer-RS", ""), Some("Knifer-RS"));
    assert_eq!(strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
    assert_eq!(strip_suffix_ignore_case("Knifer-RS", "go"), None);
    assert_eq!(strip_suffix_ignore_case("Knifer-RS", ""), Some("Knifer-RS"));
    assert_eq!(add_prefix_if_not("path", "/"), "/path");
    assert_eq!(add_prefix_if_not("/path", "/"), "/path");
    assert_eq!(add_suffix_if_not("path", "/"), "path/");
    assert_eq!(add_suffix_if_not("path/", "/"), "path/");
}

#[test]
fn length_helpers_make_byte_and_char_counts_explicit() {
    assert_eq!(byte_len("knifer-rs"), 9);
    assert_eq!(char_len("knifer-rs"), 9);
    assert_eq!(byte_len("你好"), 6);
    assert_eq!(char_len("你好"), 2);
    assert_eq!(length("你好"), 2);
    assert_eq!(rune_len("你好"), 2);
}

#[test]
fn unicode_boundary_golden_cases_document_scalar_semantics() {
    let combining = "e\u{301}";
    assert_eq!(byte_len(combining), 3);
    assert_eq!(char_len(combining), 2);
    assert_eq!(chars(combining), vec!['e', '\u{301}']);
    assert_eq!(truncate(combining, 1), "e");
    assert_eq!(take_chars(combining, 2), combining);
    assert_eq!(drop_chars(combining, 1), "\u{301}");

    let emoji_zwj = "👨‍👩‍👧‍👦";
    assert_eq!(char_len(emoji_zwj), 7);
    assert_eq!(take_chars(emoji_zwj, 2), "👨‍");
    assert_eq!(drop_chars(emoji_zwj, 6), "👦");
    assert!(contains_emoji(emoji_zwj));

    let flag = "🇨🇳";
    assert_eq!(byte_len(flag), 8);
    assert_eq!(char_len(flag), 2);
    assert_eq!(take_chars(flag, 1), "🇨");
    assert!(contains_emoji(flag));

    let cjk = "你好世界";
    assert_eq!(byte_len(cjk), 12);
    assert_eq!(char_len(cjk), 4);
    assert_eq!(sub(cjk, 1, -1), "好世");
    assert_eq!(find_all(cjk, "世界"), vec![(6, 12)]);

    let mixed_width = "A你🚀e\u{301}";
    assert_eq!(char_len(mixed_width), 5);
    assert_eq!(truncate_with_suffix(mixed_width, 4, "..."), "A...");
    assert_eq!(wrap(mixed_width, 3), "A你🚀\ne\u{301}");
}

#[cfg(feature = "unicode-segmentation")]
#[test]
fn unicode_segmentation_helpers_preserve_grapheme_boundaries() {
    let combining = "e\u{301}";
    assert_eq!(char_len(combining), 2);
    assert_eq!(grapheme_len(combining), 1);
    assert_eq!(graphemes(combining), vec![combining]);
    assert_eq!(take_chars(combining, 1), "e");
    assert_eq!(take_graphemes(combining, 1), combining);

    let flag = "🇨🇳";
    assert_eq!(char_len(flag), 2);
    assert_eq!(grapheme_len(flag), 1);
    assert_eq!(take_chars(flag, 1), "🇨");
    assert_eq!(take_graphemes(flag, 1), flag);

    let family = "👨‍👩‍👧‍👦";
    assert_eq!(char_len(family), 7);
    assert_eq!(grapheme_len(family), 1);
    assert_eq!(take_graphemes(family, 1), family);

    let mixed = "e\u{301}🇨🇳rust";
    assert_eq!(graphemes(mixed), vec!["e\u{301}", "🇨🇳", "r", "u", "s", "t"]);
    assert_eq!(take_graphemes(mixed, 2), "e\u{301}🇨🇳");
    assert_eq!(truncate_graphemes(mixed, 4, "..."), "e\u{301}...");
    assert_eq!(truncate_graphemes("short", 10, "..."), "short");
    assert_eq!(truncate_graphemes(mixed, 2, "..."), "..");
    assert_eq!(truncate_graphemes(mixed, 0, "..."), "");
}

#[cfg(feature = "unicode-segmentation")]
#[test]
fn unicode_segmentation_helpers_expose_word_boundaries() {
    let sentence = "The quick (\"brown\") fox can't jump 32.3 feet, 世界!";
    assert_eq!(
        unicode_words(sentence),
        vec![
            "The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "世", "界"
        ]
    );
    assert_eq!(unicode_word_len(sentence), 10);
    assert_eq!(
        unicode_word_indices("hi 世界 32.3"),
        vec![(0, "hi"), (3, "世"), (6, "界"), (10, "32.3")]
    );

    let bounds = split_word_bounds("Hi, 世界!");
    assert_eq!(bounds.concat(), "Hi, 世界!");
    assert_eq!(bounds, vec!["Hi", ",", " ", "世", "界", "!"]);
    assert_eq!(
        split_word_bound_indices("Hi, 世界!"),
        vec![
            (0, "Hi"),
            (2, ","),
            (3, " "),
            (4, "世"),
            (7, "界"),
            (10, "!")
        ]
    );
}

#[cfg(feature = "unicode-segmentation")]
#[test]
fn unicode_segmentation_helpers_expose_sentence_boundaries() {
    let text = "Mr. Fox jumped. [...] The dog was too lazy.";
    assert_eq!(
        unicode_sentences(text),
        vec!["Mr. ", "Fox jumped. ", "The dog was too lazy."]
    );
    assert_eq!(unicode_sentence_len(text), 3);

    let bounds = split_sentence_bounds(text);
    assert_eq!(bounds.concat(), text);
    assert_eq!(
        bounds,
        vec!["Mr. ", "Fox jumped. ", "[...] ", "The dog was too lazy."]
    );
    assert_eq!(
        split_sentence_bound_indices(text),
        vec![
            (0, "Mr. "),
            (4, "Fox jumped. "),
            (16, "[...] "),
            (22, "The dog was too lazy.")
        ]
    );
}

#[cfg(feature = "unicode-width")]
#[test]
fn unicode_width_helpers_follow_display_cell_boundaries() {
    assert_eq!(display_width("abc"), 3);
    assert_eq!(display_width("你好"), 4);
    assert_eq!(display_width("e\u{301}"), 1);
    assert_eq!(display_width("👨\u{200d}👩\u{200d}👧\u{200d}👦"), 2);
    assert_eq!(char_len("你好"), 2);

    assert_eq!(take_width("a你好", 0), "");
    assert_eq!(take_width("a你好", 1), "a");
    assert_eq!(take_width("a你好", 2), "a");
    assert_eq!(take_width("a你好", 3), "a你");
    assert_eq!(take_width("e\u{301}clair", 1), "e\u{301}");
    assert_eq!(
        take_width("👨\u{200d}👩\u{200d}👧\u{200d}👦 family", 2),
        "👨\u{200d}👩\u{200d}👧\u{200d}👦"
    );

    assert_eq!(truncate_width("abcdef", 0, "..."), "");
    assert_eq!(truncate_width("abcdef", 2, "..."), "..");
    assert_eq!(truncate_width("你好Rust", 6, "..."), "你...");
    assert_eq!(truncate_width("short", 10, "..."), "short");
    assert_eq!(truncate_width("e\u{301}clair", 4, "..."), "e\u{301}...");

    assert_eq!(wrap_width("你好Rust world", 8), "你好Rust\nworld");
    assert_eq!(wrap_width("你好世界Rust", 6), "你好世\n界Rust");
    assert_eq!(
        wrap_width_with_indent("你好Rust world", 10, "* ", "  "),
        "* 你好Rust\n  world"
    );
    assert_eq!(
        wrap_width_with_indent("alpha 你好 beta", 9, "=> ", "   "),
        "=> alpha\n   你好\n   beta"
    );
    assert_eq!(wrap_width_with_indent("ignored", 0, "> ", "  "), "");
}

#[test]
fn emoji_helpers_detect_and_remove_common_sequences() {
    assert!(contains_emoji("ship it 🚀"));
    assert!(contains_emoji("go ✅"));
    assert!(contains_emoji("flag 🇨🇳"));
    assert!(contains_emoji("key 1️⃣"));
    assert!(!contains_emoji("knifer-rs 123"));
    assert_eq!(remove_emoji("ship 🚀 now"), "ship  now");
    assert_eq!(remove_emoji("ok ✅"), "ok ");
    assert_eq!(remove_emoji("key 1️⃣ done"), "key  done");
    assert_eq!(remove_emoji("knifer-rs 123"), "knifer-rs 123");
}

#[test]
fn emoji_options_customize_matching_and_replacement() {
    let matcher = with_emoji_matcher(|input| input.contains(":rocket:"));
    assert!(contains_emoji_with_options("ship :rocket:", &matcher));
    assert!(!contains_emoji_with_options("ship 🚀", &matcher));

    let replacer = with_emoji_replacer(|input| input.replace(":rocket:", ""));
    assert_eq!(
        remove_emoji_with_options("ship :rocket: now", &replacer),
        "ship  now"
    );
    assert_eq!(remove_emoji_with_options("ship 🚀", &replacer), "ship 🚀");

    let defaults = EmojiOptions::new();
    assert!(contains_emoji_with_options("ship 🚀", &defaults));
    assert_eq!(remove_emoji_with_options("ship 🚀", &defaults), "ship ");
}

#[test]
fn html_helpers_escape_and_unescape_common_entities() {
    let escaped = escape_html("<a href='x' title=\"y\">Tom&Jerry</a>");
    assert_eq!(
        escaped,
        "&lt;a href=&#39;x&#39; title=&quot;y&quot;&gt;Tom&amp;Jerry&lt;/a&gt;"
    );
    assert_eq!(
        unescape_html("&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;"),
        "<b>Tom&Jerry</b>"
    );
    assert_eq!(unescape_html("&#x27;quoted&#x27;"), "'quoted'");
}

#[test]
fn unicode_escape_helpers_handle_bmp_and_surrogate_pairs() {
    assert_eq!(escape_unicode("Rust你好"), "Rust\\u4F60\\u597D");
    assert_eq!(escape_unicode("🚀"), "\\uD83D\\uDE80");
    assert_eq!(unescape_unicode("Rust\\u4F60\\u597D"), "Rust你好");
    assert_eq!(unescape_unicode("\\uD83D\\uDE80"), "🚀");
}

#[test]
fn unicode_unescape_preserves_malformed_or_isolated_surrogates() {
    assert_eq!(unescape_unicode("\\uZZZZ"), "\\uZZZZ");
    assert_eq!(unescape_unicode("\\uD83D"), "\\uD83D");
    assert_eq!(unescape_unicode("\\uDE80"), "\\uDE80");
    assert_eq!(unescape_unicode("\\uD83Dtext"), "\\uD83Dtext");
}

#[test]
fn character_classification_helpers_follow_unicode_semantics() {
    assert!(is_blank_char(' '));
    assert!(is_blank_char('\u{3000}'));
    assert!(is_letter('你'));
    assert!(is_letter('A'));
    assert!(!is_letter('Ⅷ'));
    assert!(is_digit('9'));
    assert!(is_digit('٣'));
    assert!(is_digit('９'));
    assert!(!is_digit('Ⅷ'));
    assert!(!is_digit('七'));
    assert!(is_ascii('A'));
    assert!(!is_ascii('你'));
    assert!(is_letter_or_digit('A'));
    assert!(is_letter_or_digit('9'));
    assert!(is_letter_or_digit('٣'));
    assert!(is_letter_or_digit('七'));
    assert!(!is_letter_or_digit('Ⅷ'));
    assert!(!is_letter_or_digit('-'));
}

#[test]
fn text_helpers_normalize_truncate_and_slugify() {
    assert_eq!(normalize_whitespace("  hello\n\tRust  "), "hello Rust");
    assert_eq!(normalize_whitespace("你好\u{3000}Rust"), "你好 Rust");
    assert_eq!(remove_whitespace(" a\n b\t "), "ab");
    assert_eq!(remove_whitespace("你 好\u{3000}Rust"), "你好Rust");
    assert_eq!(normalize_newlines("a\r\nb\rc"), "a\nb\nc");
    assert_eq!(normalize_newlines("a\nb"), "a\nb");
    assert_eq!(trim_lines("  a  \n\tb\t\n"), "a\nb\n");
    assert_eq!(trim_blank_lines("\n  \nhello\n\n"), "hello");
    assert_eq!(trim_blank_lines("\r\n  \r\nhello\r\n\r\n"), "hello");
    assert_eq!(trim_blank_lines(" \n\t"), "");
    assert_eq!(truncate("你好Rust", 3), "你好R");
    assert_eq!(truncate("你好Rust", 0), "");
    assert_eq!(truncate("short", 10), "short");
    assert_eq!(take_chars("你好Rust", 3), "你好R");
    assert_eq!(take_chars("你好Rust", 0), "");
    assert_eq!(drop_chars("你好Rust", 2), "Rust");
    assert_eq!(drop_chars("你好Rust", 0), "你好Rust");
    assert_eq!(drop_chars("你好Rust", 20), "");
    assert_eq!(truncate_with_suffix("你好Rust", 5, "..."), "你好...");
    assert_eq!(truncate_with_suffix("short", 10, "..."), "short");
    assert_eq!(truncate_with_suffix("abcdef", 2, "..."), "..");
    assert_eq!(
        abbreviate_middle("abcdefghijklmnopqrstuvwxyz", 10, "..."),
        "abcd...xyz"
    );
    assert_eq!(abbreviate_middle("short", 10, "..."), "short");
    assert_eq!(abbreviate_middle("abcdef", 2, "..."), "..");
    assert_eq!(
        limit_words("hello rust utility toolkit", 2, "..."),
        "hello rust..."
    );
    assert_eq!(limit_words("hello rust", 3, "..."), "hello rust");
    assert_eq!(
        excerpt("hello rust utility toolkit", "utility", 14, "..."),
        "...st utility too..."
    );
    assert_eq!(excerpt("hello rust", "go", 8, "..."), "hello...");
    assert_eq!(mask("13800138000", 3, 4, '*'), "138****8000");
    assert_eq!(mask("short", 10, 10, '*'), "short");
    assert_eq!(collapse_repeated_char("a---b----c", '-'), "a-b-c");
    assert_eq!(collapse_repeated_char("aaab", 'a'), "ab");
    assert_eq!(slugify("Hello, Rust World!"), "hello-rust-world");
    assert_eq!(slugify("你好 Rust"), "你好-rust");
    assert_eq!(
        slugify_with_separator("Hello, Rust World!", '_'),
        "hello_rust_world"
    );
    assert_eq!(slugify("---Hello---"), "hello");
    assert_eq!(slugify_with_separator("Hello Rust", 'x'), "hello-rust");
}

#[test]
fn text_formatting_helpers_indent_dedent_wrap_and_count() {
    assert_eq!(indent("a\nb", "  "), "  a\n  b");
    assert_eq!(indent("", "> "), "> ");
    assert_eq!(center("rust", 8, '-'), "--rust--");
    assert_eq!(center("rust", 9, '-'), "--rust---");
    assert_eq!(center("你好", 5, '*'), "*你好**");
    assert_eq!(center("ready", 3, '*'), "ready");
    assert_eq!(dedent("    a\n      b"), "a\n  b");
    assert_eq!(dedent("  a\n\n    b"), "a\n\n  b");
    assert_eq!(wrap("hello rust world", 10), "hello rust\nworld");
    assert_eq!(wrap("superlongword", 5), "super\nlongw\nord");
    assert_eq!(wrap("你好 Rust 世界", 7), "你好 Rust\n世界");
    assert_eq!(wrap("ignored", 0), "");
    assert_eq!(
        wrap_with_indent("hello rust world", 12, "* ", "  "),
        "* hello rust\n  world"
    );
    assert_eq!(
        wrap_with_indent("superlongword", 8, "> ", "  "),
        "> superl\n  ongwor\n  d"
    );
    assert_eq!(wrap_with_indent("ignored", 0, "> ", "  "), "");
    assert_eq!(lines("a\nb\n"), vec!["a", "b"]);
    assert!(lines("").is_empty());
    assert_eq!(non_blank_lines(" a \n\n b "), vec!["a", "b"]);
    assert_eq!(non_blank_lines(" \n\t"), Vec::<&str>::new());
    assert_eq!(words("hello  Rust\n世界"), vec!["hello", "Rust", "世界"]);
    assert_eq!(initials("rust string toolkit"), "RST");
    assert_eq!(initials("你好 rust"), "你R");
    assert_eq!(chars("a你"), vec!['a', '你']);
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("上海自来水来自海上"));
    assert!(!is_palindrome("knifer-rs"));
    assert_eq!(extract_digits("id=42, رقم=٣"), "42٣");
    assert_eq!(
        remove_ascii_punctuation("Hello, Rust! 你好，世界！"),
        "Hello Rust 你好，世界！"
    );
    assert_eq!(surround("value", "[", "]"), "[value]");
    assert_eq!(unsurround("[value]", "[", "]"), Some("value"));
    assert_eq!(unsurround("value]", "[", "]"), None);
    assert_eq!(word_count("hello  Rust\n世界"), 3);
    assert_eq!(word_count(" \n\t"), 0);
    assert_eq!(line_count("a\nb\n"), 2);
    assert_eq!(line_count(""), 0);
}

#[test]
fn wrap_and_truncate_boundary_cases_follow_scalar_width_policy() {
    assert_eq!(wrap("anything", 0), "");
    assert_eq!(
        wrap("supercalifragilistic", 6),
        "superc\nalifra\ngilist\nic"
    );
    assert_eq!(wrap("abcdef", 3), "abc\ndef");
    assert_eq!(wrap("abcdef", 10), "abcdef");
    assert_eq!(wrap("a   b\t\tc\n\nnext", 5), "a b c\n\nnext");
    assert_eq!(wrap("  a   b  ", 4), "a b");
    assert_eq!(wrap("one two three", 7), "one two\nthree");
    assert_eq!(wrap("你好世界 Rust", 4), "你好世界\nRust");
    assert_eq!(wrap("你好世界", 2), "你好\n世界");
    assert_eq!(wrap("🚀🚀🚀go", 3), "🚀🚀🚀\ngo");
    assert_eq!(wrap("👨‍👩‍👧‍👦", 4), "👨‍👩‍\n👧‍👦");
    assert_eq!(wrap("e\u{301}e\u{301}", 2), "e\u{301}\ne\u{301}");

    assert_eq!(wrap_with_indent("anything", 0, "=> ", "   "), "");
    assert_eq!(
        wrap_with_indent("hello rust utility", 10, "=> ", "   "),
        "=> hello\n   rust\n   utility"
    );
    assert_eq!(
        wrap_with_indent("alpha beta gamma", 9, "> ", "...."),
        "> alpha\n....beta\n....gamma"
    );
    assert_eq!(
        wrap_with_indent("abcdefghij", 4, ">>>>", "--"),
        ">>>>a\n--b\n--c\n--d\n--e\n--f\n--g\n--h\n--i\n--j"
    );

    assert_eq!(truncate_with_suffix("abcdef", 0, "..."), "");
    assert_eq!(truncate_with_suffix("abcdef", 3, ""), "abc");
    assert_eq!(truncate_with_suffix("abcdef", 2, "..."), "..");
    assert_eq!(truncate_with_suffix("abcdef", 3, "..."), "...");
    assert_eq!(truncate_with_suffix("abcdef", 4, "…"), "abc…");
    assert_eq!(truncate_with_suffix("你好世界", 3, "…"), "你好…");
    assert_eq!(truncate_with_suffix("e\u{301}clair", 4, "..."), "e...");
    assert_eq!(truncate_with_suffix("👨‍👩‍👧‍👦 family", 5, "..."), "👨‍...");

    assert_eq!(abbreviate_middle("abcdef", 0, "..."), "");
    assert_eq!(abbreviate_middle("abcdef", 3, "..."), "...");
    assert_eq!(abbreviate_middle("abcdef", 5, ""), "abcef");
    assert_eq!(abbreviate_middle("👨‍👩‍👧‍👦 family", 7, "..."), "👨‍...ly");
    assert_eq!(abbreviate_middle("你好Rust世界", 7, "..."), "你好...世界");
}

#[test]
fn knifer_go_vstr_golden_fixtures_cover_case_conversion() {
    assert_eq!(to_camel_case("hello_world"), "helloWorld");
    assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    assert_eq!(to_underline_case("HelloWorld"), "hello_world");
    assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
}

#[test]
fn knifer_go_vstr_golden_fixtures_cover_unicode_escape() {
    assert_eq!(escape_unicode("中国"), "\\u4E2D\\u56FD");
    assert_eq!(unescape_unicode("\\u4E2D\\u56FD"), "中国");
}

#[test]
fn knifer_go_vstr_golden_fixtures_cover_ant_path_matching() {
    assert!(ant_path_match("/api/**/users/?", "/api/v1/admin/users/a"));
    assert!(!ant_path_match("/api/*/users", "/api/v1/admin/users"));
    assert!(ant_path_match_with_separator(
        "foo.**.bar",
        "foo.a.b.bar",
        "."
    ));
}

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
fn ant_path_match_supports_segment_wildcards() {
    assert!(ant_path_match("/api/**", "/api/v1/users"));
    assert!(ant_path_match("/api/?/users", "/api/v/users"));
    assert!(ant_path_match("/api/*", "/api/v1"));
    assert!(!ant_path_match("/api/*", "/api/v1/users"));
    assert!(ant_path_match_with_separator("a.**.d", "a.b.c.d", "."));
    assert!(!ant_path_match_with_separator("a.*.d", "a.b.c.d", "."));
    assert!(ant_path_match_with_separator("a.?.d", "a.b.d", "."));
    assert!(ant_path_match_with_separator("a/**", "a/b", ""));
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

fn assert_approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON);
}

struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    fn usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            let upper = u64::try_from(upper).expect("test upper bound must fit in u64");
            usize::try_from(self.next() % upper).expect("bounded value must fit in usize")
        }
    }

    fn index_around(&mut self, len: usize) -> isize {
        let span = len.saturating_mul(2).saturating_add(9);
        let sampled = isize::try_from(self.usize(span)).expect("test sample must fit in isize");
        let len = isize::try_from(len).expect("test length must fit in isize");
        sampled - len - 4
    }

    fn string(&mut self, max_chars: usize) -> String {
        let len = self.usize(max_chars + 1);
        let alphabet = [
            'a', 'b', 'c', '-', '_', ' ', '.', '*', '你', '好', '🚀', 'e', '\u{301}',
        ];
        let mut output = String::new();
        for _ in 0..len {
            output.push(alphabet[self.usize(alphabet.len())]);
        }
        output
    }

    fn path(&mut self, max_segments: usize) -> String {
        let segments = self.usize(max_segments).saturating_add(1);
        let names = ["api", "v1", "users", "项目", "rust", "42"];
        let mut path = String::new();
        for _ in 0..segments {
            path.push('/');
            path.push_str(names[self.usize(names.len())]);
        }
        path
    }
}

fn expected_sub(input: &str, from_index: isize, to_index: isize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let start = expected_normalize_index(from_index, len);
    let end = expected_normalize_index(to_index, len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    chars[start..end].iter().collect()
}

fn expected_normalize_index(index: isize, len: usize) -> usize {
    if index < 0 {
        len.saturating_sub(index.unsigned_abs())
    } else {
        usize::try_from(index).map_or(len, |index| index.min(len))
    }
}
