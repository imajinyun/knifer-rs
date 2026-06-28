#![cfg_attr(fuzzing, no_main)]

use knifer_rs::vstr;

#[cfg(fuzzing)]
use libfuzzer_sys::fuzz_target;

const SEEDS: &str = include_str!("../corpus/replacement.txt");

fn corpus() -> [(&'static str, &'static [(&'static str, &'static str)]); 8] {
    [
        ("", &[("a", "b")]),
        ("hello rust world", &[("hello", "hi"), ("world", "team")]),
        ("aaaa", &[("aa", "b"), ("a", "c")]),
        ("aaaa", &[("a", "c"), ("aa", "b")]),
        ("你好你好", &[("你好", "hi")]),
        ("emoji 🚀 rocket", &[("🚀", ":rocket:")]),
        ("Case CASE case", &[("case", "word")]),
        ("skip-empty", &[("", "x"), ("skip", "take")]),
    ]
}

#[cfg(not(fuzzing))]
fn main() {
    for (input, replacements) in corpus() {
        assert_replacement_invariants(input, replacements);
    }

    let seed_replacements = [("a", "A"), ("你", "N"), ("🚀", "R"), ("line", "row")];
    for input in SEEDS.lines() {
        assert_replacement_invariants(input, &seed_replacements);
    }

    assert_eq!(
        vstr::replace_many("aaaa", [("aa", "b"), ("a", "c")]),
        "bb"
    );
    assert_eq!(
        vstr::replace_many("aaaa", [("a", "c"), ("aa", "b")]),
        "cccc"
    );
}

fn assert_replacement_invariants(input: &str, replacements: &[(&str, &str)]) {
    let replaced = vstr::replace_many(input, replacements.iter().copied());
    assert!(replaced.is_char_boundary(0) && replaced.is_char_boundary(replaced.len()));

    let without_empty: Vec<(&str, &str)> = replacements
        .iter()
        .copied()
        .filter(|(from, _)| !from.is_empty())
        .collect();
    assert_eq!(replaced, vstr::replace_many(input, without_empty));
    assert_eq!(vstr::replace_many(input, [("", "ignored")]), input);

    assert!(vstr::replace_first(input, "", "x").is_char_boundary(0));
    assert!(vstr::replace_last(input, "", "x").is_char_boundary(0));

    let insensitive = vstr::replace_ignore_case(input, "case", "word");
    assert!(insensitive.is_char_boundary(0) && insensitive.is_char_boundary(insensitive.len()));
}

#[cfg(fuzzing)]
fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let replacements = [("a", "A"), ("你", "N"), ("🚀", "R"), ("line", "row")];
        assert_replacement_invariants(input, &replacements);
    }
});
