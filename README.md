# knifer-rs

`knifer-rs` is a Safe Rust utility toolkit for everyday business development.

The project follows the same practical direction as `knifer-go`: keep public
APIs grouped by focused `v*` facade modules, prefer standard-library behavior
where it is already clear, and add reusable helpers only when they make common
business code easier to read and test.

## Benchmark Direction

`knifer-rs` is benchmarked against Rust helper and utility-function crates such
as `anyhow`, `thiserror`, `serde_json`, `regex`, and `chrono`: small crates that
ordinary business code can call directly. The goal is not to clone their APIs,
but to match their engineering discipline:

- clear first-screen README examples
- stable and small public APIs
- documented edge-case behavior
- strict fmt/test/clippy/rustdoc CI
- explicit MSRV and Safe Rust policy
- low repository noise through `.gitignore`, `.editorconfig`, and
  `.gitattributes`

See `docs/top-rust-utility-gap-analysis.md` for the general utility-crate gap
analysis and `docs/vstr-top-string-gap-analysis.md` for the focused string and
text-processing benchmark set. Complexity and allocation notes for `vstr` live
in `docs/vstr-complexity.md`. Dependency admission rules live in
`docs/dependency-policy.md`.

## Status

The crate is pre-1.0 and currently exposes one MVP-stable facade:
`knifer_rs::vstr`. Public API changes are tracked in
`docs/public-api-inventory.md`; `vstr` compatibility notes are tracked in
`docs/vstr-api-parity.md`.

## Install

```toml
[dependencies]
knifer-rs = "0.1"
```

The package is not published yet. Use a Git dependency while evaluating local
MVP builds:

```toml
[dependencies]
knifer-rs = { git = "https://github.com/imajinyun/knifer-rs" }
```

## Current MVP

The first facade is `vstr`, covering small string helpers:

```rust
use knifer_rs::vstr;

assert_eq!(vstr::trim("  hello  "), "hello");
assert_eq!(vstr::trim_to_empty("  hello  "), "hello");
assert!(vstr::is_blank(" \n\t"));
assert_eq!(vstr::default_if_blank(" ", "fallback"), "fallback");
assert!(vstr::contains_any("knifer-rs", ["go", "rs"]));
assert!(vstr::contains_emoji("ship it 🚀"));
assert_eq!(vstr::remove_emoji("ship 🚀 now"), "ship  now");
let emoji_opts = vstr::with_emoji_matcher(|input| input.contains(":rocket:"));
assert!(vstr::contains_emoji_with_options("ship :rocket:", &emoji_opts));
assert_eq!(vstr::split_trim(" a, ,b ", ","), vec!["a", "b"]);
assert_eq!(vstr::sub("你好世界", 1, -1), "好世");
assert_eq!(vstr::sub_after("a/b/c", "/", true), "c");
assert_eq!(vstr::between("id=[42]", "[", "]"), Some("42"));
assert_eq!(vstr::split_once_last("a=b=c", "="), Some(("a=b", "c")));
assert_eq!(vstr::pad_left("42", 5, '0'), "00042");
assert_eq!(vstr::to_snake_case("helloWorld ID"), "hello_world_id");
assert_eq!(vstr::to_screaming_snake_case("HTTPServerID"), "HTTP_SERVER_ID");
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
assert_eq!(vstr::abbreviate_middle("abcdefghijklmnopqrstuvwxyz", 10, "..."), "abcd...xyz");
assert_eq!(vstr::mask("13800138000", 3, 4, '*'), "138****8000");
assert_eq!(vstr::collapse_repeated_char("a---b----c", '-'), "a-b-c");
assert_eq!(vstr::center("rust", 9, '-'), "--rust---");
assert_eq!(vstr::wrap("hello rust world", 10), "hello rust\nworld");
assert_eq!(vstr::wrap_with_indent("hello rust world", 12, "* ", "  "), "* hello rust\n  world");
assert_eq!(vstr::non_blank_lines(" a \n\n b "), vec!["a", "b"]);
assert_eq!(vstr::initials("rust string toolkit"), "RST");
assert!(vstr::is_palindrome("A man, a plan, a canal: Panama"));
assert_eq!(vstr::extract_digits("id=42, رقم=٣"), "42٣");
assert!(vstr::contains_ignore_case("Knifer-RS", "rs"));
assert_eq!(vstr::find_any("hello rust", ["go", "rust"]), Some(("rust", 6, 10)));
assert_eq!(vstr::find_all("aaaa", "aa"), vec![(0, 2), (2, 4)]);
assert_eq!(vstr::find_all_ignore_case("Go go Rust", "go"), vec![(0, 2), (3, 5)]);
assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
assert_eq!(vstr::count_matches("aaaa", "aa"), 2);
assert_eq!(vstr::replace_ignore_case("Go go Rust", "go", "rs"), "rs rs Rust");
assert_eq!(vstr::replace_many("hello rust world", [("hello", "hi"), ("world", "team")]), "hi rust team");
assert_eq!(vstr::escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
assert_eq!(vstr::quote_meta("[rust]"), r"\[rust\]");
assert_eq!(vstr::format("name={}, age={}", &[&"tom", &12]), "name=tom, age=12");
assert_eq!(vstr::add_prefix_if_not("path", "/"), "/path");
assert!(vstr::ant_path_match("/api/**", "/api/v1/users"));
assert_eq!(vstr::escape_html("<b>Tom&Jerry</b>"), "&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;");
assert_eq!(vstr::escape_unicode("Rust你好"), "Rust\\u4F60\\u597D");
assert_eq!(vstr::levenshtein_distance("kitten", "sitting"), 3);
assert_eq!(vstr::hamming_distance64(0b1010, 0b0011), 2);
```

## Project Layout

```text
src/
  lib.rs       crate entry point and public facade exports
  vstr/
    mod.rs     vstr facade and public re-exports
    basic.rs   common string helpers
    case.rs    case conversion helpers
    classify.rs character classification helpers
    emoji.rs   emoji detection and removal helpers
    encoding.rs HTML and Unicode escaping helpers
    path.rs    Ant-style path matching helpers
    similarity.rs text similarity and fingerprint helpers
    text.rs    whitespace normalization, truncation, and slug helpers
```

## Development Checks

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
bash bin/check-vstr-benchmark-smoke.sh
bash bin/check-vstr-bench.sh
cargo package --list --allow-dirty
```

Benchmark smoke and benchmark suite have different jobs. The smoke command is a
fast CI coverage check that proves expensive `vstr` paths still execute and emit
expected labels. The formal `cargo bench` target is the stable benchmark entry
point for local performance comparison and future historical reports.

## Compatibility

- MSRV: Rust 1.85.
- Edition: Rust 2024.
- Safety: unsafe code is forbidden by Cargo lints and checked by the project
  contract script.
- Dependencies: zero runtime dependencies in the current MVP.
