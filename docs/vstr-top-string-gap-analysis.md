# Vstr Top String Library Gap Analysis

`vstr` is a business string-toolbox facade, not a replacement for specialized
string engines. The goal is to match top projects on explicit semantics,
testing, and release discipline while keeping the default crate lightweight.

## Reference Projects

- `rust-lang/regex`: pattern semantics and complexity guarantees.
- `BurntSushi/memchr`: search performance as a public contract.
- `BurntSushi/aho-corasick`: reusable multi-pattern search and replacement.
- `BurntSushi/bstr`: clear byte-string semantics for possibly invalid UTF-8.
- `unicode-rs/unicode-segmentation`: UAX #29 grapheme, word, and sentence
  boundaries.

## Completed Baseline

- `VSTR-GAP-002: Complete` - optional feature names are reserved in
  `Cargo.toml` and described in `docs/dependency-policy.md`.
- `VSTR-GAP-003: Complete` - Unicode boundary golden tests and optional
  grapheme, word, sentence, and display-width helpers cover combining marks,
  emoji ZWJ sequences, flags, CJK, and mixed-width text.
- `VSTR-GAP-004: Complete` - wrap and truncation golden tests cover long words,
  consecutive whitespace, CJK, emoji, indentation, and suffix or marker budget
  behavior. Wrap and truncation boundaries are tracked as golden contract cases.
- `VSTR-GAP-005: Complete` - case conversion fixtures cover acronym, number,
  separator, and Unicode examples.
- `VSTR-GAP-006: Complete` - Release Benchmark Workflow can generate benchmark
  artifacts.
- `VSTR-GAP-007: Complete` - public API inventory uses a generated signature
  snapshot.
- `VSTR-GAP-008: Complete` - `vbytes` is implemented as a separate byte-slice
  facade instead of mixing byte strings into `vstr`.
- `VSTR-GAP-009: Complete` - regex-backed optional pattern helpers are exposed
  behind the `pattern-regex` feature.
- `VSTR-GAP-010: Complete` - knifer-go golden fixtures are represented in unit
  tests and parity docs.
- `VSTR-GAP-011: Complete` - fuzz smoke targets exist for substring, escaping,
  path matching, replacement, matcher, and text-boundary helpers.
- `VSTR-GAP-012: Complete` - docs.rs readiness check exists.
- `VSTR-GAP-013: Complete` - repository hygiene uses top-project-style
  `.gitignore`, `.editorconfig`, and `.gitattributes`.
- `VSTR-GAP-014: Complete` - docs.rs/package readiness has a local gate.
- `VSTR-GAP-015: Complete` - `vencoding` is implemented as a separate
  encoding-boundary facade instead of mixing BOM and decoding policy into
  `vstr`.
- `VSTR-GAP-016: Complete` - reusable `VStrMatcher` supports leftmost-first,
  leftmost-longest, and `find_overlapping` semantics without making the simple
  facade heavy.
- `VSTR-GAP-001: Complete` - benchmark reports can be saved as plain text,
  JSON, and Markdown, then compared with a baseline JSON artifact or git ref
  using a regression threshold. The tracked formats are plain, JSON, and Markdown.
- `VSTR-TODO-004: Complete` - benchmark history operations define artifact
  locations, baseline selection, refresh rules, and regression threshold policy
  in `docs/vstr-benchmark-history.md`.

## Boundary Policy

`vstr` accepts valid UTF-8 strings. It should do not accept invalid UTF-8 in `vstr`;
byte strings that may be invalid UTF-8 belong in a future `vbytes` facade.

See `docs/vstr-complexity.md` for scalar-count behavior and
`docs/dependency-policy.md` for optional dependency admission.

## Roadmap

### Done

- Benchmark reports support plain, JSON, and Markdown output, baseline
  comparison, threshold checks, and documented history operations.
- Unicode boundary coverage includes scalar golden tests plus optional
  grapheme, word, sentence, and display-width helpers.
- `VSTR-TODO-002: Complete` - curated UAX #29 conformance fixtures now cover
  representative grapheme, word, and sentence boundaries without importing
  large upstream data files.
- Wrap and truncation behavior is locked with golden tests for long words,
  whitespace, CJK, emoji, indentation, and suffix budgets.
- Regex-backed helpers are available behind `pattern-regex`.
- `VSTR-TODO-003: Complete` - regex-backed golden tests cover invalid patterns,
  capture replacement, Unicode classes, empty matches, and multi-byte byte
  ranges.
- Reusable literal multi-pattern matching is available through `VStrMatcher`
  with leftmost-first, leftmost-longest, overlap, property, and fuzz smoke
  coverage.
- `VSTR-TODO-001: Complete` - fuzz harness strategy, deterministic smoke
  targets, and checked-in corpus seeds are tracked in `fuzz/PLAN.md`.
- `VSTR-TODO-005: Complete` - optional `aho-corasick` backend design is tracked
  in `docs/vstr-matcher-backend-plan.md` without changing the default
  dependency surface.
- `VSTR-TODO-006: Complete` - `knifer_rs::vbytes` MVP covers byte length,
  UTF-8 validation, byte slicing, ASCII trimming, search, prefix/suffix, and
  replacement without changing `vstr` semantics.
- `VSTR-TODO-007: Complete` - `knifer_rs::vencoding` MVP covers BOM sniffing,
  BOM stripping, UTF-8 validation, and lossy UTF-8 decoding without adding a
  default dependency or making `vstr` ambiguous.
- `VSTR-TODO-008: Complete` - text layout strategy now exposes `WrapOptions`,
  `WhitespaceMode`, `LongWordPolicy`, scalar `wrap_with_options`, and optional
  display-width `wrap_width_with_options` for separators, whitespace
  preservation, and long-word policy.
- `VSTR-TODO-009: Complete` - release-grade API compatibility now has
  `bin/check-release-api-semver.sh` and manual CI wiring for
  `cargo-semver-checks` against a configured baseline ref, source tree, or
  rustdoc JSON.

### Next

There are no remaining `Next` items from the current P1 roadmap. Continue with
the `Later` list when the current engineering gates stay green.

### Later

10. `VSTR-TODO-010` - keep README first-screen examples and feature flag docs
    aligned with the growing facade.
11. `VSTR-TODO-011` - extend case conversion compatibility fixtures for acronym,
    number, separator, and non-ASCII examples.

## `vbytes` Facade

`vbytes` handles byte strings that may be invalid UTF-8. All byte ranges are
byte offsets. It aligns with `bstr` concepts without making `vstr` ambiguous.

## `vencoding` Facade

`vencoding` covers BOM handling, UTF-8 validation, and lossy UTF-8 decoding.
Fallback transcoding APIs and possible optional integration with `encoding_rs`
remain future work under this facade. Java-style Unicode escape helpers can
remain in `vstr` because they operate on valid Rust strings.

## Release Benchmark Workflow

The default CI keeps using the fast benchmark smoke check. The manual release
benchmark workflow should generate `vstr-bench.md`, `vstr-bench.json`, and
plain text output as release artifacts. If `VSTR_BENCH_BASELINE_JSON` or
`VSTR_BENCH_BASE_REF` is set, the same entry point should also generate
`vstr-bench-compare.md` and `vstr-bench-compare.json`, failing when
`VSTR_BENCH_MAX_REGRESSION_PCT` is exceeded.
