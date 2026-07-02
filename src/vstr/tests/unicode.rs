use crate::vstr::*;

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
fn grapheme_correct_variants_mirror_char_apis_without_splitting_clusters() {
    // reverse_graphemes keeps clusters intact where scalar reverse would not.
    assert_eq!(reverse("e\u{301}b"), "b\u{301}e");
    assert_eq!(reverse_graphemes("e\u{301}b"), "be\u{301}");
    assert_eq!(reverse_graphemes("🇨🇳🇯🇵"), "🇯🇵🇨🇳");
    assert_eq!(
        reverse_graphemes("👨\u{200d}👩\u{200d}👧\u{200d}👦x"),
        "x👨\u{200d}👩\u{200d}👧\u{200d}👦"
    );
    assert_eq!(reverse_graphemes(""), "");

    // Padding counts grapheme clusters, so a combining pair only needs two pads.
    assert_eq!(pad_left_graphemes("e\u{301}", 3, '*'), "**e\u{301}");
    assert_eq!(pad_right_graphemes("e\u{301}", 3, '*'), "e\u{301}**");
    assert_eq!(pad_left_graphemes("🇨🇳", 1, '*'), "🇨🇳");
    assert_eq!(pad_right_graphemes("", 2, '-'), "--");

    // Centering counts clusters; the extra pad lands on the right.
    assert_eq!(center_graphemes("e\u{301}", 4, '-'), "-e\u{301}--");
    assert_eq!(center_graphemes("🇨🇳", 3, '.'), ".🇨🇳.");
    assert_eq!(center_graphemes("wide", 2, '-'), "wide");

    // Masking never splits a flag or ZWJ cluster.
    assert_eq!(mask_graphemes("🇨🇳🇯🇵🇰🇷🇺🇸", 1, 1, '*'), "🇨🇳**🇺🇸");
    assert_eq!(mask_graphemes("e\u{301}llo", 1, 1, '*'), "e\u{301}**o");
    assert_eq!(mask_graphemes("short", 10, 10, '*'), "short");
    assert_eq!(mask_graphemes("ab", 1, 1, '*'), "ab");
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

#[test]
fn whitespace_tokenizer_contract_covers_cjk_emoji_and_punctuation() {
    // `words` / `word_count` split on Unicode whitespace only. Punctuation stays
    // attached and whitespace-free scripts such as CJK stay as a single token.
    assert_eq!(words(""), Vec::<&str>::new());
    assert_eq!(word_count(""), 0);
    assert_eq!(
        words("  hello  Rust\n世界  "),
        vec!["hello", "Rust", "世界"]
    );
    assert_eq!(word_count("  hello  Rust\n世界  "), 3);
    assert_eq!(words("Rust-go, 世界!"), vec!["Rust-go,", "世界!"]);
    assert_eq!(word_count("Rust-go, 世界!"), 2);
    assert_eq!(words("can't stop"), vec!["can't", "stop"]);
    assert_eq!(words("emoji 👩‍💻 here"), vec!["emoji", "👩‍💻", "here"]);
    assert_eq!(word_count("emoji 👩‍💻 here"), 3);
}

#[cfg(feature = "unicode-segmentation")]
#[test]
fn tokenizer_contract_contrasts_whitespace_and_uax29_word_boundaries() {
    // Same inputs, two contracts: whitespace tokenization (`words`) vs UAX #29
    // word boundaries (`unicode_words`). This pins the documented difference.
    let cases: [(&str, Vec<&str>, Vec<&str>); 4] = [
        (
            "Rust-go, 世界!",
            vec!["Rust-go,", "世界!"],
            vec!["Rust", "go", "世", "界"],
        ),
        (
            "jump 32.3 feet",
            vec!["jump", "32.3", "feet"],
            vec!["jump", "32.3", "feet"],
        ),
        ("你好世界", vec!["你好世界"], vec!["你", "好", "世", "界"]),
        ("can't stop.", vec!["can't", "stop."], vec!["can't", "stop"]),
    ];

    for (input, whitespace_tokens, uax29_tokens) in cases {
        assert_eq!(words(input), whitespace_tokens, "words: {input}");
        assert_eq!(word_count(input), whitespace_tokens.len(), "count: {input}");
        assert_eq!(unicode_words(input), uax29_tokens, "unicode_words: {input}");
        assert_eq!(
            unicode_word_len(input),
            uax29_tokens.len(),
            "unicode_word_len: {input}"
        );
    }
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

#[cfg(feature = "unicode-segmentation")]
#[test]
fn unicode_segmentation_conformance_fixtures_cover_curated_uax29_subset() {
    let grapheme_fixtures = [
        ("crlf", "a\r\nb", vec!["a", "\r\n", "b"]),
        (
            "combining mark stack",
            "a\u{0308}\u{0301}",
            vec!["a\u{0308}\u{0301}"],
        ),
        ("emoji modifier", "👍🏽", vec!["👍🏽"]),
        (
            "emoji keycap",
            "1\u{FE0F}\u{20E3}",
            vec!["1\u{FE0F}\u{20E3}"],
        ),
        ("regional flag", "🇺🇳", vec!["🇺🇳"]),
        ("emoji zwj role", "👩\u{200d}💻", vec!["👩\u{200d}💻"]),
        (
            "hangul jamo",
            "\u{1100}\u{1161}\u{11A8}",
            vec!["\u{1100}\u{1161}\u{11A8}"],
        ),
    ];

    for (name, input, expected) in grapheme_fixtures {
        assert_eq!(graphemes(input), expected, "{name}");
        assert_eq!(grapheme_len(input), expected.len(), "{name}");
        assert_eq!(take_graphemes(input, expected.len()), input, "{name}");
    }

    let word_fixtures = [
        ("apostrophe", "can't stop", vec!["can't", "stop"]),
        ("decimal", "jump 32.3 feet", vec!["jump", "32.3", "feet"]),
        ("cjk", "你好世界", vec!["你", "好", "世", "界"]),
        (
            "mixed punctuation",
            "Rust-go, 世界!",
            vec!["Rust", "go", "世", "界"],
        ),
    ];

    for (name, input, expected) in word_fixtures {
        assert_eq!(unicode_words(input), expected, "{name}");
        assert_eq!(unicode_word_len(input), expected.len(), "{name}");
        assert_eq!(split_word_bounds(input).concat(), input, "{name}");
        assert_eq!(
            split_word_bound_indices(input)
                .into_iter()
                .map(|(_, segment)| segment)
                .collect::<Vec<_>>()
                .concat(),
            input,
            "{name}"
        );
    }

    let sentence_fixtures = [
        (
            "terminal punctuation",
            "One. Two? Three!",
            vec!["One. ", "Two? ", "Three!"],
        ),
        (
            "abbreviation",
            "Mr. Fox jumped. The dog slept.",
            vec!["Mr. ", "Fox jumped. ", "The dog slept."],
        ),
        (
            "separator span",
            "[...] The dog was too lazy.",
            vec!["[...] ", "The dog was too lazy."],
        ),
    ];

    for (name, input, expected_bounds) in sentence_fixtures {
        assert_eq!(split_sentence_bounds(input), expected_bounds, "{name}");
        assert_eq!(split_sentence_bounds(input).concat(), input, "{name}");
        assert_eq!(
            split_sentence_bound_indices(input)
                .into_iter()
                .map(|(_, segment)| segment)
                .collect::<Vec<_>>()
                .concat(),
            input,
            "{name}"
        );
    }
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

#[cfg(feature = "unicode-normalization")]
#[test]
fn unicode_normalization_forms_follow_uax15_golden_cases() {
    // Canonical: precomposed <-> base + combining mark.
    let precomposed = "\u{e9}"; // é
    let decomposed = "e\u{301}"; // e + combining acute
    assert_eq!(nfc(decomposed), precomposed);
    assert_eq!(nfc(precomposed), precomposed);
    assert_eq!(nfd(precomposed), decomposed);
    assert_eq!(nfd(decomposed), decomposed);

    // NFC and NFD are idempotent and inverse under the canonical mapping.
    assert_eq!(nfc(&nfd(precomposed)), precomposed);
    assert_eq!(nfd(&nfc(decomposed)), decomposed);

    // Compatibility: full-width Latin and ligatures fold under NFKC/NFKD.
    assert_eq!(nfkc("\u{ff21}\u{ff22}\u{ff23}"), "ABC");
    assert_eq!(nfkd("\u{ff21}\u{ff22}\u{ff23}"), "ABC");
    assert_eq!(nfkc("\u{fb01}"), "fi");
    assert_eq!(nfkd("\u{fb01}"), "fi");

    // Compatibility composition still recomposes canonical accents.
    assert_eq!(nfkc(decomposed), precomposed);
    assert_eq!(nfkd(precomposed), decomposed);

    // Empty and ASCII inputs are unchanged across every form.
    for form in [nfc, nfd, nfkc, nfkd] {
        assert_eq!(form(""), "");
        assert_eq!(form("ascii"), "ascii");
    }
}

#[cfg(feature = "unicode-normalization")]
#[test]
fn unicode_normalization_quick_checks_match_transform_output() {
    let precomposed = "\u{e9}";
    let decomposed = "e\u{301}";
    let fullwidth = "\u{ff21}";

    assert!(is_nfc(precomposed));
    assert!(!is_nfc(decomposed));
    assert!(is_nfd(decomposed));
    assert!(!is_nfd(precomposed));

    assert!(is_nfkc("ABC"));
    assert!(!is_nfkc(fullwidth));
    assert!(is_nfkd(decomposed));
    assert!(!is_nfkd(fullwidth));

    // Quick checks agree with the transform helpers on already-normalized input.
    assert_eq!(is_nfc(precomposed), nfc(precomposed) == precomposed);
    assert_eq!(is_nfd(decomposed), nfd(decomposed) == decomposed);
    assert_eq!(is_nfkc("ABC"), nfkc("ABC") == "ABC");
    assert_eq!(is_nfkd(decomposed), nfkd(decomposed) == decomposed);

    // ASCII and empty are normalized under all four forms.
    assert!(is_nfc("") && is_nfd("") && is_nfkc("") && is_nfkd(""));
    assert!(is_nfc("ascii") && is_nfd("ascii") && is_nfkc("ascii") && is_nfkd("ascii"));
}
