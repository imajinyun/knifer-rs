use super::search::find_starting_at;
use super::{MatchKind, Needle, VStrMatch};

#[cfg(feature = "matcher-aho-corasick")]
#[derive(Clone, Debug)]
pub(super) struct MatcherBackend {
    automaton: aho_corasick::AhoCorasick,
}

#[cfg(feature = "matcher-aho-corasick")]
impl MatcherBackend {
    pub(super) fn new(needles: &[Needle<'_>], kind: MatchKind) -> Option<Self> {
        if needles.is_empty() {
            return None;
        }

        aho_corasick::AhoCorasickBuilder::new()
            .match_kind(match kind {
                MatchKind::LeftmostFirst => aho_corasick::MatchKind::LeftmostFirst,
                MatchKind::LeftmostLongest => aho_corasick::MatchKind::LeftmostLongest,
            })
            .build(needles.iter().map(|needle| needle.value))
            .ok()
            .map(|automaton| Self { automaton })
    }

    pub(super) fn find_at<'needle>(
        &self,
        needles: &[Needle<'needle>],
        kind: MatchKind,
        input: &str,
        offset: usize,
    ) -> Option<VStrMatch<'needle>> {
        let start = self.automaton.find(&input[offset..])?.start() + offset;
        find_starting_at(needles, kind, input, start)
    }
}
