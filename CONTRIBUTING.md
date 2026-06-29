# Contributing

`knifer-rs` follows a small, facade-oriented utility-library design. Public
APIs live under focused `v*` modules such as `vstr`; implementation details stay
inside that module's directory.

## Development Checks

Run the same checks as CI before sending changes:

```bash
cargo fmt --check
cargo test --locked
cargo test --locked --no-default-features
cargo test --locked --all-features
cargo test --locked --examples
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-examples.sh
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
bash bin/check-api-semver.sh
bash bin/check-release-api-semver.sh
bash bin/check-package-contents.sh
bash bin/check-docs-rs-ready.sh
```

## API Rules

- Keep public APIs safe Rust.
- Preserve `knifer_rs::v*::*` facade ergonomics.
- Add doctest examples for public functions.
- Add unit tests for edge cases and Rust-specific behavior.
- Classify each new public API in `docs/api-behavior-contract.md` before
  refreshing the public API inventory.
- Document intentional differences from `knifer-go` or other benchmark
  libraries.
- Avoid new dependencies unless they remove real complexity or improve
  correctness in a way the standard library cannot reasonably cover.
- Keep the default crate zero-runtime-dependency; any new dependency must follow
  `docs/dependency-policy.md`.

## Compatibility

The minimum supported Rust version is declared in `Cargo.toml` as
`rust-version`. CI must keep checking that version. Public API changes should be
reflected in `CHANGELOG.md` and, for `vstr`, in `docs/vstr-api-parity.md`.
Behavior evidence requirements live in `docs/api-behavior-contract.md`. Run
`bin/check-api-semver.sh` before refreshing `docs/public-api-inventory.md`;
it reports removed or changed signatures as breaking and new signatures as
additive API work. Before a release, run `bin/check-release-api-semver.sh` with
`API_SEMVER_BASELINE_REF`, `API_SEMVER_BASELINE_ROOT`, or
`API_SEMVER_BASELINE_RUSTDOC` so `cargo-semver-checks` compares against a real
published or tagged baseline.

## Release Checklist

Before publishing a 0.1.x release, run the release gate:

```bash
bash bin/check-release-ready.sh
```

`aiflow.yaml` keeps `commands.release` as this single entry point and
`commands.release-detail` as the expanded command list used by the script.
The expanded release gate is grouped into three layers:

- fast vet gates: format, default/no-default/all-features tests, examples,
  clippy, rustdoc, project contract, public API inventory, and local semver
  checks.
- publish readiness gates: docs.rs readiness, package contents, and release API
  semver baseline checks.
- release evidence smoke gates: benchmark smoke and fuzz smoke.

Keep `commands.vet`, `commands.publish-readiness`,
`commands.release-evidence`, `commands.release`, `commands.release-detail`, and
`bin/check-release-ready.sh` aligned when adding or removing release checks. The
`bin/check-release-gate-layers.sh` guard verifies that `commands.release-detail`
is exactly `commands.vet` plus `commands.publish-readiness` plus
`commands.release-evidence`, and that `bin/check-release-ready.sh` runs the same
command sequence.

For package review, the release gate runs:

```bash
cargo package --locked --allow-dirty
```

Review the package output for unexpected warnings, confirm `CHANGELOG.md` explains the release boundary,
confirm `bin/check-package-contents.sh` keeps local runtime state out of the
publish archive,
and run `bin/check-release-api-semver.sh` with a real baseline once a previous
release or release tag exists.

## Commit Shape

Prefer focused changes:

- code and tests together
- docs updated with behavior changes
- CI or governance changes kept separate from feature code when practical

## Repository Hygiene

- Keep generated build output under `target/`; do not commit it.
- Keep local editor, environment, coverage, and benchmark artifacts ignored.
- Keep `aiflow.yaml` in the repository root. Use `.aiflow/` only for local
  generated run evidence, caches, and temporary state.
- Preserve LF line endings through `.editorconfig` and `.gitattributes`.
