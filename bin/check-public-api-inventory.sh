#!/usr/bin/env bash
set -euo pipefail

inventory="docs/public-api-inventory.md"
all_features_signature_start="<!-- public-api-signatures:start -->"
all_features_signature_end="<!-- public-api-signatures:end -->"
optional_signature_start="<!-- public-api-optional-signatures:start -->"
optional_signature_end="<!-- public-api-optional-signatures:end -->"

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

    function module_path(filename, value) {
      if (filename == "src/lib.rs") {
        return ""
      }
      if (filename ~ /src\/vstr\//) {
        return "vstr"
      }

      value = filename
      sub(/^src\//, "", value)
      sub(/\.rs$/, "", value)
      return value
    }

    function emit_signature(signature, name, path, module) {
      signature = canonicalize(signature)
      if (signature == "") {
        return
      }

      name = public_name(signature)
      if (name == "") {
        return
      }

      module = module_path(FILENAME)
      if (module == "") {
        path = "kniferrs::" name
      } else if (impl_context != "") {
        path = "kniferrs::" module "::" impl_context "::" name
      } else {
        path = "kniferrs::" module "::" name
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

default_api_files=(
  src/lib.rs
  src/vbytes.rs
  src/vencoding.rs
  src/vstr/basic/affix.rs
  src/vstr/basic/compare.rs
  src/vstr/basic/escape.rs
  src/vstr/basic/format.rs
  src/vstr/basic/measure.rs
  src/vstr/basic/mod.rs
  src/vstr/basic/predicate.rs
  src/vstr/basic/replace.rs
  src/vstr/basic/search.rs
  src/vstr/basic/split.rs
  src/vstr/basic/substring.rs
  src/vstr/basic/trim.rs
  src/vstr/basic/value.rs
  src/vstr/case.rs
  src/vstr/classify.rs
  src/vstr/emoji.rs
  src/vstr/encoding.rs
  src/vstr/humanize.rs
  src/vstr/inflection.rs
  src/vstr/matcher.rs
  src/vstr/matcher/search.rs
  src/vstr/matcher/types.rs
  src/vstr/mod.rs
  src/vstr/path.rs
  src/vstr/similarity.rs
  src/vstr/text.rs
  src/vstr/text/content.rs
  src/vstr/text/fold.rs
  src/vstr/text/inspect.rs
  src/vstr/text/normalize.rs
  src/vstr/text/transform.rs
  src/vstr/text/wrap.rs
  src/vstr/text/wrap/basic.rs
  src/vstr/text/wrap/options.rs
  src/vstr/text/wrap/options_wrap.rs
)

all_features_api_files=(
  "${default_api_files[@]}"
  src/vstr/grapheme.rs
  src/vstr/normalize.rs
  src/vstr/pattern.rs
  src/vstr/transliterate.rs
  src/vstr/width.rs
  src/vstr/width/measure.rs
  src/vstr/width/wrap.rs
  src/vstr/width/wrap/basic.rs
  src/vstr/width/wrap/options_wrap.rs
)

extract_snapshot() {
  local start="$1"
  local end="$2"

  awk -v start="$start" -v end="$end" '
    $0 == start { inside = 1; next }
    $0 == end { inside = 0 }
    inside && $0 != "" { print }
  ' "$inventory"
}

if [[ "${1:-}" == "--print-signatures" || "${1:-}" == "--print-all-features-signatures" ]]; then
  extract_public_signatures "${all_features_api_files[@]}"
  exit 0
fi

if [[ "${1:-}" == "--print-default-signatures" ]]; then
  extract_public_signatures "${default_api_files[@]}"
  exit 0
fi

# The signature arrays above are curated by hand. Guard the invariant that they
# cover every source file which declares a public item, so a new module cannot
# add public API that silently escapes the inventory snapshot. Test modules are
# excluded because they never expose crate API.
covered_files="$(printf '%s\n' "${all_features_api_files[@]}" | sort -u)"
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if ! grep -Fxq "$file" <<<"$covered_files"; then
    echo "public API source file not covered by $inventory arrays: $file" >&2
    echo "add it to default_api_files or all_features_api_files in $0" >&2
    exit 1
  fi
done < <(
  grep -rlE '^[[:space:]]*pub (const fn|fn|struct|enum|trait|type|const|mod) ' src --include='*.rs' |
    grep -vE '(^|/)tests(/|\.rs$)' |
    sort -u
)

while IFS= read -r api; do
  if ! grep -Fq "\`$api\`" "$inventory" &&
    ! grep -Fq "::$api\`" "$inventory" &&
    ! grep -Eq "(^|::)$api( = |::)" "$inventory"; then
    echo "public API missing from $inventory: $api" >&2
    exit 1
  fi
done < <(
  grep -hE '^[[:space:]]*pub (const fn|fn|struct|enum|trait|type|const) [A-Za-z_][A-Za-z0-9_]*|^[[:space:]]*pub mod [A-Za-z_][A-Za-z0-9_]*' "${all_features_api_files[@]}" |
    sed -E 's/^[[:space:]]*pub (const fn|fn|struct|enum|trait|type|const|mod) ([A-Za-z_][A-Za-z0-9_]*).*/\2/' |
    sort -u
)

expected_all_features_signatures="$(extract_snapshot "$all_features_signature_start" "$all_features_signature_end")"
expected_optional_signatures="$(extract_snapshot "$optional_signature_start" "$optional_signature_end")"

actual_default_signatures="$(extract_public_signatures "${default_api_files[@]}")"
actual_all_features_signatures="$(extract_public_signatures "${all_features_api_files[@]}")"
actual_optional_signatures="$(
  comm -13 \
    <(printf '%s\n' "$actual_default_signatures") \
    <(printf '%s\n' "$actual_all_features_signatures")
)"

if [[ -z "$expected_all_features_signatures" ]]; then
  echo "missing all-features public API signature snapshot in $inventory" >&2
  echo "add lines between $all_features_signature_start and $all_features_signature_end" >&2
  echo >&2
  printf '%s\n' "$actual_all_features_signatures" >&2
  exit 1
fi

if [[ -z "$expected_optional_signatures" ]]; then
  echo "missing optional public API signature snapshot in $inventory" >&2
  echo "add lines between $optional_signature_start and $optional_signature_end" >&2
  if [[ -n "$actual_optional_signatures" ]]; then
    echo >&2
    printf '%s\n' "$actual_optional_signatures" >&2
  fi
  exit 1
fi

if [[ "$actual_all_features_signatures" != "$expected_all_features_signatures" ]]; then
  echo "all-features public API signature snapshot is out of sync" >&2
  echo "review the diff for semver impact, then update $inventory if intentional" >&2
  diff -u \
    <(printf '%s\n' "$expected_all_features_signatures") \
    <(printf '%s\n' "$actual_all_features_signatures") >&2 || true
  exit 1
fi

if [[ "$actual_optional_signatures" != "$expected_optional_signatures" ]]; then
  echo "optional feature public API signature snapshot is out of sync" >&2
  echo "review feature-gated API changes, then update $inventory if intentional" >&2
  diff -u \
    <(printf '%s\n' "$expected_optional_signatures") \
    <(printf '%s\n' "$actual_optional_signatures") >&2 || true
  exit 1
fi

echo "public API inventory is in sync for all-features and optional feature delta"
