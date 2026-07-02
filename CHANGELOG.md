# Changelog

All notable changes to `knifer-rs` are documented here.

The project follows semantic versioning once public releases begin. Before
`1.0`, APIs may still evolve, but changes should be recorded here and reflected
in the relevant parity documents.

## Unreleased

### Added

- Added the `vrand` facade for random string, token, and value generation. The
  default zero-dependency tier exposes the seedable, reproducible,
  **non-cryptographic** `VRand` generator (`SplitMix64`) with `next_u64`,
  `next_u32`, `below`, `range`, `bool`, `string`/`string_from`, `choose`, and
  `shuffle`, plus thread-local free helpers `random_string`,
  `random_string_from`, `random_digits`, `random_hex`, and alphabet constants
  (`DIGITS`, `LOWERCASE`, `UPPERCASE`, `ALPHANUMERIC`, `HEX`, `URL_SAFE`). The
  optional `random-secure` feature adds fail-closed, cryptographically secure
  helpers (`secure_bytes`, `secure_string`, `secure_string_from`, `secure_hex`,
  and the re-exported `SecureError`) backed by the operating system CSPRNG
  through `getrandom`. Both tiers use unbiased rejection sampling.
- Added the `vstr` facade for Safe Rust string and text utilities.
- Added a dependency-free string-similarity metric suite to `vstr`:
  `optimal_string_alignment` (restricted Damerau-Levenshtein),
  `damerau_levenshtein_distance` (unrestricted), `jaro_similarity`,
  `jaro_winkler_similarity`, and `sorensen_dice`, complementing the existing
  Levenshtein, Jaccard, n-gram, and `SimHash` helpers. Values are cross-checked
  against the `strsim` crate reference and are Unicode scalar-value aware.
- Added a dependency-free English inflection family to `vstr` in a dedicated
  `inflection` module: `singularize` (inverse of `pluralize`), `deordinalize`
  (inverse of `ordinalize`), `humanize` (identifier to sentence, dropping a
  trailing `_id`), `titleize` (identifier to Title Case), and `camelize`
  (identifier to `PascalCase` class name), with `pluralize`/`singularize` and
  `ordinalize`/`deordinalize` round-trip fixtures. The existing `pluralize` and
  `ordinalize` helpers moved into this module (re-exported, so paths are
  unchanged).
- Added Apache Commons `StringUtils`-style classics to `vstr`: `common_prefix`,
  `common_suffix`, `difference`, `rotate`, `wrap_if_missing`, and the ignore-case
  `add_prefix_if_not_ignore_case` / `add_suffix_if_not_ignore_case` helpers, with
  cross-crate golden fixtures.
- Added a dependency-free position/index search family to `vstr`: `index_of`,
  `index_of_ignore_case`, `last_index_of`, `ordinal_index_of` (one-based Nth,
  non-overlapping), `index_of_any` (earliest of a needle set), and
  `index_of_difference` (first diverging byte, Apache Commons parity). Results
  are byte indexes that always land on Unicode scalar boundaries.
- Added a dependency-free scalar-index manipulation family to `vstr` in a
  dedicated `manipulate` module: `insert` (lenient, clamps to append), `overlay`
  and `remove_range` (Apache Commons `overlay` parity, clamped and range-
  normalized), `replace_range` (strict, returns `None` on an invalid range), and
  `chunk` (borrowed pieces of at most N scalars). All indexes count Unicode
  scalar values, so results never split a multi-byte character.
- Deepened `vstr` HTML support (no new dependency): `unescape_html` now decodes
  decimal (`&#NNN;`) and hexadecimal (`&#xNN;`/`&#XNN;`) numeric references plus
  a curated named-entity table (`nbsp`, `copy`, `reg`, `trade`, typographic
  dashes/quotes, symbols, fractions, and currency signs) in a single
  left-to-right pass, preserving unknown names, bare ampersands, and invalid
  scalar values verbatim. Added `strip_tags`, which removes `<...>` markup while
  honoring quoted attribute values and `<!-- ... -->` comments and keeping an
  unterminated `<` as literal text.
- Added a dependency-free fzf-style fuzzy matcher to `vstr` in a dedicated
  `fuzzy` module: `fuzzy_match` (subsequence yes/no), `fuzzy_score` (ranking
  score, `None` on no match), and `fuzzy_indices` (score plus matched byte
  offsets for highlighting). Scoring follows fzf's `FuzzyMatchV1` model with
  word-boundary, `camelCase`, and consecutive-run bonuses and gap penalties,
  using fzf "smart case" matching. Returned indices are ascending byte offsets
  on Unicode scalar boundaries.
- Added a dependency-free runtime templating helper to `vstr`: `render_template`
  expands `{name}` placeholders from key/value pairs (any
  `IntoIterator<Item = (&str, &str)>`), where `{{`/`}}` emit literal braces, an
  unknown `{name}` is kept verbatim (missing-key policy: preserve), and an
  unterminated `{` is emitted literally. Values are inserted as-is, so callers
  escape untrusted values (for example with `escape_html`) before rendering into
  markup.
- Added dependency-free ASCII folding to `vstr`: `deburr` and `remove_accents`
  map common Latin diacritics and ligatures to ASCII (for example `é` to `e` and
  `Æ` to `Ae`) while preserving non-Latin scripts. `slugify` now folds through
  `deburr` so accented input produces ASCII slugs.
