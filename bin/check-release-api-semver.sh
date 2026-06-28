#!/usr/bin/env bash
set -euo pipefail

bash bin/check-api-semver.sh

baseline_ref="${API_SEMVER_BASELINE_REF:-}"
baseline_root="${API_SEMVER_BASELINE_ROOT:-}"
baseline_rustdoc="${API_SEMVER_BASELINE_RUSTDOC:-}"
baseline_crates_io="${API_SEMVER_BASELINE_CRATES_IO:-false}"
required="${API_SEMVER_REQUIRED:-false}"

baseline_count=0
[[ -n "$baseline_ref" ]] && baseline_count=$((baseline_count + 1))
[[ -n "$baseline_root" ]] && baseline_count=$((baseline_count + 1))
[[ -n "$baseline_rustdoc" ]] && baseline_count=$((baseline_count + 1))
[[ "$baseline_crates_io" == "true" ]] && baseline_count=$((baseline_count + 1))

if [[ "$baseline_count" -gt 1 ]]; then
  echo "set only one cargo-semver-checks baseline source" >&2
  echo "supported: API_SEMVER_BASELINE_REF, API_SEMVER_BASELINE_ROOT, API_SEMVER_BASELINE_RUSTDOC, API_SEMVER_BASELINE_CRATES_IO=true" >&2
  exit 1
fi

if [[ "$baseline_count" -eq 0 ]]; then
  if [[ "$required" == "true" ]]; then
    echo "release API semver check requires a baseline" >&2
    echo "set API_SEMVER_BASELINE_REF, API_SEMVER_BASELINE_ROOT, API_SEMVER_BASELINE_RUSTDOC, or API_SEMVER_BASELINE_CRATES_IO=true" >&2
    exit 1
  fi
  echo "cargo-semver-checks skipped: no release baseline configured"
  exit 0
fi

if ! cargo semver-checks --version >/dev/null 2>&1; then
  echo "cargo-semver-checks is required for release API semver checks" >&2
  echo "install it with: cargo install cargo-semver-checks --locked" >&2
  exit 1
fi

args=(semver-checks check-release)

if [[ -n "$baseline_ref" ]]; then
  args+=(--baseline-rev "$baseline_ref")
elif [[ -n "$baseline_root" ]]; then
  args+=(--baseline-root "$baseline_root")
elif [[ -n "$baseline_rustdoc" ]]; then
  args+=(--baseline-rustdoc "$baseline_rustdoc")
fi

cargo "${args[@]}"
