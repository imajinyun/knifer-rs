# API Behavior Contract

This file defines the behavior evidence required for public `knifer-rs` APIs.
The signature inventory explains what exists; this contract explains how public
behavior stays stable enough to use.

Every new public API must be assigned to one stability class before the
signature snapshot is refreshed.

## Stability Classes

### Core Stable Facade

Core stable APIs are available in the default zero-runtime-dependency build.
Signature removals or signature changes are treated as breaking. Behavior
changes must be recorded in `CHANGELOG.md` and covered by tests before the
signature inventory is updated.

Required evidence:

- unit tests for normal, empty, and boundary inputs
- Unicode boundary tests for string slicing, truncation, wrapping, or escaping
- doctest or example coverage when the API is expected to appear in user-facing
  documentation
- fuzz smoke coverage for substring, escaping, replacement, path matching, or
  matcher behavior when the helper belongs to one of those risk areas
- benchmark smoke or benchmark suite coverage when the helper is part of the
  performance-sensitive `vstr` path

Current core stable areas:

- `kniferrs::vstr`: scalar-based string helpers, case conversion, replacement,
  literal search, escaping, Ant path matching, emoji helpers, similarity, and
  scalar text layout helpers
- `kniferrs::vbytes`: byte length, UTF-8 validation, byte slicing, ASCII trim,
  byte search, prefix/suffix stripping, and byte replacement
- `kniferrs::vencoding`: BOM detection, BOM stripping, UTF-8 validation, and
  lossy UTF-8 decoding boundaries

### Optional Feature Facade

Optional feature APIs are absent from the default build and appear only when the
matching Cargo feature is enabled. Their signatures are checked in the
all-features inventory and the optional feature delta.

Required evidence:

- default-build tests proving the API is not required by the zero-dependency
  surface
- all-features tests proving the API works when enabled
- docs or dependency-policy text explaining the feature boundary and dependency
  admission reason
- parity tests when an optional backend must preserve default semantics

Current optional feature areas:

- `pattern-regex`
- `unicode-normalization`
- `unicode-segmentation`
- `unicode-width`
- `transliterate`
- `matcher-aho-corasick`
- `search-memchr`

### Experimental-But-Public Facade

Experimental-but-public APIs are usable public APIs whose pre-1.0 semantics may
still be tuned. They are not private implementation details: signatures stay in
the all-features inventory, and behavior changes must be explicit.

Required evidence:

- unit tests for the current semantic rules
- parity tests when multiple implementations or backends exist
- fuzz smoke coverage for range, overlap, replacement, or boundary behavior
- documentation that describes which semantics are strong and which internals
  may change

Current experimental-but-public areas:

- `VStrMatcher`
- `VStrMatch`
- `MatchKind`

## Evidence Matrix

| API area | Stability class | Required evidence |
| --- | --- | --- |
| `vstr` scalar helpers | Core Stable Facade | unit tests, Unicode boundary tests, fuzz smoke for risky paths |
| `vstr` case conversion | Core Stable Facade | golden matrix tests and changelog notes for behavior changes |
| `vstr` wrap/truncate/text layout | Core Stable Facade | boundary tests, Unicode tests, benchmark smoke when performance-sensitive |
| `vstr` escaping | Core Stable Facade | unit tests, Unicode escape fixtures, fuzz smoke |
| `vstr` Ant path matching | Core Stable Facade | unit tests, knifer-go parity fixtures, fuzz smoke |
| `vbytes` | Core Stable Facade | unit tests for valid UTF-8, invalid UTF-8, empty input, and byte boundaries |
| `vencoding` | Core Stable Facade | unit tests for BOM variants, invalid UTF-8, and lossy decoding |
| regex-backed pattern helpers | Optional Feature Facade | all-features tests, default feature absence, dependency policy |
| Unicode segmentation helpers | Optional Feature Facade | all-features tests, Unicode boundary fixtures, dependency policy |
| Unicode normalization helpers | Optional Feature Facade | all-features tests, UAX #15 NFC/NFD/NFKC/NFKD golden fixtures, dependency policy |
| Unicode width helpers | Optional Feature Facade | all-features tests, mixed-width fixtures, dependency policy |
| transliteration helpers | Optional Feature Facade | all-features tests, default feature absence, CJK/Cyrillic/accent fixtures, dependency policy |
| matcher backend | Experimental-But-Public Facade | parity tests, overlap tests, replacement tests, fuzz smoke |
| vbytes search backend | Optional Feature Facade | naive-oracle parity tests in default and `search-memchr` builds, dependency policy |

## Change Rules

When adding or changing a public API:

1. Add or update behavior tests before refreshing
   `docs/public-api-inventory.md`.
2. Classify the API as core stable, optional feature, or
   experimental-but-public.
3. Update `CHANGELOG.md` when behavior, feature boundaries, or release
   contracts change.
4. Run `bash bin/check-public-api-inventory.sh`.
5. Run `bash bin/check-api-semver.sh`.
6. Run `bash bin/check-project-contract.sh`.

Do not use docs-only notes as the only evidence for new behavior. At least one
machine-checkable test, example, fuzz smoke, benchmark smoke, or script gate
must cover the behavior.