- Added a dependency-free humanize family to `vstr`: `pluralize`, `ordinalize`,
  `number_format`, `human_bytes` (binary IEC units), and `human_duration`, using
  integer-only arithmetic for deterministic output.
- Expanded `vstr` number humanization: `number_format_with` (configurable
  thousands separator), `number_format_float` (grouped integer part plus a fixed
  number of decimals, non-finite pass-through, no negative zero), and
  `human_count` (compact short-scale `K`/`M`/`B`/`T` with deterministic
  integer round-half-up and unit rollover).
- Added empty/blank predicates, trimming, splitting, substring, padding,
  defaulting, contains, prefix/suffix, length, formatting, case conversion,
  escaping, classification, Ant path matching, emoji, and similarity helpers.
- Added optional regex-backed `vstr` pattern helpers behind the
  `pattern-regex` feature.
- Added the reusable compiled `vstr::VRegex` facade behind the `pattern-regex`
  feature: compile a pattern once with `VRegex::new`, then reuse `is_match`,
  `find`, `find_all`, `captures`, `captures_named`, `replace_all`, `split`, and
  `splitn` across many inputs. `captures_named` returns named groups in
  declaration order, and `split`/`splitn` borrow slices from the input. The
  concrete regex engine type stays private to the crate.
- Added optional Unicode segmentation helpers for grapheme, word, and sentence
  boundaries behind the `unicode-segmentation` feature.
- Added grapheme-correct variants of the default char-based helpers behind the
  `unicode-segmentation` feature: `reverse_graphemes`, `pad_left_graphemes`,
  `pad_right_graphemes`, `center_graphemes`, and `mask_graphemes`. They mirror
  `reverse`/`pad_left`/`pad_right`/`center`/`mask` but count and slice by
  Unicode grapheme clusters, so combining marks, flag sequences, and ZWJ emoji
  are never split. The default char-based helpers are unchanged.
- Added optional Unicode normalization helpers `nfc`, `nfd`, `nfkc`, `nfkd`, and
  the quick-check predicates `is_nfc`, `is_nfd`, `is_nfkc`, `is_nfkd` behind the
  `unicode-normalization` feature, following Unicode Standard Annex #15.
- Added optional display-width helpers for CJK, combining marks, and emoji ZWJ
  text behind the `unicode-width` feature.
- Added optional full ASCII transliteration behind the `transliterate` feature:
  `transliterate` maps non-Latin scripts such as CJK and Cyrillic to readable
  ASCII, while `slugify_ascii` and `slugify_ascii_with_separator` produce ASCII
  slugs from any script. The default `slugify` keeps its diacritic-folding-only
  behavior, so callers opt in to full transliteration explicitly.
- Added `WrapOptions`, `WhitespaceMode`, and `LongWordPolicy` for configurable
  scalar wrapping without changing the existing `wrap` and `wrap_with_indent`
  behavior.
- Added `WrapAlgorithm` for `wrap_with_options`: the default `FirstFit` keeps
  the greedy behavior, while `OptimalFit` chooses collapsed-whitespace line
  breaks that minimize the sum of squared trailing slack (last line and
  over-long words excluded), producing more balanced paragraphs.
- Added `VStrMatcher` for reusable literal multi-pattern matching with
  leftmost-first, leftmost-longest, overlap, and replacement semantics.
- Added the optional `matcher-aho-corasick` backend for `VStrMatcher` internals
  while preserving the default zero-runtime-dependency build.
- Added the `vbytes` facade for byte slices that may not be valid UTF-8,
  covering byte length, UTF-8 checks, byte slicing, ASCII trimming, literal
  search, prefix/suffix stripping, and replacement.
- Added `bstr`-style lax-UTF-8 traversal to `vbytes`: `chars` and `char_indices`
  decode with the Unicode maximal-subpart replacement rule, while `lines` and
  `fields` split byte data without requiring valid UTF-8.
- Added the optional `search-memchr` backend for the `vbytes` literal search
  path: `find` and `find_all` route through SIMD-accelerated `memchr::memmem`
  when enabled while preserving the default zero-runtime-dependency scan and
  identical leftmost, non-overlapping results (pinned by a naive-oracle parity
  test that runs in both feature builds).
- Added the `vencoding` facade for BOM detection, BOM stripping, UTF-8
  validation, and lossy UTF-8 decoding boundaries.
- Added optional WHATWG legacy encoding conversion to `vencoding` behind the
  `encoding` feature: `encoding_name` resolves labels, `decode` decodes lossily
  with BOM sniffing, `decode_strict` rejects malformed input, and `encode`
  writes legacy bytes (HTML numeric references for unmappable characters), with
  `EncodingError` / `EncodingErrorKind` keeping `encoding_rs` off the public API
  and parity plus round-trip fixtures for GBK, Shift_JIS, windows-1252, and
  ISO-8859-1. The default build stays zero-runtime-dependency.
- Documented the `words` / `word_count` whitespace-tokenization contract (CJK and
  punctuation stay attached, whitespace-free scripts stay as one token) versus the
  feature-gated UAX #29 `unicode_words` / `unicode_word_len`, pinned by golden
  tests over CJK, emoji, and mixed punctuation input.
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
