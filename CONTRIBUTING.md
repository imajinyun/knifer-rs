# Contributing

`knifer-rs` follows a small, facade-oriented utility-library design. Public
APIs live under focused `v*` modules such as `vstr`; implementation details stay
inside that module's directory.

## Development Checks

Run the same checks as CI before sending changes:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --document-private-items
bash bin/check-project-contract.sh
bash bin/check-public-api-inventory.sh
cargo package --list --allow-dirty
```

## API Rules

- Keep public APIs safe Rust.
- Preserve `knifer_rs::v*::*` facade ergonomics.
- Add doctest examples for public functions.
- Add unit tests for edge cases and Rust-specific behavior.
- Document intentional differences from `knifer-go` or other benchmark
  libraries.
- Avoid new dependencies unless they remove real complexity or improve
  correctness in a way the standard library cannot reasonably cover.

## Compatibility

The minimum supported Rust version is declared in `Cargo.toml` as
`rust-version`. CI must keep checking that version. Public API changes should be
reflected in `CHANGELOG.md` and, for `vstr`, in `docs/vstr-api-parity.md`.

## Commit Shape

Prefer focused changes:

- code and tests together
- docs updated with behavior changes
- CI or governance changes kept separate from feature code when practical

## Repository Hygiene

- Keep generated build output under `target/`; do not commit it.
- Keep local editor, environment, coverage, and benchmark artifacts ignored.
- Preserve LF line endings through `.editorconfig` and `.gitattributes`.
