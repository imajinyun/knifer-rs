//! Locale-neutral humanization helpers for counts, sizes, and durations.
//!
//! These are pragmatic daily-business helpers with predictable English output.
//! They intentionally avoid locale databases and floating-point rounding drift:
//! number grouping and byte scaling use integer arithmetic so results are
//! deterministic across platforms. Word-level inflection (`pluralize`,
//! `ordinalize`, and their inverses) lives in the sibling `inflection` module.

use std::time::Duration;

/// Groups an integer into thousands separated by `,`.
///
/// Grouping uses integer digits only, so results are locale-neutral and exact.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::number_format(1_234_567), "1,234,567");
/// assert_eq!(vstr::number_format(-12_345), "-12,345");
/// assert_eq!(vstr::number_format(0), "0");
/// ```
#[must_use]
pub fn number_format(value: i64) -> String {
    number_format_with(value, ',')
}

/// Groups an integer into thousands using a configurable separator.
///
/// Like [`number_format`] but the thousands separator is chosen by the caller,
/// which covers locales that group with `.` or a space. Grouping still uses
/// integer digits only, so results are exact.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::number_format_with(1_234_567, '.'), "1.234.567");
/// assert_eq!(vstr::number_format_with(1_234_567, ' '), "1 234 567");
/// assert_eq!(vstr::number_format_with(-12_345, '_'), "-12_345");
/// ```
#[must_use]
pub fn number_format_with(value: i64, separator: char) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut output = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    if value < 0 {
        output.push('-');
    }
    output.push_str(&group_thousands(&digits, separator));
    output
}

/// Groups a floating-point number with `,` and a fixed number of decimals.
///
/// The integer part is grouped into thousands and the fraction is rendered with
/// exactly `decimals` digits. Rounding follows Rust's standard `{:.*}`
/// formatting (round-half-to-even), so output is deterministic across platforms.
/// Non-finite inputs (`NaN`, `inf`, `-inf`) are passed through unchanged, and
/// negative zero is rendered without a sign.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::number_format_float(1_234.5, 2), "1,234.50");
/// assert_eq!(vstr::number_format_float(-1_234.567, 1), "-1,234.6");
/// assert_eq!(vstr::number_format_float(1_000.0, 0), "1,000");
/// ```
#[must_use]
pub fn number_format_float(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return format!("{value}");
    }

    let formatted = format!("{:.*}", decimals, value.abs());
    let (int_part, frac_part) = match formatted.split_once('.') {
        Some((int_part, frac_part)) => (int_part, Some(frac_part)),
        None => (formatted.as_str(), None),
    };

    // Suppress a sign when the rounded magnitude is zero (avoids "-0.00").
    let rounded_is_zero = formatted.bytes().all(|byte| byte == b'0' || byte == b'.');
    let negative = value.is_sign_negative() && !rounded_is_zero;

    let mut output = String::with_capacity(formatted.len() + int_part.len() / 3 + 1);
    if negative {
        output.push('-');
    }
    output.push_str(&group_thousands(int_part, ','));
    if let Some(frac) = frac_part {
        output.push('.');
        output.push_str(frac);
    }
    output
}

/// Formats an integer as a compact short-scale count such as `1.2K` or `3.4M`.
///
/// Values below 1000 are rendered as-is. Larger values scale by 1000 through
/// `K`, `M`, `B`, and `T` with one decimal digit, computed with integer
/// arithmetic (round-half-up) so results are deterministic. A trailing `.0` is
/// dropped, and rounding that would reach the next unit rolls over (for example
/// `999_999` renders as `1M`). Negative values keep a leading `-`.
///
/// # Examples
///
/// ```
/// use kniferrs::vstr;
///
/// assert_eq!(vstr::human_count(999), "999");
/// assert_eq!(vstr::human_count(1_200), "1.2K");
/// assert_eq!(vstr::human_count(3_400_000), "3.4M");
/// assert_eq!(vstr::human_count(5_600_000_000), "5.6B");
/// assert_eq!(vstr::human_count(-1_500), "-1.5K");
/// ```
#[must_use]
pub fn human_count(value: i64) -> String {
    const UNITS: [&str; 4] = ["K", "M", "B", "T"];
    let magnitude = value.unsigned_abs();
    if magnitude < 1000 {
        return value.to_string();
    }

    let magnitude = u128::from(magnitude);
    let mut index = 0;
    let mut threshold: u128 = 1000;
    while index + 1 < UNITS.len() && magnitude >= threshold * 1000 {
        threshold *= 1000;
        index += 1;
    }

    // Scale to one decimal digit with round-half-up, still in integer space.
    let mut scaled = (magnitude * 10 + threshold / 2) / threshold;
    // Rounding can push the value into the next unit (e.g. 999_999 -> 1000K).
    if scaled >= 10_000 && index + 1 < UNITS.len() {
        threshold *= 1000;
        index += 1;
        scaled = (magnitude * 10 + threshold / 2) / threshold;
    }

    let whole = scaled / 10;
    let frac = scaled % 10;
    let sign = if value < 0 { "-" } else { "" };
    let unit = UNITS[index];
    if frac == 0 {
        format!("{sign}{whole}{unit}")
    } else {
        format!("{sign}{whole}.{frac}{unit}")
    }
}

fn group_thousands(digits: &str, separator: char) -> String {
    let len = digits.len();
    let mut output = String::with_capacity(len + len / 3);
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (len - index) % 3 == 0 {
            output.push(separator);
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
/// use kniferrs::vstr;
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
/// use kniferrs::vstr;
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
