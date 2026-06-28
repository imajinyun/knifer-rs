//! Core scalar-based string helpers.
//!
//! This module is split by behavior area while re-exporting a flat `vstr`
//! facade. Use these helpers for default-build operations that do not require
//! regex, grapheme, or display-width semantics.

mod affix;
mod casefold;
mod compare;
mod escape;
mod format;
mod measure;
mod predicate;
mod replace;
mod search;
mod split;
mod substring;
mod trim;
mod value;

pub use affix::*;
pub use compare::*;
pub use escape::*;
pub use format::*;
pub use measure::*;
pub use predicate::*;
pub use replace::*;
pub use search::*;
pub use split::*;
pub use substring::*;
pub use trim::*;
pub use value::*;
