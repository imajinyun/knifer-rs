#![cfg_attr(fuzzing, no_main)]

use kniferrs::vstr;

#[cfg(fuzzing)]
use libfuzzer_sys::fuzz_target;

const SEEDS: &str = include_str!("../corpus/text_boundaries.txt");

fn corpus() -> [&'static str; 12] {
    [
        "",
        "ascii words only",
        "  spaced   words\twith\nlines  ",
        "你好世界 Rust",
        "e\u{301}e\u{301} cafe",
        "🇨🇳 flag text",
        "👨\u{200d}👩\u{200d}👧\u{200d}👦 family",
        "emoji 🚀 text",
        "supercalifragilistic",
        "line\r\nbreak\rcase",
        "नमस्ते rust utility",
        "a---b----c",
    ]
}

fn assert_valid_utf8(output: &str) {
    assert!(output.is_char_boundary(0));
    assert!(output.is_char_boundary(output.len()));
}

fn assert_scalar_budget(output: &str, max_chars: usize) {
    assert!(
        output.chars().count() <= max_chars,
        "output exceeds scalar budget: max={max_chars} output={output:?}"
    );
}

#[cfg(not(fuzzing))]
fn main() {
    let suffixes = ["", ".", "...", "🚀"];
    for input in corpus().into_iter().chain(SEEDS.lines()) {
        assert_text_boundary_invariants(input, suffixes);
    }
}

fn assert_text_boundary_invariants(input: &str, suffixes: [&str; 4]) {
    let char_count = input.chars().count();
    for width in 0..=char_count + 4 {
        let wrapped = vstr::wrap(input, width);
        assert_valid_utf8(&wrapped);
        if width == 0 {
            assert!(wrapped.is_empty());
        } else {
            for line in wrapped.split('\n') {
                assert_scalar_budget(line, width);
            }
        }

        let indented = vstr::wrap_with_indent(input, width, "=> ", "   ");
        assert_valid_utf8(&indented);
        if width == 0 {
            assert!(indented.is_empty());
        }

        for suffix in suffixes {
            let truncated = vstr::truncate_with_suffix(input, width, suffix);
            assert_valid_utf8(&truncated);
            assert_scalar_budget(&truncated, width);

            let abbreviated = vstr::abbreviate_middle(input, width, suffix);
            assert_valid_utf8(&abbreviated);
            assert_scalar_budget(&abbreviated, width);
        }

        let centered = vstr::center(input, width, '-');
        assert_valid_utf8(&centered);
        assert!(centered.chars().count() >= width.min(char_count));
    }

    for visible_start in 0..=char_count + 2 {
        for visible_end in 0..=char_count + 2 {
            let masked = vstr::mask(input, visible_start, visible_end, '*');
            assert_valid_utf8(&masked);
            assert_eq!(masked.chars().count(), char_count);
        }
    }

    let normalized = vstr::normalize_whitespace(input);
    assert_valid_utf8(&normalized);
    assert!(!normalized.starts_with(char::is_whitespace));
    assert!(!normalized.ends_with(char::is_whitespace));

    let no_whitespace = vstr::remove_whitespace(input);
    assert_valid_utf8(&no_whitespace);
    assert!(no_whitespace.chars().all(|ch| !ch.is_whitespace()));
}

#[cfg(fuzzing)]
fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let suffixes = ["", ".", "...", "🚀"];
        assert_text_boundary_invariants(input, suffixes);
    }
});
