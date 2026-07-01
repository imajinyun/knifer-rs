#!/usr/bin/env bash
set -euo pipefail

cargo run --locked --example vstr_daily
cargo run --locked --example vbytes_encoding
cargo run --locked --example vstr_matcher
cargo run --locked --all-features --example vstr_unicode
cargo test --locked --examples
cargo test --locked --all-features --examples

# Each example asserts inside main(), which `cargo test --examples` only
# compiles rather than runs. Every example on disk must therefore be executed
# via `cargo run` above, or be explicitly delegated to another gate that runs
# it. Guard that coverage so a new example cannot compile while silently never
# running its assertions.
delegated_examples=(
  # Run by bin/check-vstr-benchmark-smoke.sh.
  vstr_benchmark_smoke
)

run_examples="$(grep -oE -- '--example [a-z0-9_]+' "$0" | awk '{print $2}' | sort -u)"
delegated="$(printf '%s\n' "${delegated_examples[@]}" | sort -u)"
covered="$(printf '%s\n%s\n' "$run_examples" "$delegated" | sort -u)"

while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  name="$(basename "$path" .rs)"
  if ! grep -Fxq "$name" <<<"$covered"; then
    echo "example not executed by any gate: $name" >&2
    echo "add a 'cargo run --example $name' line above or list it in delegated_examples" >&2
    exit 1
  fi
done < <(find examples -maxdepth 1 -name '*.rs' | sort)

echo "examples check passed"
