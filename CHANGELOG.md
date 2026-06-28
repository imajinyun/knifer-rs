# Changelog

All notable changes to `knifer-rs` are documented here.

The project follows semantic versioning once public releases begin. Before
`1.0`, APIs may still evolve, but changes should be recorded here and reflected
in the relevant parity documents.

## Unreleased

### Added

- Added the `vstr` facade for Safe Rust string and text utilities.
- Added empty/blank predicates, trimming, splitting, substring, padding,
  defaulting, contains, prefix/suffix, length, formatting, case conversion,
  escaping, classification, Ant path matching, emoji, and similarity helpers.
- Added optional regex-backed `vstr` pattern helpers behind the
  `pattern-regex` feature.
- Added optional Unicode segmentation helpers for grapheme, word, and sentence
  boundaries behind the `unicode-segmentation` feature.
- Added optional display-width helpers for CJK, combining marks, and emoji ZWJ
  text behind the `unicode-width` feature.
- Added `WrapOptions`, `WhitespaceMode`, and `LongWordPolicy` for configurable
  scalar wrapping without changing the existing `wrap` and `wrap_with_indent`
  behavior.
- Added `VStrMatcher` for reusable literal multi-pattern matching with
  leftmost-first, leftmost-longest, overlap, and replacement semantics.
- Added the optional `matcher-aho-corasick` backend for `VStrMatcher` internals
  while preserving the default zero-runtime-dependency build.
- Added the `vbytes` facade for byte slices that may not be valid UTF-8,
  covering byte length, UTF-8 checks, byte slicing, ASCII trimming, literal
  search, prefix/suffix stripping, and replacement.
- Added the `vencoding` facade for BOM detection, BOM stripping, UTF-8
  validation, and lossy UTF-8 decoding boundaries.
- Added executable golden coverage for knifer-go parity, Unicode boundaries,
  wrap/truncation behavior, regex-backed patterns, matcher behavior, and
  case conversion acronym/number/separator/non-ASCII matrix cases.
- Added baseline GitHub Actions CI for formatting, tests, clippy, and docs.
- Added CI coverage for no-default-features, all-features, MSRV, docs.rs
  readiness, public API inventory, local API semver checks, benchmark smoke,
  and fuzz smoke.
- Added versioned benchmark JSON/Markdown artifacts with schema and environment
  metadata for release comparison.
- Added manual release workflows for benchmark artifact generation and
  release-grade API semver checks with `cargo-semver-checks`.
- Added optional local `cargo-fuzz` support and a bounded fuzz wrapper while
  keeping default CI on deterministic fuzz smoke.
- Added public API stability classes for core stable, optional feature, and
  experimental-but-public facade surfaces.
- Added project governance files: `LICENSE`, `SECURITY.md`, and
  `CONTRIBUTING.md`.

### Release Readiness

- The default feature set remains zero-runtime-dependency and Safe Rust only.
- `CHANGELOG.md`, `docs/public-api-inventory.md`, and
  `docs/vstr-api-parity.md` describe the current 0.1.0 capability and
  compatibility boundary.
- Before publishing a 0.1.x release, run the release checklist in
  `CONTRIBUTING.md` and review `cargo package --locked --allow-dirty` output
  for warnings that need source changes.
