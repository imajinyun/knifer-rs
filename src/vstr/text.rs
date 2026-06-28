//! Higher-level scalar text cleanup and layout helpers.
//!
//! This module groups content truncation, line/word inspection, normalization,
//! text transformation, and scalar-width wrapping. Display-cell layout helpers
//! live behind the `unicode-width` feature.

mod content;
mod inspect;
mod normalize;
mod transform;
mod wrap;

pub use content::*;
pub use inspect::*;
pub use normalize::*;
pub use transform::*;
pub use wrap::*;
