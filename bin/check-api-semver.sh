#!/usr/bin/env bash
set -euo pipefail

inventory="docs/public-api-inventory.md"
signature_start="<!-- public-api-signatures:start -->"
signature_end="<!-- public-api-signatures:end -->"

if [[ ! -f "$inventory" ]]; then
  echo "missing required file: $inventory" >&2
  exit 1
fi

expected_signatures="$(
  awk -v start="$signature_start" -v end="$signature_end" '
    $0 == start { inside = 1; next }
    $0 == end { inside = 0 }
    inside && $0 != "" { print }
  ' "$inventory"
)"

if [[ -z "$expected_signatures" ]]; then
  echo "missing public API signature snapshot in $inventory" >&2
  exit 1
fi

actual_signatures="$(bash bin/check-public-api-inventory.sh --print-signatures)"

expected_paths="$(printf '%s\n' "$expected_signatures" | sed -E 's/ = .*//' | sort -u)"
actual_paths="$(printf '%s\n' "$actual_signatures" | sed -E 's/ = .*//' | sort -u)"

breaking_paths="$(comm -23 <(printf '%s\n' "$expected_paths") <(printf '%s\n' "$actual_paths"))"
additive_paths="$(comm -13 <(printf '%s\n' "$expected_paths") <(printf '%s\n' "$actual_paths"))"
changed_signatures="$(comm -12 <(printf '%s\n' "$expected_paths") <(printf '%s\n' "$actual_paths") |
  while IFS= read -r path; do
    expected_line="$(printf '%s\n' "$expected_signatures" | grep -F "$path = ")"
    actual_line="$(printf '%s\n' "$actual_signatures" | grep -F "$path = ")"
    if [[ "$expected_line" != "$actual_line" ]]; then
      printf '%s\n' "$path"
    fi
  done)"

if [[ -n "$breaking_paths" || -n "$changed_signatures" ]]; then
  echo "public API semver check failed: breaking API changes detected" >&2
  if [[ -n "$breaking_paths" ]]; then
    echo >&2
    echo "removed public APIs:" >&2
    printf '%s\n' "$breaking_paths" >&2
  fi
  if [[ -n "$changed_signatures" ]]; then
    echo >&2
    echo "changed public API signatures:" >&2
    while IFS= read -r path; do
      [[ -z "$path" ]] && continue
      expected_line="$(printf '%s\n' "$expected_signatures" | grep -F "$path = ")"
      actual_line="$(printf '%s\n' "$actual_signatures" | grep -F "$path = ")"
      printf -- '- %s\n' "$path" >&2
      printf '  expected: %s\n' "$expected_line" >&2
      printf '  actual:   %s\n' "$actual_line" >&2
    done <<<"$changed_signatures"
  fi
  exit 1
fi

if [[ -n "$additive_paths" ]]; then
  echo "public API semver check found additive APIs not in inventory" >&2
  printf '%s\n' "$additive_paths" >&2
  echo "update $inventory after reviewing the API addition" >&2
  exit 1
fi

echo "public API semver check passed"
