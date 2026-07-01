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

  if ! grep -Fq -- "$text" "$path"; then
    echo "missing required text in $path: $text" >&2
    exit 1
  fi
}

require_file Cargo.toml
require_file Cargo.lock
require_file .editorconfig
require_file .gitattributes
require_file .gitignore
require_file aiflow.yaml
require_file LICENSE
require_file SECURITY.md
require_file CONTRIBUTING.md
require_file CHANGELOG.md
require_file README.md
require_file docs/api-behavior-contract.md
require_file docs/dependency-policy.md
require_file src/vbytes.rs
require_file src/vencoding.rs
require_file src/vstr/basic/mod.rs
require_file src/vstr/basic/affix.rs
require_file src/vstr/basic/casefold.rs
require_file src/vstr/basic/compare.rs
require_file src/vstr/basic/escape.rs
require_file src/vstr/basic/format.rs
require_file src/vstr/basic/measure.rs
require_file src/vstr/basic/predicate.rs
require_file src/vstr/basic/replace.rs
require_file src/vstr/basic/search.rs
require_file src/vstr/basic/split.rs
require_file src/vstr/basic/substring.rs
require_file src/vstr/basic/trim.rs
require_file src/vstr/basic/value.rs
require_file src/vstr/matcher.rs
require_file src/vstr/matcher/backend.rs
require_file src/vstr/matcher/search.rs
require_file src/vstr/matcher/types.rs
require_file src/vstr/text.rs
require_file src/vstr/text/content.rs
require_file src/vstr/text/inspect.rs
require_file src/vstr/text/normalize.rs
require_file src/vstr/text/transform.rs
require_file src/vstr/text/wrap.rs
require_file src/vstr/text/wrap/basic.rs
require_file src/vstr/text/wrap/options.rs
require_file src/vstr/text/wrap/options_wrap.rs
require_file src/vstr/text/wrap/tokens.rs
require_file src/vstr/width.rs
require_file src/vstr/width/measure.rs
require_file src/vstr/width/wrap.rs
require_file src/vstr/width/wrap/basic.rs
require_file src/vstr/width/wrap/options_wrap.rs
require_file src/vstr/width/wrap/tokens.rs
require_file src/vstr/tests.rs
require_file src/vstr/tests/basic.rs
require_file src/vstr/tests/case.rs
require_file src/vstr/tests/classify.rs
require_file src/vstr/tests/emoji.rs
require_file src/vstr/tests/encoding.rs
require_file src/vstr/tests/matcher.rs
require_file src/vstr/tests/path.rs
require_file src/vstr/tests/property.rs
require_file src/vstr/tests/similarity.rs
require_file src/vstr/tests/support.rs
require_file src/vstr/tests/text.rs
require_file src/vstr/tests/unicode.rs
require_file bin/check-docs-rs-ready.sh
require_file bin/check-examples.sh
require_file bin/check-package-contents.sh
require_file bin/check-api-semver.sh
require_file bin/check-release-api-semver.sh
require_file bin/check-release-gate-layers.sh
require_file bin/check-release-ready.sh
require_file bin/check-project-contract.sh
require_file bin/check-public-api-inventory.sh
require_file bin/check-vstr-bench.sh
require_file bin/check-vstr-benchmark-smoke.sh
require_file bin/check-vstr-fuzz.sh
require_file bin/check-vstr-fuzz-smoke.sh
require_file .github/workflows/ci.yml
require_file docs/public-api-inventory.md
require_file docs/vstr-api-parity.md
require_file docs/vstr-complexity.md
require_file docs/vstr-benchmark-history.md
require_file docs/vstr-matcher-backend-plan.md
require_file bench/vstr_bench.rs
require_file examples/vstr_daily.rs
require_file examples/vstr_unicode.rs
require_file examples/vbytes_encoding.rs
require_file examples/vstr_matcher.rs
require_file examples/vstr_benchmark_smoke.rs
require_file fuzz/Cargo.toml
require_file fuzz/Cargo.lock
require_file fuzz/README.md
require_file fuzz/PLAN.md
require_file fuzz/corpus/substring.txt
require_file fuzz/corpus/escaping.txt
require_file fuzz/corpus/path_matching.txt
require_file fuzz/corpus/replacement.txt
require_file fuzz/corpus/matcher.txt
require_file fuzz/corpus/text_boundaries.txt
require_file fuzz/fuzz_targets/substring.rs
require_file fuzz/fuzz_targets/escaping.rs
require_file fuzz/fuzz_targets/path_matching.rs
require_file fuzz/fuzz_targets/replacement.rs
require_file fuzz/fuzz_targets/matcher.rs
require_file fuzz/fuzz_targets/text_boundaries.rs

