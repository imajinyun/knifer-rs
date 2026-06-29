#!/usr/bin/env bash
set -euo pipefail

extract_aiflow_profile() {
  local profile="$1"

  awk -v profile="  ${profile}:" '
    $0 == profile { inside = 1; next }
    inside && /^  [A-Za-z0-9_-]+:/ { inside = 0 }
    inside && /^    - / {
      sub(/^    - /, "")
      print
    }
  ' aiflow.yaml
}

extract_release_ready_commands() {
  awk '
    /^echo / { next }
    /^$/ { next }
    /^#!/ { next }
    /^set / { next }
    /^cargo / || /^bash / || /^RUSTDOCFLAGS=/ { print }
  ' bin/check-release-ready.sh
}

write_expected_release_detail() {
  extract_aiflow_profile vet
  extract_aiflow_profile publish-readiness
  extract_aiflow_profile release-evidence
}

assert_same() {
  local label="$1"
  local expected="$2"
  local actual="$3"

  if ! diff -u "$expected" "$actual"; then
    echo "release gate layer mismatch: $label" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/knifer-rs-release-gates.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT

write_expected_release_detail >"$tmp_dir/expected-release-detail"
extract_aiflow_profile release-detail >"$tmp_dir/aiflow-release-detail"
extract_release_ready_commands >"$tmp_dir/release-ready"

assert_same "aiflow release-detail must equal vet + publish-readiness + release-evidence" \
  "$tmp_dir/expected-release-detail" \
  "$tmp_dir/aiflow-release-detail"

assert_same "bin/check-release-ready.sh must equal aiflow release-detail" \
  "$tmp_dir/aiflow-release-detail" \
  "$tmp_dir/release-ready"

echo "release gate layer check passed"
