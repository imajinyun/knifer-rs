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
- `random-secure`
- `unicode-normalization`
- `unicode-segmentation`
- `unicode-width`
- `transliterate`
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
  `captures`, `captures_named`, `replace_all`, `split`, and `splitn`, but the
  concrete engine type must stay private and out of every public signature.
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

`transliterate` Admission Contract:

- The default build must not depend on `deunicode`.
- Full ASCII transliteration must be feature-gated and expose `transliterate`
  for readable ASCII conversion of non-Latin scripts, plus `slugify_ascii` and
  `slugify_ascii_with_separator` for ASCII slugs from any script.
- The default `slugify` must keep its diacritic-folding-only behavior and must
  not route through `deunicode`; full transliteration is an explicit opt-in.
- `deunicode` is admitted because it provides a maintained, zero-transitive-dep
  transliteration table with an MSRV within the crate policy.

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
- Legacy encoding conversion is gated behind the `encoding` feature and lives in
  the `vencoding` facade, never in `vstr`.
- The default build must not depend on `encoding_rs`; BOM handling and UTF-8
  validation stay pure standard library.
- The `encoding` feature enables WHATWG-labeled helpers `encoding_name`,
  `decode`, `decode_strict`, and `encode` over `encoding_rs`. These
  fallback decoding APIs cover legacy code pages: `decode` is lossy
  (`U+FFFD` replacement with BOM sniffing), `decode_strict` rejects malformed
  input, and `encode` emits HTML numeric references for unmappable characters.
- Invalid labels and malformed strict input must surface through the crate-local
  `EncodingError` / `EncodingErrorKind` types, never the underlying
  `encoding_rs` error shapes.
- `encoding_rs` is admitted because it is the canonical WHATWG Encoding Standard
  implementation, is Safe-Rust-compatible for our usage, and has an MSRV far
  below the crate policy with only `cfg-if` as a transitive dependency.
- Parity and round-trip fixtures must cover GBK, Shift_JIS, windows-1252, and
  ISO-8859-1 in both the default and `encoding` builds.

## `vrand` Optional Feature Boundary

`vrand` is a random-value facade. Its default surface must stay zero-runtime-
dependency and clearly separated from cryptographic randomness.

`random-secure` Admission Contract:

- The default build must not depend on `getrandom` or any other entropy crate.
  The default `VRand` generator is a `SplitMix64` PRNG seeded from standard
  library state only, and it is explicitly documented as **non-cryptographic**.
- Cryptographically secure helpers (`secure_bytes`, `secure_string`,
  `secure_string_from`, `secure_hex`) are gated behind the `random-secure`
  feature and live in the `vrand::secure` submodule, never in the default facade.
- Secure helpers must **fail closed**: entropy failures surface through the
  re-exported `SecureError` type as `Result::Err`, never a silent fallback to the
  non-crypto PRNG.
- `getrandom` is admitted because it is the de-facto standard OS entropy shim,
  exposes a Safe-Rust API (`getrandom::fill`), has an MSRV at or below the crate
  policy (1.85), and pulls only platform entropy backends (`libc`, `r-efi`,
  `wasip2`) plus `rand_core` as transitive dependencies.
- Alphabet sampling in both tiers must use unbiased rejection sampling so no
  character is over-represented.
