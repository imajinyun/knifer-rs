#![cfg_attr(fuzzing, no_main)]

use knifer_rs::vstr;

#[cfg(fuzzing)]
use libfuzzer_sys::fuzz_target;

const SEEDS: &str = include_str!("../corpus/escaping.txt");

fn roundtrip_corpus() -> [&'static str; 9] {
    [
        "",
        "literal",
        r".*?^$#&-~\[]{}|",
        "a+b*(c)",
        "<tag attr=\"value\">&text</tag>",
        "emoji 🚀",
        "你好 & rust",
        "line\nbreak",
        "'single' and \"double\"",
    ]
}

fn tolerant_decode_corpus() -> [&'static str; 5] {
    [
        "\\u4F60\\u597D",
        "\\uD83D\\uDE80",
        "\\uD83Dbroken",
        "\\uDE80",
        "\\uZZZZ",
    ]
}

#[cfg(not(fuzzing))]
fn main() {
    for input in roundtrip_corpus() {
        assert_roundtrip(input);
    }

    for input in SEEDS.lines() {
        if input.contains(r"\u") {
            assert_tolerant_decode(input);
        } else {
            assert_roundtrip(input);
        }
    }

    for input in tolerant_decode_corpus() {
        assert_tolerant_decode(input);
    }
}

fn assert_roundtrip(input: &str) {
    let escaped_regex = vstr::escape_regex(input);
    assert_eq!(escaped_regex, vstr::quote_meta(input));
    assert!(escaped_regex.len() >= input.len());

    let escaped_html = vstr::escape_html(input);
    let unescaped_html = vstr::unescape_html(&escaped_html);
    assert_eq!(unescaped_html, input);

    let escaped_unicode = vstr::escape_unicode(input);
    let unescaped_unicode = vstr::unescape_unicode(&escaped_unicode);
    assert_eq!(unescaped_unicode, input);
}

fn assert_tolerant_decode(input: &str) {
    let tolerant_unicode = vstr::unescape_unicode(input);
    assert!(
        tolerant_unicode.is_char_boundary(0) && tolerant_unicode.is_char_boundary(tolerant_unicode.len())
    );
}

#[cfg(fuzzing)]
fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        assert_roundtrip(input);
        assert_tolerant_decode(input);
    }
});
