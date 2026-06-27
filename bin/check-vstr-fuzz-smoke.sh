#!/usr/bin/env bash
set -euo pipefail

manifest="fuzz/Cargo.toml"

for target in \
  fuzz_substring \
  fuzz_escaping \
  fuzz_path_matching \
  fuzz_replacement
do
  cargo run --locked --manifest-path "$manifest" --bin "$target" --quiet
  echo "$target: ok"
done
