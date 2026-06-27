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
    assert_eq!(to_snake_case("  hello--rust_world  "), "hello_rust_world");
}

#[test]
fn case_helpers_handle_empty_and_unicode_words() {
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_kebab_case(""), "");
    assert_eq!(to_camel_case(""), "");
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_snake_case("你好 Rust"), "你好_rust");
    assert_eq!(to_pascal_case("你好 rust"), "你好Rust");
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
    assert!(contains_ignore_case("Knifer-RS", "rs"));
    assert!(!contains_ignore_case("Knifer-RS", "go"));
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
fn prefix_and_suffix_helpers_handle_existing_markers() {
    assert!(starts_with("knifer-rs", "knife"));
    assert!(ends_with("knifer-rs", "rs"));
    assert_eq!(remove_prefix("knifer-rs", "knifer-"), "rs");
    assert_eq!(remove_prefix("knifer-rs", "go-"), "knifer-rs");
    assert_eq!(remove_suffix("knifer-rs", "-rs"), "knifer");
    assert_eq!(remove_suffix("knifer-rs", "-go"), "knifer-rs");
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

fn assert_approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON);
}
