//! Scalar wrapping implementation family.
//!
//! Public entry points are re-exported through `kniferrs::vstr`. Keep this
//! module as a thin index: `basic` owns simple wrapping, `options` owns policy
//! types, `options_wrap` owns strategy rendering, and `tokens` stays private.

mod basic;
mod options;
mod options_wrap;
mod tokens;

pub use basic::*;
pub use options::*;
pub use options_wrap::*;
