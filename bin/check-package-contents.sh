#!/usr/bin/env bash
set -euo pipefail

package_list="$(cargo package --locked --allow-dirty --list)"

require_entry() {
  local path="$1"

  if ! grep -Fxq -- "$path" <<<"$package_list"; then
    echo "missing package entry: $path" >&2
    exit 1
  fi
}

forbid_entry() {
  local path="$1"

  if grep -Fxq -- "$path" <<<"$package_list"; then
    echo "forbidden package entry: $path" >&2
    exit 1
  fi
}

forbid_prefix() {
  local prefix="$1"

  while IFS= read -r entry; do
    if [[ "$entry" == "$prefix" || "$entry" == "$prefix/"* ]]; then
      echo "forbidden package prefix: $prefix" >&2
      exit 1
    fi
  done <<<"$package_list"
}

require_entry Cargo.toml
require_entry Cargo.lock
require_entry LICENSE
require_entry README.md
require_entry CHANGELOG.md
require_entry SECURITY.md
require_entry CONTRIBUTING.md
require_entry aiflow.yaml
require_entry .github/workflows/ci.yml
require_entry src/lib.rs
require_entry src/vstr/mod.rs
require_entry src/vstr/basic/mod.rs
require_entry src/vstr/basic/affix.rs
require_entry src/vstr/basic/casefold.rs
require_entry src/vstr/basic/compare.rs
require_entry src/vstr/basic/escape.rs
require_entry src/vstr/basic/format.rs
require_entry src/vstr/basic/measure.rs
require_entry src/vstr/basic/predicate.rs
require_entry src/vstr/basic/replace.rs
require_entry src/vstr/basic/search.rs
require_entry src/vstr/basic/split.rs
require_entry src/vstr/basic/substring.rs
require_entry src/vstr/basic/trim.rs
require_entry src/vstr/basic/value.rs
require_entry src/vstr/text.rs
require_entry src/vstr/text/content.rs
require_entry src/vstr/text/inspect.rs
require_entry src/vstr/text/normalize.rs
require_entry src/vstr/text/transform.rs
require_entry src/vstr/text/wrap.rs
require_entry src/vstr/width.rs
require_entry src/vstr/width/measure.rs
require_entry src/vstr/width/wrap.rs
require_entry src/vbytes.rs
require_entry src/vencoding.rs
require_entry examples/vstr_daily.rs
require_entry examples/vstr_unicode.rs
require_entry examples/vbytes_encoding.rs
require_entry examples/vstr_matcher.rs
require_entry examples/vstr_benchmark_smoke.rs
require_entry bench/vstr_bench.rs
require_entry docs/public-api-inventory.md
require_entry docs/api-behavior-contract.md
require_entry docs/dependency-policy.md
require_entry docs/vstr-complexity.md
require_entry docs/vstr-benchmark-history.md
require_entry bin/check-docs-rs-ready.sh

forbid_prefix .aiflow
forbid_prefix target
forbid_prefix criterion
forbid_prefix tmp
forbid_prefix temp
forbid_prefix fuzz/target
forbid_prefix fuzz/artifacts
forbid_prefix fuzz/crashes
forbid_entry .env
forbid_entry .env.local
forbid_entry .cargo/config.toml

if grep -Eq '^fuzz/' <<<"$package_list"; then
  echo "fuzz/ is local release evidence and is intentionally excluded from the package" >&2
  exit 1
fi

echo "package contents check passed"
