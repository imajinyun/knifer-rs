//! String and text utilities.
//!
//! Default `vstr` helpers are scalar-based and part of the core stable facade.
//! Regex, Unicode segmentation, and display-width helpers are optional feature
//! facades. [`VStrMatcher`] is public and behavior-tested, while matcher backend
//! internals can evolve behind the same public semantics.

mod basic;
mod case;
mod classify;
mod emoji;
mod encoding;
#[cfg(feature = "unicode-segmentation")]
mod grapheme;
mod matcher;
mod path;
#[cfg(feature = "pattern-regex")]
mod pattern;
mod similarity;
mod text;
#[cfg(feature = "unicode-width")]
mod width;

pub use basic::*;
pub use case::*;
pub use classify::*;
pub use emoji::*;
pub use encoding::*;
#[cfg(feature = "unicode-segmentation")]
pub use grapheme::*;
pub use matcher::*;
pub use path::*;
#[cfg(feature = "pattern-regex")]
pub use pattern::*;
pub use similarity::*;
pub use text::*;
#[cfg(feature = "unicode-width")]
pub use width::*;

#[cfg(test)]
mod tests;
