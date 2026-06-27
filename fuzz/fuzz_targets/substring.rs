use knifer_rs::vstr;

fn corpus() -> [&'static str; 12] {
    [
        "",
        "ascii",
        "  spaced  ",
        "你好Rust",
        "e\u{301}",
        "🇨🇳 flag",
        "👨\u{200d}👩\u{200d}👧\u{200d}👦",
        "line\r\nbreak",
        "\u{3000}wide space",
        "a/b/c",
        "emoji 🚀 text",
        "नमस्ते rust",
    ]
}

fn assert_char_boundary(input: &str, output: &str) {
    assert!(
        input.contains(output) || output.is_empty(),
        "substring output must be borrowed from input or empty: input={input:?} output={output:?}"
    );
    assert!(
        output.is_char_boundary(0) && output.is_char_boundary(output.len()),
        "substring output must preserve UTF-8 boundaries: {output:?}"
    );
}

fn main() {
    for input in corpus() {
        let char_count = input.chars().count();
        for count in 0..=char_count + 3 {
            assert_char_boundary(input, vstr::take_chars(input, count));
            assert_char_boundary(input, vstr::drop_chars(input, count));
            assert_eq!(
                vstr::take_chars(input, count).chars().count(),
                char_count.min(count)
            );
            assert_eq!(
                vstr::drop_chars(input, count).chars().count(),
                char_count.saturating_sub(count)
            );
        }

        for from in -(char_count as isize + 3)..=(char_count as isize + 3) {
            for to in -(char_count as isize + 3)..=(char_count as isize + 3) {
                let output = vstr::sub(input, from, to);
                assert!(
                    output.is_char_boundary(0) && output.is_char_boundary(output.len()),
                    "sub must return valid UTF-8: input={input:?} from={from} to={to} output={output:?}"
                );
                assert!(output.chars().count() <= char_count);
            }
        }
    }
}
