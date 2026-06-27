/// Converts text to `snake_case`.
///
/// Separators and whitespace are normalized to underscores. Existing camel-case
/// word boundaries are handled for ASCII uppercase letters.
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
    let words = split_words(input);
    words.join("_")
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
    let words = split_words(input);
    words.join("-")
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
    let words = split_words(input);
    let Some((first, rest)) = words.split_first() else {
        return String::new();
    };

    let mut output = String::from(first);
    for word in rest {
        push_title_word(&mut output, word);
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
    let words = split_words(input);
    let mut output = String::new();
    for word in words {
        push_title_word(&mut output, &word);
    }
    output
}

fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut previous_was_lower_or_digit = false;

    for ch in input.chars() {
        if ch.is_alphanumeric() {
            if ch.is_ascii_uppercase() && previous_was_lower_or_digit && !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            current.extend(ch.to_lowercase());
            previous_was_lower_or_digit = ch.is_lowercase() || ch.is_ascii_digit();
        } else {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            previous_was_lower_or_digit = false;
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}

fn push_title_word(output: &mut String, word: &str) {
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        output.extend(first.to_uppercase());
        output.push_str(chars.as_str());
    }
}
