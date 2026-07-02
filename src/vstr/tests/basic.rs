use crate::vstr::*;

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
fn index_family_returns_byte_positions() {
    // covered: Commons `indexOf` / `lastIndexOf` / `ordinalIndexOf`.
    assert_eq!(index_of("knifer-rs", "rs"), Some(7));
    assert_eq!(index_of("knifer-rs", "go"), None);
    assert_eq!(index_of("你好世界", "世界"), Some(6));
    assert_eq!(index_of("abc", ""), Some(0));

    assert_eq!(index_of_ignore_case("Knifer-RS", "rs"), Some(7));
    assert_eq!(index_of_ignore_case("abc\u{212A}", "k"), Some(3));
    assert_eq!(index_of_ignore_case("abc", "z"), None);
    assert_eq!(index_of_ignore_case("abc", ""), Some(0));

    assert_eq!(last_index_of("go go go", "go"), Some(6));
    assert_eq!(last_index_of("go go go", "x"), None);
    assert_eq!(last_index_of("abc", ""), Some(3));

    assert_eq!(ordinal_index_of("a.b.c.d", ".", 1), Some(1));
    assert_eq!(ordinal_index_of("a.b.c.d", ".", 3), Some(5));
    assert_eq!(ordinal_index_of("a.b.c.d", ".", 4), None);
    assert_eq!(ordinal_index_of("a.b.c.d", ".", 0), None);
    assert_eq!(ordinal_index_of("aaaa", "aa", 2), Some(2));
    assert_eq!(ordinal_index_of("abc", "", 1), None);

    assert_eq!(index_of_any("hello rust", ["go", "rust"]), Some(6));
    assert_eq!(index_of_any("hello rust", ["z", "l"]), Some(2));
    assert_eq!(index_of_any("hello", ["x", "y"]), None);
    assert_eq!(index_of_any("hello", ["", "l"]), Some(2));
}

