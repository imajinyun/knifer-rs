# vstr Fuzz Targets

This directory contains local fuzz targets for high-risk `vstr` helpers. It is
a separate crate so fuzz dependencies never enter the default `knifer-rs`
runtime dependency surface.

Default CI runs deterministic smoke only:

```bash
bash bin/check-vstr-fuzz-smoke.sh
```

For optional long-running local fuzzing, install `cargo-fuzz` and run:

```bash
cargo install cargo-fuzz --locked
cargo fuzz run fuzz_substring
cargo fuzz run fuzz_escaping
cargo fuzz run fuzz_matcher
```

The project wrapper runs every target for a bounded local session and skips
cleanly when `cargo-fuzz` is not installed:

```bash
VSTR_FUZZ_RUN_SECS=60 bash bin/check-vstr-fuzz.sh
```

Use `VSTR_FUZZ_RUN_SECS=60` for a quick local pass before risky string-boundary
changes. Use `VSTR_FUZZ_RUN_SECS=600` or longer before release candidates or
after changes to substring, escaping, path matching, replacement, matcher,
wrap, truncation, or width behavior.

Checked-in seed files live in `fuzz/corpus/`. Each smoke target keeps a small
inline corpus and also reads its matching seed file with `include_str!`, so
reviewed edge cases become part of the fast smoke loop.

Current targets:

- `fuzz_substring`: scalar-boundary slicing helpers.
- `fuzz_escaping`: regex and Unicode escaping helpers.
- `fuzz_path_matching`: Ant-style path matching helpers.
- `fuzz_replacement`: deterministic literal replacement helpers.
- `fuzz_matcher`: reusable multi-pattern matcher ranges, overlap, and replace
  invariants.
- `fuzz_text_boundaries`: wrap, truncation, abbreviation, masking, centering,
  and whitespace boundary helpers.

Do not commit generated `fuzz/artifacts/`, `fuzz/crashes/`, `fuzz/target/`, or
engine-expanded corpus files. Only commit reviewed `.txt` seeds under
`fuzz/corpus/`, and reduce every crash into a deterministic smoke seed, unit
test, or golden fixture before landing the fix.

Crash handling protocol:

1. Reproduce the crash with the specific `cargo fuzz run <target> <artifact>`
   command printed by `cargo-fuzz`.
2. Minimize the input if the artifact is large or includes unrelated bytes.
3. Add the reduced case to the matching `fuzz/corpus/*.txt` seed file only when
   it is human-readable and stable.
4. Add a unit test or golden fixture for semantic regressions, not just the
   fuzz seed.
5. Keep generated crash files local; they are release evidence, not repository
   source.

See `fuzz/PLAN.md` for the layered fuzz harness strategy, corpus seed policy,
and promotion rules for fuzz-found regressions.
