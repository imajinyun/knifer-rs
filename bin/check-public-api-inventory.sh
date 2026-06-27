#!/usr/bin/env bash
set -euo pipefail

inventory="docs/public-api-inventory.md"

if [[ ! -f "$inventory" ]]; then
  echo "missing required file: $inventory" >&2
  exit 1
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

echo "public API inventory is in sync"
