#!/usr/bin/env bash
set -euo pipefail

require_file() {
  local path="$1"

  if [[ ! -f "$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local text="$2"

  if ! grep -Fq "$text" "$path"; then
    echo "missing required text in $path: $text" >&2
    exit 1
  fi
}

require_file Cargo.toml
require_file Cargo.lock
require_file .editorconfig
require_file .gitattributes
require_file .gitignore
require_file LICENSE
require_file SECURITY.md
require_file CONTRIBUTING.md
require_file CHANGELOG.md
require_file README.md
require_file bin/check-project-contract.sh
require_file bin/check-public-api-inventory.sh
require_file .github/workflows/ci.yml
require_file docs/public-api-inventory.md
require_file docs/vstr-api-parity.md
require_file docs/top-rust-utility-gap-analysis.md

require_text Cargo.toml 'edition = "2024"'
require_text Cargo.toml 'rust-version = "1.85"'
require_text Cargo.toml 'unsafe_code = "forbid"'
require_text Cargo.toml 'missing_docs = "warn"'
require_text Cargo.toml 'pedantic = "warn"'
require_text Cargo.toml 'repository = "https://github.com/imajinyun/knifer-rs"'
require_text Cargo.toml 'documentation = "https://docs.rs/knifer-rs"'
require_text .github/workflows/ci.yml 'workflow_dispatch:'
require_text .github/workflows/ci.yml 'macos-latest'
require_text .github/workflows/ci.yml 'bash bin/check-public-api-inventory.sh'
require_text .github/workflows/ci.yml 'cargo package --list --allow-dirty'
require_text .editorconfig 'end_of_line = lf'
require_text .editorconfig '[*.rs]'
require_text .gitattributes '* text=auto eol=lf'
require_text .gitignore '/target/'
require_text .gitignore '.env.*'
require_text .gitignore '*.profraw'
if grep -Fq '/docs/' .gitignore; then
  echo "docs/ contains source documentation and must not be ignored" >&2
  exit 1
fi
require_text README.md 'knifer_rs::vstr'
require_text README.md 'Benchmark Direction'
require_text README.md 'docs/public-api-inventory.md'
require_text README.md 'MSRV: Rust 1.85'
require_text README.md 'zero runtime dependencies'
require_text README.md 'anyhow'
require_text README.md 'cargo test --locked'
require_text README.md 'cargo clippy --all-targets -- -D warnings'
require_text README.md 'bash bin/check-project-contract.sh'
require_text README.md 'bash bin/check-public-api-inventory.sh'
require_text README.md 'cargo package --list --allow-dirty'
require_text CONTRIBUTING.md 'bash bin/check-project-contract.sh'
require_text CONTRIBUTING.md 'bash bin/check-public-api-inventory.sh'
require_text CONTRIBUTING.md 'cargo package --list --allow-dirty'
require_text docs/public-api-inventory.md 'Public API Inventory'
require_text docs/public-api-inventory.md 'knifer_rs::vstr'
require_text docs/public-api-inventory.md 'EmojiOptions'
require_text docs/public-api-inventory.md 'ant_path_match_with_separator'
require_text docs/public-api-inventory.md 'levenshtein_distance'
require_text docs/public-api-inventory.md 'Open Inventory Work'
require_text docs/vstr-api-parity.md 'Open Compatibility Work'
require_text docs/top-rust-utility-gap-analysis.md 'Top Rust Helper Utility Gap Analysis'
require_text docs/top-rust-utility-gap-analysis.md 'dtolnay/anyhow'
require_text docs/top-rust-utility-gap-analysis.md 'dtolnay/thiserror'
require_text docs/top-rust-utility-gap-analysis.md 'serde-rs/json'
require_text docs/top-rust-utility-gap-analysis.md 'docs/public-api-inventory.md'
require_text docs/top-rust-utility-gap-analysis.md 'generated public API inventory drift check'
require_text docs/top-rust-utility-gap-analysis.md 'RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-project-contract.sh'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-public-api-inventory.sh'
require_text docs/top-rust-utility-gap-analysis.md 'cargo package --list --allow-dirty'
require_text docs/top-rust-utility-gap-analysis.md '.gitignore'
require_text docs/top-rust-utility-gap-analysis.md '.gitattributes'

if grep -R --include='*.rs' -n '\bunsafe\b' src; then
  echo "unsafe Rust is not allowed in src/" >&2
  exit 1
fi
