#!/usr/bin/env bash
set -euo pipefail

output="$(cargo bench --bench vstr_bench --quiet)"

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
done

echo "$output"
