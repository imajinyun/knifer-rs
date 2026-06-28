use crate::vstr::{MatchKind, VStrMatch};

#[cfg(feature = "unicode-width")]
use crate::vstr::{display_width, take_width, truncate_width, wrap_width, wrap_width_with_indent};

pub(super) fn assert_approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < f64::EPSILON);
}

pub(super) fn expected_matcher_find<'needle>(
    input: &str,
    needles: &[&'needle str],
    kind: MatchKind,
) -> Option<VStrMatch<'needle>> {
    expected_matcher_find_from(input, needles, kind, 0)
}

pub(super) fn expected_matcher_find_all<'needle>(
    input: &str,
    needles: &[&'needle str],
    kind: MatchKind,
) -> Vec<VStrMatch<'needle>> {
    let mut found = Vec::new();
    let mut offset = 0;

    while offset < input.len() {
        let Some(matched) = expected_matcher_find_from(input, needles, kind, offset) else {
            break;
        };
        offset = matched.end;
        found.push(matched);
    }

    found
}

pub(super) fn expected_matcher_find_overlapping<'needle>(
    input: &str,
    needles: &[&'needle str],
    kind: MatchKind,
) -> Vec<VStrMatch<'needle>> {
    input
        .char_indices()
        .filter_map(|(start, _)| expected_matcher_at(input, needles, kind, start))
        .collect()
}

pub(super) fn expected_matcher_replace_all(
    input: &str,
    needles: &[&str],
    kind: MatchKind,
    replacements: &[&str],
) -> String {
    let mut output = String::new();
    let mut offset = 0;

    for matched in expected_matcher_find_all(input, needles, kind) {
        output.push_str(&input[offset..matched.start]);
        output.push_str(
            replacements
                .get(matched.pattern_index)
                .copied()
                .unwrap_or(matched.needle),
        );
        offset = matched.end;
    }

    output.push_str(&input[offset..]);
    output
}

pub(super) fn expected_matcher_find_from<'needle>(
    input: &str,
    needles: &[&'needle str],
    kind: MatchKind,
    offset: usize,
) -> Option<VStrMatch<'needle>> {
    if offset >= input.len() {
        return None;
    }

    input[offset..]
        .char_indices()
        .find_map(|(relative_start, _)| {
            expected_matcher_at(input, needles, kind, offset + relative_start)
        })
}

pub(super) fn expected_matcher_at<'needle>(
    input: &str,
    needles: &[&'needle str],
    kind: MatchKind,
    start: usize,
) -> Option<VStrMatch<'needle>> {
    let mut matched: Option<VStrMatch<'needle>> = None;

    for (pattern_index, needle) in needles.iter().enumerate() {
        if needle.is_empty() || !input[start..].starts_with(needle) {
            continue;
        }

        let candidate = VStrMatch {
            needle,
            pattern_index,
            start,
            end: start + needle.len(),
        };

        matched = Some(match (matched, kind) {
            (None, _) => candidate,
            (Some(current), MatchKind::LeftmostLongest)
                if candidate.needle.len() > current.needle.len() =>
            {
                candidate
            }
            (Some(current), MatchKind::LeftmostFirst | MatchKind::LeftmostLongest) => current,
        });
    }

    matched
}

#[cfg(feature = "unicode-width")]
pub(super) fn assert_unicode_width_properties(
    input: &str,
    widths: impl IntoIterator<Item = usize>,
) {
    for width in widths {
        let taken = take_width(input, width);
        assert!(input.starts_with(taken));
        assert!(input.is_char_boundary(taken.len()));
        assert!(display_width(taken) <= width);
        if let Some(next) = input[taken.len()..].chars().next() {
            let next_end = taken.len() + next.len_utf8();
            assert!(display_width(&input[..next_end]) > width);
        }

        let truncated = truncate_width(input, width, "...");
        if width == 0 {
            assert!(truncated.is_empty());
        } else {
            assert!(display_width(&truncated) <= width);
        }

        let wrapped = wrap_width(input, width);
        if width == 0 {
            assert!(wrapped.is_empty());
        } else {
            for line in wrapped.lines() {
                assert!(width_line_fits_or_progresses(line, width));
            }
        }

        let indented = wrap_width_with_indent(input, width, "> ", "  ");
        if width == 0 {
            assert!(indented.is_empty());
        } else {
            for (index, line) in indented.lines().enumerate() {
                let indent = if index == 0 { "> " } else { "  " };
                assert!(line.starts_with(indent));
                let content = &line[indent.len()..];
                let content_width = width.saturating_sub(display_width(indent)).max(1);
                assert!(width_line_fits_or_progresses(content, content_width));
            }
        }
    }
}

#[cfg(feature = "unicode-width")]
fn width_line_fits_or_progresses(line: &str, width: usize) -> bool {
    display_width(line) <= width || line.chars().count() == 1
}

pub(super) struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub(super) const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.state
    }

    pub(super) fn usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            let upper = u64::try_from(upper).expect("test upper bound must fit in u64");
            usize::try_from(self.next() % upper).expect("bounded value must fit in usize")
        }
    }

    pub(super) fn index_around(&mut self, len: usize) -> isize {
        let span = len.saturating_mul(2).saturating_add(9);
        let sampled = isize::try_from(self.usize(span)).expect("test sample must fit in isize");
        let len = isize::try_from(len).expect("test length must fit in isize");
        sampled - len - 4
    }

    pub(super) fn string(&mut self, max_chars: usize) -> String {
        let len = self.usize(max_chars + 1);
        let alphabet = [
            'a', 'b', 'c', '-', '_', ' ', '.', '*', '你', '好', '🚀', 'e', '\u{301}',
        ];
        let mut output = String::new();
        for _ in 0..len {
            output.push(alphabet[self.usize(alphabet.len())]);
        }
        output
    }

    pub(super) fn path(&mut self, max_segments: usize) -> String {
        let segments = self.usize(max_segments).saturating_add(1);
        let names = ["api", "v1", "users", "项目", "rust", "42"];
        let mut path = String::new();
        for _ in 0..segments {
            path.push('/');
            path.push_str(names[self.usize(names.len())]);
        }
        path
    }
}

pub(super) fn expected_sub(input: &str, from_index: isize, to_index: isize) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let start = expected_normalize_index(from_index, len);
    let end = expected_normalize_index(to_index, len);
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };

    chars[start..end].iter().collect()
}

pub(super) fn expected_normalize_index(index: isize, len: usize) -> usize {
    if index < 0 {
        len.saturating_sub(index.unsigned_abs())
    } else {
        usize::try_from(index).map_or(len, |index| index.min(len))
    }
}