require_text Cargo.toml 'edition = "2024"'
require_text Cargo.toml 'rust-version = "1.85"'
require_text Cargo.toml 'unsafe_code = "forbid"'
require_text Cargo.toml 'missing_docs = "warn"'
require_text Cargo.toml 'pedantic = "warn"'
require_text Cargo.toml 'repository = "https://github.com/imajinyun/knifer-rs"'
require_text Cargo.toml 'documentation = "https://docs.rs/knifer-rs"'
require_text Cargo.toml 'readme = "README.md"'
require_text Cargo.toml 'keywords = ['
require_text Cargo.toml 'categories = ['
require_text Cargo.toml '[package.metadata.docs.rs]'
require_text Cargo.toml 'all-features = true'
require_text Cargo.toml 'rustdoc-args = ["--cfg", "docsrs"]'
require_text Cargo.toml 'name = "vstr_bench"'
require_text Cargo.toml 'path = "bench/vstr_bench.rs"'
require_text Cargo.toml 'harness = false'
require_text Cargo.toml '[dev-dependencies]'
require_text Cargo.toml 'serde_json = "1"'
require_text Cargo.toml '[features]'
require_text Cargo.toml 'default = []'
require_text Cargo.toml 'aho-corasick = { version = "1", optional = true }'
require_text Cargo.toml 'regex = { version = "1", optional = true }'
require_text Cargo.toml 'unicode-segmentation = { version = "1", optional = true }'
require_text Cargo.toml 'unicode-width = { version = "0.2", optional = true }'
require_text Cargo.toml 'matcher-aho-corasick = ["dep:aho-corasick"]'
require_text Cargo.toml 'pattern-regex = ["dep:regex"]'
require_text Cargo.toml 'unicode-segmentation = ["dep:unicode-segmentation"]'
require_text Cargo.toml 'unicode-width = ["dep:unicode-width"]'
require_text aiflow.yaml '    - "src/vstr/basic/*"'
require_text aiflow.yaml '    - "src/vstr/matcher/*"'
require_text aiflow.yaml '    - "src/vstr/text/*"'
require_text aiflow.yaml '    - "src/vstr/text/wrap/*"'
require_text aiflow.yaml '    - "src/vstr/width/*"'
require_text aiflow.yaml '    - "src/vstr/width/wrap/*"'
require_text aiflow.yaml '    - ".aiflow/*"'
require_text src/vstr/basic/mod.rs 'mod affix;'
require_text src/vstr/basic/mod.rs 'mod casefold;'
require_text src/vstr/basic/mod.rs 'mod compare;'
require_text src/vstr/basic/mod.rs 'mod escape;'
require_text src/vstr/basic/mod.rs 'mod format;'
require_text src/vstr/basic/mod.rs 'mod measure;'
require_text src/vstr/basic/mod.rs 'mod predicate;'
require_text src/vstr/basic/mod.rs 'mod replace;'
require_text src/vstr/basic/mod.rs 'mod search;'
require_text src/vstr/basic/mod.rs 'mod split;'
require_text src/vstr/basic/mod.rs 'mod substring;'
require_text src/vstr/basic/mod.rs 'mod trim;'
require_text src/vstr/basic/mod.rs 'mod value;'
require_text src/vstr/basic/mod.rs 'pub use affix::*;'
require_text src/vstr/basic/mod.rs 'pub use compare::*;'
require_text src/vstr/basic/mod.rs 'pub use escape::*;'
require_text src/vstr/basic/mod.rs 'pub use format::*;'
require_text src/vstr/basic/mod.rs 'pub use measure::*;'
require_text src/vstr/basic/mod.rs 'pub use predicate::*;'
require_text src/vstr/basic/mod.rs 'pub use replace::*;'
require_text src/vstr/basic/mod.rs 'pub use search::*;'
require_text src/vstr/basic/mod.rs 'pub use split::*;'
require_text src/vstr/basic/mod.rs 'pub use substring::*;'
require_text src/vstr/basic/mod.rs 'pub use trim::*;'
require_text src/vstr/basic/mod.rs 'pub use value::*;'
require_text src/vstr/matcher.rs 'mod search;'
require_text src/vstr/matcher.rs 'mod types;'
require_text src/vstr/matcher.rs 'pub use types::{MatchKind, VStrMatch, VStrMatcher};'
require_text src/vstr/matcher.rs '#[cfg(feature = "matcher-aho-corasick")]'
require_text src/vstr/matcher.rs 'mod backend;'
require_text src/vstr/text.rs 'mod content;'
require_text src/vstr/text.rs 'mod inspect;'
require_text src/vstr/text.rs 'mod normalize;'
require_text src/vstr/text.rs 'mod transform;'
require_text src/vstr/text.rs 'mod wrap;'
require_text src/vstr/text.rs 'pub use content::*;'
require_text src/vstr/text.rs 'pub use inspect::*;'
require_text src/vstr/text.rs 'pub use normalize::*;'
require_text src/vstr/text.rs 'pub use transform::*;'
require_text src/vstr/text.rs 'pub use wrap::*;'
require_text src/vstr/text/wrap.rs 'mod basic;'
require_text src/vstr/text/wrap.rs 'mod options;'
require_text src/vstr/text/wrap.rs 'mod options_wrap;'
require_text src/vstr/text/wrap.rs 'mod tokens;'
require_text src/vstr/text/wrap.rs 'pub use basic::*;'
require_text src/vstr/text/wrap.rs 'pub use options::*;'
require_text src/vstr/text/wrap.rs 'pub use options_wrap::*;'
require_text src/vstr/width.rs 'mod measure;'
require_text src/vstr/width.rs 'mod wrap;'
require_text src/vstr/width.rs 'pub use measure::*;'
require_text src/vstr/width.rs 'pub use wrap::*;'
require_text src/vstr/width/wrap.rs 'mod basic;'
require_text src/vstr/width/wrap.rs 'mod options_wrap;'
require_text src/vstr/width/wrap.rs 'mod tokens;'
require_text src/vstr/width/wrap.rs 'pub use basic::*;'
require_text src/vstr/width/wrap.rs 'pub use options_wrap::*;'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/affix.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/compare.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/escape.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/format.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/measure.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/mod.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/predicate.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/replace.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/search.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/split.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/substring.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/trim.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/basic/value.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/matcher/search.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/matcher/types.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/content.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/inspect.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/normalize.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/transform.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/wrap.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/wrap/basic.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/wrap/options.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/text/wrap/options_wrap.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/width/measure.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/width/wrap.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/width/wrap/basic.rs'
require_text bin/check-public-api-inventory.sh '  src/vstr/width/wrap/options_wrap.rs'
require_text bin/check-public-api-inventory.sh 'public API source file not covered by'
require_text bin/check-public-api-inventory.sh "grep -vE '(^|/)tests(/|\\.rs\$)'"
if awk '
  /^\[dependencies\]$/ { in_deps = 1; next }
  /^\[/ { in_deps = 0 }
  in_deps && $0 !~ /^[[:space:]]*$/ && $0 !~ /^[[:space:]]*#/ && $0 !~ /optional[[:space:]]*=[[:space:]]*true/ { found = 1 }
  END { exit found ? 0 : 1 }
' Cargo.toml; then
  echo "non-optional runtime dependencies are not allowed without updating docs/dependency-policy.md" >&2
  exit 1
fi
require_text .github/workflows/ci.yml 'workflow_dispatch:'
require_text .github/workflows/ci.yml 'macos-latest'
# CI coverage of the vet, publish-readiness, and release-evidence command layers
# is verified structurally by bin/check-release-gate-layers.sh, which asserts the
# CI jobs run each aiflow profile's commands. Keep the gate-layers step assertion
# as a bootstrap anchor (the guard cannot verify its own invocation) plus the
# structural workflow wiring (conditional inputs, env, and artifact steps) that
# are not commands in any layer.
require_text .github/workflows/ci.yml 'bash bin/check-release-gate-layers.sh'
require_text .github/workflows/ci.yml 'run_release_api_semver'
require_text .github/workflows/ci.yml 'api_semver_baseline_ref'
require_text .github/workflows/ci.yml 'API_SEMVER_BASELINE_REF'
require_text .github/workflows/ci.yml 'run_release_bench'
require_text .github/workflows/ci.yml 'benchmark_base_ref'
require_text .github/workflows/ci.yml 'benchmark_max_regression_pct'
require_text .github/workflows/ci.yml 'VSTR_BENCH_BASE_REF'
require_text .github/workflows/ci.yml 'VSTR_BENCH_MAX_REGRESSION_PCT'
require_text .github/workflows/ci.yml 'Release Benchmark Report'
require_text .github/workflows/ci.yml 'bash bin/check-vstr-bench.sh target/vstr-bench-report'
require_text .github/workflows/ci.yml 'actions/upload-artifact@v4'
require_text CHANGELOG.md '## Unreleased'
require_text CHANGELOG.md 'vbytes'
require_text CHANGELOG.md 'vencoding'
require_text CHANGELOG.md 'WrapOptions'
require_text CHANGELOG.md 'VStrMatcher'
require_text CHANGELOG.md 'matcher-aho-corasick'
require_text CHANGELOG.md 'pattern-regex'
require_text CHANGELOG.md 'unicode-segmentation'
require_text CHANGELOG.md 'unicode-width'
require_text CHANGELOG.md 'cargo-semver-checks'
require_text CHANGELOG.md 'benchmark JSON/Markdown artifacts'
require_text CHANGELOG.md 'cargo-fuzz'
require_text CHANGELOG.md 'public API stability classes'
require_text CHANGELOG.md 'case conversion acronym/number/separator/non-ASCII matrix cases'
require_text CHANGELOG.md 'zero-runtime-dependency'
require_text CHANGELOG.md 'cargo package --locked --allow-dirty'
require_text .editorconfig 'end_of_line = lf'
require_text .editorconfig '[*.rs]'
require_text .gitattributes '* text=auto eol=lf'
require_text .gitignore '/target/'
require_text .gitignore '/fuzz/target/'
require_text .gitignore '/.cargo/config.toml'
require_text .gitignore 'cargo-timing-*.html'
require_text .gitignore '.env.*'
require_text .gitignore 'tags'
require_text .gitignore 'lcov.info'
require_text .gitignore '*.profraw'
require_text .gitignore 'perf.data'
require_text .gitignore 'flamegraph.svg'
require_text .gitignore '/criterion/'
require_text .gitignore '/fuzz/artifacts/'
require_text .gitignore '/fuzz/corpus/*'
require_text .gitignore '!/fuzz/corpus/*.txt'
require_text .gitignore '/.aiflow/'
require_text .gitignore '/tmp/'
if grep -Eq '(^|/|\*)docs(/|\*|$)' .gitignore; then
  echo "docs/ contains source documentation and must not be ignored" >&2
  exit 1
fi
require_text README.md 'knifer_rs::vstr'
require_text README.md 'knifer_rs::vbytes'
require_text README.md 'knifer_rs::vencoding'
require_text README.md 'root `aiflow.yaml`'
require_text README.md '`.aiflow/` directory'
require_text README.md 'Benchmark Direction'
require_text README.md 'docs/public-api-inventory.md'
require_text README.md 'docs/api-behavior-contract.md'
require_text README.md 'docs/vstr-complexity.md'
require_text README.md 'docs/dependency-policy.md'
require_text README.md 'MSRV: Rust 1.85'
require_text README.md 'zero runtime dependencies'
require_text README.md '## Quick Start'
require_text README.md 'use knifer_rs::{vbytes, vencoding, vstr};'
require_text README.md 'vbytes::replace_all'
require_text README.md 'vencoding::strip_bom'
require_text README.md '## Feature Flags'
require_text README.md '| `matcher-aho-corasick` |'
require_text README.md '| `pattern-regex` |'
require_text README.md '| `unicode-segmentation` |'
require_text README.md '| `unicode-width` |'
require_text README.md 'anyhow'
require_text README.md 'vstr::slugify'
require_text README.md 'vbytes::replace_all'
require_text README.md 'vencoding::strip_bom'
require_text README.md 'examples/vstr_daily.rs'
require_text README.md 'examples/vstr_unicode.rs'
require_text README.md 'examples/vbytes_encoding.rs'
require_text README.md 'examples/vstr_matcher.rs'
require_text README.md 'bash bin/check-release-ready.sh'
require_text README.md 'release-detail'
require_text README.md 'fast vet gates'
require_text README.md 'publish readiness gates'
require_text README.md 'release evidence smoke gates'
require_text README.md 'bin/check-release-gate-layers.sh'
require_text README.md 'default/no-default/all-features tests'
require_text README.md 'all-features clippy'
require_text README.md 'package contents'
require_text README.md 'benchmark smoke'
require_text README.md 'fuzz smoke'
require_text README.md 'cargo-semver-checks check-release'
require_text README.md 'API_SEMVER_BASELINE_REF'
require_text README.md 'run_release_api_semver'
require_text README.md 'bash bin/check-vstr-bench.sh'
require_text README.md 'bash bin/check-vstr-fuzz.sh'
require_text README.md 'target/vstr-bench-report'
require_text README.md 'schema/version plus rustc, target, feature set, and commit metadata'
require_text README.md 'run_release_bench'
require_text README.md 'VSTR_BENCH_BASELINE_JSON'
require_text README.md 'VSTR_BENCH_BASE_REF'
require_text README.md 'VSTR_BENCH_MAX_REGRESSION_PCT'
require_text README.md 'vstr-bench-compare.json'
require_text README.md 'docs/vstr-benchmark-history.md'
require_text README.md 'fast CI coverage check'
require_text README.md 'formal `cargo bench` target'
require_text README.md 'docs.rs readiness check'
require_text README.md 'cargo package --locked --allow-dirty'
require_text README.md 'package contents check'
require_text README.md 'excluding local'
require_text README.md 'text-boundary target'
require_text README.md 'cargo fuzz run fuzz_substring'
require_text README.md 'cargo fuzz run fuzz_escaping'
require_text README.md 'cargo fuzz run fuzz_matcher'
require_text README.md 'VSTR_FUZZ_RUN_SECS'
require_text README.md 'core stable facade'
require_text README.md 'optional feature facade'
require_text README.md 'experimental-but-public APIs'
require_text README.md 'behavior evidence matrix'
require_text CONTRIBUTING.md 'bash bin/check-project-contract.sh'
require_text CONTRIBUTING.md 'bash bin/check-public-api-inventory.sh'
require_text CONTRIBUTING.md 'bash bin/check-api-semver.sh'
require_text CONTRIBUTING.md 'bash bin/check-release-api-semver.sh'
require_text CONTRIBUTING.md 'bash bin/check-package-contents.sh'
require_text CONTRIBUTING.md 'API_SEMVER_BASELINE_REF'
require_text CONTRIBUTING.md 'bash bin/check-docs-rs-ready.sh'
require_text CONTRIBUTING.md 'bash bin/check-examples.sh'
require_text CONTRIBUTING.md 'cargo test --locked --examples'
require_text CONTRIBUTING.md 'cargo test --locked --no-default-features'
require_text CONTRIBUTING.md 'cargo test --locked --all-features'
require_text CONTRIBUTING.md 'cargo clippy --all-targets --all-features -- -D warnings'
require_text CONTRIBUTING.md '## Release Checklist'
require_text CONTRIBUTING.md 'bash bin/check-release-ready.sh'
require_text CONTRIBUTING.md 'bin/check-release-gate-layers.sh'
require_text CONTRIBUTING.md 'commands.release'
require_text CONTRIBUTING.md 'commands.release-detail'
require_text CONTRIBUTING.md 'commands.publish-readiness'
require_text CONTRIBUTING.md 'commands.release-evidence'
require_text CONTRIBUTING.md 'fast vet gates'
require_text CONTRIBUTING.md 'publish readiness gates'
require_text CONTRIBUTING.md 'release evidence smoke gates'
require_text CONTRIBUTING.md 'cargo test --locked --no-default-features'
require_text CONTRIBUTING.md 'cargo test --locked --all-features'
require_text CONTRIBUTING.md 'cargo package --locked --allow-dirty'
require_text CONTRIBUTING.md 'keeps local runtime state out of the'
require_text CONTRIBUTING.md 'confirm `CHANGELOG.md` explains the release boundary'
require_text CONTRIBUTING.md 'docs/dependency-policy.md'
require_text CONTRIBUTING.md 'docs/api-behavior-contract.md'
require_text CONTRIBUTING.md 'Classify each new public API'
require_text CONTRIBUTING.md 'removed or changed signatures as breaking'
require_text CONTRIBUTING.md 'aiflow.yaml'
require_text CONTRIBUTING.md '.aiflow/'
require_text aiflow.yaml 'name: knifer-rs'
require_text aiflow.yaml 'language: rust'
require_text aiflow.yaml 'store_path: .aiflow/store.json'
require_text aiflow.yaml 'aiflow.yaml'
require_text aiflow.yaml '".aiflow"'
require_text aiflow.yaml '".aiflow/*"'
require_text aiflow.yaml 'cargo fmt --check'
require_text aiflow.yaml 'cargo test --locked --all-features'
require_text aiflow.yaml 'cargo clippy --all-targets -- -D warnings'
require_text aiflow.yaml 'cargo clippy --all-targets --all-features -- -D warnings'
require_text aiflow.yaml 'bash bin/check-project-contract.sh'
require_text aiflow.yaml 'bash bin/check-public-api-inventory.sh'
require_text aiflow.yaml 'bash bin/check-api-semver.sh'
require_text aiflow.yaml 'bash bin/check-release-gate-layers.sh'
require_text aiflow.yaml 'bash bin/check-release-api-semver.sh'
require_text aiflow.yaml 'bash bin/check-release-ready.sh'
require_text aiflow.yaml 'publish-readiness:'
require_text aiflow.yaml 'release-evidence:'
require_text aiflow.yaml 'release-detail:'
require_text aiflow.yaml 'bash bin/check-package-contents.sh'
require_text aiflow.yaml 'bash bin/check-vstr-benchmark-smoke.sh'
require_text aiflow.yaml 'bash bin/check-vstr-fuzz-smoke.sh'
require_text aiflow.yaml 'aiflow.yaml stays in the repository root while .aiflow stays local-only'
require_text aiflow.yaml '.aiflow contains only local generated evidence, caches, and temporary state'
require_text aiflow.yaml 'allow_commit: false'
require_text aiflow.yaml 'allow_push: false'
require_text aiflow.yaml 'mcp:'
require_text aiflow.yaml 'aiflow-docs-mcp'
require_text docs/dependency-policy.md 'Dependency Policy'
require_text docs/dependency-policy.md 'zero-runtime-dependency core'
require_text docs/dependency-policy.md 'Do not add non-optional runtime dependencies to the default feature set.'
require_text docs/dependency-policy.md 'optional'
require_text docs/dependency-policy.md 'MSRV'
require_text docs/dependency-policy.md 'pattern-regex'
require_text docs/dependency-policy.md 'unicode-segmentation'
require_text docs/dependency-policy.md 'unicode-width'
require_text docs/dependency-policy.md 'matcher-aho-corasick'
require_text docs/dependency-policy.md 'unicode_words'
require_text docs/dependency-policy.md 'unicode_word_indices'
require_text docs/dependency-policy.md 'split_word_bounds'
require_text docs/dependency-policy.md 'unicode_sentences'
require_text docs/dependency-policy.md 'split_sentence_bounds'
require_text docs/dependency-policy.md 'display_width'
require_text docs/dependency-policy.md 'take_width'
require_text docs/dependency-policy.md 'truncate_width'
require_text docs/dependency-policy.md 'wrap_width'
require_text docs/dependency-policy.md 'wrap_width_with_indent'
require_text docs/dependency-policy.md 'wrap_width_with_options'
require_text docs/dependency-policy.md '`vstr` Optional Feature Boundary'
require_text docs/dependency-policy.md 'cargo test --locked --no-default-features'
require_text docs/dependency-policy.md 'cargo test --locked --all-features'
require_text docs/dependency-policy.md '`pattern-regex` Admission Contract'
require_text docs/dependency-policy.md 'contains_pattern'
require_text docs/dependency-policy.md 'PatternError'
require_text docs/dependency-policy.md '`matcher-aho-corasick` Admission Contract'
require_text docs/dependency-policy.md 'The default build must not depend on `aho-corasick`'
require_text docs/dependency-policy.md 'may enable the optional `aho-corasick` dependency'
require_text docs/dependency-policy.md 'Safe Rust'
require_text docs/dependency-policy.md 'docs/vstr-matcher-backend-plan.md'
require_text docs/api-behavior-contract.md 'API Behavior Contract'
require_text docs/api-behavior-contract.md 'Every new public API must be assigned to one stability class'
require_text docs/api-behavior-contract.md 'Core Stable Facade'
require_text docs/api-behavior-contract.md 'Optional Feature Facade'
require_text docs/api-behavior-contract.md 'Experimental-But-Public Facade'
require_text docs/api-behavior-contract.md 'Evidence Matrix'
require_text docs/api-behavior-contract.md 'unit tests'
require_text docs/api-behavior-contract.md 'Unicode boundary tests'
require_text docs/api-behavior-contract.md 'fuzz smoke'
require_text docs/api-behavior-contract.md 'benchmark smoke'
require_text docs/api-behavior-contract.md 'default-build tests'
require_text docs/api-behavior-contract.md 'all-features tests'
require_text docs/api-behavior-contract.md 'parity tests'
require_text docs/api-behavior-contract.md 'knifer_rs::vstr'
require_text docs/api-behavior-contract.md 'knifer_rs::vbytes'
require_text docs/api-behavior-contract.md 'knifer_rs::vencoding'
require_text docs/api-behavior-contract.md 'pattern-regex'
require_text docs/api-behavior-contract.md 'unicode-segmentation'
require_text docs/api-behavior-contract.md 'unicode-width'
require_text docs/api-behavior-contract.md 'matcher-aho-corasick'
require_text docs/api-behavior-contract.md 'VStrMatcher'
require_text docs/api-behavior-contract.md 'bash bin/check-public-api-inventory.sh'
require_text docs/api-behavior-contract.md 'bash bin/check-api-semver.sh'
require_text docs/api-behavior-contract.md 'bash bin/check-project-contract.sh'
require_text docs/public-api-inventory.md 'Public API Inventory'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes::byte_len'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes::is_utf8'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes::to_str'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes::find_all'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes::replace_all'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding::Bom'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding::detect_bom'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding::strip_bom'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding::validate_utf8'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding::decode_utf8_lossy'
require_text docs/public-api-inventory.md 'knifer_rs::vstr'
require_text docs/public-api-inventory.md 'bin/check-api-semver.sh'
require_text docs/public-api-inventory.md 'bin/check-release-api-semver.sh'
require_text docs/public-api-inventory.md 'cargo-semver-checks check-release'
require_text docs/public-api-inventory.md '## Release Baseline Procedure'
require_text docs/public-api-inventory.md 'API_SEMVER_BASELINE_REF=v0.1.0'
require_text docs/public-api-inventory.md 'API_SEMVER_BASELINE_CRATES_IO=true'
require_text docs/public-api-inventory.md 'removed APIs and changed signatures as'
require_text docs/public-api-inventory.md 'additive inventory work'
require_text docs/public-api-inventory.md 'API Stability Classes'
require_text docs/public-api-inventory.md 'Core Stable Facade'
require_text docs/public-api-inventory.md 'Optional Feature Facade'
require_text docs/public-api-inventory.md 'Experimental-But-Public Facade'
require_text docs/public-api-inventory.md 'signature changes here are treated as breaking'
require_text docs/public-api-inventory.md 'These APIs are absent from the default build'
require_text docs/public-api-inventory.md 'All-Features Public API Signature Snapshot'
require_text docs/public-api-inventory.md 'public-api-signatures:start'
require_text docs/public-api-inventory.md 'default zero-runtime-dependency API'
require_text docs/public-api-inventory.md 'public-api-optional-signatures:start'
require_text docs/public-api-inventory.md 'only when the matching optional feature is enabled'
require_text docs/public-api-inventory.md 'EmojiOptions::with_matcher'
require_text docs/public-api-inventory.md 'where I: IntoIterator'
require_text docs/public-api-inventory.md 'Optional Feature Inventory'
require_text docs/public-api-inventory.md 'pattern-regex'
require_text docs/public-api-inventory.md 'unicode-segmentation'
require_text docs/public-api-inventory.md 'unicode-width'
require_text docs/public-api-inventory.md 'unicode_words'
require_text docs/public-api-inventory.md 'unicode_word_len'
require_text docs/public-api-inventory.md 'unicode_word_indices'
require_text docs/public-api-inventory.md 'split_word_bounds'
require_text docs/public-api-inventory.md 'split_word_bound_indices'
require_text docs/public-api-inventory.md 'unicode_sentences'
require_text docs/public-api-inventory.md 'unicode_sentence_len'
require_text docs/public-api-inventory.md 'split_sentence_bounds'
require_text docs/public-api-inventory.md 'split_sentence_bound_indices'
require_text docs/public-api-inventory.md 'display_width'
require_text docs/public-api-inventory.md 'take_width'
require_text docs/public-api-inventory.md 'truncate_width'
require_text docs/public-api-inventory.md 'wrap_width'
require_text docs/public-api-inventory.md 'wrap_width_with_indent'
require_text docs/public-api-inventory.md 'wrap_width_with_options'
require_text docs/public-api-inventory.md 'WrapOptions'
require_text docs/public-api-inventory.md 'WhitespaceMode'
require_text docs/public-api-inventory.md 'LongWordPolicy'
require_text docs/public-api-inventory.md 'wrap_with_options'
require_text docs/public-api-inventory.md 'find_pattern'
require_text docs/public-api-inventory.md 'replace_pattern'
require_text docs/public-api-inventory.md 'VStrMatcher'
require_text docs/public-api-inventory.md 'find_overlapping'
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
require_text docs/vstr-complexity.md 'bash bin/check-vstr-benchmark-smoke.sh'
require_text docs/vstr-complexity.md 'coverage smoke check, not as a performance report'
require_text docs/vstr-complexity.md 'formal benchmark entry point'
require_text docs/vstr-complexity.md 'cargo bench --bench vstr_bench --quiet -- --json'
require_text docs/vstr-complexity.md 'cargo bench --bench vstr_bench --quiet -- --markdown'
require_text docs/vstr-complexity.md 'machine-readable historical report format'
require_text docs/vstr-complexity.md 'vstr-bench.json'
require_text docs/vstr-complexity.md 'VSTR_BENCH_BASELINE_JSON'
require_text docs/vstr-complexity.md 'VSTR_BENCH_BASE_REF'
require_text docs/vstr-complexity.md 'VSTR_BENCH_MAX_REGRESSION_PCT'
require_text docs/vstr-complexity.md 'vstr-bench-compare.json'
require_text docs/vstr-complexity.md 'manual release benchmark artifact'
require_text docs/vstr-complexity.md 'docs/vstr-benchmark-history.md'
require_text docs/vstr-benchmark-history.md '`vstr` Benchmark History Operations'
require_text docs/vstr-benchmark-history.md 'Artifact Location'
require_text docs/vstr-benchmark-history.md 'target/vstr-bench-report'
require_text docs/vstr-benchmark-history.md 'schema'
require_text docs/vstr-benchmark-history.md 'rustc version, target triple, feature set, and'
require_text docs/vstr-benchmark-history.md 'Baseline Selection'
require_text docs/vstr-benchmark-history.md 'VSTR_BENCH_BASE_REF'
require_text docs/vstr-benchmark-history.md 'VSTR_BENCH_BASELINE_JSON'
require_text docs/vstr-benchmark-history.md 'baseline source'
require_text docs/vstr-benchmark-history.md 'run mode'
require_text docs/vstr-benchmark-history.md 'self-describing'
require_text docs/vstr-benchmark-history.md 'Refresh Policy'
require_text docs/vstr-benchmark-history.md 'Threshold Policy'
require_text docs/vstr-benchmark-history.md 'VSTR_BENCH_MAX_REGRESSION_PCT'
require_text docs/vstr-benchmark-history.md 'CI Policy'
require_text docs/vstr-benchmark-history.md 'release-benchmark'
require_text docs/vstr-matcher-backend-plan.md '`vstr` Matcher Backend Plan'
require_text docs/vstr-matcher-backend-plan.md 'matcher-aho-corasick'
require_text docs/vstr-matcher-backend-plan.md 'default feature set must not depend on `aho-corasick`'
require_text docs/vstr-matcher-backend-plan.md 'optional backend uses `aho-corasick`'
require_text docs/vstr-matcher-backend-plan.md 'VStrMatcher::new'
require_text docs/vstr-matcher-backend-plan.md 'MatchKind::LeftmostFirst'
require_text docs/vstr-matcher-backend-plan.md 'MatchKind::LeftmostLongest'
require_text docs/vstr-matcher-backend-plan.md 'find_overlapping'
require_text docs/vstr-matcher-backend-plan.md 'replacement arrays shorter than the original needle list'
require_text docs/vstr-matcher-backend-plan.md 'Safe Rust tie-break layer'
require_text docs/vstr-complexity.md 'Unicode Boundary Policy'
require_text docs/vstr-complexity.md 'emoji ZWJ family sequences'
require_text docs/vstr-complexity.md 'UAX #29 word-boundary helpers'
require_text docs/vstr-complexity.md '`unicode_words`,'
require_text docs/vstr-complexity.md '`split_word_bound_indices`'
require_text docs/vstr-complexity.md 'segments reconstructs the original input'
require_text docs/vstr-complexity.md '`unicode_sentences` filters separator-only spans'
require_text docs/vstr-complexity.md '`split_sentence_bounds` and `split_sentence_bound_indices`'
require_text docs/vstr-complexity.md '`unicode-width` feature adds terminal display-width helpers'
require_text docs/vstr-complexity.md '`display_width`, `take_width`, `truncate_width`, `wrap_width`, and'
require_text docs/vstr-complexity.md '`wrap_width_with_indent`'
require_text docs/vstr-complexity.md '`wrap_with_options`'
require_text docs/vstr-complexity.md '`WrapOptions`, `WhitespaceMode`, and `LongWordPolicy`'
require_text docs/vstr-complexity.md '`wrap_width_with_options`'
require_text docs/vstr-complexity.md 'CJK full-width characters, combining'
require_text docs/vstr-complexity.md 'candidate prefix as a'
require_text docs/vstr-complexity.md 'display-cell budgets'
require_text docs/vstr-complexity.md 'long-word progress guarantees'
require_text docs/vstr-complexity.md 'Wrap and Truncation Boundaries'
require_text docs/vstr-complexity.md 'Consecutive whitespace inside a paragraph collapses to one ASCII space'
require_text docs/vstr-complexity.md 'Long words are split by scalar value'
require_text docs/vstr-complexity.md 'counts indentation inside the requested width'
require_text docs/vstr-complexity.md 'suffix itself is truncated'
require_text docs/vstr-complexity.md 'favoring the front when the budget is odd'
require_text docs/vstr-complexity.md 'Future Multi-Pattern Matcher Contract'
require_text docs/vstr-complexity.md 'leftmost-first'
require_text docs/vstr-complexity.md 'find_overlapping'
require_text docs/vstr-complexity.md 'split_whitespace'
require_text docs/vstr-complexity.md 'wrap_with_indent'
require_text docs/public-api-inventory.md 'Facade Boundaries'
require_text docs/public-api-inventory.md 'knifer_rs::vbytes'
require_text docs/public-api-inventory.md 'knifer_rs::vencoding'
require_text docs/public-api-inventory.md '`vbytes` API Shape'
require_text docs/public-api-inventory.md '`vencoding` API Shape'
require_text docs/public-api-inventory.md 'All byte ranges are byte offsets'
require_text src/lib.rs 'core stable facade'
require_text src/lib.rs 'pub mod vbytes;'
require_text src/lib.rs 'pub mod vencoding;'
require_text src/vstr/mod.rs 'optional feature'
require_text src/vbytes.rs 'Byte-slice utilities for data that may not be valid UTF-8'
require_text src/vbytes.rs 'pub const fn byte_len'
require_text src/vbytes.rs 'pub const fn is_utf8'
require_text src/vbytes.rs 'pub const fn to_str'
require_text src/vbytes.rs 'pub fn find_all'
require_text src/vbytes.rs 'pub fn replace_all'
require_text src/vbytes.rs 'vbytes_core_helpers_use_byte_semantics'
require_text src/vencoding.rs 'Encoding helpers for byte-oriented text boundaries'
require_text src/vencoding.rs 'pub enum Bom'
require_text src/vencoding.rs 'pub const fn detect_bom'
require_text src/vencoding.rs 'pub fn scan_bom'
require_text src/vencoding.rs 'pub fn strip_bom'
require_text src/vencoding.rs 'pub const fn validate_utf8'
require_text src/vencoding.rs 'pub fn decode_utf8_lossy'
require_text src/vencoding.rs 'vencoding_detects_bom_prefixes_with_overlapping_order'
require_text docs/dependency-policy.md '`vencoding` Admission Contract'
require_text docs/dependency-policy.md '`encoding` feature'
require_text docs/dependency-policy.md 'fallback decoding APIs'
require_text src/vstr/tests/unicode.rs 'unicode_boundary_golden_cases_document_scalar_semantics'
require_text src/vstr/tests/text.rs 'wrap_and_truncate_boundary_cases_follow_scalar_width_policy'
require_text src/vstr/tests/text.rs 'wrap_with_options_exposes_scalar_layout_policy'
require_text src/vstr/tests/text.rs 'wrap_width_with_options_exposes_display_layout_policy'
require_text src/vstr/tests/text.rs 'WrapOptions::new(7).with_word_separators'
require_text src/vstr/tests/text.rs 'WhitespaceMode::Preserve'
require_text src/vstr/tests/text.rs 'LongWordPolicy::Preserve'
require_text src/vstr/tests/unicode.rs 'emoji_zwj'
require_text src/vstr/tests/unicode.rs 'mixed_width'
require_text src/vstr/tests/text.rs 'supercalifragilistic'
require_text src/vstr/tests/text.rs 'wrap_with_indent("abcdefghij", 4, ">>>>", "--")'
require_text src/vstr/tests/text.rs 'truncate_with_suffix("abcdef", 3, "...")'
require_text src/vstr/tests/text.rs 'abbreviate_middle("abcdef", 5, "")'
require_text docs/vstr-api-parity.md 'Golden Fixtures'
require_text docs/vstr-api-parity.md 'knifer_go_vstr_golden_fixtures_cover_case_conversion'
require_text docs/vstr-api-parity.md 'case_conversion_cross_crate_fixtures_lock_acronym_number_separator_unicode'
require_text docs/vstr-api-parity.md 'acronym boundaries, number boundaries'
require_text docs/vstr-api-parity.md 'knifer_go_vstr_golden_fixtures_cover_unicode_escape'
require_text docs/vstr-api-parity.md 'knifer_go_vstr_golden_fixtures_cover_ant_path_matching'
require_text docs/vstr-api-parity.md 'knifer_go_vstr_golden_fixtures_cover_similarity'
require_text src/vstr/tests/property.rs 'property_style_substring_helpers_keep_scalar_boundaries'
require_text src/vstr/tests/property.rs 'property_style_replacement_and_escaping_helpers_are_stable'
require_text src/vstr/tests/property.rs 'property_style_reusable_matcher_preserves_match_contracts'
require_text src/vstr/tests/matcher.rs 'reusable_matcher_backend_parity_matrix_locks_public_semantics'
require_text src/vstr/tests/property.rs 'property_style_ant_path_literal_patterns_match_themselves'
require_text src/vstr/tests/property.rs 'property_style_unicode_width_helpers_respect_display_boundaries'
require_text src/vstr/tests/case.rs 'knifer_go_vstr_golden_fixtures_cover_case_conversion'
require_text src/vstr/tests/encoding.rs 'knifer_go_vstr_golden_fixtures_cover_unicode_escape'
require_text src/vstr/tests/path.rs 'knifer_go_vstr_golden_fixtures_cover_ant_path_matching'
require_text src/vstr/tests/similarity.rs 'knifer_go_vstr_golden_fixtures_cover_similarity'
require_text src/vstr/tests/case.rs 'case_conversion_cross_crate_fixtures_lock_acronym_number_separator_unicode'
require_text src/vstr/tests/case.rs 'struct CaseFixture'
require_text src/vstr/tests/case.rs 'numeric acronym suffix'
require_text src/vstr/tests/case.rs 'repeated separators'
require_text src/vstr/tests/case.rs 'unicode lowercase expansion'
require_text src/vstr/tests/case.rs 'cjk prefix'
require_text bench/vstr_bench.rs 'ReportFormat'
require_text bench/vstr_bench.rs 'REPORT_SCHEMA'
require_text bench/vstr_bench.rs 'BenchEnvironment'
require_text bench/vstr_bench.rs 'bench_environment'
require_text bench/vstr_bench.rs 'active_feature_set'
require_text bench/vstr_bench.rs 'print_json'
require_text bench/vstr_bench.rs 'print_markdown'
require_text bench/vstr_bench.rs 'CompareFormat'
require_text bench/vstr_bench.rs 'print_compare_json'
require_text bench/vstr_bench.rs 'print_compare_markdown'
require_text bench/vstr_bench.rs 'parse_percent_to_bps'
require_text bench/vstr_bench.rs 'arg != "--bench"'
require_text bench/vstr_bench.rs 'serde_json'
require_text bin/check-vstr-bench.sh '-- --json'
require_text bin/check-vstr-bench.sh '-- --markdown'
require_text bin/check-vstr-bench.sh '--compare-json'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_BASE_REF'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_MAX_REGRESSION_PCT'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_RUSTC_VERSION'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_TARGET_TRIPLE'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_COMMIT_SHA'
require_text bin/check-vstr-bench.sh 'VSTR_BENCH_RUN_MODE'
require_text bin/check-vstr-bench.sh 'baseline_source'
require_text bin/check-vstr-bench.sh 'max_regression_percent'
require_text bin/check-vstr-bench.sh 'rustc -vV'
require_text bin/check-vstr-bench.sh 'git rev-parse --short=12 HEAD'
require_text bin/check-vstr-bench.sh 'report_dir'
require_text bin/check-vstr-bench.sh 'vstr-bench.md'
require_text bin/check-vstr-bench.sh 'vstr-bench-compare.md'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_substring'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_escaping'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_path_matching'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_replacement'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_matcher'
require_text bin/check-vstr-fuzz-smoke.sh 'fuzz_text_boundaries'
require_text bin/check-vstr-fuzz.sh 'cargo-fuzz is not installed'
require_text bin/check-vstr-fuzz.sh 'cargo install cargo-fuzz --locked'
require_text bin/check-vstr-fuzz.sh 'VSTR_FUZZ_RUN_SECS'
require_text bin/check-vstr-fuzz.sh 'cargo fuzz run "$target"'
require_text bin/check-docs-rs-ready.sh 'RUSTDOCFLAGS="-Dwarnings --cfg docsrs" cargo doc --locked --all-features --no-deps'
require_text bin/check-docs-rs-ready.sh "require_text README.md '## Status'"
require_text bin/check-docs-rs-ready.sh "require_text README.md '## Quick Start'"
require_text bin/check-docs-rs-ready.sh "require_text README.md '## Feature Flags'"
require_text bin/check-docs-rs-ready.sh 'cargo package --locked --allow-dirty'
require_text bin/check-package-contents.sh 'cargo package --locked --allow-dirty --list'
require_text bin/check-package-contents.sh 'require_entry README.md'
require_text bin/check-package-contents.sh 'require_entry LICENSE'
require_text bin/check-package-contents.sh 'require_entry aiflow.yaml'
require_text bin/check-package-contents.sh 'require_entry bin/check-release-gate-layers.sh'
require_text bin/check-package-contents.sh 'require_entry bench/vstr_bench.rs'
require_text bin/check-package-contents.sh "find src -name '*.rs'"
require_text bin/check-package-contents.sh 'require_entry "$file"'
require_text bin/check-package-contents.sh 'forbid_prefix .aiflow'
require_text bin/check-package-contents.sh 'forbid_prefix target'
require_text bin/check-package-contents.sh 'forbid_prefix fuzz/target'
require_text bin/check-package-contents.sh 'fuzz/ is local release evidence'
require_text bin/check-examples.sh 'cargo run --locked --example vstr_daily'
require_text bin/check-examples.sh 'cargo run --locked --example vbytes_encoding'
require_text bin/check-examples.sh 'cargo run --locked --example vstr_matcher'
require_text bin/check-examples.sh 'cargo run --locked --all-features --example vstr_unicode'
require_text bin/check-examples.sh 'cargo test --locked --all-features --examples'
require_text bin/check-examples.sh 'example not executed by any gate'
require_text bin/check-examples.sh 'delegated_examples=('
require_text bin/check-examples.sh "find examples -maxdepth 1 -name '*.rs'"
require_text bench/vstr_bench.rs 'bench_find_all'
require_text bench/vstr_bench.rs 'bench_levenshtein'
require_text examples/vstr_benchmark_smoke.rs 'replace_many'
require_text examples/vstr_benchmark_smoke.rs 'levenshtein_distance'

# The benchmark name list is duplicated across the bench suite, the smoke
# example, and both benchmark gates. bench/vstr_bench.rs is the source of
# truth; the other three must list exactly the same names or a new benchmark
# could escape smoke coverage or the two suites could silently diverge.
bench_names_from_run_case() {
  grep -oE 'run_case\("[a-z_]+"' "$1" | sed -E 's/run_case\("([a-z_]+)"/\1/' | sort -u
}

bench_names_from_for_loop() {
  sed -n '/for name in/,/^do$/p' "$1" |
    grep -vE 'for name in|^do$' |
    tr -d '\\ ' |
    grep -E '^[a-z_]+$' |
    sort -u
}

bench_source_names="$(bench_names_from_run_case bench/vstr_bench.rs)"
if [[ -z "$bench_source_names" ]]; then
  echo "no benchmark names found in bench/vstr_bench.rs" >&2
  exit 1
fi

require_bench_names_match() {
  local label="$1"
  local actual="$2"

  if [[ "$actual" != "$bench_source_names" ]]; then
    echo "benchmark names in $label are not aligned with bench/vstr_bench.rs" >&2
    diff <(printf '%s\n' "$bench_source_names") <(printf '%s\n' "$actual") >&2 || true
    exit 1
  fi
}

require_bench_names_match \
  examples/vstr_benchmark_smoke.rs \
  "$(bench_names_from_run_case examples/vstr_benchmark_smoke.rs)"
require_bench_names_match \
  bin/check-vstr-bench.sh \
  "$(bench_names_from_for_loop bin/check-vstr-bench.sh)"
require_bench_names_match \
  bin/check-vstr-benchmark-smoke.sh \
  "$(bench_names_from_for_loop bin/check-vstr-benchmark-smoke.sh)"

# The fuzz-target names are duplicated in three places: the [[bin]] entries in
# fuzz/Cargo.toml (which cargo actually builds) and the target lists in both
# fuzz gates. fuzz/Cargo.toml is the source of truth; the smoke gate and the
# optional long-run gate must list exactly the same targets or a new fuzz
# target could escape smoke coverage or the long-run gate.
fuzz_names_from_manifest() {
  grep -oE 'name = "(fuzz_[a-z_]+)"' "$1" |
    sed -E 's/name = "(fuzz_[a-z_]+)"/\1/' |
    sort -u
}

fuzz_names_from_list() {
  sed -n "$2" "$1" |
    tr -d '\\ ' |
    grep -E '^fuzz_[a-z_]+$' |
    sort -u
}

fuzz_source_names="$(fuzz_names_from_manifest fuzz/Cargo.toml)"
if [[ -z "$fuzz_source_names" ]]; then
  echo "no fuzz targets found in fuzz/Cargo.toml" >&2
  exit 1
fi

require_fuzz_names_match() {
  local label="$1"
  local actual="$2"

  if [[ "$actual" != "$fuzz_source_names" ]]; then
    echo "fuzz targets in $label are not aligned with fuzz/Cargo.toml" >&2
    diff <(printf '%s\n' "$fuzz_source_names") <(printf '%s\n' "$actual") >&2 || true
    exit 1
  fi
}

require_fuzz_names_match \
  bin/check-vstr-fuzz-smoke.sh \
  "$(fuzz_names_from_list bin/check-vstr-fuzz-smoke.sh '/for target in/,/^do$/p')"
require_fuzz_names_match \
  bin/check-vstr-fuzz.sh \
  "$(fuzz_names_from_list bin/check-vstr-fuzz.sh '/targets=(/,/^)/p')"

# Every test file under src/vstr/tests/ must be declared as a mod in
# src/vstr/tests.rs, or its tests silently never compile or run. Compare the
# module declarations against the files on disk.
declared_test_mods="$(grep -oE 'mod [a-z_]+;' src/vstr/tests.rs | sed -E 's/mod ([a-z_]+);/\1/' | sort -u)"
disk_test_mods="$(find src/vstr/tests -maxdepth 1 -name '*.rs' | sed -E 's#.*/##; s/\.rs$//' | sort -u)"
if [[ "$declared_test_mods" != "$disk_test_mods" ]]; then
  echo "src/vstr/tests.rs module declarations are not aligned with test files on disk" >&2
  diff <(printf '%s\n' "$declared_test_mods") <(printf '%s\n' "$disk_test_mods") >&2 || true
  echo "add a 'mod <name>;' line to src/vstr/tests.rs for each test file" >&2
  exit 1
fi

# Every optional feature declared in Cargo.toml must be documented across the
# feature-facing surfaces and reflected in the benchmark feature-set reporter.
# The literal require_text lines above pin each known feature, but only this
# derived check guarantees a new feature cannot be added without documenting it.
cargo_features="$(
  sed -n '/^\[features\]/,/^\[/p' Cargo.toml |
    grep -E '^[a-z]' |
    grep -vE '^default' |
    sed -E 's/ *=.*//' |
    sort -u
)"
if [[ -z "$cargo_features" ]]; then
  echo "no optional features found in Cargo.toml [features]" >&2
  exit 1
