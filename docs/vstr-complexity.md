# `vstr` Complexity and Allocation Notes

This document records behavior that should remain explicit as `vstr` grows. The
facade currently accepts valid UTF-8 `&str` values and uses Safe Rust only.

## Baseline Operations

- `trim`, `trim_start`, `trim_end`, `truncate`, `take_chars`,
  `drop_chars`, `sub`, and `trim_blank_lines` borrow from the original input
  when possible.
- Case conversion, normalization, wrapping, masking, escaping, and replacement
  helpers return owned `String` values.
- `find_all` and `count_matches` use non-overlapping literal search.
- `find_all_ignore_case` currently allocates lowercase search buffers.
- `replace_many` applies literal replacements in caller-provided order.
- `levenshtein_distance` is dynamic-programming based and intended for short
  business strings, labels, and identifiers.
- `ant_path_match` and `ant_path_match_with_separator` implement Ant-style
  segment matching for route and file-like paths.

## Benchmark Entry Points

Use `bash bin/check-vstr-benchmark-smoke.sh` as a coverage smoke check, not as a performance report.
It proves important paths execute in CI.

Use `bash bin/check-vstr-bench.sh` as the formal benchmark entry point. It wraps
the stable benchmark target:

```bash
cargo bench --bench vstr_bench --quiet -- --json
cargo bench --bench vstr_bench --quiet -- --markdown
```

When passed an output directory, the script writes a machine-readable historical report format:

- `vstr-bench.txt`
- `vstr-bench.json`
- `vstr-bench.md`

The manual release benchmark artifact should come from the same script so local
reports and GitHub Actions artifacts remain comparable.

## Unicode Boundary Policy

Current substring and truncation APIs are Unicode scalar based. They never cut a
UTF-8 byte sequence, but they can split user-perceived grapheme clusters such as
combining marks, flags, and emoji ZWJ family sequences.

Grapheme segmentation and terminal display width remain candidates for optional features.
They should not silently change scalar-based helpers.

## Wrap and Truncation Boundaries

`wrap` and `wrap_with_indent` use `split_whitespace` and scalar-count budgets.
Long words may occupy their own line. `truncate_with_suffix` reserves suffix
budget inside the requested scalar count.

## Future Multi-Pattern Matcher Contract

The future matcher should document overlap and tie-break behavior explicitly.
The default business-friendly policy should be leftmost-first. A dedicated
`find_overlapping` style API can expose overlap-aware behavior without changing
current non-overlapping helpers.
