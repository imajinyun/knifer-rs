#!/usr/bin/env bash
set -euo pipefail

output="$(cargo bench --bench vstr_bench --quiet)"
json_output="$(cargo bench --bench vstr_bench --quiet -- --json)"
markdown_output="$(cargo bench --bench vstr_bench --quiet -- --markdown)"

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

echo "$output"
