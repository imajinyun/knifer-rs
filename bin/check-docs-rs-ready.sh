#!/usr/bin/env bash
set -euo pipefail

require_text() {
  local path="$1"
  local text="$2"

  if ! grep -Fq -- "$text" "$path"; then
    echo "missing required text in $path: $text" >&2
    exit 1
  fi
}

require_text Cargo.toml 'documentation = "https://docs.rs/knifer-rs"'
require_text Cargo.toml 'readme = "README.md"'
require_text Cargo.toml 'keywords = ['
require_text Cargo.toml 'categories = ['
require_text Cargo.toml '[package.metadata.docs.rs]'
require_text Cargo.toml 'all-features = true'
require_text Cargo.toml 'rustdoc-args = ["--cfg", "docsrs"]'
require_text README.md '## 📦 Install'
require_text README.md '## 🚦 Status'
require_text README.md '## ⚡ Quick Start'
require_text README.md '## 🧩 Feature Flags'
require_text README.md '## 🔒 Compatibility'

RUSTDOCFLAGS="-Dwarnings --cfg docsrs" cargo doc --locked --all-features --no-deps
cargo package --locked --allow-dirty
