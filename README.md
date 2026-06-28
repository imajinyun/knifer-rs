# knifer-rs ✨

`knifer-rs` is a Safe Rust utility toolkit for everyday business development.

The project follows the same practical direction as `knifer-go`: keep public
APIs grouped by focused `v*` facade modules, prefer standard-library behavior
where it is already clear, and add reusable helpers only when they make common
business code easier to read and test.

## Benchmark Direction 📊

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

## Status 🚦

The crate is pre-1.0 and currently exposes three MVP facades:
`knifer_rs::vstr` for valid UTF-8 strings, `knifer_rs::vbytes` for byte slices
that may not be valid UTF-8, and `knifer_rs::vencoding` for BOM and UTF-8
encoding boundaries. Public API changes are tracked in
`docs/public-api-inventory.md`; `vstr` compatibility notes are tracked in
`docs/vstr-api-parity.md`.

## Install 📦

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

## Current MVP 🧰

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
#[cfg(feature = "unicode-segmentation")]
assert_eq!(vstr::take_graphemes("e\u{301}🇨🇳rust", 2), "e\u{301}🇨🇳");
#[cfg(feature = "unicode-segmentation")]
assert_eq!(
    vstr::unicode_words("Rust can't stop 32.3 世界!"),
    vec!["Rust", "can't", "stop", "32.3", "世", "界"]
);
#[cfg(feature = "unicode-segmentation")]
assert_eq!(
    vstr::unicode_sentences("Mr. Fox jumped. [...] The dog was too lazy."),
    vec!["Mr. ", "Fox jumped. ", "The dog was too lazy."]
);
#[cfg(feature = "unicode-width")]
assert_eq!(vstr::display_width("abc你好"), 7);
#[cfg(feature = "unicode-width")]
assert_eq!(vstr::take_width("👨‍👩‍👧‍👦 family", 2), "👨‍👩‍👧‍👦");
#[cfg(feature = "unicode-width")]
assert_eq!(vstr::truncate_width("你好Rust", 6, "..."), "你...");
#[cfg(feature = "unicode-width")]
assert_eq!(vstr::wrap_width("你好Rust world", 8), "你好Rust\nworld");
let wrap_options = vstr::WrapOptions::new(7).with_word_separators(&['/']);
assert_eq!(vstr::wrap_with_options("api/v1/users", &wrap_options), "api/v1/\nusers");
let preserve_options =
    vstr::WrapOptions::new(4).with_whitespace_mode(vstr::WhitespaceMode::Preserve);
assert_eq!(vstr::wrap_with_options("a   b", &preserve_options), "a   \nb");
let long_word_options =
    vstr::WrapOptions::new(5).with_long_word_policy(vstr::LongWordPolicy::Preserve);
assert_eq!(vstr::wrap_with_options("superlongword", &long_word_options), "superlongword");
#[cfg(feature = "unicode-width")]
assert_eq!(
    vstr::wrap_width_with_options("路径/api  用户", &vstr::WrapOptions::new(6).with_word_separators(&['/'])),
    "路径/\napi\n用户"
);
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
let matcher = vstr::VStrMatcher::with_kind(["a", "aa"], vstr::MatchKind::LeftmostLongest);
assert_eq!(matcher.find_overlapping("aaaa").len(), 4);
assert_eq!(vstr::strip_suffix_ignore_case("Knifer-RS", "rs"), Some("Knifer-"));
assert_eq!(vstr::count_matches("aaaa", "aa"), 2);
assert_eq!(vstr::replace_ignore_case("Go go Rust", "go", "rs"), "rs rs Rust");
assert_eq!(vstr::replace_many("hello rust world", [("hello", "hi"), ("world", "team")]), "hi rust team");
assert_eq!(vstr::escape_regex("a+b*(c)"), r"a\+b\*\(c\)");
assert_eq!(vstr::quote_meta("[rust]"), r"\[rust\]");
#[cfg(feature = "pattern-regex")]
assert_eq!(vstr::find_pattern("ticket-42", r"\d+").unwrap(), Some((7, 9)));
assert_eq!(vstr::format("name={}, age={}", &[&"tom", &12]), "name=tom, age=12");
assert_eq!(vstr::add_prefix_if_not("path", "/"), "/path");
assert!(vstr::ant_path_match("/api/**", "/api/v1/users"));
assert_eq!(vstr::escape_html("<b>Tom&Jerry</b>"), "&lt;b&gt;Tom&amp;Jerry&lt;/b&gt;");
assert_eq!(vstr::escape_unicode("Rust你好"), "Rust\\u4F60\\u597D");
assert_eq!(vstr::levenshtein_distance("kitten", "sitting"), 3);
assert_eq!(vstr::hamming_distance64(0b1010, 0b0011), 2);
```

`vbytes` keeps byte-oriented helpers separate from `vstr` so invalid UTF-8 does
not complicate string semantics:

```rust
use knifer_rs::vbytes;

