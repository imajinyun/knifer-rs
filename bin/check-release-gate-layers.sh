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

extract_ci_stable_run_commands() {
  awk '
    $0 == "  stable:" { inside = 1; next }
    inside && /^  [A-Za-z0-9_-]+:/ { inside = 0 }
    inside && /^ +run: / {
      sub(/^ +run: /, "")
      print
    }
  ' .github/workflows/ci.yml
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

# vet-layer commands that CI runs through a wrapper script instead of verbatim.
# Each exclusion must be explicit and explainable so the coverage stays auditable.
ci_wrapper_covered() {
  local command="$1"

  case "$command" in
    "cargo test --locked --examples")
      # bin/check-examples.sh runs this exact command in the CI stable job.
      echo "bash bin/check-examples.sh"
      return 0
      ;;
  esac

  return 1
}

assert_ci_runs_vet_layer() {
  local ci_commands="$1"
  local status=0
  local wrapper

  while IFS= read -r command; do
    [[ -z "$command" ]] && continue

    if wrapper="$(ci_wrapper_covered "$command")"; then
      if ! grep -Fxq "$wrapper" "$ci_commands"; then
        echo "CI stable job must run '$wrapper' to cover vet command: $command" >&2
        status=1
      fi
      continue
    fi

    if ! grep -Fxq "$command" "$ci_commands"; then
      echo "CI stable job is missing vet-layer command: $command" >&2
      status=1
    fi
  done < <(extract_aiflow_profile vet)

  return "$status"
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/knifer-rs-release-gates.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT

write_expected_release_detail >"$tmp_dir/expected-release-detail"
extract_aiflow_profile release-detail >"$tmp_dir/aiflow-release-detail"
extract_release_ready_commands >"$tmp_dir/release-ready"
extract_ci_stable_run_commands >"$tmp_dir/ci-stable"

assert_same "aiflow release-detail must equal vet + publish-readiness + release-evidence" \
  "$tmp_dir/expected-release-detail" \
  "$tmp_dir/aiflow-release-detail"

assert_same "bin/check-release-ready.sh must equal aiflow release-detail" \
  "$tmp_dir/aiflow-release-detail" \
  "$tmp_dir/release-ready"

assert_ci_runs_vet_layer "$tmp_dir/ci-stable"

echo "release gate layer check passed"
