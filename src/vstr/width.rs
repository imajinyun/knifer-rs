//! Optional terminal display-width helpers.
//!
//! Enable the `unicode-width` feature to measure, truncate, and wrap strings by
//! display cells instead of Unicode scalar counts. The `wrap` child module is
//! split into public basic/options entry points and private token/render helpers
//! while callers keep importing through `kniferrs::vstr`.

mod measure;
mod wrap;

pub use measure::*;
pub use wrap::*;
