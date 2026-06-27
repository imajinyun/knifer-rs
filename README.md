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

See `docs/top-rust-utility-gap-analysis.md` for the current gap analysis.

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
assert_eq!(vstr::remove_whitespace(" a\n b\t "), "ab");
assert_eq!(vstr::slugify("Hello, Rust World!"), "hello-rust-world");
assert_eq!(vstr::take_chars("你好Rust", 3), "你好R");
assert_eq!(vstr::truncate_with_suffix("你好Rust", 5, "..."), "你好...");
assert_eq!(vstr::wrap("hello rust world", 10), "hello rust\nworld");
assert_eq!(vstr::non_blank_lines(" a \n\n b "), vec!["a", "b"]);
assert!(vstr::contains_ignore_case("Knifer-RS", "rs"));
assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
assert_eq!(vstr::count_matches("aaaa", "aa"), 2);
assert_eq!(vstr::replace_ignore_case("Go go Rust", "go", "rs"), "rs rs Rust");
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
cargo package --list --allow-dirty
```

## Compatibility

- MSRV: Rust 1.85.
- Edition: Rust 2024.
- Safety: unsafe code is forbidden by Cargo lints and checked by the project
  contract script.
- Dependencies: zero runtime dependencies in the current MVP.
