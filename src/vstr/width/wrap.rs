//! Display-width wrapping implementation family.
//!
//! Public entry points are re-exported through `knifer_rs::vstr` when the
//! `unicode-width` feature is enabled. Keep this module as a thin index:
//! `basic` owns simple display-cell wrapping, `options_wrap` owns strategy
//! rendering, and `tokens` stays private.

mod basic;
mod options_wrap;
mod tokens;

pub use basic::*;
pub use options_wrap::*;
