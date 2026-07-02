//! Higher-level scalar text cleanup and layout helpers.
//!
//! This module groups content truncation, line/word inspection, normalization,
//! text transformation, and scalar-width wrapping. The `wrap` child module is
//! split into public basic/options entry points and private token/render helpers
//! while callers keep importing through `kniferrs::vstr`. Display-cell layout
//! helpers live behind the `unicode-width` feature.

mod content;
mod fold;
mod inspect;
mod normalize;
mod transform;
mod wrap;

pub use content::*;
pub use fold::*;
pub use inspect::*;
pub use normalize::*;
pub use transform::*;
pub use wrap::*;
