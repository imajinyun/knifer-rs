use crate::vstr::*;

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
    assert_eq!(slugify("Crème Brûlée"), "creme-brulee");
    assert_eq!(slugify_with_separator("Déjà Vu", '_'), "deja_vu");
}

#[test]
fn deburr_folds_latin_diacritics_and_ligatures() {
    assert_eq!(deburr("déjà vu"), "deja vu");
    assert_eq!(deburr("Crème Brûlée"), "Creme Brulee");
    assert_eq!(deburr("Æther Œuvre ß"), "Aether Oeuvre ss");
    assert_eq!(deburr("Þór þing"), "Thor thing");
    // Decomposed accents: base letter plus combining mark folds to the base.
    assert_eq!(deburr("e\u{0301}"), "e");
    assert_eq!(deburr("A\u{0300}B\u{0308}"), "AB");
    // Non-Latin scripts, digits, and emoji are preserved unchanged.
    assert_eq!(deburr("你好 café 123 🚀"), "你好 cafe 123 🚀");
    assert_eq!(deburr("Ω Σ Д"), "Ω Σ Д");
    assert_eq!(deburr(""), "");
    assert_eq!(remove_accents("Crème Brûlée"), "Creme Brulee");
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
fn wrap_with_options_exposes_scalar_layout_policy() {
    assert_eq!(
        wrap_with_options("hello rust world", &WrapOptions::new(10)),
        wrap("hello rust world", 10)
    );
    assert_eq!(
        wrap_with_options(
            "api/v1/users",
            &WrapOptions::new(7).with_word_separators(&['/'])
        ),
        "api/v1/\nusers"
    );
    assert_eq!(
        wrap_with_options(
            "a   b\tc",
            &WrapOptions::new(4).with_whitespace_mode(WhitespaceMode::Preserve)
        ),
        "a   \nb\tc"
    );
    assert_eq!(
        wrap_with_options(
            "superlongword",
            &WrapOptions::new(5).with_long_word_policy(LongWordPolicy::Preserve)
        ),
        "superlongword"
    );
    assert_eq!(
        wrap_with_options(
            "alpha beta gamma",
            &WrapOptions::new(9).with_indent("> ", "..")
        ),
        "> alpha\n..beta\n..gamma"
    );
    assert_eq!(wrap_with_options("ignored", &WrapOptions::new(0)), "");
}

#[cfg(feature = "unicode-width")]
#[test]
fn wrap_width_with_options_exposes_display_layout_policy() {
    assert_eq!(
        wrap_width_with_options("你好Rust world", &WrapOptions::new(8)),
        wrap_width("你好Rust world", 8)
    );
    assert_eq!(
        wrap_width_with_options(
            "路径/api  用户",
            &WrapOptions::new(6)
                .with_word_separators(&['/'])
                .with_whitespace_mode(WhitespaceMode::Preserve)
                .with_long_word_policy(LongWordPolicy::Preserve)
        ),
        "路径/\napi  \n用户"
    );
    assert_eq!(
        wrap_width_with_options(
            "你好世界Rust",
            &WrapOptions::new(6).with_long_word_policy(LongWordPolicy::Preserve)
        ),
        "你好世界Rust"
    );
    assert_eq!(
        wrap_width_with_options(
            "alpha 你好 beta",
            &WrapOptions::new(9).with_indent("=> ", "   ")
        ),
        "=> alpha\n   你好\n   beta"
    );
}
