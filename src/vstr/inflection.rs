//! English word and identifier inflection helpers.
//!
//! These are pragmatic daily-business helpers with predictable English output,
//! aligned with the naming that Rails' `ActiveSupport::Inflector` and the
//! `Inflector` crate popularized. They are deliberately rule-based rather than a
//! full linguistic engine: `pluralize`/`singularize` round-trip the documented
//! regular and common-irregular forms, and identifier helpers normalize through
//! the [`to_snake_case`](super::to_snake_case) shape so they accept
//! `snake_case`, `camelCase`, and `kebab-case` input alike.

/// Returns the English plural form of `word` when `count` is not one.
///
/// The word is returned unchanged when `count` is `1` or `-1`. Pluralization is
/// a pragmatic English helper (regular `-s`/`-es`/`-ies` rules plus a few common
/// irregulars), not a full linguistic engine. Only the word is returned, so
/// callers compose the count themselves.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::pluralize("cat", 1), "cat");
/// assert_eq!(vstr::pluralize("cat", 2), "cats");
/// assert_eq!(vstr::pluralize("bus", 3), "buses");
/// assert_eq!(vstr::pluralize("city", 5), "cities");
/// assert_eq!(vstr::pluralize("person", 4), "people");
/// ```
#[must_use]
pub fn pluralize(word: &str, count: i64) -> String {
    if count == 1 || count == -1 {
        word.to_owned()
    } else {
        pluralize_word(word)
    }
}

fn pluralize_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let lower = word.to_ascii_lowercase();
    let irregular = match lower.as_str() {
        "person" => Some("people"),
        "man" => Some("men"),
        "woman" => Some("women"),
        "child" => Some("children"),
        "tooth" => Some("teeth"),
        "foot" => Some("feet"),
        "mouse" => Some("mice"),
        "goose" => Some("geese"),
        _ => None,
    };
    if let Some(plural) = irregular {
        return plural.to_owned();
    }

    // Consonant + "y" becomes "ies" (city -> cities, but day -> days).
    if let Some(stem) = lower.strip_suffix('y') {
        let vowel_before_y = stem
            .chars()
            .next_back()
            .is_some_and(|ch| "aeiou".contains(ch));
        if !stem.is_empty() && !vowel_before_y {
            let mut plural = word[..word.len() - 1].to_owned();
            plural.push_str("ies");
            return plural;
        }
    }

    // Sibilant endings take "es" (bus -> buses, box -> boxes, dish -> dishes).
    if lower.ends_with('s')
        || lower.ends_with('x')
        || lower.ends_with('z')
        || lower.ends_with("ch")
        || lower.ends_with("sh")
    {
        return format!("{word}es");
    }

    format!("{word}s")
}

/// Returns the English singular form of `word`.
///
/// This is the pragmatic inverse of [`pluralize`]: it round-trips the documented
/// regular (`-s`/`-es`/`-ies`) and common-irregular forms. English pluralization
/// is not fully reversible with simple rules (for example `buses` and `houses`
/// share an ending), so this is a best-effort helper for the same everyday word
/// set that [`pluralize`] targets, not a full linguistic engine. A word that is
/// already singular is returned unchanged.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::singularize("cats"), "cat");
/// assert_eq!(vstr::singularize("buses"), "bus");
/// assert_eq!(vstr::singularize("cities"), "city");
/// assert_eq!(vstr::singularize("people"), "person");
/// assert_eq!(vstr::singularize("cat"), "cat");
/// ```
#[must_use]
pub fn singularize(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let lower = word.to_ascii_lowercase();
    let irregular = match lower.as_str() {
        "people" => Some("person"),
        "men" => Some("man"),
        "women" => Some("woman"),
        "children" => Some("child"),
        "teeth" => Some("tooth"),
        "feet" => Some("foot"),
        "mice" => Some("mouse"),
        "geese" => Some("goose"),
        _ => None,
    };
    if let Some(singular) = irregular {
        return singular.to_owned();
    }

    // Consonant + "ies" reverses to "y" (cities -> city).
    if let Some(stem) = lower.strip_suffix("ies") {
        if !stem.is_empty() {
            let mut singular = word[..word.len() - 3].to_owned();
            singular.push('y');
            return singular;
        }
    }

    // Sibilant "es" endings drop the "es" (buses -> bus, boxes -> box).
    for suffix in ["ses", "xes", "zes", "ches", "shes"] {
        if lower.ends_with(suffix) {
            return word[..word.len() - 2].to_owned();
        }
    }

    // A lone trailing "s" (but not "ss") drops (cats -> cat, class -> class).
    if lower.ends_with('s') && !lower.ends_with("ss") {
        return word[..word.len() - 1].to_owned();
    }

    word.to_owned()
}

