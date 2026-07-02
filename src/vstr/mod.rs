//! String and text utilities.
//!
//! Default `vstr` helpers are scalar-based and part of the core stable facade.
//! Regex, Unicode segmentation, and display-width helpers are optional feature
//! facades. [`VStrMatcher`] is public and behavior-tested, while matcher backend
//! internals can evolve behind the same public semantics.
//!
//! Module navigation:
//!
//! - `basic` contains everyday scalar-based helpers such as trimming,
//!   substring, searching, replacement, and literal escaping.
//! - `humanize` contains locale-neutral count, size, and duration formatting.
//! - `inflection` contains English word and identifier inflection helpers
//!   (`pluralize`/`singularize`, `ordinalize`/`deordinalize`, `humanize`,
//!   `titleize`, `camelize`).
//! - `text` contains higher-level text cleanup, wrapping, truncation,
//!   masking, inspection helpers, and a split `text/wrap` file family for
//!   scalar layout policy.
//! - `matcher` contains the reusable multi-pattern literal matcher facade and
//!   optional backend adapter.
//! - `normalize` contains optional Unicode normalization form helpers (NFC,
//!   NFD, NFKC, NFKD) behind the `unicode-normalization` feature.
//! - `width` contains optional display-cell measurement and wrapping helpers
//!   behind the `unicode-width` feature, including a split `width/wrap` file
//!   family for display-cell layout policy.

mod basic;
mod case;
mod classify;
mod emoji;
mod encoding;
#[cfg(feature = "unicode-segmentation")]
mod grapheme;
mod humanize;
mod inflection;
mod matcher;
#[cfg(feature = "unicode-normalization")]
mod normalize;
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
pub use humanize::*;
pub use inflection::*;
pub use matcher::*;
#[cfg(feature = "unicode-normalization")]
pub use normalize::*;
pub use path::*;
#[cfg(feature = "pattern-regex")]
pub use pattern::*;
pub use similarity::*;
pub use text::*;
#[cfg(feature = "unicode-width")]
pub use width::*;

#[cfg(test)]
mod tests;
