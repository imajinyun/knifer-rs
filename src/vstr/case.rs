/// Converts text to `snake_case`.
///
/// Underscore, hyphen, and space separators are converted to underscores.
/// Camel-case and acronym boundaries are handled in the same daily-use shape as
/// `knifer-go`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_snake_case("helloWorld ID"), "hello_world_id");
/// ```
#[must_use]
pub fn to_snake_case(input: &str) -> String {
    to_separated(input, '_')
}

/// Converts text to `snake_case`.
///
/// This is an alias for [`to_snake_case`] to align with `knifer-go`'s
/// `ToUnderlineCase` API name.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_underline_case("helloWorld ID"), "hello_world_id");
/// ```
#[must_use]
pub fn to_underline_case(input: &str) -> String {
    to_snake_case(input)
}

/// Converts text to `kebab-case`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_kebab_case("helloWorld ID"), "hello-world-id");
/// ```
#[must_use]
pub fn to_kebab_case(input: &str) -> String {
    to_separated(input, '-')
}

/// Converts text to `SCREAMING_SNAKE_CASE`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_screaming_snake_case("HTTPServerID"), "HTTP_SERVER_ID");
/// ```
#[must_use]
pub fn to_screaming_snake_case(input: &str) -> String {
    to_snake_case(input).to_uppercase()
}

/// Converts text to `SCREAMING-KEBAB-CASE`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_screaming_kebab_case("HTTPServerID"), "HTTP-SERVER-ID");
/// ```
#[must_use]
pub fn to_screaming_kebab_case(input: &str) -> String {
    to_kebab_case(input).to_uppercase()
}

/// Converts text to title case with words separated by one ASCII space.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_title_case("hello_world-id"), "Hello World Id");
/// ```
#[must_use]
pub fn to_title_case(input: &str) -> String {
    to_separated(input, ' ')
        .split_whitespace()
        .map(uppercase_first)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts text to `camelCase`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_camel_case("hello_world-id"), "helloWorldId");
/// ```
#[must_use]
pub fn to_camel_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    if !input.contains(['_', '-', ' ']) {
        return lowercase_first(input);
    }

    let mut output = String::with_capacity(input.len());
    let mut uppercase_next = false;
    let mut first = true;

    for ch in input.chars() {
        if matches!(ch, '_' | '-' | ' ') {
            uppercase_next = true;
            continue;
        }

        if first {
            output.extend(ch.to_lowercase());
            first = false;
            continue;
        }

        if uppercase_next {
            output.extend(ch.to_uppercase());
            uppercase_next = false;
        } else {
            output.extend(ch.to_lowercase());
        }
    }

    output
}

/// Converts text to `PascalCase`.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::to_pascal_case("hello_world-id"), "HelloWorldId");
/// ```
#[must_use]
pub fn to_pascal_case(input: &str) -> String {
    uppercase_first(&to_camel_case(input))
}

fn to_separated(input: &str, separator: char) -> String {
    if input.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = input.chars().collect();
    let mut output = String::with_capacity(input.len() + 4);

    for (index, ch) in chars.iter().copied().enumerate() {
        if matches!(ch, '_' | '-' | ' ') {
            output.push(separator);
            continue;
        }

        if ch.is_uppercase() {
            if index > 0 {
                let previous = chars[index - 1];
                let previous_is_lower_or_digit =
                    previous.is_lowercase() || super::is_digit(previous);
                let next_is_lower = chars.get(index + 1).is_some_and(|next| next.is_lowercase());

                if previous_is_lower_or_digit || (previous.is_uppercase() && next_is_lower) {
                    output.push(separator);
                }
            }
            output.extend(ch.to_lowercase());
        } else {
            output.push(ch);
        }
    }

    output
}

fn lowercase_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut output = String::with_capacity(input.len());
    if let Some(first) = chars.next() {
        output.extend(first.to_lowercase());
        output.push_str(chars.as_str());
    }
    output
}

fn uppercase_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut output = String::with_capacity(input.len());
    if let Some(first) = chars.next() {
        output.extend(first.to_uppercase());
        output.push_str(chars.as_str());
    }
    output
}
