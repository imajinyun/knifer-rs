//! String and text utilities.

mod basic;
mod case;
mod classify;
mod emoji;
mod encoding;
mod path;
mod similarity;

pub use basic::*;
pub use case::*;
pub use classify::*;
pub use emoji::*;
pub use encoding::*;
pub use path::*;
pub use similarity::*;

#[cfg(test)]
mod tests;
