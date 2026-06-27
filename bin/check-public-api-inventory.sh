#!/usr/bin/env bash
set -euo pipefail

inventory="docs/public-api-inventory.md"
signature_start="<!-- public-api-signatures:start -->"
signature_end="<!-- public-api-signatures:end -->"

if [[ ! -f "$inventory" ]]; then
  echo "missing required file: $inventory" >&2
  exit 1
fi

extract_public_signatures() {
  awk '
    function trim(value) {
      sub(/^[ \t\r\n]+/, "", value)
      sub(/[ \t\r\n]+$/, "", value)
      return value
    }

    function canonicalize(value) {
      gsub(/\r/, "", value)
      sub(/[ \t]*\{[^\n]*$/, "", value)
      sub(/[ \t]*;[^\n]*$/, "", value)
      gsub(/\n/, " ", value)
      value = trim(value)
      gsub(/[ \t]+/, " ", value)
      gsub(/[ \t]*,[ \t]*/, ", ", value)
      gsub(/[ \t]*\([ \t]*/, "(", value)
      gsub(/[ \t]*\)[ \t]*/, ")", value)
      gsub(/[ \t]*->[ \t]*/, " -> ", value)
      gsub(/[ \t]*\+[ \t]*/, " + ", value)
      gsub(/[ \t]*=[ \t]*/, " = ", value)
      gsub(/= \(/, "= (", value)
      gsub(/,\)/, ")", value)
      gsub(/, \)/, ")", value)
      gsub(/, $/, "", value)
      return value
    }

    function public_name(signature, value) {
      value = signature
      sub(/^[ \t]*pub[ \t]+/, "", value)
      sub(/^const[ \t]+/, "", value)
      sub(/^fn[ \t]+/, "", value)
      sub(/^struct[ \t]+/, "", value)
      sub(/^enum[ \t]+/, "", value)
      sub(/^trait[ \t]+/, "", value)
      sub(/^type[ \t]+/, "", value)
      sub(/^const[ \t]+/, "", value)
      sub(/^mod[ \t]+/, "", value)
      sub(/[<(;:{= ].*$/, "", value)
      return value
    }

    function emit_signature(signature, name, path) {
      signature = canonicalize(signature)
      if (signature == "") {
        return
      }

      name = public_name(signature)
      if (name == "") {
        return
      }

      if (FILENAME == "src/lib.rs") {
        path = "knifer_rs::" name
      } else if (impl_context != "") {
        path = "knifer_rs::vstr::" impl_context "::" name
      } else {
        path = "knifer_rs::vstr::" name
      }

      print path " = " signature
    }

    function open_braces(value, tmp) {
      tmp = value
      return gsub(/\{/, "", tmp)
    }

    function close_braces(value, tmp) {
      tmp = value
      return gsub(/\}/, "", tmp)
    }

    function begin_impl(line, value) {
      value = trim(line)
      sub(/^impl[^ \t{]*[ \t]+/, "", value)
      sub(/[ \t]*\{.*/, "", value)
      value = trim(value)
      if (value ~ / for /) {
        impl_context = ""
      } else {
        sub(/<.*/, "", value)
        impl_context = value
      }
      in_impl = 1
      impl_depth = 0
    }

    FILENAME ~ /\/tests\.rs$/ {
      next
    }

    {
      line = $0

      if (collecting) {
        signature = signature "\n" line
        if (line ~ /\{/ || line ~ /;/) {
          emit_signature(signature)
          collecting = 0
        }
      } else if (line ~ /^[ \t]*impl[^;]*\{[ \t]*$/) {
        begin_impl(line)
      } else if (line ~ /^[ \t]*pub[ \t]+(const[ \t]+fn|fn|struct|enum|trait|type|const|mod)[ \t]+/) {
        signature = line
        if (line ~ /\{/ || line ~ /;/) {
          emit_signature(signature)
        } else {
          collecting = 1
        }
      }

      if (in_impl) {
        impl_depth += open_braces(line)
        impl_depth -= close_braces(line)
        if (impl_depth <= 0) {
          in_impl = 0
          impl_context = ""
        }
      }
    }
  ' "$@" | sort -u
}

if [[ "${1:-}" == "--print-signatures" ]]; then
  extract_public_signatures src/lib.rs src/vstr/*.rs
  exit 0
fi

while IFS= read -r api; do
  if ! grep -Fq "\`$api\`" "$inventory" && ! grep -Fq "::$api\`" "$inventory"; then
    echo "public API missing from $inventory: $api" >&2
    exit 1
  fi
done < <(
  grep -R --include='*.rs' -hE '^[[:space:]]*pub (const fn|fn|struct|enum|trait|type|const) [A-Za-z_][A-Za-z0-9_]*|^[[:space:]]*pub mod [A-Za-z_][A-Za-z0-9_]*' src |
    sed -E 's/^[[:space:]]*pub (const fn|fn|struct|enum|trait|type|const|mod) ([A-Za-z_][A-Za-z0-9_]*).*/\2/' |
    sort -u
)

expected_signatures="$(
  awk -v start="$signature_start" -v end="$signature_end" '
    $0 == start { inside = 1; next }
    $0 == end { inside = 0 }
    inside && $0 != "" { print }
  ' "$inventory"
)"

if [[ -z "$expected_signatures" ]]; then
  echo "missing public API signature snapshot in $inventory" >&2
  echo "add lines between $signature_start and $signature_end" >&2
  echo >&2
  extract_public_signatures src/lib.rs src/vstr/*.rs >&2
  exit 1
fi

actual_signatures="$(extract_public_signatures src/lib.rs src/vstr/*.rs)"

if [[ "$actual_signatures" != "$expected_signatures" ]]; then
  echo "public API signature snapshot is out of sync" >&2
  echo "review the diff for semver impact, then update $inventory if intentional" >&2
  diff -u \
    <(printf '%s\n' "$expected_signatures") \
    <(printf '%s\n' "$actual_signatures") >&2 || true
  exit 1
fi

echo "public API inventory is in sync"
