use crate::vstr::*;

#[test]
fn number_format_groups_thousands_without_locale_data() {
    assert_eq!(number_format(0), "0");
    assert_eq!(number_format(1), "1");
    assert_eq!(number_format(12), "12");
    assert_eq!(number_format(123), "123");
    assert_eq!(number_format(1_234), "1,234");
    assert_eq!(number_format(12_345), "12,345");
    assert_eq!(number_format(1_234_567), "1,234,567");
    assert_eq!(number_format(-1_234), "-1,234");
    assert_eq!(number_format(-12_345), "-12,345");
    assert_eq!(number_format(i64::MAX), "9,223,372,036,854,775,807");
}

#[test]
fn number_format_with_uses_a_configurable_separator() {
    assert_eq!(number_format_with(0, '.'), "0");
    assert_eq!(number_format_with(123, '.'), "123");
    assert_eq!(number_format_with(1_234_567, '.'), "1.234.567");
    assert_eq!(number_format_with(1_234_567, ' '), "1 234 567");
    assert_eq!(number_format_with(-12_345, '_'), "-12_345");
}

#[test]
fn number_format_float_groups_and_fixes_decimals() {
    assert_eq!(number_format_float(0.0, 2), "0.00");
    assert_eq!(number_format_float(1_234.5, 2), "1,234.50");
    assert_eq!(number_format_float(1_234_567.891, 2), "1,234,567.89");
    assert_eq!(number_format_float(-1_234.567, 1), "-1,234.6");
    assert_eq!(number_format_float(1_000.0, 0), "1,000");
    assert_eq!(number_format_float(999.999, 2), "1,000.00");
    // Rounded-to-zero magnitude never keeps a negative sign.
    assert_eq!(number_format_float(-0.0, 2), "0.00");
    assert_eq!(number_format_float(-0.004, 2), "0.00");
    // Non-finite inputs pass through unchanged.
    assert_eq!(number_format_float(f64::INFINITY, 2), "inf");
    assert_eq!(number_format_float(f64::NEG_INFINITY, 2), "-inf");
    assert_eq!(number_format_float(f64::NAN, 2), "NaN");
}

#[test]
fn human_count_uses_compact_short_scale_units() {
    assert_eq!(human_count(0), "0");
    assert_eq!(human_count(999), "999");
    assert_eq!(human_count(1_000), "1K");
    assert_eq!(human_count(1_200), "1.2K");
    assert_eq!(human_count(1_500), "1.5K");
    assert_eq!(human_count(12_345), "12.3K");
    assert_eq!(human_count(3_400_000), "3.4M");
    assert_eq!(human_count(5_600_000_000), "5.6B");
    assert_eq!(human_count(7_800_000_000_000), "7.8T");
    assert_eq!(human_count(-1_500), "-1.5K");
    // Rounding that reaches the next unit rolls over.
    assert_eq!(human_count(999_999), "1M");
    assert_eq!(human_count(999_500), "999.5K");
    assert_eq!(human_count(999_950), "1M");
    // Very large values saturate at the top unit.
    assert_eq!(human_count(i64::MAX), "9223372T");
}

#[test]
fn human_bytes_scales_with_binary_iec_units() {
    assert_eq!(human_bytes(0), "0 B");
    assert_eq!(human_bytes(512), "512 B");
    assert_eq!(human_bytes(1023), "1023 B");
    assert_eq!(human_bytes(1024), "1 KiB");
    assert_eq!(human_bytes(1536), "1.5 KiB");
    assert_eq!(human_bytes(1_048_576), "1 MiB");
    assert_eq!(human_bytes(5_242_880), "5 MiB");
    assert_eq!(human_bytes(1_073_741_824), "1 GiB");
    // Round-half-up on the single decimal digit stays deterministic.
    assert_eq!(human_bytes(1_127), "1.1 KiB");
}

#[test]
fn human_duration_emits_largest_nonzero_units() {
    use std::time::Duration;

    assert_eq!(human_duration(Duration::from_secs(0)), "0s");
    assert_eq!(human_duration(Duration::from_secs(1)), "1s");
    assert_eq!(human_duration(Duration::from_secs(59)), "59s");
    assert_eq!(human_duration(Duration::from_secs(60)), "1m");
    assert_eq!(human_duration(Duration::from_secs(90)), "1m 30s");
    assert_eq!(human_duration(Duration::from_secs(3_600)), "1h");
    assert_eq!(human_duration(Duration::from_secs(3_661)), "1h 1m 1s");
    assert_eq!(human_duration(Duration::from_secs(90_061)), "1d 1h 1m 1s");
    assert_eq!(human_duration(Duration::from_millis(500)), "500ms");
    assert_eq!(human_duration(Duration::from_micros(250)), "250\u{00b5}s");
    assert_eq!(human_duration(Duration::from_nanos(7)), "7ns");
}
