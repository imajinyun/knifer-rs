/// Reports whether `path` matches an Ant-style `pattern` using `/` as separator.
///
/// Within a path segment, `*` matches any characters and `?` matches one
/// Unicode scalar value. A segment that is exactly `**` matches zero or more
/// path segments.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ant_path_match("/api/**", "/api/v1/users"));
/// assert!(vstr::ant_path_match("/api/?/users", "/api/v/users"));
/// assert!(!vstr::ant_path_match("/api/*", "/api/v1/users"));
/// ```
#[must_use]
pub fn ant_path_match(pattern: &str, path: &str) -> bool {
    ant_path_match_with_separator(pattern, path, "/")
}

/// Reports whether `path` matches an Ant-style `pattern`.
///
/// If `separator` is empty, `/` is used.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert!(vstr::ant_path_match_with_separator("a.**.d", "a.b.c.d", "."));
/// assert!(!vstr::ant_path_match_with_separator("a.*.d", "a.b.c.d", "."));
/// ```
#[must_use]
pub fn ant_path_match_with_separator(pattern: &str, path: &str, separator: &str) -> bool {
    let separator = if separator.is_empty() { "/" } else { separator };
    if pattern == path {
        return true;
    }

    let pattern_segments = split_path_segments(pattern, separator);
    let path_segments = split_path_segments(path, separator);
    match_ant_segments(&pattern_segments, &path_segments, 0, 0)
}

fn split_path_segments(input: &str, separator: &str) -> Vec<String> {
    let trimmed = input.trim_matches(|ch| separator.contains(ch));
    if trimmed.is_empty() {
        Vec::new()
    } else {
        trimmed.split(separator).map(str::to_owned).collect()
    }
}

fn match_ant_segments(
    pattern: &[String],
    path: &[String],
    pattern_index: usize,
    path_index: usize,
) -> bool {
    if pattern_index == pattern.len() {
        return path_index == path.len();
    }

    if pattern[pattern_index] == "**" {
        for next_path_index in path_index..=path.len() {
            if match_ant_segments(pattern, path, pattern_index + 1, next_path_index) {
                return true;
            }
        }
        return false;
    }

    if path_index == path.len() {
        return false;
    }

    match_ant_segment(&pattern[pattern_index], &path[path_index])
        && match_ant_segments(pattern, path, pattern_index + 1, path_index + 1)
}

fn match_ant_segment(pattern: &str, text: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();
    let mut previous = vec![false; text.len() + 1];
    let mut current = vec![false; text.len() + 1];
    previous[0] = true;

    for pattern_char in &pattern {
        current.fill(false);
        if *pattern_char == '*' {
            current[0] = previous[0];
        }

        for (text_index, text_char) in text.iter().enumerate() {
            current[text_index + 1] = match pattern_char {
                '*' => previous[text_index + 1] || current[text_index],
                '?' => previous[text_index],
                _ => *pattern_char == *text_char && previous[text_index],
            };
        }

        std::mem::swap(&mut previous, &mut current);
    }

    previous[text.len()]
}
