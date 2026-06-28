//! `knifer-rs` is a Safe Rust utility toolkit for everyday business development.
//!
//! Public APIs are grouped into focused `v*` modules. Start with [`vstr`] for
//! string helpers, [`vbytes`] for byte-slice helpers, and [`vencoding`] for
//! UTF-8/BOM helpers. The default build keeps the core stable facade at zero
//! runtime dependencies; optional facades are enabled explicitly through Cargo
//! features and tracked in `docs/public-api-inventory.md`.

pub mod vbytes;
pub mod vencoding;
pub mod vstr;
