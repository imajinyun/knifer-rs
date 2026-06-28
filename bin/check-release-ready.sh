#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo test --locked
cargo test --locked --no-default-features
cargo test --locked --all-features
cargo test --locked --examples
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-examples.sh
bash bin/check-docs-rs-ready.sh
bash bin/check-package-contents.sh
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
bash bin/check-api-semver.sh
bash bin/check-release-api-semver.sh
bash bin/check-vstr-benchmark-smoke.sh
bash bin/check-vstr-fuzz-smoke.sh

echo "release readiness check passed"
