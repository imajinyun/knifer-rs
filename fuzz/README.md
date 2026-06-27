# vstr Fuzz Smoke Targets

This directory contains local fuzz-smoke targets for high-risk `vstr` helpers.
It is a separate crate so fuzz and smoke dependencies never enter the default
`knifer-rs` runtime dependency surface.

Run all targets with:

```bash
bash bin/check-vstr-fuzz-smoke.sh
```

Current targets:

- `fuzz_substring`: scalar-boundary slicing helpers.
- `fuzz_escaping`: regex and Unicode escaping helpers.
- `fuzz_path_matching`: Ant-style path matching helpers.
- `fuzz_replacement`: deterministic literal replacement helpers.
- `fuzz_text_boundaries`: wrap, truncation, abbreviation, masking, centering,
  and whitespace boundary helpers.

These are deterministic smoke targets today. If libFuzzer coverage is added
later, keep the same target boundaries and reuse the invariant checks here.
