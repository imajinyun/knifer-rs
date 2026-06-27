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
assert_eq!(vstr::pad_left("42", 5, '0'), "00042");
assert_eq!(vstr::to_snake_case("helloWorld ID"), "hello_world_id");
assert!(vstr::contains_ignore_case("Knifer-RS", "rs"));
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
```

## Development Checks

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-project-contract.sh
```

The crate forbids unsafe code through Cargo lints.
