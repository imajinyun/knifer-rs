#!/usr/bin/env bash
set -euo pipefail

output="$(cargo run --example vstr_benchmark_smoke --quiet)"

for name in \
  contains \
  find_all \
  replace_many \
  to_snake_case \
  wrap \
  levenshtein_distance \
  ant_path_match
do
  if ! grep -Fq "$name:" <<<"$output"; then
    echo "missing benchmark smoke output for: $name" >&2
    echo "$output" >&2
    exit 1
  fi
done

echo "$output"
