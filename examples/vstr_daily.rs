//! Daily `vstr` helpers for business string handling.

use knifer_rs::vstr;

fn main() {
    assert_eq!(vstr::trim("  hello  "), "hello");
    assert_eq!(vstr::trim_to_empty("  hello  "), "hello");
    assert!(vstr::is_blank(" \n\t"));
    assert_eq!(vstr::default_if_blank(" ", "fallback"), "fallback");

    assert_eq!(vstr::split_trim(" a, ,b ", ","), vec!["a", "b"]);
    assert_eq!(vstr::sub("你好世界", 1, -1), "好世");
    assert_eq!(vstr::sub_after("a/b/c", "/", true), "c");
    assert_eq!(vstr::between("id=[42]", "[", "]"), Some("42"));
    assert_eq!(vstr::split_once_last("a=b=c", "="), Some(("a=b", "c")));

    assert_eq!(vstr::to_snake_case("helloWorld ID"), "hello_world_id");
    assert_eq!(vstr::to_train_case("HTTPServerID"), "Http-Server-Id");
    assert_eq!(vstr::to_dot_case("helloWorld ID"), "hello.world.id");
    assert_eq!(vstr::capitalize("rUST"), "Rust");

    assert_eq!(vstr::remove_whitespace(" a\n b\t "), "ab");
    assert_eq!(vstr::normalize_newlines("a\r\nb\rc"), "a\nb\nc");
    assert_eq!(vstr::trim_lines("  a  \n\tb\t\n"), "a\nb\n");
    assert_eq!(vstr::trim_blank_lines("\n  \nhello\n\n"), "hello");
    assert_eq!(vstr::slugify("Hello, Rust World!"), "hello-rust-world");

    assert_eq!(vstr::take_chars("你好Rust", 3), "你好R");
    assert_eq!(vstr::truncate_with_suffix("你好Rust", 5, "..."), "你好...");
    assert_eq!(
        vstr::abbreviate_middle("abcdefghijklmnopqrstuvwxyz", 10, "..."),
        "abcd...xyz"
    );
    assert_eq!(vstr::mask("13800138000", 3, 4, '*'), "138****8000");
    assert_eq!(vstr::collapse_repeated_char("a---b----c", '-'), "a-b-c");
    assert_eq!(vstr::center("rust", 9, '-'), "--rust---");

    let wrap_options = vstr::WrapOptions::new(7).with_word_separators(&['/']);
    assert_eq!(
        vstr::wrap_with_options("api/v1/users", &wrap_options),
        "api/v1/\nusers"
    );

    let preserve_options =
        vstr::WrapOptions::new(4).with_whitespace_mode(vstr::WhitespaceMode::Preserve);
    assert_eq!(
        vstr::wrap_with_options("a   b", &preserve_options),
        "a   \nb"
    );

    let long_word_options =
        vstr::WrapOptions::new(5).with_long_word_policy(vstr::LongWordPolicy::Preserve);
    assert_eq!(
        vstr::wrap_with_options("superlongword", &long_word_options),
        "superlongword"
    );

    assert_eq!(vstr::wrap("hello rust world", 10), "hello rust\nworld");
    assert_eq!(
        vstr::wrap_with_indent("hello rust world", 12, "* ", "  "),
        "* hello rust\n  world"
    );
    assert_eq!(vstr::non_blank_lines(" a \n\n b "), vec!["a", "b"]);
    assert_eq!(vstr::initials("rust string toolkit"), "RST");
    assert!(vstr::is_palindrome("A man, a plan, a canal: Panama"));
    assert_eq!(vstr::extract_digits("id=42, رقم=٣"), "42٣");
}
