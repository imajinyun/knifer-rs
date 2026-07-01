# knifer-rs ✨

`knifer-rs` is a Safe Rust utility toolkit for everyday business development.

The project follows the same practical direction as `knifer-go`: keep public
APIs grouped by focused `v*` facade modules, prefer standard-library behavior
where it is already clear, and add reusable helpers only when they make common
business code easier to read and test.

Workflow automation is declared in root `aiflow.yaml`. The `.aiflow/` directory
is local-only and is reserved for generated run evidence, caches, and temporary
state.

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

Complexity and allocation notes for `vstr` live in `docs/vstr-complexity.md`.
Dependency admission rules live in `docs/dependency-policy.md`. Public API
behavior evidence rules live in `docs/api-behavior-contract.md`.

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

## Quick Start ⚡

The default build has zero runtime dependencies and keeps the three public
facades separate:

```rust
use knifer_rs::{vbytes, vencoding, vstr};

assert_eq!(vstr::trim("  hello  "), "hello");
assert_eq!(vstr::sub("你好Rust", 0, 3), "你好R");
assert_eq!(vstr::slugify("Hello, Rust World!"), "hello-rust-world");

let bytes = [b'a', 0xff, b'b'];
assert!(!vbytes::is_utf8(&bytes));
assert_eq!(vbytes::replace_all(&bytes, &[0xff], b"?"), b"a?b");

let bom = [0xEF, 0xBB, 0xBF, b'o', b'k'];
assert_eq!(vencoding::strip_bom(&bom), b"ok");
```

Enable optional features only for callers that need heavier string semantics:

```toml
[dependencies]
knifer-rs = {
  version = "0.1",
  features = ["pattern-regex", "unicode-segmentation", "unicode-width"],
}
```

## Feature Flags 🧩

| Feature | Adds | Default |
| --- | --- | --- |
| `default` | Safe Rust helpers for `vstr`, `vbytes`, and `vencoding` | enabled |
| `matcher-aho-corasick` | optional automaton backend for `VStrMatcher` internals | disabled |
| `pattern-regex` | regex-backed pattern helpers such as `vstr::find_pattern` | disabled |
| `unicode-segmentation` | grapheme, word, and sentence boundary helpers | disabled |
| `unicode-width` | display-cell width, truncation, and wrap helpers | disabled |

## Examples 🧰

The README keeps only the first-screen path. Focused examples live under
`examples/` and are covered by `cargo test --examples`:

- `examples/vstr_daily.rs`: trimming, slicing, case conversion, wrapping,
  masking, and business text cleanup.
- `examples/vstr_unicode.rs`: optional `unicode-segmentation` and
  `unicode-width` helpers.
- `examples/vbytes_encoding.rs`: byte-slice and BOM/UTF-8 boundary helpers.
- `examples/vstr_matcher.rs`: searching, replacement, Ant path matching,
  escaping, matcher behavior, and similarity helpers.
- `examples/vstr_benchmark_smoke.rs`: fast benchmark smoke coverage for CI.

## Project Layout 🧭

```text
src/
  lib.rs       crate entry point and public facade exports
  vbytes.rs    byte-slice helpers for possibly invalid UTF-8
  vencoding.rs BOM and UTF-8 validation/decoding helpers
  vstr/
    mod.rs     vstr facade and public re-exports
    basic/     common string helpers split by behavior area
    case.rs    case conversion helpers
    classify.rs character classification helpers
    emoji.rs   emoji detection and removal helpers
    encoding.rs HTML and Unicode escaping helpers
    matcher/   reusable literal matcher facade and optional backend adapter
    path.rs    Ant-style path matching helpers
    similarity.rs text similarity and fingerprint helpers
    text/      text normalization, truncation, wrapping, and inspection helpers
      wrap/    scalar wrapping options, tokenization, and render helpers
    width/     optional display-width measurement and wrapping helpers
      wrap/    display-width wrapping tokenization and render helpers
```

## Development Checks ✅

```bash
bash bin/check-release-ready.sh
```

The release-ready script is split into fast vet gates, publish readiness gates,
and release evidence smoke gates. It runs formatting,
default/no-default/all-features tests, examples, default clippy and
all-features clippy, rustdoc, docs.rs readiness, package contents, API inventory, semver,
benchmark smoke, and fuzz smoke gates. The expanded command list lives in
`CONTRIBUTING.md` and `aiflow.yaml` under `release-detail`. The
`bin/check-release-gate-layers.sh` guard keeps `release-detail` aligned with
the `vet`, `publish-readiness`, and `release-evidence` profiles, and also
verifies that CI runs every command in all three layers so local and CI gates
cannot drift apart.

Benchmark smoke and benchmark suite have different jobs. The smoke command is a
fast CI coverage check that proves expensive `vstr` paths still execute and emit
expected labels. The formal `cargo bench` target is the stable benchmark entry
point for local performance comparison and future historical reports.

To generate release-grade benchmark artifacts locally, pass an output directory:

```bash
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

That writes `vstr-bench.txt`, `vstr-bench.json`, and `vstr-bench.md`. JSON and
Markdown reports include a stable metadata contract:
`schema/version plus rustc, target, feature set, and commit metadata`. The same
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

Optional long-running fuzz sessions use `cargo-fuzz` from the separate
`fuzz/` crate. The local wrapper skips cleanly when `cargo-fuzz` is missing:

```bash
cargo fuzz run fuzz_substring
cargo fuzz run fuzz_escaping
cargo fuzz run fuzz_matcher
VSTR_FUZZ_RUN_SECS=60 bash bin/check-vstr-fuzz.sh
```

Public API checks are intentionally split. `check-public-api-inventory.sh`
ensures the all-features signature snapshot and optional feature delta are in
sync, while `check-api-semver.sh` classifies removed or changed signatures as
breaking and new signatures as additive inventory work. This keeps the default
zero-runtime-dependency API boundary visible beside the all-features surface.
`check-release-api-semver.sh` keeps that fast local check and, when a release
baseline is configured, runs `cargo-semver-checks check-release`.

The API stability model is tracked in `docs/public-api-inventory.md`: the
`core stable facade` is available in the default build, the
`optional feature facade` appears only behind Cargo features, and
`experimental-but-public APIs` such as `VStrMatcher` keep tested public
semantics while pre-1.0 internals can still evolve. The behavior evidence matrix
in `docs/api-behavior-contract.md` defines the unit, golden, fuzz, benchmark,
and documentation coverage expected before public signatures are refreshed.

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

The package contents check verifies the publish archive includes source,
examples, benchmark entry points, and governance docs while excluding local
`.aiflow/`, `target/`, fuzz runtime output, and temporary state.

## Compatibility 🔒

- MSRV: Rust 1.85.
- Edition: Rust 2024.
- Safety: unsafe code is forbidden by Cargo lints and checked by the project
  contract script.
- Dependencies: zero runtime dependencies in the default feature set; optional
features add focused crates such as `regex`, `unicode-segmentation`, and
  `unicode-width`; `matcher-aho-corasick` adds `aho-corasick` only for matcher
  internals.
