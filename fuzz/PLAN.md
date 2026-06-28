# vstr Fuzz Harness Plan

`vstr` fuzz coverage has three layers. Keep them separate so default CI remains
fast while release and local deep checks can grow independently.

## Layer 1: Deterministic Smoke

The current `fuzz_targets/*.rs` binaries are deterministic smoke harnesses. They
run fixed high-risk corpora and assert invariants such as:

- returned ranges are valid UTF-8 byte boundaries;
- replacement and wrapping outputs remain valid strings;
- non-overlapping matcher results never overlap;
- overlapping matcher results remain sorted by start offset;
- helpers do not panic on CJK, combining marks, emoji, empty strings, or long
  words.

These targets must stay cheap enough for `bash bin/check-vstr-fuzz-smoke.sh` and
the default CI workflow.

## Layer 2: Checked-In Corpus Seeds

Future corpus seeds should be small text files grouped by behavior:

- `substring`: scalar indexes, negative indexes, empty input, CJK, emoji;
- `escaping`: regex metacharacters, malformed Unicode escapes, surrogate pairs;
- `path_matching`: `*`, `?`, `**`, custom separators, empty separators;
- `replacement`: overlapping needles, empty needles, repeated replacements;
- `matcher`: leftmost-first, leftmost-longest, overlap, replacement indexes;
- `text_boundaries`: wrap, truncate, mask, width, indentation.

Seed files should be reviewed like tests. Do not commit generated crash output,
large minimized corpora, or local engine state.

## Layer 3: Optional Engine Fuzzing

If `cargo-fuzz` or another engine is added later:

- keep the current target names and invariant checks;
- keep engine dependencies out of the root crate and default feature set;
- run engine fuzzing only in manual, nightly, or release workflows;
- keep default CI on deterministic smoke unless runtime stays consistently low;
- document any new generated directories in `.gitignore` before enabling the
  workflow.

## Promotion Rules

A bug found by engine fuzzing should be reduced into one of:

- a deterministic smoke corpus case;
- a unit or property-style test in `src/vstr/tests.rs`;
- a golden fixture when it defines public behavior.

The reduced case should be committed before or with the fix so future regressions
do not require the fuzz engine to reproduce.
