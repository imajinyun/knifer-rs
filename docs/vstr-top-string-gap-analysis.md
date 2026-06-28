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
- `VSTR-GAP-008: Complete` - future `vbytes` boundary is documented instead of
  mixing byte strings into `vstr`.
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
- `VSTR-GAP-015: Complete` - future `vencoding` boundary is documented instead
  of mixing encoding conversion into `vstr`.
- `VSTR-GAP-016: Complete` - reusable `VStrMatcher` supports leftmost-first,
  leftmost-longest, and `find_overlapping` semantics without making the simple
  facade heavy.
- `VSTR-GAP-001: Complete` - benchmark reports can be saved as plain text,
  JSON, and Markdown, then compared with a baseline JSON artifact or git ref
  using a regression threshold. The tracked formats are plain, JSON, and Markdown.

## Boundary Policy

`vstr` accepts valid UTF-8 strings. It should do not accept invalid UTF-8 in `vstr`;
byte strings that may be invalid UTF-8 belong in a future `vbytes` facade.

See `docs/vstr-complexity.md` for scalar-count behavior and
`docs/dependency-policy.md` for optional dependency admission.

## Roadmap

### Done

- Benchmark reports support plain, JSON, and Markdown output, baseline
  comparison, and threshold checks.
- Unicode boundary coverage includes scalar golden tests plus optional
  grapheme, word, sentence, and display-width helpers.
- `VSTR-TODO-002: Complete` - curated UAX #29 conformance fixtures now cover
  representative grapheme, word, and sentence boundaries without importing
  large upstream data files.
- Wrap and truncation behavior is locked with golden tests for long words,
  whitespace, CJK, emoji, indentation, and suffix budgets.
- Regex-backed helpers are available behind `pattern-regex`.
- Reusable literal multi-pattern matching is available through `VStrMatcher`
  with leftmost-first, leftmost-longest, overlap, property, and fuzz smoke
  coverage.

### Next

1. `VSTR-TODO-001` - introduce a real fuzz harness strategy. Keep the current
   deterministic smoke targets, then decide whether to add `cargo-fuzz`,
   checked-in corpus seeds, or both. The initial harness strategy is tracked in
   `fuzz/PLAN.md`.
2. `VSTR-TODO-003` - deepen regex-backed golden tests. Cover invalid patterns,
   capture replacement, Unicode classes, empty matches, and multi-byte byte
   ranges.
3. `VSTR-TODO-004` - define benchmark history operations. Document where
   baseline artifacts live, when they are refreshed, and how regression
   thresholds are interpreted.

### Later

5. `VSTR-TODO-005` - design an optional `aho-corasick` backend for
   `VStrMatcher` without changing the default dependency surface.
6. `VSTR-TODO-006` - implement a `vbytes` MVP for byte strings that may be
   invalid UTF-8.
7. `VSTR-TODO-007` - implement a `vencoding` MVP for BOM sniffing, UTF-8
   validation, and lossy decoding.
8. `VSTR-TODO-008` - expand text layout strategy around word separators,
   preserve-whitespace mode, display width, and long-word policy.
9. `VSTR-TODO-009` - evolve local API signature checks toward release-grade
   `cargo-semver-checks` against published versions or release tags.
10. `VSTR-TODO-010` - keep README first-screen examples and feature flag docs
    aligned with the growing facade.
11. `VSTR-TODO-011` - extend case conversion compatibility fixtures for acronym,
    number, separator, and non-ASCII examples.

## Future `vbytes` Facade Plan

`vbytes` should handle byte strings that may be invalid UTF-8. All byte ranges
are byte offsets. It should align with `bstr` concepts without making `vstr`
ambiguous.

## Future `vencoding` Facade Plan

`vencoding` should cover BOM handling, UTF-8 validation, lossy decoding,
fallback decoding APIs, and possible optional integration with `encoding_rs`.
Java-style Unicode escape helpers can remain in `vstr` because they operate on
valid Rust strings.

## Release Benchmark Workflow

The default CI keeps using the fast benchmark smoke check. The manual release
benchmark workflow should generate `vstr-bench.md`, `vstr-bench.json`, and
plain text output as release artifacts. If `VSTR_BENCH_BASELINE_JSON` or
`VSTR_BENCH_BASE_REF` is set, the same entry point should also generate
`vstr-bench-compare.md` and `vstr-bench-compare.json`, failing when
`VSTR_BENCH_MAX_REGRESSION_PCT` is exceeded.
