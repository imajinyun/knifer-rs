# `vstr` Matcher Backend Plan

`VStrMatcher` is the stable public facade for reusable literal multi-pattern
matching. The current implementation intentionally uses Safe Rust literal search
so public semantics can mature before an optional automaton backend is added.

## Feature Boundary

The future backend feature name is `matcher-aho-corasick`.

Admission rules:

- the default feature set must not depend on `aho-corasick`;
- `VStrMatcher::new`, `VStrMatcher::with_kind`, `find`, `find_all`,
  `find_overlapping`, and `replace_all` must keep their current signatures;
- public match records must keep `VStrMatch { needle, pattern_index, start,
  end }` with byte offsets into valid UTF-8 input;
- empty needles remain ignored;
- missing replacement indexes continue to preserve the original matched needle.

## Semantics Mapping

The facade owns the semantics. The backend is an implementation detail.

| `vstr` contract | Required behavior |
| --- | --- |
| `MatchKind::LeftmostFirst` | earliest start wins; registration order breaks same-start ties |
| `MatchKind::LeftmostLongest` | earliest start wins; longest same-start match wins; registration order breaks equal-length ties |
| `find_all` | non-overlapping left-to-right matches using the selected `MatchKind` |
| `find_overlapping` | overlap-aware matches at every UTF-8 scalar boundary |
| `replace_all` | non-overlapping replacement by original registration index |

If the selected `aho-corasick` match kind cannot express one of these semantics
exactly, the adapter must add a Safe Rust tie-break layer instead of changing
the public contract.

## Test Requirements

Before enabling `matcher-aho-corasick`, add parity tests that run the same input
matrix with and without the feature:

- empty needles and duplicate needles;
- leftmost-first versus leftmost-longest ties;
- overlapping examples such as `aaaa` with `a`, `aa`, and `aaa`;
- multi-byte needles and byte offsets such as `你好你好`;
- replacement arrays shorter than the original needle list.

The feature must pass:

```bash
cargo test --locked --no-default-features
cargo test --locked --all-features
bash bin/check-project-contract.sh
```

## Release Policy

Adding `matcher-aho-corasick` is a minor-version feature addition as long as the
default build and existing `VStrMatcher` signatures do not change. Any semantic
change to tie-breaks, overlapping matches, byte offsets, or replacement fallback
is a breaking change and must not be hidden behind the optional backend.
