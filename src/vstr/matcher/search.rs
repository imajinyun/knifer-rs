use super::{MatchKind, Needle, VStrMatch, VStrMatcher};

impl<'needle> VStrMatcher<'needle> {
    /// Creates a matcher with [`MatchKind::LeftmostFirst`] semantics.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::{VStrMatch, VStrMatcher};
    ///
    /// let matcher = VStrMatcher::new(["rust", "rs"]);
    /// assert_eq!(
    ///     matcher.find("hello rust"),
    ///     Some(VStrMatch {
    ///         needle: "rust",
    ///         pattern_index: 0,
    ///         start: 6,
    ///         end: 10,
    ///     })
    /// );
    /// ```
    #[must_use]
    pub fn new<I>(needles: I) -> Self
    where
        I: IntoIterator<Item = &'needle str>,
    {
        Self::with_kind(needles, MatchKind::LeftmostFirst)
    }

    /// Creates a matcher with explicit tie-break semantics.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::{MatchKind, VStrMatcher};
    ///
    /// let matcher = VStrMatcher::with_kind(["a", "aa"], MatchKind::LeftmostLongest);
    /// assert_eq!(matcher.find("aaaa").unwrap().needle, "aa");
    /// ```
    #[must_use]
    pub fn with_kind<I>(needles: I, kind: MatchKind) -> Self
    where
        I: IntoIterator<Item = &'needle str>,
    {
        let needles: Vec<Needle<'needle>> = needles
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| (!value.is_empty()).then_some(Needle { value, index }))
            .collect();

        #[cfg(feature = "matcher-aho-corasick")]
        let backend = super::backend::MatcherBackend::new(&needles, kind);

        Self {
            needles,
            kind,
            #[cfg(feature = "matcher-aho-corasick")]
            backend,
        }
    }

    /// Returns `true` when the matcher has no non-empty needles.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::VStrMatcher;
    ///
    /// assert!(VStrMatcher::new([""]).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.needles.is_empty()
    }

    /// Returns the match tie-break policy.
    #[must_use]
    pub const fn kind(&self) -> MatchKind {
        self.kind
    }

    /// Returns the number of non-empty needles in the matcher.
    #[must_use]
    pub fn len(&self) -> usize {
        self.needles.len()
    }

    /// Finds the first non-overlapping match according to the matcher's policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::VStrMatcher;
    ///
    /// let matcher = VStrMatcher::new(["go", "rust"]);
    /// assert_eq!(matcher.find("hello rust").unwrap().start, 6);
    /// ```
    #[must_use]
    pub fn find(&self, input: &str) -> Option<VStrMatch<'needle>> {
        self.find_at(input, 0)
    }

    /// Finds all non-overlapping matches from left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::VStrMatcher;
    ///
    /// let matcher = VStrMatcher::new(["aa", "a"]);
    /// assert_eq!(matcher.find_all("aaaa").len(), 2);
    /// ```
    #[must_use]
    pub fn find_all(&self, input: &str) -> Vec<VStrMatch<'needle>> {
        let mut found = Vec::new();
        let mut offset = 0;

        while offset < input.len() {
            let Some(selected) = self.find_at(input, offset) else {
                break;
            };
            offset = selected.end;
            found.push(selected);
        }

        found
    }

    /// Finds all matches, including overlapping matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::VStrMatcher;
    ///
    /// let matcher = VStrMatcher::new(["aa"]);
    /// assert_eq!(matcher.find_overlapping("aaaa").len(), 3);
    /// ```
    #[must_use]
    pub fn find_overlapping(&self, input: &str) -> Vec<VStrMatch<'needle>> {
        input
            .char_indices()
            .filter_map(|(start, _)| self.find_starting_at(input, start))
            .collect()
    }

    /// Replaces non-overlapping matches using registered replacement strings.
    ///
    /// Replacement indexes align with original needle registration indexes.
    /// Missing replacement indexes leave the matched needle unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use knifer_rs::vstr::VStrMatcher;
    ///
    /// let matcher = VStrMatcher::new(["hello", "world"]);
    /// assert_eq!(matcher.replace_all("hello rust world", ["hi", "team"]), "hi rust team");
    /// ```
    #[must_use]
    pub fn replace_all<'replacement, I>(&self, input: &str, replacements: I) -> String
    where
        I: IntoIterator<Item = &'replacement str>,
    {
        let replacements: Vec<&str> = replacements.into_iter().collect();
        if self.is_empty() {
            return input.to_owned();
        }

        let mut output = String::with_capacity(input.len());
        let mut offset = 0;

        while offset < input.len() {
            let Some(matched) = self.find_at(input, offset) else {
                output.push_str(&input[offset..]);
                return output;
            };

            output.push_str(&input[offset..matched.start]);
            output.push_str(
                replacements
                    .get(matched.pattern_index)
                    .copied()
                    .unwrap_or(matched.needle),
            );
            offset = matched.end;
        }

        output
    }

    fn find_at(&self, input: &str, offset: usize) -> Option<VStrMatch<'needle>> {
        if offset >= input.len() {
            return None;
        }

        #[cfg(feature = "matcher-aho-corasick")]
        if let Some(backend) = &self.backend {
            return backend.find_at(&self.needles, self.kind, input, offset);
        }

        let mut best = None;

        for needle in &self.needles {
            let Some(relative_start) = input[offset..].find(needle.value) else {
                continue;
            };
            let start = offset + relative_start;
            let matched = VStrMatch {
                needle: needle.value,
                pattern_index: needle.index,
                start,
                end: start + needle.value.len(),
            };

            best = Some(choose_match(best, matched, self.kind));
        }

        best
    }

    fn find_starting_at(&self, input: &str, start: usize) -> Option<VStrMatch<'needle>> {
        find_starting_at(&self.needles, self.kind, input, start)
    }
}

pub(super) fn choose_match<'needle>(
    current: Option<VStrMatch<'needle>>,
    candidate: VStrMatch<'needle>,
    kind: MatchKind,
) -> VStrMatch<'needle> {
    match current {
        None => candidate,
        Some(current) if candidate.start < current.start => candidate,
        Some(current) if candidate.start == current.start => {
            choose_same_start_match(Some(current), candidate, kind)
        }
        Some(current) => current,
    }
}

pub(super) fn choose_same_start_match<'needle>(
    current: Option<VStrMatch<'needle>>,
    candidate: VStrMatch<'needle>,
    kind: MatchKind,
) -> VStrMatch<'needle> {
    match (current, kind) {
        (None, _) => candidate,
        (Some(current), MatchKind::LeftmostLongest)
            if candidate.end - candidate.start > current.end - current.start =>
        {
            candidate
        }
        (Some(current), MatchKind::LeftmostFirst | MatchKind::LeftmostLongest) => current,
    }
}

pub(super) fn find_starting_at<'needle>(
    needles: &[Needle<'needle>],
    kind: MatchKind,
    input: &str,
    start: usize,
) -> Option<VStrMatch<'needle>> {
    let mut best = None;

    for needle in needles {
        if input[start..].starts_with(needle.value) {
            let matched = VStrMatch {
                needle: needle.value,
                pattern_index: needle.index,
                start,
                end: start + needle.value.len(),
            };
            best = Some(choose_same_start_match(best, matched, kind));
        }
    }

    best
}
