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
require_file docs/dependency-policy.md
require_file bin/check-project-contract.sh
require_file bin/check-public-api-inventory.sh
require_file bin/check-vstr-bench.sh
require_file bin/check-vstr-benchmark-smoke.sh
require_file .github/workflows/ci.yml
require_file docs/public-api-inventory.md
require_file docs/vstr-api-parity.md
require_file docs/vstr-complexity.md
require_file docs/top-rust-utility-gap-analysis.md
require_file docs/vstr-top-string-gap-analysis.md
require_file benches/vstr_bench.rs
require_file examples/vstr_benchmark_smoke.rs

require_text Cargo.toml 'edition = "2024"'
require_text Cargo.toml 'rust-version = "1.85"'
require_text Cargo.toml 'unsafe_code = "forbid"'
require_text Cargo.toml 'missing_docs = "warn"'
require_text Cargo.toml 'pedantic = "warn"'
require_text Cargo.toml 'repository = "https://github.com/imajinyun/knifer-rs"'
require_text Cargo.toml 'documentation = "https://docs.rs/knifer-rs"'
require_text Cargo.toml 'name = "vstr_bench"'
require_text Cargo.toml 'harness = false'
if awk '
  /^\[dependencies\]$/ { in_deps = 1; next }
  /^\[/ { in_deps = 0 }
  in_deps && $0 !~ /^[[:space:]]*$/ && $0 !~ /^[[:space:]]*#/ { found = 1 }
  END { exit found ? 0 : 1 }
' Cargo.toml; then
  echo "default runtime dependencies are not allowed without updating docs/dependency-policy.md" >&2
  exit 1
fi
require_text .github/workflows/ci.yml 'workflow_dispatch:'
require_text .github/workflows/ci.yml 'macos-latest'
require_text .github/workflows/ci.yml 'bash bin/check-public-api-inventory.sh'
require_text .github/workflows/ci.yml 'bash bin/check-vstr-benchmark-smoke.sh'
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
require_text README.md 'docs/vstr-top-string-gap-analysis.md'
require_text README.md 'docs/vstr-complexity.md'
require_text README.md 'docs/dependency-policy.md'
require_text README.md 'MSRV: Rust 1.85'
require_text README.md 'zero runtime dependencies'
require_text README.md 'anyhow'
require_text README.md 'vstr::slugify'
require_text README.md 'vstr::between'
require_text README.md 'vstr::split_once_last'
require_text README.md 'vstr::to_train_case'
require_text README.md 'vstr::to_dot_case'
require_text README.md 'vstr::capitalize'
require_text README.md 'vstr::remove_whitespace'
require_text README.md 'vstr::normalize_newlines'
require_text README.md 'vstr::trim_lines'
require_text README.md 'vstr::trim_blank_lines'
require_text README.md 'vstr::take_chars'
require_text README.md 'vstr::truncate_with_suffix'
require_text README.md 'vstr::abbreviate_middle'
require_text README.md 'vstr::mask'
require_text README.md 'vstr::collapse_repeated_char'
require_text README.md 'vstr::center'
require_text README.md 'vstr::wrap'
require_text README.md 'vstr::wrap_with_indent'
require_text README.md 'vstr::non_blank_lines'
require_text README.md 'vstr::initials'
require_text README.md 'vstr::is_palindrome'
require_text README.md 'vstr::extract_digits'
require_text README.md 'vstr::find_any'
require_text README.md 'vstr::find_all'
require_text README.md 'vstr::find_all_ignore_case'
require_text README.md 'vstr::strip_suffix_ignore_case'
require_text README.md 'vstr::count_matches'
require_text README.md 'vstr::replace_ignore_case'
require_text README.md 'vstr::replace_many'
require_text README.md 'vstr::escape_regex'
require_text README.md 'vstr::quote_meta'
require_text README.md 'cargo test --locked'
require_text README.md 'cargo clippy --all-targets -- -D warnings'
require_text README.md 'bash bin/check-project-contract.sh'
require_text README.md 'bash bin/check-public-api-inventory.sh'
require_text README.md 'bash bin/check-vstr-benchmark-smoke.sh'
require_text README.md 'bash bin/check-vstr-bench.sh'
require_text README.md 'cargo package --list --allow-dirty'
require_text CONTRIBUTING.md 'bash bin/check-project-contract.sh'
require_text CONTRIBUTING.md 'bash bin/check-public-api-inventory.sh'
require_text CONTRIBUTING.md 'cargo package --list --allow-dirty'
require_text CONTRIBUTING.md 'docs/dependency-policy.md'
require_text docs/dependency-policy.md 'Dependency Policy'
require_text docs/dependency-policy.md 'zero-runtime-dependency core'
require_text docs/dependency-policy.md 'Do not add runtime dependencies to the default feature set.'
require_text docs/dependency-policy.md 'optional'
require_text docs/dependency-policy.md 'MSRV'
require_text docs/dependency-policy.md 'pattern-regex'
require_text docs/dependency-policy.md 'unicode-segmentation'
require_text docs/public-api-inventory.md 'Public API Inventory'
require_text docs/public-api-inventory.md 'knifer_rs::vstr'
require_text docs/public-api-inventory.md 'EmojiOptions'
require_text docs/public-api-inventory.md 'to_screaming_snake_case'
require_text docs/public-api-inventory.md 'to_dot_case'
require_text docs/public-api-inventory.md 'to_path_case'
require_text docs/public-api-inventory.md 'to_train_case'
require_text docs/public-api-inventory.md 'to_cobol_case'
require_text docs/public-api-inventory.md 'to_sentence_case'
require_text docs/public-api-inventory.md 'capitalize'
require_text docs/public-api-inventory.md 'uncapitalize'
require_text docs/public-api-inventory.md 'swap_case'
require_text docs/public-api-inventory.md 'normalize_whitespace'
require_text docs/public-api-inventory.md 'remove_whitespace'
require_text docs/public-api-inventory.md 'between'
require_text docs/public-api-inventory.md 'contains_none'
require_text docs/public-api-inventory.md 'contains_any_ignore_case'
require_text docs/public-api-inventory.md 'find_any'
require_text docs/public-api-inventory.md 'count_matches'
require_text docs/public-api-inventory.md 'find_all'
require_text docs/public-api-inventory.md 'find_all_ignore_case'
require_text docs/public-api-inventory.md 'replace_first'
require_text docs/public-api-inventory.md 'replace_last'
require_text docs/public-api-inventory.md 'replace_ignore_case'
require_text docs/public-api-inventory.md 'replace_many'
require_text docs/public-api-inventory.md 'escape_regex'
require_text docs/public-api-inventory.md 'quote_meta'
require_text docs/public-api-inventory.md 'split_once_last'
require_text docs/public-api-inventory.md 'strip_prefix_ignore_case'
require_text docs/public-api-inventory.md 'slugify_with_separator'
require_text docs/public-api-inventory.md 'take_chars'
require_text docs/public-api-inventory.md 'drop_chars'
require_text docs/public-api-inventory.md 'normalize_newlines'
require_text docs/public-api-inventory.md 'trim_lines'
require_text docs/public-api-inventory.md 'trim_blank_lines'
require_text docs/public-api-inventory.md 'abbreviate_middle'
require_text docs/public-api-inventory.md 'limit_words'
require_text docs/public-api-inventory.md 'excerpt'
require_text docs/public-api-inventory.md 'mask'
require_text docs/public-api-inventory.md 'collapse_repeated_char'
require_text docs/public-api-inventory.md 'center'
require_text docs/public-api-inventory.md 'dedent'
require_text docs/public-api-inventory.md 'wrap_with_indent'
require_text docs/public-api-inventory.md 'non_blank_lines'
require_text docs/public-api-inventory.md 'initials'
require_text docs/public-api-inventory.md 'is_palindrome'
require_text docs/public-api-inventory.md 'extract_digits'
require_text docs/public-api-inventory.md 'remove_ascii_punctuation'
require_text docs/public-api-inventory.md 'surround'
require_text docs/public-api-inventory.md 'unsurround'
require_text docs/public-api-inventory.md 'word_count'
require_text docs/public-api-inventory.md 'ant_path_match_with_separator'
require_text docs/public-api-inventory.md 'levenshtein_distance'
require_text docs/public-api-inventory.md 'Open Inventory Work'
require_text docs/vstr-api-parity.md 'Open Compatibility Work'
require_text docs/vstr-complexity.md '`vstr` Complexity and Allocation Notes'
require_text docs/vstr-complexity.md 'replace_many'
require_text docs/vstr-complexity.md 'find_all_ignore_case'
require_text docs/vstr-complexity.md 'levenshtein_distance'
require_text docs/vstr-complexity.md 'ant_path_match'
require_text docs/vstr-complexity.md 'bash bin/check-vstr-bench.sh'
require_text docs/top-rust-utility-gap-analysis.md 'Top Rust Helper Utility Gap Analysis'
require_text docs/top-rust-utility-gap-analysis.md 'dtolnay/anyhow'
require_text docs/top-rust-utility-gap-analysis.md 'dtolnay/thiserror'
require_text docs/top-rust-utility-gap-analysis.md 'serde-rs/json'
require_text docs/top-rust-utility-gap-analysis.md 'withoutboats/heck'
require_text docs/top-rust-utility-gap-analysis.md 'unicode-rs/unicode-segmentation'
require_text docs/top-rust-utility-gap-analysis.md 'docs/public-api-inventory.md'
require_text docs/top-rust-utility-gap-analysis.md 'generated public API inventory drift check'
require_text docs/top-rust-utility-gap-analysis.md 'RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-project-contract.sh'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-public-api-inventory.sh'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-vstr-benchmark-smoke.sh'
require_text docs/top-rust-utility-gap-analysis.md 'bash bin/check-vstr-bench.sh'
require_text docs/top-rust-utility-gap-analysis.md 'cargo package --list --allow-dirty'
require_text docs/top-rust-utility-gap-analysis.md '.gitignore'
require_text docs/top-rust-utility-gap-analysis.md '.gitattributes'
require_text docs/vstr-top-string-gap-analysis.md 'Vstr Top String Library Gap Analysis'
require_text docs/vstr-top-string-gap-analysis.md 'rust-lang/regex'
require_text docs/vstr-top-string-gap-analysis.md 'BurntSushi/memchr'
require_text docs/vstr-top-string-gap-analysis.md 'BurntSushi/aho-corasick'
require_text docs/vstr-top-string-gap-analysis.md 'BurntSushi/bstr'
require_text docs/vstr-top-string-gap-analysis.md 'unicode-rs/unicode-segmentation'
require_text docs/vstr-top-string-gap-analysis.md 'docs/vstr-complexity.md'
require_text docs/vstr-top-string-gap-analysis.md 'docs/dependency-policy.md'
require_text benches/vstr_bench.rs 'bench_find_all'
require_text benches/vstr_bench.rs 'bench_levenshtein'
require_text examples/vstr_benchmark_smoke.rs 'replace_many'
require_text examples/vstr_benchmark_smoke.rs 'levenshtein_distance'

if grep -R --include='*.rs' -n '\bunsafe\b' src; then
  echo "unsafe Rust is not allowed in src/" >&2
  exit 1
fi