/// Formats an integer as an English ordinal string such as `1st` or `22nd`.
///
/// Negative numbers keep a leading `-`. The suffix uses the standard English
/// rule where 11, 12, and 13 take `th` regardless of the last digit.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::ordinalize(1), "1st");
/// assert_eq!(vstr::ordinalize(2), "2nd");
/// assert_eq!(vstr::ordinalize(11), "11th");
/// assert_eq!(vstr::ordinalize(23), "23rd");
/// assert_eq!(vstr::ordinalize(-1), "-1st");
/// ```
#[must_use]
pub fn ordinalize(value: i64) -> String {
    let magnitude = value.unsigned_abs();
    let suffix = match (magnitude % 100, magnitude % 10) {
        (11..=13, _) => "th",
        (_, 1) => "st",
        (_, 2) => "nd",
        (_, 3) => "rd",
        _ => "th",
    };
    format!("{value}{suffix}")
}

/// Strips the ordinal suffix from an ordinal string, returning the number text.
///
/// This is the inverse of [`ordinalize`]: `st`, `nd`, `rd`, or `th` is removed
/// only when it directly follows an ASCII digit, so plain words are returned
/// unchanged. The number is returned as text to stay lossless (leading `-` and
/// any surrounding characters are preserved).
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::deordinalize("1st"), "1");
/// assert_eq!(vstr::deordinalize("22nd"), "22");
/// assert_eq!(vstr::deordinalize("-13th"), "-13");
/// assert_eq!(vstr::deordinalize("month"), "month");
/// ```
#[must_use]
pub fn deordinalize(input: &str) -> String {
    for suffix in ["st", "nd", "rd", "th"] {
        if let Some(stem) = input.strip_suffix(suffix) {
            if stem
                .chars()
                .next_back()
                .is_some_and(|ch| ch.is_ascii_digit())
            {
                return stem.to_owned();
            }
        }
    }
    input.to_owned()
}

/// Converts an identifier into a lower-cased, space-separated sentence.
///
/// The input is normalized through [`to_snake_case`](super::to_snake_case), a
/// trailing `_id` is dropped (a common foreign-key convention), underscores
/// become spaces, and only the first word is capitalized. This mirrors Rails'
/// `humanize` and complements [`to_sentence_case`](super::to_sentence_case),
/// which does not strip the `_id` suffix.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::humanize("employee_salary"), "Employee salary");
/// assert_eq!(vstr::humanize("author_id"), "Author");
/// assert_eq!(vstr::humanize("EmployeeSalary"), "Employee salary");
/// ```
#[must_use]
pub fn humanize(input: &str) -> String {
    let snake = super::to_snake_case(input);
    let stem = snake.strip_suffix("_id").unwrap_or(&snake);
    super::capitalize(&stem.replace('_', " "))
}

/// Converts an identifier into `Title Case`, dropping a trailing `_id`.
///
/// The input is normalized through [`to_snake_case`](super::to_snake_case), a
/// trailing `_id` is dropped, and each remaining word is capitalized. This
/// mirrors Rails' `titleize` and complements
/// [`to_title_case`](super::to_title_case), which keeps the `_id` suffix.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::titleize("man_from_the_boondocks"), "Man From The Boondocks");
/// assert_eq!(vstr::titleize("author_id"), "Author");
/// assert_eq!(vstr::titleize("EmployeeSalary"), "Employee Salary");
/// ```
#[must_use]
pub fn titleize(input: &str) -> String {
    let snake = super::to_snake_case(input);
    let stem = snake.strip_suffix("_id").unwrap_or(&snake);
    stem.split('_')
        .filter(|word| !word.is_empty())
        .map(super::capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts an identifier into a `PascalCase` class name.
///
/// This is an inflection-style alias for
/// [`to_pascal_case`](super::to_pascal_case), provided under the name that
/// Rails' `camelize` and the `Inflector` crate popularized for producing class
/// names from `snake_case` identifiers.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::camelize("active_record"), "ActiveRecord");
/// assert_eq!(vstr::camelize("employee-salary"), "EmployeeSalary");
/// ```
#[must_use]
pub fn camelize(input: &str) -> String {
    super::to_pascal_case(input)
}
