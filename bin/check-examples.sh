#!/usr/bin/env bash
set -euo pipefail

cargo run --locked --example vstr_daily
cargo run --locked --example vbytes_encoding
cargo run --locked --example vstr_matcher
cargo run --locked --all-features --example vstr_unicode
cargo test --locked --examples
cargo test --locked --all-features --examples
