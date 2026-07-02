use crate::vstr::*;

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
fn html_unescape_decodes_numeric_named_and_preserves_unknown() {
    // Decimal and hexadecimal numeric references, including astral code points.
    assert_eq!(unescape_html("caf&#233; &#x1F600;"), "café 😀");
    assert_eq!(unescape_html("&#65;&#x42;&#67;"), "ABC");

    // Curated named references beyond the escape set.
    assert_eq!(unescape_html("2&nbsp;&mdash;&nbsp;3"), "2\u{a0}—\u{a0}3");
    assert_eq!(unescape_html("&copy;&reg;&trade;"), "©®™");
    assert_eq!(unescape_html("&frac12;&euro;&hellip;"), "½€…");

    // Single left-to-right pass: an escaped ampersand is expanded once only.
    assert_eq!(unescape_html("&amp;lt;"), "&lt;");

    // Unknown names, bare ampersands, and invalid scalars are preserved.
    assert_eq!(unescape_html("Tom & Jerry"), "Tom & Jerry");
    assert_eq!(unescape_html("&unknown;"), "&unknown;");
    assert_eq!(unescape_html("&#;"), "&#;");
    assert_eq!(unescape_html("&#xD800;"), "&#xD800;");
    assert_eq!(unescape_html("&amp"), "&amp");
}

#[test]
fn strip_tags_removes_markup_and_respects_quotes_and_comments() {
    assert_eq!(strip_tags("<b>Hello</b> <i>World</i>"), "Hello World");
    assert_eq!(strip_tags(r#"<a title="x > y">link</a>"#), "link");
    assert_eq!(strip_tags("<a title='a > b'>x</a>"), "x");
    assert_eq!(strip_tags("keep <!-- x > y --> me"), "keep  me");

    // Unterminated tags and comments are kept as literal text.
    assert_eq!(strip_tags("1 < 2 and unclosed"), "1 < 2 and unclosed");
    assert_eq!(
        strip_tags("open <!-- never closed"),
        "open <!-- never closed"
    );

    // Multi-byte content between tags is preserved on scalar boundaries.
    assert_eq!(strip_tags("<p>café 你好</p>"), "café 你好");
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
fn knifer_go_vstr_golden_fixtures_cover_unicode_escape() {
    assert_eq!(escape_unicode("中国"), "\\u4E2D\\u56FD");
    assert_eq!(unescape_unicode("\\u4E2D\\u56FD"), "中国");
}
