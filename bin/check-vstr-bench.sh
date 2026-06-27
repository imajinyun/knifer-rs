#!/usr/bin/env bash
set -euo pipefail

report_dir="${1:-}"
baseline_json="${VSTR_BENCH_BASELINE_JSON:-}"
baseline_ref="${VSTR_BENCH_BASE_REF:-}"
max_regression_pct="${VSTR_BENCH_MAX_REGRESSION_PCT:-20.00}"
baseline_tmp=""

cleanup() {
  if [[ -n "$baseline_tmp" ]]; then
    rm -rf "$baseline_tmp"
  fi
}

trap cleanup EXIT

if [[ -n "$baseline_json" && -n "$baseline_ref" ]]; then
  echo "set only one of VSTR_BENCH_BASELINE_JSON or VSTR_BENCH_BASE_REF" >&2
  exit 1
fi

if [[ -n "$baseline_ref" ]]; then
  baseline_tmp="$(mktemp -d)"
  mkdir -p "$baseline_tmp/repo"
  git archive "$baseline_ref" | tar -x -C "$baseline_tmp/repo"
  (
    cd "$baseline_tmp/repo"
    cargo bench --bench vstr_bench --quiet -- --json
  ) > "$baseline_tmp/vstr-bench-baseline.json"
  baseline_json="$baseline_tmp/vstr-bench-baseline.json"
fi

output="$(cargo bench --bench vstr_bench --quiet)"
json_output="$(cargo bench --bench vstr_bench --quiet -- --json)"
markdown_output="$(cargo bench --bench vstr_bench --quiet -- --markdown)"
compare_json_output=""
compare_markdown_output=""

for name in \
  contains \
  find_all \
  replace_many \
  to_snake_case \
  wrap \
  levenshtein_distance \
  ant_path_match
do
  if ! grep -Fq "bench=$name " <<<"$output"; then
    echo "missing benchmark output for: $name" >&2
    echo "$output" >&2
    exit 1
  fi
  if ! grep -Fq "\"bench\":\"$name\"" <<<"$json_output"; then
    echo "missing JSON benchmark output for: $name" >&2
    echo "$json_output" >&2
    exit 1
  fi
  if ! grep -Fq "| \`$name\` |" <<<"$markdown_output"; then
    echo "missing Markdown benchmark output for: $name" >&2
    echo "$markdown_output" >&2
    exit 1
  fi
done

if ! grep -Fq '"suite":"vstr_bench"' <<<"$json_output"; then
  echo "missing JSON benchmark suite metadata" >&2
  echo "$json_output" >&2
  exit 1
fi

if ! grep -Fq '# vstr_bench Report' <<<"$markdown_output"; then
  echo "missing Markdown benchmark report heading" >&2
  echo "$markdown_output" >&2
  exit 1
fi

if [[ -n "$baseline_json" ]]; then
  if [[ ! -f "$baseline_json" ]]; then
    echo "missing benchmark baseline JSON: $baseline_json" >&2
    exit 1
  fi

  compare_json_output="$(
    cargo bench --bench vstr_bench --quiet -- \
      --compare-json "$baseline_json" \
      --max-regression-pct "$max_regression_pct"
  )"
  compare_markdown_output="$(
    cargo bench --bench vstr_bench --quiet -- \
      --compare-markdown "$baseline_json" \
      --max-regression-pct "$max_regression_pct"
  )"

  if grep -Fq '"status":"fail"' <<<"$compare_json_output"; then
    echo "benchmark comparison failed threshold: $max_regression_pct%" >&2
    echo "$compare_markdown_output" >&2
    exit 1
  fi
fi

if [[ -n "$report_dir" ]]; then
  mkdir -p "$report_dir"
  printf '%s\n' "$output" > "$report_dir/vstr-bench.txt"
  printf '%s\n' "$json_output" > "$report_dir/vstr-bench.json"
  printf '%s\n' "$markdown_output" > "$report_dir/vstr-bench.md"
  if [[ -n "$baseline_json" ]]; then
    printf '%s\n' "$compare_json_output" > "$report_dir/vstr-bench-compare.json"
    printf '%s\n' "$compare_markdown_output" > "$report_dir/vstr-bench-compare.md"
  fi
fi

echo "$output"