fi

while IFS= read -r feature; do
  [[ -z "$feature" ]] && continue

  if ! grep -Fq "| \`$feature\` |" README.md; then
    echo "feature '$feature' is missing from the README feature-flags table" >&2
    exit 1
  fi
  if ! grep -Fq "$feature" CHANGELOG.md; then
    echo "feature '$feature' is missing from CHANGELOG.md" >&2
    exit 1
  fi
  if ! grep -Fq "$feature" docs/dependency-policy.md; then
    echo "feature '$feature' is missing from docs/dependency-policy.md" >&2
    exit 1
  fi
  if ! grep -Fq "$feature" docs/api-behavior-contract.md; then
    echo "feature '$feature' is missing from docs/api-behavior-contract.md" >&2
    exit 1
  fi
  if ! grep -Fq "cfg!(feature = \"$feature\")" bench/vstr_bench.rs; then
    echo "feature '$feature' is missing from the benchmark feature-set reporter in bench/vstr_bench.rs" >&2
    exit 1
  fi
done <<<"$cargo_features"

# Every gate script must be executed by a runner surface: aiflow.yaml, the CI
# workflow, or bin/check-release-ready.sh (which chains gates). Otherwise a new
# bin/check-*.sh could sit orphaned and never run. Exceptions must be explicit.
gate_runner_surfaces=(
  aiflow.yaml
  .github/workflows/ci.yml
  bin/check-release-ready.sh
)
# Gates intentionally not wired into a daily runner. Each needs a reason.
exempt_gates=(
  # Opt-in long-running fuzzing; documented in README/fuzz and requires
  # cargo-fuzz. bin/check-vstr-fuzz-smoke.sh provides the daily coverage.
  bin/check-vstr-fuzz.sh
  # It is a runner itself, executed by aiflow/CI directly rather than chained.
  bin/check-release-ready.sh
)