let input = [b'a', 0xff, b'b'];
assert_eq!(vbytes::byte_len(&input), 3);
assert!(!vbytes::is_utf8(&input));
assert_eq!(vbytes::sub(&input, 1, 2), &[0xff]);
assert_eq!(vbytes::find_all(b"aaaa", b"aa"), vec![(0, 2), (2, 4)]);
assert_eq!(vbytes::replace_all(&input, &[0xff], b"?"), b"a?b");
```

`vencoding` keeps BOM handling and UTF-8 decoding policy explicit:

```rust
use knifer_rs::vencoding::{self, Bom};

let input = [0xEF, 0xBB, 0xBF, b'a', 0xff];
assert_eq!(vencoding::detect_bom(&input), Some(Bom::Utf8));
assert_eq!(vencoding::strip_bom(&input), &[b'a', 0xff]);
assert!(vencoding::validate_utf8_without_bom(&input).is_err());
assert_eq!(vencoding::decode_utf8_lossy_without_bom(&input), "a\u{FFFD}");
```

## Project Layout 🧭

```text
src/
  lib.rs       crate entry point and public facade exports
  vbytes.rs    byte-slice helpers for possibly invalid UTF-8
  vencoding.rs BOM and UTF-8 validation/decoding helpers
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

## Development Checks ✅

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
bash bin/check-api-semver.sh
bash bin/check-release-api-semver.sh
bash bin/check-vstr-benchmark-smoke.sh
bash bin/check-vstr-bench.sh
bash bin/check-vstr-fuzz-smoke.sh
bash bin/check-docs-rs-ready.sh
```

Benchmark smoke and benchmark suite have different jobs. The smoke command is a
fast CI coverage check that proves expensive `vstr` paths still execute and emit
expected labels. The formal `cargo bench` target is the stable benchmark entry
point for local performance comparison and future historical reports.

To generate release-grade benchmark artifacts locally, pass an output directory:

```bash
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

That writes `vstr-bench.txt`, `vstr-bench.json`, and `vstr-bench.md`. The same
entry point is used by the manual GitHub Actions benchmark workflow when
`run_release_bench` is set to `true`.

To compare a run against a saved baseline report, point the script at a previous
`vstr-bench.json` artifact:

```bash
VSTR_BENCH_BASELINE_JSON=target/vstr-bench-report/vstr-bench.json \
VSTR_BENCH_MAX_REGRESSION_PCT=20.00 \
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

For commit-to-commit comparison, set `VSTR_BENCH_BASE_REF` instead:

```bash
VSTR_BENCH_BASE_REF=main \
VSTR_BENCH_MAX_REGRESSION_PCT=20.00 \
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

When a baseline is present, the script also writes `vstr-bench-compare.json` and
`vstr-bench-compare.md`, and fails if any benchmark exceeds the configured
regression threshold. Baseline selection, refresh rules, and threshold policy
are documented in [docs/vstr-benchmark-history.md](docs/vstr-benchmark-history.md).

Fuzz smoke targets live under `fuzz/` as a separate local crate. They cover
substring boundaries, escaping, Ant-style path matching, and replacement
invariants without adding runtime dependencies to the main library. The
text-boundary target also covers wrap, truncation, abbreviation, masking,
centering, and whitespace invariants.

Public API checks are intentionally split. `check-public-api-inventory.sh`
ensures the generated signature snapshot is in sync, while
`check-api-semver.sh` classifies removed or changed signatures as breaking and
new signatures as additive inventory work. `check-release-api-semver.sh` keeps
that fast local check and, when a release baseline is configured, runs
`cargo-semver-checks check-release`.

For release review against a git tag or branch, install `cargo-semver-checks`
and pass a baseline ref:

```bash
cargo install cargo-semver-checks --locked
API_SEMVER_BASELINE_REF=v0.1.0 \
API_SEMVER_REQUIRED=true \
bash bin/check-release-api-semver.sh
```

The manual GitHub Actions workflow exposes `run_release_api_semver` and
`api_semver_baseline_ref` for the same release-grade API gate.

Optional `pattern-regex` helpers are available for callers that want regex-backed
matching while keeping the default feature set at zero runtime dependencies:

```toml
[dependencies]
knifer-rs = { version = "0.1", features = ["pattern-regex"] }
```

Optional `unicode-segmentation` helpers add grapheme and word boundaries for UI
and human-facing text without changing scalar-based helpers such as
`take_chars`:

```toml
[dependencies]
knifer-rs = { version = "0.1", features = ["unicode-segmentation"] }
```

Optional `unicode-width` helpers add terminal display-cell measurement and
wrapping/truncation for CJK, combining marks, and emoji ZWJ text:

```toml
[dependencies]
knifer-rs = { version = "0.1", features = ["unicode-width"] }
```

The docs.rs readiness check is the local publish gate. It verifies crate
metadata, builds rustdoc with the docs.rs configuration and all features, and
runs `cargo package --locked --allow-dirty`.

## Compatibility 🔒

- MSRV: Rust 1.85.
- Edition: Rust 2024.
- Safety: unsafe code is forbidden by Cargo lints and checked by the project
  contract script.
- Dependencies: zero runtime dependencies in the default feature set; optional
  features add focused crates such as `regex`, `unicode-segmentation`, and
  `unicode-width`.
