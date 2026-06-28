# `vstr` Benchmark History Operations

This document defines how benchmark artifacts are generated, compared, and
refreshed. The default CI keeps benchmark work cheap; release-grade history is a
manual or release workflow concern.

## Artifact Location

Local benchmark reports should be written under `target/vstr-bench-report/`:

```bash
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

The script writes:

- `vstr-bench.txt`
- `vstr-bench.json`
- `vstr-bench.md`

When a baseline is supplied, it also writes:

- `vstr-bench-compare.json`
- `vstr-bench-compare.md`

GitHub Actions uploads the same directory from the manual
`release-benchmark` workflow as `vstr-benchmark-report-${{ github.sha }}`.

## Baseline Selection

Use `VSTR_BENCH_BASE_REF` for commit-to-commit comparisons:

```bash
VSTR_BENCH_BASE_REF=main \
VSTR_BENCH_MAX_REGRESSION_PCT=20.00 \
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

Use `VSTR_BENCH_BASELINE_JSON` when comparing against a saved artifact:

```bash
VSTR_BENCH_BASELINE_JSON=target/vstr-bench-baseline/vstr-bench.json \
VSTR_BENCH_MAX_REGRESSION_PCT=20.00 \
bash bin/check-vstr-bench.sh target/vstr-bench-report
```

Set only one of `VSTR_BENCH_BASE_REF` and `VSTR_BENCH_BASELINE_JSON`.

## Refresh Policy

Refresh the baseline when one of these changes is intentional:

- a benchmark input corpus changes;
- a benchmarked API is added, removed, or renamed;
- the benchmark harness changes measurement units, iteration counts, or output
  schema;
- an accepted optimization or regression changes expected runtime behavior.

Do not refresh the baseline only to hide an unexplained slowdown. First inspect
`vstr-bench-compare.md`, identify the affected benchmark names, and record the
reason in the release note, pull request, or changelog entry.

## Threshold Policy

`VSTR_BENCH_MAX_REGRESSION_PCT` defaults to `20.00`. A comparison fails when any
benchmark exceeds the configured regression threshold. Tighten the threshold
only after benchmark noise is understood on the target runner. Loosen it only
for a documented release reason, and prefer limiting the exception to the
manual benchmark workflow rather than default CI.

## CI Policy

Default CI runs `bash bin/check-vstr-benchmark-smoke.sh` only as a coverage
smoke check. It should not run release-grade benchmark comparison because runner
noise makes daily CI too unstable.

The manual `release-benchmark` workflow is the source of benchmark artifacts for
release review. Use it before publishing releases or after large `vstr`
performance changes.
