pub(super) fn chars_equal_ignore_case(left: char, right: char) -> bool {
    left == right
        || left.to_lowercase().eq(right.to_lowercase())
        || left.to_uppercase().eq(right.to_uppercase())
}

pub(super) fn prefix_end_ignore_case(input: &str, prefix: &str) -> Option<usize> {
    if prefix.is_empty() {
        return Some(0);
    }

    let mut input_chars = input.char_indices();
    for prefix_ch in prefix.chars() {
        let (_, input_ch) = input_chars.next()?;
        if !chars_equal_ignore_case(input_ch, prefix_ch) {
            return None;
        }
    }

    Some(input_chars.next().map_or(input.len(), |(index, _)| index))
}

pub(super) fn suffix_start_ignore_case(input: &str, suffix: &str) -> Option<usize> {
    if suffix.is_empty() {
        return Some(input.len());
    }

    let mut input_chars = input.char_indices().rev();
    let mut start = None;
    for suffix_ch in suffix.chars().rev() {
        let (index, input_ch) = input_chars.next()?;
        if !chars_equal_ignore_case(input_ch, suffix_ch) {
            return None;
        }
        start = Some(index);
    }

    start
}
