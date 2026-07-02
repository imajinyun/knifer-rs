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
- `unicode-normalization`
- `unicode-segmentation`
- `unicode-width`
- `matcher-aho-corasick`
- `search-memchr`

## `vstr` Optional Feature Boundary

`vstr` is a UTF-8 string facade. It should remain useful without optional
dependencies, while making space for standards-backed behavior where the
standard library is intentionally lower level.

`pattern-regex` Admission Contract:

- The default build must not depend on `regex`.
- Regex-backed helpers should expose business-friendly APIs such as
  `contains_pattern`, `find_pattern`, `find_all_patterns`, and
  `replace_pattern`.
- A reusable compiled facade `VRegex` may expose `is_match`, `find`, `find_all`,
  `captures`, and `replace_all`, but the concrete engine type must stay private
  and out of every public signature.
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

`unicode-normalization` Admission Contract:

- The default build must not depend on `unicode-normalization`.
- Normalization helpers must be feature-gated and expose the four Unicode
  normalization forms as `nfc`, `nfd`, `nfkc`, and `nfkd`, plus quick-check
  predicates `is_nfc`, `is_nfd`, `is_nfkc`, and `is_nfkd`.
- Scalar helpers such as `chars`, `take_chars`, and `truncate_with_suffix` must
  keep their current semantics; normalization must be an explicit opt-in step,
  never applied implicitly by default helpers.
- Behavior must follow Unicode Standard Annex #15 (UAX #15), and golden tests
  must cover canonical (NFC/NFD) and compatibility (NFKC/NFKD) cases.

`unicode-width` Admission Contract:

- Terminal/display-width helpers must be separate from scalar-count helpers.
- The default build must not depend on `unicode-width`.
- Display-width helpers are exposed as `display_width`, `take_width`,
  `truncate_width`, `wrap_width`, `wrap_width_with_indent`, and
  `wrap_width_with_options` only when the `unicode-width` feature is enabled.
- Width-based wrapping and truncation must document CJK, emoji, indentation,
  long-word splitting, and combining mark behavior.
  `wrap_width_with_options` must reuse the scalar `WrapOptions` policy type
  while measuring budgets in display cells.

`matcher-aho-corasick` Admission Contract:

- The default build must not depend on `aho-corasick`.
- `matcher-aho-corasick` may enable the optional `aho-corasick` dependency for
  reusable matcher internals only.
- `VStrMatcher` signatures, `VStrMatch` byte offsets, empty-needle handling, and
  replacement fallback semantics must not change.
- The optional backend must preserve `MatchKind::LeftmostFirst`,
  `MatchKind::LeftmostLongest`, `find_all`, `find_overlapping`, and
  `replace_all` behavior.
- If the backend crate cannot express a tie-break rule exactly, a Safe Rust
  adapter layer must preserve the `vstr` contract.
- The detailed backend plan lives in `docs/vstr-matcher-backend-plan.md`.

`search-memchr` Admission Contract:

- The default build must not depend on `memchr`.
- `search-memchr` may enable the optional `memchr` dependency for the `vbytes`
  literal byte search backend only.
- `vbytes::find`, `vbytes::find_all`, and `vbytes::contains` signatures,
  empty-needle handling, and leftmost non-overlapping semantics must not change.
- The backend selects a single internal literal searcher (`raw_find`); no public
  `vbytes` API is added, removed, or altered when the feature is toggled.
- A naive-oracle parity test must confirm identical results in both the default
  and `search-memchr` builds.

`vencoding` Admission Contract:

- `vstr` must not become a byte-string or encoding-conversion facade.
- If an `encoding` feature is introduced, fallback decoding APIs, BOM handling,
  and lossy behavior belong under a future `vencoding` facade.
