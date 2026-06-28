# vstr Fuzz Smoke Targets

This directory contains local fuzz-smoke targets for high-risk `vstr` helpers.
It is a separate crate so fuzz and smoke dependencies never enter the default
`knifer-rs` runtime dependency surface.

Run all targets with:

```bash
bash bin/check-vstr-fuzz-smoke.sh
```

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

These are deterministic smoke targets today. If libFuzzer coverage is added
later, keep the same target boundaries and reuse the invariant checks here.
See `fuzz/PLAN.md` for the layered fuzz harness strategy, corpus seed policy,
and promotion rules for fuzz-found regressions.
