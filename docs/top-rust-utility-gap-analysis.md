# Top Rust Helper Utility Gap Analysis

This project benchmarks itself against widely used Rust utility crates by
engineering discipline rather than by cloning their APIs.

## Reference Projects

- `dtolnay/anyhow`: small ergonomic facade, excellent docs, clear error story.
- `dtolnay/thiserror`: focused API, compile-time integration, stable examples.
- `serde-rs/json`: strict compatibility discipline and broad test coverage.
- `withoutboats/heck`: narrow case-conversion scope with predictable fixtures.
- `unicode-rs/unicode-segmentation`: standards-backed Unicode behavior.

## Current Strengths

- Safe Rust is enforced by `unsafe_code = "forbid"` and the project contract.
- The public facade is small and stable: `knifer_rs::vstr`.
- CI runs fmt, test, feature combinations, clippy, docs, project contract,
  public API inventory, semver-aware API checks, benchmark smoke, fuzz smoke,
  and docs.rs publish gate.
- Repository hygiene is explicit through `.gitignore`, `.editorconfig`, and
  `.gitattributes`.

## Current Gaps

1. Keep `docs/public-api-inventory.md` synchronized with public API signature snapshot checks and generated signature snapshot review.
2. Continue evolving from local semver-aware checks toward release-grade
   `cargo-semver-checks` against published versions.
3. Continue expanding benchmark history beyond fast smoke coverage.
4. Continue expanding fuzz/property tests for boundary-heavy helpers.
5. Keep dependency policy explicit before optional Unicode or regex features
   enter the crate.

## Local Gates

Run these before release-oriented changes:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
bash bin/check-api-semver.sh
bash bin/check-vstr-benchmark-smoke.sh
bash bin/check-vstr-bench.sh
bash bin/check-vstr-fuzz-smoke.sh
bash bin/check-docs-rs-ready.sh
```

`bash bin/check-docs-rs-ready.sh` is the docs.rs publish gate. Public API
changes should update the generated signature snapshot and the changelog.
