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
