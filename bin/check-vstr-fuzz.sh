#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-fuzz >/dev/null 2>&1; then
  cat >&2 <<'MSG'
cargo-fuzz is not installed; skipping optional long-running fuzz checks.
Install it with:
  cargo install cargo-fuzz --locked
Then run:
  bash bin/check-vstr-fuzz.sh
MSG
  exit 0
fi

run_secs="${VSTR_FUZZ_RUN_SECS:-60}"
targets=(
  fuzz_substring
  fuzz_escaping
  fuzz_path_matching
  fuzz_replacement
  fuzz_matcher
  fuzz_text_boundaries
)

for target in "${targets[@]}"; do
  cargo fuzz run "$target" -- -max_total_time="$run_secs"
done
