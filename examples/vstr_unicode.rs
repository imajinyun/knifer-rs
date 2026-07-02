//! Optional Unicode helpers for grapheme, word, sentence, and display width.

#[cfg(any(feature = "unicode-segmentation", feature = "unicode-width"))]
use kniferrs::vstr;

fn main() {
    #[cfg(feature = "unicode-segmentation")]
    {
        assert_eq!(vstr::take_graphemes("e\u{301}🇨🇳rust", 2), "e\u{301}🇨🇳");
        assert_eq!(
            vstr::unicode_words("Rust can't stop 32.3 世界!"),
            vec!["Rust", "can't", "stop", "32.3", "世", "界"]
        );
        assert_eq!(
            vstr::unicode_sentences("Mr. Fox jumped. [...] The dog was too lazy."),
            vec!["Mr. ", "Fox jumped. ", "The dog was too lazy."]
        );
    }

    #[cfg(feature = "unicode-width")]
    {
        assert_eq!(vstr::display_width("abc你好"), 7);
        assert_eq!(vstr::take_width("👨‍👩‍👧‍👦 family", 2), "👨‍👩‍👧‍👦");
        assert_eq!(vstr::truncate_width("你好Rust", 6, "..."), "你...");
        assert_eq!(vstr::wrap_width("你好Rust world", 8), "你好Rust\nworld");
        assert_eq!(
            vstr::wrap_width_with_options(
                "路径/api  用户",
                &vstr::WrapOptions::new(6).with_word_separators(&['/']),
            ),
            "路径/\napi\n用户"
        );
    }
}