#[test]
fn manipulation_helpers_are_scalar_safe() {
    // insert clamps a large index to an append.
    assert_eq!(insert("abcd", 2, "XY"), "abXYcd");
    assert_eq!(insert("你好", 1, "-"), "你-好");
    assert_eq!(insert("abc", 0, ">"), ">abc");
    assert_eq!(insert("abc", 99, "!"), "abc!");
    assert_eq!(insert("", 0, "x"), "x");

    // overlay clamps and normalizes ranges (Commons parity).
    assert_eq!(overlay("abcdef", "ZZ", 2, 4), "abZZef");
    assert_eq!(overlay("abcdef", "ZZ", 4, 2), "abZZef");
    assert_eq!(overlay("abc", "ZZ", 2, 99), "abZZ");
    assert_eq!(overlay("你好世界", "-", 1, 3), "你-界");
    assert_eq!(overlay("abc", "", 0, 3), "");

    // remove_range is overlay with an empty replacement.
    assert_eq!(remove_range("abcdef", 2, 4), "abef");
    assert_eq!(remove_range("你好世界", 1, 3), "你界");
    assert_eq!(remove_range("abc", 1, 99), "a");

    // replace_range rejects invalid ranges instead of clamping.
    assert_eq!(
        replace_range("abcdef", 2, 4, "ZZ").as_deref(),
        Some("abZZef")
    );
    assert_eq!(replace_range("abc", 1, 1, "-").as_deref(), Some("a-bc"));
    assert_eq!(replace_range("abc", 3, 3, "!").as_deref(), Some("abc!"));
    assert_eq!(replace_range("abc", 2, 1, "-"), None);
    assert_eq!(replace_range("abc", 0, 99, "-"), None);

    // chunk splits into borrowed scalar pieces.
    assert_eq!(chunk("abcdefg", 3), vec!["abc", "def", "g"]);
    assert_eq!(chunk("你好世界", 2), vec!["你好", "世界"]);
    assert_eq!(chunk("abc", 5), vec!["abc"]);
    assert!(chunk("abc", 0).is_empty());
    assert!(chunk("", 3).is_empty());
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
fn render_template_expands_named_placeholders() {
    assert_eq!(
        render_template("Hi {name}, {count} new", [("name", "Tom"), ("count", "3")]),
        "Hi Tom, 3 new"
    );

    // Escaped braces become literals; missing keys stay verbatim.
    assert_eq!(
        render_template("{{literal}} {missing}", [("name", "Tom")]),
        "{literal} {missing}"
    );

    // First matching pair wins; unrelated pairs are ignored.
    assert_eq!(
        render_template("{k}", [("k", "first"), ("k", "second")]),
        "first"
    );

    // Values are inserted as-is, including multi-byte content.
    assert_eq!(
        render_template("{greeting} {who}", [("greeting", "你好"), ("who", "🌍")]),
        "你好 🌍"
    );

    // Unterminated braces and empty inputs never panic.
    assert_eq!(render_template("a {unclosed", [("x", "y")]), "a {unclosed");
    assert_eq!(render_template("{}", [("", "empty-key")]), "empty-key");
    assert_eq!(render_template("plain", std::iter::empty()), "plain");
    assert_eq!(render_template("", [("x", "y")]), "");
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
    assert_eq!(add_prefix_if_not_ignore_case("Path", "/"), "/Path");
    assert_eq!(
        add_prefix_if_not_ignore_case("HTTP://x", "http://"),
        "HTTP://x"
    );
    assert_eq!(
        add_suffix_if_not_ignore_case("report", ".TXT"),
        "report.TXT"
    );
    assert_eq!(
        add_suffix_if_not_ignore_case("report.TXT", ".txt"),
        "report.TXT"
    );
}

#[test]
fn common_prefix_and_suffix_are_scalar_safe() {
    assert_eq!(common_prefix("knifer-rs", "knifer-go"), "knifer-");
    assert_eq!(common_prefix("你好世界", "你好朋友"), "你好");
    assert_eq!(common_prefix("abc", "xyz"), "");
    assert_eq!(common_prefix("", "abc"), "");
    assert_eq!(common_prefix("abc", "abcdef"), "abc");
    // A shared byte prefix that splits a multi-byte scalar must not leak.
    assert_eq!(common_prefix("é", "e"), "");
    assert_eq!(common_suffix("knifer-rs", "wrapper-rs"), "er-rs");
    assert_eq!(common_suffix("读写世界", "编写世界"), "写世界");
    assert_eq!(common_suffix("abc", "xyz"), "");
    assert_eq!(common_suffix("abc", ""), "");
    assert_eq!(common_suffix("abcdef", "def"), "def");
}

#[test]
fn difference_returns_the_diverging_tail_of_the_right_side() {
    assert_eq!(difference("i am a machine", "i am a robot"), "robot");
    assert_eq!(difference("", "abc"), "abc");
    assert_eq!(difference("abc", ""), "");
    assert_eq!(difference("abc", "abc"), "");
    assert_eq!(difference("abc", "ab"), "");
    assert_eq!(difference("ab", "abxyz"), "xyz");
    assert_eq!(difference("你好世界", "你好朋友"), "朋友");
}

#[test]
fn index_of_difference_returns_first_diverging_byte() {
    // covered: Commons `indexOfDifference`.
    assert_eq!(
        index_of_difference("i am a machine", "i am a robot"),
        Some(7)
    );
    assert_eq!(index_of_difference("abc", "abc"), None);
    assert_eq!(index_of_difference("abc", "abcdef"), Some(3));
    assert_eq!(index_of_difference("abcdef", "abc"), Some(3));
    assert_eq!(index_of_difference("", "abc"), Some(0));
    assert_eq!(index_of_difference("", ""), None);
    assert_eq!(index_of_difference("你好世界", "你好朋友"), Some(6));
}

#[test]
fn rotate_wraps_by_scalar_positions_in_both_directions() {
    assert_eq!(rotate("abcdefg", 0), "abcdefg");
    assert_eq!(rotate("abcdefg", 2), "fgabcde");
    assert_eq!(rotate("abcdefg", -2), "cdefgab");
    assert_eq!(rotate("abcdefg", 7), "abcdefg");
    assert_eq!(rotate("abcdefg", 9), "fgabcde");
    assert_eq!(rotate("abcdefg", -9), "cdefgab");
    assert_eq!(rotate("你好世界", 1), "界你好世");
    assert_eq!(rotate("你好世界", -1), "好世界你");
    assert_eq!(rotate("", 3), "");
    assert_eq!(rotate("a", 5), "a");
}

#[test]
fn wrap_if_missing_only_adds_absent_markers() {
    assert_eq!(wrap_if_missing("path", "/"), "/path/");
    assert_eq!(wrap_if_missing("/path/", "/"), "/path/");
    assert_eq!(wrap_if_missing("/path", "/"), "/path/");
    assert_eq!(wrap_if_missing("path/", "/"), "/path/");
    assert_eq!(wrap_if_missing("/", "/"), "/");
    assert_eq!(wrap_if_missing("", "/"), "");
    assert_eq!(wrap_if_missing("path", ""), "path");
    assert_eq!(wrap_if_missing("value", "**"), "**value**");
    assert_eq!(wrap_if_missing("**value**", "**"), "**value**");
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
fn commons_string_utils_classics_lock_cross_crate_shape() {
    // These fixtures track Apache Commons Lang `StringUtils` behavior so the
    // classics keep a familiar shape. Rust adds scalar-safety and borrowing:
    // `common_prefix`/`common_suffix`/`difference` return borrowed `&str`.
    // covered: Commons `getCommonPrefix` returns the shared leading run.
    assert_eq!(common_prefix("i am a machine", "i am a robot"), "i am a ");
    assert_eq!(common_prefix("abc", "abc"), "abc");
    assert_eq!(common_prefix("abc", "def"), "");
    // covered-rust-shape: Commons has no `getCommonSuffix`; symmetric helper.
    assert_eq!(common_suffix("running", "jumping"), "ing");
    // covered: Commons `difference("i am a machine", "i am a robot")` -> "robot".
    assert_eq!(difference("i am a machine", "i am a robot"), "robot");
    assert_eq!(difference("foo", "foo"), "");
    assert_eq!(difference("", "abc"), "abc");
    // covered: Commons `rotate("abcdefg", 2)` -> "fgabcde".
    assert_eq!(rotate("abcdefg", 2), "fgabcde");
    assert_eq!(rotate("abcdefg", -2), "cdefgab");
    // covered: Commons `wrapIfMissing("path", "/")` -> "/path/".
    assert_eq!(wrap_if_missing("path", "/"), "/path/");
    assert_eq!(wrap_if_missing("/path/", "/"), "/path/");
    assert_eq!(wrap_if_missing("", "/"), "");
    // covered-rust-shape: `appendIfMissing`/`prependIfMissing` ignore-case.
    assert_eq!(add_suffix_if_not_ignore_case("a.TXT", ".txt"), "a.TXT");
    assert_eq!(add_prefix_if_not_ignore_case("A", "a"), "A");
}
