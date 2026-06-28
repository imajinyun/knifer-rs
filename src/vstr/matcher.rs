#[cfg(feature = "matcher-aho-corasick")]
mod backend;
mod search;
mod types;

pub use types::{MatchKind, VStrMatch, VStrMatcher};

use types::Needle;
