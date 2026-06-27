use knifer_rs::vstr;

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

fn main() {
    for (input, replacements) in corpus() {
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

    assert_eq!(
        vstr::replace_many("aaaa", [("aa", "b"), ("a", "c")]),
        "bb"
    );
    assert_eq!(
        vstr::replace_many("aaaa", [("a", "c"), ("aa", "b")]),
        "cccc"
    );
}
