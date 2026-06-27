//! String and text utilities.

mod basic;
mod case;
mod classify;
mod emoji;
mod encoding;
mod path;
#[cfg(feature = "pattern-regex")]
mod pattern;
mod similarity;
mod text;

pub use basic::*;
pub use case::*;
pub use classify::*;
pub use emoji::*;
pub use encoding::*;
pub use path::*;
#[cfg(feature = "pattern-regex")]
pub use pattern::*;
pub use similarity::*;
pub use text::*;

#[cfg(test)]
mod tests;
