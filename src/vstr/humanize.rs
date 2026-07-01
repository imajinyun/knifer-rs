//! Locale-neutral humanization helpers for counts, sizes, and durations.
//!
//! These are pragmatic daily-business helpers with predictable English output.
//! They intentionally avoid locale databases and floating-point rounding drift:
//! number grouping and byte scaling use integer arithmetic so results are
//! deterministic across platforms.

use std::time::Duration;

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
/// use knifer_rs::vstr;
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

/// Formats an integer as an English ordinal string such as `1st` or `22nd`.
///
/// Negative numbers keep a leading `-`. The suffix uses the standard English
/// rule where 11, 12, and 13 take `th` regardless of the last digit.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
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

/// Groups an integer into thousands separated by `,`.
///
/// Grouping uses integer digits only, so results are locale-neutral and exact.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::number_format(1_234_567), "1,234,567");
/// assert_eq!(vstr::number_format(-12_345), "-12,345");
/// assert_eq!(vstr::number_format(0), "0");
/// ```
#[must_use]
pub fn number_format(value: i64) -> String {
    let digits = value.unsigned_abs().to_string();
    let len = digits.len();
    let mut output = String::with_capacity(len + len / 3 + 1);
    if value < 0 {
        output.push('-');
    }
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (len - index) % 3 == 0 {
            output.push(',');
        }
        output.push(ch);
    }
    output
}

/// Formats a byte count using binary IEC units (`B`, `KiB`, `MiB`, ...).
///
/// Scaling uses 1024 as the base with one decimal digit, computed with integer
/// arithmetic so rounding is deterministic. Values below 1024 are reported in
/// whole bytes.
///
/// # Examples
///
/// ```
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::human_bytes(512), "512 B");
/// assert_eq!(vstr::human_bytes(1024), "1 KiB");
/// assert_eq!(vstr::human_bytes(1536), "1.5 KiB");
/// assert_eq!(vstr::human_bytes(5_242_880), "5 MiB");
/// ```
#[must_use]
pub fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let value = u128::from(bytes);
    let mut index = 1;
    let mut threshold: u128 = 1024;
    while index + 1 < UNITS.len() && value >= threshold * 1024 {
        threshold *= 1024;
        index += 1;
    }

    // Scale to one decimal digit with round-half-up, still in integer space.
    let scaled = (value * 10 + threshold / 2) / threshold;
    let whole = scaled / 10;
    let frac = scaled % 10;
    let unit = UNITS[index];
    if frac == 0 {
        format!("{whole} {unit}")
    } else {
        format!("{whole}.{frac} {unit}")
    }
}

/// Formats a [`Duration`] as a compact human string such as `1h 2m 3s`.
///
/// Units are emitted from the largest non-zero component down to seconds. For
/// sub-second durations the largest non-zero of milliseconds, microseconds, or
/// nanoseconds is used. A zero duration renders as `0s`.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use knifer_rs::vstr;
///
/// assert_eq!(vstr::human_duration(Duration::from_secs(3_661)), "1h 1m 1s");
/// assert_eq!(vstr::human_duration(Duration::from_secs(90)), "1m 30s");
/// assert_eq!(vstr::human_duration(Duration::from_millis(500)), "500ms");
/// assert_eq!(vstr::human_duration(Duration::from_secs(0)), "0s");
/// ```
#[must_use]
pub fn human_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds == 0 {
        let millis = duration.subsec_millis();
        if millis > 0 {
            return format!("{millis}ms");
        }
        let micros = duration.subsec_micros();
        if micros > 0 {
            return format!("{micros}\u{00b5}s");
        }
        let nanos = duration.subsec_nanos();
        if nanos > 0 {
            return format!("{nanos}ns");
        }
        return "0s".to_owned();
    }

    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::with_capacity(4);
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if secs > 0 {
        parts.push(format!("{secs}s"));
    }
    parts.join(" ")
}
