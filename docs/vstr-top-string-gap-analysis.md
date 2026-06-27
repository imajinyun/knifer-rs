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
- `VSTR-GAP-011: Complete` - fuzz smoke targets exist for substring, escaping,
  path matching, and replacement.
- `VSTR-GAP-012: Complete` - docs.rs readiness check exists.
- `VSTR-GAP-005: Complete` - case conversion fixtures cover acronym, number,
  separator, and Unicode examples.
- `VSTR-GAP-007: Complete` - public API inventory uses a generated signature
  snapshot.
- `VSTR-GAP-008: Complete` - future `vbytes` boundary is documented instead of
  mixing byte strings into `vstr`.
- `VSTR-GAP-013: Complete` - repository hygiene uses top-project-style
  `.gitignore`, `.editorconfig`, and `.gitattributes`.
- `VSTR-GAP-014: Complete` - docs.rs/package readiness has a local gate.
- `VSTR-GAP-015: Complete` - future `vencoding` boundary is documented instead
  of mixing encoding conversion into `vstr`.
- `VSTR-GAP-006: Complete` - Release Benchmark Workflow can generate benchmark
  artifacts.
- `VSTR-GAP-010: Complete` - knifer-go golden fixtures are represented in unit
  tests and parity docs.
- `VSTR-GAP-001: Complete` - benchmark reports can be saved as plain text,
  JSON, and Markdown, then compared with a baseline JSON artifact or git ref
  using a regression threshold. The tracked formats are plain, JSON, and Markdown.

## Boundary Policy

`vstr` accepts valid UTF-8 strings. It should do not accept invalid UTF-8 in `vstr`;
byte strings that may be invalid UTF-8 belong in a future `vbytes` facade.

See `docs/vstr-complexity.md` for scalar-count behavior and
`docs/dependency-policy.md` for optional dependency admission.

## Open Todo

1. `VSTR-GAP-001: Complete` - keep benchmark reports in plain, JSON, and
   Markdown formats, with baseline comparison and threshold checks.
2. `VSTR-GAP-003: Complete` - Unicode boundary golden tests and optional
   grapheme helpers cover combining marks, emoji ZWJ sequences, flags, CJK, and
   mixed-width text.
3. `VSTR-GAP-004`: keep expanding Wrap and truncation boundaries for long words,
   indentation, suffix budget, and display-width candidates.
4. `VSTR-GAP-009: Complete` - regex-backed optional pattern helpers are exposed
   behind the `pattern-regex` feature.
5. `VSTR-GAP-016: Complete` - reusable `VStrMatcher` supports leftmost-first,
   leftmost-longest, and `find_overlapping` semantics without making the simple
   facade heavy.

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