is_exempt_gate() {
  local candidate="$1"
  local gate
  for gate in "${exempt_gates[@]}"; do
    [[ "$gate" == "$candidate" ]] && return 0
  done
  return 1
}

for gate in bin/check-*.sh; do
  is_exempt_gate "$gate" && continue
  name="$(basename "$gate")"
  run=""
  for surface in "${gate_runner_surfaces[@]}"; do
    if grep -qE "bash bin/$name( |\$)" "$surface"; then
      run="$surface"
      break
    fi
  done
  if [[ -z "$run" ]]; then
    echo "gate not executed by any runner: $gate" >&2
    echo "wire it into aiflow.yaml, .github/workflows/ci.yml, or bin/check-release-ready.sh, or add it to exempt_gates in $0" >&2
    exit 1
  fi
done

require_text examples/vstr_daily.rs 'vstr::between'
require_text examples/vstr_daily.rs 'vstr::split_once_last'
require_text examples/vstr_daily.rs 'vstr::to_train_case'
require_text examples/vstr_daily.rs 'vstr::to_dot_case'
require_text examples/vstr_daily.rs 'vstr::capitalize'
require_text examples/vstr_daily.rs 'vstr::remove_whitespace'
require_text examples/vstr_daily.rs 'vstr::normalize_newlines'
require_text examples/vstr_daily.rs 'vstr::trim_lines'
require_text examples/vstr_daily.rs 'vstr::trim_blank_lines'
require_text examples/vstr_daily.rs 'vstr::take_chars'
require_text examples/vstr_daily.rs 'vstr::truncate_with_suffix'
require_text examples/vstr_daily.rs 'vstr::abbreviate_middle'
require_text examples/vstr_daily.rs 'vstr::mask'
require_text examples/vstr_daily.rs 'vstr::collapse_repeated_char'
require_text examples/vstr_daily.rs 'vstr::center'
require_text examples/vstr_daily.rs 'vstr::wrap'
require_text examples/vstr_daily.rs 'vstr::wrap_with_indent'
require_text examples/vstr_daily.rs 'vstr::WrapOptions'
require_text examples/vstr_daily.rs 'vstr::WhitespaceMode::Preserve'
require_text examples/vstr_daily.rs 'vstr::LongWordPolicy::Preserve'
require_text examples/vstr_daily.rs 'vstr::wrap_with_options'
require_text examples/vstr_daily.rs 'vstr::non_blank_lines'
require_text examples/vstr_daily.rs 'vstr::initials'
require_text examples/vstr_daily.rs 'vstr::is_palindrome'
require_text examples/vstr_daily.rs 'vstr::extract_digits'
require_text examples/vstr_unicode.rs 'vstr::unicode_words'
require_text examples/vstr_unicode.rs 'vstr::unicode_sentences'
require_text examples/vstr_unicode.rs 'vstr::display_width'
require_text examples/vstr_unicode.rs 'vstr::take_width'
require_text examples/vstr_unicode.rs 'vstr::truncate_width'
require_text examples/vstr_unicode.rs 'vstr::wrap_width'
require_text examples/vstr_unicode.rs 'vstr::wrap_width_with_options'
require_text examples/vbytes_encoding.rs 'vbytes::byte_len'
require_text examples/vbytes_encoding.rs 'vbytes::is_utf8'
require_text examples/vbytes_encoding.rs 'vbytes::sub'
require_text examples/vbytes_encoding.rs 'vbytes::find_all'
require_text examples/vbytes_encoding.rs 'vbytes::replace_all'
require_text examples/vbytes_encoding.rs 'vencoding::detect_bom'
require_text examples/vbytes_encoding.rs 'vencoding::strip_bom'
require_text examples/vbytes_encoding.rs 'vencoding::validate_utf8_without_bom'
require_text examples/vbytes_encoding.rs 'vencoding::decode_utf8_lossy_without_bom'
require_text examples/vstr_matcher.rs 'vstr::find_any'
require_text examples/vstr_matcher.rs 'vstr::find_all'
require_text examples/vstr_matcher.rs 'vstr::find_all_ignore_case'
require_text examples/vstr_matcher.rs 'vstr::strip_suffix_ignore_case'
require_text examples/vstr_matcher.rs 'vstr::count_matches'
require_text examples/vstr_matcher.rs 'vstr::replace_ignore_case'
require_text examples/vstr_matcher.rs 'vstr::replace_many'
require_text examples/vstr_matcher.rs 'vstr::escape_regex'
require_text examples/vstr_matcher.rs 'vstr::quote_meta'
require_text bin/check-api-semver.sh 'breaking API changes detected'
require_text bin/check-api-semver.sh 'additive APIs not in inventory'
require_text bin/check-api-semver.sh '--print-signatures'
require_text bin/check-api-semver.sh 'comm -23'
require_text bin/check-api-semver.sh 'comm -13'
require_text bin/check-release-api-semver.sh 'cargo semver-checks --version'
require_text bin/check-release-api-semver.sh 'API_SEMVER_BASELINE_REF'
require_text bin/check-release-api-semver.sh 'API_SEMVER_BASELINE_ROOT'
require_text bin/check-release-api-semver.sh 'API_SEMVER_BASELINE_RUSTDOC'
require_text bin/check-release-api-semver.sh 'API_SEMVER_REQUIRED'
require_text bin/check-release-api-semver.sh 'check-release'
require_text bin/check-release-gate-layers.sh 'extract_aiflow_profile'
require_text bin/check-release-gate-layers.sh 'extract_release_ready_commands'
require_text bin/check-release-gate-layers.sh 'extract_ci_stable_run_commands'
require_text bin/check-release-gate-layers.sh 'extract_ci_all_run_commands'
require_text bin/check-release-gate-layers.sh 'assert_ci_runs_layer vet'
require_text bin/check-release-gate-layers.sh 'assert_ci_runs_layer publish-readiness'
require_text bin/check-release-gate-layers.sh 'assert_ci_runs_layer release-evidence'
require_text bin/check-release-gate-layers.sh 'executable_profiles'
require_text bin/check-release-gate-layers.sh 'assert_profiles_within_allowlist'
require_text bin/check-release-gate-layers.sh 'ci_wrapper_covered'
require_text bin/check-release-gate-layers.sh 'write_expected_release_detail'
require_text bin/check-release-gate-layers.sh 'release gate layer check passed'
require_text bin/check-release-ready.sh '== fast vet gates =='
require_text bin/check-release-ready.sh '== publish readiness gates =='
require_text bin/check-release-ready.sh '== release evidence smoke gates =='
require_text bin/check-release-ready.sh 'cargo fmt --check'
require_text bin/check-release-ready.sh 'cargo test --locked --no-default-features'
require_text bin/check-release-ready.sh 'cargo test --locked --all-features'
require_text bin/check-release-ready.sh 'cargo clippy --all-targets --all-features -- -D warnings'
require_text bin/check-release-ready.sh 'bash bin/check-docs-rs-ready.sh'
require_text bin/check-release-ready.sh 'bash bin/check-package-contents.sh'
require_text bin/check-release-ready.sh 'bash bin/check-project-contract.sh'
require_text bin/check-release-ready.sh 'bash bin/check-public-api-inventory.sh'
require_text bin/check-release-ready.sh 'bash bin/check-api-semver.sh'
require_text bin/check-release-ready.sh 'bash bin/check-release-gate-layers.sh'
require_text bin/check-release-ready.sh 'bash bin/check-release-api-semver.sh'
require_text bin/check-release-ready.sh 'bash bin/check-vstr-benchmark-smoke.sh'
require_text bin/check-release-ready.sh 'bash bin/check-vstr-fuzz-smoke.sh'
require_text src/vstr/mod.rs 'Module navigation:'
require_text src/vstr/mod.rs 'split `text/wrap` file family'
require_text src/vstr/mod.rs 'split `width/wrap` file'
require_text src/vstr/basic/mod.rs 'Core scalar-based string helpers.'
require_text src/vstr/text.rs 'Higher-level scalar text cleanup and layout helpers.'
require_text src/vstr/text.rs 'callers keep importing through `knifer_rs::vstr`'
require_text src/vstr/text/wrap.rs 'Scalar wrapping implementation family.'
require_text src/vstr/text/wrap.rs '`tokens` stays private'
require_text src/vstr/matcher.rs 'Reusable literal multi-pattern matching.'
require_text src/vstr/width.rs 'Optional terminal display-width helpers.'
require_text src/vstr/width.rs 'callers keep importing through `knifer_rs::vstr`'
require_text src/vstr/width/wrap.rs 'Display-width wrapping implementation family.'
require_text src/vstr/width/wrap.rs '`tokens` stays private'
require_text fuzz/Cargo.toml 'name = "knifer-rs-fuzz"'
require_text fuzz/Cargo.toml 'publish = false'
require_text fuzz/Cargo.toml 'cargo-fuzz = true'
require_text fuzz/Cargo.toml 'knifer_rs = { package = "knifer-rs", path = ".." }'
require_text fuzz/Cargo.toml 'libfuzzer-sys = "0.4"'
require_text fuzz/Cargo.toml 'cfg(fuzzing)'
require_text fuzz/README.md 'fuzz_substring'
require_text fuzz/README.md 'fuzz_matcher'
require_text fuzz/README.md 'fuzz_text_boundaries'
require_text fuzz/README.md 'bash bin/check-vstr-fuzz-smoke.sh'
require_text fuzz/README.md 'cargo fuzz run fuzz_substring'
require_text fuzz/README.md 'cargo fuzz run fuzz_escaping'
require_text fuzz/README.md 'cargo fuzz run fuzz_matcher'
require_text fuzz/README.md 'bash bin/check-vstr-fuzz.sh'
require_text fuzz/README.md 'Do not commit generated'
require_text fuzz/README.md 'fuzz/PLAN.md'
require_text fuzz/README.md 'fuzz/corpus/'
require_text fuzz/README.md 'VSTR_FUZZ_RUN_SECS=600'
require_text fuzz/README.md 'Crash handling protocol'
require_text fuzz/README.md 'Reproduce the crash'
require_text fuzz/README.md 'Keep generated crash files local'
require_text fuzz/PLAN.md 'Layer 1: Deterministic Smoke'
require_text fuzz/PLAN.md 'Layer 2: Checked-In Corpus Seeds'
require_text fuzz/PLAN.md 'Layer 3: Optional Engine Fuzzing'
require_text fuzz/PLAN.md 'Promotion Rules'
require_text fuzz/PLAN.md 'default CI'
require_text fuzz/PLAN.md 'cargo-fuzz'
require_text fuzz/PLAN.md 'VSTR_FUZZ_RUN_SECS'
require_text fuzz/PLAN.md 'include_str!'
require_text fuzz/corpus/substring.txt '你好Rust'
require_text fuzz/corpus/escaping.txt '\uD83D\uDE80'
require_text fuzz/corpus/path_matching.txt '/api/v1/users'
require_text fuzz/corpus/replacement.txt 'Case CASE case'
require_text fuzz/corpus/matcher.txt 'emoji 🚀🚀'
require_text fuzz/corpus/text_boundaries.txt 'supercalifragilistic'
require_text fuzz/fuzz_targets/substring.rs 'take_chars'
require_text fuzz/fuzz_targets/substring.rs 'include_str!("../corpus/substring.txt")'
require_text fuzz/fuzz_targets/substring.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/substring.rs 'drop_chars'
require_text fuzz/fuzz_targets/substring.rs 'sub(input'
require_text fuzz/fuzz_targets/escaping.rs 'escape_regex'
require_text fuzz/fuzz_targets/escaping.rs 'include_str!("../corpus/escaping.txt")'
require_text fuzz/fuzz_targets/escaping.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/escaping.rs 'escape_unicode'
require_text fuzz/fuzz_targets/path_matching.rs 'ant_path_match'
require_text fuzz/fuzz_targets/path_matching.rs 'include_str!("../corpus/path_matching.txt")'
require_text fuzz/fuzz_targets/path_matching.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/replacement.rs 'replace_many'
require_text fuzz/fuzz_targets/replacement.rs 'include_str!("../corpus/replacement.txt")'
require_text fuzz/fuzz_targets/replacement.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/matcher.rs 'VStrMatcher'
require_text fuzz/fuzz_targets/matcher.rs 'find_overlapping'
require_text fuzz/fuzz_targets/matcher.rs 'include_str!("../corpus/matcher.txt")'
require_text fuzz/fuzz_targets/matcher.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/text_boundaries.rs 'truncate_with_suffix'
require_text fuzz/fuzz_targets/text_boundaries.rs 'abbreviate_middle'
require_text fuzz/fuzz_targets/text_boundaries.rs 'wrap_with_indent'
require_text fuzz/fuzz_targets/text_boundaries.rs 'include_str!("../corpus/text_boundaries.txt")'
require_text fuzz/fuzz_targets/text_boundaries.rs 'fuzz_target!'
require_text fuzz/fuzz_targets/text_boundaries.rs 'mask'
require_text src/vstr/tests/unicode.rs 'unicode_segmentation_conformance_fixtures_cover_curated_uax29_subset'
require_text src/vstr/tests/unicode.rs 'emoji modifier'
require_text src/vstr/tests/unicode.rs 'hangul jamo'
require_text src/vstr/tests/unicode.rs 'terminal punctuation'
require_text src/vstr/tests/pattern.rs 'pattern_regex_golden_cases_cover_unicode_empty_and_multibyte_ranges'
require_text src/vstr/tests/pattern.rs 'find_all_patterns("a你好b世界", r"\p{Han}")'
require_text src/vstr/tests/pattern.rs 'replace_pattern("ab", r"", "|")'
require_text src/vstr/tests/pattern.rs 'NotAClass'

if grep -R --include='*.rs' -n '\bunsafe\b' src; then
  echo "unsafe Rust is not allowed in src/" >&2
  exit 1
fi
