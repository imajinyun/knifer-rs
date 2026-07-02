//! Search, replacement, matcher, path, and similarity helpers.

use kniferrs::vstr;

fn main() {
    assert!(vstr::contains_any("knifer-rs", ["go", "rs"]));
    assert!(vstr::contains_ignore_case("Knifer-RS", "rs"));
    assert_eq!(
        vstr::find_any("hello rust", ["go", "rust"]),
        Some(("rust", 6, 10))
    );
    assert_eq!(vstr::find_all("aaaa", "aa"), vec![(0, 2), (2, 4)]);
    assert_eq!(
        vstr::find_all_ignore_case("Go go Rust", "go"),
        vec![(0, 2), (3, 5)]
    );

    let matcher = vstr::VStrMatcher::with_kind(["a", "aa"], vstr::MatchKind::LeftmostLongest);
    assert_eq!(matcher.find_overlapping("aaaa").len(), 4);

    assert_eq!(
        vstr::strip_suffix_ignore_case("Knifer-RS", "rs"),
        Some("Knifer-")
    );
    assert_eq!(vstr::count_matches("aaaa", "aa"), 2);
    assert_eq!(
        vstr::replace_ignore_case("Go go Rust", "go", "rs"),
        "rs rs Rust"
    );
    assert_eq!(
        vstr::replace_many("hello rust world", [("hello", "hi"), ("world", "team")]),
        "hi rust team"
    );
    assert_eq!(vstr::escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
    assert_eq!(vstr::quote_meta("[rust]"), r"\[rust\]");

    #[cfg(feature = "pattern-regex")]
    assert_eq!(
        vstr::find_pattern("ticket-42", r"\d+").unwrap(),
        Some((7, 9))
    );

    assert_eq!(
        vstr::format("name={}, age={}", &[&"tom", &12]),
        "name=tom, age=12"
    );
    assert_eq!(vstr::add_prefix_if_not("path", "/"), "/path");
    assert!(vstr::ant_path_match("/api/**", "/api/v1/users"));
    assert_eq!(
        vstr::escape_html("<b>Tom&Jerry</b>"),
        "&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;"
    );
    assert_eq!(vstr::escape_unicode("Rust你好"), "Rust\\u4F60\\u597D");
    assert_eq!(vstr::levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(vstr::hamming_distance64(0b1010, 0b0011), 2);
}
