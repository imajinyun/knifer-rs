//! Optional terminal display-width helpers.
//!
//! Enable the `unicode-width` feature to measure, truncate, and wrap strings by
//! display cells instead of Unicode scalar counts.

mod measure;
mod wrap;

pub use measure::*;
pub use wrap::*;
