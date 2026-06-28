/// Tie-break policy for reusable literal multi-pattern matching.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Choose the first needle registered with the matcher when multiple
    /// needles start at the same byte index.
    LeftmostFirst,
    /// Choose the longest needle when multiple needles start at the same byte
    /// index. Registration order breaks equal-length ties.
    LeftmostLongest,
}

/// A single match produced by [`VStrMatcher`].
///
/// `start` and `end` are byte offsets into the searched input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VStrMatch<'needle> {
    /// The matched needle.
    pub needle: &'needle str,
    /// The registration index of the matched needle.
    pub pattern_index: usize,
    /// Inclusive byte start offset.
    pub start: usize,
    /// Exclusive byte end offset.
    pub end: usize,
}

/// Reusable literal multi-pattern matcher for `vstr`.
///
/// Empty needles are ignored. The default build uses straightforward literal
/// search. The optional `matcher-aho-corasick` feature may use an automaton
/// backend internally, but the public tie-break and replacement semantics stay
/// owned by this facade.
#[derive(Clone, Debug)]
pub struct VStrMatcher<'needle> {
    pub(super) needles: Vec<Needle<'needle>>,
    pub(super) kind: MatchKind,
    #[cfg(feature = "matcher-aho-corasick")]
    pub(super) backend: Option<super::backend::MatcherBackend>,
}

#[derive(Clone, Debug)]
pub(super) struct Needle<'needle> {
    pub(super) value: &'needle str,
    pub(super) index: usize,
}
