# Dependency Policy

`knifer-rs` keeps a zero-runtime-dependency core. The default feature set must
stay small, auditable, and friendly to ordinary business services that want a
utility crate without pulling in a dependency graph.

## Rules

- Do not add non-optional runtime dependencies to the default feature set.
- Prefer the Rust standard library when behavior is clear and maintainable.
- Use optional dependencies only when they materially improve correctness,
  performance, or standards compliance.
- Any optional dependency must document its MSRV impact, feature flag,
  maintenance risk, and public API boundary before admission.
- Optional capabilities must be tested with both
  `cargo test --locked --no-default-features` and
  `cargo test --locked --all-features`.

Current reserved optional feature names:

- `pattern-regex`
- `unicode-segmentation`
- `unicode-width`

## `vstr` Optional Feature Boundary

`vstr` is a UTF-8 string facade. It should remain useful without optional
dependencies, while making space for standards-backed behavior where the
standard library is intentionally lower level.

`pattern-regex` Admission Contract:

- The default build must not depend on `regex`.
- Regex-backed helpers should expose business-friendly APIs such as
  `contains_pattern`, `find_pattern`, `find_all_patterns`, and
  `replace_pattern`.
- Invalid pattern handling must use a crate-local error type such as
  `PatternError`; APIs that compile a user pattern should return `Result`.
- Complexity notes must state that regex-backed matching follows the selected
  engine's documented guarantees.

`unicode-segmentation` Admission Contract:

- Grapheme, word, or sentence segmentation must be feature-gated.
- Scalar-based helpers such as `take_chars` and `truncate_with_suffix` must keep
  their current semantics and documentation.
- Segmentation helpers should use names such as `graphemes`, `grapheme_len`,
  `take_graphemes`, `truncate_graphemes`, `unicode_words`,
  `unicode_word_indices`, `split_word_bounds`, `unicode_sentences`, and
  `split_sentence_bounds` so callers can see the boundary model at the call
  site.

`unicode-width` Admission Contract:

- Terminal/display-width helpers must be separate from scalar-count helpers.
- The default build must not depend on `unicode-width`.
- Display-width helpers are exposed as `display_width`, `take_width`,
  `truncate_width`, `wrap_width`, and `wrap_width_with_indent` only when the
  `unicode-width` feature is enabled.
- Width-based wrapping and truncation must document CJK, emoji, indentation,
  long-word splitting, and combining mark behavior.

`vencoding` Admission Contract:

- `vstr` must not become a byte-string or encoding-conversion facade.
- If an `encoding` feature is introduced, fallback decoding APIs, BOM handling,
  and lossy behavior belong under a future `vencoding` facade.
