//! Reusable literal multi-pattern matching.
//!
//! [`VStrMatcher`] owns the public search, overlap, and replacement semantics.
//! The optional `matcher-aho-corasick` backend is kept behind the same facade so
//! callers see identical behavior in default and all-features builds.

#[cfg(feature = "matcher-aho-corasick")]
mod backend;
mod search;
mod types;

pub use types::{MatchKind, VStrMatch, VStrMatcher};

use types::Needle;
