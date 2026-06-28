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

When `VSTR_BENCH_BASELINE_JSON` points to a previous `vstr-bench.json`, the same
entry point writes:

- `vstr-bench-compare.json`
- `vstr-bench-compare.md`

`VSTR_BENCH_MAX_REGRESSION_PCT` sets the allowed regression threshold before the
script fails. The comparison uses benchmark names and elapsed nanoseconds from
the JSON report, so artifacts from different commits remain directly
comparable. `VSTR_BENCH_BASE_REF` can be used instead of
`VSTR_BENCH_BASELINE_JSON` to generate a baseline report from another git ref
before comparing it with the current checkout.

The manual release benchmark artifact should come from the same script so local
reports and GitHub Actions artifacts remain comparable.
The operational baseline selection, refresh rules, and threshold policy are
tracked in `docs/vstr-benchmark-history.md`.

## Unicode Boundary Policy

Current substring and truncation APIs are Unicode scalar based. They never cut a
UTF-8 byte sequence, but they can split user-perceived grapheme clusters such as
combining marks, flags, and emoji ZWJ family sequences.

The `unicode-segmentation` feature adds grapheme-aware helpers such as
`graphemes`, `grapheme_len`, `take_graphemes`, and `truncate_graphemes`, plus
UAX #29 word-boundary helpers such as `unicode_words`,
`unicode_word_indices`, `split_word_bounds`, and
`split_word_bound_indices`. `unicode_words` filters punctuation and
whitespace, while `split_word_bounds` keeps separators so concatenating the
segments reconstructs the original input. Sentence-boundary helpers follow the
same pattern: `unicode_sentences` filters separator-only spans, while
`split_sentence_bounds` and `split_sentence_bound_indices` preserve them.

The `unicode-width` feature adds terminal display-width helpers such as
`display_width`, `take_width`, `truncate_width`, `wrap_width`, and
`wrap_width_with_indent`. These helpers follow the `unicode-width` crate's
rules, including CJK full-width characters, combining marks, and emoji ZWJ
sequences. `take_width` measures each candidate prefix as a complete string so
its behavior stays aligned with `display_width`. `truncate_width` reserves
suffix budget in display cells. `wrap_width` and `wrap_width_with_indent` use
display-cell budgets while preserving the same whitespace-collapse and
long-word progress guarantees as scalar `wrap`. Optional helpers must not
silently change scalar-based helpers.

## Wrap and Truncation Boundaries

`wrap` and `wrap_with_indent` use `split_whitespace` and scalar-count budgets.
Consecutive whitespace inside a paragraph collapses to one ASCII space, while
blank newline-separated paragraphs are preserved as blank lines. Long words are split by scalar value
so wrapping always makes progress. CJK, emoji, combining
marks, and ZWJ sequences are counted as Unicode scalar values, not terminal
display cells.

`wrap_with_options` makes the scalar layout policy explicit through
`WrapOptions`, `WhitespaceMode`, and `LongWordPolicy`. It can preserve
whitespace runs, keep long words intact, and split at caller-provided word
separators such as `/` or `-` without changing the default `wrap` behavior. The
`unicode-width` feature adds `wrap_width_with_options`, which uses the same
policy object but measures line budgets in display cells.

`wrap_with_indent` counts indentation inside the requested width. If an indent
is equal to or wider than the width, content still progresses at one scalar per
line. `truncate_with_suffix` reserves suffix budget inside the requested scalar
count. If the suffix is longer than the budget, the suffix itself is truncated.
`abbreviate_middle` applies the same marker-budget rule and splits the remaining
budget between the front and back, favoring the front when the budget is odd.

## Future Multi-Pattern Matcher Contract

`VStrMatcher` is the current Safe Rust MVP for this contract. It documents
overlap and tie-break behavior explicitly. The default business-friendly policy
is leftmost-first. `MatchKind::LeftmostLongest` can be selected when the longest
match at the same byte offset should win. `find_overlapping` exposes
overlap-aware behavior without changing current non-overlapping helpers.

## Optional Regex Pattern Helpers

The `pattern-regex` feature enables regex-backed helpers such as
`contains_pattern`, `find_pattern`, `find_all_patterns`, and `replace_pattern`.
They compile the caller-provided pattern for each call and return `PatternError`
when the pattern is invalid. Matching complexity follows the Rust `regex`
crate's documented guarantees.
