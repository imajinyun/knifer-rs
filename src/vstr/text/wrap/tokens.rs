#[derive(Clone, Copy)]
pub(super) struct CollapseWrapToken<'src> {
    pub(super) text: &'src str,
    pub(super) prefix_space: bool,
}

pub(super) fn collapse_wrap_tokens<'src>(
    paragraph: &'src str,
    word_separators: &[char],
) -> Vec<CollapseWrapToken<'src>> {
    let mut tokens = Vec::new();
    for (word_index, word) in paragraph.split_whitespace().enumerate() {
        push_collapse_word_segments(word, word_index > 0, word_separators, &mut tokens);
    }
    tokens
}

fn push_collapse_word_segments<'src>(
    word: &'src str,
    prefix_space: bool,
    word_separators: &[char],
    tokens: &mut Vec<CollapseWrapToken<'src>>,
) {
    let mut start = 0usize;
    let mut segment_prefix_space = prefix_space;
    for (index, ch) in word.char_indices() {
        if word_separators.contains(&ch) {
            let end = index + ch.len_utf8();
            tokens.push(CollapseWrapToken {
                text: &word[start..end],
                prefix_space: segment_prefix_space,
            });
            start = end;
            segment_prefix_space = false;
        }
    }
    if start < word.len() {
        tokens.push(CollapseWrapToken {
            text: &word[start..],
            prefix_space: segment_prefix_space,
        });
    }
}

pub(super) fn preserve_wrap_tokens<'src>(
    paragraph: &'src str,
    word_separators: &[char],
) -> Vec<&'src str> {
    let mut tokens = Vec::new();
    let mut start = 0usize;
    let mut current_is_whitespace = None;

    for (index, ch) in paragraph.char_indices() {
        let is_whitespace = ch.is_whitespace();
        match current_is_whitespace {
            None => current_is_whitespace = Some(is_whitespace),
            Some(previous) if previous != is_whitespace => {
                tokens.push(&paragraph[start..index]);
                start = index;
                current_is_whitespace = Some(is_whitespace);
            }
            Some(_) => {}
        }

        if !is_whitespace && word_separators.contains(&ch) {
            let end = index + ch.len_utf8();
            tokens.push(&paragraph[start..end]);
            start = end;
            current_is_whitespace = None;
        }
    }

    if start < paragraph.len() {
        tokens.push(&paragraph[start..]);
    }

    tokens
}
